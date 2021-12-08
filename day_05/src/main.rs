use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input_path = Path::new("input.txt");
    let reader = BufReader::new(File::open(&input_path).unwrap());
    let input = reader.lines().flatten().collect();
    let p1 = part_01(&input);
    println!("p1 {}", p1);
    let p2 = part_02(&input);
    println!("p2 {}", p2);
}

struct Line {
    pub a: (usize, usize),
    pub b: (usize, usize),
}

impl Line {
    fn is_angle(&self) -> bool {
        (self.a.0 != self.b.0) && (self.a.1 != self.b.1)
    }
    fn points(&self) -> Vec<(usize, usize)> {
        let diff_x = self.b.0 as isize - self.a.0 as isize;
        let diff_y = self.b.1 as isize - self.a.1 as isize;

        let size = diff_y.abs().max(diff_x.abs());
        let step_x = (diff_x as f64 / size as f64).round() as isize;
        let step_y = (diff_y as f64 / size as f64).round() as isize;

        let mut p = Vec::with_capacity(size as usize);
        for i in 0..size + 1 {
            p.push((
                (self.a.0 as isize + (step_x * i)) as usize,
                (self.a.1 as isize + (step_y * i)) as usize,
            ));
        }
        p
    }
}

impl From<&String> for Line {
    fn from(s: &String) -> Self {
        let v = s
            .split(" -> ")
            .map(|x| {
                let pair = x.split(",").collect::<Vec<&str>>();
                (pair[0].parse().unwrap(), pair[1].parse().unwrap())
            })
            .collect::<Vec<(usize, usize)>>();
        Line { a: v[0], b: v[1] }
    }
}

fn part_01(input: &Vec<String>) -> usize {
    let mut map = HashMap::<(usize, usize), usize>::new();
    input
        .iter()
        .map(|x| Line::from(x))
        .filter(|x| !x.is_angle())
        .map(|x| x.points())
        .flatten()
        .for_each(|v| {
            let e = map.get_mut(&v);
            if e.is_none() {
                map.insert(v, 1);
            } else {
                *e.unwrap() += 1;
            }
        });
    map.iter().filter(|x| x.1 >= &2).count()
}

fn part_02(input: &Vec<String>) -> usize {
    let mut map = HashMap::<(usize, usize), usize>::new();
    input
        .iter()
        .map(|x| Line::from(x).points())
        .flatten()
        .for_each(|v| {
            let e = map.get_mut(&v);
            if e.is_none() {
                map.insert(v, 1);
            } else {
                *e.unwrap() += 1;
            }
        });
    map.iter().filter(|x| x.1 >= &2).count()
}
