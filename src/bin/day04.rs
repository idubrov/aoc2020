use std::collections::HashMap;

const KEYS_1: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn is_valid(pass: &HashMap<String, String>) -> bool {
  KEYS_1.iter().all(|k| pass.contains_key(*k))
}

fn is_valid_2(pass: &HashMap<String, String>) -> bool {
  let color: regex::Regex = regex::Regex::new("^#[0-9a-f]{6}$").unwrap();
  let pid: regex::Regex = regex::Regex::new("^[0-9]{9}$").unwrap();

  if !is_valid(pass) {
    return false;
  }
  let byr = pass["byr"].parse::<usize>().unwrap_or(0);
  let iyr = pass["iyr"].parse::<usize>().unwrap_or(0);
  let eyr = pass["eyr"].parse::<usize>().unwrap_or(0);

  let hgt = &pass["hgt"];
  let hgt_valid = if hgt.ends_with("cm") {
    let hgt = hgt[0..hgt.len() - 2].parse::<usize>().unwrap_or(2);
    hgt >= 150 && hgt <= 193
  } else if hgt.ends_with("in") {
    let hgt = hgt[0..hgt.len() - 2].parse::<usize>().unwrap_or(2);
    hgt >= 59 && hgt <= 76
  } else {
    false
  };

  byr >= 1920
    && byr <= 2002
    && iyr >= 2010
    && iyr <= 2020
    && eyr >= 2020
    && eyr <= 2030
    && hgt_valid
    && color.is_match(&pass["hcl"])
    && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&pass["ecl"].as_str())
    && pid.is_match(&pass["pid"])
}

fn solve(input: &str) {
  let passports = std::fs::read_to_string(input)
    .unwrap()
    .split("\n\n")
    .map(|pass| {
      let mut map = HashMap::new();
      pass.lines().flat_map(|line| line.split(" ")).for_each(|val| {
        let (key, value) = val.split_once(":").unwrap();
        map.insert(key.to_owned(), value.to_owned());
      });
      map
    })
    .collect::<Vec<_>>();

  let first = passports.iter().filter(|x| is_valid(*x)).count();

  println!("{} (first): {}", input, first);

  let second = passports.iter().filter(|x| is_valid_2(*x)).count();

  println!("{} (second): {}", input, second);
}

fn main() {
  solve("src/bin/day04/test.txt");
  solve("src/bin/day04/input.txt");
}
