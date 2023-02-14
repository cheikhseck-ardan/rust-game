use bracket_lib::prelude::*;

struct State {
    y: i32,
    game_over: bool,
    going_down: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let col2 = RGB::named(YELLOW);

        ctx.set_scale(
        	1.05,
        	1,
        	1
        );

        ctx.cls_bg(RGB::named(WHITE));
        ctx.draw_bar_horizontal(
		    0,
		    40,
		   	80,
		    50,
		    50,
		    col2,
		    col2
		);

        ctx.draw_box_double(
		    10,
		    self.y,
		    5,
		    5,
		    RGB::named(RED),
		    RGB::named(RED)
		);

        if self.going_down {
            self.y += 1;
            if self.y > 34 {
                self.going_down = false;
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
        going_down: true
    };

    register_palette_color("pink", RGB::named(MAGENTA));

    main_loop(context, gs)
}