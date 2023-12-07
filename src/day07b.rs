use std::collections::HashMap;
use std::cmp::Ordering;

const HAND_SIZE: i64 = 5;

#[derive(PartialEq, Debug)]
struct Hand(String, i64);


#[derive(Debug)]
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
        self.0.chars().for_each(|c| *char_count.entry(c).or_insert(0) += 1);

        let mut four = false;
        let mut three = false;
        let mut two_count = 0;
        let mut one_count = 0;
        let joker_count = *char_count.get(&'J').unwrap_or(&0);

        // Ignore jokers
        char_count.remove(&'J');

        for (_c, count) in char_count.iter() {
            match *count {
                5 => { return HandType::FiveOfAKind; }
                4 => { four = true; }
                3 => { three = true; },
                2 => { two_count += 1; },
                _ => { one_count += 1; },
            }
        }

        if joker_count == 5 || joker_count == 4
            || joker_count == 3 && two_count == 1
            || joker_count == 2 && three
            || joker_count == 1 && four {
            return HandType::FiveOfAKind;
        } else if joker_count == 3 && two_count == 0
            || joker_count == 2 && two_count == 1
            || joker_count == 1 && three {
            return HandType::FourOfAKind;
        } else if joker_count == 1 && two_count == 2 {
            return HandType::FullHouse;
        } else if joker_count == 2 && two_count == 0
            || joker_count == 1 && two_count == 1 {
            return HandType::ThreeOfAKind;
        } else if joker_count == 1 && two_count == 0 {
            return HandType::OnePair;
        } else if four {
            return HandType::FourOfAKind;
        } else if three && two_count == 1 {
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
    "J23456789TQKA".chars()
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

    // hands.iter().for_each(|h| println!("{}, {:?}", h.0, h.get_hand_type()));

    hands.iter().zip(1..)
        .map(|(h, rank)| h.1 * rank)
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day07b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day07b_test.txt")), "5905".to_string());
    }
}