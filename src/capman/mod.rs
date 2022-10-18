use crate::animation::Animations;
use crate::game_assets::loaded_assets::LoadedAssets;
use bevy::prelude::*;

use crate::capman::edible_eaten::EdibleEatenPlugin;
use crate::capman::movement::{move_capman, set_direction_based_on_keyboard_input, InputBuffer};
use crate::capman::spawn::spawn_capman;
use crate::capman::textures::{start_animation, update_capman_appearance};
use crate::life_cycle::LifeCycle::*;

mod edible_eaten;
mod movement;
mod spawn;
mod textures;

/// Marker component for a capman entity.
#[derive(Component)]
pub struct Capman;

/// Fired when capman died.
pub struct ECapmanDead;

pub struct CapmanPlugin;

impl Plugin for CapmanPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ECapmanDead>()
            .add_plugin(EdibleEatenPlugin)
            .insert_resource(InputBuffer(None))
            .add_system_set(SystemSet::on_enter(Ready).with_system(spawn_capman))
            .add_system_set(SystemSet::on_enter(Running).with_system(start_animation))
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(move_capman)
                    .with_system(set_direction_based_on_keyboard_input)
                    .with_system(
                        update_capman_appearance.after(set_direction_based_on_keyboard_input),
                    ),
            )
            .add_system_set(SystemSet::on_enter(CapmanHit).with_system(stop_animation))
            .add_system_set(
                SystemSet::on_enter(CapmanDying)
                    .with_system(play_the_dying_animation)
                    .with_system(play_the_dying_sound),
            )
            .add_system_set(
                SystemSet::on_update(CapmanDying).with_system(check_if_capman_finished_dying),
            )
            .add_system_set(SystemSet::on_enter(CapmanDead).with_system(despawn_capman))
            .add_system_set(SystemSet::on_enter(LevelTransition).with_system(stop_animation))
            .add_system_set(SystemSet::on_exit(LevelTransition).with_system(despawn_capman))
            .add_system_set(SystemSet::on_enter(GhostEatenPause).with_system(set_invisible))
            .add_system_set(SystemSet::on_exit(GhostEatenPause).with_system(set_visible));
    }
}

fn stop_animation(mut query: Query<&mut Animations, With<Capman>>) {
    for mut animations in query.iter_mut() {
        animations.stop()
    }
}

fn play_the_dying_animation(mut query: Query<&mut Animations, With<Capman>>) {
    for mut animations in query.iter_mut() {
        animations.resume();
        animations.change_animation_to("dying")
    }
}

fn play_the_dying_sound(audio: Res<Audio>, loaded_assets: Res<LoadedAssets>) {
    audio.play(loaded_assets.get_handle("sounds/dying.ogg"));
}

fn check_if_capman_finished_dying(
    mut event_writer: EventWriter<ECapmanDead>,
    query: Query<&Animations, With<Capman>>,
) {
    for animations in query.iter() {
        if animations.current().is_completely_finished() {
            event_writer.send(ECapmanDead)
        }
    }
}

fn despawn_capman(mut commands: Commands, query: Query<Entity, With<Capman>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}

fn set_invisible(mut query: Query<&mut Visibility, With<Capman>>) {
    for mut vis in &mut query {
        vis.is_visible = false
    }
}

fn set_visible(mut query: Query<&mut Visibility, With<Capman>>) {
    for mut vis in &mut query {
        vis.is_visible = true
    }
}
