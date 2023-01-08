use num_traits::FromPrimitive;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
enum Opcode {
    Add = 1,
    Mul = 2,
    Halt = 99,
}
pub type Word = i64;

pub struct IntComputer {
    mem: Vec<Word>,
    pc: usize,
    trace: bool,
}

type ResultT<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl IntComputer {
    fn op_len(op: Opcode) -> usize {
        match op {
            Opcode::Add | Opcode::Mul => 4,
            Opcode::Halt => 1,
        }
    }

    pub fn from_str(inp: &str) -> ResultT<Self> {
        let mem: Vec<Word> = inp
            .trim()
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<_, ParseIntError>>()?;
        Ok(Self {
            mem,
            pc: 0,
            trace: false,
        })
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

    fn fetch_op(&self) -> ResultT<(usize, Opcode, Vec<Word>)> {
        if let Some(op) = Opcode::from_i64(self.mem[self.pc]) {
            let len = Self::op_len(op);
            Ok((len, op, self.mem[(self.pc + 1)..(self.pc + len)].to_vec()))
        } else {
            Err("Unknown opcode".into())
        }
    }

    pub fn step(&mut self) -> ResultT<bool> {
        let (oplen, op, arg) = self.fetch_op()?;

        if self.trace {
            println!("PC {} -> {:?}({}) {:?}", self.pc, op, oplen, arg);
        }

        self.pc += oplen;

        match op {
            Opcode::Add => {
                self.write_mem(
                    arg[2].try_into()?,
                    self.read_mem(arg[0].try_into()?) + self.read_mem(arg[1].try_into()?),
                );
                Ok(false)
            }
            Opcode::Mul => {
                self.write_mem(
                    arg[2].try_into()?,
                    self.read_mem(arg[0].try_into()?) * self.read_mem(arg[1].try_into()?),
                );
                Ok(false)
            }
            Opcode::Halt => Ok(true),
        }
    }

    pub fn run(&mut self) -> ResultT<&mut Self> {
        while !self.step()? {}
        Ok(self)
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
}
