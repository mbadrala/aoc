// --- Day 1: Historian Hysteria ---
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = read_lines("./src/2024/01.txt")
        .expect("File read error")
        .map(|line| {
            let line = line.expect("Line read error");
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Parse error"))
                .collect();
            (nums[0], nums[1])
        })
        .unzip();

    a.sort_unstable();
    b.sort_unstable();

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
    let result: i32 = a
        .iter()
        .flat_map(|&i| b.iter().filter(move |&&j| i == j).map(|&x| x))
        .sum();
    println!("Day 01 part two result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
