use macroquad::input::KeyCode::Space;
/*
By: <Abdul Baig>
Date: 2026-06-01
Program Details: a game where you have to click space when you are above the rectangle and it gets smaller each time
*/
use crate::modules::collision::check_collision;
use crate::modules::grid::draw_grid;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;







pub async fn run(tm: TextureManager,_elapsed: f64, game1: i32, game2: i32, game3: i32, playername: String, btnclicks: i32) -> 
(String, TextureManager, f64, i32, i32, i32, String, i32) {

let mut game2 = game2;
let mut btnclicks = btnclicks;
let mut score = 0;
 
 
 
 let mut img_player = StillImage::new(
        "",
        300.0,  // width
        300.0,  // height
        -50.0,  // x position 
        300.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await; 
    img_player.set_preload(tm.get_preload("assets/placeholder.png").unwrap());


let mut img_bar = StillImage::new(
        "",
        1000.0,  // width
        1000.0,  // height
        0.0,  // x position 
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
img_bar.set_preload(tm.get_preload("assets/rect1.png").unwrap());

request_new_screen_size(1000.0, 1000.0);

  let mut player_x = 0.0;
    let mut player_y = 0.0;

loop {
clear_background(DARKGRAY);
        draw_grid(50.0, WHITE);


let collision = check_collision(&img_player, &img_bar, 1);

 let mut player_pos = img_player.pos();
        //println!("player position: x:{:.1}, y:{:.1}", player_pos.x, player_pos.y);
        if player_pos.x == -50.0 && player_pos.y == 300.0 {
            player_x = 10.0;
            player_y = 0.0;

        }
if player_pos.x == 1050.0 && player_pos.y == 300.0 {
    
            player_x = -10.0;
            player_y = 0.0;
    
        }
player_pos.y += player_y;
        player_pos.x += player_x;
        img_player.set_position(player_pos);
      

img_player.draw();

img_bar.draw();


if collision && is_key_pressed(Space){
    score += 1;
}
    if score == 1 {
img_bar.set_preload(tm.get_preload("assets/rect2.png").unwrap());

    }

    if score == 2 {
img_bar.set_preload(tm.get_preload("assets/rect3.png").unwrap());

    }

    if score == 3 {
img_bar.set_preload(tm.get_preload("assets/rect4.png").unwrap());

    }

    if score == 4 {
img_bar.set_preload(tm.get_preload("assets/rect5.png").unwrap());

    }    

    if score == 5 {
img_bar.set_preload(tm.get_preload("assets/rect6.png").unwrap());

    }    
    if score == 6 {
        game2 += 1;
       btnclicks += 1;
        return ("slothtalk".to_string(), tm, _elapsed, game1, game2, game3, playername, btnclicks);
    }


next_frame().await;

}
}