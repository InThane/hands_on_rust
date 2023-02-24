#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera
) 
{
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => MOVE_LEFT,
            VirtualKeyCode::H => MOVE_LEFT,
            VirtualKeyCode::Right => MOVE_RIGHT,
            VirtualKeyCode::L => MOVE_RIGHT,
            VirtualKeyCode::Up => MOVE_UP,
            VirtualKeyCode::K => MOVE_UP,
            VirtualKeyCode::Down => MOVE_DOWN,
            VirtualKeyCode::J => MOVE_DOWN,
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query()//(1)
                .filter(component::<Player>());//(2)
            players.iter_mut(ecs).for_each(|pos| {//(3)
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}