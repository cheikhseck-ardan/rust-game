use bracket_lib::prelude::*;
use rltk::{Rltk};
mod state;

mod prelude {
    pub use crate::state::*;
}

use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Bracket World")
        .build()?;

    let gs: State = State::new();
    main_loop(context, gs) 
}