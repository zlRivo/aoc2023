use std::collections::HashMap;

use aoc2023::utils::lcm;

#[derive(Debug)]
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

    // Get starting nodes
    let iterator = directions.iter().cycle();
    let mut keys: Vec<&&str> = nodes.keys().filter(|k| k.ends_with('A')).collect();

    // Count number of steps for each starting node
    let steps: Vec<i64> = keys.iter_mut().map(|key| {
        let mut step_count = 0;
        let mut iterator = iterator.clone();
        while !key.ends_with('Z') {
            *key = match iterator.next().unwrap() {
                Direction::Left => &nodes[*key].0,
                Direction::Right => &nodes[*key].1
            };
            step_count += 1;
        }
        step_count
    }).collect::<Vec<i64>>();

    // Calculate the LCM between all of them
    steps.iter().fold(1, |acc, n| lcm(acc, *n)).to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day08b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day08b_test.txt")), "6".to_string());
    }
}