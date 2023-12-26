use crate::vec3::Vec3f;
use pyo3::prelude::*;
use std::sync::Arc;

pub trait SDF: Send + Sync {
    fn distance(&self, p: Vec3f) -> f32;

    fn hit(&self, p: Vec3f) -> Option<SDFHitInfo> {
        let sd = self.distance(p);
        if sd < 0.0 {
            Some(SDFHitInfo {
                distance: sd,
                u: 0.0,
                v: 0.0,
                w: 0.0,
            })
        } else {
            None
        }
    }

    fn inside(&self, p: Vec3f) -> bool {
        self.inside_bounding_box(p) && self.distance(p) < 0.0
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f);

    fn inside_bounding_box(&self, p: Vec3f) -> bool {
        let (min, max) = self.bounding_box();
        p.x >= min.x && p.y >= min.y && p.z >= min.z && p.x <= max.x && p.y <= max.y && p.z <= max.z
    }
}

pub struct SDFHitInfo {
    pub distance: f32,
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

/**
 * Wrapper for SDF
 */

#[pyclass]
#[pyo3(name = "SDF")]
#[derive(Clone)]
pub struct DynSDF(Arc<dyn SDF>);

#[pymethods]
impl DynSDF {
    fn distance(&self, p: (f32, f32, f32)) -> f32 {
        SDF::distance(self, Vec3f::from(p))
    }

    fn inside(&self, p: (f32, f32, f32)) -> bool {
        SDF::inside(self, Vec3f::from(p))
    }

    fn bounding_box(&self) -> ((f32, f32, f32), (f32, f32, f32)) {
        let (min, max) = SDF::bounding_box(self);
        ((min.x, min.y, min.z), (max.x, max.y, max.z))
    }
}

impl SDF for DynSDF {
    fn distance(&self, p: Vec3f) -> f32 {
        self.0.distance(p)
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.0.bounding_box()
    }
}

impl From<Arc<dyn SDF>> for DynSDF {
    fn from(sdf: Arc<dyn SDF>) -> Self {
        Self(sdf)
    }
}
