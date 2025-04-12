use winnow::ascii::{float, newline, space0};
use winnow::combinator::{delimited, opt, permutation, preceded};
use winnow::error::{ErrMode, ParseError, ParserError, TreeError};
use winnow::stream::{AsChar, Stream, StreamIsPartial};
use winnow::token::take_till;
use winnow::Parser;

#[derive(PartialEq, Debug)]
pub struct ExtrudeMove {
    pub x: f32,
    pub y: f32,
    pub e: f32,
}

pub fn trim<I, O, E: ParserError<I>>(inner: impl Parser<I, O, E>) -> impl Parser<I, O, E>
where
    I: Stream + StreamIsPartial,
    <I as Stream>::Token: AsChar + Clone,
{
    delimited(space0, inner, space0)
}

pub fn comment<'i, O>(
    inner: impl Parser<&'i str, O, TreeError<&'i str>>,
    input: &'i str,
) -> Result<O, ParseError<&'i str, TreeError<&'i str>>> {
    delimited(
        trim(';'),
        take_till(0.., AsChar::is_newline)
            .map(|s: &str| s.trim())
            .and_then(inner),
        opt(newline),
    )
    .parse(input)
}

pub fn extrude_move(input: &str) -> Result<ExtrudeMove, ErrMode<TreeError<&str>>> {
    let x = trim(preceded('X', float));
    let y = trim(preceded('Y', float));
    let e = trim(preceded('E', float));
    preceded(
        ('G', take_till(1.., AsChar::is_space)),
        permutation((x, y, e)).map(|(x, y, e)| ExtrudeMove { x, y, e }),
    )
    .parse_next(&mut &*input)
}

#[cfg(test)]
mod test {
    use super::*;
    use winnow::token::rest;

    #[test]
    fn parse_extrude_move() {
        let result = extrude_move("G1 X98.536 Y84.964 E2.15296").unwrap();
        assert_eq!(
            result,
            ExtrudeMove {
                x: 98.536,
                y: 84.964,
                e: 2.15296
            }
        );
    }

    #[test]
    fn parse_comment() {
        assert_eq!(comment(rest, "; test test\n").unwrap(), "test test");
        assert_eq!(comment(rest, "; test test ").unwrap(), "test test");
        assert_eq!(comment(rest, ";test test").unwrap(), "test test");
    }
}
