use crate::vec3::{self, Vec3f};
use pyo3::prelude::*;
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn hit(&self, u: f32, v: f32) -> Vec3f;
}

/**
 * Wrapper for material
 */

#[pyclass]
#[pyo3(name = "Material")]
#[derive(Clone)]
pub struct DynMaterial(Arc<dyn Material>);

impl Material for DynMaterial {
    fn hit(&self, u: f32, v: f32) -> Vec3f {
        self.0.hit(u, v)
    }
}

impl From<Arc<dyn Material>> for DynMaterial {
    fn from(x: Arc<dyn Material>) -> Self {
        Self(x)
    }
}

impl Into<Arc<dyn Material>> for DynMaterial {
    fn into(self) -> Arc<dyn Material> {
        Arc::from(self)
    }
}

/**
 * Colored material
 */

#[pyclass]
#[derive(Clone, Copy)]
pub struct ColoredMaterial {
    color: Vec3f,
}

#[pymethods]
impl ColoredMaterial {
    #[new]
    pub fn new(color: (f32, f32, f32)) -> Self {
        Self {
            color: color.into(),
        }
    }

    pub fn into(&self) -> DynMaterial {
        DynMaterial(Arc::new(self.clone()))
    }
}

impl Material for ColoredMaterial {
    fn hit(&self, _u: f32, _v: f32) -> Vec3f {
        self.color
    }
}

/**
 * Horizontal axis linear gradient
 */

struct VAxisLinearGradient {
    c1: Vec3f,
    c2: Vec3f,
}

impl VAxisLinearGradient {
    pub fn new(c1: Vec3f, c2: Vec3f) -> Arc<dyn Material> {
        Arc::new(VAxisLinearGradient { c1, c2 })
    }
}

impl Material for VAxisLinearGradient {
    fn hit(&self, u: f32, _v: f32) -> Vec3f {
        let u = u.clamp(0.0, 1.0);
        vec3::interpolate(self.c1, self.c2, u)
    }
}
