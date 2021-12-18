use nom::{branch::alt, bytes::complete::tag, character::complete::u64, IResult};

fn node_value(s: &str) -> IResult<&str, super::Node> {
    let (s, v) = u64(s)?;
    Ok((s, (v as usize).into()))
}

fn node_pair(s: &str) -> IResult<&str, super::Node> {
    let (s, v) = snailfish(s)?;
    Ok((s, v.into()))
}

fn node(s: &str) -> IResult<&str, super::Node> {
    alt((node_value, node_pair))(s)
}

pub(super) fn snailfish(s: &str) -> IResult<&str, super::Snailfish> {
    let (s, _) = tag("[")(s)?;
    let (s, x) = node(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, y) = node(s)?;
    let (s, _) = tag("]")(s)?;
    Ok((s, (x, y).into()))
}

#[cfg(test)]
mod tests {
    macros::test_roundtrip!(
        super::super::Snailfish,
        "[1,2]",
        "[[1,2],3]",
        "[9,[8,7]]",
        "[[1,9],[8,5]]",
        "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
        "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
        "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
        "[[3,4],5]",
        "[[1,2],[[3,4],5]]",
        "[[[[[9,8],1],2],3],4]",
        "[[[[0,9],2],3],4]",
        "[7,[6,[5,[4,[3,2]]]]]",
        "[7,[6,[5,[7,0]]]]",
        "[[6,[5,[4,[3,2]]]],1]",
        "[[6,[5,[7,0]]],3]",
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        "[[[[4,3],4],4],[7,[[8,4],9]]]",
    );
}
