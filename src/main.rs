#![feature(globs)]
extern crate debug;

extern crate graphics;
extern crate piston;
extern crate opengl_graphics;
extern crate sdl2_game_window;

use std::os;

use graphics::*;
use opengl_graphics::{Gl, Texture};
use piston::*;
use sdl2_game_window::GameWindowSDL2;


use lightsout::{StructLevel, Level};
use number_renderer::NumberRenderer;
use game::Game;

mod lightsout;
mod number_renderer;
mod game;


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

fn render_score(score: int, bg: &Texture, nr: &NumberRenderer, c: &Context, gl: &mut Gl) {
	c.rect(800.0-160.0, 0.0, 160.0, 600.0)
		.image(bg)
		.draw(gl);
	nr.render(score as u32, 715.0, 100.0, 170.0, [1.0, 1.0, 1.0], c, gl);
}

fn win_screen<T: GameWindow>(gameiter: &mut GameIterator<T>, game: &Game, assets: &AssetStore, gl: &mut Gl) {
	let win = Texture::from_path(&assets.path("win.png").unwrap()).unwrap();
	let bg = Texture::from_path(&assets.path("bg.png").unwrap()).unwrap();
	let nr = NumberRenderer::new(assets);
	let mut t = 0.0f64;

	for event in *gameiter {
		match event {
			Render(args) => {
				let c = Context::abs(args.width as f64, args.height as f64);
				c.rect(200.0, 175.0, 400.0, 250.0)
					.image(&win)
					.draw(gl);
				nr.render(game.score as u32, 400.0, 350.0, 200.0, [1.0, 1.0, 1.0], &c, gl);
				render_score(game.score, &bg, &nr, &c, gl);
			},
			Update(args) => {
				t += args.dt;
			},
			MousePress(_)
			| KeyPress(_) => {
				if t > 0.6 {
					return;
				}
			},
			_ => {},
		}
	}
}

fn start_screen<T: GameWindow>(gameiter: &mut GameIterator<T>, assets: &AssetStore, gl: &mut Gl) {
	let msg = Texture::from_path(&assets.path("start.png").unwrap()).unwrap();
	let mut t = 0.0f64;

	for event in *gameiter {
		match event {
			Render(args) => {
				let c = Context::abs(args.width as f64, args.height as f64);
				c.rect(200.0, 175.0, 400.0, 250.0)
					.image(&msg)
					.draw(gl);
			},
			Update(args) => {
				t += args.dt;
			},
			MousePress(_)
			| KeyPress(_) => {
				if t > 1.0 {
					return;
				}
			},
			_ => {},
		}
	}
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
		updates_per_second: 30,
		max_frames_per_second: 60,
	};

	let ref mut gl = Gl::new();

	let mut env = Env {
		mousex: 0f64,
		mousey: 0f64,
		window_width: window.get_settings().size[0] as f64,
		window_height: window.get_settings().size[1] as f64,
	};

	let assets = AssetStore::from_folder("../bin/assets");
	let nr = NumberRenderer::new(&assets);
	let bg = Texture::from_path(&assets.path("bg.png").unwrap()).unwrap();

	let mut game = Game::new(2u, 2u);
	let mut gameiter = GameIterator::new(&mut window, &game_iter_settings);

	start_screen(&mut gameiter, &assets, gl);

	for event in gameiter {
		match event {
			Render(args) => {
				gl.viewport(0, 0, args.width as i32, args.height as i32);
				let c = Context::abs(args.width as f64, args.height as f64);
				clear_board(9, 9, &c, gl);
				render_level(&game.level, &env, &c, gl);
				render_score(game.score, &bg, &nr, &c, gl);
			},
			Update(args) => {
				game.update(args.dt);
				if game.level.is_solved() {
					win_screen(&mut gameiter, &game, &assets, gl);
					if !game.change_level_size(1, 1) {
						game.restart(2, 2);
					}
				}
			},
			MouseMove(args) => {
				env.mousex = args.x;
				env.mousey = args.y;
			},
			MousePress(args) => {
				println!("{:?} {:?}", args, env);
				match mouse_to_level(&game.level, env.mousex, env.mousey) {
					Some((x, y)) => { game.level.make_move(x, y); game.add_score(-1); },
					_ => {}
				}
			},
			KeyPress(args) => {
				println!("{:?} {:?}", args, env);
				match args.key {
					keyboard::Space => { game.make_ai_move(); game.add_score(-3); },
					keyboard::Up => { game.change_level_size(0, -1); },
					keyboard::Right => { game.change_level_size(1, 0); },
					keyboard::Down => { game.change_level_size(0, 1); },
					keyboard::Left => { game.change_level_size(-1, 0); },
					keyboard::D1 => match mouse_to_level(&game.level, env.mousex, env.mousey) {
						Some((x, y)) => { game.level.set(x, y, 0); },
						_ => {},
					},
					keyboard::D2 => match mouse_to_level(&game.level, env.mousex, env.mousey) {
						Some((x, y)) => { game.level.set(x, y, 1); },
						_ => {},
					},
					_ => {}
				}
			},
			_ => {}
		}
	}
}
