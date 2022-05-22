use bevy::prelude::*;

use crate::ghosts::movement::MovePlugin;
use crate::ghosts::schedule::SchedulePlugin;
use crate::ghosts::spawn::spawn_ghosts;
use crate::ghosts::state::StatePlugin;
use crate::ghosts::target::{Target, TargetPlugin};
use crate::tunnels::GhostPassedTunnel;

pub mod movement;
pub mod spawn;
pub mod state;
pub mod target;
mod schedule;

/// Used to mark every ghost.
#[derive(Component, Eq, PartialEq)]
pub struct Ghost;

#[derive(Copy, Clone, Component)]
pub struct Blinky;

#[derive(Copy, Clone, Component)]
pub struct Pinky;

#[derive(Copy, Clone, Component)]
pub struct Inky;

#[derive(Copy, Clone, Component)]
pub struct Clyde;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MovePlugin)
            .add_plugin(TargetPlugin)
            .add_plugin(StatePlugin)
            .add_plugin(SchedulePlugin)
            .add_startup_system(spawn_ghosts)
            .add_system(ghost_passed_tunnel);
    }
}

fn ghost_passed_tunnel(
    mut commands: Commands,
    mut event_reader: EventReader<GhostPassedTunnel>,
    mut query: Query<Entity, With<Ghost>>,
) {
    for event in event_reader.iter() {
        for entity in query.iter_mut() {
            if entity == **event {
                commands.entity(entity).remove::<Target>();
            }
        }
    }
}