use std::collections::HashMap;

enum RuleOutcome<'a> { Accepted, Rejected, Redirection(&'a str) }
struct Rule<'a> { condition: Option<(fn(usize, usize) -> bool, &'a str, usize)>, outcome: RuleOutcome<'a> }

impl<'a> From<&str> for RuleOutcome<'a> {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            other => Self::Redirection(other),
        }
    }
}

fn less_than(a: usize, b: usize) -> bool { a < b }
fn greater_than(a: usize, b: usize) -> bool { a > b }

pub(crate) fn main(input: &str) -> String {
    let mut op_funcs: HashMap<u8, fn(usize, usize) -> bool> = HashMap::new();
    op_funcs.insert(b'<', less_than);
    op_funcs.insert(b'>', greater_than);

    let (workflow_str, ratings_str) = input.split_once("\n\n").unwrap();

    // Read ratings
    let ratings: Vec<HashMap<&str, usize>> = ratings_str.lines().map(|l| {
        let mut categories = HashMap::new();
        for cat in l[1..l.len() - 1].split(',') {
            categories.insert(&cat[..1], cat[2..].parse::<usize>().unwrap()).unwrap();
        }
        categories
    }).collect();

    let workflows = workflow_str.lines().map(|l| {
        let splitter = l.find('{').unwrap();
        let workflow_key = &l[..splitter];
        let rules_str = &l[splitter + 1..l.len() - 1];

        let rules = rules_str.split(',').map(|r| {
            if r.contains(':') {
                let (r_cond, outcome_key) = r.split_once(':').unwrap();
                let r_cond_splitter = r_cond.find('<').unwrap_or(r_cond.find('>').unwrap());
                let (r_key, n_str) = r_cond.split_at(r_cond_splitter);
                let n = n_str.parse::<usize>().unwrap();

                Rule { condition: Some((op_funcs[&r_cond.as_bytes()[r_cond_splitter]], r_key, n)), outcome: RuleOutcome::from(outcome_key)}
            } else {
                Rule { condition: None, outcome: RuleOutcome::from(r) }
            }
        }).collect::<Vec<_>>();

        (workflow_key, rules)
    }).collect::<HashMap<_, _>>();

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