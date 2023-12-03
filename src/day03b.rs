use std::collections::HashSet;
use std::collections::HashMap;

fn digit_count(n: i32) -> i32 {
    (n.checked_ilog10().unwrap_or(0) + 1) as i32
}

fn get_num(current_num: &mut String, num_list: &mut Vec<i32>, nums: &mut HashMap<(i32, i32), i32>, pos: (i32, i32)) {
    if !current_num.is_empty() {
        // Read number
        let n = current_num.parse::<i32>().unwrap();
        current_num.clear();

        // Save to list
        num_list.push(n);

        // Add number positions to hashmap
        let digits = digit_count(n);
        for x in (pos.1 - digits + 1)..=(pos.1) {
            nums.insert((pos.0, x), n);
        }
    }
}

pub(crate) fn main(input: &str) -> String {
    let mut gear_sum = 0;
    let mut current_num = String::new();
    let mut num_list: Vec<i32> = vec![];
    let mut nums: HashMap<(i32, i32), i32> = HashMap::new();

    // Find numbers
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => { current_num.push(c); },
                _ => { get_num(&mut current_num, &mut num_list, &mut nums, (y as i32, (x as i32) - 1)); }
            }

            if x == line.len() - 1 {
                get_num(&mut current_num, &mut num_list, &mut nums, (y as i32, x as i32))
            }
        }
    }


    // Find gears
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                // Search for gear numbers
                let mut gear_nums = vec![];
                let mut previous_was_digit = false;
                for gy in (y as i32 - 1)..=(y as i32 + 1) {
                    for gx in (x as i32 - 1)..=(x as i32 + 1) {
                        if nums.contains_key(&(gy, gx)) {
                            if !previous_was_digit {
                                gear_nums.push(nums[&(gy, gx)]);
                            }
                            previous_was_digit = true;
                        } else {
                            previous_was_digit = false;
                        }
                    }
                    previous_was_digit = false;
                }

                if gear_nums.len() == 2 {
                    gear_sum += gear_nums.iter().fold(1, |a, n| { a * (*n) });
                }
            }
        }
    }

    gear_sum.to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day03b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day03b_test.txt")), "467835".to_string());
    }
}