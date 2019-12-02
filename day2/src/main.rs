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