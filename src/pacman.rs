use bevy::prelude::*;

use crate::board::Board;
use crate::common::{Direction, Position};

pub struct PacmanPlugin;

impl Plugin for PacmanPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_pacman.system())
            .add_system(set_direction.system())
            .add_system(move_pacman.system());
    }
}

struct Pacman {
    movement: Movement
}

#[derive(Debug)]
enum Movement {
    Idle,
    Moving(Direction),
}

fn spawn_pacman(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, board: Res<Board>) {
    let start_position = Position::new(1, 1);
    let pacman_dimension = Vec2::new(20.0, 20.0);
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::hex("FFEE00").unwrap().into()),
            transform: Transform::from_translation(board.window_coordinates(&start_position)),
            sprite: Sprite::new(pacman_dimension),
            ..Default::default()
        })
        .with(Pacman { movement: Movement::Idle })
        .with(start_position);
}

fn set_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Pacman>) {
    for mut pacman in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pacman.movement = Movement::Moving(Direction::Left)
        }

        if keyboard_input.pressed(KeyCode::Right) {
            pacman.movement = Movement::Moving(Direction::Right)
        }

        if keyboard_input.pressed(KeyCode::Up) {
            pacman.movement = Movement::Moving(Direction::Up)
        }

        if keyboard_input.pressed(KeyCode::Down) {
            pacman.movement = Movement::Moving(Direction::Down)
        }
    }
}

fn move_pacman(time: Res<Time>, board: Res<Board>, mut query: Query<(&mut Pacman, &Sprite, &mut Position, &mut Transform)>) {
    for (mut pacman, sprite, mut position, mut transform) in query.iter_mut() {
        let direction = match pacman.movement {
            Movement::Idle => return,
            Movement::Moving(dir) => dir
        };

        let translation = &mut transform.translation;
        *position = board.calculate_position(translation);
        move_in_direction(&direction, translation, time.delta_seconds);

        if board.collides_with_obstacle(&position, &direction, translation, &sprite.size) {
            process_collision(&board, &position, &direction, translation, &mut pacman)
        } else {
            center_position(&board, translation, &position, &direction)
        }
    }
}

fn move_in_direction(direction: &Direction, translation: &mut Vec3, delta_seconds: f32) {
    let speed = 250.0;
    let (x, y) = get_modifiers_for_direction(direction);
    *translation.x_mut() += delta_seconds * x * speed;
    *translation.y_mut() += delta_seconds * y * speed;
}

fn get_modifiers_for_direction(direction: &Direction) -> (f32, f32) {
    match direction {
        Direction::Up => (0.0, 1.0),
        Direction::Down => (0.0, -1.0),
        Direction::Left => (-1.0, 0.0),
        Direction::Right => (1.0, 0.0)
    }
}

fn process_collision(board: &Board, position: &Position, direction: &Direction, translation: &mut Vec3, pacman: &mut Pacman) {
    let border_coordinates = board.window_coordinates(&position);
    limit_movement(direction, &border_coordinates, translation);
    stop_if_at_border(direction, &border_coordinates, translation, pacman)
}

fn limit_movement(direction: &Direction, border_coordinates: &Vec3, translation: &mut Vec3) {
    match direction {
        Direction::Up => *translation.y_mut() = translation.y().min(border_coordinates.y()),
        Direction::Down => *translation.y_mut() = translation.y().max(border_coordinates.y()),
        Direction::Left => *translation.x_mut() = translation.x().max(border_coordinates.x()),
        Direction::Right => *translation.x_mut() = translation.x().min(border_coordinates.x())
    }
}

fn stop_if_at_border(direction: &Direction, border_coordinates: &Vec3, translation: &mut Vec3, pacman: &mut Pacman) {
    match direction {
        Direction::Up | Direction::Down => if border_coordinates.y() == translation.y() {
            pacman.movement = Movement::Idle
        }
        ,
        Direction::Left | Direction::Right => if border_coordinates.x() == translation.x() {
            pacman.movement = Movement::Idle
        }
    }
}

fn center_position(board: &Board, translation: &mut Vec3, position: &Position, direction: &Direction) {
    let position_coordinates = board.window_coordinates(position);
    match direction {
        Direction::Up | Direction::Down => *translation.x_mut() = position_coordinates.x(),
        Direction::Left | Direction::Right => *translation.y_mut() = position_coordinates.y()
    }
}