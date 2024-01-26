use std::fmt::{self, Display, Formatter};
use std::ops::Range;

#[derive(Debug)]
pub struct KnownObject {
    id: String,
    ranges: Vec<Range<u64>>,
    hull: BoundingBox,
}

impl KnownObject {
    pub fn new(id: String, range: Range<u64>, hull: BoundingBox) -> Self {
        Self {
            id: id.replace(|c: char| !c.is_ascii_alphanumeric(), "_"),
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

#[derive(Copy, Clone, Debug)]
pub struct BoundingBox {
    min: (f32, f32),
    max: (f32, f32),
}

impl BoundingBox {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            min: (x, y),
            max: (x, y),
        }
    }

    pub fn union(&mut self, x: f32, y: f32) {
        self.min.0 = self.min.0.min(x);
        self.min.1 = self.min.1.min(y);
        self.max.0 = self.max.0.max(x);
        self.max.1 = self.max.1.max(y);
    }

    pub fn union_with(&mut self, other: &Self) {
        self.union(other.min.0, other.min.1);
        self.union(other.max.0, other.max.1);
    }

    pub fn center(&self) -> (f32, f32) {
        let (x1, y1) = self.min();
        let (x2, y2) = self.max();
        ((x2 - x1) / 2.0 + x1, (y2 - y1) / 2.0 + y1)
    }

    pub fn min(&self) -> (f32, f32) {
        self.min
    }

    pub fn max(&self) -> (f32, f32) {
        self.max
    }
}

impl Display for BoundingBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (x1, y1) = self.min();
        let (x2, y2) = self.max();
        write!(f, "[[{x1},{y1}],[{x1},{y2}],[{x2},{y2}],[{x2},{y1}]]")
    }
}
