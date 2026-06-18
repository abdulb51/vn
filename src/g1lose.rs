use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::preload_image::LoadingScreenOptions;
use crate::modules::preload_image::TextureManager;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

pub async fn run(tm: TextureManager,_elapsed: f64, game1: i32, game2: i32, game3: i32, playername: String, btnclicks: i32) -> 
(String, TextureManager, f64, i32, i32, i32, String, i32) {

    let elapsed_time = _elapsed;

    let btn_replay = TextButton::new(625.0, 500.0, 200.0, 60.0, "RETRY", GREEN, BLACK, 30);

    let mut lbl_death = Label::new(
        &format!("Time's Up! You only had {:.1}s", elapsed_time),
        200.0, 200.0, 55
    );
    let mut lbl_2 = Label::new("Better luck next time", 300.0, 290.0, 35);

    lbl_death.with_colors(RED, None);
    lbl_2.with_colors(WHITE, None);

    loop {
        clear_background(DARKGRAY);
        draw_grid(50.0, WHITE);

        if btn_replay.click() {
          return ("slothg1".to_string(), tm, _elapsed, game1, game2, game3, playername.to_string(), btnclicks);
        }
    
        lbl_death.draw();
        lbl_2.draw();
        next_frame().await;
    }
}