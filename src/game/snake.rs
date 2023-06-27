//! Definitions for the [Snake] struct.

use std::collections::VecDeque;

use raylib::prelude::*;

use super::{COLS, GRID_SCALE, GRID_SQUARE, ROWS};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
const S_HEIGHT_F32: f32 = SCREEN_HEIGHT as f32;
const S_WIDTH_F32: f32 = SCREEN_WIDTH as f32;

const INITIAL_LENGTH: u8 = 1;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    const fn v(&self) -> Vector2 {
        match self {
            &Self::UP => Vector2 { x: 0.0, y: -1.0 },
            &Self::DOWN => Vector2 { x: 0.0, y: 1.0 },
            &Self::LEFT => Vector2 { x: -1.0, y: 0.0 },
            &Self::RIGHT => Vector2 { x: 1.0, y: 0.0 },
        }
    }
}

/// Represents the position and movement direction of the on-screen snake.
pub struct Snake {
    direction: Direction,

    /// Fixes a bug which allows players to turn directly backwards if they
    /// input multiple keys during the same frame.
    prev_direction: Direction,
    segments: VecDeque<Vector2>,
    head_color: Color,
    tail_color: Color,
}

impl Snake {
    pub fn new(head_color: Color, tail_color: Color) -> Self {
        let direction = Direction::RIGHT;
        let mut segments: VecDeque<_> = vec![Vector2 {
            x: ((COLS / 2) * GRID_SCALE) as f32,
            y: ((ROWS / 2) * GRID_SCALE) as f32,
        }]
        .into();

        // Create extra squares behind head, to taste
        for _ in 0..(INITIAL_LENGTH - 1) {
            segments.push_back(
                *segments.back().unwrap() + direction.v() * -GRID_SCALE as f32,
            );
        }

        Snake {
            direction,
            prev_direction: direction,
            segments,
            head_color,
            tail_color,
        }
    }

    pub fn head(&self) -> Vector2 {
        self.segments[0]
    }

    pub fn tail_iter(&self) -> impl Iterator<Item = &Vector2> {
        self.segments.range(1..)
    }

    pub fn body(&self) -> &VecDeque<Vector2> {
        &self.segments
    }

    pub fn handle_input(&mut self, input: Option<KeyboardKey>) {
        if let None = input {
            return;
        }

        // Change direction unless that means turning directly backwards, unless
        // the snake's length is 1
        let one_block = self.segments.len() == 1;
        match input {
            Some(KeyboardKey::KEY_W) => {
                if one_block || self.prev_direction != Direction::DOWN {
                    self.direction = Direction::UP;
                }
            }
            Some(KeyboardKey::KEY_A) => {
                if one_block || self.prev_direction != Direction::RIGHT {
                    self.direction = Direction::LEFT;
                }
            }
            Some(KeyboardKey::KEY_S) => {
                if one_block || self.prev_direction != Direction::UP {
                    self.direction = Direction::DOWN;
                }
            }
            Some(KeyboardKey::KEY_D) => {
                if one_block || self.prev_direction != Direction::LEFT {
                    self.direction = Direction::RIGHT;
                }
            }
            _ => (),
        }
    }

    pub fn add_tail_block(&mut self) {
        match self.segments.back() {
            Some(pos) => self.segments.push_back(*pos),
            None => panic!("no back segment found"),
        }
    }

    pub fn update(&mut self) {
        let mut next = self.head() + self.direction.v() * GRID_SCALE as f32;

        // Wraparound
        let x_bounds = (0.0, S_WIDTH_F32 - GRID_SCALE as f32);
        if next.x < x_bounds.0 {
            next.x = x_bounds.1;
        }
        if next.x > x_bounds.1 {
            next.x = x_bounds.0;
        }

        let y_bounds = (0.0, S_HEIGHT_F32 - GRID_SCALE as f32);
        if next.y < y_bounds.0 {
            next.y = y_bounds.1;
        }
        if next.y > y_bounds.1 {
            next.y = y_bounds.0;
        }

        self.segments.push_front(next);
        self.segments.pop_back();

        self.prev_direction = self.direction;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for i in 0..self.segments.len() {
            let color = if i == 0 {
                self.head_color
            } else {
                self.tail_color
            };
            d.draw_rectangle_v(self.segments[i], GRID_SQUARE, color)
        }
    }
}
