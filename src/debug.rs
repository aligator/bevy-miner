use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(FpsCounterPlugin);
    }
}
