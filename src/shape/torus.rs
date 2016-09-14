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
//
// 2*R*sqrt(a^2 + 2*t*a*d + t^2*d^2 + b^2 + 2*t*b*e + t^2*e^2) =
// R^2 + a^2 + 2*t*a*d + b^2 +
// 2*R*sqrt(x_E^2 + 2*t*x_E*x_D + t^2*x_D^2 + y_E^2 + 2*t*y_E*y_D + t^2*y_D^2)
// = R^2 + x_E^2+2*t*x_E*x_D + t^2*x_D^2 + y_E^2 + 2*t*y_E*y_D + t^2*y_D^2
//  + z_E^2 + 2*t*z_E*z_D + t^2*z_D^2 - r^2

// R^2 + a^2+2*t*a*b + t^2*b^2 + d^2 + 2*t*d*c + t^2*c^2 + f^2 + 2*t*f*g + t^2*g^2 - r^2

impl Shape for Torus {
    fn intersect_without_normal(&self, (b, a): Ray) -> Option<(f64)> {
        println!("a: {:?}", a);
        println!("b: {:?}", b);
        let a_dot_a = a.dot(a);
        println!("a.a: {:?}", a_dot_a);
        let a_dot_b = a.dot(b);
        println!("a.b: {:?}", a_dot_b);
        let radius2 = self.radius * self.radius;
        let minor_radius2 = self.tube_radius * self.tube_radius;
        let k = a_dot_a - minor_radius2 - radius2;
        let t1 = 4.0 * a_dot_b;
        let t2 = 2.0 * (2.0 * a_dot_b * a_dot_b + k + 2.0 * radius2 * b.z * b.z);
        let t3 = 4.0 * (k * a_dot_b + 2.0 * radius2 * a.z * b.z);
        let t4 = k * k + 4.0 * radius2 * (a.z * a.z - minor_radius2);
        println!("{:?}", (t1, t2, t3, t4));
        let s = solve_quartic_smallest_positive_real(1.0, t1, t2, t3, t4);
        println!("{:?}", s);
        return s;
    }

    fn intersect_with_normal(&self, (dir, origin): Ray) -> Option<(f64, Vector3)> {
        return self.intersect_without_normal((dir, origin))
            .map(|s| (s, self.normal(origin + (dir * s))));
    }
}

impl Torus {
    fn normal(&self, point: Vector3) -> Vector3 {
        let point_on_ring = Vector3::new(point.x, point.y, 0.0).normalize() * self.radius;
        return (point - point_on_ring).normalize();
    }
}
