use vec3::Vector3;
use super::Shape;
use Ray;
use shape::INTERSECT_EPSILON;

pub struct Sphere {
    pub centre: Vector3,
    pub radius: f64, // color: Color
}

impl Sphere {
    fn normal(&self, point: Vector3) -> Vector3 {
        (point - self.centre).normalize()
    }
}

impl Shape for Sphere {
    fn intersect(&self, (ray_dir, ray_origin): Ray) -> Option<f64> {
        let c = ray_origin.dot(ray_origin) + self.centre.dot(self.centre) -
                2.0 * ray_origin.dot(self.centre) - self.radius * self.radius;
        let b = ray_dir.dot(ray_origin - self.centre);
        // Assume ray_dir is normalized.
        let dsq = b * b - c;
        if dsq <= 0.0 {
            return None::<f64>;
        }
        let d = dsq.sqrt();
        let s1 = -b + d;
        let s2 = -b - d;
        if s1 < s2 {
            if s1 > INTERSECT_EPSILON {
                return Some(s1);
            }
        }
        if s2 > INTERSECT_EPSILON {
            return Some(s2);
        } else {
            if s1 > INTERSECT_EPSILON {
                return Some(s1);
            } else {
                return None::<f64>;
            }
        }
    }

    fn normal(&self, point: Vector3) -> Vector3 {
        return (point - self.centre).normalize();
    }
}
