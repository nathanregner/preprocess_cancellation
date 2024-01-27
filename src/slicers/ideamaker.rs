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
            return Ok(vec![]);
        }

        if let Ok(name) = comment(preceded("PRINTING:", rest), &line) {
            let name = name.to_owned();
            line.clear();
            file.read_line(&mut line)?;
            let id = comment(preceded("PRINTING_ID:", rest), &line)?;
            if id == "-1" {
                object_tracker.end(pos);
            } else {
                object_tracker.start(name, pos)?;
            }
        } else if comment("REMAINING_TIME: 0", &line).is_ok() {
            object_tracker.end(pos);
        } else if let Ok(extrude) = extrude_move(&line) {
            object_tracker.extrude(extrude);
        }
        line.clear();
    }

    object_tracker.into_objects()
}
