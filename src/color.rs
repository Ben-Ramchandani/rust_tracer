#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        let result = Color {
            red: (r as f64) / 255.0,
            green: (g as f64) / 255.0,
            blue: (b as f64) / 255.0
        };
        if result.red > 1.0 || result.green > 1.0 || result.blue > 1.0 {
            panic!("Color is invalid: {:?}", result);
        }
        return result;
    }
    
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        ((self.red * 255.0) as u8,
        (self.blue * 255.0) as u8,
        (self.green * 255.0) as u8)
    }
}

impl ::std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue
        }
    }
}
