use macroquad::prelude::*;
use crate::modules::text_button::TextButton;
use crate::modules::grid::draw_grid;
use crate::modules::still_image::StillImage;
 use crate::modules::preload_image::TextureManager;
 use crate::modules::preload_image::LoadingScreenOptions;


pub async fn run(tm: TextureManager,_elapsed: f64, game1: i32, game2: i32, game3: i32, playername: String, btnclicks: i32) -> 
(String, TextureManager, f64, i32, i32, i32, String, i32) {
  let btn_exit = TextButton::new(
    1870.0,
    0.0,
    50.0,
    50.0,
    "X",
    RED,
    BLACK,
    50);



      let img_mrplaceholder = StillImage::new(
        "assets/placeholder.png",
        1000.0,  // width
        1000.0,  // height
        500.0,  // x position 
        400.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;


    let img_mrsloth = StillImage::new(
        "assets/sloth.png",
        1000.0,  // width
        1000.0,  // height
        1000.0,  // x position
        200.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let img_mrgreed = StillImage::new(
        "assets/greed.png",
        1000.0,  // width
        1000.0,  // height
        0.0,  // x position
        200.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;




    let btn_play = TextButton::new(50.0, 500.0, 250.0, 50.0, "Play", GREEN, BLACK, 30);
    
    
    
    
    
    
    loop {
        clear_background(WHITE);
       

      

       if btn_play.click() {
            return ("slothtalk".to_string(),tm, _elapsed, game1, game2, game3, playername, btnclicks);
        }
       
img_mrplaceholder.draw();
img_mrsloth.draw();
img_mrgreed.draw();

if btn_exit.click() {
    return ("main".to_string(),tm, _elapsed, game1, game2, game3, playername, btnclicks);
}
        draw_text("Menu", 20.0, 40.0, 30.0, BLACK);
    
        next_frame().await;
    }
}