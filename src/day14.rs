use std::collections::HashMap;

const WORD_LENGTH: usize = 36;

fn parse_mask(mask: &str) -> (u64, u64, u64) {
    let mut one_mask = 0;
    let mut zero_mask = u64::MAX;
    let mut floating_mask = 0;
    mask.chars()
        .rev()
        .enumerate()
        .for_each(|(i, bit)| match bit {
            'X' => floating_mask |= 1 << i,
            '1' => one_mask |= 1 << i,
            '0' => zero_mask ^= 1 << i,
            _ => panic!(),
        });
    (one_mask, zero_mask, floating_mask)
}

pub fn task1(input: &str) -> String {
    let mut one_mask: u64 = 0;
    let mut zero_mask: u64 = u64::MAX;
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for line in input.lines() {
        let (operation, operand) = line.split_once(" = ").unwrap();
        if operation == "mask" {
            (one_mask, zero_mask, _) = parse_mask(operand);
        } else if operation.starts_with("mem") {
            let mut value = operand.parse().unwrap();
            let address: u64 = operation[4..(operation.len() - 1)].parse().unwrap();
            value = (value | one_mask) & zero_mask;
            mem.insert(address, value);
        } else {
            panic!()
        }
    }
    mem.values().sum::<u64>().to_string()
}

fn all_addresses(orig_adress: u64, floating_bits: u64) -> Vec<u64> {
    let mut addresses = vec![orig_adress];
    for i in 0..WORD_LENGTH {
        if (1 << i) & floating_bits != 0 {
            for j in 0..addresses.len() {
                // the address with either a 1 or a 0 at the ith index already exists,
                // add the other one
                addresses.push(addresses[j] ^ (1 << i));
            }
        }
    }
    addresses
}

pub fn task2(input: &str) -> String {
    let mut one_mask: u64 = 0;
    let mut floating_mask: u64 = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for line in input.lines() {
        let (operation, operand) = line.split_once(" = ").unwrap();
        if operation == "mask" {
            (one_mask, _, floating_mask) = parse_mask(operand);
        } else if operation.starts_with("mem") {
            let value = operand.parse().unwrap();
            let orig_address: u64 = operation[4..(operation.len() - 1)].parse().unwrap();
            for address in all_addresses(orig_address | one_mask, floating_mask) {
                mem.insert(address, value);
            }
        } else {
            panic!()
        }
    }
    mem.values().sum::<u64>().to_string()
}
