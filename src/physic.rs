use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_voxel_world::prelude::{Chunk, ChunkWillRemesh};

use crate::map::PhysicWorld;

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
                (
                    (mark_new_chunks, mark_updated_chunks),
                    generate_chunk_colliders,
                )
                    .chain(),
            );
    }
}

#[derive(Component, Default)]
struct NeedsColliderUpdate;

fn mark_new_chunks(mut commands: Commands, new_chunk: Query<Entity, Added<Chunk<PhysicWorld>>>) {
    for entity in new_chunk.iter() {
        commands.entity(entity).insert(NeedsColliderUpdate);
    }
}

fn mark_updated_chunks(mut commands: Commands, mut event_reader: EventReader<ChunkWillRemesh<PhysicWorld>>) {
    for event in event_reader.read() {
        commands.entity(event.entity).insert(NeedsColliderUpdate);
    }
}

fn generate_chunk_colliders(
    mut commands: Commands,
    updated_chunks: Query<(&Handle<Mesh>, Entity), With<NeedsColliderUpdate>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (mesh_handle, entity) in updated_chunks.iter() {
        commands.entity(entity).remove::<NeedsColliderUpdate>();

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

        commands.entity(entity).insert((RigidBody::Fixed, collider));
    }
}
