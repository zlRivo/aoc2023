use std::collections::HashMap;

fn count_possible<'a>(s: &'a str, nums: Vec<i64>, mut cache: &mut HashMap<(&'a str, Vec<i64>), i64>) -> i64 {
    if s == "" {
        return if nums.is_empty() { 1 } else { 0 };
    }

    if nums.is_empty() {
        return if s.contains('#') { 0 } else { 1 };
    }

    if let Some(result) = cache.get(&(s, nums.clone())) {
        return *result;
    }

    let mut result = 0;

    if s.as_bytes()[0] == b'.' || s.as_bytes()[0] == b'?' {
        result += count_possible(&s[1..], nums.clone(), &mut cache);
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
            result += count_possible(new_s, new_nums, &mut cache);
        }
    }

    // Memoization
    cache.insert((s, nums.clone()), result);

    result
}

pub(crate) fn main(input: &str) -> String {
    input.lines().map(|l| {
        let (row, nums_str) = l.split_once(' ').unwrap();
        let nums: Vec<i64> = nums_str.split(',').map(|n| n.parse::<i64>().unwrap()).collect();

        let mut new_row = String::new();
        let mut new_nums = vec![];
        for _ in 0..5 {
            new_row.push_str(row);
            new_row.push('?');
            for n in nums.iter() {
                new_nums.push(*n);
            }
        }
        new_row.pop();

        let mut cache: HashMap<(&str, Vec<i64>), i64> = HashMap::new();

        count_possible(&new_row, new_nums, &mut cache)
    }).sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day12b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day12b_test.txt")), "525152".to_string());
    }
}