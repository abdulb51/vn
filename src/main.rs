/*
By: <Abdul baig>
Date: 2026-06-05
Program Details: <its a visual novel, with multiple scenes and mini games>
*/

mod modules;
use crate::modules::grid::draw_grid;
use crate::modules::text_input::TextInput;
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


let lbl




    loop {
        clear_background(WHITE);
        draw_grid(50.0, BLACK);




        next_frame().await;
    }
}
