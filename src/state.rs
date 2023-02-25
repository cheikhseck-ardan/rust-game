// State contains all the game state and logic.

pub const TOP_SCREEN_PIXEL: i32 = 8;
pub const BOX_HEIGHTWIDTH:  i32 = 5;
pub const GROUND_PIXEL:     i32 = 45;
pub const GROUND_WIDTH:     i32 = 80;
pub const GAME_WINDOW:      i32 = 50;
pub const GROUND_COLLISION: i32 = GROUND_PIXEL-BOX_HEIGHTWIDTH;

enum Moving {
    Not,
    Up,
    Down,
}

// =============================================================================

pub struct State {
    box_y:       i32,    // Box's veritical position.
    box_moving:  Moving, // Direction the box is moving.
}

pub fn new() -> State {
    return State {
        box_y:      GROUND_COLLISION,
        box_moving: Moving::Not,
    }
}

impl rltk::GameState for State {
    fn tick(&mut self, ctx: &mut rltk::BTerm) {
        self.keyboard_input(ctx);
        self.render(ctx);
    }
}

impl State {
    fn keyboard_input(&mut self, ctx: &mut rltk::Rltk) {
        match ctx.key {
            None => {},
            Some(key) => match key {
                rltk::VirtualKeyCode::Space => {
                    if self.box_y == GROUND_COLLISION {
                        self.box_moving = Moving::Up;
                    }
                },
                _ => {},
            },
        };
    }

    fn render(&mut self, ctx: &mut rltk::BTerm) {
        ctx.cls_bg(rltk::RGB::named(rltk::WHITE));
       
        ctx.draw_bar_horizontal(
            0,                              // x
            TOP_SCREEN_PIXEL,               // y
            GROUND_WIDTH,                   // width
            GAME_WINDOW,                    // n
            GAME_WINDOW,                    // max
            rltk::RGB::named(rltk::YELLOW), // foreground color
            rltk::RGB::named(rltk::YELLOW), // background color
        );

        ctx.draw_bar_horizontal(
            0,                              // x
            GROUND_PIXEL,                   // y
            GROUND_WIDTH,                   // width
            GAME_WINDOW,                    // n
            GAME_WINDOW,                    // max
            rltk::RGB::named(rltk::YELLOW), // foreground color
            rltk::RGB::named(rltk::YELLOW), // background color
        );

        ctx.draw_box_double(
            10,                          // x
            self.box_y,                  // y
            BOX_HEIGHTWIDTH,             // width
            BOX_HEIGHTWIDTH,             // height
            rltk::RGB::named(rltk::RED), // foreground color
            rltk::RGB::named(rltk::RED), // background color
        );

        match self.box_moving {
            Moving::Down => {
                self.box_y += 1;
                if self.box_y == GROUND_COLLISION {
                    self.box_moving = Moving::Not;
                }
            },
            Moving::Up => {
                self.box_y -= 1;
                if self.box_y == TOP_SCREEN_PIXEL {
                    self.box_moving = Moving::Down;
                }
            },
            _ => {},
        }

        ctx.draw_box(36, 0, 20, 3, rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK));
        
        ctx.printer(
            55,
            1,
            &format!("#[pink]FPS: #[]{}", ctx.fps),
            rltk::TextAlign::Right,
            None,
        );
        
        ctx.printer(
            55,
            5,
            &format!("#[pink]Frame Time: #[]{} ms", ctx.frame_time_ms),
            rltk::TextAlign::Right,
            None,
        );
    }
}