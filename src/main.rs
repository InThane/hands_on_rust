#![warn(clippy::all, clippy::pedantic)]

mod map;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT:i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub const MOVE_UP: Point = Point { x: 0, y: -1};
    pub const MOVE_DOWN: Point = Point { x: 0, y: 1};
    pub const MOVE_RIGHT: Point = Point { x: 1, y: 0};
    pub const MOVE_LEFT: Point = Point { x: -1, y: 0};
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self { 
            map: Map::new(),
            player: Player::new(
                Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)
            ),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);

    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
    .with_title("Rusty Rogue")
    .with_fps_cap(30.0)
    .build()?;

    main_loop(context, State::new())
}
