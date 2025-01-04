/*
    [
        (x0, y0) (x1, y0)...
        (x0, y1) (x1, y1)...
        ...
    ]
*/
use rand::{Rng, SeedableRng};
use crate::world::World;
use crate::world::block_textures::BlockTypes;

const MAP_WIDTH: u32 = World::MAP_WIDTH;
const MAP_HEIGHT: u32 = World::MAP_HEIGHT;
const MAP_AREA: u32 = MAP_WIDTH * MAP_HEIGHT;

pub fn generate_world() -> Box<[u16; MAP_AREA as usize]> {
    const SEED: [u8; 32] = [1; 32];

    let mut rng: rand::rngs::StdRng = rand::rngs::StdRng::from_seed(SEED);

    let mut world_map: Box<[u16; MAP_AREA as usize]> = Box::new([0u16; MAP_AREA as usize]);

    fn set_block(world_map: &mut Box<[u16; MAP_AREA as usize]>, x: u32, y: u32, block_type: BlockTypes) {
        match world_map.get((x + y * MAP_WIDTH) as usize) {
            Some(_) => world_map[(x + y * MAP_WIDTH) as usize] = block_type as u16,
            None => {
                println!("Error: Write to out of bounds coordinate X:{}, Y:{}", x, y);
            },
        }
    }

    fn get_block(world_map: &Box<[u16; MAP_AREA as usize]>, x: u32, y: u32) -> Result<u16, ()> {
        match world_map.get((x + y * MAP_WIDTH) as usize) {
            Some(&result) => Ok(result),
            None => {
                println!("Error: Read to out of bounds coordinate X:{}, Y:{}", x, y);
                Err(())
            },
        }
    }

    // base ground

    /*for i in 0..100 {
        world_map[(i * MAP_WIDTH) as usize..(i * MAP_WIDTH + MAP_WIDTH) as usize].copy_from_slice(&[BlockTypes::Air as u16; MAP_WIDTH as usize]);
    }

    world_map[(100 * MAP_WIDTH) as usize..(100 * MAP_WIDTH + MAP_WIDTH) as usize].copy_from_slice(&[BlockTypes::Grass as u16; MAP_WIDTH as usize]);
    
    for i in 101..107 {
        world_map[(i * MAP_WIDTH) as usize..(i * MAP_WIDTH + MAP_WIDTH) as usize].copy_from_slice(&[BlockTypes::Dirt as u16; MAP_WIDTH as usize]);
    }

    for i in 107..MAP_HEIGHT {
        world_map[(i * MAP_WIDTH) as usize..(i * MAP_WIDTH + MAP_WIDTH) as usize].copy_from_slice(&[BlockTypes::Stone as u16; MAP_WIDTH as usize]);
    }*/

    // surface shape

    // height range for random surface points, is not equal to maximum and minimum height
    const HEIGHT_POINT_RANGE: std::ops::RangeInclusive<u32> = 75..=125;
    // odd number, determines smoothness of the final surface, higher numbers will make the surface smoother
    const FILTER_WIDTH: u32 = 3;
    // number of random height points including map edges
    const HEIGHT_POINT_COUNT: u32 = 2 + MAP_WIDTH / 150;

    let mut surface_height_map: [u32; MAP_WIDTH as usize] = [0; MAP_WIDTH as usize];

    let mut random_height_points: [u32; HEIGHT_POINT_COUNT as usize] = [0; HEIGHT_POINT_COUNT as usize];
    for i in 0..HEIGHT_POINT_COUNT {
        random_height_points[i as usize] = rng.gen_range(HEIGHT_POINT_RANGE);
    }

    fn filter(x: f32, offset: f32) -> f32 {
        if x == offset {
            1.5
        } else {
            1.5 / (std::f32::consts::PI * (x - offset)) * f32::sin(std::f32::consts::PI * (x - offset))
        }
    }

    for x in 0..MAP_WIDTH {
        let mapped_x: f32 = x as f32 / (MAP_WIDTH - 1) as f32 * (HEIGHT_POINT_COUNT - 1) as f32;
        let interpolated_y: f32 = random_height_points[mapped_x.floor() as usize] as f32 * filter(
            mapped_x.floor(),
            mapped_x,
        );

        let mut filter_value: f32 = 0.0;

        for filter_x in mapped_x.floor() as u32 + 1..mapped_x.floor() as u32 + 1 + (FILTER_WIDTH - 1) / 2 {
            if filter_x < HEIGHT_POINT_COUNT {
                filter_value += random_height_points[filter_x as usize] as f32 * filter(
                    filter_x as f32,
                    mapped_x,
                );
            }
        }
        for filter_x in mapped_x.floor() as i32 - 1 - (FILTER_WIDTH - 1) as i32 / 2..mapped_x.floor() as i32 - 1 {
            if filter_x >= 0 {
                filter_value += random_height_points[filter_x as usize] as f32 * filter(
                    filter_x as f32,
                    mapped_x,
                );
            }
        }

        let surface_y: u32 = (interpolated_y + filter_value).round() as u32; 

        surface_height_map[x as usize] = surface_y;

        set_block(&mut world_map, x, surface_y, BlockTypes::Grass);

        for y in surface_y + 1..surface_y + 6 {
            set_block(&mut world_map, x, y, BlockTypes::Dirt);
        }
        for y in surface_y + 6..MAP_HEIGHT {
            set_block(&mut world_map, x, y, BlockTypes::Stone);
        }
    }
    
    // trees
    const TREE_HEIGHT_RANGE: std::ops::RangeInclusive<u32> = 5..=10;

    let tree_count: u32 = MAP_WIDTH / 20; // number of attempts to create a tree, 1 tree for every 20 blocks
    for _ in 0..tree_count {
        let tree_x: u32 = rng.gen_range(0..=MAP_WIDTH);
        attempt_generate_surface_tree(&mut world_map, tree_x, &mut rng);
    }

    fn get_ground_y_if_clear(world_map: &Box<[u16; MAP_AREA as usize]>, x: u32) -> Option<u32> {
        for y in 0..MAP_HEIGHT {
            let target_block = get_block(&world_map, x, y);

            match target_block {
                Ok(target_block) => {
                    if target_block != BlockTypes::Air as u16 {
                        if target_block == BlockTypes::Grass as u16 {
                            return Some(y);
                        } else {
                            return None;
                        }
                    }
                },
                Err(_) => return None,
            }
        }

        return None;
    }

    fn attempt_generate_surface_tree(world_map: &mut Box<[u16; MAP_AREA as usize]>, tree_x: u32, rng: &mut rand::rngs::StdRng) {
        match get_ground_y_if_clear(world_map, tree_x) {
            Some(tree_base_y) => {
                let tree_height: u32 = rng.gen_range(TREE_HEIGHT_RANGE);

                set_block(world_map, tree_x, tree_base_y, BlockTypes::Dirt);

                for tree_segment_y in 0..tree_height {
                    set_block(world_map, tree_x, tree_base_y - 1 - tree_segment_y, BlockTypes::Log);
                }

                for leaf_y in 0..5 {
                    for i in 0..leaf_y * 2 + 1 {
                        set_block(world_map, tree_x + i - leaf_y, tree_base_y - 5 - tree_height + leaf_y, BlockTypes::Leaves);
                    }
                }
            },
            None => {}
        }
    }

    world_map
}