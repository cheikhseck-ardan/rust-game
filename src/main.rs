use bracket_lib::prelude::*;
use rltk::{Rltk};
const GROUND_ON_SCREEN: i32 = 34;
struct State {
    y: i32,
    game_over: bool,
    going_down: bool,
    jumping: bool,
}

fn jump(state: &mut State) {
    if state.y < GROUND_ON_SCREEN {
        return
    }

    state.jumping = true
}

fn player_input(state: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Up => jump(state),
            VirtualKeyCode::Space => jump(state),
            _ => {}
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {

        player_input(self, ctx);

        ctx.cls_bg(RGB::named(WHITE));
        ctx.draw_bar_horizontal(
            0,                  // sx
            40,                 // sy
            80,                 // width
            50,                 // n
            50,                 // max
            RGB::named(YELLOW), // foreground color
            RGB::named(YELLOW), // background color
		);

        ctx.draw_box_double(
            10,              // x
            self.y,          // y
            5,               // width
            5,               // height
            RGB::named(RED), // foreground color
            RGB::named(RED)  // background color
		);

        if self.going_down {
            self.y += 1;
            if self.y > 34 {
                self.going_down = false;
            }
        } 

        if self.jumping {
	        self.y -= 1;
	        if self.y < 2 {
	            self.going_down = true;
	            self.jumping = false
	        }
        }

        ctx.draw_box(36, 0, 20, 3, RGB::named(WHITE), RGB::named(BLACK));
        ctx.printer(
            55,
            1,
            &format!("#[pink]FPS: #[]{}", ctx.fps),
            TextAlign::Right,
            None,
        );
        ctx.printer(
            55,
            5,
            &format!("#[pink]Frame Time: #[]{} ms", ctx.frame_time_ms),
            TextAlign::Right,
            None,
        );
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Bracket World")
        .build()?;

    let gs: State = State {
        y: 1,
        // Game over will be set
        // to pause the games animation
        // from happenning
        game_over: false,
        going_down: true,
        jumping: false
    };

    register_palette_color("pink", RGB::named(MAGENTA));

    main_loop(context, gs)
}