use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::color::BLACK;
use macroquad::miniquad::window::quit;
use macroquad::prelude::*;

pub async fn run(tm: TextureManager,_elapsed: f64, game1: i32, game2: i32, game3: i32, playername: String, btnclicks: i32) -> 
(String, TextureManager, f64, i32, i32, i32, String, i32) {
    // let img_danny = StillImage::new(
    //     "assets/danny.png",
    //     500.0,
    //     1080.0,
    //     400.0,
    //     70.0,
    //     true,
    //     1.0,
    // )
    // .await;

request_new_screen_size(1920.0, 1080.0);

    let mut game1 = game1;
    let mut btnclicks = btnclicks;


    let btn_back = TextButton::new(625.0, 500.0, 200.0, 60.0, "BACK", GREEN, BLACK, 30);
    let btn_replay = TextButton::new(1025.0, 500.0, 200.0, 60.0, "PLAY AGAIN?", RED, BLACK, 30);
    let lbl_win = Label::new(
        &format!("You caught them all!\nTime: {:.1}s", _elapsed),
        200.0, 200.0, 55
    );

    loop {
        clear_background(DARKGRAY);
        // img_danny.draw();

        lbl_win.draw();
        if btn_replay.click() {
            return ("slothg1".to_string(), tm, 0.0, game1, game2, game3, playername.to_string(), btnclicks);
       
        }
        if btn_back.click() {
          
          
          game1 = 1;
       btnclicks += 1 ; return ("slothtalk".to_string(), tm, _elapsed, game1, game2, game3, playername.to_string(), btnclicks);
    }
           
       
        next_frame().await;
    }
}
