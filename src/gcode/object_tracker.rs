use crate::gcode::parser::{comment, ExtrudeMove};
use crate::gcode::{BoundingBox, KnownObject};
use crate::patch::Patch;
use std::collections::HashMap;
use std::io::{BufRead, Seek, SeekFrom};
use winnow::token::rest;

#[derive(Clone, Default, Debug)]
pub struct ObjectTracker {
    objects: HashMap<String, KnownObject>,
    active: Option<ActiveObject>,
}

#[derive(Clone, Debug)]
struct ActiveObject {
    id: String,
    start_pos: u64,
    hull: Option<BoundingBox>,
}

impl ObjectTracker {
    pub fn start(&mut self, id: String, pos: u64) -> crate::Result<()> {
        if let Some(pending) = &self.active {
            return Err(crate::error::Error::UnclosedObject(pending.id.to_string()));
        }
        self.active = Some(ActiveObject {
            id,
            start_pos: pos,
            hull: None,
        });
        Ok(())
    }

    pub fn extrude(&mut self, extrude: ExtrudeMove) {
        if extrude.e < 0.0 {
            return;
        }
        let Some(ActiveObject { hull, .. }) = &mut self.active else {
            return;
        };
        match hull {
            Some(hull) => hull.union(extrude.x, extrude.y),
            None => *hull = Some(BoundingBox::new(extrude.x, extrude.y)),
        }
    }

    pub fn end(&mut self, pos: u64) {
        let Some(ActiveObject {
            id,
            start_pos,
            hull: Some(hull),
        }) = self.active.take()
        else {
            return;
        };
        self.objects
            .entry(id)
            .and_modify(|o| o.union(start_pos..pos, hull))
            .or_insert_with_key(|id| KnownObject::new(id, start_pos..pos, hull));
    }

    pub fn into_patch(self, header_pos: u64) -> crate::Result<Patch> {
        let mut patch = Patch::default();
        self.format_patch(&mut patch, header_pos)?;
        Ok(patch)
    }

    pub fn format_patch(self, patch: &mut Patch, header_pos: u64) -> crate::Result<()> {
        if let Some(pending) = self.active {
            return Err(crate::error::Error::UnclosedObject(pending.id));
        }

        let mut objects = self.objects.into_values().collect::<Vec<_>>();
        objects.sort_by_cached_key(|o| o.id().to_string());
        for object in objects.iter() {
            let (x, y) = object.hull().center();
            patch.insert(
                header_pos,
                format!(
                    "EXCLUDE_OBJECT_DEFINE NAME={} CENTER={x},{y} POLYGON={}\n",
                    object.id(),
                    object.hull()
                ),
            );
        }

        let mut ranges = objects
            .iter()
            .flat_map(|o| o.ranges().iter().map(|r| (o.id(), r.clone())))
            .collect::<Vec<_>>();
        ranges.sort_unstable_by_key(|(_, r)| r.start);

        for (name, range) in ranges {
            patch.insert(range.start, format!("EXCLUDE_OBJECT_START NAME={}\n", name));
            patch.insert(range.end, format!("EXCLUDE_OBJECT_END NAME={}\n", name));
        }

        Ok(())
    }
}

pub fn last_comment(src: &mut (impl BufRead + Seek)) -> crate::Result<u64> {
    src.seek(SeekFrom::Start(0))?;

    let mut line = String::new();
    while src.read_line(&mut line)? != 0 && line.is_empty() || comment(rest, &line).is_ok() {
        line.clear();
    }

    Ok(src.stream_position()?)
}
