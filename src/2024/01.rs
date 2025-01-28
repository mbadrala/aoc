// --- Day 1: Historian Hysteria ---
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./src/2024/01.txt") {
        for line in lines.map_while(Result::ok) {
            let numbers: Vec<&str> = line.split_whitespace().collect();
            let num1: i32 = numbers[0].parse().unwrap();
            let num2: i32 = numbers[1].parse().unwrap();

            a.push(num1);
            b.push(num2);
        }
        a.sort();
        b.sort();
    }

    do_part_one(&a, &b);
    do_part_two(&a, &b);
}

fn do_part_one(a: &[i32], b: &[i32]) {
    let result: i32 = a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    println!("Day 01 part one result: {}", result);
}

fn do_part_two(a: &[i32], b: &[i32]) {
    let mut result: i32 = 0;

    for i in a {
        for j in b {
            if i == j {
                result += i;
            }
        }
    }

    println!("Day 01 part two result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
