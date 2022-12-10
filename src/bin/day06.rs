use anyhow::Error;
use std::collections::HashSet;

fn solve(input: &str) -> Result<(), Error> {
  let answers = std::fs::read_to_string(input)?;
  let answers = answers
    .split("\n\n")
    .map(|ch| ch.lines().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let unique = answers
    .iter()
    .map(|g| g.iter().flat_map(|g| g.chars()).collect::<HashSet<char>>().len())
    .sum::<usize>();

  let second = answers
    .iter()
    .map(|g| {
      let mut sets = g.iter().map(|g| g.chars().collect::<HashSet<char>>());
      sets
        .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<_>>())
        .unwrap()
        .len()
    })
    .sum::<usize>();

  println!("{} (first): {}", input, unique);
  println!("{} (second): {}", input, second);
  Ok(())
}

fn main() -> Result<(), Error> {
  solve("src/bin/day06/test.txt")?;
  solve("src/bin/day06/input.txt")?;
  Ok(())
}
