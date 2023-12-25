use crate::vec3::{self, Vec3f};

pub fn proj_p_to_line(p: Vec3f, a: Vec3f, b: Vec3f) -> f32 {
    let ap = p - a;
    let ab = b - a;
    let ao = a + ab * (vec3::dot(ap, ab) / vec3::dot(ab, ab)); // proj P to AB in O
    vec3::dot(ao, ab).signum() * ao.norm() / ab.norm() // O = A + k * (AB)
}

pub fn proj_vector_on_plane(vec: Vec3f, plane_normal_vec: Vec3f) -> Vec3f {
    let v = vec;
    let n = plane_normal_vec.normalize();

    // Project v onto n
    let projection_on_n = n * vec3::dot(v, n);

    // Project v onto the plane
    let projection_on_plane = v - projection_on_n;

    projection_on_plane
}
