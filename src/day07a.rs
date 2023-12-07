use std::collections::HashMap;

const HAND_SIZE: i64 = 5;

enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_card_strength(card: char) -> i64 {
    "23456789TJQKA".chars()
        .position(|c| card == c).unwrap_or(0)
        .try_into()
        .unwrap()
}

fn get_hand_type(hand: &str) -> HandType {
    let mut char_count: HashMap<char, i64> = HashMap::new();
    hand.chars().for_each(|c|
        *char_count.entry(c).or_insert(0) += 1);

    let mut three = false;
    let mut two_count = 0;
    let mut one_count = 0;

    for (_c, count) in char_count.iter() {
        match *count {
            5 => { return HandType::FiveOfAKind; }
            4 => { return HandType::FourOfAKind; }
            3 => { three = true; },
            2 => { two_count += 1; },
            _ => { one_count += 1; },
        }
    }

    if three && two_count == 1 {
        return HandType::FullHouse;
    } else if three && one_count == 2 {
        return HandType::ThreeOfAKind;
    } else if two_count == 2 {
        return HandType::TwoPair;
    } else if two_count == 1 {
        return HandType::OnePair;
    }

    HandType::HighCard
}

pub(crate) fn main(input: &str) -> String {
    let hands = input.lines().map(|l| {
        let (hand, bid) = l.split_once(' ').unwrap();
        (hand, bid.parse::<i64>().unwrap())
    }).collect::<Vec<(&str, i64)>>();

    println!("{:?}", hands);

    todo!()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day07a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day07a_test.txt")), "6440".to_string());
    }
}