use crate::{
    material::{DynMaterial, Material},
    sdf::{DynSDF, SDF},
    vec3::Vec3f,
};
use pyo3::prelude::*;
use std::sync::Arc;

pub trait Object: Send + Sync {
    fn hit(&self, p: Vec3f) -> Option<Vec3f>;
    fn bounding_box(&self) -> (Vec3f, Vec3f);
}

/**
 * Wrapper for object
 */

#[pyclass]
#[pyo3[name="Object"]]
#[derive(Clone)]
pub struct DynObject(Arc<dyn Object>);

impl Into<Arc<dyn Object>> for DynObject {
    fn into(self) -> Arc<dyn Object> {
        self.0
    }
}

impl Object for DynObject {
    fn hit(&self, p: Vec3f) -> Option<Vec3f> {
        self.0.hit(p)
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.0.bounding_box()
    }
}

/**
 * Wrap a SDF into an Object
 */

#[pyclass]
#[derive(Clone)]
pub struct SDFObject {
    sdf: DynSDF,
    material: Arc<dyn Material>,
}

#[pymethods]
impl SDFObject {
    #[new]
    pub fn new(sdf: DynSDF, material: DynMaterial) -> Self {
        Self {
            sdf,
            material: material.into(),
        }
    }

    pub fn into(&self) -> DynObject {
        DynObject(Arc::new(self.clone()))
    }
}

impl Object for SDFObject {
    fn hit(&self, p: Vec3f) -> Option<Vec3f> {
        self.sdf
            .hit(p)
            .and_then(|info| Some(self.material.hit(info)))
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.sdf.bounding_box()
    }
}
