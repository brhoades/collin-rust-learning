#![allow(dead_code, unused)]
#![warn(clippy::all, clippy::pedantic)]

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    slice::SliceIndex,
    str::FromStr,
    time::Instant,
};

use clap::Parser;

mod custom;

mod prelude {
    pub use crate::custom::*;
}

use prelude::*;

#[derive(Parser)]
struct Args {
    #[arg(value_enum, short, default_value = "custom")]
    algorithm: Algorithm,

    #[arg(short, default_value = "input.txt")]
    filename: String,
}

#[derive(Clone, Copy, Debug)]
enum Algorithm {
    AStar,
    Custom,
}

impl FromStr for Algorithm {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "astar" => Ok(Self::AStar),
            "custom" => Ok(Self::Custom),
            _ => Err("welp".to_string()),
        }
    }
}

fn main() {
    let args = Args::parse();
    println!(
        "Filename: {}, Algorithm: {:?}",
        args.filename, args.algorithm
    );
    run(&args);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point(usize, usize); // row,col

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Point {
    fn adjust(&self, rows: i32, cols: i32) -> Option<Point> {
        if rows < 0 && self.0 == 0 {
            None
        } else if cols < 0 && self.1 == 0 {
            None
        } else {
            let new_rows = (self.0 as i32 + rows) as usize;
            let new_cols = (self.1 as i32 + cols) as usize;
            Some(Self(new_rows, new_cols))
        }
    }
    fn direction_to(&self, other: &Point) -> Direction {
        if other.0 > self.0 {
            Direction::Down
        } else if other.0 < self.0 {
            Direction::Up
        } else if other.1 > self.1 {
            Direction::Right
        } else {
            Direction::Left
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Map {
    tiles: Vec<Vec<char>>,
    start: Point,
    finish: Point,
    rows: usize,
    cols: usize,
}

impl Map {
    fn solve(&self) {
        println!("Solve:\n{self}\n");
        let start = Instant::now();
        let mut solver = Solver::new(self);
        let res = solver.solve();
        match res {
            None => eprintln!("No solution."),
            Some(path) => {
                let rendered = self.render_path(path);
                println!("Solution:\n{rendered}");
            }
        }
        let duration = start.elapsed();
        println!("Elapsed: {}.{}s", duration.as_secs(), duration.as_millis());
    }
    fn distance(&self, from: &Point, to: &Point) -> i32 {
        let from = self.get(from) as i32;
        let to = self.get(to) as i32;
        to - from
    }
    fn get(&self, p: &Point) -> char {
        *self.tiles.get(p.0).unwrap().get(p.1).unwrap()
    }
    fn next_moves_from(&self, cur: &Point) -> [Option<Point>; 4] {
        [
            cur.adjust(-1, 0),
            cur.adjust(1, 0),
            cur.adjust(0, -1),
            cur.adjust(0, 1),
        ]
        .map(|p| match p {
            Some(p) => {
                if p.0 > self.rows - 1 || p.1 > self.cols - 1 {
                    // out of bounds
                    None
                } else {
                    if self.can_move(cur, &p) {
                        Some(p)
                    } else {
                        // can't move
                        None
                    }
                }
            }
            // out of bounds
            _ => None,
        })
    }
    fn can_move(&self, from: &Point, to: &Point) -> bool {
        let from = self.get_char(from) as u8;
        let to = self.get_char(to) as u8;
        if to <= from {
            return true;
        }
        // to is > from. we must make sure the difference is only 1
        if to == from + 1 {
            return true;
        }
        false
    }
    fn get_char(&self, p: &Point) -> char {
        *self.tiles.get(p.0).unwrap().get(p.1).unwrap()
    }
    fn render_path(&self, path: Vec<Point>) -> String {
        let mut tiles = self.tiles.clone();
        tiles
            .iter_mut()
            .for_each(|r| r.iter_mut().for_each(|c| *c = '.'));
        println!("Path has {} entries", path.len() - 1);
        path.windows(2).for_each(|pair| {
            if let [p1, p2] = pair {
                let ch = match p1.direction_to(p2) {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                };
                *tiles.get_mut(p1.0).unwrap().get_mut(p1.1).unwrap() = ch;
            }
        });
        *tiles
            .get_mut(self.start.0)
            .unwrap()
            .get_mut(self.start.1)
            .unwrap() = 'S';
        *tiles
            .get_mut(self.finish.0)
            .unwrap()
            .get_mut(self.finish.1)
            .unwrap() = 'E';
        tiles
            .iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn render(&self) -> String {
        self.tiles
            .iter()
            .enumerate()
            .map(|(row, rows)| {
                rows.iter()
                    .enumerate()
                    .map(|(col, char)| {
                        let point = Point(row, col);
                        if self.start == point {
                            &'S'
                        } else if self.finish == point {
                            &'E'
                        } else {
                            char
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn new(tiles: Vec<Vec<char>>, start: Point, finish: Point) -> Self {
        let rows = tiles.len();
        let cols = tiles.get(0).map_or(0, |r| r.len());
        Self {
            tiles,
            start,
            finish,
            rows,
            cols,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

fn read_map(iter: impl Iterator<Item = String>) -> Map {
    let mut tiles = vec![];
    let mut start = Point(0, 0);
    let mut finish = Point(0, 0);
    for (row, line) in iter.enumerate() {
        let row: Vec<char> = line
            .chars()
            .enumerate()
            .map(|(col, c)| match c {
                'S' => {
                    start = Point(row, col);
                    'a'
                }
                'E' => {
                    finish = Point(row, col);
                    'z'
                }
                c => c,
            })
            .collect();
        tiles.push(row);
    }
    Map::new(tiles, start, finish)
}

fn run(args: &Args) {
    let file = File::open(args.filename.as_str()).unwrap();
    let read = BufReader::new(file);
    let lines = read.lines().flatten();
    let map = read_map(lines);
    map.solve();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_visited_cloning() {
        let mut visited = HashSet::new();
        let p1 = Point(1, 1);
        let p2 = Point(2, 2);
        let p3 = Point(3, 3);
        visited.insert(p1);
        assert!(visited.contains(&p1));
        {
            let mut visited = visited.clone();
            assert!(visited.contains(&p1));
            assert!(!visited.contains(&p2));
            assert!(!visited.contains(&p3));
            visited.insert(p2);
            assert!(visited.contains(&p1));
            assert!(visited.contains(&p2));
            assert!(!visited.contains(&p3));
        }
        assert!(visited.contains(&p1));
        assert!(!visited.contains(&p2));
        assert!(!visited.contains(&p3));
    }
}
