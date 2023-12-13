use std::collections::HashSet;

pub(crate) fn main(input: &str) -> String {
    input.lines().map(|l| {
        let (row, nums_str) = l.split_once(' ').unwrap();
        let nums: Vec<i64> = nums_str.split(',').map(|n| n.parse::<i64>().unwrap()).collect();

        // Find unknowns
        let mut unknowns: HashSet<usize> = HashSet::new();
        for (i, c) in row.chars().enumerate() {
            if c == '#' {
                unknowns.insert(i);
            }
        }

        let row = row.replace("?", ".");

        let mut chars: Vec<char> = row.chars().collect();
        let mut consecutive_springs = 0;

        for u in unknowns.iter() {
            for i in 0..row.len() {
                // if u == i {

                // }

                match chars[i] {
                    '#' => { consecutive_springs += 1; },
                    _ => {

                    },
                }
            }
        }

        0
    }).sum::<i64>().to_string()
}

// Solutions:
// Brute force (find all possibilities of ?)

// Parcourir la chaine du début à la fin
// Si inconnu mettre l'état du ressort à vrai
// Si le nombre de ressorts consécutifs dépasse le numéro de la liste après avoir modifié l'état

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    #[ignore]
    fn day12a_test() {
        assert_eq!(super::main(&read_file!("./inputs/day12a_test.txt")), "".to_string());
    }
}