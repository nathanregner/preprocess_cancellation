use winnow::ascii::{float, space0};
use winnow::combinator::{delimited, permutation, preceded};
use winnow::error::{ContextError, ParserError};
use winnow::stream::{AsChar, Stream, StreamIsPartial};
use winnow::token::take_till;
use winnow::Parser;

#[derive(Debug)]
pub struct ExtrudeMove {
    pub x: f32,
    pub y: f32,
    pub e: f32,
}

fn trim<I, O, E: ParserError<I>>(inner: impl Parser<I, O, E>) -> impl Parser<I, O, E>
where
    I: Stream + StreamIsPartial,
    <I as Stream>::Token: AsChar + Clone,
{
    delimited(space0, inner, space0)
}

// pub trait StrParser<'i, O> = Parser<&'i str, O, TreeError<&'i str>>;

pub fn comment<'i, O>(
    inner: impl Parser<&'i str, O, ContextError<&'i str>>,
    input: &'i str,
) -> crate::Result<O> {
    Ok(preceded(';', trim(inner)).parse(input)?)
}

pub fn extrude_move(input: &str) -> crate::Result<ExtrudeMove> {
    let x = trim(preceded('X', float));
    let y = trim(preceded('Y', float));
    let e = trim(preceded('E', float));
    let i: &mut &str = &mut &input[..];
    Ok(preceded(
        ('G', take_till(1.., AsChar::is_space)),
        permutation((x, y, e)).map(|(x, y, e)| ExtrudeMove { x, y, e }),
    )
    .parse_next(i)?)
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO:
    #[test]
    fn test_extrude_move() {
        let result = extrude_move("G1 X98.536 Y84.964 E2.15296");
        dbg!(&result);
    }
}
