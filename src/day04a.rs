use std::cmp::min;

pub(crate) fn main(input: &str) -> String {
    input.lines()
        .map(|line| {
            let (title, card) = line.split_once(':').unwrap();
            let (winning_nums_str, card_nums_str) = card.split_once('|').unwrap();
            let sides = card.split('|').map(|part| {
                part.trim().split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            }).collect::<Vec<Vec<i32>>>();
            
            sides[1].iter()
                .filter(|n| sides[0].contains(n))
                .zip(-1..)
                .map(|(n, i)| 2_i32.pow(min(0, i)))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day04a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day04a_test.txt")), "13".to_string());
    }
}