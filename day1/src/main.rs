use std::io::{self, Read};

fn calculate_required_fuel(input: i64) -> i64 {
    let retval = input/3 - 2;
    if retval > 0 {
        retval + calculate_required_fuel(retval)
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let mut result = 0;

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let lines = buffer.lines();

    for line in lines {
        let mass = line.parse::<i64>().unwrap();
        result += calculate_required_fuel(mass);
    }

    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_required_fuel() {
        assert_eq!(calculate_required_fuel(14), 2);
        assert_eq!(calculate_required_fuel(1969), 966);
        assert_eq!(calculate_required_fuel(100756), 50346);
    }
}