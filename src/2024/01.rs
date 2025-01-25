// --- Day 1: Historian Hysteria ---
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    let mut len: usize = 0;

    if let Ok(lines) = read_lines("./src/2024/01.txt") {
        for line in lines.map_while(Result::ok) {
            let numbers: Vec<&str> = line.split_whitespace().collect();
            let num1: i32 = numbers[0].parse().unwrap();
            let num2: i32 = numbers[1].parse().unwrap();

            a.push(num1);
            b.push(num2);

            len += 1;
        }
        a.sort();
        b.sort();
    }

    do_part_one(&a, &b, len);
    do_part_two(a, b, len);
}

fn do_part_one(a: &Vec<i32>, b: &Vec<i32>, len: usize) {
    let mut result: i32 = 0;

    for i in 0..len {
        result += (a[i] - b[i]).abs();
    }

    println!("Day 01 part one result: {}", result);
}

fn do_part_two(a: Vec<i32>, b: Vec<i32>, len: usize) {
    let mut result: i32 = 0;

    for i in 0..len {
        for j in 0..len {
            if a[i] == b[j] {
                result += a[i];
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
