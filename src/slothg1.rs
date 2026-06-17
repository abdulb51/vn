/*
By: <Abdul Baig>
Date: 2026-06-01
Program Details: a tag game where you have to tag the target bot 10 times so you can progress the dialogue
*/
use macroquad::rand::rand;
use crate::modules::collision::check_collision;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use macroquad::prelude::KeyCode::Right;
use macroquad::prelude::*;


/// Time limit in seconds - player must catch target 5 times before this
const TIME_LIMIT: f64 = 30.0;

/// Player movement speed
const MOVE_SPEED: f32 = 500.0;


pub async fn run(tm: TextureManager, elapsed: f64, game1: i32, game2: i32, game3: i32) -> (String, TextureManager, f64, i32, i32, i32) {
    let mut tags = 0;

    // The target bot
    let mut img_target = StillImage::new("assets/sloth.png", 130.0, 130.0, 900.0, 70.0, true, 1.0).await;

    // The player
    let mut img_player = StillImage::new("assets/.png", 150.0, 150.0, 10.0, 40.0, true, 1.0).await;

    // Background maze
    let img_bg = StillImage::new("assets/maze.png", 1080.0, 1080.0, 0.0, 0.0, true, 1.0).await;

    // Cooldown after being tagged so the target doesn't instantly get retagged
    let mut tag_cooldown: f32 = 0.0;
    const TAG_COOLDOWN_DURATION: f32 = 1.5;

    let mut lbl_tags = Label::new("Tags: 0 / 10", 20.0, 40.0, 32);
    lbl_tags.with_colors(WHITE, Some(Color::new(0.0, 0.0, 0.0, 0.6)));

    let mut lbl_timer = Label::new("Time: 30.0s", 800.0, 40.0, 32);
    lbl_timer.with_colors(WHITE, Some(Color::new(0.0, 0.0, 0.0, 0.6)));

    let mut lbl_speed = Label::new("Speed: 1.0x", 400.0, 40.0, 28);
    lbl_speed.with_colors(YELLOW, Some(Color::new(0.0, 0.0, 0.0, 0.6)));

    let start_time = get_time();

    loop {
        clear_background(WHITE);
        img_bg.draw();
   

        let elapsed = get_time() - start_time;
        let remaining = TIME_LIMIT - elapsed;

        // Timer check
        if remaining <= 0.0 {
            return ("deathscreen".to_string(), tm, elapsed, game1, game2, game3);
        }

        // Win check
        if tags >= 10 {
            return ("win".to_string(), tm, elapsed, game1, game2, game3);
        }

        // Update UI labels
        lbl_tags.set_text(format!("Tags: {} / 10", tags));

        // Timer turns red when under 10 seconds
        if remaining < 10.0 {
            lbl_timer.with_colors(RED, Some(Color::new(0.0, 0.0, 0.0, 0.6)));
        }
        lbl_timer.set_text(format!("Time: {:.1}s", remaining.max(0.0)));

        // Tag cooldown countdown
        if tag_cooldown > 0.0 {
            tag_cooldown -= get_frame_time();
        }

        // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        // Keyboard input
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            move_dir.x += 2.0;
        }
        if is_key_pressed(Right) || is_key_pressed(KeyCode::D) {
            img_player.set_image("assets/subaru.png").await; // Change to right looking image
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            move_dir.x -= 2.0;

            if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                img_player.set_image("assets/subaruflip.png").await; // Change to left looking image
            }
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            move_dir.y += 2.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            move_dir.y -= 2.0;
        }

        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        // Apply movement based on frame time
        let movement = move_dir * MOVE_SPEED * get_frame_time();

        // Save old position in case of collision
        let old_pos = img_player.pos();

        // Move X first
        if movement.x != 0.0 {
            img_player.set_x(img_player.get_x() + movement.x);
            if check_collision(&img_player, &img_bg, 1) {
                img_player.set_x(old_pos.x); // Undo if collision happens
            }
        }

        // Move Y next
        if movement.y != 0.0 {
            img_player.set_y(img_player.get_y() + movement.y);
            if check_collision(&img_player, &img_bg, 1) {
                img_player.set_y(old_pos.y); // Undo if collision happens
            }
        }

        // Tag detection
        if tag_cooldown <= 0.0 && check_collision(&img_player, &img_target, 1) {
            tags += 1;
            tag_cooldown = TAG_COOLDOWN_DURATION;

            let idx = (rand() % 4) as usize;
            let target_spots: Vec<(f32, f32)> = vec![(20.0, 20.0), (900.0, 20.0), (20.0, 900.0), (900.0, 900.0)]; // Target spawn points
            img_target.set_position(vec2(target_spots[idx].0, target_spots[idx].1)); // Random target spawn

        

        }

        img_target.draw();
        img_player.draw();

        lbl_tags.draw();
        lbl_timer.draw();
        lbl_speed.draw();

        next_frame().await;
    }
}
