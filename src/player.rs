use eframe::egui;
use crate::world::block_textures::BlockTypes;
use crate::world::block_data::SOLIDS;
use crate::renderer::Renderer;
use crate::World;

pub struct Player {
    pub position_x: f32,
    pub velocity_x: f32,
    pub position_y: f32,
    pub velocity_y: f32,
}

impl Player {
    pub const MAX_X_VELOCITY: f32 = 0.2;
    pub const MAX_Y_VELOCITY: f32 = 2.0;
    //pub const MAX_X_VELOCITY: f32 = 10.0;
    //pub const MAX_Y_VELOCITY: f32 = 10.0;

    pub const MOVEMENT_ACCELERATION: f32 = 0.01;
    //pub const MOVEMENT_ACCELERATION: f32 = 0.1;
    pub const JUMP_VELOCITY: f32 = 0.45;
    //pub const JUMP_VELOCITY: f32 = 1.0;

    pub const WIDTH: f32 = 40.0;
    pub const HEIGHT: f32 = 86.0;

    pub fn update_frame_mesh(&self, mesh: &mut egui::Mesh, window_width: f32, window_height: f32) {
        mesh.add_colored_rect(
            egui::Rect::from_x_y_ranges((window_width / 2.0).ceil()..=(window_width / 2.0 + Player::WIDTH).ceil(), (window_height / 2.0).ceil()..=(window_height / 2.0 + Player::HEIGHT).ceil()),
            match egui::Color32::from_hex("#ff00ff") {
                Ok(color) => color,
                Err(_) => {
                    println!("Invalid HEX string");
                    egui::Color32::BLACK
                },
            },
        );
    }

