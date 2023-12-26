use crate::{
    sdf::{DynSDF, SDF},
    solid_geometry::proj_vector_on_plane,
    vec3::{self, Vec3f},
};
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
pub struct FrustumCone {
    a: Vec3f,
    b: Vec3f,
    ra: f32,
    rb: f32,
    bounding_box: (Vec3f, Vec3f),
}

impl FrustumCone {
    pub fn new(a: Vec3f, b: Vec3f, ra: f32, rb: f32) -> FrustumCone {
        let bounding_box = aabb_frustum_cone(a, b, ra, rb);
        Self {
            a,
            b,
            ra,
            rb,
            bounding_box,
        }
    }
}

#[pymethods]
impl FrustumCone {
    #[new]
    pub fn __new__(a: (f32, f32, f32), b: (f32, f32, f32), ra: f32, rb: f32) -> FrustumCone {
        Self::new(a.into(), b.into(), ra, rb)
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

impl SDF for FrustumCone {
    fn distance(&self, p: Vec3f) -> f32 {
        sd_frustum_cone(p, self.a, self.b, self.ra, self.rb)
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.bounding_box
    }
}

fn aabb_frustum_cone(a: Vec3f, b: Vec3f, ra: f32, rb: f32) -> (Vec3f, Vec3f) {
    let up = a - b;

    let vx = proj_vector_on_plane(Vec3f::new(1.0, 0.0, 0.0), up);
    let vy = proj_vector_on_plane(Vec3f::new(0.0, 1.0, 0.0), up);
    let vz = proj_vector_on_plane(Vec3f::new(0.0, 0.0, 1.0), up);

    let a1 = a - vx * ra - vy * ra - vz * ra;
    let a2 = a + vx * ra + vy * ra + vz * ra;
    let b1 = b - vx * rb - vy * rb - vz * rb;
    let b2 = b + vx * rb + vy * rb + vz * rb;

    (vec3::minimum(a1, b1), vec3::maximum(a2, b2))
}

fn sd_frustum_cone(p: Vec3f, a: Vec3f, b: Vec3f, ra: f32, rb: f32) -> f32 {
    let rba = rb - ra;
    let baba = vec3::dot(b - a, b - a);
    let papa = vec3::dot(p - a, p - a);
    let paba = vec3::dot(p - a, b - a) / baba;
    let x = (papa - paba * paba * baba).sqrt();
    let cax = f32::max(0.0, x - (if paba < 0.5 { ra } else { rb }));
    let cay = (paba - 0.5).abs() - 0.5;
    let k = rba * rba + baba;
    let f = ((rba * (x - ra) + paba * baba) / k).clamp(0.0, 1.0);
    let cbx = x - ra - f * rba;
    let cby = paba - f;
    let s = if cbx < 0.0 && cay < 0.0 { -1.0 } else { 1.0 };
    return s * (f32::min(cax * cax + cay * cay * baba, cbx * cbx + cby * cby * baba)).sqrt();
}
