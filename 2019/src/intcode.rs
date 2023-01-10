use num_traits::FromPrimitive;
use std::collections::VecDeque;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
enum Opcode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
enum AddrMode {
    Position = 0,
    Immediate = 1,
}
pub type Word = i64;

#[derive(Debug, Clone)]
struct DecodedOp {
    op: Opcode,
    len: usize,
    modes: Vec<AddrMode>,
    args: Vec<Word>,
}

impl DecodedOp {
    fn args_modes(&self) -> Vec<(Word, AddrMode)> {
        self.args
            .to_owned()
            .into_iter()
            .zip(self.modes.to_owned().into_iter())
            .collect()
    }

    fn op_len(op: Opcode) -> usize {
        match op {
            Opcode::Halt => 1,
            Opcode::Input | Opcode::Output => 2,
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => 3,
            Opcode::Add | Opcode::Mul | Opcode::LessThan | Opcode::Equals => 4,
        }
    }

    fn decode(raw: &[Word]) -> ResultT<Self> {
        let raw_op = raw.get(0).ok_or("Address outof range")?;
        let op =
            Opcode::from_i64(raw_op % 100).ok_or(format!("Unknown opcode: {}", raw_op % 100))?;
        let raw_modes: [Word; 3] = [raw_op / 100 % 10, raw_op / 1000 % 10, raw_op / 10000 % 10];
        let modes: Vec<AddrMode> = raw_modes
            .into_iter()
            .map(|i| AddrMode::from_i64(i))
            .collect::<Option<_>>()
            .ok_or("Unknown addressing mode")?;
        let len = Self::op_len(op);

        Ok(Self {
            len,
            op,
            args: raw[1..len].to_vec(),
            modes,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IntComputerState {
    Reset,
    Continue,
    Halt,
}

#[derive(Debug, PartialEq)]
pub struct IntComputer {
    mem: Vec<Word>,
    pc: usize,
    trace: bool,
    input: VecDeque<Word>,
    output: VecDeque<Word>,
    state: IntComputerState,
}

type ResultT<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl IntComputer {
    pub fn from_str(prog: &str) -> ResultT<Self> {
        let mem: Vec<Word> = prog
            .trim()
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<_, ParseIntError>>()?;
        Ok(Self {
            mem,
            pc: 0,
            trace: false,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: IntComputerState::Reset,
        })
    }

    pub fn from_str_input(prog: &str, input: &[Word]) -> ResultT<Self> {
        let mut c = Self::from_str(prog)?;
        c.input(input);
        Ok(c)
    }

    pub fn trace(&mut self) -> &mut Self {
        self.trace = true;
        self
    }

    pub fn mem(&self) -> Vec<Word> {
        self.mem.to_owned()
    }

    pub fn read_mem(&self, addr: usize) -> Word {
        self.mem[addr]
    }

    pub fn write_mem(&mut self, addr: usize, val: Word) -> &mut Self {
        if self.mem.len() < addr {
            self.mem.resize(addr, 0);
        }
        self.mem[addr] = val;
        self
    }

    pub fn input(&mut self, val: &[Word]) -> &mut Self {
        self.input.extend(val.iter());
        self
    }

    pub fn output(&mut self) -> Vec<Word> {
        self.output.drain(..).collect()
    }

    pub fn step(&mut self) -> ResultT<bool> {
        let op = DecodedOp::decode(&self.mem[self.pc..])?;

        self.state = IntComputerState::Continue;
        self.pc += op.len;

        // Translated arguments (using address modes)
        let targ: Vec<_> = op
            .args_modes()
            .iter()
            .map(|(a, m)| match m {
                AddrMode::Immediate => Ok(*a),
                AddrMode::Position => Ok(self.read_mem((*a).try_into()?)),
            })
            .collect::<ResultT<_>>()?;

        // Original arguments
        let arg = op.args;

        if self.trace {
            println!(
                "PC {} -> {:?}({}) {:?} ({:?})",
                self.pc, op.op, op.len, arg, op.modes
            );
        }

        match op.op {
            Opcode::Add => {
                self.write_mem(arg[2].try_into()?, targ[0] + targ[1]);
                Ok(false)
            }
            Opcode::Mul => {
                self.write_mem(arg[2].try_into()?, targ[0] * targ[1]);
                Ok(false)
            }
            Opcode::Input => {
                let val = self.input.pop_front().ok_or("Input underrun")?;
                self.write_mem(arg[0].try_into()?, val);
                Ok(false)
            }
            Opcode::Output => {
                self.output.push_back(targ[0].try_into()?);
                Ok(false)
            }
            Opcode::JumpIfTrue => {
                if targ[0] != 0 {
                    self.pc = targ[1].try_into()?;
                }
                Ok(false)
            }
            Opcode::JumpIfFalse => {
                if targ[0] == 0 {
                    self.pc = targ[1].try_into()?;
                }
                Ok(false)
            }
            Opcode::LessThan => {
                self.write_mem(arg[2].try_into()?, if targ[0] < targ[1] { 1 } else { 0 });
                Ok(false)
            }
            Opcode::Equals => {
                self.write_mem(arg[2].try_into()?, if targ[0] == targ[1] { 1 } else { 0 });
                Ok(false)
            }
            Opcode::Halt => {
                self.state = IntComputerState::Halt;
                Ok(true)
            }
        }
    }

    pub fn run(&mut self) -> ResultT<&mut Self> {
        while !self.step()? {}
        Ok(self)
    }

    pub fn run_to_output_or_halt(&mut self) -> ResultT<&mut Self> {
        let outlen = self.output.len();
        while outlen >= self.output.len() {
            if self.step()? {
                break;
            }
        }

        Ok(self)
    }

    pub fn run_to_output(&mut self) -> ResultT<&mut Self> {
        let outlen = self.output.len();
        while outlen >= self.output.len() {
            if self.step()? {
                return Err("Program halted".into());
            }
        }

        Ok(self)
    }

    pub fn is_halted(&self) -> bool {
        self.state == IntComputerState::Halt
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode::IntComputer;

    #[test]
    fn day2_add_mul_halt() {
        assert_eq!(
            IntComputer::from_str("1,9,10,3,2,3,11,0,99,30,40,50")
                .unwrap()
                .run()
                .unwrap()
                .mem()
                .get(0),
            Some(&3500)
        );
    }

    #[test]
    fn day5_input_output() {
        assert_eq!(
            IntComputer::from_str("4,7,3,7,4,7,99,567")
                .unwrap()
                .trace()
                .input(&[1234])
                .run()
                .unwrap()
                .output(),
            &[567, 1234]
        );
    }

    #[test]
    fn day5_input_fail_underrun() {
        assert_eq!(
            IntComputer::from_str("3,0,99")
                .unwrap()
                .run()
                .unwrap_err()
                .to_string(),
            "Input underrun"
        );
    }

    #[test]
    fn day5_immediate_mode() {
        IntComputer::from_str("1002,4,3,4,33")
            .unwrap()
            .run()
            .unwrap();
    }

    #[test]
    fn day5_negative_words() {
        IntComputer::from_str("1101,100,-1,4,0")
            .unwrap()
            .run()
            .unwrap();
    }

    #[test]
    fn day5_equal() {
        const EQUAL_POS: &str = "3,9,8,9,10,9,4,9,99,-1,8";
        const EQUAL_IMMEDIATE: &str = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(
            IntComputer::from_str(EQUAL_POS)
                .unwrap()
                .input(&[8])
                .run()
                .unwrap()
                .output(),
            &[1]
        );
        assert_eq!(
            IntComputer::from_str(EQUAL_POS)
                .unwrap()
                .input(&[4])
                .run()
                .unwrap()
                .output(),
            &[0]
        );
        assert_eq!(
            IntComputer::from_str(EQUAL_IMMEDIATE)
                .unwrap()
                .input(&[8])
                .run()
                .unwrap()
                .output(),
            &[1]
        );
        assert_eq!(
            IntComputer::from_str(EQUAL_IMMEDIATE)
                .unwrap()
                .input(&[4])
                .run()
                .unwrap()
                .output(),
            &[0]
        );
    }

    #[test]
    fn day5_lessthan() {
        const LT_POS: &str = "3,9,7,9,10,9,4,9,99,-1,8";
        const LT_IMMEDIATE: &str = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(
            IntComputer::from_str(LT_POS)
                .unwrap()
                .input(&[8])
                .run()
                .unwrap()
                .output(),
            &[0]
        );
        assert_eq!(
            IntComputer::from_str(LT_POS)
                .unwrap()
                .input(&[4])
                .run()
                .unwrap()
                .output(),
            &[1]
        );
        assert_eq!(
            IntComputer::from_str(LT_IMMEDIATE)
                .unwrap()
                .input(&[8])
                .run()
                .unwrap()
                .output(),
            &[0]
        );
        assert_eq!(
            IntComputer::from_str(LT_IMMEDIATE)
                .unwrap()
                .input(&[4])
                .run()
                .unwrap()
                .output(),
            &[1]
        );
    }

    #[test]
    fn day5_jump() {
        const JUMP_POSITION: &str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        const JUMP_IMMEDIATE: &str = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(
            IntComputer::from_str(JUMP_POSITION)
                .unwrap()
                .input(&[0])
                .run()
                .unwrap()
                .output(),
            &[0]
        );
        assert_eq!(
            IntComputer::from_str(JUMP_POSITION)
                .unwrap()
                .input(&[123])
                .run()
                .unwrap()
                .output(),
            &[1]
        );
        assert_eq!(
            IntComputer::from_str(JUMP_IMMEDIATE)
                .unwrap()
                .input(&[0])
                .run()
                .unwrap()
                .output(),
            &[0]
        );
        assert_eq!(
            IntComputer::from_str(JUMP_IMMEDIATE)
                .unwrap()
                .input(&[123])
                .run()
                .unwrap()
                .output(),
            &[1]
        );
    }

    #[test]
    fn day5_large() {
        const PROGRAM: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(
            IntComputer::from_str(PROGRAM)
                .unwrap()
                .input(&[0])
                .run()
                .unwrap()
                .output(),
            &[999]
        );
        assert_eq!(
            IntComputer::from_str(PROGRAM)
                .unwrap()
                .input(&[8])
                .run()
                .unwrap()
                .output(),
            &[1000]
        );
        assert_eq!(
            IntComputer::from_str(PROGRAM)
                .unwrap()
                .input(&[9])
                .run()
                .unwrap()
                .output(),
            &[1001]
        );
    }
}
