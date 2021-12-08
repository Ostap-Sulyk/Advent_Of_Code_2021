use std::fs::File;
use std::io::Read;
use std::io::Result;

// i know there could be more optimized way of solving it
// but i am really new to rust
fn main() -> Result<()> {
    let mut file = File::open("data")?; // open file
    let mut context = String::new();
    let mut data: Vec<usize> = Vec::new(); // create variable to hold every number in

    file.read_to_string(&mut context)?; // read data from the file and store it as a string;

    for number in context.lines() {
        // convert string to vector of numbers
        data.push(number.parse().unwrap());
    }

    let mut increase_count = 0; // will hold number of times depth increased
    let mut next_number;
    let mut current_number;

    let mut index = 0; // while iterating over array
    let num_of_elements = data.len(); // to avoid calling data.len() multiple times, i assigned its value to number_of_elements

    while index < num_of_elements - 1 {
        // iterating through vector with our data
        current_number = data[index]; // assigning current_number
        next_number = data[index + 1]; // assigning next_number
        if current_number < next_number {
            increase_count += 1;
        }
        index += 1;
    }
    println!("{}", increase_count);

    Ok(())
}
