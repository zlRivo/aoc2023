pub(crate) fn main(input: &str) -> String {
    input.lines()
        .map(|l| {
            let mut nums_tree: Vec<Vec<i64>> = vec![];
            let line_nums: Vec<i64> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();

            nums_tree.push(line_nums);

            // Create number tree
            let mut last_line_i = nums_tree.len() - 1;
            while nums_tree[last_line_i].iter().any(|n| *n != 0) {
                let mut new_line: Vec<i64> = vec![];
                
                for i in 0..(nums_tree[last_line_i].len() - 1) {
                    new_line.push(nums_tree[last_line_i][i + 1] - nums_tree[last_line_i][i]);
                }

                nums_tree.push(new_line);
                last_line_i = nums_tree.len() - 1;
            }

            // Find extrapolated numbers
            let mut extrapolated = vec![0_i64];
            for i in (0..(nums_tree.len() - 1)).rev() {
                extrapolated.push(nums_tree[i].first().unwrap() - extrapolated.last().unwrap())
            }

            *extrapolated.last().unwrap()
        })
        .sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use aoc2023::read_file;

    #[test]
    fn day09b_test() {
        assert_eq!(super::main(&read_file!("./inputs/day09b_test.txt")), "2".to_string());
    }
}