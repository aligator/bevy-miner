use std::f32::consts::FRAC_2_PI;
use std::time::Duration;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_rapier3d::prelude::*;
use bevy_tnua::builtins::TnuaBuiltinCrouch;
use bevy_tnua::control_helpers::{TnuaCrouchEnforcer, TnuaCrouchEnforcerPlugin};
use bevy_tnua::prelude::*;
use bevy_tnua_rapier3d::{TnuaRapier3dIOBundle, TnuaRapier3dPlugin, TnuaRapier3dSensorShape};
use bevy_voxel_world::prelude::*;

use crate::map::MainWorld;

#[derive(Component, Default)]
pub struct PlayerBody {
    desired_rotation: Quat,
    desired_velocity: Vec3,

    jump: bool,
    crouch: bool,
}

#[derive(Component)]
pub struct PlayerCamera {}

#[derive(Component)]
struct PlayerSpawnTimeout {
    timer: Timer,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TnuaRapier3dPlugin::default(),
            TnuaControllerPlugin::default(),
            TnuaCrouchEnforcerPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_player,
                (
                    toggle_cursor_lock,
                    apply_controls.before(execute_move),
                    apply_mouse.before(execute_move),
                    execute_move.in_set(TnuaUserControlsSystemSet),
                ),
            )
                .chain(),
        );
    }
}

fn setup(mut commands: Commands) {
    // TODO: not sure whats the best way to find out if the world has finished rendering initially...
    commands.spawn(PlayerSpawnTimeout {
        timer: Timer::from_seconds(1.0, TimerMode::Once),
    });
}

pub fn toggle_cursor_lock(
    input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if input.just_pressed(KeyCode::KeyQ) {
        let mut window = windows.single_mut();
        match window.cursor.grab_mode {
            CursorGrabMode::Locked => {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
            _ => {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
        }
    }
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

    commands.entity(world_camera.single()).insert((
        PlayerCamera {},
        // Replace the camera
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 1.0), // For debugging set the camera a bit back to see the collider
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 90.0 * (std::f32::consts::PI / 180.0),
                aspect_ratio: 1.0,
                near: 0.1,
                far: 1000.0,
            }),
            ..default()
        },
    ));

    let mut cmd = commands.spawn((
        Name::new("Player"),
        TransformBundle::from_transform(transform),
        PlayerBody::default(),
    ));
    cmd.add_child(world_camera.single());

    // Add physics to the player
    cmd.insert(RigidBody::Dynamic);
    cmd.insert(Collider::cylinder(0.8, 0.5));
    cmd.insert(TnuaRapier3dIOBundle::default());
    cmd.insert(TnuaControllerBundle::default());
    cmd.insert(TnuaCrouchEnforcer::new(0.5 * Vec3::Y, |cmd| {
        cmd.insert(TnuaRapier3dSensorShape(Collider::cylinder(0.0, 0.5)));
    }));

    // Lock rotation completely, as we rotate manually without physics in first person.
    cmd.insert(
        LockedAxes::ROTATION_LOCKED_X
            | LockedAxes::ROTATION_LOCKED_Z
            | LockedAxes::ROTATION_LOCKED_Y,
    );
}

pub fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut PlayerBody, &Transform)>,
) {
    for (mut body, transform) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            direction += transform.forward().normalize();
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction += transform.back().normalize();
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction += transform.left().normalize()
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction += transform.right().normalize()
        }

        direction = direction.clamp_length_max(1.0);

        body.jump = keyboard.pressed(KeyCode::Space);

        let crouch_buttons = [KeyCode::ShiftLeft, KeyCode::ShiftLeft];
        body.crouch = keyboard.any_pressed(crouch_buttons);

        let mut speed_factor = 3.0;
        if body.crouch {
            speed_factor *= 0.2;
        }
        body.desired_velocity = direction * speed_factor;
    }
}

pub fn apply_mouse(
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    mut player_query: Query<(&mut PlayerBody, &Transform), Without<PlayerCamera>>,
    mut input: EventReader<MouseMotion>,
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };
    let mut mouse_move: Vec2 = -(input.read().map(|motion| &motion.delta).sum::<Vec2>());

    for (mut body, body_transform) in player_query.iter_mut() {
        // Vertical
        let rot = camera_transform.rotation;

        // Ensure the vertical rotation is clamped
        if rot.x > FRAC_2_PI && mouse_move.y.is_sign_positive()
            || rot.x < -FRAC_2_PI && mouse_move.y.is_sign_negative()
        {
            mouse_move.y = 0.0;
        }

        camera_transform.rotate(Quat::from_scaled_axis(rot * Vec3::X * mouse_move.y / 180.0));

        // Horizontal
        let rot = body_transform.rotation;

        let mut new_rotation = *body_transform;
        new_rotation.rotate(Quat::from_scaled_axis(rot * Vec3::Y * mouse_move.x / 180.0));

        body.desired_rotation = new_rotation.rotation;
    }
}

pub fn execute_move(
    mut player_query: Query<(
        &mut TnuaController,
        &mut TnuaCrouchEnforcer,
        &mut Transform,
        &PlayerBody,
    )>,
) {
    for (mut controller, mut crouch_enforcer, mut transform, body) in player_query.iter_mut() {
        if body.jump {
            controller.action(TnuaBuiltinJump {
                height: 1.5,
                fall_extra_gravity: 2.0,
                ..default()
            });
        }

        if body.crouch {
            controller.action(crouch_enforcer.enforcing(TnuaBuiltinCrouch {
                float_offset: -0.2,

                ..default()
            }));
        }

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: body.desired_velocity,
            desired_forward: Vec3::ZERO, // Rotation must be instant in FP - not by physics.

            float_height: 2.0,
            cling_distance: 0.5,
            up: Direction3d::Y,
            spring_strengh: 1000.0,
            spring_dampening: 1.2,
            acceleration: 60.0,
            air_acceleration: 20.0,
            coyote_time: 0.15,
            free_fall_extra_gravity: 60.0,
            tilt_offset_angvel: 5.0,
            tilt_offset_angacl: 500.0,
            turning_angvel: 10.0,
        });
        transform.rotation = body.desired_rotation;
    }
}
