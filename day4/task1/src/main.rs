use std::cell;

#[allow(dead_code)]
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

    for i in 0..cells.len() / 5 {
        let mut board = Vec::new();
        {
            let mut line = [0; 5];
            for i in 0..5 {
                line[i] = cells[cell_index];
                cell_index += 1;
            }
            board.push(line);
        }
        boards.push(board);
    }
}
