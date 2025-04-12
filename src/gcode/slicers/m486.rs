use crate::gcode::parser::{extrude_move, trim};
use crate::gcode::{last_comment, ObjectTracker};
use crate::patch::Patch;
use std::io::{BufRead, Seek};
use winnow::ascii::dec_int;
use winnow::combinator::{alt, preceded};
use winnow::token::rest;
use winnow::Parser;

pub fn format_patch(src: &mut (impl BufRead + Seek)) -> crate::Result<Patch> {
    let mut patch = Patch::default();
    let mut object_tracker = ObjectTracker::default();

    let mut line = String::new();
    let mut prev_pos = src.stream_position()?;
    while src.read_line(&mut line)? != 0 {
        let pos = src.stream_position()?;

        if line.starts_with("EXCLUDE_OBJECT_DEFINE") {
            return Err(crate::error::Error::AlreadySupported);
        }

        if let Ok(id) = M486::parse(&line) {
            patch.replace(prev_pos..pos, format!("; {line}"));
            if let M486::S(id) = id {
                let id = format!("{}", id);
                object_tracker.end(pos);
                object_tracker.start(id, pos)?;
            }
        } else if let Ok(extrude) = extrude_move(&line) {
            object_tracker.extrude(extrude);
        }
        line.clear();
        prev_pos = pos;
    }
    object_tracker.end(prev_pos);
    object_tracker.format_patch(&mut patch, last_comment(src)?)?;
    Ok(patch)
}

#[derive(Copy, Clone, Debug)]
enum M486 {
    S(i32),
    T,
}

impl M486 {
    fn parse(input: &str) -> crate::Result<Self> {
        Ok(preceded(
            trim("M486 "),
            alt((
                trim(preceded('S', dec_int)).map(M486::S),
                trim(preceded('T', rest)).value(M486::T),
            )),
        )
        .parse_next(&mut &*input)?)
    }
}
