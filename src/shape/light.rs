use vec3::Vector3;
use color::Color;

#[derive(Debug)]
pub struct Light {
    pub position: Vector3,
    pub color: Color,
}
