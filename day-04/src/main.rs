use std::ops::RangeInclusive;

fn main() {
    let data = std::fs::read_to_string("./data/04.txt").unwrap();
    println!(
        "Day 04: There are {} assignments that fully contains the assignment of the other elve.",
        containing_assignments_count(&data)
    );
    println!(
        "Day 04: There are {} assignments that overlap.",
        overlapping_assignments_count(&data)
    );
}

fn parse<T>(data: &str, lambda: T) -> usize
where
    T: Fn(&RangeInclusive<usize>, &RangeInclusive<usize>) -> bool,
{
    return data
        .lines()
        .map(|line| {
            let x = line
                .split(|c| (c == '-') || (c == ','))
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .collect::<Vec<_>>();
            return (x[0]..=x[1], x[2]..=x[3]);
        })
        .filter(|x| lambda(&x.0, &x.1))
        .count();
}

fn containing_assignments_count(data: &str) -> usize {
    return parse(data, |a, b| {
        a.contains(&b.start()) && a.contains(&b.end())
            || b.contains(&a.start()) && b.contains(&a.end())
    });
}

fn overlapping_assignments_count(data: &str) -> usize {
    return parse(data, |a, b| {
        a.contains(&b.start())
            || a.contains(&b.end())
            || b.contains(&a.start())
            || b.contains(&a.end())
    });
}

#[cfg(test)]
mod day_04 {
    use super::*;

    const ASSIGNMENT_LIST: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n";

    #[test]
    fn containing_assignments() {
        assert_eq!(2, containing_assignments_count(ASSIGNMENT_LIST));
    }

    #[test]
    fn overlapping_assignments() {
        assert_eq!(4, overlapping_assignments_count(ASSIGNMENT_LIST));
    }
}
