use vec3::Vector3;
use super::Ray;
use shape::Shape;
use shape::Light;
use screen;

struct World<'a> {
    shapes: Vec<&'a Shape>,
    lights: Vec<&'a Light>,
}

impl<'a> World<'a> {
    pub fn trace_nearest(&self, ray: Ray) -> Option<(&Shape, f64)> {

        let mut closest_shape: Option<&Shape> = None;
        let mut closest_distance: f64 = 1.0 / 0.0;
        let mut iter = self.shapes.iter();

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

    fn trace_collision(&self, ray: Ray) -> bool {

        for shape in self.shapes.iter() {
            if let Some(_) = shape.intersect(ray) {
                return false;
            }
        }
        return true;
    }

    fn trace_lights(&self,
                    point: Vector3,
                    normal: Vector3)
                    -> Box<Iterator<Item = &'a &Light> + 'a> {
        return Box::new(self.lights.iter().filter(move |l| {
            let ray_to_light = l.position - point;
            if ray_to_light.dot(normal) < 0.0 {
                return false;
            } else {
                return !self.trace_collision((ray_to_light.normalize(), point));
            }
        }));
    }
}

use super::color::Color;
use super::pnm;
use super::shape::{ORIGIN, Sphere, Torus};
use std::io::stdout;

pub fn simple_trace() {
    let screen = screen::get_screen();
    let s: &Sphere = &Sphere {
        centre: ORIGIN,
        radius: 0.5,
    };
    let t = &Torus {
        center: Vector3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        tube_radius: 0.4,
        rotx: 0.5,
        roty: 0.5,
    };
    let the_shapes: Vec<&Shape> = vec![t];
    let world = World {
        shapes: the_shapes,
        lights: vec![],
    };
    let pixels = screen.map(|ray| {
        if let Some((_, t)) = world.trace_nearest(ray) {
            let d: i32 = 255 - (t * 40.0) as i32;
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
