use std::collections::HashMap;

pub(crate) fn main(input: &str) -> String {
    let (workflow_str, ratings_str) = input.split_once("\n\n").unwrap();

    // Read ratings
    let ratings: Vec<HashMap<&str, usize>> = ratings_str.lines().map(|l| {
        let mut categories = HashMap::new();
        for cat in l[0..l.len() - 1].split(',') {
            categories.insert(&cat[..1], cat[2..].parse::<usize>().unwrap()).unwrap();
        }
        categories
    }).collect();

    todo!()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day19a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day19a_test.txt")), "".to_string());
    }
}