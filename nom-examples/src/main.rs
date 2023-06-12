use nom::bytes::complete::tag;
use nom::sequence::{pair, tuple};
use nom::IResult;

fn hello_parser(input: &str) -> IResult<&str, &str> {
    tag("hello")(input)
}

fn common_parser(input: &str) -> IResult<&str, &str> {
    tag(", ")(input)
}

fn world_parser(input: &str) -> IResult<&str, &str> {
    tag("world")(input)
}
fn parser_select(input: &str) -> IResult<&str, (&str, &str, &str)> {
    match hello_parser(input) {
        Ok((input, hello_out)) => match common_parser(input) {
            Ok((input, common_out)) => match world_parser(input) {
                Ok((input, world_out)) => Ok((input, (hello_out, common_out, world_out))),
                Err(e) => Err(e),
            },
            Err(er) => return Err(er),
        },
        Err(e) => Err(e),
    }
}
// nom 优化
// 方式一 pair两两嵌套组合2个以上解析器
fn pair_parse(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    pair(tag("hello"), pair(tag(", "), tag("world")))(input)
}
// 方式二 tuple 支持同时组合20多个解析器
fn tuple_combine(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((tag("hello"), tag(", "), tag("world")))(input)
}
fn main() {
    let str = "hello, world";
    let res = parser_select(str);
    assert_eq!(res, Ok(("", ("hello", ", ", "world"))));

    //pair 组合方式
    assert_eq!(pair_parse(str), Ok(("", ("hello", (", ", "world")))));

    // tuple组合方式
    assert_eq!(tuple_combine(str), Ok(("", ("hello", ", ", "world"))));
}
