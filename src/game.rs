//! Definitions for the [Game] and [GameState] structs.

use rand::prelude::*;
use raylib::prelude::*;

use snake::Snake;
use tick::TickCounter;

mod snake;
mod tick;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

/// Number of times the game renders each second.
const TICKS_PER_SECOND: u128 = 10;

/// Pixel size of a grid square.
const GRID_SCALE: i32 = 30;

/// [Vector2] representation of [`GRID_SCALE`].
const GRID_SQUARE: Vector2 = Vector2 {
    x: GRID_SCALE as f32,
    y: GRID_SCALE as f32,
};

/// Number of vertical columns in the on-screen grid.
const COLS: i32 = SCREEN_WIDTH / GRID_SCALE;

/// Number of horizontal rows in the on-screen grid.
const ROWS: i32 = SCREEN_HEIGHT / GRID_SCALE;

const BACKGROUND_COLOR: Color = Color::WHITE;
const GRID_LINE_COLOR: Color = Color::WHITE;

/// Color of the snake food shown on-screen.
const FOOD_COLOR: Color = Color::RED;

const LIVE_SCORE_SIZE: i32 = 20;
const LIVE_SCORE_COLOR: Color = Color::GREEN;

/// Represents the state of the current game session.
pub enum GameState {
    RUNNING,
    ENDED,
}

/// Tracks all game objects.
pub struct Game {
    pub state: GameState,
    tick_counter: TickCounter,
    rng: ThreadRng,
    score: u64,
    snake: Snake,
    food: Vector2,
}

impl Game {
    pub fn init() -> Self {
        assert!(GRID_SCALE > 0, "grid scale is negative");
        assert!(
            SCREEN_WIDTH % GRID_SCALE == 0,
            "screen width must be divisible by grid scale"
        );
        assert!(
            SCREEN_HEIGHT % GRID_SCALE == 0,
            "screen height must be divisible by grid scale"
        );

        let mut g = Game {
            state: GameState::RUNNING,
            tick_counter: TickCounter::start(TICKS_PER_SECOND),
            rng: rand::thread_rng(),
            score: 0,
            snake: Snake::new(Color::GREEN, Color::DARKGREEN),
            food: Vector2 { x: 0.0, y: 0.0 },
        };

        g.move_food();

        g
    }

    fn move_food(&mut self) {
        let mut gen_pos = || Vector2 {
            x: (self.rng.gen_range(0..COLS) * GRID_SCALE) as f32,
            y: (self.rng.gen_range(0..ROWS) * GRID_SCALE) as f32,
        };

        let mut new_pos = gen_pos();
        while self.snake.body().contains(&new_pos) {
            new_pos = gen_pos();
        }

        self.food = new_pos;
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        let keyboard_input = rl.get_key_pressed();
        self.snake.handle_input(keyboard_input);

        if self.tick_counter.is_next_tick() {
            self.snake.update();

            if self.snake.tail_iter().any(|v| *v == self.snake.head()) {
                self.state = GameState::ENDED;
            }

            if self.snake.head() == self.food {
                self.snake.add_tail_block();
                self.move_food();
                self.score += 10;
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(BACKGROUND_COLOR);

        // Draw grid
        for i in 1..COLS {
            let col_x = i * GRID_SCALE;
            d.draw_line(col_x, 0, col_x, SCREEN_HEIGHT, GRID_LINE_COLOR);
        }

        for i in 1..ROWS {
            let row_y = i * GRID_SCALE;
            d.draw_line(0, row_y, SCREEN_WIDTH, row_y, GRID_LINE_COLOR);
        }

        self.snake.draw(d);

        d.draw_rectangle_v(self.food, GRID_SQUARE, FOOD_COLOR);

        d.draw_text(
            &format!("SCORE: {}", self.score),
            0,
            0,
            LIVE_SCORE_SIZE,
            LIVE_SCORE_COLOR,
        );
    }

    pub fn score(&self) -> u64 {
        self.score
    }
}
