use std::env::args;
use std::process::exit;
use colored::Colorize;
use image::{Rgb, RgbImage};

const AMOGUS_WIDTH: u32 = 4;
const AMOGUS_HEIGHT: u32 = 5;

struct Position {
    pub x: u32,
    pub y: u32,
}

fn main() {

    let args: Vec<String> = args().collect();
    let argslen = args.len();
    if argslen < 3 || argslen > 5 {
        println!("{}", "Invalid arguments. Usage: ./amongusify <input file> <output file> [sensibility] [altoffset]".red());
        exit(1);
    }

    let sensibility = if argslen == 4 {
        args[3].parse::<u8>().expect("Sensibility must be a number between 0 and 255")
    } else {
        255
    };

    let altoffset = if argslen == 5 {
        args[4].parse::<u8>().expect("Alternative color offset must be a number between 0 and 127")
    } else {
        5
    };
    if altoffset >= 128 {
        println!("{}", "Alternative color offset must be a number between 0 and 127".red());
        exit(1);
    }

    let mut image = image::open(&args[1]).expect(format!("Couldn't open image {}", args[0]).as_str()).into_rgb8();

    let width = image.width();
    let height = image.height();

    println!("Amongussifying {} ({}x{}px) to {} with {} sensibility...", args[1], width, height, args[2], sensibility);

    let mut x = 0;
    let mut y = 0;

    while x < width - AMOGUS_WIDTH {
        'a: while y < height - AMOGUS_HEIGHT {

            let pixels = get_pixels_in_rectangle(x, y, AMOGUS_WIDTH, AMOGUS_HEIGHT);

            let mut min: Rgb<u8> = image.get_pixel(pixels[0].x, pixels[0].y).clone();
            let mut max: Rgb<u8> = image.get_pixel(pixels[0].x, pixels[0].y).clone();
            let mut sum: [u32; 3] = [0, 0, 0];

            for pos in pixels {
                let color = image.get_pixel(pos.x, pos.y);
                for channel in 0..3 {
                    let c = color.0[channel];
                    sum[channel] += c as u32;

                    if c < min.0[channel] {
                        min.0[channel] = c;
                    }

                    if c > max.0[channel] {
                        max.0[channel] = c;
                    }
                }
            }

            let mut avg: [u8; 3] = [0, 0, 0];
            for channel in 0..3 {
                avg[channel] = (sum[channel] / (AMOGUS_WIDTH * AMOGUS_HEIGHT)) as u8;
                if max.0[channel] - min.0[channel] > sensibility {
                    println!("Skipped ({}, {})", x, y);
                    y+=AMOGUS_HEIGHT;
                    continue 'a;
                }
            }

            draw_amongus(&mut image, x, y, Rgb(avg), get_alt_color(Rgb(avg), altoffset));

            y+=AMOGUS_HEIGHT;
        }
        y = 0;
        x+=AMOGUS_WIDTH;
    }

    image.save(&args[2]).expect("Couldn't save image");

    println!("{} You may now open {} to see the result", "Done!".green(), args[2]);
}

fn get_pixels_in_rectangle(x: u32, y: u32, width: u32, height: u32) -> Vec<Position> {
    let mut result = Vec::new();
    for x in x..(x+width) {
        for y in y..(y+height) {
            result.push(Position { x, y });
        }
    }

    return result;
}

fn draw_amongus(image: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>, altcolor: Rgb<u8>) {

    image.put_pixel(x + 1, y + 0, color);
    image.put_pixel(x + 2, y + 0, color);
    image.put_pixel(x + 3, y + 0, color);

    image.put_pixel(x + 0, y + 1, color);
    image.put_pixel(x + 1, y + 1, color);
    image.put_pixel(x + 2, y + 1, altcolor);
    image.put_pixel(x + 3, y + 1, altcolor);

    image.put_pixel(x + 0, y + 2, color);
    image.put_pixel(x + 1, y + 2, color);
    image.put_pixel(x + 2, y + 2, color);
    image.put_pixel(x + 3, y + 2, color);

    image.put_pixel(x + 0, y + 3, color);
    image.put_pixel(x + 1, y + 3, color);
    image.put_pixel(x + 2, y + 3, color);
    image.put_pixel(x + 3, y + 3, color);

    image.put_pixel(x + 1, y + 4, color);
    image.put_pixel(x + 3, y + 4, color);
}

fn get_alt_color(color: Rgb<u8>, offset: u8) -> Rgb<u8> {
    let mut alt = Rgb([0, 0, 0]);
    for channel in 0..3 {
        let c = color.0[channel];
        if c > 255-offset {
            alt.0[channel] = c - offset;
        } else {
            alt.0[channel] = c + offset;
        }
    }

    return alt;
}