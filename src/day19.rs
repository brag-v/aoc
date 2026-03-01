#[derive(Debug, Clone)]
enum Rule {
    Recursive(Vec<Vec<usize>>),
    Letter(char),
}

fn parse_rules(rule_input: &str) -> Vec<Rule> {
    let mut stated_rules = vec![None; rule_input.lines().count()];
    for line in rule_input.lines() {
        let (i, rule) = line.split_once(": ").unwrap();
        let rule = if rule.starts_with('"') {
            Rule::Letter(rule.chars().nth(1).unwrap())
        } else {
            Rule::Recursive(
                rule.split(" | ")
                    .map(|sequence| {
                        sequence
                            .split(' ')
                            .map(|index| index.parse().unwrap())
                            .collect()
                    })
                    .collect(),
            )
        };
        stated_rules[i.parse::<usize>().unwrap()] = Some(rule);
    }

    stated_rules
        .iter()
        .map(|rule| rule.to_owned().unwrap())
        .collect()
}

fn is_valid(message: &str, rules: &[Rule], base_rule: usize) -> Option<usize> {
    match &rules[base_rule] {
        Rule::Recursive(choices) => {
            'choice: for sequence in choices {
                let mut prefix_length = 0;
                for sub_rule in sequence {
                    match is_valid(&message[prefix_length..], rules, *sub_rule) {
                        Some(l) => prefix_length += l,
                        None => continue 'choice,
                    }
                }
                return Some(prefix_length);
            }
            None
        }
        Rule::Letter(c) => {
            if *c == message.chars().next().unwrap() {
                Some(1)
            } else {
                None
            }
        }
    }
}

pub fn task1(input: &str) -> String {
    let (rule_input, messages) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rule_input);

    messages
        .lines()
        .filter(|message| {
            is_valid(message, &rules, 0).is_some_and(|length| length == message.len())
        })
        .count()
        .to_string()
}

pub fn task2(_input: &str) -> String {
    todo!("Day 1 task 2")
}
