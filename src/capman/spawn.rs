use crate::board_dimensions::BoardDimensions;
use crate::common::Direction::*;
use bevy::prelude::*;

use crate::capman::textures::create_capman_animations;
use crate::capman::Capman;
use crate::constants::CAPMAN_Z;
use crate::game_assets::loaded_assets::LoadedAssets;
use crate::is;
use crate::level::Level;
use crate::map::Element::CapManSpawn;
use crate::map::Map;
use crate::specs_per_level::SpecsPerLevel;
use crate::speed::Speed;
use crate::sprite_sheet::SpriteSheet;

/// Resource that tells at which position capman spawns.
#[derive(Deref, DerefMut)]
pub struct CapmanSpawn(Vec3);

pub(in crate::capman) fn spawn_capman(
    mut commands: Commands,
    game_assets: Res<LoadedAssets>,
    sprite_sheets: Res<Assets<SpriteSheet>>,
    map: Res<Map>,
    level: Res<Level>,
    specs_per_level: Res<SpecsPerLevel>,
    dimensions: Res<BoardDimensions>,
) {
    let transform =
        dimensions.positions_to_trans(map.get_positions_matching(is!(CapManSpawn)), CAPMAN_Z);
    let dimension = Vec2::new(dimensions.capman(), dimensions.capman());

    let mut animations = create_capman_animations(&game_assets, &sprite_sheets);
    animations.stop();

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: animations.current().texture(),
            sprite: Sprite {
                custom_size: Some(dimension),
                ..default()
            },
            transform,
            ..Default::default()
        })
        .insert(Capman)
        .insert(Speed(
            dimensions.capman_base_speed()
                * specs_per_level.get_for(&level).capman_normal_speed_modifier,
        ))
        .insert(Up)
        .insert(animations);
}
