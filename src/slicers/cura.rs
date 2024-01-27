use crate::model::{KnownObject, ObjectTracker};
use crate::parser::{comment, extrude_move};
use std::io::{BufRead, Seek};
use winnow::combinator::{preceded, rest};

pub fn list_objects(file: &mut (impl BufRead + Seek)) -> crate::Result<Vec<KnownObject>> {
    let mut object_tracker = ObjectTracker::default();

    let mut line = String::new();
    while file.read_line(&mut line)? != 0 {
        let pos = file.stream_position()?;

        if line.starts_with("EXCLUDE_OBJECT_DEFINE") {
            return Err(crate::error::Error::AlreadySupported);
        }

        if let Ok(id) = comment(preceded("MESH:", rest), &line) {
            object_tracker.end(pos);
            if id != "NONMESH" {
                object_tracker.start(id.trim().to_owned(), pos)?;
            }
        } else if comment(preceded("TIME_ELAPSED", rest), &line).is_ok() {
            object_tracker.end(pos);
        } else if let Ok(extrude) = extrude_move(&line) {
            object_tracker.extrude(extrude);
        }
        line.clear();
    }

    object_tracker.into_objects()
}
