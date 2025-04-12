use crate::gcode::parser::{comment, extrude_move};
use crate::gcode::{last_comment, ObjectTracker};
use crate::patch::Patch;
use std::io::{BufRead, Seek};
use winnow::combinator::preceded;
use winnow::token::rest;

pub fn format_patch(src: &mut (impl BufRead + Seek)) -> crate::Result<Patch> {
    let mut object_tracker = ObjectTracker::default();

    let mut line = String::new();
    while src.read_line(&mut line)? != 0 {
        let pos = src.stream_position()?;

        if line.starts_with("EXCLUDE_OBJECT_DEFINE") {
            return Err(crate::error::Error::AlreadySupported);
        }

        if let Ok(id) = comment(preceded("printing object", rest), &line) {
            object_tracker.start(id.trim().to_owned(), pos)?;
        } else if comment(preceded("stop printing", rest), &line).is_ok() {
            object_tracker.end(pos);
        } else if let Ok(extrude) = extrude_move(&line) {
            object_tracker.extrude(extrude);
        }
        line.clear();
    }

    object_tracker.into_patch(last_comment(src)?)
}
