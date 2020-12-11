pub fn part_1(input: &str) -> i32 {
    input.lines()
        .map(|op| Op::parse(op))
        .take_while(|op| !(op.instruction != Instruction::JMP && op.val < 0))
        .fold(0, |acc, x| acc + x.val)
}

struct Op {
    instruction: Instruction,
    val: i32,
}

impl Op {
    fn parse(s: &str) -> Self {
        let splt = s.split(' ');

        let inst_fromstr = match splt.next() {
            Some("nop") => Instruction::NOP,
            Some("jmp") => Instruction::JMP,
            Some("acc") => Instruction::ACC,
            _ => panic!("invalid"),
        };
        let val_fromstr = splt.next().unwrap().parse::<i32>().unwrap();

        Op {
            instruction: inst_fromstr,
            val: val_fromstr
        }
    }
}

#[derive(PartialEq)]
enum Instruction {
    NOP,
    JMP,
    ACC,
}