use crate::AABB;

pub fn aabb_aabb(a: &AABB, b: &AABB) -> bool {
    for i in 0..2 {
        if a.max[i] < b.min[i] || a.min[i] > b.max[i] {
            return false;
        }
    }
    true
}
