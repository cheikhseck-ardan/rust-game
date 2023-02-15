use bracket_lib::prelude::*;
use rltk::{Rltk};
use std::time::{Instant, Duration};

pub struct State {
    y: i32,
    game_over: bool,
    going_down: bool,
    jumping: bool,
    obs : Vec<Obstacle>,
    time_since_spawn : Instant
}

#[derive(Debug)]
struct Obstacle {
    pub y: i32,
    pub x: i32,
}

impl State {

	pub fn new() -> Self {

		return Self {
	        y: 1,
	        // Game over will be set
	        // to pause the games animation
	        // from happenning
	        game_over: false,
	        going_down: true,
	        jumping: false,
	        obs : Vec::new(),
	        time_since_spawn: Instant::now(),
		}
	}

	fn game_scene(&mut self, ctx: &mut Rltk) {
  
	    player_input(self, ctx);
	    let elapsed_time = self.time_since_spawn.elapsed();


	    if elapsed_time.as_secs() > 5 {
	    	self.spawn_enemy();
	    	self.time_since_spawn = Instant::now()
	    }

		ctx.cls_bg(RGB::named(WHITE));
		ctx.draw_bar_horizontal(
		    0,
		    40,
		   	80,
		    50,
		    50,
		    RGB::named(YELLOW),
		    RGB::named(YELLOW)
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

		if self.jumping {
		    self.y -= 1;
		    if self.y < 7 {
		        self.going_down = true;
		        self.jumping = false;     
		    }
		}

		// Begin code to move obstacles
	}

	pub fn spawn_enemy(&mut self) {

		if self.game_over {
    		return
    	}

		let mut rng = RandomNumberGenerator::new();

		self.obs.push(Obstacle{
        	y: rng.range(7, 35), // set random Y positon for obs.
        	x : 20
        });

        print!("Found {:?}", self.obs)

	}

	fn start_screen(&mut self, ctx: &mut Rltk) {

	}

	
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {

        ctx.set_scale(
        	1.05,
        	1,
        	1
        );

        if !self.game_over {
        	self.game_scene(ctx);
        } else {

        }

        ctx.draw_box(36, 0, 20, 7, RGB::named(WHITE), RGB::named(BLACK));
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

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Up => jump(gs),
            VirtualKeyCode::Space => jump(gs),
            _ => {}
        },
    }
}

fn jump(gs: &mut State) {
 	if gs.jumping || gs.y < 34 {
 		return
 	}

 	gs.jumping = true
}


