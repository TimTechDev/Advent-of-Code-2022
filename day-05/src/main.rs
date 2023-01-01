mod parse;

fn main() {
    let data = &std::fs::read_to_string("./data/05.txt").unwrap();
    println!("Day 05: The top crates of the CrateMover 9000 are: {}", first_part_count(data));
    println!("Day 05: The top crates moved by the CrateMover 9001, the one with an extra cupholder, are: {}", second_part_count(data));
}

fn first_part_count(data: &str) -> String {
    let (mut stacks, moves) = parse::parse(&data);

    for mover in moves {
        for _i in 0..mover.count {
            let ch = stacks[mover.from - 1].pop().unwrap();
            stacks[mover.to - 1].push(ch);
        }
    }

    let mut res = String::new();
    res.reserve(stacks.len());
    for mut stack in stacks {
        res.push(stack.pop().unwrap());
    }

    return res;
}

fn second_part_count(data: &str) -> String {
    let (mut stacks, moves) = parse::parse(&data);

    for mover in moves {
        let mut tmp = Vec::new();

        for _i in 0..mover.count {
            let ch = stacks[mover.from - 1].pop().unwrap();
            tmp.push(ch);
        }
        for _i in 0..mover.count {
            let ch = tmp.pop().unwrap();
            stacks[mover.to - 1].push(ch);
        }
    }

    let mut res = String::new();
    res.reserve(stacks.len());
    for mut stack in stacks {
        res.push(stack.pop().unwrap());
    }

    return res;
}

#[cfg(test)]
mod day_05 {
    use super::*;
    const TESTCASE: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

    #[test]
    fn containing_assignments() {
        assert_eq!("CMZ", first_part_count(TESTCASE));
    }

    #[test]
    fn overlapping_assignments() {
        assert_eq!("MCD", second_part_count(TESTCASE));
    }
}
