use crate::object::Object;
use crate::vec3::{self, Vec3f};
use std::{cmp::Ordering, sync::Arc};

pub trait Accelerator: Send + Sync {
    fn hit(&self, p: Vec3f) -> Option<Vec3f>;
    fn bounding_box(&self) -> Option<(Vec3f, Vec3f)>;
}

/**
 * Bounding Volume Hierarchy
 */

pub struct BVH {
    root: Option<BVHNode>,
    objects: Vec<Arc<dyn Object>>,
}

impl BVH {
    pub fn new(objects: Vec<Arc<dyn Object>>) -> Self {
        Self {
            root: match objects.len() {
                0 => None,
                _ => Some(BVHNode::new(&objects)),
            },
            objects,
        }
    }
}

impl Accelerator for BVH {
    fn hit(&self, p: Vec3f) -> Option<Vec3f> {
        match &self.root {
            Some(r) => r.hit(p).find_map(|idx| self.objects[idx].hit(p)),
            None => None,
        }
    }

    fn bounding_box(&self) -> Option<(Vec3f, Vec3f)> {
        match &self.root {
            Some(r) => Some(r.bounding_box),
            None => None,
        }
    }
}

struct BVHNode {
    children: Option<(Box<BVHNode>, Box<BVHNode>)>,
    bounding_box: (Vec3f, Vec3f),
    n: usize, // number of objects, 1 when leaf
    index: usize,
}

impl BVHNode {
    fn new(objects: &Vec<Arc<dyn Object>>) -> BVHNode {
        let mut objects = objects
            .iter()
            .enumerate()
            .map(|(i, x)| BVHBuildInfo::new(i, x))
            .collect::<Vec<_>>();

        Self::new_(&mut objects)
    }

    fn new_(objects: &mut [BVHBuildInfo]) -> BVHNode {
        if objects.len() == 1 {
            return BVHNode {
                children: None,
                bounding_box: objects[0].bounding_box,
                n: 1,
                index: objects[0].index,
            };
        }

        let bounding_box = objects
            .iter()
            .map(|a| a.bounding_box)
            .reduce(|(min, max), (emin, emax)| (vec3::minimum(min, emin), vec3::maximum(max, emax)))
            .unwrap();

        let shape = bounding_box.1 - bounding_box.0;
        let cmp = if shape.x >= shape.y && shape.x >= shape.z {
            BVHBuildInfo::cmp_x
        } else if shape.y >= shape.z {
            BVHBuildInfo::cmp_y
        } else {
            BVHBuildInfo::cmp_z
        };

        objects.sort_unstable_by(cmp);
        let (left, right) = objects.split_at_mut(objects.len() / 2);
        BVHNode {
            children: Some((Box::new(Self::new_(left)), Box::new(Self::new_(right)))),
            bounding_box,
            n: objects.len(),
            index: objects[0].index,
        }
    }

    fn hit(&self, p: Vec3f) -> BVHHitIter {
        BVHHitIter { s: vec![&self], p }
    }

    fn isin(&self, p: Vec3f) -> bool {
        let (min, max) = self.bounding_box;
        p.x >= min.x && p.y >= min.y && p.z >= min.z && p.x <= max.x && p.y <= max.y && p.z <= max.z
    }
}

struct BVHHitIter<'a> {
    s: Vec<&'a BVHNode>,
    p: Vec3f,
}

impl<'a> Iterator for BVHHitIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.s.pop() {
            match node.isin(self.p) {
                true if node.n == 1 => return Some(node.index),
                true => {
                    if let Some((left, right)) = &node.children {
                        self.s.push(&left);
                        self.s.push(&right);
                    };
                }
                _ => (),
            }
        }

        None
    }
}

struct BVHBuildInfo {
    index: usize,
    center: Vec3f,
    bounding_box: (Vec3f, Vec3f),
}

impl BVHBuildInfo {
    fn new(index: usize, object: &Arc<dyn Object>) -> BVHBuildInfo {
        let (min, max) = object.bounding_box();
        BVHBuildInfo {
            index,
            center: (min + max) / 2.0,
            bounding_box: (min, max),
        }
    }

    fn cmp_x(a: &Self, b: &Self) -> Ordering {
        a.center.x.total_cmp(&b.center.x)
    }

    fn cmp_y(a: &Self, b: &Self) -> Ordering {
        a.center.y.total_cmp(&b.center.y)
    }

    fn cmp_z(a: &Self, b: &Self) -> Ordering {
        a.center.z.total_cmp(&b.center.z)
    }
}
