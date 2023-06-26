use game::Game;

const SCREEN_WIDTH: i32 = 720;
const SCREEN_HEIGHT: i32 = 480;

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Snake")
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .vsync()
        .build();

    let mut game_handle = Game::init();

    while !rl.window_should_close() {
        if !game_handle.update(&mut rl) {
            break;
        }

        let mut d = rl.begin_drawing(&thread);
        game_handle.draw(&mut d);
    }

    println!("Good game! Score: {}", game_handle.score());
}

mod game {
    use raylib::prelude::*;

    use food::Food;
    use grid::Grid;
    use snake::Snake;
    use tick::TickCounter;

    const TICKS_PER_SECOND: u128 = 10;
    const GRID_SPACING_PX: i32 = 40;

    /// Tracks all game state.
    pub struct Game {
        background: Color,
        counter: TickCounter,
        grid: Grid,
        snake: Snake,
        food: Food,
    }

    impl Game {
        pub fn init() -> Self {
            Game {
                background: Color::WHITE,
                counter: TickCounter::start(TICKS_PER_SECOND),
                grid: Grid::new(GRID_SPACING_PX, Color::RAYWHITE),
                snake: Snake::new(1, Color::GREEN, Color::DARKGREEN),
                food: Food::new(Color::RED),
            }
        }

        /// Returns false is the game should exit.
        pub fn update(&mut self, rl: &mut RaylibHandle) -> bool {
            self.snake.poll(rl);

            if self.counter.is_next_tick() {
                return self.snake.update(&mut self.food);
            }

            true
        }

        pub fn draw(&self, d: &mut RaylibDrawHandle) {
            d.clear_background(self.background);

            self.grid.draw(d);
            self.snake.draw(d);
            self.food.draw(d);
        }

        pub fn score(&self) -> u64 {
            self.snake.score()
        }
    }

    mod grid {
        use raylib::prelude::*;

        use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

        /// Represents an even 2D grid of lines across the screen
        pub struct Grid {
            num_cols: i32,
            num_rows: i32,
            spacing: i32,
            color: Color,
        }

        impl Grid {
            pub fn new(spacing: i32, color: Color) -> Self {
                assert!(spacing > 0, "spacing must be positive");

                let screen_width = SCREEN_WIDTH;
                let screen_height = SCREEN_HEIGHT;

                assert!(
                    screen_width % spacing == 0,
                    "screen width not divisible by spacing"
                );
                assert!(
                    screen_height % spacing == 0,
                    "screen height not divisible by spacing"
                );

                Grid {
                    num_cols: screen_width / spacing,
                    num_rows: screen_height / spacing,
                    spacing,
                    color,
                }
            }

            pub fn draw(&self, d: &mut RaylibDrawHandle) {
                // Draw vertical lines
                for i in 1..self.num_cols {
                    let col_x = i * self.spacing;
                    d.draw_line(col_x, 0, col_x, SCREEN_HEIGHT, self.color);
                }

                // Draw horizontal lines
                for i in 1..self.num_rows {
                    let row_y = i * self.spacing;
                    d.draw_line(0, row_y, SCREEN_WIDTH, row_y, self.color);
                }
            }
        }
    }

    mod snake {
        use std::collections::VecDeque;

        use raylib::prelude::*;

        use super::{food::Food, GRID_SPACING_PX};
        use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

        const S_HEIGHT_F32: f32 = SCREEN_HEIGHT as f32;
        const S_WIDTH_F32: f32 = SCREEN_WIDTH as f32;

        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Direction {
            UP,
            DOWN,
            LEFT,
            RIGHT,
        }
        impl Direction {
            fn v(&self, scale: f32) -> Vector2 {
                (match self {
                    &Self::UP => Vector2 { x: 0.0, y: -1.0 },
                    &Self::DOWN => Vector2 { x: 0.0, y: 1.0 },
                    &Self::LEFT => Vector2 { x: -1.0, y: 0.0 },
                    &Self::RIGHT => Vector2 { x: 1.0, y: 0.0 },
                } * scale)
            }
        }

        pub struct Snake {
            positions: Vec<Vector2>,
            directions: VecDeque<Direction>,
            next_direction: Direction,
            grid_spacing: f32,
            size: f32,
            head_color: Color,
            body_color: Color,
            active: bool,
            score: u64,
        }

        impl Snake {
            pub fn new(
                initial_size: u8,
                head_color: Color,
                body_color: Color,
            ) -> Self {
                let grid_spacing = GRID_SPACING_PX as f32;
                let mut s = Snake {
                    positions: vec![Vector2 {
                        x: S_WIDTH_F32 / 2.0,
                        y: S_HEIGHT_F32 / 2.0,
                    }],
                    directions: vec![Direction::RIGHT].into(),
                    next_direction: Direction::RIGHT,
                    grid_spacing,
                    size: grid_spacing,
                    head_color,
                    body_color,
                    active: true,
                    score: 0,
                };

                for _ in 0..(initial_size - 1) {
                    Self::add_tail_block(
                        &mut s.positions,
                        &mut s.directions,
                        grid_spacing,
                    );
                }

                s
            }

            pub fn head(&self) -> Vector2 {
                self.positions[0]
            }

