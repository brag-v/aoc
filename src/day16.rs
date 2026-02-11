#[derive(Debug, PartialEq, Eq)]
struct TicketField<'a> {
    name: &'a str,
    ranges: Vec<(u32, u32)>,
}

fn parse_fields<'a>(fields: &'a str) -> Box<[TicketField<'a>]> {
    fields
        .lines()
        .map(|line| {
            // cleaner with regex?
            let (name, ranges) = line.split_once(": ").unwrap();
            let words: Box<[&str]> = ranges.split(&[' ', '-']).collect();
            TicketField {
                name,
                ranges: vec![
                    (words[0].parse().unwrap(), words[1].parse().unwrap()),
                    (words[3].parse().unwrap(), words[4].parse().unwrap()),
                ],
            }
        })
        .collect()
}

pub fn task1(input: &str) -> String {
    let sections: Box<[&str]> = input.split("\n\n").collect();
    let ticket_fields = parse_fields(sections[0]);
    let nearby_tickets = sections[2];
    nearby_tickets
        .lines()
        .skip(1)
        .flat_map(|ticket| {
            ticket
                .split(',')
                .map(|value| value.parse().unwrap())
                .filter(|value| {
                    !ticket_fields
                        .iter()
                        .flat_map(|field| field.ranges.iter())
                        .any(|(lower_bound, upper_bound)| {
                            (lower_bound..=upper_bound).contains(&value)
                        })
                })
        })
        .sum::<u32>()
        .to_string()
}

/// ac3 algorithm for filtering domain of variables
/// where every variable must have an unique assignment
fn ac3<T: Clone + Eq>(domains: &mut [Vec<T>]) {
    let mut assigned = vec![false; domains.len()];
    let mut assigned_values = Vec::new();
    if let Some((i, singelton_domain)) = domains
        .iter()
        .enumerate()
        .find(|(_, domain)| domain.len() == 1)
    {
        assigned_values.push(singelton_domain[0].clone());
        assigned[i] = true;
    }
    while let Some(constrained_assignment) = assigned_values.pop() {
        for (i, domain) in domains.iter_mut().enumerate() {
            if assigned[i] {
                continue;
            }
            *domain = domain
                .iter()
                .filter(|assignment| **assignment != constrained_assignment)
                .cloned()
                .collect::<Vec<T>>();
            if domain.len() == 1 {
                assigned_values.push(domain[0].clone());
                assigned[i] = true;
            }
        }
    }
}

pub fn task2(input: &str) -> String {
    let sections: Box<[&str]> = input.split("\n\n").collect();
    let ticket_fields = parse_fields(sections[0]);
    let n = ticket_fields.len();

    // find valid tickets
    let nearby_tickets: Box<[Box<[u32]>]> = sections[2]
        .lines()
        .skip(1)
        .filter_map(|ticket| {
            let values: Box<[u32]> = ticket
                .split(',')
                .map(|value| value.parse().unwrap())
                .collect();
            if values.iter().all(|value| {
                ticket_fields
                    .iter()
                    .flat_map(|field| field.ranges.iter())
                    .any(|(lower_bound, upper_bound)| (lower_bound..=upper_bound).contains(&value))
            }) {
                debug_assert_eq!(values.len(), n);
                Some(values)
            } else {
                None
            }
        })
        .collect();

    // find possible mapping from ticket column to ticket fields
    let mut field_mapping: Vec<Vec<&TicketField>> = vec![];
    for i in 0..n {
        let field = ticket_fields
            .iter()
            .filter(|field| {
                nearby_tickets.iter().all(|ticket| {
                    field
                        .ranges
                        .iter()
                        .any(|(lower, upper)| (lower..=upper).contains(&&ticket[i]))
                })
            })
            .collect();
        field_mapping.push(field);
    }

    // for this task, the ac3 algorithm is enough to find
    // a single mapping between columns and ticket fileds
    ac3(&mut field_mapping);

    debug_assert!(field_mapping.iter().all(|mapping| mapping.len() == 1));

    // solve task (product of ticket fields starting with departure)
    let your_ticket = sections[1].split_once('\n').unwrap().1;
    your_ticket
        .split(',')
        .enumerate()
        .filter(|(i, _)| field_mapping[*i][0].name.starts_with("departure"))
        .map(|(_, value)| value.parse::<u64>().unwrap())
        .product::<u64>()
        .to_string()
}
