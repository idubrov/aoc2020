use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum OpKind {
  Nop,
  Acc,
  Jmp,
}

impl OpKind {
  fn rewrite(&mut self) {
    *self = match self {
      OpKind::Acc => OpKind::Acc,
      OpKind::Nop => OpKind::Jmp,
      OpKind::Jmp => OpKind::Nop,
    }
  }
}

#[derive(Debug)]
struct InvalidOp;
impl FromStr for OpKind {
  type Err = InvalidOp;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "nop" => OpKind::Nop,
      "acc" => OpKind::Acc,
      "jmp" => OpKind::Jmp,
      _ => return Err(InvalidOp),
    })
  }
}

#[derive(Debug, Clone, Copy)]
struct Op {
  kind: OpKind,
  value: i32,
}

impl FromStr for Op {
  type Err = InvalidOp;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (op, val) = s.split_once(" ").ok_or(InvalidOp)?;
    Ok(Op {
      kind: op.parse()?,
      value: val.parse().map_err(|_| InvalidOp)?,
    })
  }
}

#[derive(Default)]
struct VM {
  ip: usize,
  acc: i32,
}

impl VM {
  fn advance(&mut self, op: Op) {
    match op.kind {
      OpKind::Nop => {
        self.ip += 1;
      }
      OpKind::Acc => {
        self.acc += op.value;
        self.ip += 1;
      }
      OpKind::Jmp => {
        self.ip = self.ip.wrapping_add(op.value as usize);
      }
    }
  }
}

fn run(code: &[Op]) -> (bool, i32) {
  let mut visited = vec![false; code.len()];
  let mut vm = VM::default();
  let mut last_acc = 0;
  while vm.ip < code.len() && !visited[vm.ip] {
    visited[vm.ip] = true;
    last_acc = vm.acc;
    vm.advance(code[vm.ip]);
  }
  if vm.ip < code.len() {
    (true, last_acc)
  } else {
    (false, vm.acc)
  }
}

fn find(mut code: Vec<Op>) -> i32 {
  for idx in 0..code.len() {
    // jmp <-> nop
    code[idx].kind.rewrite();
    let (lp, acc) = run(&code);
    if !lp {
      return acc;
    }
    code[idx].kind.rewrite();
  }
  unreachable!();
}

fn solve(input: &str) {
  let code = std::fs::read_to_string(input)
    .unwrap()
    .lines()
    .map(|l| l.parse::<Op>().unwrap())
    .collect::<Vec<_>>();
  println!("{} (first): {}", input, run(&code).1);
  println!("{} (second): {}", input, find(code));
}

fn main() {
  solve("src/bin/day08/test.txt");
  solve("src/bin/day08/input.txt");
}
