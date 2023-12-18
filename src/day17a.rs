use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Reverse;

const NEIGHBOR_OFFSETS: &[(i64, i64)] = &[
    (-1, 0), (1, 0), (0, -1), (0, 1)
];

fn get_weight(grid: &Vec<Vec<char>>, y: i64, x: i64) -> i64 {
    grid[y as usize][x as usize].to_digit(10).unwrap() as i64
}

pub(crate) fn main(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut shortest: HashSet<(i64, i64, i64, i64, i64)> = HashSet::new();
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse((get_weight(&grid, 0, 0), 0_i64, 0_i64, 0_i64, 0_i64, 0_i64)));

    let mut result = 0;

    while !min_heap.is_empty() {
        println!("{:?}", min_heap);
        let (weight, y, x, dy, dx, n) = min_heap.pop().unwrap().0;
        println!("({}, {}) direction ({}, {}) n = {} - weight = {}\n", y, x, dy, dx, n, weight);
        if (y, x) == (grid.len() as i64 - 1, grid[0].len() as i64 - 1) {
            result = weight;
            break;
        }

        if shortest.contains(&(y, x, dy, dx, n)) { continue; }
        shortest.insert((y, x, dy, dx, n));

        // Forward
        if n < 3 && (dy, dx) != (0, 0) {
            let ny = y + dy;
            let nx = y + dx;

            // Bound check
            if ny >= 0 && ny < grid.len() as i64 && nx >= 0 && nx < grid[0].len() as i64 {
                min_heap.push(Reverse((weight + get_weight(&grid, ny, nx), ny, nx, dy, dx, n + 1)));
            }
        }

        // Turning left and right
        for (ndy, ndx) in NEIGHBOR_OFFSETS {
            // Ignore forward (already treated) and backward
            if (*ndy, *ndx) == (dy, dx) || (*ndy, *ndx) == (-dy, -dx) { continue; }
            
            let ny = y + ndy;
            let nx = y + ndx;

            // Bound check
            if ny >= 0 && ny < grid.len() as i64 && nx >= 0 && nx < grid[0].len() as i64 {
                min_heap.push(Reverse((weight + get_weight(&grid, ny, nx), ny, nx, *ndy, *ndx, 1)));
            }
        }
    }

    println!("{}", result);

    todo!()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day17a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day17b_test.txt")), "102".to_string());
    }
}