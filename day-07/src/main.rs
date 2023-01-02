use parse::FSItem;

mod parse;

fn main() {
    let data = std::fs::read_to_string("./data/07.txt").unwrap();
    println!("Day 07: {}", part_one(&data));
    println!("Day 07: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let mut fs = parse::parse(data);
    fs.du(); // calculate directory sizes
    let map = fs
        .get_nodes()
        .iter()
        .filter(|node| node.fstype == FSItem::Dir && node.size.unwrap() <= 100000)
        .map(|node| node.size.unwrap());

    return map.sum();
}

fn part_two(data: &str) -> usize {
    let mut fs = parse::parse(data);
    fs.du(); // calculate directory sizes
    let current_free_space = 70000000 - fs.get_nodes()[0].size.unwrap();
    let target = 30000000 - current_free_space;
    let mut dirs = fs
        .get_nodes()
        .iter()
        .filter(|node| node.fstype == FSItem::Dir && node.size.unwrap() >= target)
        .collect::<Vec<_>>();
    dirs.sort_by_key(|k| k.size.unwrap());
    println!("{:?}", dirs);
    return dirs.first().unwrap().size.unwrap();
}

#[cfg(test)]
mod day_07 {
    use super::*;

    const TESTCASE: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

    #[test]
    fn part_one_test() {
        assert_eq!(95437, part_one(TESTCASE));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(24933642, part_two(TESTCASE));
    }
}
