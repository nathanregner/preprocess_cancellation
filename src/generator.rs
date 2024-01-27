use crate::model::KnownObject;
use crate::parser::comment;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;
use std::vec;
use tempfile::NamedTempFile;
use winnow::combinator::rest;

fn copy_to(mut src: impl BufRead + Seek, dst: &mut impl Write, end: u64) -> io::Result<u64> {
    let pos = src.stream_position()?;
    let count = end - pos;
    io::copy(&mut src.take(count), dst)
}

pub fn rewrite(src: &Path, objects: &mut [KnownObject]) -> crate::Result<Option<NamedTempFile>> {
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
) -> crate::Result<String> {
    let mut result = vec![];
    if objects.is_empty() {
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
) -> crate::Result<()> {
    src.seek(SeekFrom::Start(0))?;

    let mut line = String::new();
    while src.read_line(&mut line)? != 0 && line.is_empty() || comment(rest, &line).is_ok() {
        println!("comment: {:?}", line);
        line.clear();
    }

    let mut writer = BufWriter::new(dst);

    let last_comment = src.stream_position()?;
    src.seek(SeekFrom::Start(0))?;
    copy_to(&mut src, &mut writer, last_comment)?;

    objects.sort_by_cached_key(|o| o.id().to_string());
    for object in objects.iter() {
        let (x, y) = object.hull().center();
        writeln!(
            writer,
            "EXCLUDE_OBJECT_DEFINE NAME={} CENTER={x},{y} POLYGON={}",
            object.id(),
            object.hull()
        )?;
    }

    let mut ranges = objects
        .iter()
        .flat_map(|o| o.ranges().iter().map(|r| (o.id(), r.clone())))
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
