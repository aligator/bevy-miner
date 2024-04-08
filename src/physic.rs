use crate::map::MainWorld;
use bevy::ecs::world::FilteredEntityRef;
use bevy::prelude::*;
use bevy::utils::info;
use bevy_rapier3d::prelude::*;
use bevy_voxel_world::prelude::{Chunk, ChunkWillRemesh, VoxelWorld};

pub struct PhysicPlugin;
impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec3::new(0.0, -98.0, 0.0),
                ..Default::default()
            })
            .add_systems(
                Update,
                (mark_updated_chunks, generate_chunk_colliders).chain(),
            );
    }
}

#[derive(Component, Default)]
struct NeedsColliderUpdate;

fn mark_updated_chunks(
    mut commands: Commands,
    new_chunk: Query<Entity, Added<Chunk<MainWorld>>>,
    mut event_reader: EventReader<ChunkWillRemesh>,
) {
    // Changed chunks
    for event in event_reader.read() {
        commands.entity(event.entity).insert(NeedsColliderUpdate);
    }

    // New chunks
    for entity in new_chunk.iter() {
        commands.entity(entity).insert(NeedsColliderUpdate);
    }
}

fn generate_chunk_colliders(
    mut commands: Commands,
    updated_chunks: Query<(&Handle<Mesh>, &Chunk<MainWorld>, Entity), With<NeedsColliderUpdate>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (mesh_handle, chunk, entity) in updated_chunks.iter() {
        commands.entity(entity).remove::<NeedsColliderUpdate>();

        let Some(mesh) = meshes.get(mesh_handle) else {
            info!("Failed to get mesh for chunk");
            continue;
        };

        // Generate colliders from the mesh
        if mesh.count_vertices() == 0 {
            info!("Mesh has no vertices");
            continue;
        }
        let Some(collider) = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh) else {
            info!("Failed to generate collider for chunk mesh");
            continue;
        };

        info!("Generated collider for chunk mesh {:?}", chunk.position);

        commands.entity(entity).insert((RigidBody::Fixed, collider));
    }
}
