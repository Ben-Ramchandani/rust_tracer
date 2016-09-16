use super::color::Color;

use std::io::prelude::*;

pub fn write_console<I>(pixels: I, width: i64)
    where I: Iterator<Item = Color>
{
    let mut count: i64 = 0;
    let mut string = String::from("|");
    for color in pixels {
        if color.red > 0.0 {
            string = string + "X";
        } else {
            string = string + " ";
        }
        count += 1;
        if count == width {
            string = string + "|\n|";
            count = 0;
        }
    }
    print!("{}", string);
}

pub fn write_pnm<I>(pixels: I, width: i64, height: i64, file: &mut Write)
    where I: Iterator<Item = Color>
{
    let rgb = pixels.map(|p| p.to_rgb());
    let mut buffer: Vec<u8> = Vec::with_capacity((width * height * 3) as usize);
    for (r, g, b) in rgb {
        buffer.push(r);
        buffer.push(g);
        buffer.push(b);
    }

    file.write_all(format!("P6\n{} {}\n255\n", width, height).as_bytes()).unwrap();
    file.write_all(buffer.as_slice()).unwrap();
}
