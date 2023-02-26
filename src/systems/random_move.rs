#![warn(clippy::all, clippy::pedantic)]

use std::process::CommandArgs;

use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &mut Point, &MovingRandomly)>::query();
    movers
        .iter_mut(ecs)
        .for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0,4) {
                0 => MOVE_UP,
                1 => MOVE_DOWN,
                2 => MOVE_LEFT,
                _ => MOVE_RIGHT,
            } + *pos;

            commands
                .push(((), WantsToMove{entity: *entity, destination}));
        }
    );
}