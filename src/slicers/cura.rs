use crate::model::{BoundingBox, KnownObject};
use crate::parser::{comment, extrude_move};
use std::collections::HashMap;
use std::io::{self, BufRead, Seek};
use std::vec;
use winnow::combinator::{not, preceded, rest};

pub fn list_objects(file: &mut (impl BufRead + Seek)) -> io::Result<Vec<KnownObject>> {
    let mut objects = HashMap::<String, KnownObject>::new();

    let mut printing = None;
    let mut hull: Option<BoundingBox> = None;

    let mut line = String::new();
    while file.read_line(&mut line)? != 0 {
        let pos = file.stream_position()?;

        if line.starts_with("EXCLUDE_OBJECT_DEFINE") {
            return Ok(vec![]);
        }

        if let Ok(id) = comment(preceded("MESH:", rest), &line) {
            if let Some(hull) = hull.take() {
                let (id, start_pos) = printing.take().expect("printing");
                objects
                    .entry(id)
                    .and_modify(|o| o.union(start_pos..pos, hull))
                    .or_insert_with_key(|id| {
                        KnownObject::new(id.to_string(), start_pos..pos, hull)
                    });
            }
            if id != "NONMESH" {
                printing = Some((id.trim().to_owned(), pos));
            }
        } else if let Ok(_) = comment(preceded("TIME_ELAPSED", rest), &line) {
            if let Some(hull) = hull.take() {
                let (id, start_pos) = printing.take().expect("printing");
                objects
                    .entry(id)
                    .and_modify(|o| o.union(start_pos..pos, hull))
                    .or_insert_with_key(|id| {
                        KnownObject::new(id.to_string(), start_pos..pos, hull)
                    });
            }
        } else if let Ok(extrude) = extrude_move(&line) {
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

    Ok(objects.into_values().collect())
}
