use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part_one("example.txt");
}

fn part_one(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let mut iter = read.lines().flatten();
    loop {
        let one = iter.next().unwrap();
        let two = iter.next().unwrap();
        println!("{}\n{}", one.trim(), two.trim());
        if iter.next().is_none() {
            break;
        }
        println!();
    }
}
