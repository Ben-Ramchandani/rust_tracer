use super::color::Color;

pub fn write_console<I>(pixels: I, width: i64)
    where I: Iterator<Item = Color>
{
    println!("hi");
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
    print!("{:}", string);
}
