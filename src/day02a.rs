use std::collections::HashMap;

pub(crate) fn main(input: &str) -> String {
    let mut max_cubes: HashMap<&'static str, u32> = HashMap::new();
    max_cubes.insert("red", 12);
    max_cubes.insert("green", 13);
    max_cubes.insert("blue", 14);

    input.lines()
        .filter_map(|line| {
            let game: Vec<&str> = line.split(":").collect();
            let game_id: u32 = game[0].split_whitespace().last().unwrap().parse().unwrap();

            game[1].split(";").all(|set| {
                set.split(",").all(|cubes| {
                    let pick: Vec<&str> = cubes.trim().split_whitespace().collect();
                    // Amount of cubes must be under max cubes
                    pick[0].parse::<u32>().unwrap() <= max_cubes[pick[1]]
                })
            })
            .then(|| Some(game_id))
        })
        .flatten()
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day02a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day02a_test.txt")), "8".to_string());
    }
}