use std::collections::HashMap;

enum Direction { Left, Right }

pub(crate) fn main(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    // Read directions
    let directions: Vec<Direction> = lines[0].chars().map(|c| {
        if c == 'R' { Direction::Right } else { Direction::Left }}).collect();

    // Read nodes
    (&lines[2..]).iter().for_each(|l| {
        let node = &l[0..3];
        let node_left = &l[7..10];
        let node_right = &l[12..15];
        nodes.insert(node, (node_left, node_right));
    });

    // Find step count
    let mut key = "AAA";
    let mut step_count = 0;
    let mut iterator = directions.iter().cycle();
    while key != "ZZZ" {
        key = match iterator.next().unwrap() {
            Direction::Left => nodes[key].0,
            Direction::Right => nodes[key].1
        };
        step_count += 1;
    }

    step_count.to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day08a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day08a_test.txt")), "2".to_string());
        assert_eq!(super::main(&read_file!("./inputs/day08a_test_b.txt")), "6".to_string());
    }
}