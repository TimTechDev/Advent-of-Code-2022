use std::fmt::Debug;

use FromIterator;

#[derive(PartialEq)]
pub struct Move {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

pub fn parse(data: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let parts = data.split("\n\n").collect::<Vec<_>>();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    parts[0].lines().rev().skip(1).for_each(|f| {
        f.chars()
            .enumerate()
            .filter(|(_i, c)| c.is_alphabetic())
            .for_each(|(i, c)| {
                let i = i / 4;
                if i < stacks.len() {
                    stacks[i].push(c)
                } else {
                    stacks.push(vec![c])
                }
            });
    });

    let moves = parts[1]
        .lines()
        .map(|x| {
            x.split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Move>()
        })
        .collect::<Vec<_>>();

    return (stacks, moves);
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            &self.count, &self.from, &self.to
        )
    }
}

impl FromIterator<usize> for Move {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        return Move {
            count: iter.next().unwrap(),
            from: iter.next().unwrap(),
            to: iter.next().unwrap(),
        };
    }
}

#[cfg(test)]
mod day_05_parse {
    use super::*;
    const TESTCASE: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

    fn n_move(count: usize, from: usize, to: usize) -> Move {
        return Move { count, from, to };
    }

    #[test]
    fn containing_assignments() {
        let expected_stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let expected_moves = vec![
            n_move(1, 2, 1),
            n_move(3, 1, 3),
            n_move(2, 2, 1),
            n_move(1, 1, 2),
        ];

        assert_eq!((expected_stacks, expected_moves), parse(TESTCASE));
    }
}
