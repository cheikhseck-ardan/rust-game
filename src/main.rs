use bracket_lib::prelude::*;
use rltk::Rltk;

const TOP_SCREEN_PIXEL: i32 = 8;
const BOX_HEIGHTWIDTH:  i32 = 5;
const GROUND_PIXEL:     i32 = 45;
const GROUND_WIDTH:     i32 = 80;
const GAME_WINDOW:      i32 = 50;
const GROUND_COLLISION: i32 = GROUND_PIXEL-BOX_HEIGHTWIDTH;

const MOVING_NONE: i32 = 0;
const MOVING_UP:   i32 = 1;
const MOVING_DOWN: i32 = 2;

struct State {
     y:       i32,  // Box's veritical position.
     moving:  i32,  // Direction the box is moving.
}

fn player_input(state: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Space => {
                if state.y == GROUND_COLLISION {
                    state.moving = MOVING_UP;
                }
            },
            _ => {},
        },
    };
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        player_input(self, ctx);

        ctx.cls_bg(RGB::named(WHITE));
        
        ctx.draw_bar_horizontal(
            0,                  // sx
            TOP_SCREEN_PIXEL,   // sy
            GROUND_WIDTH,       // width
            GAME_WINDOW,        // n
            GAME_WINDOW,        // max
            RGB::named(YELLOW), // foreground color
            RGB::named(YELLOW), // background color
		);

        ctx.draw_bar_horizontal(
            0,                  // sx
            GROUND_PIXEL,       // sy
            GROUND_WIDTH,       // width
            GAME_WINDOW,        // n
            GAME_WINDOW,        // max
            RGB::named(YELLOW), // foreground color
            RGB::named(YELLOW), // background color
		);

        ctx.draw_box_double(
            10,              // x
            self.y,          // y
            BOX_HEIGHTWIDTH, // width
            BOX_HEIGHTWIDTH, // height
            RGB::named(RED), // foreground color
            RGB::named(RED), // background color
		);

        if self.moving == MOVING_DOWN {
            self.y += 1;
            if self.y == GROUND_COLLISION {
                self.moving = MOVING_NONE;
            }
        }

        if self.moving == MOVING_UP {
	        self.y -= 1;
	        if self.y == TOP_SCREEN_PIXEL {
	            self.moving = MOVING_DOWN;
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
        y:      GROUND_COLLISION,
        moving: MOVING_NONE,
    };

    register_palette_color("pink", RGB::named(MAGENTA));

    main_loop(context, gs)
}