use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy::render::camera::Exposure;
use bevy::window::CursorGrabMode;
use bevy_fps_controller::controller::{
    CameraConfig, FpsController, FpsControllerInput, FpsControllerPlugin, LogicalPlayer,
    RenderPlayer,
};
use bevy_rapier3d::prelude::*;
use bevy_voxel_world::prelude::*;

use crate::map::MainWorld;

#[derive(Component)]
struct PlayerSpawnTimeout {
    timer: Timer,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsControllerPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, (spawn_player, (manage_cursor)).chain());
    }
}

fn setup(mut commands: Commands) {
    // TODO: not sure whats the best way to find out if the world has finished rendering initially...
    commands.spawn(PlayerSpawnTimeout {
        timer: Timer::from_seconds(1.0, TimerMode::Once),
    });
}

fn spawn_player(
    mut commands: Commands,
    time: Res<Time>,
    mut player_spawn_timout: Query<&mut PlayerSpawnTimeout>,
    world_camera: Query<Entity, With<VoxelWorldCamera<MainWorld>>>,
    voxel_world: VoxelWorld<MainWorld>,
) {
    let mut timeout = player_spawn_timout.get_single_mut().unwrap();
    timeout.timer.tick(time.delta());

    if !timeout.timer.just_finished() {
        return;
    }

    let spawn = voxel_world
        .get_surface_voxel_at_2d_pos(Vec2::new(6.0, 6.0))
        .unwrap();

    let transform = Transform {
        translation: spawn.0.as_vec3() + Vec3::new(0.0, 2.0, 0.0),
        ..default()
    };

    let logical_entity = commands
        .spawn((
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(transform),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                air_acceleration: 80.0,
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: 0.0,
            radius_scale: 0.75,
        })
        .id();

    commands.entity(world_camera.single()).insert((
        // Replace the camera
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / 5.0,
                ..default()
            }),
            ..default()
        },
        RenderPlayer { logical_entity },
    ));
}

fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut window = window_query.single_mut();
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}
