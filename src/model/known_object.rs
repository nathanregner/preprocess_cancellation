use super::BoundingBox;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct KnownObject {
    id: String,
    ranges: Vec<Range<u64>>,
    hull: BoundingBox,
}

impl KnownObject {
    pub fn new(id: String, range: Range<u64>, hull: BoundingBox) -> Self {
        let id = id
            .trim_matches(|c: char| !c.is_ascii_alphanumeric())
            .replace(|c: char| !c.is_ascii_alphanumeric(), "_");
        Self {
            id,
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
