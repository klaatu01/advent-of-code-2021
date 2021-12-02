#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input_path = Path::new("input.txt");
    let reader = BufReader::new(File::open(&input_path).unwrap());
    let input = reader
        .lines()
        .map(|s| s.unwrap().parse::<usize>())
        .flatten()
        .collect();
    let p1 = part_01(&input);
    println!("p1 {}", p1);
    let p2 = part_02(&input);
    println!("p2 {}", p2);
}

fn part_01(input: &Vec<usize>) -> usize {
    input.windows(2).filter(|&w| w[1] > w[0]).count()
}

fn part_02(input: &Vec<usize>) -> usize {
    let summed = input
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<usize>>();
    part_01(&summed)
}
