use crate::model::KnownObject;
use crate::parser::comment;
use std::io::{BufRead, BufReader, Read, Seek, Write};
use winnow::combinator::rest;

pub mod cura;
pub mod ideamaker;
pub mod slic3r;

pub fn list_objects(mut file: (impl Read + Write + Seek)) -> crate::Result<Vec<KnownObject>> {
    let mut reader = BufReader::new(&mut file);
    let mut line = String::new();
    let slicer = loop {
        if reader.read_line(&mut line)? == 0 {
            break None;
        }
        let Ok(comment) = comment(rest, &line) else {
            if line.is_empty() {
                continue;
            } else {
                break None;
            }
        };

        if comment.contains("Cura") {
            break Some(Slicer::Cura);
        }
        if comment.contains("Slic3r")
            || comment.contains("PrusaSlicer")
            || comment.contains("SuperSlicer")
        {
            break Some(Slicer::Slic3r);
        }
        if comment.contains("ideaMaker") {
            break Some(Slicer::IdeaMaker);
        }
    };

    let Some(slicer) = slicer else {
        return Err(crate::error::Error::Parse("Unknown slicer".to_string()));
    };

    slicer.list_objects(&mut reader)
}

#[derive(Copy, Clone, Debug)]
pub enum Slicer {
    Cura,
    Slic3r,
    IdeaMaker,
}

impl Slicer {
    pub fn list_objects(
        &self,
        reader: &mut (impl BufRead + Seek),
    ) -> crate::Result<Vec<KnownObject>> {
        match self {
            Slicer::Cura => cura::list_objects(reader),
            Slicer::Slic3r => slic3r::list_objects(reader),
            Slicer::IdeaMaker => ideamaker::list_objects(reader),
        }
    }
}
