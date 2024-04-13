mod asset_loader;
mod debug;
mod map;
mod physic;
mod player;
mod schedule;

use crate::asset_loader::AssetLoaderPlugin;
use crate::debug::DebugPlugin;
use crate::map::MapPlugin;
use crate::physic::PhysicPlugin;
use crate::player::PlayerPlugin;
use crate::schedule::SchedulePlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            DebugPlugin,
            AssetLoaderPlugin,
            SchedulePlugin,
            PhysicPlugin,
            MapPlugin,
            PlayerPlugin,
        ))
        .run();
}
