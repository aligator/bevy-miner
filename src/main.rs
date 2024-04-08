mod asset_loader;
mod camera;
mod debug;
mod map;
mod physic;
mod schedule;

use crate::asset_loader::AssetLoaderPlugin;
use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::map::MapPlugin;
use crate::physic::PhysicPlugin;
use crate::schedule::SchedulePlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            DebugPlugin,
            CameraPlugin,
            AssetLoaderPlugin,
            SchedulePlugin,
            PhysicPlugin,
            MapPlugin,
        ))
        .run();
}
