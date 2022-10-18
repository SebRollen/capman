use crate::board_dimensions::BoardDimensions;
use bevy::ecs::event::Event;
use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use std::fmt::Formatter;

use crate::common::Direction;
use crate::common::XYEqual;
use crate::edibles::energizer::EnergizerOver;
use crate::ghost_house::GhostHouse;
use crate::ghosts::schedule::Schedule;
use crate::ghosts::state::State::*;
use crate::ghosts::target::Target;
use crate::ghosts::Ghost;
use crate::interactions::{
    EEnergizerEaten, EGhostEaten, LCapmanEnergizerHitDetection, LCapmanGhostHitDetection,
};
use crate::life_cycle::LifeCycle::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(Running)
                .with_system(update_state)
                .after(LCapmanGhostHitDetection)
                .after(LCapmanEnergizerHitDetection)
                .label(StateSetter),
        )
        .add_system_set(
            SystemSet::on_update(GhostEatenPause).with_system(update_state_on_eaten_pause),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub struct StateSetter;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    Scatter,
    Chase,
    Frightened,
    Eaten,
    Spawned,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
struct StateUpdateComponents<'a> {
    entity: Entity,
    ghost: &'a Ghost,
    state: &'a mut State,
    target: &'a mut Target,
    direction: &'a mut Direction,
    transform: &'a Transform,
}

fn update_state(
    schedule: Res<Schedule>,
    ghost_house: Res<GhostHouse>,
    dimensions: Res<BoardDimensions>,
    energizer_over_events: EventReader<EnergizerOver>,
    energizer_eaten_events: EventReader<EEnergizerEaten>,
    ghost_eaten_events: EventReader<EGhostEaten>,
    mut query: Query<StateUpdateComponents, With<Ghost>>,
) {
    let energizer_eaten = energizer_eaten(energizer_eaten_events);
    let energizer_over = energizer_over(energizer_over_events);
    let ghost_eaten_events = collect_events(ghost_eaten_events);

    for mut components in &mut query {
        if ghost_eaten(components.entity, &ghost_eaten_events) {
            *components.state = Eaten;
            continue;
        }

        if energizer_eaten && matches!(*components.state, Chase | Scatter) {
            process_energizer_eaten(&dimensions, &mut components);
            continue;
        }

        match *components.state {
            Spawned => process_spawned(&schedule, &ghost_house, &mut components),
            Scatter | Chase => process_scatter_chase(&schedule, &dimensions, &mut components),
            Frightened => process_frightened(&schedule, energizer_over, &mut components),
            Eaten => process_eaten(&ghost_house, &mut components),
        }
    }
}

fn update_state_on_eaten_pause(
    schedule: Res<Schedule>,
    ghost_house: Res<GhostHouse>,
    mut query: Query<StateUpdateComponents, With<Ghost>>,
) {
    for mut components in &mut query {
        match *components.state {
            Spawned => process_spawned(&schedule, &ghost_house, &mut components),
            Eaten => process_eaten(&ghost_house, &mut components),
            _ => continue,
        }
    }
}

fn collect_events<'a, E: Copy + Event>(mut event_reader: EventReader<E>) -> Vec<E> {
    event_reader.iter().map(|e| *e).collect()
}

fn energizer_eaten(mut events: EventReader<EEnergizerEaten>) -> bool {
    events.iter().count() > 0
}

fn energizer_over(mut events: EventReader<EnergizerOver>) -> bool {
    events.iter().count() > 0
}

fn ghost_eaten(entity: Entity, eaten_events: &Vec<EGhostEaten>) -> bool {
    eaten_events.iter().filter(|e| e.0 == entity).count() > 0
}

fn process_energizer_eaten(
    dimensions: &BoardDimensions,
    components: &mut StateUpdateComponentsItem,
) {
    let target_coordinates = if components.target.is_set() {
        components.target.get()
    } else {
        components.transform.translation
    };
    let target_position = dimensions.vec_to_pos(&target_coordinates);
    let coordinates_ghost_came_from = dimensions.pos_to_vec(
        &target_position
            .get_neighbour_in_direction(&components.direction.opposite())
            .position,
        0.0,
    );

    *components.state = State::Frightened;
    components.direction.reverse();
    components.target.set(coordinates_ghost_came_from);
}

fn process_spawned(
    schedule: &Schedule,
    ghost_house: &GhostHouse,
    components: &mut StateUpdateComponentsItem,
) {
    let coordinates = components.transform.translation;
    if coordinates.xy_equal_to(&ghost_house.coordinates_in_front_of_entrance()) {
        *components.state = schedule.current_state();
        *components.direction = ghost_house.entrance_direction.rotate_left();
    }
}

/// If the current schedule is different to the ghosts state, the new state is the current schedule and
/// the ghost reverses his location.
fn process_scatter_chase(
    schedule: &Schedule,
    dimensions: &BoardDimensions,
    components: &mut StateUpdateComponentsItem,
) {
    let schedule_state = schedule.current_state();

    if let (Chase, Scatter) | (Scatter, Chase) = (*components.state, schedule_state) {
        *components.state = schedule_state;

        let target_coordinates = if components.target.is_set() {
            components.target.get()
        } else {
            components.transform.translation
        };

        let target_position = dimensions.vec_to_pos(&target_coordinates);
        let coordinates_ghost_came_from = dimensions.pos_to_vec(
            &target_position
                .get_neighbour_in_direction(&components.direction.opposite())
                .position,
            0.0,
        );

        components.direction.reverse();
        components.target.set(coordinates_ghost_came_from);
    }
}

fn process_frightened(
    schedule: &Schedule,
    energizer_over: bool,
    components: &mut StateUpdateComponentsItem,
) {
    if energizer_over {
        *components.state = schedule.current_state()
    }
}

fn process_eaten(ghost_house: &GhostHouse, components: &mut StateUpdateComponentsItem) {
    let coordinates = components.transform.translation;

    if coordinates.xy_equal_to(&ghost_house.respawn_coordinates_of(components.ghost)) {
        *components.state = Spawned
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[macro_export]
macro_rules! state_skip_if {
    ($components:ident.$state:ident = $pattern:pat) => {
        if let $pattern = *$components.$state {
            continue;
        }
    };

    ($state:ident = $pattern:pat) => {
        if let $pattern = *$state {
            continue;
        }
    };

    ($components:ident.$state:ident != $pattern:pat) => {
        match *$components.$state {
            $pattern => (),
            _ => continue,
        }
    };

    ($state:ident != $pattern:pat) => {
        match *$state {
            $pattern => (),
            _ => continue,
        }
    };
}
