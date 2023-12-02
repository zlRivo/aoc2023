mod day01a;
mod day01b;
mod day02a;
mod day02b;

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
    ]);
}

fn main() {
    for j in JOBS.iter() {
        println!("{}: ", j.name);
        println!("{}", (j.job)(&j.input));
    }
}
