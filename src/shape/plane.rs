use vec3::Vector3;
use super::Shape;
use Ray;
use shape::{ANGLE_EPSILON, INTERSECT_EPSILON};

pub struct Plane {
    // Normal must point away from origin such that `origin_distance >= 0.0`.
    pub normal: Vector3,
    pub origin_distance: f64, // color: Color
}

impl Shape for Plane {
    fn intersect(&self, (ray_dir, ray_origin): Ray) -> Option<(f64, Vector3)> {
        // Here normal is in the same hemisphere as the ray direction.
        let intersect_cosine: f64 = ray_dir.dot(self.normal);

        if intersect_cosine < ANGLE_EPSILON && intersect_cosine > -ANGLE_EPSILON {
            // Ray is parallel to plane
            return None::<(f64, Vector3)>;
        }

        let s: f64 = self.origin_distance - ray_origin.dot(self.normal);
        let t: f64 = s / intersect_cosine;

        if t < INTERSECT_EPSILON {
            // Ray starts after plane.
            return None::<(f64, Vector3)>;
        } else {
            if intersect_cosine > 0.0 {
                return Some((t, -self.normal));
            } else {
                return Some((t, self.normal));
            }
        }
    }
}
