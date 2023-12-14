fn tilt_vertical(grid: &mut Vec<Vec<char>>, backwards: bool) {
    for x in 0..grid[0].len() {
        let mut move_rock = false;
        let mut rock_pos = 0_i64;
        for y in 0..grid.len() {
            let new_y = if !backwards {
                y
            } else {
                grid.len() - 1 - y
            };

            match grid[new_y][x] {
                '.' => { 
                    if !move_rock {
                        move_rock = true;
                        rock_pos = new_y as i64;
                    }
                },
                'O' => {
                    if move_rock {
                        // Move rock
                        grid[new_y][x] = '.';
                        grid[rock_pos as usize][x] = 'O';
                        rock_pos += if !backwards { 1 } else { -1 };
                    }
                },
                '#' => { move_rock = false; }
                _ => { }
            }
        }
    }
}

fn tilt_horizontal(grid: &mut Vec<Vec<char>>, backwards: bool) {
    for y in 0..grid.len() {
        let mut move_rock = false;
        let mut rock_pos = 0_i64;
        for x in 0..grid[y].len() {
            let new_x = if !backwards {
                x
            } else {
                grid[y].len() - 1 - x
            };

            match grid[y][new_x] {
                '.' => { 
                    if !move_rock {
                        move_rock = true;
                        rock_pos = new_x as i64;
                    }
                },
                'O' => {
                    if move_rock {
                        // Move rock
                        grid[y][new_x] = '.';
                        grid[y][rock_pos as usize] = 'O';
                        rock_pos += if !backwards { 1 } else { -1 };
                    }
                },
                '#' => { move_rock = false; }
                _ => { }
            }
        }
    }
}

pub(crate) fn main(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut grid_history = vec![(0, grid.clone())];

    for i in 1..=1000 {

        tilt_vertical(&mut grid, false);
        tilt_horizontal(&mut grid, false);
        tilt_vertical(&mut grid, true);
        tilt_horizontal(&mut grid, true);

        let grid_match = grid_history.iter().find(|(_, g)| {
            for y in 0..g.len() {
                for x in 0..g[y].len() {
                    if g[y][x] != grid[y][x] {
                        return false;
                    }
                }
            }
            true
        });

        if let Some((idx, _)) = grid_match {
            let cycles_per_loop = i - idx;
            let offset = (1_000_000_000 - idx) % cycles_per_loop;
            grid = grid_history[idx + offset].1.clone();
            break;
        }

        grid_history.push((i, grid.clone()));
    }


    grid.iter()
        .rev()
        .zip(1..)
        .map(|(l, i)|
            l.iter().filter(|c| **c == 'O').count() * i
        )
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day14b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day14b_test.txt")), "64".to_string());
    }
}