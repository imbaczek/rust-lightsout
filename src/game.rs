use std;

use lightsout;
use lightsout::{StructLevel, Level};


pub struct Game {
    pub level: StructLevel,
    pub score: isize,
    time: f64,
    had_tick: bool,
}

impl Game {
    pub fn new(sx: usize, sy: usize) -> Game {
        Game {
            score: Game::init_score(sx, sy),
            time: 0.0,
            level: StructLevel::new(sx, sy),
            had_tick: false,
        }
    }

    fn init_score(sx: usize, sy: usize) -> isize {
        (2 * sx * sy) as isize
    }

    pub fn add_score(&mut self, ds: isize) {
        self.score = std::cmp::max(0, self.score + ds);
    }

    pub fn update(&mut self, dt: f64) {
        self.had_tick = false;
        if self.time % 1.0 > (self.time + dt) % 1.0 {
            self.add_score(-1);
            self.had_tick = true;
        }
        self.time += dt;
    }

    pub fn ticked(&self) -> bool {
        self.had_tick
    }

    pub fn restart(&mut self, sx: usize, sy: usize, score: bool) -> bool {
        if sx > 1 && sy > 1 && sx < 10 && sy < 10 {
            self.level = StructLevel::new(sx, sy);
            self.score = if score { Game::init_score(sx, sy) } else { 0 } + self.score;
            self.time = 0.0;
            return true;
        }
        false
    }

    pub fn make_ai_move(&mut self) {
        let sol = lightsout::solve(&self.level);
        println!("solution: {:?}", sol);
        match sol {
            Some(moves) => {
                if moves.len() > 0 {
                    let (mx, my) = moves[0];
                    self.level.make_move(mx, my);
                }
            }
            None => {}
        };
    }

    pub fn change_level_size(&mut self, dx: isize, dy: isize, score: bool) -> bool {
        let (sx, sy) = self.level.size();
        let sx = (sx as isize + dx) as usize;
        let sy = (sy as isize + dy) as usize;
        self.restart(sx, sy, score)
    }
}
