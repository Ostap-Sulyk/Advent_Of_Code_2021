fn main() {
    let data: Vec<&str> = include_str!("../input")
        .lines()
        .map(|line| line.trim())
        .collect();

    let mut vec_data_oxygen: Vec<Vec<char>> = Vec::new();
    let mut vec_data_co2: Vec<Vec<char>> = Vec::new();

    for i in 0..data.len() {
        vec_data_oxygen.push(data[i].chars().collect());
        vec_data_co2.push(data[i].chars().collect());
    }

    // looking  oxygen generator rating
    for column in 0..vec_data_oxygen[0].len() {
        let mut vec_most_common_0: Vec<Vec<char>> = Vec::new();
        let mut vec_most_common_1: Vec<Vec<char>> = Vec::new();
        let mut count0: usize = 0;
        let mut count1: usize = 0;

        for row in 0..vec_data_oxygen.len() {
            if vec_data_oxygen[row][column] == '0' {
                count0 += 1;
                vec_most_common_0.push(vec_data_oxygen[row].iter().map(|&x| x).collect());
            } else {
                count1 += 1;
                vec_most_common_1.push(vec_data_oxygen[row].iter().map(|&x| x).collect());
            }
        }
        if count0 > count1 {
            vec_data_oxygen = vec_most_common_0;
        } else {
            vec_data_oxygen = vec_most_common_1;
        }
    }

    // looking for CO2 scrubber rating
    'outer: for column in 0..vec_data_co2[0].len() {
        let mut vec_most_common_0: Vec<Vec<char>> = Vec::new();
        let mut vec_most_common_1: Vec<Vec<char>> = Vec::new();
        let mut count0: usize = 0;
        let mut count1: usize = 0;

        for row in 0..vec_data_co2.len() {
            if vec_data_co2.len() != 2 {
                if vec_data_co2[row][column] == '0' {
                    count0 += 1;
                    vec_most_common_0.push(vec_data_co2[row].iter().map(|&x| x).collect());
                } else {
                    count1 += 1;
                    vec_most_common_1.push(vec_data_co2[row].iter().map(|&x| x).collect());
                }
            } else {
                if vec_data_co2[row][column] == '0' {
                    vec_most_common_0.push(vec_data_co2[row].iter().map(|&x| x).collect());
                    vec_data_co2 = vec_most_common_0;
                } else {
                    vec_most_common_1.push(vec_data_co2[row + 1].iter().map(|&x| x).collect());
                    vec_data_co2 = vec_most_common_1;
                }
                break 'outer;
            }
        }
        if count0 <= count1 {
            vec_data_co2 = vec_most_common_0;
        } else {
            vec_data_co2 = vec_most_common_1;
        }
    }

    println!("CO2: {:?}", vec_data_co2);
    println!("Oxygen: {:?}", vec_data_oxygen);
    let oxygen_rating: String = vec_data_oxygen[0].iter().collect();
    let co2_rating: String = vec_data_co2[0].iter().collect();
    let life_support_rating = isize::from_str_radix(oxygen_rating.as_str(), 2).unwrap()
        * isize::from_str_radix(co2_rating.as_str(), 2).unwrap();

    println!("{}", life_support_rating);
}
