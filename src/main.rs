/*
By: <Abdul baig>
Date: 2026-06-05
Program Details: <its a interactable story game, with multiple scenes and mini games>
*/

mod modules;
mod menu;
mod slothtalk;
use macroquad::prelude::*;
use crate::modules::text_button::TextButton;
use crate::modules::preload_image::TextureManager;
    use crate::modules::preload_image::LoadingScreenOptions;
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


let mut tm = TextureManager::new();
tm.preload_with_loading_screen(&["assets/sloth.png", "assets/greed.png","assets/placeholder.png", "assets/parkday.png"], None).await;



    
    loop {
     clear_background(WHITE);
     
        if get_time() - last_switch > 0.01 {
            (current_screen, tm) = match current_screen.as_str() {
                "menu" => menu::run(tm).await,
                "slothtalk" => slothtalk::run(tm).await,
                _ => break,
            };
            
            last_switch = get_time();
        }
        next_frame().await;
    }
}