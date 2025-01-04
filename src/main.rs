mod renderer;
mod player;
mod world;

use eframe::{egui, CreationContext, NativeOptions};
use renderer::Renderer;
use player::Player;
use world::World;

fn main() -> eframe::Result {
    let content: Box<Content> = Box::<Content>::default();

    let options: NativeOptions = eframe::NativeOptions::default();
    eframe::run_native(
        "Terrarius",
        options,
        Box::new(|_cc: &CreationContext| Ok(content)),
    )
}

struct Content {
    player: Player,
    world: World,
    last_frame_start: std::time::Instant,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            player: Player::default(),
            world: World::default(),
            last_frame_start: std::time::Instant::now(),
        }
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            let delta_time: u32 = self.last_frame_start.elapsed().subsec_millis();
            self.last_frame_start = std::time::Instant::now();

            let window_size: egui::Rect = ctx.input(|i: &egui::InputState| i.screen_rect());
            let window_width: f32 = window_size.max.x;
            let window_height: f32 = window_size.max.y;

            // update stage
            self.player.update(&mut self.world, ctx, window_width, window_height, delta_time);

            // render stage
            let mut mesh: egui::Mesh = egui::Mesh::default();

            self.world.update_frame_mesh(&self.player, &mut mesh, window_width, window_height);
            self.player.update_frame_mesh(&mut mesh, window_width, window_height);

            ui.painter().add(egui::Shape::Mesh(mesh));

            // temp statistics
            let player_block_x: i32 = (self.player.position_x / Renderer::BLOCK_PIXEL_COUNT).trunc() as i32;
            let player_block_y: i32 = (self.player.position_y / Renderer::BLOCK_PIXEL_COUNT).trunc() as i32;
            ui.label(format!("X: {}, Y: {}, Delta time: {:02}", self.player.position_x, self.player.position_y, delta_time));
            ui.label(format!("X: {}, Y: {}, Block: {}", player_block_x, player_block_y, self.world.get_block(player_block_x as u32, player_block_y as u32)));

            ctx.request_repaint();
        });
    }
}
