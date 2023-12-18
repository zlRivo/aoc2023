use std::collections::HashSet;

type Position = (i64, i64);

fn get_offsets(grid: &HashSet<Position>, pos: Position) -> Vec<Position> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().filter_map(|(oy, ox)| {
        let y = pos.0 + *oy;
        let x = pos.1 + *ox;

        grid.contains(&(y, x)).then_some((*oy, *ox))
    }).collect::<Vec<Position>>()
}

pub(crate) fn main(input: &str) -> String {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut pos = [0; 2];

    // Build the map
    for l in input.lines() {
        let parts: Vec<&str> = l.split_whitespace().collect();

        let direction = parts[0];
        let meters = parts[1].parse::<usize>().unwrap();

        for _ in 0..meters {
            visited.insert((pos[0], pos[1]));

            match direction {
                "U" => { pos[0] -= 1; },
                "D" => { pos[0] += 1; },
                "L" => { pos[1] -= 1; },
                "R" => { pos[1] += 1; },
                _ => { panic!(); }
            }
        }
    }

    // Get the min and max positions
    let (mut y_min, mut x_min, mut y_max, mut x_max) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);

    for pos in visited.iter() {
        if pos.0 < y_min { y_min = pos.0; }
        if pos.1 < x_min { x_min = pos.1; }
        if pos.0 > y_max { y_max = pos.0; }
        if pos.1 > x_max { x_max = pos.1; }
    }

    // println!("{}, {} - {}, {}", x_min, y_min, x_max, y_max);

    // Raycast to find inside tiles
    let mut crossings = 0;
    let mut inside_tiles = 0;
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if visited.contains(&(y, x)) {
                let offsets = get_offsets(&visited, (y, x));
                if (offsets.contains(&(-1, 0)) && !offsets.contains(&(1, 0)) // Contains top side but not bottom
                    && (offsets.contains(&(0, -1)) && !offsets.contains(&(0, 1)) // Left side
                    || offsets.contains(&(0, 1)) && !offsets.contains(&(0, -1)))) // Right side
                    || !offsets.contains(&(0, -1)) && !offsets.contains(&(0, 1)) // Vertical crossing
                {
                    crossings += 1;
                }
            } else {
                if crossings % 2 == 1 {
                    inside_tiles += 1;
                }            
            }
        }
    }

    (visited.len() + inside_tiles).to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day18a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day18a_test.txt")), "62".to_string());
    }
}