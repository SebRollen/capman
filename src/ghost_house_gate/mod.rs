use bevy::prelude::*;
use std::any::TypeId;
use std::collections::HashSet;
use std::time::Duration;

use crate::dots::DotEaten;
use crate::game_state::GameState;
use crate::ghosts::{Blinky, Clyde, GhostType, Inky, Pinky};
use crate::level::Level;
use crate::pacman::PacmanKilled;
use crate::ghost_house_gate::counter::Counter;

mod counter;

const NUM_GHOST_TYPES: usize = 4;

pub struct GhostHouseGatePlugin;

impl Plugin for GhostHouseGatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(create_gate)
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(update_ghost_house_gate)
                    .with_system(increment_counter_when_dot_eaten)
                    .with_system(switch_to_global_counter_when_pacman_got_killed)
            )
        ;
    }
}

fn create_gate(
    mut commands: Commands,
    level: Res<Level>,
) {
    commands.insert_resource(GhostHouseGate::new(&level));
}

fn update_ghost_house_gate(
    time: Res<Time>,
    mut ghost_house_gate: ResMut<GhostHouseGate>,
) {
    ghost_house_gate.update(time.delta())
}

fn increment_counter_when_dot_eaten(
    mut event_reader: EventReader<DotEaten>,
    mut ghost_house_gate: ResMut<GhostHouseGate>,
) {
    for _ in event_reader.iter() {
        ghost_house_gate.increment_counter()
    }
}

fn switch_to_global_counter_when_pacman_got_killed(
    mut event_reader: EventReader<PacmanKilled>,
    mut ghost_house_gate: ResMut<GhostHouseGate>,
) {
    for _ in event_reader.iter() {
        ghost_house_gate.switch_to_global_counter()
    }
}

/// Resource that tells if ghost can leave the ghost house.
///
/// This is the most complex piece of logic in this entire game. It basically works like this:
///
/// There are two types of counters: one which counts per ghost and a global counter. At the beginning
/// of the game, the per ghost counter is set to zero for each ghost and the global counter is inactive.
/// If pacman eats a dot, the current counter gets incremented.
///
/// If the per ghost counter is active, a ghost can leave if its personal limit is reached. Only
/// the counter from the currently waiting ghost is incremented.
///
/// The order of preference for ghosts is Blinky, Pinky, Inky and Clyde. Blinky and Pinky can always
/// leave the house at the beginning of the game.
///
/// If pacman dies, the per ghost counter is switched with a newly initialized global one (while retaining
/// the per ghost one). The waiting ghost can now leave when its predefined limit is reached. When
/// Clyde left the house, the counter switches back to the per ghost one.
///
/// There is also a timer active. If the timer reaches zero, the waiting ghost can return immediately.
/// The timer gets reset when pacman eats a dot.
pub struct GhostHouseGate {
    released_ghosts: HashSet<TypeId>,
    ghost_preference_iterator: GhostPreferenceIterator,
    current_waiting_ghost_id: TypeId,
    counter: Counter,
    release_timer: Timer,
}

impl GhostHouseGate {
    fn new(level: &Level) -> Self {
        let mut iterator = GhostPreferenceIterator::new();
        let current_waiting_ghost_id = iterator.next().unwrap();

        GhostHouseGate {
            released_ghosts: HashSet::with_capacity(NUM_GHOST_TYPES),
            ghost_preference_iterator: iterator,
            current_waiting_ghost_id,
            counter: Counter::new(level),
            release_timer: Self::create_release_timer_for_level(level),
        }
    }

    fn create_release_timer_for_level(level: &Level) -> Timer {
        match **level {
            l if l < 5 => Timer::from_seconds(4.0, false),
            _ => Timer::from_seconds(3.0, false)
        }
    }

    /// Ask the gate if the given ghost type can be released.
    pub fn ghost_can_leave_house<G: GhostType + 'static>(&self) -> bool {
        self.released_ghosts.contains(&TypeId::of::<G>())
    }

    /// Increment the current counter. Typically when a dot was eaten.
    /// Also resets the release timer.
    fn increment_counter(&mut self) {
        self.release_timer.reset();
        self.counter.increment(&self.current_waiting_ghost_id)
    }

    /// Switch to the global counter. Typically called when pacman died.
    fn switch_to_global_counter(&mut self) {
        self.counter.switch_to_global();
        self.release_timer.reset();
        self.released_ghosts.clear();
        self.ghost_preference_iterator = GhostPreferenceIterator::new();
        self.current_waiting_ghost_id = self.ghost_preference_iterator.next().expect("first item should exists");
    }

    /// Proceed the release timer and check if the current waiting ghost can be released.
    fn update(&mut self, delta: Duration) {
        if self.all_ghosts_released() { return; }

        self.release_timer.tick(delta);

        if self.release_timer.finished() {
            self.release_timer.reset();
            self.release_current_waiting_ghost();
        } else if self.counter.limit_reached(&self.current_waiting_ghost_id) {
            self.release_current_waiting_ghost()
        }
    }

    fn all_ghosts_released(&self) -> bool {
        self.released_ghosts.len() == NUM_GHOST_TYPES
    }

    fn release_current_waiting_ghost(&mut self) {
        self.released_ghosts.insert(self.current_waiting_ghost_id);

        if let Some(id) = self.ghost_preference_iterator.next() {
            self.current_waiting_ghost_id = id
        }
    }
}

struct GhostPreferenceIterator {
    ghost_preferences: [TypeId; NUM_GHOST_TYPES],
    current: usize,
}

impl GhostPreferenceIterator {
    fn new() -> Self {
        GhostPreferenceIterator {
            ghost_preferences: [TypeId::of::<Blinky>(), TypeId::of::<Pinky>(), TypeId::of::<Inky>(), TypeId::of::<Clyde>()],
            current: 0,
        }
    }
}

impl Iterator for GhostPreferenceIterator {
    type Item = TypeId;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            c if c >= NUM_GHOST_TYPES => None,
            ref mut c => {
                let next = self.ghost_preferences[*c];
                *c += 1;
                Some(next)
            }
        }
    }
}