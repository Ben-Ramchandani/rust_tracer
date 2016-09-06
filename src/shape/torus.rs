use vec3::Vector3;
use shape::Shape;
use Ray;
use shape::INTERSECT_EPSILON;

pub struct torus {
    pub radius: f64;
    pub tube_radius: f64;
    pub centre: Vector3;
    pub normal: Vector3;
}

