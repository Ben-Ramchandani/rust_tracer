use vec3::Vector3;
use super::Ray;
use shape::Shape;

const RES_W: i64 = 500;
const RES_H: i64 = 500;
const SCREEN_Z: f64 = -2.0;
const SCREEN_W: f64 = 2.0;
const SCREEN_H: f64 = 2.0;
const VIEW_Z: f64 = -5.0;


pub fn trace_nearest<'a>(ray: Ray, shapes: &'a Vec<&Shape>) -> Option<(&'a Shape, Vector3)> {

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
        let (dir, origin) = ray;
        let intersection_point = origin + dir * closest_distance;
        return (s, intersection_point);
    });
}

struct Screen {
    res_w: i64,
    res_h: i64,
    eye: Vector3,
    top_left: Vector3,
    increment_w: Vector3,
    increment_h: Vector3,
    curr_w: i64,
    curr_h: i64,
    exausted: bool,
}

// TODO unused
impl Screen {
    fn pixel_count(&self) -> i64 {
        return self.res_w * self.res_h;
    }
}

impl Iterator for Screen {
    type Item = Ray;
    fn next(&mut self) -> Option<Ray> {
        if self.exausted {
            return None;
        }
        let screen_point = self.top_left + self.increment_w * (self.curr_w as f64) +
                           self.increment_h * (self.curr_h as f64);
        let res = Some(((screen_point - self.eye).normalize(), self.eye));
        self.curr_w += 1;
        if self.curr_w == self.res_w {
            self.curr_h += 1;
            self.curr_w = 0;
            if self.curr_h == self.res_h {
                self.exausted = true;
            }
        }
        return res;
    }
}

fn get_screen() -> Screen {
    return Screen {
        res_w: RES_W,
        res_h: RES_H,
        eye: Vector3::new(0.0, 0.0, VIEW_Z),
        top_left: Vector3::new(-SCREEN_W / 2.0, SCREEN_H / 2.0, SCREEN_Z),
        increment_w: Vector3::new(SCREEN_W / RES_W as f64, 0.0, 0.0),
        increment_h: Vector3::new(0.0, -SCREEN_H / RES_H as f64, 0.0),
        curr_w: 0,
        curr_h: 0,
        exausted: false,
    };
}

use super::color::Color;
use super::pnm;
use super::shape::{ORIGIN, Sphere, Plane, Torus};
use std::io::stdout;

pub fn simple_trace() {
    let screen = get_screen();
    let s = Sphere {
        centre: ORIGIN,
        radius: 1.3,
    };
    let p = Plane {
        normal: Vector3::new(0.0, 1.0, 0.0),
        origin_distance: -5.0,
    };
    let t = Torus {
        radius: 1.0,
        tube_radius: 0.3,
    };
    let world: Vec<&Shape> = vec![&t];
    let pixels = screen.map(|ray| {
        if let Some(_) = trace_nearest(ray, &world) {
            return Color::from_rgb(255, 0, 0);
        } else {
            return Color::from_rgb(0, 0, 0);
        }
    });
    // pnm::write_console(pixels, RES_W);

    pnm::write_pnm(pixels, RES_W, RES_H, &mut stdout());
}
