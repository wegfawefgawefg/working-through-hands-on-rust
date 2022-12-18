#![warn(clippy::all, clippy::pedantic)]

// game state struct
// player
// obstacle

fn main() {}
use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Hello Bracket World");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Nine Eleven")
        .build()?;

    let gs: State = State {};
    main_loop(context, gs)
}