            pub fn poll(&mut self, rl: &RaylibHandle) {
                let one_block = self.positions.len() == 0;

                // Change direction unless that means turning into the tail
                if rl.is_key_pressed(KeyboardKey::KEY_W)
                    && (one_block || self.directions[0] != Direction::DOWN)
                {
                    self.next_direction = Direction::UP;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_A)
                    && (one_block || self.directions[0] != Direction::RIGHT)
                {
                    self.next_direction = Direction::LEFT;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_S)
                    && (one_block || self.directions[0] != Direction::UP)
                {
                    self.next_direction = Direction::DOWN;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_D)
                    && (one_block || self.directions[0] != Direction::LEFT)
                {
                    self.next_direction = Direction::RIGHT;
                }
            }

            fn add_tail_block(
                positions: &mut Vec<Vector2>,
                directions: &mut VecDeque<Direction>,
                spacing: f32,
            ) {
                let last_pos = positions.last().unwrap().clone();
                let last_dir = *directions.back().unwrap();
                positions.push(last_pos - last_dir.v(spacing));
                directions.push_back(last_dir);
            }

            /// Returns false in the case that the snake died.
            pub fn update(&mut self, food: &mut Food) -> bool {
                if !self.active {
                    return false;
                }

                assert!(self.positions.len() == self.directions.len());

                self.directions.push_front(self.next_direction);
                self.directions.pop_back();

                // generate tail (for testing)
                for i in 0..self.positions.len() {
                    self.positions[i] +=
                        self.directions[i].v(self.grid_spacing);

                    // Screen wraparound
                    if self.positions[i].x < 0.0 {
                        self.positions[i].x = S_WIDTH_F32 - self.grid_spacing;
                    }
                    if self.positions[i].x > S_WIDTH_F32 - self.grid_spacing {
                        self.positions[i].x = 0.0;
                    }

                    if self.positions[i].y < 0.0 {
                        self.positions[i].y = S_HEIGHT_F32 - self.grid_spacing;
                    }
                    if self.positions[i].y > S_HEIGHT_F32 - self.grid_spacing {
                        self.positions[i].y = 0.0;
                    }
                }

                // Eating food
                if self.head() == food.position() {
                    Self::add_tail_block(
                        &mut self.positions,
                        &mut self.directions,
                        self.grid_spacing,
                    );

                    food.eat(&self.positions);
                    self.score += 10;
                }

                // Self collisions
                if self.positions[1..].contains(&self.head()) {
                    self.active = false;
                    return false;
                }

                true
            }

            pub fn draw(&self, d: &mut RaylibDrawHandle) {
                let size = Vector2 {
                    x: self.size,
                    y: self.size,
                };
                for i in 0..self.positions.len() {
                    let color = if i == 0 {
                        self.head_color
                    } else {
                        self.body_color
                    };
                    d.draw_rectangle_v(self.positions[i], size, color);
                }
            }

            pub fn score(&self) -> u64 {
                self.score
            }
        }
    }

    mod food {
        use rand::prelude::*;
        use raylib::prelude::*;

        use super::GRID_SPACING_PX;
        use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

        const COLS: i32 = SCREEN_WIDTH / GRID_SPACING_PX;
        const ROWS: i32 = SCREEN_HEIGHT / GRID_SPACING_PX;

        pub struct Food {
            rng: ThreadRng,
            position: Option<Vector2>, // Needless?
            grid_spacing: f32,
            size: f32,
            color: Color,
        }

        impl Food {
            pub fn new(color: Color) -> Self {
                Food {
                    rng: thread_rng(),
                    position: Some(Vector2 { x: 0.0, y: 0.0 }),
                    grid_spacing: GRID_SPACING_PX as f32,
                    size: GRID_SPACING_PX as f32,
                    color,
                }
            }

            pub fn eat(&mut self, blocked: &Vec<Vector2>) {
                let mut gen_pos = || Vector2 {
                    x: self.rng.gen_range(0..COLS) as f32 * self.grid_spacing,
                    y: self.rng.gen_range(0..ROWS) as f32 * self.grid_spacing,
                };

                let mut new_pos = gen_pos();
                while blocked.contains(&new_pos) {
                    new_pos = gen_pos();
                }

                self.position = Some(new_pos);
            }

            /// Returns an impossible to reach position in the case that the
            /// food isn't rendered yet
            pub fn position(&self) -> Vector2 {
                match self.position {
                    Some(pos) => pos,
                    None => Vector2 { x: -1.0, y: -1.0 },
                }
            }

            pub fn draw(&self, d: &mut RaylibDrawHandle) {
                if let Some(pos) = self.position {
                    d.draw_rectangle_v(
                        pos,
                        Vector2 {
                            x: self.size,
                            y: self.size,
                        },
                        self.color,
                    );
                }
            }
        }
    }

    mod tick {
        use std::time::Instant;

        pub struct TickCounter {
            start: Instant,
            nanos_per_tick: u128,
            tick: u128,
        }

        impl TickCounter {
            pub fn start(ticks_per_second: u128) -> Self {
                TickCounter {
                    start: Instant::now(),
                    nanos_per_tick: 1_000_000_000 / ticks_per_second,
                    tick: 0,
                }
            }

            pub fn is_next_tick(&mut self) -> bool {
                let curr = self
                    .start
                    .elapsed()
                    .as_nanos()
                    .saturating_div(self.nanos_per_tick);

                if curr > self.tick {
                    self.tick = curr;
                    return true;
                }

                false
            }
        }
    }
}
