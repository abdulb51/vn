/*
By: <Abdul baig>
Date: 2026-06-05
Program Details: <its a interactable story game, with multiple scenes and mini games>
*/

mod modules;
mod menu;
mod slothtalk;
mod slothg1;
mod wing1;
mod g1lose;
mod slothg2;
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
        window_resizable: false,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    
    let mut last_switch = get_time() - 0.02;
    let mut game1 = 0;
    let mut game2 = 0;
    let mut game3 = 0;
let mut elapsed = 0.0;
let mut btnclicks = 0;
let mut tm = TextureManager::new();
let mut playername = String::new();

tm.preload_with_loading_screen(&["assets/sloth.png", "assets/greed.png","assets/placeholder.png",
 "assets/parkday.png", "assets/maze.png", "assets/rect1.png", "assets/rect2.png","assets/rect3.png","assets/rect4.png","assets/rect5.png","assets/rect6.png"], None).await;


let mut current_screen = "menu".to_string();
    
    loop {
     clear_background(WHITE);
     
        if get_time() - last_switch > 0.01 {
            (current_screen, tm, elapsed, game1, game2, game3, playername, btnclicks) = match current_screen.as_str() {
                "menu" => menu::run(tm, elapsed, game1, game2, game3, playername, btnclicks).await,
                "slothtalk" => slothtalk::run(tm, elapsed, game1, game2, game3, playername, btnclicks).await,
                "slothg1" => slothg1::run(tm, elapsed, game1, game2, game3, playername, btnclicks).await,
                "wing1" => wing1::run(tm, elapsed, game1, game2, game3, playername, btnclicks).await,
                "g1lose" => g1lose::run(tm, elapsed, game1, game2, game3, playername, btnclicks).await,
                "slothg2" => slothg2::run(tm, elapsed, game1, game2, game3, playername, btnclicks).await,
                _ => break,
            };
            
            last_switch = get_time();
        }
        next_frame().await;
    }
}