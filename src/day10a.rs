type Position = (i64, i64);

const NEIGHBOR_OFFSETS: &[Position] = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

const PIPE_OFFSETS: &[(char, &[Position])] = &[
    ('|', &[(-1, 0), (1, 0)]),
    ('-', &[(0, -1), (0, 1)]),
    ('L', &[(-1, 0), (0, 1)]),
    ('J', &[(0, -1), (-1, 0)]),
    ('7', &[(0, -1), (1, 0)]),
    ('F', &[(1, 0), (0, 1)]),
];

fn get_first_pipe(map: &Vec<Vec<char>>, pos: Position) -> Position {
    let height = map.len() as i64;
    let width = map[0].len() as i64;

    for (oy, ox) in NEIGHBOR_OFFSETS {
        let y = pos.0 + *oy;
        let x = pos.1 + *ox;

        if y < 0 || y >= height || x < 0 || x >= width {
            continue;
        }

        let off_c = map[y as usize][x as usize];
        if let Some((_, valid_offsets)) = PIPE_OFFSETS.iter().find(|p| p.0 == off_c) {
            let off_p = (y - pos.0, x - pos.1);
            if valid_offsets.contains(&off_p) {
                return (y, x);
            }
        }
    }
    return (0, 0)
}

fn get_next_pipe(map: &Vec<Vec<char>>, pos: &mut Position, previous_pos: &mut Position) {
    let pos_c = map[pos.0 as usize][pos.1 as usize];
    // println!("{}", pos_c);
    let (_, offsets) = *PIPE_OFFSETS.iter().find(|p| p.0 == pos_c).unwrap();

    // println!("{:?}", offsets);

    for (oy, ox) in offsets {
        let y = pos.0 + *oy;
        let x = pos.1 + *ox;

        let new_pos = (y, x);
        if new_pos == *previous_pos {
            continue;
        }

        *previous_pos = *pos;
        *pos = new_pos;
        return;
    }
}

pub(crate) fn main(input: &str) -> String {
    let mut position = (0, 0);
    let map: Vec<Vec<char>> = input.lines()
        .enumerate()
        .map(|(y, l)| {
            let chars: Vec<char> = l.chars().collect();
            if let Some(x) = chars.iter().position(|c| *c == 'S') {
                position = (y as i64, x as i64);
            }
            chars
        }).collect();

    
    let mut steps = 1;
    let start_pos = position;
    let mut previous_pos = position;
    position = get_first_pipe(&map, position);
    // println!("prev pos = {:?}, pos = {:?}", previous_pos, position);

    while position != start_pos {
        // println!("{:?}", position);
        get_next_pipe(&map, &mut position, &mut previous_pos);
        // println!("{:?}, previous pos: {:?}", position, previous_pos);
        steps += 1;
    }
    
    (steps / 2).to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day10a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day10a_test.txt")), "4".to_string());
    }
}