mod day01a;
mod day01b;
mod day02a;
mod day02b;
mod day03a;
mod day03b;
mod day04a;
mod day04b;
mod day05a;
mod day05b;
mod day06a;
mod day06b;
mod day07a;
mod day07b;
mod day08a;
mod day08b;
mod day09a;
mod day09b;
mod day10a;
mod day10b;
mod day11a;
mod day11b;
mod day12a;
mod day12b;
mod day13a;
mod day13b;

use aoc2023::read_file;
use lazy_static::lazy_static;

struct Job {
    name: &'static str,
    job: fn(&str) -> String,
    input: String
}

lazy_static! {
    static ref JOBS: Box<[Job]> = Box::new([
        Job { name: "Day 1a", job: crate::day01a::main, input: read_file!("./inputs/day01.txt") },
        Job { name: "Day 1b", job: crate::day01b::main, input: read_file!("./inputs/day01.txt") },
        Job { name: "Day 2a", job: crate::day02a::main, input: read_file!("./inputs/day02.txt") },
        Job { name: "Day 2b", job: crate::day02b::main, input: read_file!("./inputs/day02.txt") },
        Job { name: "Day 3a", job: crate::day03a::main, input: read_file!("./inputs/day03.txt") },
        Job { name: "Day 3b", job: crate::day03b::main, input: read_file!("./inputs/day03.txt") },
        Job { name: "Day 4a", job: crate::day04a::main, input: read_file!("./inputs/day04.txt") },
        Job { name: "Day 4b", job: crate::day04b::main, input: read_file!("./inputs/day04.txt") },
        Job { name: "Day 5a", job: crate::day05a::main, input: read_file!("./inputs/day05.txt") },
        // Job { name: "Day 5b", job: crate::day05b::main, input: read_file!("./inputs/day05.txt") },
        Job { name: "Day 6a", job: crate::day06a::main, input: read_file!("./inputs/day06.txt") },
        // Job { name: "Day 6b", job: crate::day06b::main, input: read_file!("./inputs/day06.txt") },
        Job { name: "Day 7a", job: crate::day07a::main, input: read_file!("./inputs/day07.txt") },
        Job { name: "Day 7b", job: crate::day07b::main, input: read_file!("./inputs/day07.txt") },
        Job { name: "Day 8a", job: crate::day08a::main, input: read_file!("./inputs/day08.txt") },
        Job { name: "Day 8b", job: crate::day08b::main, input: read_file!("./inputs/day08.txt") },
        Job { name: "Day 9a", job: crate::day09a::main, input: read_file!("./inputs/day09.txt") },
        Job { name: "Day 9b", job: crate::day09b::main, input: read_file!("./inputs/day09.txt") },
        Job { name: "Day 10a", job: crate::day10a::main, input: read_file!("./inputs/day10.txt") },
        Job { name: "Day 10b", job: crate::day10b::main, input: read_file!("./inputs/day10.txt") },
        // Job { name: "Day 11a", job: crate::day11a::main, input: read_file!("./inputs/day11.txt") },
        // Job { name: "Day 11b", job: crate::day11b::main, input: read_file!("./inputs/day11.txt") },
        Job { name: "Day 12a", job: crate::day12a::main, input: read_file!("./inputs/day12.txt") },
        // Job { name: "Day 12b", job: crate::day12b::main, input: read_file!("./inputs/day12.txt") },
        // Job { name: "Day 13a", job: crate::day13a::main, input: read_file!("./inputs/day13.txt") },
        // Job { name: "Day 13b", job: crate::day13b::main, input: read_file!("./inputs/day13.txt") },
    ]);
}

/*
...#......
.......#..
#.........
..........
..........
......#...
.#........
.........#
..........
..........
.......#..
#...#.....
*/

fn main() {
    for j in JOBS.iter() {
        println!("{}: ", j.name);
        println!("{}", (j.job)(&j.input));
    }
}
