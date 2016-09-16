
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn dot(self, v: Vector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    fn len_sq(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn cross(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn normalize(self) -> Vector3 {
        let len: f64 = self.len();
        if len == 0.0 {
            return Vector3::new(0.0, 0.0, 0.0);
        } else {
            return Vector3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            };
        }
    }

    pub fn rotate_inv(self, a: f64, b: f64) -> Vector3 {
        let cosa = a.cos();
        let cosb = b.cos();
        let sina = a.sin();
        let sinb = b.sin();
        return Vector3 {
            x: self.x * cosb + self.y * sina * sinb + self.z * cosa * sinb,
            y: 0.0 + self.y * cosa + self.z * (-sina),
            z: self.x * (-sinb) + self.y * cosb * sina + self.z * cosa * cosb,
        };
    }
}

impl ::std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ::std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ::std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ::std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, num: f64) -> Vector3 {
        Vector3 {
            x: self.x * num,
            y: self.y * num,
            z: self.z * num,
        }
    }
}

impl ::std::ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, num: f64) -> Vector3 {
        self * (1.0 / num)
    }
}
