use std::collections::HashMap;

pub(crate) fn main(input: &str) -> String {
    input.lines()
        .map(|line| {
            let game: Vec<&str> = line.split(":").collect();

            let mut max_cubes: HashMap<&str, u32> = HashMap::new();

            for set in game[1].split(";") {
                for cubes in set.split(",") {
                    let pick: Vec<&str> = cubes.trim().split_whitespace().collect();
                    // Amount of cubes must be under max cubes
                    let amount = pick[0].parse::<u32>().unwrap();

                    if !max_cubes.contains_key(pick[1]) {
                        max_cubes.insert(pick[1], amount);
                    } else {
                        if amount > max_cubes[pick[1]] {
                            *max_cubes.get_mut(pick[1]).unwrap() = amount;
                        }
                    }
                }
            }

            max_cubes.iter().fold(1, |a, (_, min)| a * (*min))
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day02b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day02b_test.txt")), "2286".to_string());
    }
}