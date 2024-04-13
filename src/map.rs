use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_voxel_world::prelude::*;
use noise::{HybridMulti, NoiseFn, Perlin};
use rand::Rng;

#[derive(Resource, Clone, Default)]
pub struct MainWorld;

impl VoxelWorldConfig for MainWorld {
    fn spawning_distance(&self) -> u32 {
        5
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate {
        Box::new(move |_chunk_pos| get_voxel_fn(false))
    }

    fn init_root(&self, mut commands: Commands, root: Entity) {
        commands.entity(root).insert(Name::new("main_world"));
    }
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VoxelWorldPlugin::with_config(MainWorld))
            .add_systems(Startup, setup)
            .add_systems(Update, random_block_modification);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(RandomTime(Timer::new(
        Duration::from_millis(1000),
        TimerMode::Repeating,
    )));
}

fn get_voxel_fn(filter_non_solid: bool) -> Box<dyn FnMut(IVec3) -> WorldVoxel + Send + Sync> {
    // Set up some noise to use as the terrain height map
    let mut noise = HybridMulti::<Perlin>::new(1234);
    noise.octaves = 5;
    noise.frequency = 1.1;
    noise.lacunarity = 2.8;
    noise.persistence = 0.4;

    // We use this to cache the noise value for each y column so we only need
    // to calculate it once per x/z coordinate
    let mut cache = HashMap::<(i32, i32), f64>::new();

    // Then we return this boxed closure that captures the noise and the cache
    // This will get sent off to a separate thread for meshing by bevy_voxel_world
    Box::new(move |pos: IVec3| {
        // Sea level
        if pos.y < 1 {
            // Filter water as it is not really solid.
            if filter_non_solid {
                return WorldVoxel::Air;
            }
            return WorldVoxel::Solid(3);
        }

        let [x, y, z] = pos.as_dvec3().to_array();

        // If y is less than the noise sample, we will set the voxel to solid
        let is_ground = y < match cache.get(&(pos.x, pos.z)) {
            Some(sample) => *sample,
            None => {
                let sample = noise.get([x / 1000.0, z / 1000.0]) * 50.0;
                cache.insert((pos.x, pos.z), sample);
                sample
            }
        };

        if is_ground {
            // Solid voxel of material type 0
            WorldVoxel::Solid(0)
        } else {
            WorldVoxel::Air
        }
    })
}

#[derive(Component)]
struct RandomTime(Timer);

fn random_block_modification(
    mut voxel_world: VoxelWorld<MainWorld>,
    time: Res<Time>,
    mut timer: Query<&mut RandomTime>,
) {
    let mut timer = timer.single_mut();
    timer.0.tick(time.delta());

    // if it finished, despawn the bomb
    if !timer.0.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0..10);
    let z = rng.gen_range(0..10);

    let Some(top_voxel) = voxel_world.get_surface_voxel_at_2d_pos(Vec2::new(x as f32, z as f32))
    else {
        return;
    };

    let y = top_voxel.0.y + 1;

    let pos = IVec3::new(x, y, z);

    println!("Modifying voxel at {:?}", pos);
    voxel_world.set_voxel(pos, WorldVoxel::Solid(rng.gen_range(0..3)))
}
