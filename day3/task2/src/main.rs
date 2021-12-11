fn main() {
    // let mut gamma_rate = String::new();
    // let mut epsilon_rate = String::new();

    let mut vec_with_zeros = Vec::new();
    let mut vec_with_ones = Vec::new();
    // creating vector from test_input
    let data: Vec<&str> = include_str!("../test_input")
        .lines()
        .map(|line| line.trim())
        .collect();

    let mut vec_data: Vec<Vec<char>> = Vec::new();
    for i in 0..data.len() {
        vec_data.push(data[i].chars().collect());
    }

    for column in 0..1 {
        for row in 0..vec_data.len() {
            if vec_data[row][column] == '0' {
                vec_with_zeros.push(&vec_data[row]);
            } else {
                vec_with_ones.push(&vec_data[row]);
            }
        }
    }

    println!("0000000000000000000");
    println!("{:?}", vec_with_zeros.len());
    println!("11111111111111111");
    println!("{:?}", vec_with_ones.len());
    // println!("{:#?}", vec_data);
}
