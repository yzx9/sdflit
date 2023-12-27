use crate::{
    scene::{DynScene, Scene},
    vec3::Vec3f,
};
use ndarray::prelude::*;
use numpy::{IntoPyArray, PyArray2};
use pyo3::prelude::*;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};
use std::sync::Arc;

/**
 * Distribution Sampler
 */

pub struct DistributionSampler<D>
where
    D: Distribution<f32>,
{
    dist: (D, D, D),
}

impl<D> DistributionSampler<D>
where
    D: Distribution<f32> + Copy,
{
    fn sample(&self, scene: Arc<dyn Scene>, count: usize) -> Array2<f32> {
        let samples = DistributionSamples {
            scene,
            count,
            rng: rand::thread_rng(),
            dist: self.dist,
        };

        let mut flat = Vec::with_capacity(count * 3);
        for v in samples {
            flat.push(v.x);
            flat.push(v.y);
            flat.push(v.z);
        }
        Array::from_shape_vec((count, 3), flat).unwrap()
    }
}

struct DistributionSamples<D>
where
    D: Distribution<f32>,
{
    scene: Arc<dyn Scene>,
    count: usize,
    rng: ThreadRng,
    dist: (D, D, D),
}

impl<D> Iterator for DistributionSamples<D>
where
    D: Distribution<f32>,
{
    type Item = Vec3f;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count <= 0 {
            return None;
        }
        self.count = self.count - 1;

        let x = self.dist.0.sample(&mut self.rng);
        let y = self.dist.1.sample(&mut self.rng);
        let z = self.dist.2.sample(&mut self.rng);
        let p = Vec3f::new(x, y, z);
        Some(self.scene.hit(p))
    }
}

/**
 * Uniform Sampler
 */

#[pyclass]
pub struct UniformSampler(DistributionSampler<Uniform<f32>>);

impl UniformSampler {
    fn new(min: Vec3f, max: Vec3f) -> Self {
        Self(DistributionSampler {
            dist: (
                Uniform::from(min.x..max.x),
                Uniform::from(min.y..max.y),
                Uniform::from(min.z..max.z),
            ),
        })
    }
}

#[pymethods]
impl UniformSampler {
    #[new]
    fn __new__(min: (f32, f32, f32), max: (f32, f32, f32)) -> Self {
        Self::new(min.into(), max.into())
    }

    fn sample(&mut self, scene: DynScene, count: usize) -> Py<PyArray2<f32>> {
        let samples = self.0.sample(scene.into(), count);
        Python::with_gil(|py| samples.into_pyarray(py).to_owned())
    }
}
