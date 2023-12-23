use crate::vec3::Vec3f;

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
