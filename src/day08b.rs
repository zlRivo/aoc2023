use std::collections::HashMap;

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

    // Find step count
    let mut step_count = 0;
    let mut iterator = directions.iter().cycle();
    let mut keys: Vec<&&str> = nodes.keys().filter(|k| k.ends_with('A')).collect();
    while !keys.iter().all(|k| k.ends_with('Z')) {
        let direction = iterator.next().unwrap();
        for key in keys.iter_mut() {
            *key = match direction {
                Direction::Left => &mut &nodes[*key].0,
                Direction::Right => &mut &nodes[*key].1
            }
        }
        step_count += 1;
        // if step_count < 1000 {
        //     println!("{}, key[0] = {}", step_count, keys[0]);
        // }
    }

    step_count.to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day08b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day08b_test.txt")), "6".to_string());
    }
}