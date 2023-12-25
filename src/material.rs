use crate::{
    sdf::SDFHitInfo,
    vec3::{self, Vec3f},
};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn hit(&self, hit: SDFHitInfo) -> Vec3f;
}

/**
 * Wrapper for material
 */

#[pyclass]
#[pyo3(name = "Material")]
#[derive(Clone)]
pub struct DynMaterial(Arc<dyn Material>);

impl Material for DynMaterial {
    fn hit(&self, hit: SDFHitInfo) -> Vec3f {
        self.0.hit(hit)
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
    fn hit(&self, _hit: SDFHitInfo) -> Vec3f {
        self.color
    }
}

/**
 * Linear Gradient Material
 */

#[derive(Clone, Copy)]
pub enum Axis {
    U,
    V,
    W,
}

impl TryFrom<&str> for Axis {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "u" | "U" => Ok(Self::U),
            "v" | "V" => Ok(Self::V),
            "w" | "W" => Ok(Self::W),
            _ => Err(()),
        }
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct LinearGradientMaterial {
    c1: Vec3f,
    c2: Vec3f,
    axis: Axis,
}

impl LinearGradientMaterial {
    pub fn new(c1: Vec3f, c2: Vec3f, axis: Axis) -> Self {
        Self { c1, c2, axis }
    }
}

#[pymethods]
impl LinearGradientMaterial {
    #[new]
    pub fn __new__(c1: (f32, f32, f32), c2: (f32, f32, f32), axis: &str) -> PyResult<Self> {
        let axis: Axis = axis
            .try_into()
            .or_else(|_| Err(PyValueError::new_err("Invalid axis")))?;

        Ok(Self::new(c1.into(), c2.into(), axis))
    }

    pub fn into(&self) -> DynMaterial {
        DynMaterial(Arc::new(self.clone()))
    }
}

impl Material for LinearGradientMaterial {
    fn hit(&self, hit: SDFHitInfo) -> Vec3f {
        let axis = match self.axis {
            Axis::U => hit.u,
            Axis::V => hit.v,
            Axis::W => hit.w,
        };
        vec3::interpolate(self.c1, self.c2, axis.clamp(0.0, 1.0))
    }
}
