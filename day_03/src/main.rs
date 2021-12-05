use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input_path = Path::new("input.txt");
    let reader = BufReader::new(File::open(&input_path).unwrap());
    let input = reader.lines().flatten().collect();
    let p1 = part_01(&input);
    println!("p1 {}", p1);
}

fn get_digit_from_string(s: &String, n: usize) -> usize {
    s.chars().nth(n).unwrap().to_digit(10).unwrap() as usize
}

fn calculate_diagnostic(input: &Vec<String>) -> Vec<f64> {
    input
        .iter()
        .fold(vec![0; 12], |acc, x| {
            acc.iter()
                .enumerate()
                .map(|(i, &e)| e + get_digit_from_string(&x, i))
                .collect()
        })
        .iter()
        .map(|&x| (x as f64 / input.len() as f64))
        .collect()
}

fn part_01(input: &Vec<String>) -> usize {
    calculate_diagnostic(input)
        .iter()
        .map(|x| x.round() as usize)
        .fold(vec![0, 0], |v, x| vec![v[0] * 2 + x, v[1] * 2 + (1 - x)])
        .iter()
        .fold(1, |v, x| v * x)
}

fn part_02(input: &Vec<String>) -> usize {
    let (gamma, epsilon) = calculate_diagnostic(input)
        .iter()
        .fold((0.0, 0.0), |v, x| (v.0 * 2.0 + x, v.1 * 2.0 + (1.0 - x)));
    0
}
