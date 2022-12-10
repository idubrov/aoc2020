use regex::Regex;

fn is_valid(pwd: &str) -> (bool, bool) {
  let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
  let captures = re.captures(pwd).unwrap();
  let min = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
  let max = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
  let ch = captures.get(3).unwrap().as_str().as_bytes()[0];
  let text = captures.get(4).unwrap().as_str().as_bytes();
  let count = text.iter().filter(|c| ch == **c).count();
  (
    count >= min && count <= max,
    (text[min - 1] == ch && text[max - 1] != ch) ||
      (text[min - 1] != ch && text[max - 1] == ch)
  )
}

fn solve(input: &str) {
  let all = std::fs::read_to_string(input)
    .unwrap()
    .lines()
    .map(is_valid)
    .collect::<Vec<_>>();
  println!("{} (first) => {}", input, all.iter().filter(|x| x.0).count());
  println!("{} (second) => {}", input, all.iter().filter(|x| x.1).count());
}

fn main() {
  solve("src/bin/day02/test.txt");
  solve("src/bin/day02/input.txt");
}