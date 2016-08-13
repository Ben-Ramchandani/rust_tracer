#![allow(dead_code)]
mod vec3;
mod color;
mod shape;
mod trace;
mod test;
mod pnm;
use vec3::Vector3;

pub type Direction = Vector3;
pub type Origin = Vector3;

pub type Ray = (Direction, Origin);

fn main() {
    trace::simple_trace();
}