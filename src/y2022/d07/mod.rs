// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

use std::collections::HashMap;

#[derive(Debug, PartialEq, macros::TryFromStr)]
struct Shell(Vec<Command>);

::aoc::derive_FromStr_for_nom!(Shell, parser::shell);

#[derive(Debug)]
struct Filesystem {
    nodes: HashMap<usize, Node>,
    pwd: usize,
}

#[derive(Debug, PartialEq, macros::Unwrap)]
enum Command {
    ChangeDir(String),
    List(Vec<ListLine>),
}

#[derive(Debug, PartialEq, macros::Unwrap)]
enum ListLine {
    File(FileLine),
    Directory(String),
}

#[derive(Debug, PartialEq)]
struct FileLine {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Node {
    #[allow(dead_code)]
    id: usize,
    parent_id: usize,
    #[allow(dead_code)]
    name: String,
    node_type: NodeType,
}

#[derive(Debug, PartialEq, macros::Unwrap)]
enum NodeType {
    File(FileNode),
    Directory(DirectoryNode),
}

#[derive(Debug, PartialEq)]
struct FileNode {
    #[allow(dead_code)]
    size: usize,
}

#[derive(Debug, PartialEq)]
struct DirectoryNode {
    /// String to node id map.
    files: HashMap<String, usize>,
    /// Sum of the size of the files in this directory.
    files_size: usize,
    /// String to node id map.
    directories: HashMap<String, usize>,
    /// Sum of the size of the files in this directory and all
    /// subdirectories.
    directories_size: usize,
}

impl Shell {
    fn evaluate(self) -> Filesystem {
        let mut fs = Filesystem::new();

        for cmd in self.0 {
            match cmd {
                Command::ChangeDir(path) => fs.chdir(&path),
                Command::List(output) => {
                    for line in output {
                        match line {
                            ListLine::File(FileLine { name, size }) => fs.touch(&name, size),
                            ListLine::Directory(name) => fs.mkdir(&name),
                        };
                    }
                }
            }
        }

        fs
    }
}

impl Filesystem {
    fn new() -> Self {
        let root = Node {
            id: 0,
            parent_id: 0,
            name: "/".to_string(),
            node_type: NodeType::Directory(DirectoryNode {
                files: HashMap::new(),
                files_size: 0,
                directories: HashMap::new(),
                directories_size: 0,
            }),
        };
        let mut nodes = HashMap::new();
        nodes.insert(0, root);
        Self { nodes, pwd: 0 }
    }

    fn child(&self, path: &str) -> Option<usize> {
        let dir = self
            .nodes
            .get(&self.pwd)
            .unwrap()
            .node_type
            .unwrap_directory_ref();

        dir.files
            .get(path)
            .copied()
            .or_else(|| dir.directories.get(path).copied())
    }

    fn chdir(&mut self, path: &str) {
        match path {
            "/" => self.pwd = 0,
            ".." => {
                self.pwd = self.nodes.get(&self.pwd).unwrap().parent_id;
            }
            child => {
                self.pwd = self.child(child).unwrap();
            }
        }
    }

    fn mkdir(&mut self, name: &str) -> usize {
        assert!(self.child(name).is_none());

        let id = self.nodes.len();
        let node = Node {
            id,
            parent_id: self.pwd,
            name: name.to_string(),
            node_type: NodeType::Directory(DirectoryNode {
                files: HashMap::new(),
                files_size: 0,
                directories: HashMap::new(),
                directories_size: 0,
            }),
        };
        self.nodes.insert(id, node);
        self.nodes
            .get_mut(&self.pwd)
            .unwrap()
            .node_type
            .unwrap_directory_mut()
            .directories
            .insert(name.to_string(), id);

        id
    }

    fn touch(&mut self, name: &str, size: usize) -> usize {
        {
            let node = self.nodes[&self.pwd].node_type.unwrap_directory_ref();
            assert!(node.files.get(name).is_none());
            assert!(node.directories.get(name).is_none());
        }

        let id = self.nodes.len();
        let node = Node {
            id,
            parent_id: self.pwd,
            name: name.to_string(),
            node_type: NodeType::File(FileNode { size }),
        };
        self.nodes.insert(id, node);

        {
            let node = self
                .nodes
                .get_mut(&self.pwd)
                .unwrap()
                .node_type
                .unwrap_directory_mut();
            node.files.insert(name.to_string(), id);
            node.files_size += size;
        }

        let mut parent = self.pwd;
        loop {
            let node = self.nodes.get_mut(&parent).unwrap();
            node.node_type.unwrap_directory_mut().directories_size += size;
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

impl Command {
    fn chdir(s: &str) -> Self {
        Self::ChangeDir(s.to_owned())
    }

    fn list(v: Vec<ListLine>) -> Self {
        Self::List(v)
    }
}

impl ListLine {
    fn file(s: &str, size: usize) -> Self {
        Self::File(FileLine {
            name: s.to_owned(),
            size,
        })
    }

    fn directory(s: &str) -> Self {
        Self::Directory(s.to_owned())
    }
}
