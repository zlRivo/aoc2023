use std::collections::HashSet;

type Position = (i64, i64);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction { North = 0, South, East, West }

impl Direction {
    fn get_opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn mirror(&self, south_east: bool) -> Self {
        let mut result: Direction = ((*self as usize + 2) % 4).try_into().unwrap();
        if south_east { result = result.get_opposite(); }
        result
    }
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            value if value == Direction::North as usize => Ok(Direction::North),
            value if value == Direction::South as usize => Ok(Direction::South),
            value if value == Direction::East as usize => Ok(Direction::East),
            value if value == Direction::West as usize => Ok(Direction::West),
            _ => Err(())
        }
    }
}

fn count_energized(grid: &Vec<Vec<char>>, mut pos: Position, mut dir: Direction, visited: &mut HashSet<(Position, Direction)>) {
    loop {
        // Out of bounds
        if pos.0 < 0 || pos.1 < 0 || pos.0 as usize >= grid.len() || pos.1 as usize >= grid[0].len() { return; }

        // Already visited in the same direction
        if visited.contains(&(pos, dir)) { return; }

        visited.insert((pos, dir)); // Mark position as visited
        let c = grid[pos.0 as usize][pos.1 as usize];
        // println!("{:?}, c = {}", pos, c);

        match c {
            '\\' => { dir = dir.mirror(true) },
            '/' => { dir = dir.mirror(false) },
            '-' => {
                if dir == Direction::North || dir == Direction::South {
                    count_energized(grid, (pos.0, pos.1 - 1), Direction::West, visited);
                    count_energized(grid, (pos.0, pos.1 + 1), Direction::East, visited);
                    return;
                }
            },
            '|' => {
                if dir == Direction::West || dir == Direction::East {
                    count_energized(grid, (pos.0 - 1, pos.1), Direction::North, visited);
                    count_energized(grid, (pos.0 + 1, pos.1), Direction::South, visited);
                    return;
                }
            },
            _ => { },
        }

        match dir {
            Direction::North => { pos.0 -= 1; },
            Direction::South => { pos.0 += 1; },
            Direction::East => { pos.1 += 1; },
            Direction::West => { pos.1 -= 1; },
        }
    }
}

pub(crate) fn main(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();
    count_energized(&grid, (0, 0), Direction::East, &mut visited);

    let mut energized: HashSet<Position> = HashSet::new();

    for (pos, _) in visited.iter() {
        energized.insert(*pos);
    }

    energized.len().to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day16a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day16a_test.txt")), "46".to_string());
    }
}