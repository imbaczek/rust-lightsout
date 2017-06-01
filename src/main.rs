extern crate gfx_core;
extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2;
extern crate sdl2_window;

extern crate gfx_graphics;
extern crate gfx;
extern crate gfx_device_gl;
extern crate music;

use std::os;
use std::path::Path;

use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;
use graphics::{Image, Context, clear, DrawState, rectangle};
use opengl_graphics::{GlGraphics};
use piston::*;
use piston::input::*;
use piston::input::Input::*;
use piston_window::{Window, G2dTexture, WindowSettings, Events, EventLoop,
        EventSettings, PistonWindow, Flip, TextureSettings, OpenGL};
use sdl2::mixer;
use sdl2_window::Sdl2Window;


use lightsout::{StructLevel, Level};
use number_renderer::NumberRenderer;
use game::Game;

mod lightsout;
mod number_renderer;
mod game;


// workaround for a link failure
// mod link {
//      // #[link(name="SDL2_mixer")]
//      extern {}
// }


#[derive(Debug)]
struct Env {
        mousex: f64,
        mousey: f64,
        window_width: f64,
        window_height: f64,
}


#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Music;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {
	Win,
	Tick,
	Click,
	Ai,
}


#[inline(always)]
fn pt_in_rect(px:f64, py:f64, rx: f64, ry:f64, w:f64, h:f64) -> bool {
        px >= rx && px <= rx + w && py >= ry && py <= ry + h
}

fn render_level(level: &Level, env: &Env, c: &Context, gl: &mut GfxGraphics<Resources, CommandBuffer>) {
        let (sx, sy) = level.size();
        let margin = 10.0;
        let w = 60.0;
        let h = 50.0;
        for y in 0 .. sy {
                for x in 0 .. sx {
                        let cx = margin + (margin + w) * (x as f64);
                        let cy = margin + (margin + h) * (y as f64);
                        if pt_in_rect(env.mousex, env.mousey, cx, cy, w, h) {
                                rectangle([1.0, 1.0, 0.0, 1.0],
                                          [cx-1.0, cy-1.0, w+2.0, h+2.0],
                                          c.transform,
                                          gl);
                        }
                        rectangle(
                                if level.get(x, y).unwrap() == 0 {
                                        [0.4, 0.4, 0.4, 1.0]
                                } else {
                                        [0.8, 0.8, 0.8, 1.0]
                                },
                                [cx, cy, w, h],
                                c.transform,
                                gl);
                }
        }
}

fn mouse_to_level(level: &Level, mx: f64, my: f64) -> Option<(usize, usize)> {
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
        let ulx = lx as usize;
        let uly = ly as usize;
        let sx = margin + (margin + w) * (ulx as f64);
        let sy = margin + (margin + h) * (uly as f64);
        if pt_in_rect(mx, my, sx, sy, w, h) {
                Some((ulx, uly))
        } else {
                None
        }
}

fn render_score(score: isize, bg: &G2dTexture, nr: &NumberRenderer, c: &Context, gl: &mut GfxGraphics<Resources, CommandBuffer>) {
        let img = Image::new().rect([800.0-160.0, 0.0, 160.0, 600.0]);
        img .draw(bg, &DrawState::default(), c.transform, gl);
        nr.render(score as u32, 715.0, 100.0, 170.0, [1.0, 1.0, 1.0], c, gl);
}


fn win_screen(window: &mut PistonWindow<Sdl2Window>, game: &mut Game, assets: &Path, gl: &mut GlGraphics) {
        let win = G2dTexture::from_path(&mut window.factory, &assets.join("win.png"), Flip::None, &TextureSettings::new()).unwrap();
        let imgwin = Image::new().rect([200.0, 175.0, 400.0, 250.0]);
        let bg = G2dTexture::from_path(&mut window.factory, &assets.join("bg.png"), Flip::None, &TextureSettings::new()).unwrap();
        let nr = NumberRenderer::new(window, assets);
        let mut t = 0.0f64;

        while let Some(event) = window.next() {
                match event {
                        Render(args) => {
                                window.draw_2d(&event, |c, gl| {
                                        clear([0., 0., 0., 0.], gl);
                                        imgwin.draw(&win, &DrawState::default(), c.transform, gl);
                                        nr.render(game.score as u32, 400.0, 350.0, 200.0, [1.0, 1.0, 1.0], &c, gl);
                                        render_score(game.score, &bg, &nr, &c, gl);
                                });
                        },
                        Update(args) => {
                                t += args.dt;
                        },
                        Press(_) => {
                                if t > 0.6 {
                                        return;
                                }
                        },
                        _ => {},
                }
        }
}


