use super::color::Color;

pub fn write_console(arr: &Vec<Vec<Color>>) {
    for row in arr.iter() {
        print!("|");
        for col in row.iter() {
            if col.red > 0.0 {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
}
