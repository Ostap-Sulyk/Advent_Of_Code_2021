use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() {
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    let data: Vec<&str> = include_str!("../input")
        .lines()
        .map(|line| line.trim())
        .collect();

    let mut vec_data: Vec<Vec<char>> = Vec::new();
    for i in 0..data.len() {
        vec_data.push(data[i].chars().collect());
    }

    for column in 0..vec_data[0].len() {
        let mut count0: usize = 0;
        let mut count1: usize = 0;
        for row in 0..vec_data.len() {
            if vec_data[row][column] == '0' {
                count0 += 1;
            } else {
                count1 += 1;
            }
        }
        if count0 > count1 {
            gamma_rate.push_str("0");
        } else {
            gamma_rate.push_str("1");
        }
    }

    for bit in gamma_rate.chars() {
        if bit == '0' {
            epsilon_rate.push_str("1")
        } else {
            epsilon_rate.push_str("0")
        }
    }
    let gamma_rate = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();

    println!("The power consumption is: {}", gamma_rate * epsilon_rate);
}
