#![allow(unused_imports)]

use super::vec3::Vector3;
use super::shape::{Sphere, Shape, Plane, ORIGIN};
use super::{trace, Ray};


#[test]
fn testy() {
    assert!(true);
}

#[test]
fn test_plane() {
    let v1 = Vector3 {x:1.0, y:0.0, z:0.0};
    let v2 = Vector3 {x:-1.0, y:0.0, z:0.0};
    let plane = Plane {normal: v1 , origin_distance: 1.0};
    let (t, n) = plane.intersect((v1, ORIGIN)).unwrap();
    assert_eq!(t, 1.0);
    assert_eq!(n, v2);
}

#[test]
fn test_trace_nearest() {
    let v = Vector3::new(1.0, 0.0, 0.0);
    let p = Plane {normal: v, origin_distance: 1.0};
    trace::trace_nearest((v, ORIGIN), &vec![&p as &Shape]).unwrap();    
}

#[test]
fn test_sphere() {
    let v1 = Vector3::new(-3.0, 0.0, 0.0);
    let v2 = Vector3::new(1.0, 0.0, 0.0);
    let s = Sphere {centre: ORIGIN, radius: 1.0};
    let r: Ray = (v2, v1);
    let d = s.intersect_without_normal(r).unwrap();
    assert_eq!(d, 2.0);
}

