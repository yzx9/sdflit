use crate::sdf::{DynSDF, SDF};
use crate::vec3::{self, Vec3f};
use pyo3::prelude::*;
use std::sync::Arc;

#[pyfunction]
pub fn merge(a: DynSDF, b: DynSDF) -> DynSDF {
    let union: Arc<dyn SDF> = Arc::new(Union::new(a, b));
    DynSDF::from(union)
}

#[pyfunction]
pub fn intersect(a: DynSDF, b: DynSDF) -> DynSDF {
    let intersection: Arc<dyn SDF> = Arc::new(Intersection::new(a, b));
    DynSDF::from(intersection)
}

#[pyfunction]
pub fn subtract(a: DynSDF, b: DynSDF) -> DynSDF {
    let difference: Arc<dyn SDF> = Arc::new(Difference::new(a, b));
    DynSDF::from(difference)
}

pub struct Union {
    a: DynSDF,
    b: DynSDF,
    bounding_box: (Vec3f, Vec3f),
}

impl Union {
    pub fn new(a: DynSDF, b: DynSDF) -> Union {
        let (min_a, max_a) = a.bounding_box();
        let (min_b, max_b) = b.bounding_box();
        let bounding_box = (vec3::minimum(min_a, min_b), vec3::maximum(max_a, max_b));
        Self { a, b, bounding_box }
    }
}

impl SDF for Union {
    fn distance(&self, p: Vec3f) -> f32 {
        f32::min(self.a.distance(p), self.b.distance(p))
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.bounding_box
    }

    fn inside(&self, p: Vec3f) -> bool {
        self.inside_bounding_box(p) && (self.a.inside(p) || self.b.inside(p))
    }
}

pub struct Intersection {
    a: DynSDF,
    b: DynSDF,
    bounding_box: (Vec3f, Vec3f),
}

impl Intersection {
    pub fn new(a: DynSDF, b: DynSDF) -> Intersection {
        let (min_a, max_a) = a.bounding_box();
        let (min_b, max_b) = b.bounding_box();
        let bounding_box = (vec3::maximum(min_a, min_b), vec3::maximum(max_a, max_b));
        Self { a, b, bounding_box }
    }
}

impl SDF for Intersection {
    fn distance(&self, p: Vec3f) -> f32 {
        f32::max(self.a.distance(p), self.b.distance(p))
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.bounding_box
    }

    fn inside(&self, p: Vec3f) -> bool {
        self.inside_bounding_box(p) && self.a.inside(p) && self.b.inside(p)
    }
}

pub struct Difference {
    a: DynSDF,
    b: DynSDF,
    bounding_box: (Vec3f, Vec3f),
}

impl Difference {
    pub fn new(a: DynSDF, b: DynSDF) -> Difference {
        let bounding_box = a.bounding_box();
        Self { a, b, bounding_box }
    }
}

impl SDF for Difference {
    fn distance(&self, p: Vec3f) -> f32 {
        f32::max(self.a.distance(p), -self.b.distance(p))
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.bounding_box
    }

    fn inside(&self, p: Vec3f) -> bool {
        self.inside_bounding_box(p) && self.a.inside(p) && !self.b.inside(p)
    }
}
