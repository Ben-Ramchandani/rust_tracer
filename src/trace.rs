use vec3::Vector3;
use super::Ray;
use shape::Shape;


pub fn trace_nearest<'a>(ray: Ray, shapes: &'a Vec<&Shape>)
        -> Option<(&'a Shape, Vector3)> {

    let mut closest_shape: Option<&Shape> = None;
    let mut closest_distance: f64 = 1.0/0.0;
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

fn gen_screen(i: i32, j: i32) -> Vector3 {
    let top_left = Vector3::new(-1.0, 1.0, -2.0);
    let width = Vector3::new(2.0, 0.0, 0.0);
    let height = Vector3::new(0.0, -2.0, 0.0);
    let (resolution_w, resolution_h): (f64, f64) = (5.0, 5.0);
    
    let increment_w = width / resolution_w;
    let increment_h = height / resolution_h;
    return top_left + increment_w * (j as f64) + increment_h * (i as f64);
}

pub fn print_screen() {
    for i in 0..6 {
        for j in 0..6 {
            println!("{:?}", gen_screen(i, j));
        }
        println!("");
    }
}

use super::color::Color;
use super::pnm;
use super::shape::{ORIGIN, Sphere, Plane};

pub fn simple_trace() {
    let s = Sphere {centre: ORIGIN, radius: 1.3};
    let p = Plane {normal: Vector3::new(0.0, 1.0, 0.0), origin_distance: -5.0};
    let world: Vec<&Shape> = vec![&p, &s];
    let mut pixels: Vec<Vec<Color>> = vec![];
    let eye = Vector3::new(0.0, 0.0, -5.0);
    for i in 0..6 {
        let mut row: Vec<Color> = vec![];
        for j in 0..6 {
            let ray = ((gen_screen(i, j) - eye).normalize(), eye);
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


