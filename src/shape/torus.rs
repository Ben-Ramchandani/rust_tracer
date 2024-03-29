use vec3::Vector3;
use shape::{Shape, INTERSECT_EPSILON};
use Ray;
use cubic::solve_quartic_smallest_positive_real;

pub struct Torus {
    pub radius: f64,
    pub tube_radius: f64,
    pub center: Vector3,
    pub rotx: f64,
    pub roty: f64,
}

// Torus in the x-y plane centerd on the origin.
// tube_radius < raduis.

impl Torus {
    fn intersect_origin(&self, (b, a): Ray) -> Option<(f64)> {
        let a_dot_a = a.dot(a);
        let a_dot_b = a.dot(b);

        let radius2 = self.radius * self.radius;
        let minor_radius2 = self.tube_radius * self.tube_radius;
        let k = a_dot_a - minor_radius2 - radius2;
        let t1 = 4.0 * a_dot_b;
        let t2 = 2.0 * (2.0 * a_dot_b * a_dot_b + k + 2.0 * radius2 * b.z * b.z);
        let t3 = 4.0 * (k * a_dot_b + 2.0 * radius2 * a.z * b.z);
        let t4 = k * k + 4.0 * radius2 * (a.z * a.z - minor_radius2);

        let s = solve_quartic_smallest_positive_real(1.0, t1, t2, t3, t4, INTERSECT_EPSILON);

        return s;
    }

    fn normal_origin(&self, point: Vector3) -> Vector3 {
        let point_on_ring = Vector3::new(point.x, point.y, 0.0).normalize() * self.radius;
        return (point - point_on_ring).normalize();
    }
}

impl Shape for Torus {
    fn intersect(&self, (b, a): Ray) -> Option<(f64)> {
        return self.intersect_origin((b.rotate_inv(self.rotx, self.roty),
                                      (a - self.center).rotate_inv(self.rotx, self.roty)));
    }

    fn normal(&self, _: Vector3) -> Vector3 {
        unimplemented!();
    }

    fn intersect_with_normal(&self, ray: Ray) -> Option<(f64, Vector3)> {
        let (dir, origin): Ray = ray;
        let moved_dir = dir.rotate_inv(self.rotx, self.roty);
        let moved_origin = (origin - self.center).rotate_inv(self.rotx, self.roty);

        return self.intersect_origin((moved_dir, moved_origin))
            .map(|s| {
                (s, self.normal_origin(moved_origin + (moved_dir * s)).rotate(self.rotx, self.roty))
            });
    }
}
