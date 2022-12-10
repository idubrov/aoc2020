use std::ops::Index;

#[derive(Clone)]
pub struct CharMap {
  map: Vec<Vec<u8>>,
  tmp: Vec<Vec<u8>>,
  border: u8,
}

pub type Pos = (isize, isize);
pub type Dir = (isize, isize);

impl Index<Pos> for CharMap {
  type Output = u8;

  fn index(&self, (x, y): Pos) -> &Self::Output {
    if x < 0 || x >= self.map[0].len() as isize || y < 0 || y >= self.map.len() as isize {
      &self.border
    } else {
      &self.map[y as usize][x as usize]
    }
  }
}

pub fn dir8() -> impl Iterator<Item = Pos> {
  (-1..=1).flat_map(move |dy| (-1..=1).filter(move |dx| (*dx != 0 || dy != 0)).map(move |dx| (dx, dy)))
}

pub fn cast8((mut x, mut y): (isize, isize), (dx, dy): (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
  std::iter::from_fn(move || {
    let res = (x, y);
    x += dx;
    y += dy;
    Some(res)
  })
}

pub fn find_cast8(map: &CharMap, pos: Pos, dir: &Pos, cond: impl Fn(u8) -> bool) -> Option<u8> {
  cast8(pos, *dir)
    .skip(1)
    .take_while(|pos| map.in_bounds(*pos))
    .map(|pos| map[pos])
    .find(|ch| cond(*ch))
}

impl CharMap {
  pub fn from_text(text: &str) -> Self {
    let map = text.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<_>>();
    CharMap {
      tmp: map.clone(),
      map,
      border: 0,
    }
  }

  pub fn border(mut self, border: u8) -> Self {
    self.border = border;
    self
  }

  pub fn count_adjacent8(&self, pos: Pos, ch: u8) -> usize {
    dir8().filter(|dir| self[(pos.0 + dir.0, pos.1 + dir.1)] == ch).count()
  }

  pub fn in_bounds(&self, (x, y): Pos) -> bool {
    x >= 0 && y >= 0 && x < self.map[0].len() as isize && y < self.map.len() as isize
  }

  pub fn advance(&mut self, update_cb: impl Fn(&CharMap, Pos) -> u8) -> bool {
    let mut changes = false;
    for y in 0..self.map.len() {
      for x in 0..self.map[0].len() {
        let updated = update_cb(&self, (x as isize, y as isize));
        if self.map[y][x] != updated {
          changes = true;
        }
        self.tmp[y][x] = updated;
      }
    }
    std::mem::swap(&mut self.map, &mut self.tmp);
    changes
  }

  pub fn count(&self, ch: u8) -> usize {
    self
      .map
      .iter()
      .map(|line| line.iter().filter(|ch2| ch == **ch2).count())
      .sum::<usize>()
  }
}

impl std::fmt::Display for CharMap {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for line in &self.map {
      for item in line {
        write!(f, "{}", *item as char)?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}