    pub fn update(&mut self, world: &mut World, ctx: &egui::Context, window_width: f32, window_height: f32, delta_time: u32) -> () {
        if ctx.input(|i| i.pointer.primary_clicked()) {
            let click_position: Option<egui::Pos2> = ctx.input(|i| i.pointer.interact_pos());
            match click_position {
                Some(click_position) => {
                    let (block_x, block_y) = (((self.position_x + click_position.x - window_width / 2.0) / Renderer::BLOCK_PIXEL_COUNT).floor() as i32, ((self.position_y + click_position.y - window_height / 2.0) / Renderer::BLOCK_PIXEL_COUNT).floor() as i32);
                    world.set_block(block_x as u32, block_y as u32, BlockTypes::Air as u16);
                }
                None => {}
            }
        }
        if ctx.input(|i| i.pointer.secondary_clicked()) {
            let click_position: Option<egui::Pos2> = ctx.input(|i| i.pointer.interact_pos());
            match click_position {
                Some(click_position) => {
                    let (block_x, block_y) = (((self.position_x + click_position.x - window_width / 2.0) / Renderer::BLOCK_PIXEL_COUNT).floor() as i32, ((self.position_y + click_position.y - window_height / 2.0) / Renderer::BLOCK_PIXEL_COUNT).floor() as i32);
                    world.set_block(block_x as u32, block_y as u32, BlockTypes::Stone as u16);
                }
                None => {}
            }
        }

        if ctx.input(|i| i.key_pressed(egui::Key::W)) {
            let player_bottom_edge: f32 = (self.position_y + Player::HEIGHT) / Renderer::BLOCK_PIXEL_COUNT;
            let block_top_edge: f32 = player_bottom_edge.ceil();

            if block_top_edge == player_bottom_edge {
                let left_edge: f32 = (self.position_x / Renderer::BLOCK_PIXEL_COUNT).trunc();
                let right_edge: f32 = ((self.position_x + Player::WIDTH) / Renderer::BLOCK_PIXEL_COUNT).ceil() - 1.0;

                for block_x in left_edge as u32..=right_edge as u32 {
                    let can_jump: bool = SOLIDS.contains(&world.get_block(block_x, block_top_edge as u32));

                    if can_jump {
                        self.velocity_y = -Player::JUMP_VELOCITY;

                        break;
                    }
                }
            }
        }
        if ctx.input(|i| i.key_down(egui::Key::A)) {
            self.velocity_x -= Player::MOVEMENT_ACCELERATION;
        }
        if ctx.input(|i| i.key_down(egui::Key::S)) {
            //
        }
        if ctx.input(|i| i.key_down(egui::Key::D)) {
            self.velocity_x += Player::MOVEMENT_ACCELERATION;
        }

        self.velocity_x *= 0.90; 
        self.velocity_y += 0.01;

        self.velocity_x = self.velocity_x.clamp(-Player::MAX_X_VELOCITY, Player::MAX_X_VELOCITY);
        self.velocity_y = self.velocity_y.clamp(-Player::MAX_Y_VELOCITY, Player::MAX_Y_VELOCITY);            

        let mut new_position_x: f32 = self.position_x + self.velocity_x * delta_time as f32;
        if self.velocity_x > 0.0 {
            let right_edge: f32 = new_position_x + Player::WIDTH;
            let right_x: u32 = (right_edge / Renderer::BLOCK_PIXEL_COUNT).floor() as u32;
            let top_edge: f32 = (self.position_y / Renderer::BLOCK_PIXEL_COUNT).trunc();
            let bottom_edge: f32 = ((self.position_y + Player::HEIGHT) / Renderer::BLOCK_PIXEL_COUNT).ceil() - 1.0;

            for block_y in top_edge as u32..=bottom_edge as u32 {
                let is_solid: bool = SOLIDS.contains(&world.get_block(right_x, block_y));

                if is_solid {
                    self.velocity_x = 0.0;
                    new_position_x = (right_x as f32 * Renderer::BLOCK_PIXEL_COUNT).ceil() - Player::WIDTH;

                    break;
                }
            }
        } else {
            let left_edge: f32 = new_position_x;
            let left_x: u32 = (left_edge / Renderer::BLOCK_PIXEL_COUNT).floor() as u32;
            let top_edge: f32 = (self.position_y / Renderer::BLOCK_PIXEL_COUNT).trunc();
            let bottom_edge: f32 = ((self.position_y + Player::HEIGHT) / Renderer::BLOCK_PIXEL_COUNT).ceil() - 1.0;

            for block_y in top_edge as u32..=bottom_edge as u32 {
                let is_solid: bool = SOLIDS.contains(&world.get_block(left_x, block_y));

                if is_solid {
                    self.velocity_x = 0.0;
                    new_position_x = ((left_x + 1) as f32 * Renderer::BLOCK_PIXEL_COUNT).ceil();

                    break;
                }
            }
        }

        self.position_x = new_position_x;

        let mut new_position_y: f32 = self.position_y + self.velocity_y * delta_time as f32;
        if self.velocity_y > 0.0 {
            let bottom_edge: f32 = new_position_y + Player::HEIGHT;
            let bottom_y: u32 = (bottom_edge / Renderer::BLOCK_PIXEL_COUNT).floor() as u32;
            let left_edge: f32 = (self.position_x / Renderer::BLOCK_PIXEL_COUNT).trunc();
            let right_edge: f32 = ((self.position_x + Player::WIDTH) / Renderer::BLOCK_PIXEL_COUNT).ceil() - 1.0;

            for block_x in left_edge as u32..=right_edge as u32 {
                let is_solid: bool = SOLIDS.contains(&world.get_block(block_x, bottom_y));

                if is_solid {
                    self.velocity_y = 0.0;
                    new_position_y = (bottom_y as f32 * Renderer::BLOCK_PIXEL_COUNT).ceil() - Player::HEIGHT;

                    break;
                }
            }
        } else {
            let top_edge: f32 = new_position_y;
            let top_y: u32 = (top_edge / Renderer::BLOCK_PIXEL_COUNT).floor() as u32;
            let left_edge: f32 = (self.position_x / Renderer::BLOCK_PIXEL_COUNT).trunc();
            let right_edge: f32 = ((self.position_x + Player::WIDTH) / Renderer::BLOCK_PIXEL_COUNT).ceil() - 1.0;

            for block_x in left_edge as u32..=right_edge as u32 {
                let is_solid: bool = SOLIDS.contains(&world.get_block(block_x, top_y));

                if is_solid {
                    self.velocity_y = 0.0;
                    new_position_y = ((top_y + 1) as f32 * Renderer::BLOCK_PIXEL_COUNT).ceil();

                    break;
                }
            }
        }

        self.position_y = new_position_y;
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position_x: World::MAP_WIDTH as f32 / 2.0 * Renderer::BLOCK_PIXEL_COUNT,
            velocity_x: 0.0,
            position_y: 50.0 * Renderer::BLOCK_PIXEL_COUNT,
            velocity_y: 0.0,
        }
    }
}