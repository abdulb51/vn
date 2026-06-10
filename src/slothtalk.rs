use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
 use crate::modules::preload_image::TextureManager;
use crate::modules::text_button::TextButton;
use crate::modules::grid::draw_grid;
  use crate::modules::label::Label;
  use crate::modules::label::TextAlign;


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
  let mut btnclicks = 0;

  let btn_exit = TextButton::new(
    1870.0,
    0.0,
    50.0,
    50.0,
    "X",
    RED,
    BLACK,
    50
  );

let btn_back = TextButton::new(
    100.0,
    100.0,
    200.0,
    60.0,
    "<-",
    PINK,
    WHITE,
    30
  );


  let btn_next = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "->",
        PINK,
        WHITE,
        30
    );
 
  let sloth = StillImage::new(
      "assets/sloth.png",
      1000.0,  // width
      1000.0,  // height
      500.0,  // x position 
      400.0,   // y position
      true,   // Enable stretching
      1.0,    // Normal zoom (100%)
  ).await;
  

  let mut lbl_slothtalk = Label::new("", 50.0, 100.0, 30);
   lbl_slothtalk.with_alignment(TextAlign::Center);
  




    loop {
  clear_background(WHITE);
        draw_text("slothtalk", 20.0, 40.0, 30.0, BLACK);
    draw_grid(50.0,BLACK);


       

        if btn_exit.click() {
            return "menu".to_string();
        }
if btn_next.click() {
  btnclicks += 1;
  
  
  
 


  }

 if btnclicks ==1{

  }
  

        sloth.draw();
        next_frame().await;
    }
}