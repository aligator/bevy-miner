use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_voxel_world::prelude::VoxelWorldCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_camera)
            .add_systems(Update, move_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-200.0, 150.0, -200.0).looking_at(Vec3::new(-100.0, 0.0, -100.0), Vec3::Y),
            ..default()
        },
        // This tells bevy_voxel_world tos use this cameras transform to calculate spawning area
        VoxelWorldCamera,
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

fn move_camera(time: Res<Time>, mut cam_transform: Query<&mut Transform, With<VoxelWorldCamera>>) {
    cam_transform.single_mut().translation.x += time.delta_seconds() * 30.0;
    cam_transform.single_mut().translation.z += time.delta_seconds() * 60.0;
}
