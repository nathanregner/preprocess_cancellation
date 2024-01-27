use crate::model::{BoundingBox, KnownObject};
use crate::parser::ExtrudeMove;
use std::collections::HashMap;

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
            .or_insert_with_key(|id| KnownObject::new(id.to_string(), start_pos..pos, hull));
    }

    pub fn into_objects(self) -> crate::Result<Vec<KnownObject>> {
        if let Some(pending) = self.active {
            return Err(crate::error::Error::UnclosedObject(pending.id));
        }
        Ok(self.objects.into_values().collect())
    }
}
