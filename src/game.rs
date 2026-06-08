use macroquad::prelude::*;

pub async fn run() -> String {
  
  
  
  
  
  
  
  
    loop {
  clear_background(WHITE);
        draw_text("Screen 2", 20.0, 40.0, 30.0, BLACK);

        if is_key_pressed(KeyCode::Space) {
            return "screen1".to_string();
        }

        next_frame().await;
    }
}