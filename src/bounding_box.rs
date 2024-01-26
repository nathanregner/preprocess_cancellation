use std::fmt::{self, Display, Formatter};

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
        ((x1 + x2) / 2.0, (y1 + y2) / 2.0)
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
