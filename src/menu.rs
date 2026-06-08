use macroquad::prelude::*;
use crate::modules::text_button::TextButton;
use crate::modules::grid::draw_grid;
pub async fn run() -> String {
    
    
    let btn_play = TextButton::new(50.0, 500.0, 250.0, 50.0, "ba ba black sheep", GREEN, BLACK, 30);
    
    
    
    
    
    
    loop {
        clear_background(WHITE);
       draw_grid(50.0,BLACK);


       
       
        draw_text("Menu", 20.0, 40.0, 30.0, BLACK);
        if is_key_pressed(KeyCode::Space) {
            return "game".to_string();
        }

        next_frame().await;
    }
}