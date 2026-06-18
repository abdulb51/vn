use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::label::TextAlign;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;
use macroquad::prelude::*;

pub async fn run(
    tm: TextureManager,
    _elapsed: f64,
    game1: i32,
    game2: i32,
    game3: i32,
    playername: String,
    btnclicks: i32,
) -> (String, TextureManager, f64, i32, i32, i32, String, i32) {
    let mut playername = playername;
    let mut btnclicks = btnclicks;
    let mut game1 = game1;
    let mut game2 = game2;
    let mut game3 = game3;

    let mut name = TextInput::new(500.0, 800.0, 300.0, 40.0, 25.0);
    name.set_prompt("Enter your name...");

    let btn_exit = TextButton::new(1870.0, 0.0, 50.0, 50.0, "X", RED, BLACK, 50);

    let mut btn_back = TextButton::new(450.0, 900.0, 50.0, 50.0, "<-", PINK, WHITE, 30);

    let mut btn_next = TextButton::new(1400.0, 900.0, 50.0, 50.0, "->", PINK, WHITE, 30);

    let mut show_cheats = false;

    let mut sloth = StillImage::new(
        "",
        1080.0, // width
        1080.0, // height
        550.0,  // x position
        -150.0, // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
sloth.set_preload(tm.get_preload("assets/sloth.png").unwrap());
    let mut lbl_slothtalk = Label::new("", 500.0, 800.0, 30);
    lbl_slothtalk.with_alignment(TextAlign::Center);

    let btn_play = TextButton::new(800.0, 500.0, 200.0, 100.0, "play?", PINK, WHITE, 60);

    let mut img_bg = StillImage::new(
        "",
        1920.0, // width
        1080.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
img_bg.set_preload(tm.get_preload("assets/parkday.png").unwrap());
    let mut cheats = TextInput::new(450.0, 650.0, 500.0, 40.0, 25.0);
    cheats.set_prompt("set clicks,game1,game2,game3,name");

    request_new_screen_size(1920.0, 1080.0);
    loop {
        clear_background(WHITE);

        img_bg.draw();
        draw_grid(50.0, GRAY);
        sloth.draw();

        draw_rectangle(450.0, 700.0, 1000.0, 200.0, GRAY);

        if btn_exit.click() {
            return ("menu".to_string(), tm, _elapsed, game1, game2, game3, playername.to_string(), btnclicks);
        }

        if btn_next.click() {
            btnclicks += 1;
        }
        if btn_back.click() {
            btnclicks -= 1;
        }

        if btnclicks == 0 {
            btn_next.enabled = true;
            btn_back.enabled = false;
        }

        if btnclicks == 1 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text("zzz");
        }

        if btnclicks == 2 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text("zzzzz");
        }

        if btnclicks == 3 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text("huh... who's there?!");
        }

        if btnclicks == 4 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text("zzz");
        }

        if btnclicks == 5 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text("Who are you?");
        }

        if btnclicks == 6 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text("");
            name.draw();
        }

        if btnclicks == 7 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            let playername = name.get_text();
            if playername == "" {
                btnclicks -= 1;
            }
            lbl_slothtalk.set_text(format!("Hello, {}", playername));
        }

        if btnclicks == 8 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text(format!("zz"));
        }

        if btnclicks == 9 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text(format!("if you can beat me and my brother, I'll let you live \n for the first game, catch 10 times and move on to the next game"));
        }

        if btnclicks == 10 {
            btn_next.enabled = false;
            lbl_slothtalk.set_text(format!(""));

            if btn_play.click() {
                return (
                    "slothg1".to_string(),
                    tm,
                    _elapsed,
                    game1,
                    game2,
                    game3,
                    playername.to_string(),
                    btnclicks,
                );
            }
        }

        if btnclicks == 11 && game1 == 1 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text(format!("No way!\n how'd you catch me?"));
        }

        if btnclicks == 12 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text(format!("zzz"));
        }

        if btnclicks == 13 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text(format!("you truely arent slothful..."));
        }

        if btnclicks == 14 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text(format!("zzz"));
        }

        if btnclicks == 15 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text(format!("very well..."));
        }

        if btnclicks == 16 {
            btn_back.enabled = true;
            btn_next.enabled = true;

            lbl_slothtalk.set_text(format!("let's see if you're fast enough to beat this one...\n click space on the green bar, but be careful... it shrinks..."));
        }

        if btnclicks == 17 {
            btn_back.enabled = true;
            btn_next.enabled = false;
            lbl_slothtalk.set_text(format!(""));
       
       if btn_play.click() {
                return (
                    "slothg2".to_string(),
                    tm,
                    _elapsed,
                    game1,
                    game2,
                    game3,
                    playername.to_string(),
                    btnclicks,
                );
            }
        }

       
       if btnclicks == 18 && game2 == 1 && game1 == 1 {
            btn_back.enabled = true;
            btn_next.enabled = true;
            lbl_slothtalk.set_text(format!("zzz"));
        }
       
          if btnclicks == 19 {
            btn_back.enabled = true;
            btn_next.enabled = true;
            lbl_slothtalk.set_text(format!("woah you're really fast..."));
        }

        if btnclicks == 20 {
            btn_back.enabled = true;
            btn_next.enabled = true;
            lbl_slothtalk.set_text(format!("lets see if you can beat my brother..."));
        }
       
       
       
       if btnclicks == 21 {
            btn_back.enabled = true;
            btn_next.enabled = true;
            lbl_slothtalk.set_text(format!(""));
            sloth.set_image("assets/greed.png");
            
        }
       
       if btnclicks == 22 {
   btn_back.enabled = true;
            btn_next.enabled = true;
  lbl_slothtalk.set_text(format!("Hello, {}\n I'm Greed.", playername));
       }
       
