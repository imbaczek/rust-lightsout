pub trait Level {
    fn size(&self) -> (usize, usize);
    fn is_solved(&self) -> bool;
    fn make_move<'a>(&'a mut self, x: usize, y: usize) -> &'a mut Level;
    fn get(&self, x: usize, y: usize) -> Option<usize>;
    fn set(&mut self, x: usize, y: usize, v: usize) -> bool;
}


#[derive(Debug)]
pub struct StructLevel {
    sx: usize,
    sy: usize,
    level: Vec<Vec<usize>>,
}


impl StructLevel {
    pub fn new(sx: usize, sy: usize) -> StructLevel {
        let v = vec![vec![0usize; sx]; sy];
        StructLevel {
            sx: sx,
            sy: sy,
            level: v,
        }
    }
}

impl Level for StructLevel {
    fn size(&self) -> (usize, usize) {
        (self.sx, self.sy)
    }

    fn get(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.sx && y < self.sy {
            Some(self.level[y][x])
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, v: usize) -> bool {
        if x < self.sx && y < self.sy {
            match v {
                0...1 => {
                    *self.level.get_mut(y).unwrap().get_mut(x).unwrap() = v;
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_solved(&self) -> bool {
        for v in self.level.iter() {
            for e in v.iter() {
                if *e != 1 {
                    return false;
                }
            }
        }
        true
    }

    fn make_move<'a>(&'a mut self, x: usize, y: usize) -> &'a mut Level {
        if x >= self.sx || y >= self.sy {
            return self;
        }
        fn switch(this: &mut StructLevel, x: usize, y: usize) {
            match this.get(x, y) {
                Some(1) => this.set(x, y, 0),
                Some(0) => this.set(x, y, 1),
                _ => false,
            };
        }
        if y > 0 {
            switch(self, x, y - 1);
        }
        if x > 0 {
            switch(self, x - 1, y);
        }
        switch(self, x, y);
        switch(self, x + 1, y);
        switch(self, x, y + 1);
        self
    }
}

pub fn solve(level: &StructLevel) -> Option<Vec<(usize, usize)>> {
    let (sx, sy) = level.size();
    let mut mat = vec![vec![0usize; sy*sx]; sy*sx];
    let mut blank = StructLevel::new(sx, sy);
    let mut expected = vec![0; sx * sy];
    for y in 0..sy {
        for x in 0..sx {
            let row = y * sx + x;
            blank.make_move(x, y);
            for by in 0..sy {
                for bx in 0..sx {
                    let col = by * sx + bx;
                    *mat.get_mut(row).unwrap().get_mut(col).unwrap() = blank.get(bx, by).unwrap();
                }
            }
            blank.make_move(x, y);
            *expected.get_mut(row).unwrap() = level.get(x, y).unwrap() + 1 % 2;
        }
    }
    let sol = gauss_jordan_zf2(mat, expected);
    match sol {
        Some(s) => {
            let mut ret = Vec::with_capacity(s.len());
            for i in 0..s.len() {
                if s[i] != 0 {
                    let x = i % sx;
                    let y = i / sx;
                    ret.push((x, y));
                }
            }
            Some(ret)
        }
        None => None,
    }
}


fn gauss_jordan_zf2(mat: Vec<Vec<usize>>, expected: Vec<usize>) -> Option<Vec<usize>> {
    let mut m = mat.clone();
    let mut bs = expected.clone();
    let rows = m.len();
    if rows == 0 {
        return None;
    }
    let cols = m[0].len();
    if cols == 0 {
        return None;
    }

    fn swap(m: &mut Vec<Vec<usize>>, bs: &mut Vec<usize>, i: usize, j: usize) {
        if i == j {
            return;
        }
        // XXX is cloning optimal here?
        let tmpi = m.get(i).unwrap().clone();
        let tmpj = m.get(j).unwrap().clone();
        *m.get_mut(i).unwrap() = tmpj;
        *m.get_mut(j).unwrap() = tmpi;

        let tmp = *bs.get(i).unwrap();
        *bs.get_mut(i).unwrap() = *bs.get(j).unwrap();
        *bs.get_mut(j).unwrap() = tmp;
    };

    fn add(m: &mut Vec<Vec<usize>>, bs: &mut Vec<usize>, i: usize, j: usize) {
        if i == j {
            panic!("trying to add row to itself");
        }
        for x in 0..m.get(i).unwrap().len() {
            *m.get_mut(i).unwrap().get_mut(x).unwrap() += *m.get(j).unwrap().get(x).unwrap();
            *m.get_mut(i).unwrap().get_mut(x).unwrap() %= 2;
        }
        *bs.get_mut(i).unwrap() += *bs.get(j).unwrap();
        *bs.get_mut(i).unwrap() %= 2;
    };

    for pivot in 0..rows {
        // 1. find pivot row
        for i in pivot..rows {
            if m[i][pivot] != 0 {
                swap(&mut m, &mut bs, i, pivot);
                break;
            }
        }
        if m[pivot][pivot] == 0 && bs[pivot] != 0 {
            return None;
        }

        // 2. add pivot to all rows that have 1 in this column
        for i in 0..rows {
            if i == pivot {
                continue;
            }
            if m[i][pivot] != 0 {
                add(&mut m, &mut bs, i, pivot);
            }
        }
    }
    Some(bs)
}
