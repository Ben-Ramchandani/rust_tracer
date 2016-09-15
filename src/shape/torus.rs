use vec3::Vector3;
use shape::Shape;
use Ray;
use cubic::solve_quartic_smallest_positive_real;

pub struct Torus {
    pub radius: f64,
    pub tube_radius: f64,
}

// Torus in the x-y plane centerd on the origin.
// tube_radius < raduis.

impl Shape for Torus {
    fn intersect(&self, (b, a): Ray) -> Option<(f64)> {


        let a_dot_a = a.dot(a);

        let a_dot_b = a.dot(b);

        let radius2 = self.radius * self.radius;
        let minor_radius2 = self.tube_radius * self.tube_radius;
        let k = a_dot_a - minor_radius2 - radius2;
        let t1 = 4.0 * a_dot_b;
        let t2 = 2.0 * (2.0 * a_dot_b * a_dot_b + k + 2.0 * radius2 * b.z * b.z);
        let t3 = 4.0 * (k * a_dot_b + 2.0 * radius2 * a.z * b.z);
        let t4 = k * k + 4.0 * radius2 * (a.z * a.z - minor_radius2);

        let s = solve_quartic_smallest_positive_real(1.0, t1, t2, t3, t4);

        return s;
    }

    fn normal(&self, point: Vector3) -> Vector3 {
        let point_on_ring = Vector3::new(point.x, point.y, 0.0).normalize() * self.radius;
        return point - point_on_ring;
    }
}

impl Torus {
    fn normal(&self, point: Vector3) -> Vector3 {
        let point_on_ring = Vector3::new(point.x, point.y, 0.0).normalize() * self.radius;
        return point - point_on_ring;
    }
}
