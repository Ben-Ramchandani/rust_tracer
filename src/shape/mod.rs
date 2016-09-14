use vec3::Vector3;
use color::Color;
use Ray;

pub mod plane;
pub use shape::plane::Plane;

pub mod sphere;
pub use shape::sphere::Sphere;

pub mod torus;
pub use shape::torus::Torus;

pub trait Shape {
    // Intersection point of ray with shape.
    // the return value, (R, N), is such that
    // Ray origin + R * Ray direction = intersect point.
    fn intersect(&self, ray: Ray) -> Option<(f64, Vector3)>;

    fn intersect_without_normal(&self, ray: Ray) -> Option<f64> {
        self.intersect(ray).map(|(s, _)| s)
    }
}

trait Drawable: Shape {
    fn get_colour(&self) -> Color;
}

pub const ORIGIN: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
pub const ANGLE_EPSILON: f64 = 0.000001;
pub const INTERSECT_EPSILON: f64 = 0.000001;
