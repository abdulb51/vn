use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
 use crate::modules::preload_image::TextureManager;
use crate::modules::text_button::TextButton;
use crate::modules::grid::draw_grid;
  use crate::modules::label::Label;
  use crate::modules::label::TextAlign;
  use crate::modules::text_input::TextInput;


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



pub async fn run(tm: TextureManager) -> (String,TextureManager)
 {






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
    350.0,
    900.0,
    50.0,
    50.0,
    "<-",
    PINK,
    WHITE,
    30
  );


  let btn_next = TextButton::new(
        1350.0,
        900.0,
        50.0,
        50.0,
        "->",
        PINK,
        WHITE,
        30
    );
 
  let sloth = StillImage::new(
      "assets/sloth.png",
      1080.0,  // width
      1080.0,  // height
      350.0,  // x position 
      -100.0,   // y position
      true,   // Enable stretching
      1.0,    // Normal zoom (100%)
  ).await;
  

  let mut lbl_slothtalk = Label::new("sloth", 50.0, 100.0, 30);
   lbl_slothtalk.with_alignment(TextAlign::Center);
  
  

let mut img_bg = StillImage::new(
    "assets/parkday.png",
    1920.0,  // width
    1080.0,  // height
    0.0,  // x position 
    0.0,   // y position
    true,   // Enable stretching
    1.0,    // Normal zoom (100%)
).await;


loop {
  
  clear_background(WHITE);
    
// img_bg.set_preload(tm.get_preload("assets/parkday.png").unwrap());
    
    img_bg.draw();



     draw_text("slothtalk", 20.0, 40.0, 30.0, BLACK);
    draw_grid(50.0,BLACK);


       

        if btn_exit.click() {
            return ("menu".to_string(),tm);
        }




        if btn_back.click() {
  btnclicks -= 1;
}

if btn_next.click() {
  btnclicks += 1;
  
  
  
 


  }

 if btnclicks ==1{



  }
  

        sloth.draw();
        draw_rectangle(450.0, 700.0, 1000.0, 300.0, GRAY);
        next_frame().await;
    }
}