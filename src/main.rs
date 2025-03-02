use std::env;
mod assets;
use assets::load_assets;
use macroquad::prelude::*;
mod entities;
use entities::Player;

mod weather;
use weather::Weather;

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
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);

    loop {
        clear_background(RED);
        draw_texture(terrain_texture, 0.0, 0.0, WHITE);
        weather.update();
        weather.draw();
        player.update();
        player.draw(player_texture);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_text("TEST", 20.0, 20.0, 30.0, DARKGRAY);
        let key = get_keys_pressed();
        if key.len() > 0 {
            println!("{:?}", key);
        }
        next_frame().await
    }
}
