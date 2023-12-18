use std::collections::HashSet;
use std::collections::HashMap;

type Position = (i64, i64);

fn get_offsets(grid: &HashSet<Position>, pos: Position) -> Vec<Position> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().filter_map(|(oy, ox)| {
        let y = pos.0 + *oy;
        let x = pos.1 + *ox;

        grid.contains(&(y, x)).then_some((*oy, *ox))
    }).collect::<Vec<Position>>()
}

pub(crate) fn main(input: &str) -> String {
    let mut directions: HashMap<&str, (i64, i64)> = HashMap::new();
    directions.insert("U", (-1, 0));
    directions.insert("D", (1, 0));
    directions.insert("L", (0, -1));
    directions.insert("R", (0, 1));

    let mut boundary_count = 0;
    let mut boundaries: Vec<Position> = vec![(0, 0)];
    let mut pos = [0; 2];

    // Get boundaries
    for l in input.lines() {
        let parts: Vec<&str> = l.split_whitespace().collect();

        let dir = match parts[2].as_bytes()[parts[2].len() - 2] {
            b'0' => { "R" },
            b'1' => { "D" },
            b'2' => { "L" },
            b'3' => { "U" },
            _ => { panic!() },
        };

        let meters = usize::from_str_radix(&parts[2][2..parts[2].len() - 2], 16).unwrap();
        boundary_count += meters as i64;

        pos = [pos[0] + directions[dir].0 * meters as i64, pos[1] + directions[dir].1 * meters as i64];
        boundaries.push((pos[0], pos[1]));
    }

    let mut area = 0;
    let len = boundaries.len();
    for i in 0..len {
        area += boundaries[i].1 *
            (boundaries[if i == 0 { len - 1 } else { i - 1 }].0
                - boundaries[(i + 1) % len].0)
    }
    area = area.abs() / 2;

    (area - boundary_count / 2 + 1 + boundary_count).to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day18b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day18b_test.txt")), "952408144115".to_string());
    }
}