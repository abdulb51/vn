use macroquad::prelude::*;
use crate::modules::text_button::TextButton;
use crate::modules::grid::draw_grid;
use crate::modules::still_image::StillImage;
 use crate::modules::preload_image::TextureManager;
    use crate::modules::preload_image::LoadingScreenOptions;
    
    
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
    


      let img_mrplaceholder = StillImage::new(
        "assets/mrplaceholder.png",
        1000.0,  // width
        1000.0,  // height
        500.0,  // x position
        400.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;


    let img_mrsloth = StillImage::new(
        "assets/sloth.png",
        1000.0,  // width
        1000.0,  // height
        1000.0,  // x position
        200.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let img_mrgreed = StillImage::new(
        "assets/greed.png",
        1000.0,  // width
        1000.0,  // height
        0.0,  // x position
        200.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;




    let btn_play = TextButton::new(50.0, 500.0, 250.0, 50.0, "Play", GREEN, BLACK, 30);
    
    
    
    
    
    
    loop {
        clear_background(WHITE);
       draw_grid(50.0,BLACK);

      

       if btn_play.click() {
            return "slothtalk".to_string();
        }
       
        img_mrplaceholder.draw();
img_mrsloth.draw();
img_mrgreed.draw();
        draw_text("Menu", 20.0, 40.0, 30.0, BLACK);
    
        next_frame().await;
    }
}