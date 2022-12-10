use aoc2020::*;

fn advance_first(map: &CharMap, pos: Pos) -> u8 {
  let count = map.count_adjacent8(pos, b'#');
  match map[pos] {
    b'L' if count == 0 => b'#',
    b'#' if count >= 4 => b'L',
    ch => ch,
  }
}

fn advance_second(map: &CharMap, pos: Pos) -> u8 {
  let count = dir8()
    .filter(|dir| find_cast8(map, pos, dir, |ch| ch != b'.') == Some(b'#'))
    .count();

  match map[pos] {
    b'L' if count == 0 => b'#',
    b'#' if count >= 5 => b'L',
    ch => ch,
  }
}

fn solve(input: &str) {
  let mut map = CharMap::from_text(&std::fs::read_to_string(input).unwrap()).border(b'.');
  let mut map2 = map.clone();

  while map.advance(advance_first) {}
  let first = map.count(b'#');
  println!("{} (first): {}", input, first);

  while map2.advance(advance_second) {}
  let second = map2.count(b'#');
  println!("{} (second): {}", input, second);
}

fn main() {
  solve("src/bin/day11/test.txt");
  solve("src/bin/day11/input.txt");
}
