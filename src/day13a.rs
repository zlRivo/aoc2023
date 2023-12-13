pub(crate) fn main(input: &str) -> String {
    #[cfg(target_os = "macos")]
    let separator: &str = "\r";
    #[cfg(target_os = "linux")]
    let separator: &str = "\n";
    #[cfg(target_os = "windows")]
    let separator: &str = "\r\n";

    input.split(separator)
        .map(|note| {
            let grid: Vec<Vec<char>> = note.lines().map(|l| l.chars().collect()).collect();
            let mut total = 0;

            // Vertical comparison
            'outer: for y in 1..grid.len() {
                let mut mirror_line = true;
                for x in 0..grid[y].len() {
                    if grid[y][x] != grid[y - 1][x] {
                        mirror_line = false;
                    }
                }

                if mirror_line {
                    let mut i = 1;
                    while (y as i64 - i as i64 - 1) >= 0 && y + i < grid.len() {
                        for x in 0..grid[y].len() {
                            if grid[y + i][x] != grid[y - i - 1][x] {
                                continue 'outer;
                            }
                        }

                        i += 1;
                    }
                    total += y * 100;
                    break 'outer;
                }
            }

            // Horizontal comparison
            'outer: for x in 1..grid[0].len() {
                let mut mirror_col = true;
                for y in 0..grid.len() {
                    if grid[y][x] != grid[y][x - 1] {
                        mirror_col = false;
                    }
                }

                if mirror_col {
                    let mut i = 1;
                    while (x as i64 - i as i64 - 1) >= 0 && x + i < grid[0].len() {
                        for y in 0..grid.len() {
                            if grid[y][x + i] != grid[y][x - i - 1] {
                                continue 'outer;
                            }
                        }

                        i += 1;
                    }

                    total += x;
                    break 'outer;
                }
            }

            total
        }).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    #[ignore]
    fn day13a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day13a_test.txt")), "405".to_string());
    }
}