use std::ops::RangeInclusive;
use bevy::prelude::*;
use crate::common::Position;
use crate::constants::{GHOST_SPEED, PACMAN_SPEED};
use crate::ghosts::Ghost;
use crate::ghosts::state::{Frightened, FrightenedTimer};
use crate::level::Level;
use crate::map::board::Board;
use crate::pacman::Pacman;

pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpeedByLevel::new())
            .add_system(update_normal_ghost_speed)
            .add_system(update_frightened_ghost_speed)
            .add_system(update_pacman_speed)
        ;
    }
}

/// The current speed of a moving entity
#[derive(Copy, Clone, Component, Deref, DerefMut)]
pub struct Speed(pub f32);

pub struct SpeedByLevel {
    pacman_ranges: Vec<PacmanLevelRangeSpeed>,
    ghost_ranges: Vec<GhostLevelRangeSpeed>,
}

impl SpeedByLevel {
    /// Creates the default settings for speed by level.
    /// Taken from https://www.gamedeveloper.com/design/the-pac-man-dossier
    pub fn new() -> Self {
        let pacman_ranges = vec![
            PacmanLevelRangeSpeed::new(Level(1)..=Level(1), PacmanSpeed {
                normal: Speed(0.8 * PACMAN_SPEED),
                frightened: Speed(0.9 * PACMAN_SPEED)
            }),
            PacmanLevelRangeSpeed::new(Level(2)..=Level(4), PacmanSpeed {
                normal: Speed(0.9 * PACMAN_SPEED),
                frightened: Speed(0.95 * PACMAN_SPEED)
            }),
            PacmanLevelRangeSpeed::new(Level(5)..=Level(20), PacmanSpeed {
                normal: Speed(1.0 * PACMAN_SPEED),
                frightened: Speed(1.0 * PACMAN_SPEED)
            }),
            PacmanLevelRangeSpeed::new(Level(21)..=Level(usize::MAX), PacmanSpeed {
                normal: Speed(0.9 * PACMAN_SPEED),
                frightened: Speed(0.9 * PACMAN_SPEED)
            }),
        ];

        let ghost_ranges = vec![
            GhostLevelRangeSpeed::new(Level(1)..=Level(1), GhostSpeed {
                normal: Speed(0.75 * GHOST_SPEED),
                frightened: Speed(0.5 * GHOST_SPEED),
                tunnel: Speed(0.4 * GHOST_SPEED)
            }),
            GhostLevelRangeSpeed::new(Level(2)..=Level(4), GhostSpeed {
                normal: Speed(0.85 * GHOST_SPEED),
                frightened: Speed(0.55 * GHOST_SPEED),
                tunnel: Speed(0.45 * GHOST_SPEED)
            }),
            GhostLevelRangeSpeed::new(Level(5)..=Level(20), GhostSpeed {
                normal: Speed(0.95 * GHOST_SPEED),
                frightened: Speed(0.6 * GHOST_SPEED),
                tunnel: Speed(0.5 * GHOST_SPEED)
            }),
            GhostLevelRangeSpeed::new(Level(21)..=Level(usize::MAX), GhostSpeed {
                normal: Speed(0.95 * GHOST_SPEED),
                frightened: Speed(0.95 * GHOST_SPEED),
                tunnel: Speed(0.5 * GHOST_SPEED)
            }),
        ];

        SpeedByLevel {
            pacman_ranges,
            ghost_ranges,
        }
    }

    pub fn for_pacman(&self, level: &Level) -> &PacmanSpeed {
        self.pacman_ranges.iter()
            .find_map(|PacmanLevelRangeSpeed {range, pacman_speed}| match range.contains(level) {
                true => Some(pacman_speed),
                false => None
            })
            .expect("For the given level was no speed assigned")
    }

    pub fn for_ghosts(&self, level: &Level) -> &GhostSpeed {
        self.ghost_ranges.iter()
            .find_map(|GhostLevelRangeSpeed {range, ghost_speed}| match range.contains(level) {
                true => Some(ghost_speed),
                false => None
            })
            .expect("For the given level was no speed assigned")
    }
}

struct PacmanLevelRangeSpeed {
    range: RangeInclusive<Level>,
    pacman_speed: PacmanSpeed
}

impl PacmanLevelRangeSpeed {
    pub fn new(range: RangeInclusive<Level>, pacman_speed: PacmanSpeed) -> Self {
        Self { range, pacman_speed }
    }
}

pub struct PacmanSpeed {
    pub normal: Speed,
    pub frightened: Speed
}

struct GhostLevelRangeSpeed {
    range: RangeInclusive<Level>,
    ghost_speed: GhostSpeed
}

impl GhostLevelRangeSpeed {
    pub fn new(range: RangeInclusive<Level>, ghost_speed: GhostSpeed) -> Self {
        Self { range, ghost_speed }
    }
}

pub struct GhostSpeed {
    pub normal: Speed,
    pub frightened: Speed,
    pub tunnel: Speed
}

fn update_normal_ghost_speed(
    board: Res<Board>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>,
    mut query: Query<(&Position, &mut Speed), (With<Ghost>, Without<Frightened>)>
) {
    for (position, mut speed) in query.iter_mut() {
        let ghost_speed = speed_by_level.for_ghosts(&level);

        if board.position_is_tunnel(&position) {
            *speed = ghost_speed.tunnel;
        } else {
            *speed = ghost_speed.normal
        }
    }
}

fn update_frightened_ghost_speed(
    board: Res<Board>,
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>,
    mut query: Query<(&Position, &mut Speed), (With<Ghost>, With<Frightened>)>
) {
    for (position, mut speed) in query.iter_mut() {
        let ghost_speed = speed_by_level.for_ghosts(&level);

        if board.position_is_tunnel(&position) {
            *speed = ghost_speed.tunnel;
        } else  {
            *speed = ghost_speed.frightened
        }
    }
}

fn update_pacman_speed(
    level: Res<Level>,
    speed_by_level: Res<SpeedByLevel>,
    frightened_timer: Option<Res<FrightenedTimer>>,
    mut query: Query<&mut Speed, With<Pacman>>,
) {
    for mut speed in query.iter_mut() {
        let pacman_speed = speed_by_level.for_pacman(&level);

        if frightened_timer.is_some() {
            *speed = pacman_speed.frightened;
        } else {
            *speed = pacman_speed.normal;
        }
    }
}