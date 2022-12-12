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

// --- Part Two ---
// Now, you're ready to choose a directory to delete.
//
// The total disk space available to the filesystem is 70000000. To run the
// update, you need unused space of at least 30000000. You need to find a
// directory you can delete that will free up enough space to run the update.
//
// In the example above, the total size of the outermost directory (and thus the
// total amount of used space) is 48381165; this means that the size of the
// unused space must currently be 21618835, which isn't quite the 30000000
// required by the update. Therefore, the update still requires a directory with
// total size of at least 8381165 to be deleted before it can run.
//
// To achieve this, you have the following options:
//
// - Delete directory e, which would increase unused space by 584.
// - Delete directory a, which would increase unused space by 94853.
// - Delete directory d, which would increase unused space by 24933642.
// - Delete directory /, which would increase unused space by 48381165.
//
// Directories e and a are both too small; deleting them would not free up
// enough space. However, directories d and / are both big enough! Between
// these, choose the smallest: d, increasing unused space by 24933642.
//
// Find the smallest directory that, if deleted, would free up enough space on
// the filesystem to run the update. What is the total size of that directory?
//

fn solve(shell: super::Shell) -> usize {
    const CAP: usize = 70_000_000;
    const REQ: usize = 30_000_000;

    let fs = shell.evaluate();

    let root_size = fs.nodes[&0]
        .node_type
        .unwrap_directory_ref()
        .directories_size;

    let need = REQ - (CAP - root_size);

    let mut min = usize::MAX;
    for node in fs.walk() {
        if let Some(dir) = node.node_type.as_directory_ref() {
            if dir.directories_size >= need && dir.directories_size < min {
                min = dir.directories_size;
            }
        }
    }

    min
}

#[cfg(test)]
mod test {
    ::aoc::tests_for_problem!(super::solve, {
        example => 24933642,
        live => 7490863,
    });
}
