use std::sync::Arc;

use crate::sdf::{DynSDF, SDF};
use crate::vec3::Vec3f;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Sphere {
    center: Vec3f,
    radius: f32,
    bounding_box: (Vec3f, Vec3f),
}

#[pymethods]
impl Sphere {
    #[new]
    pub fn new(center: (f32, f32, f32), radius: f32) -> Self {
        let center = Vec3f::from(center);

        Self {
            center,
            radius,
            bounding_box: (center - radius, center + radius),
        }
    }

    pub fn distance(&self, p: (f32, f32, f32)) -> f32 {
        SDF::distance(self, Vec3f::from(p))
    }

    pub fn inside(&self, p: (f32, f32, f32)) -> bool {
        SDF::inside(self, Vec3f::from(p))
    }

    pub fn into(&self) -> DynSDF {
        let arc: Arc<dyn SDF> = Arc::new(self.clone());
        DynSDF::from(arc)
    }
}

impl SDF for Sphere {
    fn distance(&self, p: Vec3f) -> f32 {
        sd_sphere(p, self.center, self.radius)
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.bounding_box
    }
}

fn sd_sphere(p: Vec3f, c: Vec3f, r: f32) -> f32 {
    (p - c).norm() - r
}
