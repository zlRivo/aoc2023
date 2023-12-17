pub(crate) fn main(input: &str) -> String {
    input.split(',').map(|group| group.chars().fold(0, |a, c| (a + c as usize) * 17 % 256)).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day15a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day15a_test.txt")), "1320".to_string());
    }
}