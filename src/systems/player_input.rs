#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
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

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let mut did_something = false;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });
            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }
        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1);
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
