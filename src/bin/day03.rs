fn slope(map: &[Vec<u8>], dx: usize, dy: usize) -> usize {
  (0..(map.len() / dy))
    .map(|i| map[i * dy][(i * dx) % map[0].len()] == b'#')
    .filter(|x| *x)
    .count()
}

fn solve(input: &str) {
  let map = std::fs::read_to_string(input)
    .unwrap()
    .lines()
    .map(|s| s.as_bytes().to_vec())
    .collect::<Vec<Vec<u8>>>();
  println!("{} (first): {}", input, slope(&map, 3, 1));

  let second = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    .iter()
    .map(|(dx, dy)| slope(&map, *dx, *dy))
    .product::<usize>();
  println!("{} (second): {}", input, second);
}

fn main() {
  solve("src/bin/day03/test.txt");
  solve("src/bin/day03/input.txt");
}