use vec3::Vector3;
use super::Ray;
use shape::Shape;
use shape::Light;
use screen;
use super::color;
use super::color::Color;
use super::pnm;
use super::shape::{ORIGIN, Sphere, Torus};
use std::io::stdout;

struct World<'a> {
    shapes: Vec<&'a Shape>,
    lights: Vec<&'a Light>,
}

impl<'a> World<'a> {
    pub fn trace_nearest(&self, ray: Ray) -> Option<(&Shape, f64, Vector3)> {

        let mut closest: Option<(&Shape, f64, Vector3)> = None;
        let mut closest_distance: f64 = 1.0 / 0.0;
        let mut iter = self.shapes.iter();

        while let Some(shape) = iter.next() {
            if let Some((distance, normal)) = shape.intersect_with_normal(ray) {
                if distance < closest_distance {
                    closest_distance = distance;
                    closest = Some((*shape, distance, normal));
                }
            }
        }

        return closest;
    }

    fn trace_collision(&self, ray: Ray, max_distance: f64) -> bool {

        for shape in self.shapes.iter() {
            if let Some(s) = shape.intersect(ray) {
                if s < max_distance {
                    return false;
                }
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
                return self.trace_collision((ray_to_light.normalize(), point), ray_to_light.len());
            }
        }));
    }

    fn trace(&self, (dir, origin): Ray) -> Color {
        if let Some((shape, t, normal)) = self.trace_nearest((dir, origin)) {
            let point = origin + dir * t;
            let lights = self.trace_lights(point, normal);
            let mut color = color::BLACK;
            for light in lights {
                color = color + diffuse(shape, light, normal, point);
            }
            color = color + ambient(shape);
            return color;
        } else {
            return color::BLACK;
        }
    }
}

pub const K_DIFFUSE: f64 = 5.0;
pub const K_AMBIENT: f64 = 0.1;


fn diffuse(shape: &Shape, light: &Light, normal: Vector3, point: Vector3) -> Color {
    let ray_to_light = light.position - point;
    let distance = ray_to_light.len();
    let cosine = normal.dot(ray_to_light.normalize());
    if cosine.is_nan() {
        panic!("Cosine is NaN");
    }
    let factor = K_DIFFUSE * normal.dot(ray_to_light.normalize()) * 1.0 / (distance * distance);
    return (shape.color_diffuse() * light.color) * factor;
}

fn ambient(shape: &Shape) -> Color {
    return shape.color_ambient() * K_AMBIENT;
}

pub fn simple_trace() {
    let screen = screen::get_screen();
    // let s: &Sphere = &Sphere {
    //     centre: ORIGIN,
    //     radius: 0.5,
    // };
    let t = &Torus {
        center: Vector3::new(0.0, 0.0, 0.0),
        radius: 0.8,
        tube_radius: 0.25,
        rotx: 0.0,
        roty: 0.0,
    };
    let t2 = &Torus {
        center: Vector3::new(0.8, 0.0, 0.0),
        radius: 0.8,
        tube_radius: 0.25,
        rotx: 3.1415926 / 2.0,
        roty: 0.0,
    };
    let the_shapes: Vec<&Shape> = vec![t, t2];
    let l: &Light = &Light {
        position: Vector3::new(-1.5, 1.0, -3.0),
        color: color::RED,
    };
    let world = World {
        shapes: the_shapes,
        lights: vec![l],
    };
    let pixels = screen.map(|ray| {
        return world.trace(ray);
    });
    // pnm::write_console(pixels, screen::RES_W);

    pnm::write_pnm(pixels, screen::RES_W, screen::RES_H, &mut stdout());
}
