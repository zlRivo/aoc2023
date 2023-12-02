use std::collections::HashMap;

pub(crate) fn main(input: &str) -> String {
    input.lines()
        .map(|line| {
            let mut max_cubes: HashMap<&str, u32> = HashMap::new();

            for set in line.split(":").last().unwrap().split(";") {
                for cubes in set.split(",") {
                    let pick: Vec<&str> = cubes.trim().split_whitespace().collect();
                    // Amount of cubes must be under max cubes
                    let amount = pick[0].parse::<u32>().unwrap();

                    let max_amount = max_cubes.entry(pick[1]).or_insert(amount);
                    if amount > *max_amount {
                        *max_amount = amount;
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