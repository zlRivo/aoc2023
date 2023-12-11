use std::collections::HashSet;
use std::collections::VecDeque;

type Position = (usize, usize);

const NEIGHBOR_OFFSETS: &[(i64, i64)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

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
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut galaxies: HashSet<Position> = HashSet::new();

    // Horizonzal expansion and galaxies locations
    let mut y = 0;
    let mut found_galaxy = false;
    while y < map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                found_galaxy = true;
            }
        }

        if !found_galaxy {
            map.insert(y, map[y].clone());
            y += 1;
        }

        found_galaxy = false;
        y += 1;
    }

    // Vertical expansion
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
            for y in 0..map.len() {
                map[y].insert(x, '.');
            }
            x += 1;
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
            (!iterated_galaxies.contains(pos)).then(|| steps)
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
    use aoc2023::read_file;

    #[test]
    fn day11a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day11a_test.txt")), "374".to_string());
    }
}