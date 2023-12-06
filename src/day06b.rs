pub(crate) fn main(input: &str) -> String {
    let nums = input.lines().map(|l| 
        l.split_once(": ").unwrap().1
            .replace(" ", "")
            .parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let race_time = nums[0];
    let race_dist = nums[1];

    ((1..race_time)
        .filter(|press_time| press_time * (race_time - press_time) > race_dist)
        .count() as i64).to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day06b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day06b_test.txt")), "71503".to_string());
    }
}