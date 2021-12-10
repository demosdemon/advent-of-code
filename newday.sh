#!/bin/bash

if [ -z "$1" ]; then
  echo "required day arg" >&2
  exit 1
fi

echo "stubbing $1"
mkdir -p ./src/y2021/$1/inputs
touch ./src/y2021/$1/inputs/{live,example}

echo "pub mod $1;" >> ./src/y2021/mod.rs

echo "pub mod part1;" >> ./src/y2021/$1/mod.rs
echo "pub mod part2;" >> ./src/y2021/$1/mod.rs

cat >./src/y2021/$1/part1.rs <<"EOF"
/*

*/

use std::io::BufRead;

use crate::errors::Error;
use crate::problem::Problem;
use crate::IntoAnswer;

#[derive(Default, Debug, macros::Answer)]
#[answer(example = 0, live = 0)]
struct Answer;

impl<R: BufRead> TryFrom<Problem<R>> for Answer {
    type Error = Error;

    fn try_from(value: Problem<R>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl IntoAnswer for Answer {
    fn into_answer(self) -> isize {
        todo!()
    }
}
EOF

cp ./src/y2021/$1/part1.rs ./src/y2021/$1/part2.rs
