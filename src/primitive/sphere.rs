use crate::{
    sdf::{DynSDF, SDFHitInfo, SDF},
    vec3::Vec3f,
};
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
pub struct Sphere {
    center: Vec3f,
    radius: f32,
    bounding_box: (Vec3f, Vec3f),
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32) -> Self {
        Self {
            center,
            radius,
            bounding_box: (center - radius, center + radius),
        }
    }
}

#[pymethods]
impl Sphere {
    #[new]
    pub fn __new__(center: (f32, f32, f32), radius: f32) -> Self {
        Self::new(center.into(), radius)
    }

    pub fn distance(&self, p: (f32, f32, f32)) -> f32 {
        SDF::distance(self, Vec3f::from(p))
    }

    pub fn inside(&self, p: (f32, f32, f32)) -> bool {
        SDF::inside(self, Vec3f::from(p))
    }

    pub fn bounding_box(&self) -> ((f32, f32, f32), (f32, f32, f32)) {
        let (min, max) = self.bounding_box;
        ((min.x, min.y, min.z), (max.x, max.y, max.z))
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

    fn hit(&self, p: Vec3f) -> Option<SDFHitInfo> {
        let distance = SDF::distance(self, p);
        if distance <= 0.0 {
            Some(SDFHitInfo {
                distance,
                u: (distance / self.radius).clamp(0.0, 1.0),
                v: 0.0,
                w: 0.0,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.bounding_box
    }
}

fn sd_sphere(p: Vec3f, c: Vec3f, r: f32) -> f32 {
    (p - c).norm() - r
}
