use std::collections::{HashSet, HashMap};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

type Position = (usize, usize);

const MAX_SAME_MOVE: usize = 3;
const NEIGHBOR_OFFSETS: &[(i64, i64)] = &[
    (-1, 0), (1, 0), (-1, 0), (0, 1)
];


fn get_neighbors(grid: &Vec<Vec<char>>, pos: Position) -> Vec<(Position, (i64, i64))> {
    let mut neighbors = vec![];
    for (oy, ox) in NEIGHBOR_OFFSETS.iter() {
        let y = *oy + pos.0 as i64;
        let x = *ox + pos.1 as i64;

        // Out of bounds
        if y < 0 || x < 0 || y as usize >= grid.len() || x as usize >= grid[0].len() {
            continue;
        }

        neighbors.push(((y as usize, x as usize), (*oy, *ox)));
    }
    neighbors
}

fn get_weight(grid: &Vec<Vec<char>>, pos: Position) -> usize {
    grid[pos.0][pos.1].to_digit(10).unwrap() as usize
}

pub(crate) fn main(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    
    let mut shortest: HashMap<(Position, (i64, i64), usize), (usize, Position)> = HashMap::new();
    let mut min_heap: BinaryHeap<Reverse<(usize, Position, Position, (i64, i64), usize)>> = BinaryHeap::new();
    min_heap.push(Reverse((get_weight(&grid, (0, 0)), (0, 0), (0, 0), (0, 0), 0)));
    // Weight, position, previous, direction, moves

    let mut result = 0;

    while !min_heap.is_empty() {
        let (weight, pos, prev, dir, moves) = min_heap.pop().unwrap().0;

        if pos == (grid.len() - 1, grid[0].len() - 1) {
            result = moves;
            break;
        }

        if shortest.contains_key(&(pos, dir, moves)) { continue; }
        
        println!("{:?}", pos);
        shortest.insert((pos, dir, moves), (weight, prev));

        if moves < 3 && dir != (0, 0) {
            let y = pos.0 as i64 + dir.0;
            let x = pos.1 as i64 + dir.1;

            if !(y < 0 || x < 0 || y as usize >= grid.len() || x as usize >= grid[0].len()) {
                let new_pos = (y as usize, x as usize);
                let total_weight = weight + get_weight(&grid, new_pos);
                min_heap.push(Reverse((total_weight, new_pos, pos, dir, moves + 1)));
            }
        }

        for (n, (oy, ox)) in get_neighbors(&grid, pos) {
            if (oy, ox) == dir || (-oy, -ox) == dir {
                continue;
            }

            let total_weight = weight + get_weight(&grid, n);
            min_heap.push(Reverse((total_weight, n, pos, (oy, ox), 1)));
        }
    }

    // let mut p = (grid.len() - 1, grid[0].len() - 1);
    // while p != (0, 0) {
    //     grid[p.0][p.1] = 'X';
    //     p = shortest[&p].1;
    // }

    for l in &grid {
        for c in l {
            print!("{c}");
        }
        println!("");
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