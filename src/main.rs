// main initialized the game and starts the game loop.

mod state;

fn main() -> rltk::BError {
    let context = rltk::BTermBuilder::simple80x50()
        .with_title("Hello Bracket World")
        .build()?;

    rltk::main_loop(context, state::State::new())
}