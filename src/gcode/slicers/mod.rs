use crate::gcode::parser::comment;
use crate::patch::Patch;
use crate::DEFAULT_BUF_SIZE;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use winnow::combinator::rest;

pub mod cura;
pub mod ideamaker;
pub mod m486;
pub mod slic3r;

#[derive(Copy, Clone, Debug)]
pub enum Slicer {
    Cura,
    Slic3r,
    IdeaMaker,
    M486,
}

impl Slicer {
    pub fn infer(mut src: (impl Read + Seek)) -> crate::Result<Self> {
        src.seek(SeekFrom::Start(0))?;

        let mut reader = BufReader::with_capacity(DEFAULT_BUF_SIZE, &mut src);
        let mut line = String::new();
        let slicer = loop {
            line.clear();
            if reader.read_line(&mut line)? == 0 {
                break None;
            }

            if line.trim().is_empty() {
                continue;
            }

            let Ok(comment) = comment(rest, &line) else {
                break None;
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

        slicer.ok_or_else(|| crate::error::Error::Parse("Unknown slicer".to_string()))
    }

    pub fn format_patch(&self, src: &mut (impl BufRead + Seek)) -> crate::Result<Patch> {
        src.seek(SeekFrom::Start(0))?;
        match self {
            Slicer::Cura => cura::format_patch(src),
            Slicer::Slic3r => slic3r::format_patch(src),
            Slicer::IdeaMaker => ideamaker::format_patch(src),
            Slicer::M486 => m486::format_patch(src),
        }
    }
}
