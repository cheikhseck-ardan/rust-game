pub mod state;

fn main() {
    let bterm = rltk::BTermBuilder::simple80x50()
        .with_title("Hello Bracket World")
        .build();

    match bterm {
        Ok(rltk) => {
            let _ = rltk::main_loop(rltk, state::new());
        }
        Err(error) => {
            println!("An error has occurred: {error:?}");
            println!("Aborting");
        }
    }

    return;
}
