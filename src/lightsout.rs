#[deriving(Show)]

pub trait Level {
	fn new(x:uint, y:uint) -> Self;
	fn size(&self) -> (uint, uint);
	fn is_solved(&self) -> bool;
	fn make_move<'a>(&'a mut self, x:uint, y:uint) -> &'a mut Self;
	fn get(&self, x:uint, y:uint) -> Option<uint>;
	fn set(&mut self, x:uint, y:uint, v:uint) -> bool;
}


#[deriving(Show)]
pub struct StructLevel {
	sx: uint,
	sy: uint,
	level: Vec<Vec<uint>>,
}


impl Level for StructLevel {
	fn new(sx:uint, sy:uint) -> StructLevel {
		let v = Vec::from_elem(sy, Vec::from_elem(sx, 0u));
		StructLevel { sx: sx, sy: sy, level: v }
	}

	fn size(&self) -> (uint, uint) {
		(self.sx, self.sy)
	}

	fn get(&self, x:uint, y:uint) -> Option<uint> {
		if x < self.sx && y < self.sy {
			Some(self.level[y][x])
		} else {
			None
		}
	}

	fn set(&mut self, x:uint, y:uint, v:uint) -> bool {
		if x < self.sx && y < self.sy {
			match v {
				0..1 => { *self.level.get_mut(y).get_mut(x) = v; true }
				_ => false
			}
		} else {
			false
		}
	}

	fn is_solved(&self) -> bool {
		for v in self.level.iter() {
			for e in v.iter() {
				if *e != 1 {
					return false
				}
			}
		}
		true
	}

	fn make_move<'a>(&'a mut self, x: uint, y: uint) -> &'a mut StructLevel {
		if x >= self.sx || y >= self.sy {
			return self
		}
		fn switch(this: &mut StructLevel, x:uint, y:uint) {
			match this.get(x, y) {
				Some(1) => this.set(x, y, 0),
				Some(0) => this.set(x, y, 1),
				_ => false
			};
		}
		switch(self, x, y-1);
		switch(self, x-1, y);
		switch(self, x, y);
		switch(self, x+1, y);
		switch(self, x, y+1);
		self
	}
}

pub fn solve(level: &StructLevel) -> Option<Vec<(uint, uint)>> {
	let (sx, sy) = level.size();
	let mut mat = Vec::from_elem(sy*sx, Vec::from_elem(sy*sx, 0u));
	let mut blank:StructLevel = Level::new(sx, sy);
	let mut expected = Vec::from_elem(sx*sy, 0);
	for y in range(0, sy) {
		for x in range(0, sx) {
			let row = y*sx + x;
			blank.make_move(x, y);
			for by in range(0, sy) {
				for bx in range(0, sx) {
					let col = by * sx + bx;
					*mat.get_mut(row).get_mut(col) = blank.get(bx, by).unwrap();
				}
			}
			blank.make_move(x, y);
			*expected.get_mut(row) = level.get(x, y).unwrap() + 1 % 2;
		}
	}
	let sol = gauss_jordan_zf2(mat, expected);
	match sol {
		Some(s) =>  {
			let mut ret = Vec::with_capacity(s.len());
			for i in range(0, s.len()) {
				if s[i] != 0 {
					let x = i % sx;
					let y = i / sx;
					ret.push((x, y));
				}
			}
			Some(ret)
		},
		None => None
	}
}


fn gauss_jordan_zf2(mat: Vec<Vec<uint>>, expected: Vec<uint>) -> Option<Vec<uint>> {
	let mut m = mat.clone();
	let mut bs = expected.clone();
	let rows = m.len();
	if rows == 0 {
		return None
	}
	let cols = m[0].len();
	if cols == 0 {
		return None
	}

	fn swap(m: &mut Vec<Vec<uint>>, bs: &mut Vec<uint>, i: uint, j:uint) {
		if i == j {
			return;
		}
		// XXX is cloning optimal here?
		let tmpi = m.get(i).clone();
		let tmpj = m.get(j).clone();
		*m.get_mut(i) = tmpj;
		*m.get_mut(j) = tmpi;

		let tmp = *bs.get(i);
		*bs.get_mut(i) = *bs.get(j);
		*bs.get_mut(j) = tmp;
	};

	fn add(m: &mut Vec<Vec<uint>>, bs: &mut Vec<uint>, i: uint, j:uint) {
		if i == j {
			fail!("trying to add row to itself");
		}
		for x in range(0, m.get(i).len()) {
			*m.get_mut(i).get_mut(x) += *m.get(j).get(x);
			*m.get_mut(i).get_mut(x) %= 2;
		}
		*bs.get_mut(i) += *bs.get(j);
		*bs.get_mut(i) %= 2;
	};

	for pivot in range(0, rows) {
		// 1. find pivot row
		for i in range(pivot, rows) {
			if m[i][pivot] != 0 {
				swap(&mut m, &mut bs, i, pivot);
				break;
			}
		}
		if m[pivot][pivot] == 0 && bs[pivot] != 0 {
			return None
		}

		// 2. add pivot to all rows that have 1 in this column
		for i in range(0, rows) {
			if i == pivot { continue; }
			if m[i][pivot] != 0 {
				add(&mut m, &mut bs, i, pivot);
			}
		}
	}
	Some(bs)
}

