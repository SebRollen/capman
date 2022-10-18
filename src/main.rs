use bevy::prelude::*;
use bevy::render::texture::ImageSettings;

use crate::animation::AnimationPlugin;
use crate::background_noise::BackgroundNoisePlugin;
use crate::camera::CameraPlugin;
use crate::capman::CapmanPlugin;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::debug::DebugPlugin;
use crate::edibles::EdiblePlugin;
use crate::game_assets::GameAssetsPlugin;

use crate::game_over_screen::GameOverScreenPlugin;
use crate::ghost_corners::GhostCornersPlugin;
use crate::ghost_house::GhostHousePlugin;
use crate::ghost_house_gate::GhostHouseGatePlugin;
use crate::ghosts::GhostPlugin;
use crate::interactions::InteractionsPlugin;
use crate::level::LevelPlugin;
use crate::life_cycle::GameStatePlugin;
use crate::lives::LivesPlugin;
use crate::map::MapPlugin;
use crate::random::RandomPlugin;
use crate::ready_screen::ReadyScreenPlugin;
use crate::score::ScorePlugin;
use crate::specs_per_level::SpecsPerLevelPlugin;
use crate::speed::SpeedPlugin;
use crate::sprite_sheet::SpriteSheetPlugin;
use crate::tunnels::TunnelPlugin;
use crate::walls::WallsPlugin;

mod animation;
mod background_noise;
mod board_dimensions;
mod camera;
mod capman;
mod common;
mod constants;
mod debug;
mod edibles;
mod game_assets;
mod game_over_screen;
mod ghost_corners;
mod ghost_house;
mod ghost_house_gate;
mod ghosts;
mod interactions;
mod level;
mod life_cycle;
mod lives;
mod map;
mod random;
mod ready_screen;
mod score;
mod specs_per_level;
mod speed;
mod sprite_sheet;
mod tunnels;
mod walls;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "CapMan".to_string(),
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(ImageSettings::default_nearest())
        .add_plugin(GameStatePlugin)
        .add_plugin(GameAssetsPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(WallsPlugin)
        .add_plugin(EdiblePlugin)
        .add_plugin(GhostHousePlugin)
        .add_plugin(GhostCornersPlugin)
        .add_plugin(CapmanPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(GhostPlugin)
        .add_plugin(TunnelPlugin)
        .add_plugin(RandomPlugin)
        .add_plugin(LivesPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(SpeedPlugin)
        .add_plugin(InteractionsPlugin)
        .add_plugin(GhostHouseGatePlugin)
        .add_plugin(SpriteSheetPlugin)
        .add_plugin(ReadyScreenPlugin)
        .add_plugin(GameOverScreenPlugin)
        .add_plugin(SpecsPerLevelPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(BackgroundNoisePlugin)
        .run()
}
