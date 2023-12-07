use std::collections::HashMap;
use std::cmp::Ordering;

const HAND_SIZE: i64 = 5;

#[derive(PartialEq, Debug)]
struct Hand(String, i64);

enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        let mut char_count: HashMap<char, i64> = HashMap::new();
        self.0.chars().for_each(|c|
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
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.len() != HAND_SIZE as usize
            || other.0.len() != HAND_SIZE as usize {
            return None;
        }

        let type_a = self.get_hand_type();
        let type_b = other.get_hand_type();

        return match (type_a as i64).cmp(&(type_b as i64)) {
            Ordering::Equal => {
                // Start looking at cards strength if hands are of the same type
                self.0.chars().zip(other.0.chars())
                    .find_map(|(ca, cb)| {
                        let ord = get_card_strength(ca).cmp(&get_card_strength(cb));
                        match ord {
                            Ordering::Equal => None,
                            _ => Some(ord)
                        }
                    })
            },
            val => Some(val)
        }
    }
}

fn get_card_strength(card: char) -> i64 {
    "23456789TJQKA".chars()
        .position(|c| card == c).unwrap_or(0)
        .try_into()
        .unwrap()
}

pub(crate) fn main(input: &str) -> String {
    let mut hands = input.lines().map(|l| {
        let (hand, bid) = l.split_once(' ').unwrap();
        Hand(hand.to_string(), bid.parse::<i64>().unwrap())
    }).collect::<Vec<Hand>>();

    hands.sort_by(|h1, h2| h1.partial_cmp(&h2).unwrap());

    hands.iter().zip(1..)
        .map(|(h, rank)| h.1 * rank)
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day07a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day07a_test.txt")), "6440".to_string());
    }
}