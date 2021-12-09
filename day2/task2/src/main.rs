use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut file_context = String::new();
    file.read_to_string(&mut file_context)?;

    let mut operations: Vec<(String, i32)> = Vec::new();
    for line in file_context.lines() {
        let mut index_of_space = 0;
        for character in line.as_bytes() {
            if *character == b' ' {
                break;
            }
            index_of_space += 1;
        }

        let word = String::from(&line[..index_of_space]); // extracting word from file
        let number = (&line[index_of_space..].trim()).parse::<i32>().unwrap(); // extracting number from file

        let tuple = (String::from(word), number);

        operations.push(tuple);
    }

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for operation in operations {
        if operation.0 == "down" {
            aim += operation.1;
        } else if operation.0 == "up" {
            aim -= operation.1;
        } else {
            horizontal_position += operation.1;
            depth += aim * operation.1;
        }
    }
    println!("{}", horizontal_position * depth);
    Ok(())
}
