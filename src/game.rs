use std;

use lightsout;
use lightsout::{StructLevel, Level};


pub struct Game {
	pub level: StructLevel,
	pub score: int,
	time: f64,
}

impl Game {
	pub fn new(sx: uint, sy: uint) -> Game {
		Game {
			score: Game::init_score(sx, sy),
			time: 0.0,
			level: Level::new(sx, sy),
		}
	}

	fn init_score(sx: uint, sy: uint) -> int {
		(2 * sx * sy) as int
	}

	pub fn add_score(&mut self, ds: int) {
		self.score = std::cmp::max(0, self.score + ds);
	}

	pub fn update(&mut self, dt: f64) {
		if self.time % 1.0 > (self.time + dt) % 1.0 {
			self.add_score(-1);
		}
		self.time += dt;
	}

	pub fn restart(&mut self, sx: uint, sy: uint) -> bool {
		if sx > 1 && sy> 1 && sx < 10 && sy < 10 {
			self.level = Level::new(sx, sy);
			self.score = Game::init_score(sx, sy) + self.score;
			self.time = 0.0;
			return true
		}
		false
	}

	pub fn make_ai_move(&mut self) {
		let sol = lightsout::solve(&self.level);
		println!("solution: {}", sol);
		match sol {
			Some(moves) => {
				if moves.len() > 0 {
					let move = moves[0];
					let (mx, my) = move;
					self.level.make_move(mx, my);
				}
			},
			None => {}
		};
	}

	pub fn change_level_size(&mut self, dx: int, dy: int) -> bool {
		let (sx, sy) = self.level.size();
		let sx = (sx as int + dx) as uint;
		let sy = (sy as int + dy) as uint;
		self.restart(sx, sy)
	}
}
