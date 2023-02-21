use bracket_lib::prelude::*;
use rltk::{Rltk};
use std::time::Instant;

pub struct State {
    y: i32,
    game_over: bool,
    going_down: bool,
    jumping: bool,
    obs : Vec<Obstacle>,
    time_since_spawn : Instant
}

struct Obstacle {
    pub y: i32,
    pub x: i32,
    pub height: i32
}

struct Rect {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Rect {
    fn contains(&self, x: i32, y: i32) -> bool {
        return self.x1 < x && x < self.x2 && self.y1 < y && y < self.y2;
    }
}

impl Obstacle {
	pub fn move_left(&mut self){
		self.x -= 1
	}
}

impl State {

	pub fn new() -> Self {
		return Self {
	        y: 1,
	        // Game over will be set
	        // to pause the games animation
	        // from happenning
	        game_over: true,
	        going_down: true,
	        jumping: false,
	        obs : Vec::new(),
	        time_since_spawn: Instant::now(),
		}
	}

	fn game_scene(&mut self, ctx: &mut Rltk) {
  
	   
	    let elapsed_timed = self.time_since_spawn.elapsed();


	    if elapsed_timed.as_secs() > 2 {
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
		// Filter obstacles out of screen
		self.obs.retain(|o| o.x > 0);

		for o in self.obs.iter_mut() {

			if o.x < 0 {
				continue
			}

			let rect = Rect {
				x1: o.x - 2,
				y1: o.y - (o.height/2),
				x2: o.x + 2,
				y2: o.y + (o.height/2),
			};

			if rect.contains(5, self.y) {
				self.game_over = true
			}
			

			ctx.draw_box_double(
			    o.x,
			    o.y,
			    3,
			    o.height,
			    RGB::named(BLUE),
			    RGB::named(BLUE)
			);
			o.move_left();
		}

		
	}

	pub fn spawn_enemy(&mut self) {

		if self.game_over {
    		return;
    	}

    	let mut rng = RandomNumberGenerator::new();
    	let height = rng.range(3, 25);

		self.obs.push(Obstacle{
        	y:  40 - height, 
        	x : 70,
        	height: height
        });


	}

	fn start_screen(&mut self, ctx: &mut Rltk) {

		ctx.cls_bg(RGB::named(BLACK));

		ctx.printer(
            40,
            10,
            "#[white]Press the Up or Spacebar key to start",
            TextAlign::Right,
            None,
        );
	}

	
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {

        ctx.set_scale(
        	1.05,
        	1,
        	1
        );

        player_input(self, ctx);

        if !self.game_over {
        	self.game_scene(ctx);
        } else {
        	self.start_screen(ctx)
        }

        ctx.draw_box(36, 0, 20, 7, RGB::named(WHITE), RGB::named(BLACK));
        ctx.printer(
            55,
            1,
            &format!("#FPS: #[]{}", ctx.fps),
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

	if gs.game_over {
		gs.game_over = false;
		gs.obs = Vec::new();
		return
	}

 	if gs.jumping || gs.y < 34 {
 		return
 	}

 	gs.jumping = true
}


