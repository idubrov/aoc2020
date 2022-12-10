fn solve(input: &str) {
  let mut adapters = std::fs::read_to_string(input)
    .unwrap()
    .lines()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<_>>();
  adapters.push(0);
  adapters.sort();
  adapters.push(adapters.last().unwrap() + 3);

  let mut diffs = [0, 0, 0, 0];
  for pairs in adapters.windows(2) {
    diffs[pairs[1] - pairs[0]] += 1;
  }

  let mut combinations = vec![0usize; *adapters.last().unwrap() + 4];
  combinations[3] = 1;
  for adapter in adapters.iter().skip(1) {
    let adapter = *adapter;
    combinations[adapter + 3] = combinations[adapter] + combinations[adapter + 1] + combinations[adapter + 2];
  }
  println!("{} (first): {}", input, diffs[1] * diffs[3]);
  println!("{} (second): {}", input, combinations[adapters.last().unwrap() + 3]);
}

fn main() {
  solve("src/bin/day10/test1.txt");
  solve("src/bin/day10/test2.txt");
  solve("src/bin/day10/input.txt");
}
