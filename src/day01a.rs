pub(crate) fn main(input: &str) -> String {
    input.lines()
        .map(|line| {
            let first = line.chars()
                .find(|c| c.is_numeric())
                .unwrap();

            let last = line.chars().rev()
                .find(|c| c.is_numeric())
                .unwrap();

            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day01a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day01a_test.txt")), "142".to_string());
    }
}