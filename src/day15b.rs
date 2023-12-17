use std::collections::HashMap;

fn get_hash(input: &str) -> usize {
    input.chars().fold(0, |a, c| (a + c as usize) * 17 % 256)
}

pub(crate) fn main(input: &str) -> String {
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();

    for group in input.split(',') {
        let bytes = group.as_bytes();
        if *bytes.last().unwrap() == b'-' {
            let key = &group[..group.len() - 1];
            let hash = get_hash(key);

            // Remove key from vec
            for (h, entry) in boxes.iter_mut() {
                *entry = entry.iter().filter_map(|(k, f)| (*k != key).then_some((*k, *f))).collect();
            }
        } else {
            let (key, focal) = group.split_once('=').unwrap();
            let (hash, focal) = (get_hash(key), focal.parse::<usize>().unwrap());

            let mut key_exists = false;
            for (h, v) in boxes.iter_mut() {
                // Edit key if existing
                if let Some(entry) = v.iter_mut().find(|(k, f)| *k == key) {
                    key_exists = true;
                    entry.1 = focal;
                }
            }

            // Add key to hashmap
            if !key_exists {
                (*boxes.entry(hash).or_insert(vec![])).push((key, focal))
            }
        }
    }
    
    boxes.iter().map(|(h, v)| {
        v.iter().zip(1..).map(|((_k, f), i)| (h + 1) * i * f).sum::<usize>()
    })
    .sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day15b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day15b_test.txt")), "145".to_string());
    }
}