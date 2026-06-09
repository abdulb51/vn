use macroquad::prelude::*;
use crate::modules::still_image::StillImage;






fn window_conf() -> Conf {
    Conf {
        window_title: "vn".to_string(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}



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