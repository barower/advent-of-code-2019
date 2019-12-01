use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut result = 0;

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let lines = buffer.lines();

    for line in lines {
        let mass = line.parse::<i64>().unwrap();
        result += mass/3 - 2; // Rust automatically rounds down on integer division
    }

    println!("{}", result);

    Ok(())
}