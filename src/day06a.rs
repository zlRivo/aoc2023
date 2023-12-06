pub(crate) fn main(input: &str) -> String {
    let nums = input.lines().map(|l| 
        l.split_once(": ").unwrap().1.trim()
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();

    (0..nums[0].len()).map(|race_i: usize| {
        let race_time = nums[0][race_i];
        let race_dist = nums[1][race_i];

        (1..race_time)
            .filter(|press_time| press_time * (race_time - press_time) > race_dist)
            .count() as i64
    })
    .product::<i64>()
    .to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day06a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day06a_test.txt")), "288".to_string());
    }
}