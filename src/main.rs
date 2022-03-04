extern crate image;
extern crate num;
use std::path::Path;
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

fn print_available_commands() {
    println!("==============================================================");
    println!("available commands:");
    println!("stop                             | stops the program");
    println!("make <name>                      | generates a fractal");
    println!("color <default/dark/gray/random> | changes the coloring method");
    println!("shape <default/simple/messy>     | changes how messy");
    println!("res <width> <height>             | changes resolution");
    println!("==============================================================");
}

fn main() {
    let mut config = Config::new(
        Coloring::default(),           // try the presets or experiment with ""::new"
        Shape::default(),                 // how messy the fractal is basically
        Screen::new(1080, 1080),  // fractal height is always same
    );
    print_available_commands();
    loop {
        println!();
        //println!("input command: ");
        let command: String = read!();
        if command == "stop" {
            println!("program stopped");
            break;
        }
        else if command == "make" {
            //println!("input name: ");
            let name: String = read!();
            randomish_fractal(&name, &config);
            println!("=> Fractal \"{}\" saved!", name);
        }
        else if command == "color" {
            //println!("input color: ");
            let color: String = read!();
            if color == "dark" {
                config.coloring = Coloring::dark();
                println!("=> color set to dark")
            }
            else if color == "gray" {
                config.coloring = Coloring::gray();
                println!("=> color set to gray")
            }
            else if color == "random" {
                config.coloring = Coloring::random();
                println!("=> color set to random")
            }
            else {
                config.coloring = Coloring::default();
                println!("=> color set to default")
            }
        }
        else if command == "shape" {
            //println!("input shape: ");
            let shape: String = read!();
            if shape == "simple" {
                config.shape = Shape::simple();
                println!("=> shape set to simple")
            }
            else if shape == "messy" {
                config.shape = Shape::messy();
                println!("=> shape set to messy")
            }
            else {
                config.shape = Shape::default();
                println!("=> shape set to default");
            }
        }
        else if command == "res" {
            let width: i32 = read!();
            let height: i32 = read!();
            config.screen = Screen::new(width, height);
            println!("=> screen set to {}x{}", width, height);
        }
        else {
            print_available_commands();
        }

        
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
    let path = format!("{}{}", name, ".png");
    let os_path = Path::new(&path);
    img.save(os_path).unwrap();
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
