extern crate image;
extern crate num;
use num::complex::Complex;
use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg64;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use text_io::read;
mod configstructs;
use configstructs::Coloring;
use configstructs::Config;
use configstructs::Screen;
use configstructs::Shape;

fn main() {
    let config = Config::new(
        Coloring::colorful(),            // try the presets or experiment with ""::new"
        Shape::default(),                // how messy the fractal is basically
        Screen::new(1920 * 2, 1080 * 2), // fractal height is always same
    );
    loop {
        println!("input name: ");
        let name: String = read!();
        if name == "stop" {
            println!("program stopped");
            break;
        }
        randomish_fractal(&name, &config);
        println!("Fractal \"{}\" saved!", name);
    }
}

fn randomish_fractal(name: &String, config: &Config) {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let hash_val = hasher.finish();
    let mut rng = Pcg64::seed_from_u64(hash_val);
    if name == "random" {
        rng = Pcg64::seed_from_u64(rand::random());
    }

    let width = config.screen.width;
    let height = config.screen.height;
    let max_iterations = config.shape.iterations;

    let rmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);
    let gmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);
    let bmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);

    let mut x: f64;
    let mut y: f64;
    let mut seed_coordinate: Complex<f64>;
    loop {
        x = rng.gen_range(-2.5..2.5);
        y = rng.gen_range(-2.5..2.5);
        seed_coordinate = Complex::new(x, y);
        let i = mandel(seed_coordinate, 100);
        if i >= config.shape.min && i <= config.shape.max {
            break;
        }
    }

    use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    let zoom = 3.8;
    let aspect_ratio = width as f64 / height as f64;
    for x in 0..width {
        for y in 0..height {
            let co_x = aspect_ratio * zoom * (x as f64 / width as f64 - 0.5);
            let co_y = zoom * (y as f64 / height as f64 - 0.5);
            let coordinate = Complex::new(co_x, co_y);
            let iterations = julia(coordinate, max_iterations, seed_coordinate) / 10.0;

            let red = (iterations * rmul).sin() * 255.0;
            let green = (iterations * gmul).sin() * 255.0;
            let blue = (iterations * bmul).sin() * 255.0;

            let color = [red as u8, green as u8, blue as u8];

            img.put_pixel(x as u32, y as u32, image::Rgb(color));
        }
    }
    let mut path = format!("{}{}", name, ".png");
    img.save(path).unwrap();
}
fn julia(coordinate: Complex<f64>, max_iterations: i32, seed: Complex<f64>) -> f64 {
    let mut z = coordinate;
    let c = seed;
    let mut iteration = 0;
    while iteration < max_iterations && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        iteration += 1;
    }
    if iteration < max_iterations {
        for i in 0..3 {
            z = z * z + c;
            iteration += 1;
        }
        let onedivbylntwo = 1.44269504089;

        return iteration as f64 + 1.0 - (z.norm().ln().ln() * onedivbylntwo) as f64;
    }
    return iteration as f64;
}
fn mandel(coordinate: Complex<f64>, max_iterations: i32) -> i32 {
    let mut z = Complex::new(0.0, 0.0);
    let c = coordinate;
    let mut iteration = 0;
    while (iteration < max_iterations) & (z.norm_sqr() <= 4.0) {
        z = z.powu(2) + c;
        iteration += 1;
    }
    return iteration;
}
