use std::ops::Range;

fn tilt(grid: &mut Vec<Vec<char>>, iter1: Range<usize>, iter2: Range<usize>)
{
    for i in iter1 {
        let mut move_rock = false;
        let mut rock_pos = 0;
        for j in iter2.clone() {
            match grid[j][i] {
                '.' => { 
                    if !move_rock {
                        move_rock = true;
                        rock_pos = j;
                    }
                },
                'O' => {
                    if move_rock {
                        // Move rock
                        grid[j][i] = '.';
                        grid[rock_pos][i] = 'O';
                        rock_pos += 1;
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

    let width = grid[0].len();
    let height = grid.len();

    tilt(&mut grid, 0..width, 0..height);
    tilt(&mut grid, 0..height, (0..width).rev());

    for l in grid.iter() {
        for c in l {
            print!("{}", c);
        }
        println!("");
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
        assert_eq!(super::main(&read_file!("./inputs/day14b_test.txt")), "".to_string());
    }
}