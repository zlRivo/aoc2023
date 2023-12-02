fn get_digit(text: &str) -> Option<u32> {

    // Iterate from left to right then right to left
    // to find first and last numbers

    let digits_text: &[(&str, u32)] = &[
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ];

    text.chars().next().unwrap().to_digit(10).or_else(|| {
        digits_text.iter().find_map(|(t, n)| {
            if text.starts_with(t) {
                Some(*n)
            } else {
                None
            }
        })
    })
}

pub(crate) fn main(input: &str) -> String {

    input.lines()
        .map(|line| {

            // Iterate from left to right then right to left
            // to find first number appearance

            let first = (0..line.len()).find_map(|n| get_digit(&line[n..])).unwrap();
            let last = (0..line.len()).rev().find_map(|n| get_digit(&line[n..])).unwrap();

            first * 10 + last
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day01b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day01b_test.txt")), "281".to_string());
    }
}