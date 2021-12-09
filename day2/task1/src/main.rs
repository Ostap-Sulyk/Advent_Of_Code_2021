use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut file_context = String::new();
    file.read_to_string(&mut file_context)?;

    let test_string = String::from("down 8");
    let mut index_of_space = 0;
    for character in test_string.as_bytes() {
        if *character == b' ' {
            break;
        }
        index_of_space += 1;
    }
    let word = String::from(&test_string[..index_of_space]);
    let digit = &test_string[index_of_space..].trim();
    let digit = *(&digit.parse::<i32>().unwrap());

    println!("{}", word);
    println!("{}", digit);

    let mut operaion: Vec<(String, i32)> = Vec::new();
    let tuple = (String::from(word), digit);
    operaion.push(tuple);
    println!("{:#?}", operaion);

    // println!("{}", file_context);
    Ok(())
}
