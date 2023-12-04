pub(crate) fn main(input: &str) -> String {
    let mut cards_amount = input.lines()
        .map(|line| {
            let (title, card) = line.split_once(':').unwrap();
            let sides = card.split('|').map(|part| {
                part.trim().split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            }).collect::<Vec<Vec<i32>>>();
            
            let winning_count = sides[1].iter()
                .filter(|n| sides[0].contains(n))
                .enumerate()
                .count();

            (1_i32, winning_count as i32)
        })
        .collect::<Vec<(i32, i32)>>();

    for i in 0..cards_amount.len() {
        for j in 1..=cards_amount[i].1 {
            if i + j as usize > cards_amount.len() - 1 {
                continue;
            }
            cards_amount[i + j as usize].0 += cards_amount[i].0;
        }
    }

    cards_amount.iter().fold(0, |acc, (cards, _winning)| acc + *cards).to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day04b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day04b_test.txt")), "".to_string());
    }
}