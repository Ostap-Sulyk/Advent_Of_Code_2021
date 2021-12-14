#[allow(dead_code, unused_mut)]
fn main() {
    let raw_data: Vec<&str> = include_str!("../test_input").split("\n").collect();
    // println!("{:}", raw_data);

    let _random_numbers: Vec<i32> = raw_data[0]
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut cells = Vec::new();

    for i in 1..raw_data.len() {
        if raw_data[i] == "" {
            continue;
        }
        let line = raw_data[i]
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("error"))
            .collect::<Vec<i32>>();

        for cell in line {
            cells.push(cell);
        }
    }

    let mut boards = Vec::new();

    let mut cell_index = 0;

    for _i in 0..cells.len() / 5 {
        let mut board = Vec::new();
        {
            let mut line = [(0, false); 5];
            for i in 0..5 {
                line[i] = (cells[cell_index], false);
                cell_index += 1;
            }
            board.push(line);
        }
        boards.push(board);
    }

    // iterate over
    // iterating over each boards row and column
    // and finding index from random numbers at witch all rows or columns are filled

    let mut when_bingo_happened: usize; // will hold the biggest index from random_numbers
    let mut current_last_index_per_board: usize;
    let mut current_last_index_per_row = -1;
    let mut current_last_index_per_column = -1;
    let mut current_last_index_per_cell = -1;
    let mut index_of_winning_board: usize;

    let mut couner = 0;
    for board in boards.iter() {
        for row in board {
            for cell in row {
                for (index, number) in _random_numbers.iter().enumerate() {
                    if cell.0 == *number {
                        if current_last_index_per_cell < index as i32 {
                            current_last_index_per_cell = index as i32;
                        }
                    }
                }
            }
        }
        if current_last_index_per_row < current_last_index_per_cell {
            current_last_index_per_row = current_last_index_per_cell
        }
        if couner == 4 {
            break;
        } else {
            couner += 1;
        }
    }

    println!(
        "Current last index per first row in first table is {}",
        current_last_index_per_row
    );

    let mut couner = 0;
    for board in boards.iter() {
        for row in board {
            for (index, boolean) in row {
                println!(
                    "{}",
                    _random_numbers.iter().position(|&x| x == *index).unwrap()
                )
            }
        }
        println!();
        if couner == 4 {
            break;
        } else {
            couner += 1;
        }
    }
}
