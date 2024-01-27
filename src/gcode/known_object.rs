use super::BoundingBox;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct KnownObject {
    id: String,
    ranges: Vec<Range<u64>>,
    hull: BoundingBox,
}

impl KnownObject {
    pub fn new(id: &str, range: Range<u64>, hull: BoundingBox) -> Self {
        Self {
            id: escape_id(id),
            ranges: vec![range],
            hull,
        }
    }

    pub fn union(&mut self, range: Range<u64>, hull: BoundingBox) {
        self.ranges.push(range);
        self.hull.union_with(&hull);
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn ranges(&self) -> &[Range<u64>] {
        &self.ranges
    }

    pub fn hull(&self) -> &BoundingBox {
        &self.hull
    }
}

fn escape_id(id: &str) -> String {
    let mut escaped = String::new();
    let mut escaping = false;
    for c in id.chars() {
        if c.is_ascii_alphanumeric() {
            escaped.push(c);
            escaping = false;
        } else if !escaping {
            if !escaped.is_empty() {
                escaped.push('_');
            }
            escaping = true;
        }
    }
    if let Some('_') = escaped.chars().last() {
        escaped.pop();
    }
    escaped
}
