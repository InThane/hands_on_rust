#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

pub struct Player {
    pub position: Point
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self {
            position
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map : &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => MOVE_LEFT,
                VirtualKeyCode::H => MOVE_LEFT,
                VirtualKeyCode::Right => MOVE_RIGHT,
                VirtualKeyCode::L => MOVE_RIGHT,
                VirtualKeyCode::Up => MOVE_UP,
                VirtualKeyCode::K => MOVE_UP,
                VirtualKeyCode::Down => MOVE_DOWN,
                VirtualKeyCode::J => MOVE_DOWN,
                _ => Point::zero()
            };

            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
            }
        }
    }
}