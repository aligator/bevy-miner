use bevy::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all()
                - DebugRenderMode::COLLIDER_SHAPES
                - DebugRenderMode::COLLIDER_AABBS,
            ..default()
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FpsCounterPlugin);
    }
}
