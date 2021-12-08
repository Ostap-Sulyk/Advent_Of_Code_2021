use std::fs::File;
use std::io::Read;
use std::io::Result;

fn main() -> Result<()> {
    let mut file = File::open("data")?; // open file with data
    let mut context = String::new(); // store read data from file
    let mut data: Vec<usize> = Vec::new(); // store read data in form of vector

    file.read_to_string(&mut context)?; // reading all data from file and saving to context

    for number in context.lines() {
        // converting each line in context to number and pushing it to data
        data.push(number.parse().unwrap());
    }

    // Stop when there aren't enough measurements left to create a new three-measurement sum.
    // this means we have to find last index in our data array fully divisible by 3
    let mut index_of_last_valid_window = data.len();
    for number in (0..data.len()).rev() {
        if number % 3 == 0 {
            index_of_last_valid_window = number;
            break;
        }
    }

    let mut index = 0; // creating index for iteration
    let mut data_summed: Vec<usize> = Vec::new(); // vector to store sum of every three-measurement sliding window

    while index < index_of_last_valid_window {
        data_summed.push(data[index] + data[index + 1] + data[index + 2]);
        index += 1;
    }
    let mut next_number;
    let mut current_number;
    let mut increase_count = 0; // will hold number of times depth increased

    index = 0;
    while index < data_summed.len() - 1 {
        // iterating through vector with our data
        current_number = data_summed[index]; // assigning current_number
        next_number = data_summed[index + 1]; // assigning next_number
        if current_number < next_number {
            increase_count += 1;
        }
        index += 1;
    }

    println!("{:?}", data_summed.len());
    println!("{}", increase_count);

    Ok(())
}
