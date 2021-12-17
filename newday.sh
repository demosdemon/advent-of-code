#!/bin/bash

if [ -z "$1" ]; then
  echo "required day arg" >&2
  exit 1
fi

echo "stubbing $1"
mkdir -p ./src/y2021/$1/inputs
touch ./src/y2021/$1/inputs/{live,example}

echo "mod $1;" >> ./src/y2021/mod.rs

echo "mod part1;" >> ./src/y2021/$1/mod.rs
echo "// mod part2;" >> ./src/y2021/$1/mod.rs

cat >./src/y2021/$1/part1.rs <<"EOF"
/*

*/

crate::problem! {
    struct Answer(input: &super::Ocean) -> isize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    crate::tests_for_problem!(super::Answer, {
        example => 0,
        // live => 0,
    });
}
EOF

cp ./src/y2021/$1/part1.rs ./src/y2021/$1/part2.rs
