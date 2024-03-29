pub(crate) fn main(input: &str) -> String {
    input.lines()
        .map(|line| {
            let (_title, card) = line.split_once(':').unwrap();
            let sides = card.split('|').map(|part| {
                part.trim().split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            }).collect::<Vec<Vec<i32>>>();
            
            sides[1].iter()
                .filter(|n| sides[0].contains(n))
                .enumerate()
                .map(|(i, _n)| {
                    if i > 0 {
                        2_i32.pow(i as u32 - 1)
                    } else {
                        1
                    }
                })
                .sum::<i32>()
        })
        .sum::<i32>()
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