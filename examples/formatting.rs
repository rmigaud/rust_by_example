// cargo run --example formatting

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        // `write!` is like `format!`, but it will write the formatted string to a buffer
        write!(
            f,
            "{}: {:.3}°{}, {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    //! Display Format Example: "RGB (128, 255, 90) 0x80FF5A"
    //! The formula for calculating a color in the RGB color space is RGB = R * 65536 + G * 256 + B
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let rgb: u32 = self.red as u32 * 65536 + self.green as u32 * 256 + self.blue as u32;

        write!(
            f,
            "RGB ({}, {}, {}) 0x{:06X}",
            self.red, self.green, self.blue, rgb
        )
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.34778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -125.1,
        },
    ] {
        println!("{city}");
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("{color}");
    }
}

// cargo test --example formatting
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn city_display_test() {
        // Format the display as follows:
        //
        // RGB (128, 255, 90) 0x80FF5A
        // RGB (0, 3, 254) 0x0003FE
        // RGB (0, 0, 0) 0x000000
        let black = Color {
            red: 0,
            green: 0,
            blue: 0,
        };
        let black_fmt = "RGB (0, 0, 0) 0x000000";
        let blue_fmt = "RGB (0, 3, 254) 0x0003FE";
        let green_fmt = "RGB (128, 255, 90) 0x80FF5A";

        assert_eq!(black_fmt, format!("{black}"));
        assert_eq!(blue_fmt, format!("{blue_fmt}"));
        assert_eq!(green_fmt, format!("{green_fmt}"));
    }
}
