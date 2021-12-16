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

use std::str::FromStr;

use crate::IntoAnswer;

#[derive(Debug, macros::Answer)]
#[answer(example = 0 /*, live = 0 */)]
struct Answer;

impl FromStr for Answer {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl IntoAnswer for Answer {
    type Output = isize;

    fn into_answer(self) -> Self::Output {
        todo!()
    }
}
EOF

cp ./src/y2021/$1/part1.rs ./src/y2021/$1/part2.rs
