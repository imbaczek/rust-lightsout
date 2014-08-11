#![feature(globs)]
extern crate debug;

extern crate graphics;
extern crate piston;
extern crate opengl_graphics;
extern crate sdl2_game_window;

use std::os;

use graphics::*;
use opengl_graphics::Gl;
use piston::*;
use sdl2_game_window::GameWindowSDL2;


use lightsout::{StructLevel, Level};

mod lightsout;


fn cmd_main() {
	let sx:Option<uint> = from_str(os::args()[1].as_slice());
	let sx:uint = match sx {
		Some(x) => x,
		None => 4
	};
	let sy:Option<uint> = from_str(os::args()[2].as_slice());
	let sy:uint = match sy {
		Some(y) => y,
		None => 4
	};
	let mut l:StructLevel = Level::new(sx, sy);
	println!("{}", l);
	println!("solved: {}", l.is_solved());

	let sol = lightsout::solve(&mut l);

	println!("{}", sol);
}

struct Env {
	mousex: f64,
	mousey: f64,
	window_width: f64,
	window_height: f64,
}

#[inline(always)]
fn pt_in_rect(px:f64, py:f64, rx: f64, ry:f64, w:f64, h:f64) -> bool {
	px >= rx && px <= rx + w && py >= ry && py <= ry + h
}

fn clear_board(sx: uint, sy: uint, c: &Context, gl: &mut Gl) {
	let margin = 10.0;
	let w = 60.0;
	let h = 50.0;
	c.rect(0.0, 0.0, margin + (margin + w) * (sx as f64) + 1.0, margin + (margin + h) * (sy as f64) + 1.0)
		.rgb(0.0, 0.0, 0.0).draw(gl);
}

fn render_level(level: &Level, env: &Env, c: &Context, gl: &mut Gl) {
	let (sx, sy) = level.size();
	let margin = 10.0;
	let w = 60.0;
	let h = 50.0;
	for y in range(0, sy) {
		for x in range(0, sx) {
			let cx = margin + (margin + w) * (x as f64);
			let cy = margin + (margin + h) * (y as f64);
			if pt_in_rect(env.mousex, env.mousey, cx, cy, w, h) {
				c.rect(cx-1.0, cy-1.0, w+2.0, h+2.0).rgb(1.0, 1.0, 0.0).draw(gl);
			}
			let c = c.rect(cx, cy, w, h);
			if level.get(x, y).unwrap() == 0 {
				c.rgb(0.4, 0.4, 0.4)
			} else {
				c.rgb(0.8, 0.8, 0.8)
			}.draw(gl);
		}
	}
}

fn mouse_to_level(level: &Level, mx: f64, my: f64) -> Option<(uint, uint)> {
	let (sx, sy) = level.size();
	let margin = 10.0;
	let w = 60.0;
	let h = 50.0;
	if !pt_in_rect(mx, my,
		       margin, margin,
		       margin + (w + margin) * sx as f64, margin + (h + margin) * sy as f64) {
		return None
	}

	let lx = (mx - margin) / (margin + w);
	let ly = (my - margin) / (margin + h);
	println!("lx={} ly={}", lx, ly);
	let ulx = lx as uint;
	let uly = ly as uint;
	let sx = margin + (margin + w) * (ulx as f64);
	let sy = margin + (margin + h) * (uly as f64);
	if pt_in_rect(mx, my, sx, sy, w, h) {
		Some((ulx, uly))
	} else {
		None
	}
}

fn make_ai_move(level: &mut StructLevel) {
	let sol = lightsout::solve(level);
	println!("solution: {}", sol);
	match sol {
		Some(moves) => {
			if moves.len() > 0 {
				let move = moves[0];
				let (mx, my) = move;
				level.make_move(mx, my);
			}
		},
		None => {}
	};
}

fn main() {
	let mut window = GameWindowSDL2::new(
		GameWindowSettings {
			title: "Rust Lights Out".to_string(),
			size: [800u32, 600u32],
			fullscreen: false,
			exit_on_esc: true,
		}
	);
	println!("window: {:?}", window);

	let game_iter_settings = GameIteratorSettings {
		updates_per_second: 120,
		max_frames_per_second: 60,
	};

	let ref mut gl = Gl::new();
	let mut level:StructLevel = Level::new(5, 5);

	let mut env = Env {
		mousex: 0f64,
		mousey: 0f64,
		window_width: window.get_settings().size[0] as f64,
		window_height: window.get_settings().size[1] as f64,
	};


	level.make_move(2, 2);

	for event in GameIterator::new(&mut window, &game_iter_settings) {
		match event {
			Render(args) => {
				gl.viewport(0, 0, args.width as i32, args.height as i32);
				let c = Context::abs(args.width as f64, args.height as f64);
				c.rgb(1.0, 0.0, 0.0)
					.rect(0.0, 0.0, 100.0, 100.0)
					.draw(gl);
				clear_board(9, 9, &c, gl);
				render_level(&level, &env, &c, gl);
			},
			MouseMove(args) => {
				env.mousex = args.x;
				env.mousey = args.y;
			},
			MousePress(args) => {
				println!("{:?} {:?}", args, env);
				match mouse_to_level(&level, env.mousex, env.mousey) {
					Some((x, y)) => { level.make_move(x, y); },
					_ => {}
				}
			},
			KeyPress(args) => {
				println!("{:?} {:?}", args, env);
				match args.key {
					keyboard::Space => make_ai_move(&mut level),
					keyboard::Left
					| keyboard::Right 
					| keyboard::Up
					| keyboard::Down => {
						let change_level_size = |dx: uint, dy: uint| {
							let (sx, sy) = level.size();
							if sx + dx > 1 && sy + dy > 1
									&& sx + dx < 10 && sy + dy < 10 {
								level = Level::new(sx + dx, sy + dy);
								println!("new level: {}", level);
							}
						};
						match args.key {
							keyboard::Up => change_level_size(0, -1),
							keyboard::Right => change_level_size(1, 0),
							keyboard::Down => change_level_size(0, 1),
							keyboard::Left => change_level_size(-1, 0),
							_ => {},
						};
					},
					keyboard::D1 => match mouse_to_level(&level, env.mousex, env.mousey) {
						Some((x, y)) => { level.set(x, y, 0); },
						_ => {},
					},
					keyboard::D2 => match mouse_to_level(&level, env.mousex, env.mousey) {
						Some((x, y)) => { level.set(x, y, 1); },
						_ => {},
					},
					_ => {}
				}
			},
			_ => {}
		}
	}
}
