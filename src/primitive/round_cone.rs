use crate::{
    sdf::{DynSDF, SDFHitInfo, SDF},
    solid_geometry::proj_p_to_line,
    vec3::{self, Vec3f},
};
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
pub struct RoundCone {
    a: Vec3f,
    b: Vec3f,
    ra: f32,
    rb: f32,
    bounding_box: (Vec3f, Vec3f),
}

impl RoundCone {
    pub fn new(a: Vec3f, b: Vec3f, ra: f32, rb: f32) -> RoundCone {
        let (min_a, min_b) = (a - ra, b - rb);
        let (max_a, max_b) = (a + ra, b + rb);
        let bounding_box = (vec3::minimum(min_a, min_b), vec3::maximum(max_a, max_b));
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
impl RoundCone {
    #[new]
    pub fn __new__(a: (f32, f32, f32), b: (f32, f32, f32), ra: f32, rb: f32) -> RoundCone {
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

impl SDF for RoundCone {
    fn distance(&self, p: Vec3f) -> f32 {
        sd_round_cone(p, self.a, self.b, self.ra, self.rb)
    }

    fn hit(&self, p: Vec3f) -> Option<SDFHitInfo> {
        let distance = SDF::distance(self, p);
        if distance <= 0.0 {
            Some(SDFHitInfo {
                distance,
                u: distance.clamp(0.0, 1.0),
                v: proj_p_to_line(p, self.a, self.b).clamp(0.0, 1.0),
                w: 0.0,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.bounding_box
    }
}

fn sd_round_cone(p: Vec3f, a: Vec3f, b: Vec3f, ra: f32, rb: f32) -> f32 {
    // sampling independent computations (only depend on shape)
    let ba = b - a;
    let l2 = vec3::dot(ba, ba);
    let rr = ra - rb;
    let a2 = l2 - rr * rr;
    let il2 = 1.0 / l2;

    // sampling dependant computations
    let pa = p - a;
    let y = vec3::dot(pa, ba);
    let z = y - l2;
    let x2 = (pa * l2 - ba * y).dot2();
    let y2 = y * y * l2;
    let z2 = z * z * l2;

    // single square root!
    let k = f32::signum(rr) * rr * rr * x2;
    if f32::signum(z) * a2 * z2 > k {
        f32::sqrt(x2 + z2) * il2 - rb
    } else if f32::signum(y) * a2 * y2 < k {
        f32::sqrt(x2 + y2) * il2 - ra
    } else {
        (f32::sqrt(x2 * a2 * il2) + y * rr) * il2 - ra
    }
}
