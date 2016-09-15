use super::Ray;
use shape::Shape;
use screen;

pub fn trace_nearest<'a>(ray: Ray, shapes: &'a Vec<&Shape>) -> Option<(&'a Shape, f64)> {

    let mut closest_shape: Option<&Shape> = None;
    let mut closest_distance: f64 = 1.0 / 0.0;
    let mut iter = shapes.iter();

    while let Some(shape) = iter.next() {
        if let Some((distance, _)) = shape.intersect_with_normal(ray) {
            if distance < closest_distance {
                closest_distance = distance;
                closest_shape = Some(*shape);
            }
        }
    }

    return closest_shape.map(|s| {
        return (s, closest_distance);
    });
}

use super::color::Color;
use super::pnm;
use super::shape::{ORIGIN, Sphere, Torus};
use std::io::stdout;

pub fn simple_trace() {
    let screen = screen::get_screen();
    let s = Sphere {
        centre: ORIGIN,
        radius: 0.5,
    };
    let t = Torus {
        radius: 1.0,
        tube_radius: 0.3,
    };
    let world: Vec<&Shape> = vec![&t, &s];
    let pixels = screen.map(|ray| {
        if let Some((_, t)) = trace_nearest(ray, &world) {
            let d: i32 = 300 - (t * 50.0) as i32;
            if d < 0 || d > 255 {
                panic!();
            }
            return Color::from_rgb(d as u8, 0, 0);
        } else {
            return Color::from_rgb(0, 0, 0);
        }
    });
    // pnm::write_console(pixels, RES_W);

    pnm::write_pnm(pixels, screen::RES_W, screen::RES_H, &mut stdout());
}
