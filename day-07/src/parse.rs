#[derive(Debug, PartialEq)]
pub enum FSItem {
    Dir,
    File,
}

#[derive(Debug, PartialEq)]
pub struct FSNode {
    pub fstype: FSItem,
    pub name: String,
    pub size: Option<usize>,
}

pub struct FileSystem {
    nodes: Vec<FSNode>,
    edges: Vec<(usize, usize)>,
    cwd: usize,
}

impl FileSystem {
    fn new() -> FileSystem {
        let root = FSNode {
            fstype: FSItem::Dir,
            name: "/".to_string(),
            size: None,
        };
        return Self {
            nodes: vec![root],
            edges: Vec::new(),
            cwd: 0,
        };
    }

    pub fn get_nodes(&self) -> &[FSNode] {
        return self.nodes.as_slice();
    }

    pub fn get_edges(&self) -> &[(usize, usize)] {
        return self.edges.as_slice();
    }

    fn find_parent(&self, index: usize) -> Result<usize, &str> {
        let parents = self
            .edges
            .clone()
            .into_iter()
            .filter(|(a, b)| b == &self.cwd)
            .collect::<Vec<_>>();
        match parents.len() {
            0 => Err("No parent dir found!"),
            1 => return Ok(parents.first().unwrap().0),
            _ => Err("More then one parent dir found!"),
        }
    }

    pub fn ls(&self) {
        self.ls_children(0, 0);
    }

    fn ls_children(&self, index: usize, indent: usize) {
        (0..indent).for_each(|_| print!(" "));
        let node = self.nodes.get(index).unwrap();
        println!("{} {:?}", node.name, node.size);
        self.edges
            .clone()
            .into_iter()
            .filter(|(a, _b)| a == &index)
            .for_each(|(_a, b)| self.ls_children(b, indent + 1));
    }

    pub fn du(&mut self) -> usize {
        return self.du_children(0);
    }

    pub fn du_children(&mut self, index: usize) -> usize {
        let node = self.nodes.get(index).unwrap();
        return match node.fstype {
            FSItem::Dir => {
                if &node.size != &None {
                    node.size.unwrap()
                } else {
                    let res = self
                        .edges
                        .clone()
                        .into_iter()
                        .filter(|(a, _b)| a == &index)
                        .map(|(_a, b)| self.du_children(b))
                        .sum();
                    let node = self.nodes.get_mut(index).unwrap();
                    node.size.replace(res);
                    return res;
                }
            }
            FSItem::File => node.size.unwrap(),
        };
    }

    pub fn mkdir(&mut self, name: &str) {
        let new_index = self.nodes.len();
        self.nodes.push(FSNode {
            fstype: FSItem::Dir,
            name: name.to_owned(),
            size: None,
        });
        self.edges.push((self.cwd, new_index));
    }

    pub fn touch(&mut self, name: &str, size: usize) {
        let new_index = self.nodes.len();
        self.nodes.push(FSNode {
            fstype: FSItem::File,
            name: name.to_owned(),
            size: Some(size),
        });
        self.edges.push((self.cwd, new_index));
    }
}

pub fn parse(data: &str) -> FileSystem {
    let mut fs = FileSystem::new();

    for line in data.lines().skip(1) {
        if line.starts_with("$ cd ") {
            let path = &line[5..];
            if path.starts_with("..") {
                fs.cwd = fs
                    .find_parent(fs.cwd)
                    .expect("There shoud be only one parent by definition.");
            } else {
                fs.mkdir(path);
                fs.cwd = fs.nodes.len() - 1;
            }
        } else if line.starts_with(|ch: char| ch.is_ascii_digit()) {
            let (size, name) = line.split_once(" ").unwrap();
            fs.touch(name, size.parse().unwrap())
        }
    }

    // calculate all directory sizes
    fs.du();

    return fs;
}

#[cfg(test)]
mod day_07 {
    use super::*;

    const TESTCASE: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

    #[test]
    fn part_one_test() {
        parse(TESTCASE);
    }
}
