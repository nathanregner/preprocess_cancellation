use super::{comment, ExtrudeMove, KnownObject};
use crate::bounding_box::BoundingBox;
use std::io::{self, BufRead, Seek};
use std::vec;
use winnow::combinator::{preceded, rest};
use winnow::Parser;

pub fn list_objects(file: &mut (impl BufRead + Seek)) -> io::Result<Vec<KnownObject>> {
    let mut objects = vec![];

    let mut printing = None;
    let mut hull: Option<BoundingBox> = None;

    let mut line = String::new();
    while file.read_line(&mut line)? != 0 {
        let pos = file.stream_position()?;

        if line.starts_with("EXCLUDE_OBJECT_DEFINE") {
            return Ok(vec![]);
        }

        if let Some(id) = comment(preceded("printing object", rest))
            .parse(&*line)
            .ok()
        {
            printing = Some((id.trim().to_owned(), pos));
        } else if let Some(_) = comment(preceded("stop printing", rest)).parse(&*line).ok() {
            if let Some(hull) = hull.take() {
                let (id, start_pos) = printing.take().expect("printing");
                objects.push(KnownObject {
                    id,
                    start_pos,
                    end_pos: pos,
                    hull,
                });
            }
        } else if let Some(extrude) = ExtrudeMove::parser().parse_next(&mut &*line).ok() {
            if printing.is_some() {
                if let Some(hull) = &mut hull {
                    hull.union(extrude.x, extrude.y);
                } else {
                    hull = Some(BoundingBox::new(extrude.x, extrude.y));
                }
            }
        }
        line.clear();
    }

    Ok(objects)
}
