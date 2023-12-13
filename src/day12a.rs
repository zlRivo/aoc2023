fn count_possible(s: &str, nums: Vec<i64>) -> i64 {
    if s == "" {
        return if nums.is_empty() { 1 } else { 0 };
    }

    if nums.is_empty() {
        return if s.contains('#') { 0 } else { 1 };
    }

    let mut result = 0;

    if s.as_bytes()[0] == b'.' || s.as_bytes()[0] == b'?' {
        result += count_possible(&s[1..], nums.clone());
    }
    
    if s.as_bytes()[0] == b'#' || s.as_bytes()[0] == b'?' {
        let n = *nums.first().unwrap() as usize;
        if n <= s.len() && !&s[..n].contains('.') && (s.len() == n || s.as_bytes()[n] != b'#') {
            let mut new_nums = nums.clone();
            new_nums.remove(0);
            let new_s = if n + 1 > s.len() - 1 {
                ""
            } else {
                &s[(n + 1)..]
            };
            result += count_possible(new_s, new_nums);
        }
    }

    result
}

pub(crate) fn main(input: &str) -> String {
    input.lines().map(|l| {
        let (row, nums_str) = l.split_once(' ').unwrap();
        let nums: Vec<i64> = nums_str.split(',').map(|n| n.parse::<i64>().unwrap()).collect();
        count_possible(row, nums)
    }).sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day12a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day12a_test.txt")), "21".to_string());
    }
}