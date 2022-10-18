use crate::animation::{Animation, Animations};
use bevy::prelude::*;

use crate::capman::Capman;
use crate::common::Direction;
use crate::common::Direction::*;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::sprite_sheet::SpriteSheet;

pub(in crate::capman) fn update_capman_appearance(
    mut query: Query<(&Direction, &mut Animations), With<Capman>>,
) {
    for (direction, mut animations) in query.iter_mut() {
        match direction {
            Up => animations.change_animation_to("eating_up"),
            Down => animations.change_animation_to("eating_down"),
            Left => animations.change_animation_to("eating_left"),
            Right => animations.change_animation_to("eating_right"),
        }
    }
}

pub(in crate::capman) fn create_capman_animations(
    game_assets: &LoadedAssets,
    sprite_sheets: &Assets<SpriteSheet>,
) -> Animations {
    Animations::new(
        [
            (
                "eating_left",
                create_eating_animation(
                    game_assets.get_asset("textures/capman/capman_walking_left", sprite_sheets),
                ),
            ),
            (
                "eating_right",
                create_eating_animation(
                    game_assets.get_asset("textures/capman/capman_walking_right", sprite_sheets),
                ),
            ),
            (
                "eating_up",
                create_eating_animation(
                    game_assets.get_asset("textures/capman/capman_walking_up", sprite_sheets),
                ),
            ),
            (
                "eating_down",
                create_eating_animation(
                    game_assets.get_asset("textures/capman/capman_walking_down", sprite_sheets),
                ),
            ),
            (
                "dying",
                create_dying_animation(
                    game_assets.get_asset("textures/capman/capman_dying", sprite_sheets),
                ),
            ),
        ],
        "eating_up",
    )
}

fn create_eating_animation(sheet: &SpriteSheet) -> Animation {
    Animation::from_textures(0.2, true, sheet.images_at(0..4))
}

fn create_dying_animation(sheet: &SpriteSheet) -> Animation {
    Animation::from_textures(1.5, false, sheet.images_at(0..12))
}

/// capman starts with stopped animations. Restart them here
pub(in crate::capman) fn start_animation(mut query: Query<&mut Animations, With<Capman>>) {
    for mut anim in &mut query {
        anim.resume()
    }
}
