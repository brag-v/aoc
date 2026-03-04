use rayon::{iter::ParallelIterator, str::ParallelString};

#[derive(Debug, Clone)]
enum Rule {
    Recursive(Vec<Vec<usize>>),
    Letter(char),
}

fn parse_rule(rule: &str) -> Rule {
    if rule.starts_with('"') {
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
    }
}

fn parse_rules(rule_input: &str) -> Vec<Option<Rule>> {
    let mut stated_rules = vec![None; rule_input.lines().count().max(43)];
    for line in rule_input.lines() {
        let (i, rule) = line.split_once(": ").unwrap();
        stated_rules[i.parse::<usize>().unwrap()] = Some(parse_rule(rule));
    }

    stated_rules
}

fn valid_prefixes_rule_sequence(
    message: &str,
    rules: &[Option<Rule>],
    rule_sequence: &[usize],
) -> Box<[usize]> {
    let mut prefix_lengths = vec![0];
    for sub_rule in rule_sequence {
        let mut new_prefix_lengths = vec![];
        for prefix_length in prefix_lengths {
            for added_length in valid_prefixes(&message[prefix_length..], rules, *sub_rule) {
                new_prefix_lengths.push(prefix_length + added_length);
            }
        }
        if new_prefix_lengths.is_empty() {
            return Box::new([]);
        }
        new_prefix_lengths.sort_unstable();
        new_prefix_lengths.dedup();
        prefix_lengths = new_prefix_lengths;
    }
    prefix_lengths.into_boxed_slice()
}

fn valid_prefixes(message: &str, rules: &[Option<Rule>], base_rule: usize) -> Box<[usize]> {
    match &rules[base_rule] {
        Some(Rule::Recursive(choices)) => {
            let mut prefix_lengths = vec![];
            for sequence in choices {
                prefix_lengths
                    .extend(valid_prefixes_rule_sequence(message, rules, sequence).iter());
            }
            prefix_lengths.sort_unstable();
            prefix_lengths.dedup();
            prefix_lengths.into_boxed_slice()
        }
        Some(Rule::Letter(rule_c)) => {
            if message
                .chars()
                .next()
                .is_some_and(|message_c| message_c == *rule_c)
            {
                Box::new([1])
            } else {
                Box::new([])
            }
        }
        None => panic!("Encoundered reference to uninitialized rule: {base_rule}"),
    }
}

fn valid_message_count(messages: &str, rules: &[Option<Rule>]) -> usize {
    messages
        .par_lines()
        .filter(|message| valid_prefixes(message, rules, 0).contains(&message.len()))
        .count()
}

pub fn task1(input: &str) -> String {
    let (rule_input, messages) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rule_input);

    valid_message_count(messages, &rules).to_string()
}

pub fn task2(input: &str) -> String {
    let (rule_input, messages) = input.split_once("\n\n").unwrap();
    let mut rules = parse_rules(rule_input);

    rules[8] = Some(parse_rule("42 | 42 8"));
    rules[11] = Some(parse_rule("42 31 | 42 11 31"));

    valid_message_count(messages, &rules).to_string()
}
