
fn solve(input: &str) {
  let expenses = std::fs::read_to_string(input)
    .unwrap()
    .lines()
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<_>>();
  'outer: for i in 0..expenses.len() {
    for j in i + 1..expenses.len() {
      if expenses[i] + expenses[j] == 2020 {
        println!("{}: 2 => {}", input, expenses[i] * expenses[j]);
        break 'outer;
      }
    }
  }

  'outer: for i in 0..expenses.len() {
    for j in i + 1..expenses.len() {
      for k in j + 1..expenses.len() {
        if expenses[i] + expenses[j] + expenses[k] == 2020 {
          println!("{}: 3 => {}", input, expenses[i] * expenses[j] * expenses[k]);
          break 'outer;
        }
      }
    }
  }
}

fn main() {
  solve("src/bin/day01/test.txt");
  solve("src/bin/day01/input.txt");
}