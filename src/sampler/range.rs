use crate::{
    scene::{DynScene, Scene},
    vec3::Vec3f,
};
use ndarray::prelude::*;
use numpy::{IntoPyArray, PyArray4};
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
pub struct RangeSampler {
    min: Vec3f,
    max: Vec3f,
    stride: Vec3f,
}

impl RangeSampler {
    fn new(min: Vec3f, max: Vec3f, stride: Vec3f) -> Self {
        Self { min, max, stride }
    }

    fn sample(&self, scene: Arc<dyn Scene>) -> Array4<f32> {
        let samples = RangeSamples {
            cur: self.min,
            scene,

            min: self.min,
            max: self.max,
            stride: self.stride,
        };

        let (x, y, z) = self.samples_shape();
        let mut flat = Vec::with_capacity(x * y * z * 3);
        for v in samples {
            flat.push(v.x);
            flat.push(v.y);
            flat.push(v.z);
        }
        Array::from_shape_vec((x, y, z, 3), flat).unwrap()
    }

    fn samples_shape(&self) -> (usize, usize, usize) {
        let size = (self.max - self.min) / self.stride;
        (
            size.x.ceil() as usize,
            size.y.ceil() as usize,
            size.z.ceil() as usize,
        )
    }
}

#[pymethods]
impl RangeSampler {
    #[new]
    fn __new__(min: (f32, f32, f32), max: (f32, f32, f32), stride: (f32, f32, f32)) -> Self {
        Self::new(min.into(), max.into(), stride.into())
    }

    #[pyo3(name = "sample")]
    fn py_sample(&self, scene: DynScene) -> Py<PyArray4<f32>> {
        let samples = self.sample(scene.into());
        Python::with_gil(|py| samples.into_pyarray(py).to_owned())
    }
}

pub struct RangeSamples {
    cur: Vec3f,
    scene: Arc<dyn Scene>,

    min: Vec3f,
    max: Vec3f,
    stride: Vec3f,
}

impl Iterator for RangeSamples {
    type Item = Vec3f;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.z >= self.max.z {
            self.cur.z = self.min.z;
            self.cur.y += self.stride.y;
        }

        if self.cur.y >= self.max.y {
            self.cur.y = self.min.y;
            self.cur.x += self.stride.x;
        }

        if self.cur.x >= self.max.x {
            return None;
        }

        let p = self.cur;
        self.cur.z += self.stride.z;

        Some(self.scene.hit(p))
    }
}