fn start_screen(window: &mut PistonWindow<Sdl2Window>, evloop: &Events, assets: &Path, gl: &mut GlGraphics) {
        let msg = G2dTexture::from_path(&mut window.factory, &assets.join("start.png"), Flip::None, &TextureSettings::new()).unwrap();
        let mut t = 0.0f64;
        let img = Image::new().rect([200., 175., 400., 250.]);

        while let Some(event) = window.next() {
                match event {
                        Render(args) => {
                                window.draw_2d(&event, |c, gl| {
                                        clear([0., 0., 0., 0.], gl);
                                        img.draw(&msg, &DrawState::default(), c.transform, gl);
                                });
                        },
                        Update(args) => {
                                t += args.dt;
                        },
                        Press(_) => {
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
        // println!("audio disabled temporarily");
        // for i in 0 .. sdl2::audio::get_num_audio_drivers() {
        //      if "directsound" == sdl2::audio::get_audio_driver(i).as_slice() {
        //              sdl2::audio::audio_init("directsound").unwrap();
        //              break;
        //      }
        // }
        // println!("audio: {}", sdl2::audio::get_current_audio_driver());
        let _ = mixer::init(mixer::INIT_OGG);
        // TODO: 0x8010 is SDL_audio flag
        mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, mixer::DEFAULT_CHANNELS, 1024).unwrap();
        mixer::allocate_channels(4);
}


fn game_screen(window: &mut PistonWindow<Sdl2Window>, game: &mut Game, assets: &Path, gl: &mut GlGraphics) {
        let nr = NumberRenderer::new(window, assets);
        let bg = G2dTexture::from_path(&mut window.factory, &assets.join("bg.png"), Flip::None, &TextureSettings::new()).unwrap();

        let mut env = Env {
                mousex: 0.,
                mousey: 0.,
                window_width: 800.,      // TODO fixme
                window_height: 600.,
        };

        let snd_click = mixer::Chunk::from_file(&assets.join("click.ogg")).unwrap();
        let snd_ai = mixer::Chunk::from_file(&assets.join("ai.ogg")).unwrap();
        let snd_tick = mixer::Chunk::from_file(&assets.join("tick.ogg")).unwrap();
        let channel_all = mixer::Channel::all();

        while let Some(event) = window.next() {
                match event {
                        Render(args) => {
                                window.draw_2d(&event, |c, gl| {
                                        clear([0.0, 0.0, 0.0, 1.0], gl);
                                        render_level(&game.level, &env, &c, gl);
                                        render_score(game.score, &bg, &nr, &c, gl);
                                });
                        },
                        Update(args) => {
                                game.update(args.dt);
                                if game.ticked() {
                                        channel_all.play(&snd_tick, 0);
					// music::play_sound(&Sound::Tick, music::Repeat::Times(1));
                                }

                                if game.level.is_solved() {
                                        let snd_win = mixer::Chunk::from_file(&assets.join("win.ogg")).unwrap();
                                        let channel_all = mixer::Channel::all();
                                        channel_all.play(&snd_win, 0);

                                        win_screen(window, game, &assets, gl);

                                        if !game.change_level_size(1, 1, true) {
                                                game.restart(2, 2, true);
                                        }
                                }
                        },
                        Move(Motion::MouseCursor(x, y)) => {
                                env.mousex = x;
                                env.mousey = y;
                        },
                        Press(Button::Mouse(args)) => {
                                println!("{:?} {:?}", args, env);
                                match mouse_to_level(&game.level, env.mousex, env.mousey) {
                                        Some((x, y)) => {
                                                game.level.make_move(x, y);
                                                game.add_score(-1);
                                                channel_all.play(&snd_click, 0);
						// music::play_sound(&Sound::Click, music::Repeat::Times(1));
                                        },
                                        _ => {}
                                }
                        },
                        Press(Button::Keyboard(key)) => {
                                println!("{:?} {:?}", key, env);
                                match key {
                                        Key::Space => {
                                                game.make_ai_move();
                                                game.add_score(-3);
                                                channel_all.play(&snd_ai, 0);
						// music::play_sound(&Sound::Ai, music::Repeat::Times(1));
                                        },
                                        Key::Up => { game.change_level_size(0, -1, false); game.add_score(-50); },
                                        Key::Right => { game.change_level_size(1, 0, false); game.add_score(-50); },
                                        Key::Down => { game.change_level_size(0, 1, false); game.add_score(-50); },
                                        Key::Left => { game.change_level_size(-1, 0, false); game.add_score(-50); },
                                        Key::D1 => match mouse_to_level(&game.level, env.mousex, env.mousey) {
                                                Some((x, y)) => { game.level.set(x, y, 0); },
                                                _ => {},
                                        },
                                        Key::D2 => match mouse_to_level(&game.level, env.mousex, env.mousey) {
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

fn main() {
        let opengl = OpenGL::V4_0;

        let mut window: PistonWindow<Sdl2Window> =
                WindowSettings::new(
                         "Rust Lights Out",
                        (800, 600))
                .fullscreen(false)
                .exit_on_esc(true)
                .samples(4)
                .opengl(opengl)
                .build().unwrap();

        let ev_settings = EventSettings::new().ups(30).max_fps(60);
        let ev_loop = Events::new(ev_settings);

        let ref mut gl = GlGraphics::new(opengl);

        let mut env = Env {
                mousex: 0f64,
                mousey: 0f64,
                window_width: window.size().width as f64,
                window_height: window.size().height as f64,
        };

        init_audio();

        let assets = Path::new("bin/assets");

        let mut game = Game::new(2usize, 2usize);

	println!("pwd: {:?}", std::env::current_dir());
	// music::bind_sound_file(Sound::Click, assets.join("click.ogg"));
	// music::bind_sound_file(Sound::Tick, assets.join("tick.ogg"));
	// music::bind_sound_file(Sound::Ai, assets.join("ai.ogg"));
	// music::bind_sound_file(Sound::Win, assets.join("win.ogg"));
	start_screen(&mut window, &ev_loop, &assets, gl);
	game_screen(&mut window, &mut game, &assets, gl);
}
