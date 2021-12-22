#!/bin/bash

if [ -z "$1" ]; then
  echo "required day arg" >&2
  exit 1
fi

echo "stubbing $1"
mkdir -p ./src/y2021/$1/inputs
touch ./src/y2021/$1/inputs/{live,example}

echo "pub(crate) mod $1;" >> ./src/y2021/mod.rs

echo "pub(crate) mod part1;" >> ./src/y2021/$1/mod.rs
echo "// pub(crate) mod part2;" >> ./src/y2021/$1/mod.rs

cat >./src/y2021/$1/part1.rs <<"EOF"
/*

*/

pub fn solve(input: &String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 0,
        // live => 0,
    });
}
EOF

cp ./src/y2021/$1/part1.rs ./src/y2021/$1/part2.rs
