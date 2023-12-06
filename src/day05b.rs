use rayon::prelude::*;

fn is_num_in_range(n: i64, start: i64, size: i64) -> bool {
    n >= start && n <= start + size - 1
}

pub(crate) fn main(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let seeds: Vec<i64> = lines[0].split_once(": ").unwrap().1
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
    let seed_chunks: Vec<&[i64]> = seeds.chunks_exact(2).collect();

    // Iterate in reverse this time.
    // Find the lowest location and check
    // if the corresponding seed exists

    let result_seed = (0..10_i64.pow(15)).into_par_iter()
        // Seed guess
        .find_map_any(|mut location_n| {
            let mut map_output = -1;
            let location_n_start = location_n;
            // Line iteration
            for i in (2..lines.len()).rev() {
                if lines[i].is_empty() || !lines[i].as_bytes()[0].is_ascii_digit() {
                    if map_output != -1 {
                        location_n = map_output;
                        // if _j == 1 {
                        //     println!("{}", location_n);
                        // }
                    }
                    map_output = -1;
                    continue;
                }

                let line_nums: Vec<i64> = lines[i].split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap()).collect();

                let range = line_nums[2];
                let dest_min = line_nums[0];
                let dest_max = dest_min + range - 1;
                let src_offset = line_nums[1] - line_nums[0];

                // if _j == 1 {
                //     println!("{} - {} -> {} - {} / n = {} / offset = {}", source_min, source_max, source_min + dest_offset, source_max + dest_offset, *seed_n, dest_offset);
                // }

                if location_n >= dest_min && location_n <= dest_max {
                    map_output = location_n + src_offset;
                }
            }

            seed_chunks.iter()
                .any(|chunk| is_num_in_range(location_n, chunk[0], chunk[1]))
                .then(|| Some(location_n_start))
        }).unwrap();

    return match result_seed {
        Some(n) => n.to_string(),
        None => (-1).to_string()
    };
    
    // (-1).to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day05b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day05b_test.txt")), "46".to_string());
    }
}