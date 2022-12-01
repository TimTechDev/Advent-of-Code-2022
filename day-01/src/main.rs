
fn main() {
    let list = std::fs::read_to_string("./data/01.txt").unwrap();
    let max = count_top_calories(&list);
    println!("Day 01: The elve with the most calories has {} calories.", max);
    println!("Day 01: The top tree elves with the most calories have {} calories in total.", count_top_tree_calories(&list))
}

fn parse(list: &str) -> Vec<u32> {
    return list
        .split("\n\n")
        .into_iter()
        .map(|x| {
            x.lines()
                .fold(0, |acc, l| acc + l.parse::<u32>().unwrap_or(0))
        }).collect();
}

fn count_top_calories(list: &str) -> u32 {
    return parse(list).into_iter().max().unwrap();
}

fn count_top_tree_calories(list: &str) -> u32 {
    let mut vec = parse(list);
    vec.sort();
    return vec[vec.len() - 3..vec.len()].into_iter().sum();
}

#[cfg(test)]
mod day_01 {
    use super::*;

    #[test]
    fn top_calories() {
        let list = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(24000, count_top_calories(list));
    }

    #[test]
    fn top_tree_calories() {
        let list = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(45000, count_top_tree_calories(list));
    }
}
