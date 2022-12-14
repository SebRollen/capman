use std::fmt::Formatter;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::Direction;
use crate::common::Direction::*;

/// A position describes the index of a field on the map. It is used for
/// collision checks, target settings and entity spawns.
///
/// It is not a component, as this would lead to necessary synchronization
/// between Transform and Position when any of those gets updated.
#[derive(Copy, Clone, Deserialize, Hash, Debug, Eq, PartialEq, Serialize)]
pub struct Position {
    pub x: isize,
    pub y: isize
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Position {x, y}
    }

    /// Returns the distance between two positions.
    pub fn distance_to(&self, other: &Position) -> isize {
        let x_diff = match self.x < other.x {
            true => other.x - self.x,
            false => self.x - other.x
        };
        let y_diff = match self.y < other.y {
            true => other.y - self.y,
            false => self.y - other.y
        };
        x_diff.pow(2) + y_diff.pow(2)
    }

    pub fn neighbour_position(&self, direction: &Direction) -> Position {
        match direction {
            Up => Position::new(self.x, self.y + 1),
            Down => Position::new(self.x, self.y - 1),
            Left => Position::new(self.x - 1, self.y),
            Right => Position::new(self.x + 1, self.y),
        }
    }

    pub fn get_neighbour_in_direction(&self, direction: &Direction) -> Neighbour {
        match direction {
            Up => Neighbour::new(Position::new(self.x, self.y + 1), *direction),
            Down => Neighbour::new(Position::new(self.x, self.y - 1), *direction),
            Left => Neighbour::new(Position::new(self.x - 1, self.y), *direction),
            Right => Neighbour::new(Position::new(self.x + 1, self.y), *direction),
        }
    }

    pub fn neighbour_behind(&self, direction: &Direction) -> Neighbour {
        self.get_neighbour_in_direction(&direction.opposite())
    }

    /// Return the direction where to find the other position when neighbored.
    /// If not neighbored, return None.
    pub fn get_neighbour_direction(&self, other: &Position) -> Option<Direction> {
        self.get_neighbours()
            .into_iter()
            .filter(|n| &n.position == other)
            .map(|n| n.direction)
            .next()
    }

    pub fn get_neighbours(&self) -> [Neighbour; 4] {
        [
            self.get_neighbour_in_direction(&Up),
            self.get_neighbour_in_direction(&Down),
            self.get_neighbour_in_direction(&Left),
            self.get_neighbour_in_direction(&Right),
        ]
    }

    pub fn get_nearest_position_from<'a, I: Into<Position>>(&self, iter: impl IntoIterator<Item=I>) -> Position {
        iter.into_iter()
            .map(Into::into)
            .min_by(|pos_0, pos_1| self.distance_to(pos_0).cmp(&self.distance_to(pos_1)))
            .expect("The given iterator of positions should not be empty!")
    }

    pub fn get_position_in_direction_with_offset(&self, direction: &Direction, offset: usize) -> Self {
        match direction {
            Up => Position::new(self.x, self.y + (offset as isize)),
            Down => Position::new(self.x, self.y - (offset as isize)),
            Left => Position::new(self.x - (offset as isize), self.y),
            Right => Position::new(self.x + (offset as isize), self.y)
        }
    }
}

impl From<&Position> for Position {
    fn from(pos: &Position) -> Self {
        *pos
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone)]
pub struct Neighbour {
    pub position: Position,
    pub direction: Direction
}

impl Neighbour {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self { position, direction  }
    }
}

/// Provides helper methods for working with coordinates (Vec3).
///
/// The games logic widely uses positions to perform specific checks (like collisions and distance calculations).
/// These methods aim to make this easier.
pub trait Vec3Helper {
    fn set_xy(&mut self, target: &Vec3);
}

impl Vec3Helper for Vec3 {
    /// Set this coordinates x and y to the ones of the other transform.
    /// The z value defines what is rendered before or after another sprite,
    /// so this value should not be changed.
    fn set_xy(&mut self, target: &Vec3) {
        self.x = target.x;
        self.y = target.y;
    }
}