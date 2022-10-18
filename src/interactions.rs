use crate::board_dimensions::BoardDimensions;
use bevy::prelude::*;

use crate::capman::Capman;
use crate::edibles::dots::{Dot, EatenDots};
use crate::edibles::energizer::Energizer;
use crate::edibles::fruit::{Fruit, FruitDespawnTimer};
use crate::ghosts::state::State;
use crate::ghosts::{CurrentlyEatenGhost, Ghost};
use crate::life_cycle::LifeCycle::Running;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ECapmanHit>()
            .add_event::<EGhostEaten>()
            .add_event::<EDotEaten>()
            .add_event::<EEnergizerEaten>()
            .add_event::<EFruitEaten>()
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(capman_hits_ghost.label(LCapmanGhostHitDetection))
                    .with_system(capman_eat_dot)
                    .with_system(capman_eat_energizer)
                    .with_system(eat_fruit_when_capman_touches_it)
                    .label(LCapmanEnergizerHitDetection),
            );
    }
}

/// Marks systems that check hits between capman and ghosts
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub struct LCapmanGhostHitDetection;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub struct LCapmanEnergizerHitDetection;

/// Fired when capman was hit by a ghost.
pub struct ECapmanHit;

/// Fired when Capman ate a ghost in frightened state.
/// Contains the eaten ghost entity and transform.
#[derive(Copy, Clone)]
pub struct EGhostEaten(pub Entity, pub Transform);

/// Fired when capman eats a dot.
pub struct EDotEaten;

/// Fired when capman eats an energizer.
#[derive(Copy, Clone)]
pub struct EEnergizerEaten;

/// Event that gets fired when capman ate a fruit.
pub struct EFruitEaten(pub Fruit, pub Transform);

fn capman_hits_ghost(
    mut commands: Commands,
    mut killed_event_writer: EventWriter<ECapmanHit>,
    mut eat_event_writer: EventWriter<EGhostEaten>,
    dimensions: Res<BoardDimensions>,
    capman_query: Query<&Transform, With<Capman>>,
    ghost_query: Query<(Entity, &Transform, &State), With<Ghost>>,
) {
    for capman_transform in &capman_query {
        for (entity, ghost_transform, state) in &ghost_query {
            if dimensions.trans_to_pos(capman_transform) == dimensions.trans_to_pos(ghost_transform)
            {
                if let State::Scatter | State::Chase = state {
                    killed_event_writer.send(ECapmanHit)
                }

                if let State::Frightened = state {
                    eat_event_writer.send(EGhostEaten(entity, *ghost_transform));
                    commands.insert_resource(CurrentlyEatenGhost(entity))
                }
            }
        }
    }
}

fn capman_eat_dot(
    mut commands: Commands,
    mut event_writer: EventWriter<EDotEaten>,
    mut eaten_dots: ResMut<EatenDots>,
    dimensions: Res<BoardDimensions>,
    capman_positions: Query<&Transform, With<Capman>>,
    dot_positions: Query<(Entity, &Transform), With<Dot>>,
) {
    for capman_tf in &capman_positions {
        for (entity, dot_tf) in &dot_positions {
            if dimensions.trans_to_pos(capman_tf) == dimensions.trans_to_pos(dot_tf) {
                commands.entity(entity).despawn();
                eaten_dots.increment();
                event_writer.send(EDotEaten)
            }
        }
    }
}

fn capman_eat_energizer(
    mut commands: Commands,
    mut event_writer: EventWriter<EEnergizerEaten>,
    dimensions: Res<BoardDimensions>,
    capman_positions: Query<&Transform, With<Capman>>,
    energizer_positions: Query<(Entity, &Transform), With<Energizer>>,
) {
    for capman_transform in &capman_positions {
        for (energizer_entity, energizer_transform) in &energizer_positions {
            if dimensions.trans_to_pos(energizer_transform)
                == dimensions.trans_to_pos(capman_transform)
            {
                commands.entity(energizer_entity).despawn();
                event_writer.send(EEnergizerEaten)
            }
        }
    }
}

/// If capman touches the fruit, despawn it, remove the timer and send an event.
fn eat_fruit_when_capman_touches_it(
    mut commands: Commands,
    mut event_writer: EventWriter<EFruitEaten>,
    dimensions: Res<BoardDimensions>,
    capman_query: Query<&Transform, With<Capman>>,
    fruit_query: Query<(Entity, &Fruit, &Transform)>,
) {
    for capman_tf in &capman_query {
        for (entity, fruit, fruit_tf) in &fruit_query {
            if dimensions.trans_to_pos(capman_tf) == dimensions.trans_to_pos(fruit_tf) {
                commands.entity(entity).despawn();
                commands.remove_resource::<FruitDespawnTimer>();
                event_writer.send(EFruitEaten(*fruit, *fruit_tf))
            }
        }
    }
}
