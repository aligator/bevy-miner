use std::sync::Arc;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_voxel_world::{mesh, prelude::*};

use crate::map::{MainWorld, WATER};

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
    updated_chunks: Query<&Chunk<MainWorld>, Changed<Handle<Mesh>>>,
    main_world: VoxelWorld<MainWorld>,
) {
    for chunk in updated_chunks.iter() {
        println!("Generating collider for chunk {:?}", chunk.position);
        // Generate colliders from the mesh

        let Some(chunk_data) = main_world.get_chunk(chunk.position) else {
            continue;
        };

        // Filter out voxels wich do not need a collider.
        let mut filtered_voxels = *chunk_data.voxels.unwrap().clone();
        filtered_voxels.iter_mut().for_each(|voxel| match voxel {
            WorldVoxel::Solid(WATER) => *voxel = WorldVoxel::Air,
            WorldVoxel::Solid(_) => (),
            WorldVoxel::Unset => (),
            WorldVoxel::Air => (),
        });

        let physic_mesh = mesh::generate_chunk_mesh(
            Arc::new(filtered_voxels),
            chunk.position,
            Arc::new(|_| [0, 0, 0]),
        );

        if physic_mesh.count_vertices() == 0 {
            continue;
        }

        let Some(collider) =
            Collider::from_bevy_mesh(&physic_mesh, &ComputedColliderShape::TriMesh)
        else {
            continue;
        };

        commands
            .entity(chunk.entity)
            .insert((RigidBody::Fixed, collider));
    }
}
