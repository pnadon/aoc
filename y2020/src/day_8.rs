pub fn part_1(input: &str) -> i32 {
  let mut cpu = Cpu::new(input.lines().map(Op::parse).collect::<Vec<Op>>());

  loop {
    if cpu.process().is_none() {
      break;
    }
  }

  cpu.acc
}

#[derive(Debug)]
struct Cpu {
  acc: i32,
  ptr: usize,
  ops: Vec<Op>,
}

impl Cpu {
  fn new(ops: Vec<Op>) -> Self {
    Self {
      acc: 0,
      ptr: 0,
      ops,
    }
  }

  fn process(&mut self) -> Option<i32> {
    let valid = match self.cur_ins() {
      Instruction::Acc => self.handle_acc(),
      Instruction::Inv => false,
      Instruction::Jmp => self.handle_jmp(),
      Instruction::Nop => self.increment(),
    };
    match valid {
      true => Some(self.acc),
      false => None,
    }
  }

  fn cur_op(&self) -> Op {
    self.ops[self.ptr]
  }

  fn cur_val(&self) -> i32 {
    self.cur_op().val
  }

  fn cur_ins(&self) -> Instruction {
    self.cur_op().instruction
  }

  fn handle_acc(&mut self) -> bool {
    if self.cur_val() > self.acc && self.cur_val() < 0 {
      return false;
    }
    self.acc += self.cur_val();
    self.increment()
  }

  fn handle_jmp(&mut self) -> bool {
    self.consume();
    match self.cur_val() >= 0 {
      true => {
        if self.cur_val() as usize + self.ptr > self.len() {
          false
        } else {
          self.ptr += self.cur_val() as usize;
          true
        }
      }
      false => {
        if self.cur_val().abs() as usize > self.ptr {
          false
        } else {
          self.ptr -= self.cur_val().abs() as usize;
          true
        }
      }
    }
  }

  fn increment(&mut self) -> bool {
    self.consume();
    self.ptr += 1;
    true
  }

  fn consume(&mut self) {
    self.ops[self.ptr].instruction = Instruction::Inv;
  }

  fn len(&self) -> usize {
    self.ops.len()
  }
}

#[derive(Debug, Copy, Clone)]
struct Op {
  instruction: Instruction,
  val: i32,
}

impl Op {
  fn parse(s: &str) -> Self {
    let mut splt = s.split(' ');

    let inst_fromstr = match splt.next() {
      Some("nop") => Instruction::Nop,
      Some("jmp") => Instruction::Jmp,
      Some("acc") => Instruction::Acc,
      _ => panic!("invalid"),
    };
    let val_fromstr = splt.next().unwrap().parse::<i32>().unwrap();

    Op {
      instruction: inst_fromstr,
      val: val_fromstr,
    }
  }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Instruction {
  Nop,
  Jmp,
  Acc,
  Inv,
}
