pub mod cura;
pub mod slic3r;

use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};

use winnow::{
    ascii::{float, space0},
    combinator::{delimited, permutation, preceded, rest},
    error::{ParserError, TreeError},
    stream::{AsChar, Stream, StreamIsPartial},
    token::take_till,
    Parser,
};

use crate::bounding_box::BoundingBox;

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

trait StrParser<'i, O>: Parser<&'i str, O, TreeError<&'i str>> {}
impl<'i, O, P> StrParser<'i, O> for P where P: Parser<&'i str, O, TreeError<&'i str>> {}

impl ExtrudeMove {
    pub fn parser<'i>() -> impl StrParser<'i, Self> {
        let x = trim(preceded('X', float));
        let y = trim(preceded('Y', float));
        let e = trim(preceded('E', float));
        preceded(
            ('G', take_till(1.., AsChar::is_space)),
            permutation((x, y, e)).map(|(x, y, e)| ExtrudeMove { x, y, e }),
        )
    }
}

pub fn comment<'i, O>(inner: impl StrParser<'i, O>) -> impl StrParser<'i, O> {
    preceded(';', trim(inner))
}

#[derive(Debug)]
pub struct KnownObject {
    pub id: String,
    pub start_pos: u64,
    pub end_pos: u64,
    pub hull: BoundingBox,
}

pub fn extract_objects(mut file: (impl Read + Write + Seek)) -> io::Result<()> {
    let mut reader = BufReader::new(&mut file);
    let mut line = String::new();
    let mut slicer = None;
    while reader.read_line(&mut line)? != 0 {
        if let Some(comment) = comment(rest).parse(&line).ok() {
            if comment.contains("Cura") {
                slicer = Some(Slicer::Cura);
            } else if comment.contains("Slic3r")
                || comment.contains("PrusaSlicer")
                || comment.contains("SuperSlicer")
            {
                slicer = Some(Slicer::Slic3r);
            }
        } else {
            break;
        }
    }

    let Some(slicer) = slicer else {
        panic!("Unknown slicer");
    };

    let mut last_comment = reader.stream_position()?;
    let objects = match slicer {
        Slicer::Cura => cura::list_objects(&mut reader)?,
        Slicer::Slic3r => slic3r::list_objects(&mut reader)?,
    };

    let mut writer = BufWriter::new(&mut file);
    writer.seek(SeekFrom::Start(last_comment))?;

    println!("{:#?}", objects);

    for object in objects {
        let (x, y) = object.hull.center();
        writeln!(
            writer,
            "; EXCLUDE_OBJECT_DEFINE NAME={} CENTER={x},{y} POLYGON={}",
            object.id, object.hull
        )?;
    }

    // EXCLUDE_OBJECT_DEFINE NAME=cylinder_2_stl CENTER=143.502,143.490 POLYGON=[[141.2,141.201],[141.2,145.799],[145.8,145.799],[145.8,141.201]]

    dbg!(slicer);

    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum Slicer {
    Cura,
    Slic3r,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extrude_move() {
        let result = ExtrudeMove::parser().parse("G1 X98.536 Y84.964 E2.15296");
        dbg!(&result);
    }
}
