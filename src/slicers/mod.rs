use crate::model::KnownObject;
use crate::parser::comment;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use winnow::combinator::rest;

pub mod cura;
pub mod slic3r;

pub fn extract_objects(mut file: (impl Read + Write + Seek)) -> io::Result<()> {
    let mut reader = BufReader::new(&mut file);
    let mut line = String::new();
    let mut slicer = None;
    while reader.read_line(&mut line)? != 0 {
        if let Ok(comment) = comment(rest, &line) {
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
        let (x, y) = object.hull().center();
        writeln!(
            writer,
            "; EXCLUDE_OBJECT_DEFINE NAME={} CENTER={x},{y} POLYGON={}",
            object.id(),
            object.hull()
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
