// SPDX-FileCopyrightText: 2023-2026 Zexin Yuan <aim@yzx9.xyz>
//
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use pyo3::prelude::*;

use crate::material::{DynMaterial, Material};
use crate::sdf::{DynSDF, SDF};
use crate::vec3::Vec3f;

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
#[allow(missing_debug_implementations)]
pub struct DynObject(Arc<dyn Object>);

impl From<DynObject> for Arc<dyn Object> {
    fn from(val: DynObject) -> Self {
        val.0
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
#[allow(missing_debug_implementations)]
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
        self.sdf.hit(p).map(|info| self.material.hit(info))
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.sdf.bounding_box()
    }
}
