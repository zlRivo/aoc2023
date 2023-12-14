pub(crate) fn main(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    for x in 0..grid[0].len() {
        let mut move_rock = false;
        let mut rock_pos = usize::MAX;
        for y in 0..grid.len() {
            match grid[y][x] {
                '.' => { 
                    if !move_rock {
                        move_rock = true;
                        rock_pos = y;
                    }
                },
                'O' => {
                    if move_rock {
                        // Move rock
                        grid[y][x] = '.';
                        grid[rock_pos][x] = 'O';
                        rock_pos += 1;
                    }
                },
                '#' => { move_rock = false; }
                _ => { }
            }
        }
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
    fn day14a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day14a_test.txt")), "136".to_string());
    }
}