mod intcode {
    pub fn intcode(program: &mut[i32]) {
        program[0] = 2;
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