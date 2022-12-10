fn parse(pass: &str) -> usize {
  let row = usize::from_str_radix(&pass[0..7].replace('F', "0").replace('B', "1"), 2).unwrap();
  let column = usize::from_str_radix(&pass[7..10].replace('L', "0").replace('R', "1"), 2).unwrap();
  row * 8 + column
}

fn solve(input: &str) {
  let mut seats = std::fs::read_to_string(input)
    .unwrap()
    .lines()
    .map(parse)
    .collect::<Vec<_>>();
  seats.sort();

  let seat = seats.windows(2)
    .filter(|s| s[0] + 2 == s[1])
    .map(|s| s[0] + 1)
    .next()
    .unwrap();
  println!("{} (first): {}", input, seats.iter().max().unwrap());
  println!("{} (second): {}", input, seat);
}

fn main() {
  solve("day05/src/input.txt");
}