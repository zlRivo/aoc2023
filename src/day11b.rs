use std::collections::HashSet;
use std::collections::VecDeque;

type Position = (usize, usize);

const NEIGHBOR_OFFSETS: &[(i64, i64)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

const EXPANSION_MULTIPLIER: usize = 1_000_000;
// const EXPANSION_MULTIPLIER: usize = 100;

fn get_neighbors(map: &Vec<Vec<char>>, pos: Position) -> Vec<Position> {
    let mut neighbors = vec![];

    for (oy, ox) in NEIGHBOR_OFFSETS.iter() {
        let y = pos.0 as i64 + *oy;
        let x = pos.1 as i64 + *ox;

        if x < 0 || x >= map[0].len() as i64 || y < 0 || y >= map.len() as i64 {
            continue;
        }

        neighbors.push((y as usize, x as usize))
    }

    neighbors
}

pub(crate) fn main(input: &str) -> String {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut galaxies: HashSet<Position> = HashSet::new();
    let mut h_expansions: HashSet<usize> = HashSet::new();
    let mut v_expansions: HashSet<usize> = HashSet::new();

    // Vertical expansion and galaxies locations
    let mut y = 0;
    let mut found_galaxy = false;
    while y < map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                found_galaxy = true;
            }
        }

        if !found_galaxy {
            v_expansions.insert(y);
        }

        found_galaxy = false;
        y += 1;
    }

    // Horizontal expansion
    let mut x = 0;
    let mut found_galaxy = false;
    while x < map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == '#' {
                galaxies.insert((y, x));
                found_galaxy = true;
            }
        }

        if !found_galaxy {
            h_expansions.insert(x);
        }

        found_galaxy = false;
        x += 1;
    }

    // Store iterated galaxies to prevent doubles
    let mut iterated_galaxies: HashSet<Position> = HashSet::new();
    let mut total_sum = 0;

    for (start_y, start_x) in galaxies.iter() {
        // Find galaxies using breadth first search
        // let mut found_galaxy = false; // If we found a galaxy (we should stop iterating at this level)
        let mut found_galaxies: Vec<(usize, Position)> = vec![];
        let mut visited: HashSet<Position> = HashSet::new();
        let mut to_visit: VecDeque<(usize, Position)> = VecDeque::new();

        to_visit.push_back((0, (*start_y, *start_x)));
        while !to_visit.is_empty() {
            let (steps, pos) = to_visit.pop_front().unwrap();

            // Check if already visited
            if visited.contains(&pos) {
                continue;
            }

            if map[pos.0][pos.1] == '#' {
                found_galaxies.push((steps, pos))
            }

            for nei in get_neighbors(&map, pos) {
                to_visit.push_back((steps + 1, nei));
            }

            visited.insert(pos);
        }

        total_sum += found_galaxies.iter().filter_map(|(steps, pos)| {
            (!iterated_galaxies.contains(pos)).then(|| {
                let mut expansions_traversals = 0;

                let min = std::cmp::min(*start_y, pos.0);
                let max = std::cmp::max(*start_y, pos.0);

                for y in min..max {
                    if v_expansions.contains(&y) {
                        expansions_traversals += 1;
                    }
                }

                let min = std::cmp::min(*start_x, pos.1);
                let max = std::cmp::max(*start_x, pos.1);

                for x in min..max {
                    if h_expansions.contains(&x) {
                        expansions_traversals += 1;
                    }
                }

                (*steps - expansions_traversals) + EXPANSION_MULTIPLIER * expansions_traversals
            })
        }).sum::<usize>();

        iterated_galaxies.insert((*start_y, *start_x));
    }

    // for l in map {
    //     let s: String = l.iter().collect();
    //     println!("{}", s);
    // }

    total_sum.to_string()
}

#[cfg(test)]
mod tests {
    // use aoc2023::read_file;

    #[test]
    fn day11b_test() {
        // assert_eq!(super::main(&read_file!("./inputs/day11b_test.txt")), "".to_string());
    }
}