// main initialized the game and starts the game loop.

pub mod state;

fn main() -> rltk::BError {
    let bterm = rltk::BTermBuilder::simple80x50()
        .with_title("Hello Bracket World")
        .build()?;

    return rltk::main_loop(bterm, state::new());
}
