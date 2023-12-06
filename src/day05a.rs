pub(crate) fn main(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut seeds: Vec<i64> = lines[0].split_once(": ").unwrap().1
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let mut map_output = -1;

    for (_j,seed_n) in seeds.iter_mut().enumerate() {
        for i in 3..lines.len() {
            if lines[i].is_empty() || !lines[i].as_bytes()[0].is_ascii_digit() {
                if map_output != -1 {
                    *seed_n = map_output;
                }
                map_output = -1;
                continue;
            }

            let line_nums: Vec<i64> = lines[i].split_whitespace()
                .map(|n| n.parse::<i64>().unwrap()).collect();

            let range = line_nums[2];
            let source_min = line_nums[1];
            let source_max = source_min + range - 1;
            let dest_offset = line_nums[0] - line_nums[1];

            // if _j == 1 {
            //     println!("{} - {} -> {} - {} / n = {} / offset = {}", source_min, source_max, source_min + dest_offset, source_max + dest_offset, *seed_n, dest_offset);
            // }

            if *seed_n >= source_min && *seed_n <= source_max {
                map_output = *seed_n + dest_offset;
            }
        }

        if map_output != -1 {
            *seed_n = map_output;
            map_output = -1;
        }
    }

    seeds.iter().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day05a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day05a_test.txt")), "35".to_string());
    }
}