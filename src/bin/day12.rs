use aoc2020::*;

fn rotate(dir: Dir) -> Dir {
  (-dir.1, dir.0)
}

struct State {
  pos: Pos,
  dir: Dir,
}

impl State {
  fn advance(&mut self, instr: &str) {
    let v = instr[1..].parse::<isize>().unwrap();
    match &instr[0..1] {
      "L" => {
        self.dir = (0..v / 90).fold(self.dir, |d, _| rotate(d));
      }
      "R" => {
        self.dir = (0..(4 - v / 90)).fold(self.dir, |d, _| rotate(d));
      }
      "F" => {
        self.pos = (self.pos.0 + self.dir.0 * v, self.pos.1 + self.dir.1 * v);
      }
      "E" => {
        self.pos.0 += v;
      }
      "N" => {
        self.pos.1 += v;
      }
      "W" => {
        self.pos.0 -= v;
      }
      "S" => {
        self.pos.1 -= v;
      }
      _ => unreachable!(),
    }
  }
}

impl State {
  fn advance2(&mut self, instr: &str) {
    let v = instr[1..].parse::<isize>().unwrap();
    match &instr[0..1] {
      "L" => {
        self.dir = (0..v / 90).fold(self.dir, |d, _| rotate(d));
      }
      "R" => {
        self.dir = (0..(4 - v / 90)).fold(self.dir, |d, _| rotate(d));
      }
      "F" => {
        self.pos = (self.pos.0 + self.dir.0 * v, self.pos.1 + self.dir.1 * v);
      }
      "E" => {
        self.dir.0 += v;
      }
      "N" => {
        self.dir.1 += v;
      }
      "W" => {
        self.dir.0 -= v;
      }
      "S" => {
        self.dir.1 -= v;
      }
      _ => unreachable!(),
    }
  }
}

fn solve(input: &str) {
  let lines = std::fs::read_to_string(input).unwrap();
  let mut state = State {
    pos: (0, 0),
    dir: (1, 0),
  };
  for line in lines.lines() {
    state.advance(line);
  }
  println!("{} (first): {}", input, state.pos.0.abs() + state.pos.1.abs());

  let mut state = State {
    pos: (0, 0),
    dir: (10, 1),
  };
  for line in lines.lines() {
    state.advance2(line);
  }
  println!("{} (first): {}", input, state.pos.0.abs() + state.pos.1.abs());
}

fn main() {
  solve("src/bin/day12/test.txt");
  solve("src/bin/day12/input.txt");
}
