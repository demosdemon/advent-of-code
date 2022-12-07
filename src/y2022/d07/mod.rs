mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Command {
    ChangeDir { path: String },
    List { output: Vec<ListLine> },
}

impl Command {
    fn chdir(s: &str) -> Self {
        Self::ChangeDir {
            path: s.to_string(),
        }
    }

    fn list(v: Vec<ListLine>) -> Self {
        Self::List { output: v }
    }
}

#[derive(Debug, PartialEq)]
enum ListLine {
    File { name: String, size: usize },
    Directory { name: String },
}

impl ListLine {
    fn file(s: &str, size: usize) -> Self {
        Self::File {
            name: s.to_string(),
            size,
        }
    }

    fn directory(s: &str) -> Self {
        Self::Directory {
            name: s.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, macros::TryFromStr)]
struct Shell(Vec<Command>);

::aoc::derive_FromStr_for_nom!(Shell, parser::shell);

impl Shell {
    fn evaluate(self) -> Filesystem {
        let mut fs = Filesystem::new();

        for cmd in self.0 {
            match cmd {
                Command::ChangeDir { path } => fs.chdir(&path),
                Command::List { output } => {
                    for line in output {
                        match line {
                            ListLine::File { name, size } => fs.touch(&name, size),
                            ListLine::Directory { name } => fs.mkdir(&name),
                        };
                    }
                }
            }
        }

        fs
    }
}

struct Filesystem {
    nodes: HashMap<usize, Node>,
    cd: usize,
}

impl Filesystem {
    fn new() -> Self {
        let root = Node {
            id: 0,
            parent_id: 0,
            name: "/".to_string(),
            node_type: NodeType::Directory {
                files: HashMap::new(),
                files_size: 0,
                directories: HashMap::new(),
                directories_size: 0,
            },
        };
        let mut nodes = HashMap::new();
        nodes.insert(0, root);
        Self { nodes, cd: 0 }
    }

    fn child(&self, path: &str) -> Option<usize> {
        let node = self.nodes.get(&self.cd).unwrap();
        match &node.node_type {
            NodeType::File { .. } => None,
            NodeType::Directory {
                files, directories, ..
            } => files
                .get(path)
                .copied()
                .or_else(|| directories.get(path).copied()),
        }
    }

    fn chdir(&mut self, path: &str) {
        match path {
            "/" => self.cd = 0,
            ".." => {
                self.cd = self.nodes.get(&self.cd).unwrap().parent_id;
            }
            child => {
                self.cd = self.child(child).unwrap();
            }
        }
    }

    fn mkdir(&mut self, name: &str) -> usize {
        let parent = self.nodes.get(&self.cd).unwrap();
        match &parent.node_type {
            NodeType::File { .. } => panic!("parent is a file"),
            NodeType::Directory {
                files, directories, ..
            } => {
                assert!(files.get(name).is_none());
                assert!(directories.get(name).is_none());
            }
        }

        let id = self.nodes.len();
        let node = Node {
            id,
            parent_id: self.cd,
            name: name.to_string(),
            node_type: NodeType::Directory {
                files: HashMap::new(),
                files_size: 0,
                directories: HashMap::new(),
                directories_size: 0,
            },
        };
        self.nodes.insert(id, node);
        let parent = self.nodes.get_mut(&self.cd).unwrap();
        match &mut parent.node_type {
            NodeType::File { .. } => unreachable!(),
            NodeType::Directory { directories, .. } => {
                directories.insert(name.to_string(), id);
            }
        }

        id
    }

    fn touch(&mut self, name: &str, size: usize) -> usize {
        let parent = self.nodes.get(&self.cd).unwrap();
        match &parent.node_type {
            NodeType::File { .. } => panic!("parent is a file"),
            NodeType::Directory {
                files, directories, ..
            } => {
                assert!(files.get(name).is_none());
                assert!(directories.get(name).is_none());
            }
        }

        let id = self.nodes.len();
        let node = Node {
            id,
            parent_id: self.cd,
            name: name.to_string(),
            node_type: NodeType::File { size },
        };
        self.nodes.insert(id, node);

        {
            let parent = self.nodes.get_mut(&self.cd).unwrap();
            match &mut parent.node_type {
                NodeType::Directory {
                    files, files_size, ..
                } => {
                    files.insert(name.to_string(), id);
                    *files_size += size;
                }
                _ => unreachable!(),
            }
        }

        let mut parent = self.cd;
        loop {
            let node = self.nodes.get_mut(&parent).unwrap();
            match &mut node.node_type {
                NodeType::Directory {
                    directories_size, ..
                } => {
                    *directories_size += size;
                }
                _ => unreachable!(),
            }
            if parent == 0 {
                break;
            }
            parent = node.parent_id;
        }

        id
    }

    fn walk(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }
}

enum NodeType {
    File {
        #[allow(dead_code)]
        size: usize,
    },
    Directory {
        /// String to node id map.
        files: HashMap<String, usize>,
        /// Sum of the size of the files in this directory.
        files_size: usize,
        /// String to node id map.
        directories: HashMap<String, usize>,
        /// Sum of the size of the files in this directory and all
        /// subdirectories.
        directories_size: usize,
    },
}

struct Node {
    #[allow(dead_code)]
    id: usize,
    parent_id: usize,
    #[allow(dead_code)]
    name: String,
    node_type: NodeType,
}
