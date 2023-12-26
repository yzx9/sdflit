use crate::{
    accelerator::{Accelerator, BVH},
    object::{DynObject, Object},
    vec3::{self, Vec3f},
};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::sync::Arc;

pub trait Scene: Send + Sync {
    fn hit(&self, p: Vec3f) -> Vec3f;
    fn bounding_box(&self) -> Option<(Vec3f, Vec3f)>;
}

/**
 * Wrapper for scene
 */

#[pyclass]
#[pyo3(name = "Scene")]
#[derive(Clone)]
pub struct DynScene(Arc<dyn Scene>);

#[pymethods]
impl DynScene {
    fn hit(&self, p: (f32, f32, f32)) -> (f32, f32, f32) {
        self.0.hit(Vec3f::from(p)).into()
    }

    fn bounding_box(&self) -> Option<((f32, f32, f32), (f32, f32, f32))> {
        if let Some((min, max)) = self.0.bounding_box() {
            Some((min.into(), max.into()))
        } else {
            None
        }
    }
}

impl Scene for DynScene {
    fn hit(&self, p: Vec3f) -> Vec3f {
        self.0.hit(p)
    }

    fn bounding_box(&self) -> Option<(Vec3f, Vec3f)> {
        self.0.bounding_box()
    }
}

impl From<Arc<dyn Scene>> for DynScene {
    fn from(x: Arc<dyn Scene>) -> Self {
        Self(x)
    }
}

impl Into<Arc<dyn Scene>> for DynScene {
    fn into(self) -> Arc<dyn Scene> {
        self.0
    }
}

/**
 * A scene with a list of objects
 */

#[pyclass]
#[derive(Clone)]
pub struct ObjectsScene {
    objects: Option<Vec<Arc<dyn Object>>>,
    background: Vec3f,
    acceletor: Option<Arc<dyn Accelerator>>,
}

#[pymethods]
impl ObjectsScene {
    #[new]
    pub fn new() -> Self {
        Self {
            objects: Some(Vec::new()),
            background: Vec3f::new(0.0, 0.0, 0.0),
            acceletor: None,
        }
    }

    pub fn add_object(&mut self, object: DynObject) -> PyResult<()> {
        match &mut self.objects {
            Some(objs) => {
                objs.push(object.into());
                Ok(())
            }
            None => Err(PyValueError::new_err("scene is not editable")),
        }
    }

    pub fn build_bvh(&mut self) -> PyResult<()> {
        match self.objects.take() {
            Some(objs) => {
                self.acceletor = Some(Arc::from(BVH::new(objs)));
                Ok(())
            }
            None => Err(PyValueError::new_err("scene is not editable")),
        }
    }

    pub fn set_background(&mut self, background: (f32, f32, f32)) {
        self.background = background.into()
    }

    pub fn bounding_box(&self) -> Option<((f32, f32, f32), (f32, f32, f32))> {
        if let Some((min, max)) = Scene::bounding_box(self) {
            Some((min.into(), max.into()))
        } else {
            None
        }
    }

    pub fn into(&self) -> DynScene {
        DynScene(Arc::new(self.clone()))
    }
}

impl Scene for ObjectsScene {
    fn hit(&self, p: Vec3f) -> Vec3f {
        let hit = match (&self.acceletor, &self.objects) {
            (Some(acc), _) => acc.hit(p),
            (None, Some(objs)) => objs.iter().find_map(|obj| obj.hit(p)),
            (None, None) => panic!("unexpect mode"),
        };
        hit.unwrap_or(self.background)
    }

    fn bounding_box(&self) -> Option<(Vec3f, Vec3f)> {
        match (&self.acceletor, &self.objects) {
            (Some(acc), _) => acc.bounding_box(),
            (None, Some(objs)) => match objs.len() {
                0 => None,
                _ => Some(objs.iter().fold(objs[0].bounding_box(), |(min, max), obj| {
                    let (obj_min, obj_max) = obj.bounding_box();
                    (vec3::minimum(min, obj_min), vec3::maximum(max, obj_max))
                })),
            },
            (None, None) => None,
        }
    }
}
