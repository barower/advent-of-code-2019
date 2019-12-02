#[macro_use] extern crate itertools;

mod intcode {
    pub struct Intcode<'a> {
        prog: &'a mut[u64],
        pc: usize,
    }

    impl Intcode<'_> {
        pub fn new(program: &mut[u64]) -> Intcode {
            let pr = Intcode {
                prog: program,
                pc: 0,
            };

            pr
        }

        pub fn run(&mut self) {
            loop {
                match self.get_opcode() {
                    1 => self.op1(),
                    2 => self.op2(),
                    99 => {
                        return;
                    },
                    x => {
                        let retstr = format!("Wrong opcode: {}", x);
                        panic!(retstr);
                    }
                }
            }
        }

        fn get_opcode(&self) -> u64 {
            self.prog[self.pc]
        }

        fn get_from_pc_offset(&self, offset: usize) -> u64 {
            self.prog[self.prog[self.pc + offset] as usize]
        }

        fn set_from_pc_offset(&mut self, offset: usize, val: u64) {
            self.prog[self.prog[self.pc + offset] as usize] = val;
        }

        fn op1(&mut self) {
            let n1 = self.get_from_pc_offset(1);
            let n2 = self.get_from_pc_offset(2);
            self.set_from_pc_offset(3, n1 + n2);
            self.pc += 4;
        }

        fn op2(&mut self) {
            let n1 = self.get_from_pc_offset(1);
            let n2 = self.get_from_pc_offset(2);
            self.set_from_pc_offset(3, n1 * n2);
            self.pc += 4;
        }
    }

}

const DESIRED_VALUE: u64 = 19690720;

fn main() {
    let program = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,19,10,23,1,23,13,27,1,6,27,31,1,9,31,35,2,10,35,39,
                   1,39,6,43,1,6,43,47,2,13,47,51,1,51,6,55,2,6,55,59,2,59,6,63,2,63,13,67,1,5,67,71,2,9,71,75,1,
                   5,75,79,1,5,79,83,1,83,6,87,1,87,6,91,1,91,5,95,2,10,95,99,1,5,99,103,1,10,103,107,1,107,9,111,
                   2,111,10,115,1,115,9,119,1,13,119,123,1,123,9,127,1,5,127,131,2,13,131,135,1,9,135,139,1,2,139,
                   143,1,13,143,0,99,2,0,14,0];


    for (noun, verb) in iproduct!(0..100, 0..100) {
        let mut test_program = vec![0; program.len()];
        test_program.clone_from_slice(&program);

        test_program[1] = noun;
        test_program[2] = verb;

        let mut intcode = intcode::Intcode::new(&mut test_program);
        intcode.run();

        if test_program[0] == DESIRED_VALUE {
            println!("{}", 100 * noun + verb);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode0() {
        let mut program = [1, 0, 0, 0, 99];
        let mut intcode = intcode::Intcode::new(&mut program);
        intcode.run();
        assert_eq!(program, [2, 0, 0, 0, 99]);
    }   

    #[test]
    fn test_intcode1() {
        let mut program = [2, 3, 0, 3, 99];
        let mut intcode = intcode::Intcode::new(&mut program);
        intcode.run();
        assert_eq!(program, [2, 3, 0, 6, 99]);
    }   

    #[test]
    fn test_intcode2() {
        let mut program = [2, 4, 4, 5, 99, 0];
        let mut intcode = intcode::Intcode::new(&mut program);
        intcode.run();
        assert_eq!(program, [2, 4, 4, 5, 99, 9801]);
    }   

    #[test]
    fn test_intcode3() {
        let mut program = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut intcode = intcode::Intcode::new(&mut program);
        intcode.run();
        assert_eq!(program, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }   
}