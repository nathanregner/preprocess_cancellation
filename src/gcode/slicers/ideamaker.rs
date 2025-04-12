use crate::gcode::parser::{comment, extrude_move, trim};
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

        if let Ok(name) = comment(preceded("PRINTING:", trim(rest)), &line) {
            let name = name.to_owned();
            line.clear();
            src.read_line(&mut line)?;
            let id = comment(preceded("PRINTING_ID:", trim(rest)), &line)?;
            object_tracker.end(pos);
            // ignore internal non-object meshes
            if id != "-1" {
                object_tracker.start(name, pos)?;
            }
        } else if comment("REMAINING_TIME: 0", &line).is_ok() {
            object_tracker.end(pos);
        } else if let Ok(extrude) = extrude_move(&line) {
            object_tracker.extrude(extrude);
        }
        line.clear();
    }

    object_tracker.into_patch(last_comment(src)?)
}
