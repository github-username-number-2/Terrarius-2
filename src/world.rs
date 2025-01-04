mod world_generator;
pub mod block_textures;
pub mod block_data;

use crate::Renderer;
use crate::Player;

pub struct World {
    pub world_map: Box<[u16; (Self::MAP_WIDTH * Self::MAP_HEIGHT) as usize]>,
}

impl World {
    pub const MAP_WIDTH: u32 = 4000;
    pub const MAP_HEIGHT: u32 = 500;

    pub fn update_frame_mesh(&self, player: &Player, mesh: &mut egui::Mesh, window_width: f32, window_height: f32) {
        let texture_pixel_width: f32 = Renderer::BLOCK_PIXEL_COUNT / 5.0;
                
        let screen_block_count_x: f32 = window_width / Renderer::BLOCK_PIXEL_COUNT;
        let screen_block_count_y: f32 = window_height / Renderer::BLOCK_PIXEL_COUNT;

        let player_block_x: i32 = (player.position_x / Renderer::BLOCK_PIXEL_COUNT).trunc() as i32;
        let player_block_y: i32 = (player.position_y / Renderer::BLOCK_PIXEL_COUNT).trunc() as i32;

        for relative_x in (-screen_block_count_x / 2.0).floor() as i32..(screen_block_count_x / 2.0).ceil() as i32 + 1 {
            for relative_y in (-screen_block_count_y / 2.0).floor() as i32..(screen_block_count_y / 2.0).ceil() as i32 + 1 {
                for texture_x in 0..5 {
                    for texture_y in 0..5 {

                        let (block_x, block_y) = (relative_x + player_block_x, relative_y + player_block_y);
                        let block_type: u16 = self.get_block(block_x as u32, block_y as u32);
                        let block_texture: &[[&str; 5]; 5] = block_textures::TEXTURE_MAP[block_type as usize];
                        let block_coordinates: (f32, f32) = (
                            texture_x as f32 * texture_pixel_width + relative_x as f32 * Renderer::BLOCK_PIXEL_COUNT - ((player.position_x / Renderer::BLOCK_PIXEL_COUNT).fract() * Renderer::BLOCK_PIXEL_COUNT - window_width / 2.0).floor(),
                            texture_y as f32 * texture_pixel_width + relative_y as f32 * Renderer::BLOCK_PIXEL_COUNT - ((player.position_y / Renderer::BLOCK_PIXEL_COUNT).fract() * Renderer::BLOCK_PIXEL_COUNT - window_height / 2.0).floor(),
                        );

                        mesh.add_colored_rect(
                            egui::Rect::from_x_y_ranges(
                                block_coordinates.0..=block_coordinates.0 + texture_pixel_width, 
                                block_coordinates.1..=block_coordinates.1 + texture_pixel_width
                            ),
                            match egui::Color32::from_hex(block_texture[texture_y][texture_x]) {
                                Ok(color) => color,
                                Err(_) => {
                                    println!("Invalid HEX string {}", block_texture[texture_y][texture_x]);
                                    egui::Color32::BLACK
                                },
                            },
                        );
                    }
                }
            }
        }
    }

    pub fn get_block(&self, x: u32, y: u32) -> u16 {
        self.world_map[(x + y * Self::MAP_WIDTH) as usize]
    }

    pub fn set_block(&mut self, x: u32, y: u32, block_type: u16) -> () {
        self.world_map[(x + y * Self::MAP_WIDTH) as usize] = block_type;
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            world_map: world_generator::generate_world(),
        }
    }
}