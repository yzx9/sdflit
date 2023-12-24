use crate::primitive;
use crate::vec3::Vec3f;
use pyo3::prelude::*;
use std::sync::Arc;

pub trait SDF: Send + Sync {
    fn distance(&self, p: Vec3f) -> f32;

    fn inside(&self, p: Vec3f) -> bool {
        self.inside_bounding_box(p) && self.distance(p) < 0.0
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f);

    fn inside_bounding_box(&self, p: Vec3f) -> bool {
        let (min, max) = self.bounding_box();
        p.x >= min.x && p.y >= min.y && p.z >= min.z && p.x <= max.x && p.y <= max.y && p.z <= max.z
    }
}

#[pyclass]
#[pyo3(name = "SDF")]
#[derive(Clone)]
pub struct DynSDF(Arc<dyn SDF>);

#[pymethods]
impl DynSDF {
    #[staticmethod]
    pub fn new_frustum_cone(a: (f32, f32, f32), b: (f32, f32, f32), ra: f32, rb: f32) -> Self {
        let a = Vec3f::from(a);
        let b = Vec3f::from(b);
        Self(Arc::new(primitive::FrustumCone::new(a, b, ra, rb)))
    }

    #[staticmethod]
    pub fn new_round_cone(a: (f32, f32, f32), b: (f32, f32, f32), ra: f32, rb: f32) -> Self {
        let a = Vec3f::from(a);
        let b = Vec3f::from(b);
        Self(Arc::new(primitive::RoundCone::new(a, b, ra, rb)))
    }

    #[staticmethod]
    pub fn new_sphere(center: (f32, f32, f32), radius: f32) -> Self {
        let center = Vec3f::from(center);
        Self(Arc::new(primitive::Sphere::new(center, radius)))
    }

    fn distance(&self, p: (f32, f32, f32)) -> f32 {
        SDF::distance(self, Vec3f::from(p))
    }

    fn inside(&self, p: (f32, f32, f32)) -> bool {
        SDF::inside(self, Vec3f::from(p))
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
