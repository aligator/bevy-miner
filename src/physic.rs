use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_voxel_world::prelude::{Chunk, ChunkWillRemesh, VoxelWorld};

use crate::map::{MainWorld, PhysicWorld};

pub struct PhysicPlugin;
impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec3::new(0.0, -98.0, 0.0),
                ..Default::default()
            })
            .add_systems(Update, generate_chunk_colliders);
    }
}

fn generate_chunk_colliders(
    mut commands: Commands,
    updated_chunks: Query<(&Handle<Mesh>, &Chunk<PhysicWorld>), Changed<Handle<Mesh>>>,
    meshes: Res<Assets<Mesh>>,
    main_world: VoxelWorld<MainWorld>,
) {
    for (mesh_handle, chunk) in updated_chunks.iter() {
        commands.entity(chunk.entity).insert(Visibility::Hidden);

        let Some(mesh) = meshes.get(mesh_handle) else {
            continue;
        };

        // Generate colliders from the mesh
        if mesh.count_vertices() == 0 {
            continue;
        }
        let Some(collider) = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh) else {
            continue;
        };

        let Some(main_chunk) = main_world.get_chunk(chunk.position) else {
            continue;
        };
        
        commands
            .entity(main_chunk.entity)
            .insert((RigidBody::Fixed, collider));
    }
}