if btnclicks == 23 {
            btn_back.enabled = true;
            btn_next.enabled = true;
            lbl_slothtalk.set_text(format!("for your next game, you will have to dodge incoming traffic and get to the other side."));
        }

       if btnclicks == 24 {

  lbl_slothtalk.set_text(format!("if you beat my next game your life will be spared\n but if you fail, you will have to restart from zero,\n good luck! {}...", playername));
       }

       if btnclicks == 25 {
            btn_back.enabled = true;
            btn_next.enabled = false;
            lbl_slothtalk.set_text(format!(""));
       
       if btn_play.click() {
                return (
                    "slothg3".to_string(),
                    tm,
                    _elapsed,
                    game1,
                    game2,
                    game3,
                    playername.to_string(),
                    btnclicks,
                );
            }
        }

       if is_key_pressed(KeyCode::F1) {
            show_cheats = !show_cheats;
        }
        if show_cheats {
            cheats.draw();

            if is_key_pressed(KeyCode::Enter) {
                let input = cheats.get_text();
                let parts: Vec<&str> = input.split(',').collect();

                if parts.len() == 5 {
                    if let (Ok(c), Ok(g1), Ok(g2), Ok(g3)) = (
                        parts[0].trim().parse::<i32>(),
                        parts[1].trim().parse::<i32>(),
                        parts[2].trim().parse::<i32>(),
                        parts[3].trim().parse::<i32>(),
                    ) {
                        btnclicks = c;
                        game1 = g1;
                        game2 = g2;
                        game3 = g3;
                        playername = parts[4].trim().to_string();
                        println!("{:?}", parts);
                        println!("Cheats applied!");
                    } else {
                        println!("provide: clicks,game1,game2,game3,name");
                        cheats.set_text("provide: clicks,game1,game2,game3,name");
                    }
                } else {
                    println!("provide: clicks(num),game1(num),game2(num),game3(num),name(words)");
                    cheats.set_text("provide: clicks(num),game1(num),game2(num),game3(num),name(words)");
                }
            }
        }

        draw_rectangle(500., 900., 900., 50., GRAY);
        lbl_slothtalk.draw();
        next_frame().await;
    }
}
