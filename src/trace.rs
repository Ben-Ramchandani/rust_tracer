use vec3::Vector3;
use super::Ray;
use shape::Shape;

const RES_W: i32 = 40;
const RES_H: i32 = 20;
const SCREEN_Z: f64 = -2.0;
const SCREEN_W: f64 = 2.0;
const SCREEN_H: f64 = 2.0;
const VIEW_Z: f64 = -5.0;


pub fn trace_nearest<'a>(ray: Ray, shapes: &'a Vec<&Shape>) -> Option<(&'a Shape, Vector3)> {

    let mut closest_shape: Option<&Shape> = None;
    let mut closest_distance: f64 = 1.0 / 0.0;
    let mut iter = shapes.iter();

    while let Some(shape) = iter.next() {
        if let Some((distance, _)) = shape.intersect(ray) {
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

// This is a terrible name
fn get_view_ray_fn() -> Box<Fn(i32, i32) -> Ray> {
    let width = Vector3::new(SCREEN_W, 0.0, 0.0);
    let height = Vector3::new(0.0, -SCREEN_H, 0.0);
    let resolution_w = RES_W as f64;
    let resolution_h = RES_H as f64;
    let eye = Vector3::new(0.0, 0.0, VIEW_Z);
    {
        let top_left = Vector3::new(-SCREEN_W / 2.0, SCREEN_H / 2.0, SCREEN_Z);
        let increment_w = width / resolution_w;
        let increment_h = height / resolution_h;
        return Box::new(move |i: i32, j: i32| {
            let screen_point = top_left + increment_w * (j as f64) + increment_h * (i as f64);
            return ((screen_point - eye).normalize(), eye);
        });
    }
}

use super::color::Color;
use super::pnm;
use super::shape::{ORIGIN, Sphere, Plane};

pub fn simple_trace() {
    let s = Sphere {
        centre: ORIGIN,
        radius: 1.3,
    };
    let p = Plane {
        normal: Vector3::new(0.0, 1.0, 0.0),
        origin_distance: -5.0,
    };
    let world: Vec<&Shape> = vec![&p, &s];
    let mut pixels: Vec<Vec<Color>> = vec![];
    let gen_screen: Box<Fn(i32, i32) -> Ray> = get_view_ray_fn();
    for i in 0..(RES_H + 1) {
        let mut row: Vec<Color> = vec![];
        for j in 0..(RES_W + 1) {
            let ray = gen_screen(i, j);
            if let Some(_) = trace_nearest(ray, &world) {
                row.push(Color::from_rgb(255, 0, 0));
            } else {
                row.push(Color::from_rgb(0, 0, 0));
            }
        }
        pixels.push(row);
    }
    pnm::write_pnm(&pixels);
}
