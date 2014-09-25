#![feature(globs)]
extern crate debug;

extern crate piston;
extern crate opengl_graphics;
extern crate sdl2;
extern crate sdl2_game_window;
extern crate sdl2_mixer;

use std::os;

use piston::graphics::*;
use piston::input::*;
use opengl_graphics::{Gl, Texture};
use piston::*;
use sdl2_game_window::WindowSDL2;


use lightsout::{StructLevel, Level};
use number_renderer::NumberRenderer;
use game::Game;

mod lightsout;
mod number_renderer;
mod game;


struct ScreenFn(fn(event: &Event, game: &mut Game, env: &mut Env, assets: &AssetStore, gl: &mut Gl) -> ScreenFn);


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


fn win_screen(event: &Event, game: &mut Game, env: &mut Env, assets: &AssetStore, gl: &mut Gl) -> ScreenFn {
	let win = Texture::from_path(&assets.path("win.png").unwrap()).unwrap();
	let bg = Texture::from_path(&assets.path("bg.png").unwrap()).unwrap();
	let nr = NumberRenderer::new(assets);
	let mut t = 0.0f64;

	match *event {
		Render(args) => {
			let c = Context::abs(args.width as f64, args.height as f64);
			c.rgb(0.0, 0.0, 0.0).draw(gl);
			c.rect(200.0, 175.0, 400.0, 250.0)
				.image(&win)
				.draw(gl);
			nr.render(game.score as u32, 400.0, 350.0, 200.0, [1.0, 1.0, 1.0], &c, gl);
			render_score(game.score, &bg, &nr, &c, gl);
		},
		Update(args) => {
			t += args.dt;
		},
		Input(Press(_)) => {
			if t > 0.6 {
				return ScreenFn(game_screen);
			}
		},
		_ => {},
	}
	ScreenFn(win_screen)
}


fn start_screen<T: Window>(gameiter: &mut EventIterator<T>, assets: &AssetStore, gl: &mut Gl) {
	let msg = Texture::from_path(&assets.path("start.png").unwrap()).unwrap();
	let mut t = 0.0f64;

	for event in *gameiter {
		match event {
			Render(args) => {
				let c = Context::abs(args.width as f64, args.height as f64);
				c.rgb(0.0, 0.0, 0.0).draw(gl);
				c.rect(200.0, 175.0, 400.0, 250.0)
					.image(&msg)
					.draw(gl);
			},
			Update(args) => {
				t += args.dt;
			},
			Input(Press(_)) => {
				if t > 1.0 {
					return;
				}
			},
			_ => {},
		}
	}
}


fn init_audio() {
	// use directsound if possible. xaudio2 doesn't work for some reason.
	for i in range(0, sdl2::audio::get_num_audio_drivers()) {
		if "directsound" == sdl2::audio::get_audio_driver(i).as_slice() {
			sdl2::audio::audio_init("directsound").unwrap();
			break;
		}
	}
	println!("audio: {}", sdl2::audio::get_current_audio_driver());
	println!("inited => {}", sdl2_mixer::init(sdl2_mixer::InitMp3 | sdl2_mixer::InitOgg).bits());
	// TODO: 0x8010 is SDL_audio flag
	sdl2_mixer::open_audio(sdl2_mixer::DEFAULT_FREQUENCY, 0x8010u16, 2, 1024).unwrap();
	sdl2_mixer::allocate_channels(2);
}


fn game_screen(event: &Event, game: &mut Game, env: &mut Env, assets: &AssetStore, gl: &mut Gl) -> ScreenFn {
	let nr = NumberRenderer::new(assets);
	let bg = Texture::from_path(&assets.path("bg.png").unwrap()).unwrap();

	let snd_click = sdl2_mixer::Chunk::from_file(&assets.path("click.ogg").unwrap()).unwrap();
	let snd_ai = sdl2_mixer::Chunk::from_file(&assets.path("ai.ogg").unwrap()).unwrap();
	let snd_tick = sdl2_mixer::Chunk::from_file(&assets.path("tick.ogg").unwrap()).unwrap();
	let channel_all = sdl2_mixer::Channel::all();

	match *event {
		Render(args) => {
			gl.viewport(0, 0, args.width as i32, args.height as i32);
			let c = Context::abs(args.width as f64, args.height as f64);
			c.rgb(0.0, 0.0, 0.0).draw(gl);
			render_level(&game.level, env, &c, gl);
			render_score(game.score, &bg, &nr, &c, gl);
		},
		Update(args) => {
			game.update(args.dt);
			if game.ticked() {
				channel_all.play(&snd_tick, 0);
			}

			if game.level.is_solved() {
				// let snd_win = sdl2_mixer::Chunk::from_file(&assets.path("win.ogg").unwrap()).unwrap();
				// let channel_all = sdl2_mixer::Channel::all();
				// channel_all.play(&snd_win, 0);

				// win_screen(event, &game, &assets, gl);

				if !game.change_level_size(1, 1, true) {
					game.restart(2, 2, true);
				}
			}
		},
		Input(Move(MouseCursor(x, y))) => {
			env.mousex = x;
			env.mousey = y;
		},
		Input(Press(Mouse(args))) => {
			println!("{:?} {:?}", args, env);
			match mouse_to_level(&game.level, env.mousex, env.mousey) {
				Some((x, y)) => {
					game.level.make_move(x, y);
					game.add_score(-1);
					channel_all.play(&snd_click, 0);
				},
				_ => {}
			}
		},
		Input(Press(Keyboard(key))) => {
			println!("{:?} {:?}", key, env);
			match key {
				keyboard::Space => {
					game.make_ai_move();
					game.add_score(-3);
					channel_all.play(&snd_ai, 0);
				},
				keyboard::Up => { game.change_level_size(0, -1, false); game.add_score(-50); },
				keyboard::Right => { game.change_level_size(1, 0, false); game.add_score(-50); },
				keyboard::Down => { game.change_level_size(0, 1, false); game.add_score(-50); },
				keyboard::Left => { game.change_level_size(-1, 0, false); game.add_score(-50); },
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
	ScreenFn(game_screen)
}

fn main() {
	let mut window = WindowSDL2::new(
		shader_version::opengl::OpenGL_3_2,
		WindowSettings {
			title: "Rust Lights Out".to_string(),
			size: [800u32, 600u32],
			fullscreen: false,
			exit_on_esc: true,
			samples: 4,
		}
	);
	println!("window: {:?}", window);

	let game_iter_settings = EventSettings {
		updates_per_second: 30,
		max_frames_per_second: 60,
	};

	let ref mut gl = Gl::new(shader_version::opengl::OpenGL_3_2);

	let mut env = Env {
		mousex: 0f64,
		mousey: 0f64,
		window_width: window.get_settings().size[0] as f64,
		window_height: window.get_settings().size[1] as f64,
	};

	init_audio();

	let assets = AssetStore::from_folder("../bin/assets");

	let mut game = Game::new(2u, 2u);
	let mut gameiter = EventIterator::new(&mut window, &game_iter_settings);

	start_screen(&mut gameiter, &assets, gl);

	let mut current_handler:ScreenFn = ScreenFn(game_screen);

	for event in gameiter {
		let ScreenFn(chf) = current_handler;
		current_handler = chf(&event, &mut game, &mut env, &assets, gl);
	}
}
