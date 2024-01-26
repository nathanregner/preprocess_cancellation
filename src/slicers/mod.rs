use crate::bounding_box::BoundingBox;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::Path;
use std::vec;
use tempfile::NamedTempFile;
use winnow::ascii::{float, space0};
use winnow::combinator::{delimited, permutation, preceded, rest};
use winnow::error::{ParserError, TreeError};
use winnow::stream::{AsChar, Stream, StreamIsPartial};
use winnow::token::take_till;
use winnow::Parser;

pub mod cura;
pub mod slic3r;

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
    id: String,
    ranges: Vec<Range<u64>>,
    hull: BoundingBox,
}

impl KnownObject {
    pub fn new(id: String, range: Range<u64>, hull: BoundingBox) -> Self {
        Self {
            id: id.replace(|c: char| !c.is_ascii_alphanumeric(), "_"),
            ranges: vec![range],
            hull,
        }
    }

    pub fn union(&mut self, range: Range<u64>, hull: BoundingBox) {
        self.ranges.push(range);
        self.hull.union_with(&hull);
    }
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

    let last_comment = reader.stream_position()?;
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
pub enum Slicer {
    Cura,
    Slic3r,
}

impl Slicer {
    pub fn list_objects(&self, reader: &mut (impl BufRead + Seek)) -> io::Result<Vec<KnownObject>> {
        match self {
            Slicer::Cura => cura::list_objects(reader),
            Slicer::Slic3r => slic3r::list_objects(reader),
        }
    }
}

fn copy_to(mut src: impl BufRead + Seek, dst: &mut impl Write, end: u64) -> io::Result<u64> {
    let pos = src.stream_position()?;
    let count = end - pos;
    io::copy(&mut src.take(count), dst)
}

pub fn rewrite<'i>(
    src: &'i Path,
    objects: &mut [KnownObject],
) -> anyhow::Result<Option<NamedTempFile>> {
    if objects.is_empty() {
        println!("preprocess_slicer: no objects found");
        return Ok(None);
    }
    // let mut dst = NamedTempFile::new_in(src.parent().unwrap_or(src))?;
    // let dst = NamedTempFile::new()?;
    let dst = NamedTempFile::new_in(src.parent().unwrap_or(src))?;

    rewrite_to(BufReader::new(File::open(src)?), objects, dst.reopen()?)?;
    Ok(Some(dst))
}

pub fn rewrite_to_string(
    mut src: impl BufRead + Seek,
    objects: &mut [KnownObject],
) -> anyhow::Result<String> {
    let mut result = vec![];
    if objects.is_empty() {
        // println!("preprocess_slicer: no objects found");
        src.read_to_end(&mut result)?;
    } else {
        rewrite_to(BufReader::new(src), objects, &mut result)?;
    }

    Ok(String::from_utf8(result)?)
}

pub fn rewrite_to(
    mut src: impl BufRead + Seek,
    objects: &mut [KnownObject],
    dst: impl Write,
) -> anyhow::Result<()> {
    src.seek(SeekFrom::Start(0))?;

    let mut line = String::new();
    while src.read_line(&mut line)? != 0 && comment(rest).parse(&line).is_ok() {
        println!("comment: {:?}", line);
        line.clear();
    }

    let mut writer = BufWriter::new(dst);

    let last_comment = src.stream_position()?;
    src.seek(SeekFrom::Start(0))?;
    copy_to(&mut src, &mut writer, last_comment)?;

    objects.sort_by_cached_key(|o| o.id.to_string());
    for object in objects.iter() {
        let (x, y) = object.hull.center();
        writeln!(
            writer,
            "EXCLUDE_OBJECT_DEFINE NAME={} CENTER={x},{y} POLYGON={}",
            object.id, object.hull
        )?;
    }

    let mut ranges = objects
        .iter()
        .flat_map(|o| o.ranges.iter().map(|r| (&o.id, r.clone())))
        .collect::<Vec<_>>();
    ranges.sort_unstable_by_key(|(_, r)| r.start);

    println!("objects={:#?}", objects);
    for (object, range) in ranges {
        copy_to(&mut src, &mut writer, range.start)?;
        writeln!(writer, "EXCLUDE_OBJECT_START NAME={}", object)?;
        copy_to(&mut src, &mut writer, range.end)?;
        writeln!(writer, "EXCLUDE_OBJECT_END NAME={}", object)?;
    }

    io::copy(&mut src, &mut writer)?;

    writer.flush()?;

    Ok(())
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
