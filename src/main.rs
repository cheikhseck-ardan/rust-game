use bracket_lib::prelude::*;

struct State {
    y: i32,
    game_over: bool,
    going_down: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let col2 = RGB::named(YELLOW);

        ctx.cls();
        ctx.draw_bar_horizontal(
		    0,
		    40,
		   	80,
		    50,
		    50,
		    col2,
		    col2
		);

        if self.going_down {
            self.y += 1;
            if self.y > 48 {
                self.going_down = false;
            }
        } else {
            self.y -= 1;
            if self.y < 2 {
                self.going_down = true;
            }
        }

        ctx.draw_box(51, 0, 20, 3, RGB::named(WHITE), RGB::named(BLACK));
        ctx.printer(
            70,
            1,
            &format!("#[pink]FPS: #[]{}", ctx.fps),
            TextAlign::Right,
            None,
        );
        ctx.printer(
            70,
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
        going_down: true
    };

  
    main_loop(context, gs)
}