mod intcode {
    pub fn intcode(program: &mut[u64]) {
        let mut program_index: usize = 0;

        loop {
            let opcode = program[program_index];
            match opcode {
                1 => {
                    println!("Running op1");
                    let n1 = program[program[program_index + 1 as usize] as usize];
                    let n2 = program[program[program_index + 2 as usize] as usize];
                    program[program[program_index + 3 as usize] as usize] = n1 + n2;
                    println!("program[{}] = {} + {}", program[program_index + 3], n1, n2);
                    program_index += 4;
                },
                2 => {
                    println!("Running op2");
                    let n1 = program[program[program_index + 1 as usize] as usize];
                    let n2 = program[program[program_index + 2 as usize] as usize];
                    program[program[program_index + 3 as usize] as usize] = n1 * n2;
                    println!("program[{}] = {} * {}", program[program_index + 3], n1, n2);
                    program_index += 4;
                },
                99 => {
                    println!("Opcode 99, finishing");
                    return;
                },
                x => {
                    let retstr = format!("Wrong opcode: {}", x);
                    panic!(retstr);
                }
            }
        }
    }
}

fn main() {
    let mut program = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,19,10,23,1,23,13,27,1,6,27,31,1,9,31,35,2,10,35,39,
                   1,39,6,43,1,6,43,47,2,13,47,51,1,51,6,55,2,6,55,59,2,59,6,63,2,63,13,67,1,5,67,71,2,9,71,75,1,
                   5,75,79,1,5,79,83,1,83,6,87,1,87,6,91,1,91,5,95,2,10,95,99,1,5,99,103,1,10,103,107,1,107,9,111,
                   2,111,10,115,1,115,9,119,1,13,119,123,1,123,9,127,1,5,127,131,2,13,131,135,1,9,135,139,1,2,139,
                   143,1,13,143,0,99,2,0,14,0];

    program[1] = 12;
    program[2] = 2;

    intcode::intcode(&mut program);

    println!("Result is {}", program[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode0() {
        let mut program = [1, 0, 0, 0, 99];
        intcode::intcode(&mut program);
        assert_eq!(program, [2, 0, 0, 0, 99]);
    }   

    #[test]
    fn test_intcode1() {
        let mut program = [2, 3, 0, 3, 99];
        intcode::intcode(&mut program);
        assert_eq!(program, [2, 3, 0, 6, 99]);
    }   

    #[test]
    fn test_intcode2() {
        let mut program = [2, 4, 4, 5, 99, 0];
        intcode::intcode(&mut program);
        assert_eq!(program, [2, 4, 4, 5, 99, 9801]);
    }   

    #[test]
    fn test_intcode3() {
        let mut program = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        intcode::intcode(&mut program);
        assert_eq!(program, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }   
}