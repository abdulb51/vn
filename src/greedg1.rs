
/*
By: <Abdul Baig>
Date: 2026-06-01
Program Details: crossy roads inspired game
*/
use macroquad::input::KeyCode::Space;
use macroquad::rand::rand;
use crate::modules::collision::check_collision;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use macroquad::prelude::KeyCode::Right;
use macroquad::prelude::*;







pub async fn run(tm: TextureManager,_elapsed: f64, game1: i32, game2: i32, game3: i32, playername: String, btnclicks: i32) -> 
(String, TextureManager, f64, i32, i32, i32, String, i32) {

let mut _game3 = game3;
let mut _btnclicks = btnclicks;
let mut _score = 0;
 
 let mut img_road = StillImage::new(
        "",
        350.0, // width
        1080.0, // height
        0.0, // x position
        0.0, // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
 img_road.set_preload(tm.get_preload("assets/longroad.png").unwrap());


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



 let mut img_road2 = StillImage::new(
        "",
        350.0, // width
        1080.0, // height
        350.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
 img_road2.set_preload(tm.get_preload("assets/longroad.png").unwrap());


 let mut img_road3 = StillImage::new(
        "",
        350.0, // width
        1080.0, // height
        800.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
 img_road3.set_preload(tm.get_preload("assets/longroad.png").unwrap());


 let mut img_road4 = StillImage::new(
        "",
        350.0, // width
        1080.0, // height
        1150.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
 img_road4.set_preload(tm.get_preload("assets/longroad.png").unwrap());

let mut img_carup1 = StillImage::new(
        "",
        300.0, // width
        400.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    img_carup1.set_preload(tm.get_preload("assets/redcar.png").unwrap());

    let mut img_carup2 = StillImage::new(
        "",
        300.0, // width
        400.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    img_carup2.set_preload(tm.get_preload("assets/redcar.png").unwrap());

let mut cardown1 = StillImage::new(
        "",
        300.0, // width
        400.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    cardown1.set_preload(tm.get_preload("assets/redcardown.png").unwrap());

let mut cardown2 = StillImage::new(
        "",
        300.0, // width
        400.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    cardown2.set_preload(tm.get_preload("assets/redcardown.png").unwrap());

// img_win = StillImage::new(
//         "",
//         350.0, // width
//         1080.0, // height
//         0.0,    // x position
//         0.0,    // y position
//         true,   // Enable stretching
//         1.0,    // Normal zoom (100%)
//     )
//     .await;
//  img_road.set_preload(tm.get_preload("assets/.png").unwrap());


loop {
clear_background(DARKGREEN);
        draw_grid(50.0, WHITE);





img_road.draw();
img_player.draw();
img_road2.draw();
img_road3.draw();
img_road4.draw();
img_player.draw();
img_carup1.draw();

//img_win.draw();
next_frame().await;

}
}