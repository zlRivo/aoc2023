use std::collections::HashSet;

struct Number {
    n: i32,
    pos: (i32, i32)
}

impl Number {
    fn digit_count(&self) -> i32 {
        (self.n.checked_ilog10().unwrap_or(0) + 1) as i32
    }

    fn is_adjacent_of_symbol(&self, symbols: &HashSet<(i32, i32)>) -> bool {
        let digits = self.digit_count();
        for y in (self.pos.0 - 1)..=(self.pos.0 + 1) {
            for x in (self.pos.1 - digits)..=(self.pos.1 + 1) {
                if symbols.contains(&(y, x)) {
                    return true;
                }
            }
        }
        false
    }
}

fn get_num(current_num: &mut String, num_list: &mut Vec<Number>, pos: (i32, i32)) {
    if !current_num.is_empty() {
        num_list.push(Number { n: current_num.parse::<i32>().unwrap(), pos });
        current_num.clear();
    }
}

pub(crate) fn main(input: &str) -> String {
    let mut symbols = HashSet::new();
    let mut current_num = String::new();
    let mut nums: Vec<Number> = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    get_num(&mut current_num, &mut nums, (y as i32, (x as i32) - 1))
                },
                '0'..='9' => { current_num.push(c); },
                _ => {
                    get_num(&mut current_num, &mut nums, (y as i32, (x as i32) - 1));
                    symbols.insert((y as i32, x as i32));
                }
            }

            if x == line.len() - 1 {
                get_num(&mut current_num, &mut nums, (y as i32, x as i32))
            }
        }

    }

    nums.into_iter()
        .filter_map(|n| n.is_adjacent_of_symbol(&symbols).then(|| Some(n.n)))
        .flatten()
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day03a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day03a_test.txt")), "4361".to_string());
    }
}