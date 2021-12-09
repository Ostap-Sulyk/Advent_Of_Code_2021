use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let mut file = File::open("input")?;
    let mut context = String::new();
    file.read_to_string(&mut context)?;

    let mut data = Vec::new();
    for line in context.lines() {
        let line = String::from(line.trim());
        let mut bits = Vec::new();
        for bit in line.chars() {
            bits.push(bit);
        }

        data.push(bits);
    }

    let mut gamma_rate =0;
    for line in 0..data.len() {
    }

    


    Ok(())
}
