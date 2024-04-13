use crate::map::MainWorld;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_voxel_world::prelude::{VoxelWorld, VoxelWorldCamera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_camera)
            .add_systems(Update, set_camera_to_map);
    }
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        // Camera3dBundle {
        //     transform: Transform::from_xyz(-200.0, 150.0, -200.0)
        //         .looking_at(Vec3::new(-100.0, 0.0, -100.0), Vec3::Y),
        //     ..default()
        // },
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(Vec3::new(-100.0, 0.0, -100.0), Vec3::Y),
            ..default()
        },
        // This tells bevy_voxel_world to use this cameras transform to calculate spawning area
        VoxelWorldCamera::<MainWorld>::default(),
    ));

    // Sun
    let cascade_shadow_config = CascadeShadowConfigBuilder { ..default() }.build();
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.1, 0.15), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });

    // Ambient light, same color as sun
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.98, 0.95, 0.82),
        brightness: 100.0,
    });
}

fn set_camera_to_map(
    mut query: Query<&mut Transform, With<VoxelWorldCamera<MainWorld>>>,
    voxel_world: VoxelWorld<MainWorld>,
) {
    for mut transform in query.iter_mut() {
        let Some(spawn) = voxel_world.get_surface_voxel_at_2d_pos(Vec2::new(-10.0, -10.0)) else {
            return;
        };

        let Some(look_at) = voxel_world.get_surface_voxel_at_2d_pos(Vec2::new(10.0, 10.0)) else {
            return;
        };

        transform.translation = spawn.0.as_vec3() + Vec3::new(0.0, 10.0, 0.0);
        transform.look_at(look_at.0.as_vec3(), Vec3::Y);
    }
}

fn move_camera(
    time: Res<Time>,
    mut cam_transform: Query<&mut Transform, With<VoxelWorldCamera<MainWorld>>>,
) {
    cam_transform.single_mut().translation.x += time.delta_seconds() * 30.0;
    cam_transform.single_mut().translation.z += time.delta_seconds() * 60.0;
}
