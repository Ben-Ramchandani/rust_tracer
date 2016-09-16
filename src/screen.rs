use vec3::Vector3;
use super::Ray;

pub const RES_W: i64 = 1920;
pub const RES_H: i64 = 1080;
pub const SCREEN_Z: f64 = -2.0;
pub const SCREEN_W: f64 = 3.55555555555;
pub const SCREEN_H: f64 = 2.0;
pub const VIEW_Z: f64 = -5.0;

pub struct Screen {
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

pub fn get_screen() -> Screen {
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
