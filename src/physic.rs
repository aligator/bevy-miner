use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicPlugin;
impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec3::new(0.0, -98.0, 0.0),
                ..Default::default()
            });
    }
}
