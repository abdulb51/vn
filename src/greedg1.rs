/*
By: <Abdul Baig>
Date: 2026-06-01
Program Details: crossy roads inspired game
*/
use crate::modules::collision::check_collision;

use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;

use macroquad::prelude::*;


const MOVE_SPEED: f32 = 250.0;

pub async fn run(
    tm: TextureManager,
    _elapsed: f64,
    _game1: i32,
    _game2: i32,
    game3: i32,
    _playername: String,
    btnclicks: i32,
) -> (String, TextureManager, f64, i32, i32, i32, String, i32) {
    let mut _game3 = game3;
    let mut _btnclicks = btnclicks;
    let mut _score = 0;
    let mut img_player = StillImage::new(
        "", 200.0, // width
        200.0, // height
        -50.0, // x position
        300.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    img_player.set_preload(tm.get_preload("assets/placeholder.png").unwrap());

    let mut img_road = StillImage::new(
        "", 350.0,  // width
        1080.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    img_road.set_preload(tm.get_preload("assets/longroad.png").unwrap());

    let mut img_road2 = StillImage::new(
        "", 350.0,  // width
        1080.0, // height
        350.0,  // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    img_road2.set_preload(tm.get_preload("assets/longroad.png").unwrap());

    let mut img_road3 = StillImage::new(
        "", 350.0,  // width
        1080.0, // height
        800.0,  // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    img_road3.set_preload(tm.get_preload("assets/longroad.png").unwrap());

    let mut img_road4 = StillImage::new(
        "", 350.0,  // width
        1080.0, // height
        1150.0, // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    img_road4.set_preload(tm.get_preload("assets/longroad.png").unwrap());

    let mut img_carup1 = StillImage::new(
        "", 300.0, // width
        400.0, // height
        0.0,   // x position
        1480.0,   // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    img_carup1.set_preload(tm.get_preload("assets/redcar.png").unwrap());

    let mut img_carup2 = StillImage::new(
        "", 300.0,  // width
        400.0,  // height
        1150.0, // x position
        1580.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    img_carup2.set_preload(tm.get_preload("assets/redcar.png").unwrap());

    let mut cardown1 = StillImage::new(
        "", 300.0, // width
        400.0, // height
        850.0, // x position
        -400.0,   // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    cardown1.set_preload(tm.get_preload("assets/redcardown.png").unwrap());

    let mut cardown2 = StillImage::new(
        "", 300.0, // width
        400.0, // height
        350.0, // x position
        -400.0,   // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    cardown2.set_preload(tm.get_preload("assets/redcardown.png").unwrap());

    let mut img_bg = StillImage::new(
        "", 1970.0, // width
        1130.0, // height
        -50.0,  // x position
        -50.0,  // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    img_bg.set_preload(tm.get_preload("assets/frame.png").unwrap());

    let mut img_win = StillImage::new(
            "",
            350.0, // width
            350.0, // height
            1550.0,    // x position
            250.0,    // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        )
        .await;
     img_win.set_preload(tm.get_preload("assets/wincon.png").unwrap());

    let mut car1_x = 0.0;
    let mut car1_y = 0.0; // up 1
    let mut car2_x = 0.0;
    let mut car2_y = 0.0; // up 2
    let mut car3_x = 0.0;
    let mut car3_y = 0.0; // down 1


    loop {
        clear_background(DARKGREEN);
        


        
 let mut car1_pos = img_carup1.pos();
        if car1_pos.x == 0.0 && car1_pos.y == 1480.0 {
            car1_x = 0.0;
            car1_y = -10.0;

        }
if car1_pos.x == 0.0 && car1_pos.y == -370.0 {
         car1_pos.y = 1850.0;
    
        }
car1_pos.y += car1_y;
        car1_pos.x += car1_x;
        img_carup1.set_position(car1_pos);
      






        let mut car2_pos = img_carup2.pos();
        if car2_pos.x == 1150.0 && car2_pos.y == 1580.0 {
            car2_x = 0.0;
            car2_y = -10.0;

        }
if car2_pos.x == 0.0 && car2_pos.y == -370.0 {
         car2_pos.y = 1950.0;
    
        }
car2_pos.y += car2_y;
        car2_pos.x += car2_x;
        img_carup2.set_position(car2_pos);

         
         
         let mut car3_pos = cardown1.pos();
         if car3_pos.x == 850.0 && car3_pos.y == -400.0 {
         car3_pos.y = 10.0;
    
        }  if car3_pos.x == 850.0 && car3_pos.y == 1580.0 {
            car3_x = 0.0;
            car3_y = -1980.0;
        }
car3_pos.y += car3_y;
        car3_pos.x += car3_x;
        cardown1.set_position(car3_pos);
      
 

        

        if check_collision(&img_player, &img_carup2, 1) {
            return (
                "g3lose".to_string(),
                tm,
                _elapsed,
                _game1,
                _game2,
                game3,
                _playername.to_string(),
                btnclicks,
            );
        }

        if check_collision(&img_player, &cardown1, 1) {
            return (
                "g3lose".to_string(),
                tm,
                _elapsed,
                _game1,
                _game2,
                game3,
                _playername.to_string(),
                btnclicks,
            );
        }

        if check_collision(&img_player, &cardown2, 1) {
            return (
                "g3lose".to_string(),
                tm,
                _elapsed,
                _game1,
                _game2,
                game3,
                _playername.to_string(),
                btnclicks,
            );
        }

        if check_collision(&img_player, &img_carup1, 1) {
            return (
                "g3lose".to_string(),
                tm,
                _elapsed,
                _game1,
                _game2,
                game3,
                _playername.to_string(),
                btnclicks,
            );
        }
        // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        // Keyboard input
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            move_dir.x += 2.0;
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            move_dir.x -= 2.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            move_dir.y += 2.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            move_dir.y -= 2.0;
        }

        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        // Apply movement based on frame time
        let movement = move_dir * MOVE_SPEED * get_frame_time();

        // Save old position in case of collision
        let old_pos = img_player.pos();

        // Move X first
        if movement.x != 0.0 {
            img_player.set_x(img_player.get_x() + movement.x);
            if check_collision(&img_player, &img_bg, 1) {
                img_player.set_x(old_pos.x); // Undo if collision happens
            }
        }

        // Move Y next
        if movement.y != 0.0 {
            img_player.set_y(img_player.get_y() + movement.y);
            if check_collision(&img_player, &img_bg, 1) {
                img_player.set_y(old_pos.y); // Undo if collision happens
            }
        }


if check_collision(&img_player, &img_win, 1) {
            return (
                "gamewin".to_string(),
                tm,
                _elapsed,
                _game1,
                _game2,
                game3,
                _playername.to_string(),
                btnclicks,
            );}

        img_road.draw();
        img_player.draw();
        img_road2.draw();
        img_road3.draw();
        img_road4.draw();
        img_player.draw();

        img_carup1.draw();
        img_carup2.draw();
        cardown1.draw();
        cardown2.draw();

        img_bg.draw();
        img_win.draw();
        next_frame().await;
    }
}
