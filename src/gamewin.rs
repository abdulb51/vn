use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

pub async fn run(tm: TextureManager,_elapsed: f64, game1: i32, game2: i32, game3: i32, playername: String, btnclicks: i32) -> 
(String, TextureManager, f64, i32, i32, i32, String, i32) {

    let mut _game3 = game3;
    let mut _btnclicks = btnclicks;
    let mut _game2 = game2;
    let mut _game1 = game1;
    let mut _playername = playername;
    let mut _game1 = game1;
    request_new_screen_size(1920.0, 1080.0);

    let btn_replay = TextButton::new(625.0, 500.0, 200.0, 60.0, "REPLAY?", GREEN, BLACK, 30);

    let mut lbl_death = Label::new(
        &format!("You won all 3 games!, the brothers let you go and you lived to tell the tale! for now..."),
        200.0, 200.0, 55
    );
    let mut lbl_2 = Label::new("would you like to replay?", 300.0, 290.0, 35);

    lbl_death.with_colors(RED, None);
    lbl_2.with_colors(WHITE, None);




    loop {
        clear_background(DARKGRAY);
        draw_grid(50.0, WHITE);

        if btn_replay.click() {
        _btnclicks = 0; 
        _game3 = 0;
       _game2 = 0;
        _game1 = 0;
      _playername = String::new();
          return ("menu".to_string(), tm, _elapsed, game1, game2, game3, _playername.to_string(), btnclicks);
        }
    
        lbl_death.draw();
        lbl_2.draw();
        next_frame().await;
    }
}