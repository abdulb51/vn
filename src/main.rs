/*
By: <Abdul baig>
Date: 2026-06-05
Program Details: <its a interactable story game, with multiple scenes and mini games>
*/

mod modules;

mod menu;
mod game;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "vn".to_string(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut current_screen = "menu".to_string();
    let mut last_switch = get_time() - 0.02;



    loop {
     clear_background(WHITE);
     
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "menu" => menu::run().await,
                "game" => game::run().await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}