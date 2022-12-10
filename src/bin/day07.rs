use std::collections::HashMap;

type Color<'a> = (&'a str, &'a str);
type Rule<'a> = HashMap<Color<'a>, usize>;

fn allow_single(rules: &HashMap<Color, Rule>, rule: &Rule, c: Color) -> bool {
  if rule.get(&c).copied().unwrap_or(0) >= 1 {
    return true;
  }
  rule.keys().any(|key| allow_single(rules, &rules[key], c))
}

fn count(rules: &HashMap<Color, Rule>, color: Color) -> usize {
  let rule = &rules[&color];
  rule.iter().map(|(k, cnt)| cnt * (1 + count(rules, *k))).sum::<usize>()
}

fn parse_rule(line: &str) -> (Color, HashMap<Color, usize>) {
  let (l, r) = line.split_once(" contain ").unwrap();
  let mut it = l.trim().split(" ");
  let color = (it.next().unwrap(), it.next().unwrap());
  let mut rule = HashMap::new();
  for item in r.split(",") {
    let item = item.trim();
    if item != "no other bags." {
      let mut it = item.split(" ");
      let d = it.next().unwrap().parse::<usize>().unwrap();
      let m = it.next().unwrap();
      let c = it.next().unwrap();
      rule.insert((m, c), d);
    }
  }
  (color, rule)
}

fn solve(input: &str) {
  let rules = std::fs::read_to_string(input).unwrap();
  let rules = rules.lines().map(parse_rule).collect::<HashMap<_, _>>();
  let first: usize = rules
    .iter()
    .filter(|r| allow_single(&rules, &r.1, ("shiny", "gold")))
    .count();
  println!("{} (first): {}", input, first);

  let count: usize = count(&rules, ("shiny", "gold"));
  println!("{} (second): {}", input, count);
}

fn main() {
  solve("src/bin/day07/test.txt");
  solve("src/bin/day07/test2.txt");
  solve("src/bin/day07/input.txt");
}
