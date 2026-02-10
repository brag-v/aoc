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

pub fn task2(input: &str) -> String {
    let sections: Box<[&str]> = input.split("\n\n").collect();
    let ticket_fields = parse_fields(sections[0]);
    let n = ticket_fields.len();

    // find valid tickets
    let nearby_tickets: Box<[Box<[u32]>]> = sections[2]
        .lines()
        .skip(1)
        .filter_map(|ticket| {
            let fields: Box<[u32]> = ticket
                .split(',')
                .map(|value| value.parse().unwrap())
                .collect();
            if fields.iter().any(|value| {
                !ticket_fields
                    .iter()
                    .flat_map(|field| field.ranges.iter())
                    .any(|(lower_bound, upper_bound)| (lower_bound..=upper_bound).contains(&value))
            }) {
                None
            } else {
                debug_assert_eq!(fields.len(), n);
                Some(fields)
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

    // ac-3 algorithm?
    loop {
        // TODO: change single field to queue, 
        // run while queue is not empty
        let single_fields: Box<[&TicketField]> = field_mapping
            .iter()
            .filter_map(|mapping| {
                if mapping.len() == 1 {
                    Some(mapping[0])
                } else {
                    None
                }
            })
            .collect();


        let mut changed = false;
        for mapping in &mut field_mapping {
            let old_length = mapping.len();
            if old_length == 1 {
                continue;
            }
            *mapping = mapping
                .iter()
                .cloned()
                .filter(|field| !single_fields.contains(field))
                .collect::<Vec<&TicketField>>();
            if mapping.len() != old_length {
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

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
