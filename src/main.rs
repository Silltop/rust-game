use std::env;
mod assets;
use assets::load_assets;
use macroquad::prelude::*;
mod entities;
use entities::Player;
mod camera;
mod weather;
use camera::GameCamera;
use weather::Weather;

struct WorldPlane {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl WorldPlane {
    fn new(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    // pub fn world_to_screen(&self, world_x: f32, world_y: f32) -> (f32, f32) {
    //     let screen_x = (world_x - self.min_x) / (self.max_x - self.min_x) * screen_width();
    //     let screen_y = (world_y - self.min_y) / (self.max_y - self.min_y) * screen_height();
    //     (screen_x, screen_y)
    // }

    // pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
    //     let world_x = screen_x / screen_width() * (self.max_x - self.min_x) + self.min_x;
    //     let world_y = screen_y / screen_height() * (self.max_y - self.min_y) + self.min_y;
    //     (world_x, world_y)
    // }

    pub fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.min_x, self.max_x, self.min_y, self.max_y)
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut weather = Weather::new();
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    println!("PID: {}", std::process::id());
    let assets = load_assets().await;
    let player_texture = assets.get("ship.png").unwrap();
    let terrain_texture = assets.get("terrain.png").unwrap();
    const WORLD_SIZE: (f32, f32) = (500.0, 500.0);
    let world_plane = WorldPlane::new(-WORLD_SIZE.0, WORLD_SIZE.0, -WORLD_SIZE.1, WORLD_SIZE.1);
    let mut player = Player::new(0.0, 0.0);

    let mut game_camera = GameCamera::new(world_plane.get_bounds());

    loop {
        clear_background(BLACK);
        player.update();
        game_camera.update();

        draw_texture(terrain_texture, -250.0, -250.0, WHITE);

        player.draw(player_texture);

        set_default_camera(); // Set camera to default (screen space)

        game_camera.draw_rotation_text();
        weather.update();
        weather.draw(); // Draw weather effects

        let (mouse_x, mouse_y) = mouse_position();
        let cursor_pos_text = format!("x: {:.1}, y: {:.1}", mouse_x, mouse_y);
        let text_size = measure_text(&cursor_pos_text, None, 30, 1.0);
        draw_text(
            &format!("MOUSE POS: {}", cursor_pos_text),
            screen_width() - text_size.width - 150.0,
            screen_height() - 10.0,
            30.0,
            WHITE,
        );

        // Check if mouse is hovering over the player
        if player.is_hovered(mouse_x, mouse_y) {
            draw_tooltip(mouse_x, mouse_y, "Player: This is the player character");
        }

        let key = get_keys_pressed();
        if key.len() > 0 {
            println!("{:?}", key);
        }

        next_frame().await
    }
}

fn draw_tooltip(x: f32, y: f32, text: &str) {
    let padding = 5.0;
    let text_size = measure_text(text, None, 20, 1.0);
    let rect_width = text_size.width + padding * 2.0;
    let rect_height = text_size.height + padding * 2.0;

    draw_rectangle(x, y - rect_height, rect_width, rect_height, DARKGRAY);
    draw_text(
        text,
        x + padding,
        y - rect_height + padding + text_size.height,
        20.0,
        WHITE,
    );
}
