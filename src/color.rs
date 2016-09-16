#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};
pub const RED: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};
pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};


impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        let result = Color {
            red: (r as f64) / 255.0,
            green: (g as f64) / 255.0,
            blue: (b as f64) / 255.0,
        };
        if result.red > 1.0 || result.green > 1.0 || result.blue > 1.0 {
            panic!("Color is invalid: {:?}", result);
        }
        return result;
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        ((self.red * 255.0) as u8, (self.blue * 255.0) as u8, (self.green * 255.0) as u8)
    }
}

impl ::std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl ::std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {

        let mut red: f64 = self.red + other.red;
        let mut green: f64 = self.green + other.green;
        let mut blue: f64 = self.blue + other.blue;
        if red > 1.0 {
            red = 1.0;
        }
        if green > 1.0 {
            green = 1.0;
        }
        if blue > 1.0 {
            blue = 1.0;
        }

        return Color {
            red: red,
            green: green,
            blue: blue,
        };
    }
}

impl ::std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        assert!(other >= 0.0);
        let mut red: f64 = self.red * other;
        let mut green: f64 = self.green * other;
        let mut blue: f64 = self.blue * other;
        if red > 1.0 {
            red = 1.0;
        }
        if green > 1.0 {
            green = 1.0;
        }
        if blue > 1.0 {
            blue = 1.0;
        }

        return Color {
            red: red,
            green: green,
            blue: blue,
        };
    }
}
