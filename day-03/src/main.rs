use std::collections::HashSet;

fn main() {
    let data = std::fs::read_to_string("./data/03.txt").unwrap();
    println!(
        "Day 03: {} is the total priority for the rearrangement of all rucksacks.",
        sum_compartment_priorities(&data)
    );
    println!(
        "Day 03: The sum of all priorities across all groups is {} - the elves have stuff to sort!",
        sum_group_priorities(&data)
    );
}

const CHARCODE_UPPER_A: u8 = 0x41;
const CHARCODE_LOWER_A: u8 = 0x61;

fn calculate_priority(byte: &u8) -> usize {
    return if byte >= &CHARCODE_LOWER_A {
        (byte - CHARCODE_LOWER_A + 1).into()
    } else {
        (byte - CHARCODE_UPPER_A + 27).into()
    };
}

fn sum_compartment_priorities(data: &str) -> usize {
    return data
        .lines()
        .map(|line| {
            let mut first = line[0..line.len() / 2].bytes();
            let second: Vec<u8> = line[line.len() / 2..line.len()].bytes().collect();
            let opt = &first.find(|byte| second.contains(&byte)).unwrap();
            return calculate_priority(opt);
        })
        .sum::<usize>();
}

fn find_uniqe_bytes<'a>(array: &[&'a Vec<u8>]) -> Vec<&'a u8> {
    let mut bytes: HashSet<&u8> = HashSet::from_iter(array[0]);
    for vec in array {
        let set: HashSet<&u8> = HashSet::from_iter(*vec);
        bytes.retain(|byte| set.contains(byte));
    }

    return bytes.iter().cloned().collect::<Vec<_>>();
}

fn sum_group_priorities(data: &str) -> usize {
    let mut total_priority = 0;
    let mut number_of_newlines = 0;

    let mut first: Vec<u8> = Vec::new();
    let mut secnd: Vec<u8> = Vec::new();
    let mut third: Vec<u8> = Vec::new();

    for byte in data.bytes().chain("\n".bytes()) {
        if byte != 10 {
            match number_of_newlines % 3 {
                0 => first.push(byte),
                1 => secnd.push(byte),
                2 => third.push(byte),
                _ => panic!("Oh my god we fond a number mod 3 that is not 0,1 or 2!"),
            }
        } else {
            if number_of_newlines % 3 == 2 {
                let array = [&first, &secnd, &third];
                let res = find_uniqe_bytes(&array);
                total_priority += calculate_priority(&res[0]);
                first.drain(0..first.len());
                secnd.drain(0..secnd.len());
                third.drain(0..third.len());
            }
            number_of_newlines += 1;
        }
    }

    return total_priority;
}

#[cfg(test)]
mod day_03 {
    use super::*;

    const RUCKSACKS_CONTENTS: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_one_test() {
        assert_eq!(157, sum_compartment_priorities(RUCKSACKS_CONTENTS));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(70, sum_group_priorities(RUCKSACKS_CONTENTS));
    }
}
