fn sum_of(vals: &[usize], val: usize) -> bool {
  for i in 0..vals.len() {
    for j in i + 1..vals.len() {
      if vals[i] + vals[j] == val {
        return true;
      }
    }
  }
  false
}

fn find_non_sum(values: &[usize], preamble: usize) -> usize {
  for idx in preamble..values.len() {
    if !sum_of(&values[idx - preamble..idx], values[idx]) {
      return idx;
    }
  }
  unreachable!()
}

fn find_second(values: &[usize], sum: usize) -> usize {
  for i in 0..values.len() {
    for j in i + 2..values.len() {
      if values[i..j].iter().sum::<usize>() == sum {
        let min = values[i..j].iter().min().unwrap();
        let max = values[i..j].iter().max().unwrap();
        return min + max;
      }
    }
  }
  unreachable!()
}

fn solve(input: &str, preamble: usize) {
  let values = std::fs::read_to_string(input)
    .unwrap()
    .lines()
    .map(|s| s.parse::<usize>().unwrap())
    .collect::<Vec<_>>();
  let idx = find_non_sum(&values, preamble);
  println!("{} (first): {}", input, values[idx]);
  println!("{} (second): {}", input, find_second(&values, values[idx]));
}

fn main() {
  solve("src/bin/day09/test.txt", 5);
  solve("src/bin/day09/input.txt", 25);
}