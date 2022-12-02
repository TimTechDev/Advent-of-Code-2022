fn main() {
    let list = std::fs::read_to_string("./data/02.txt").unwrap();
    println!(
        "Day 02: Paying my strategy we get a total score of {} Points.",
        my_strategy(&list)
    );
    println!(
        "Day 02: Paying the elves strategy we get a total score of {} Points.",
        elve_strategy(&list)
    )
}

fn my_strategy(list: &str) -> i32 {
    return list
        .lines()
        .map(|x| match x {
            "B X" => 0 + 1, // loose
            "C Y" => 0 + 2,
            "A Z" => 0 + 3,

            "A X" => 3 + 1, // draw
            "B Y" => 3 + 2,
            "C Z" => 3 + 3,

            "C X" => 6 + 1, // win
            "A Y" => 6 + 2,
            "B Z" => 6 + 3,
            _ => 0,
        })
        .sum::<i32>();
}

fn elve_strategy(list: &str) -> i32 {
    return list
        .lines()
        .map(|x| match x {
            "B X" => 0 + 1, // loose
            "C X" => 0 + 2,
            "A X" => 0 + 3,

            "A Y" => 3 + 1, // draw
            "B Y" => 3 + 2,
            "C Y" => 3 + 3,

            "C Z" => 6 + 1, // win
            "A Z" => 6 + 2,
            "B Z" => 6 + 3,
            _ => 0,
        })
        .sum::<i32>();
}

#[cfg(test)]
mod day_02 {
    use super::*;

    const STRATEGY_GUIDE: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn my_strategy_test() {
        assert_eq!(15, my_strategy(STRATEGY_GUIDE));
    }

    #[test]
    fn elve_strategy_test() {
        assert_eq!(12, elve_strategy(STRATEGY_GUIDE));
    }
}
