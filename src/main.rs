//! # Snake
//!
//! `snake-raylib` is an implementation of the classic arcade game Snake with
//! [raylib-rs](https://docs.rs/raylib/latest/raylib/index.html#).
//!
//! **Author:** Jude Muriithi (GitHub: [jkmuriithi](https://github.com/jkmuriithi))
//!
//! **TODO:**
//! - Configuration file system for in-game constants
//! - Linux/MacOS build tests

// Hide debug console in Windows build
#![cfg_attr(target_os = "windows",windows_subsystem = "windows")]

use game::{Game, GameState};
use raylib::prelude::*;

mod game;

/// Width of the game window in pixels.
const SCREEN_WIDTH: i32 = 720;

/// Height of the game window in pixels.
const SCREEN_HEIGHT: i32 = 480;

/// Horizontal pixel offset of the final score.
const SCORE_OFFSET_X: i32 = 140;

/// Additional horizontal pixel offset for each digit in the final score.
const SCORE_OFFSET_DELTA: i32 = 20;

/// Vertical pixel offset of the final score.
const SCORE_OFFSET_Y: i32 = 190;

/// Starts a new game session.
fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Snake")
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .vsync()
        .build();

    let mut game = Game::init();

    while !rl.window_should_close() {
        match game.state {
            GameState::RUNNING => {
                game.update(&mut rl);
                let mut d = rl.begin_drawing(&thread);
                game.draw(&mut d);
            }
            GameState::ENDED => {
                let mut d = rl.begin_drawing(&thread);
                let score = game.score();

                // Draw final score in the middle of the screen
                d.clear_background(Color::GRAY);
                let text = format!("Score: {}", score);
                let digit_offset = if score == 0 {
                    SCORE_OFFSET_DELTA
                } else {
                    (score as f64).log10() as i32 * SCORE_OFFSET_DELTA
                };
                d.draw_text(
                    &text,
                    SCORE_OFFSET_X - digit_offset,
                    SCORE_OFFSET_Y,
                    100,
                    Color::BLACK,
                );
            }
        }
    }
}
