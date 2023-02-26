#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) 
{
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());
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

        players.iter(ecs).for_each(|(entity,pos)| {
            let destination = *pos + delta;
            commands
                .push (((), WantsToMove{ entity: *entity, destination}));
        });
        *turn_state = TurnState::PlayerTurn;
    }
}