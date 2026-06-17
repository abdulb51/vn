use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::color::BLACK;
use macroquad::miniquad::window::quit;
use macroquad::prelude::*;

pub async fn run(tm: TextureManager, elapsed: f64, game1: i32, game2: i32, game3: i32) -> (String, TextureManager, f64, i32, i32, i32) {
    let img_danny = StillImage::new(
        "assets/danny.png",
        500.0,
        1080.0,
        400.0,
        70.0,
        true,
        1.0,
    )
    .await;

    let btn_end = TextButton::new(225.0, 500.0, 200.0, 60.0, "END GAME", RED, BLACK, 30);
    let btn_replay = TextButton::new(625.0, 500.0, 200.0, 60.0, "PLAY AGAIN", GREEN, BLACK, 30);
    let lbl_win = Label::new(
        &format!("You caught them all!\nTime: {:.1}s", elapsed),
        200.0, 200.0, 55
    );

    loop {
        clear_background(DARKGRAY);
        img_danny.draw();

        lbl_win.draw();
        if btn_replay.click() {
            return ("game".to_string(), tm, 0.0, game1, game2, game3);
        }
        if btn_end.click() {
            quit();
        }

        next_frame().await;
    }
}
