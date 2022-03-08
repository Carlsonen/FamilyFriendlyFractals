extern crate image;
extern crate num;
use image::io::Reader as ImageReader;
use std::path::Path;
use num::complex::Complex;
use rand::{distributions::Alphanumeric, Rng};
use rand::prelude::*;
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
    println!("========================================================");
    println!("available commands:");
    println!("make <name>                      | generates a fractal");
    println!("color <default/dark/gray/random> | changes the coloring");
    println!("r                                | makes a random fractal");
    println!();
    println!("shape <default/simple/messy>     | changes how messy");
    println!("res <width> <height>             | changes resolution");
    println!();
    println!("orbit set <path>                 | name sample image in ./sample/");
    println!("orbit make <name>                | fractal from image");
    println!("orbit r                          | makes random from image");
    println!();
    println!("stop                             | stops the program");
    println!("========================================================");

}

fn main() {
    let mut config = Config::new(
        Coloring::default(),            // try the presets or experiment with ""::new"
        Shape::default(),               // how messy the fractal is basically
        Screen::default(),              // fractal height is always same
    );
    print_available_commands();
    let mut orbit_path: String = "bear.ico".to_string();
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
            randomish_fractal(&name, &name, &config);
            println!("=> Fractal \"{}\" saved!", name);
        }
        else if command == "r" {
            let name: String = "random".to_string();
            let seed: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect();
            randomish_fractal(&name, &seed, &config);
            println!("=> Fractal \"{}\" saved!", seed);
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
        else if command == "orbit" {
            let func: String = read!();
            if func == "make" {
                let name: String = read!();
                julia_orbit_trap(&name, &name, &config, &orbit_path);
                println!("=> Fractal \"{}\" saved!", name);
                
            }
            else if func == "r" {
                let name: String = "random".to_string();
                let seed: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect();
                julia_orbit_trap(&name, &seed, &config, &orbit_path);
                println!("=> Fractal \"{}\" saved!", seed);
            }
            else if func == "set" {
                orbit_path = read!();
            }
            else {
                print_available_commands();
            }
            
        }
        else {
            print_available_commands();
        }

        
    }
}

fn randomish_fractal(name: &String, seed: &String, config: &Config) {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash_val = hasher.finish();
    let mut rng = Pcg64::seed_from_u64(hash_val);
    

    let width = config.screen.width;
    let height = config.screen.height;
    let max_iterations = config.shape.iterations;

    

    let mut x: f64;
    let mut y: f64;
    let mut seed_coordinate: Complex<f64>;

    loop {
        x = rng.gen_range(-1.0..0.5);
        y = rng.gen_range(-1.0..1.0);
        seed_coordinate = Complex::new(x, y);
        let i = mandel(seed_coordinate, 100);
        if i >= config.shape.min && i <= config.shape.max {
            break;
        }
    }

    use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, open};
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);
    let rmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);
    let gmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);
    let bmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);
    let zoom = 3.8;
    let aspect_ratio = width as f64 / height as f64;
    for x in 0..width {
        for y in 0..height {
            let co_x = aspect_ratio * zoom * (x as f64 / width as f64 - 0.5);
            let co_y = zoom * (y as f64 / height as f64 - 0.5);
            let coordinate = Complex::new(co_x, co_y);
            let mut red: f64 = 0.0;
            let mut green: f64 = 0.0;
            let mut blue: f64 = 0.0;

            
            let iterations = julia(coordinate, max_iterations, seed_coordinate);
            red = (0.1 * iterations * rmul).sin() * 255.0;
            green = (0.1 * iterations * gmul).sin() * 255.0;
            blue = (0.1 * iterations * bmul).sin() * 255.0;
            let mut color = [red as u8, green as u8, blue as u8];
        
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

fn julia_orbit_trap(name: &String, seed: &String, config: &Config, path: &String) {
    use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, open};

    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash_val = hasher.finish();
    let mut rng = Pcg64::seed_from_u64(hash_val);

    let width = config.screen.width;
    let height = config.screen.height;

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);
    let sample_pic = open(format!("sample/{}", path)).unwrap().into_rgb8();
    
    let zoom = 3.8;
    let aspect_ratio = width as f64 / height as f64;

    let mut c;
    let mut x;
    let mut y;
    loop {
        x = rng.gen_range(-1.0..0.5);
        y = rng.gen_range(-1.0..1.0);
        c = Complex::new(x, y);
        let i = mandel(c, 100);
        if i >= config.shape.min && i <= config.shape.max {
            break;
        }
    }

    for x in 0..width {
        for y in 0..height {
            let co_x = aspect_ratio * zoom * (x as f64 / width as f64 - 0.5);
            let co_y = zoom * (y as f64 / height as f64 - 0.5);

            let mut z = Complex::new(co_x, co_y);
            
            let s = Complex::new(0.15, 0.4);
            let mut pixel = image::Rgb([0,0,0]);
            for i in 0..1000 {
                if z.norm_sqr() > 4.0 {break;}
                z = z*z+c;
                let mut p = 0.4*(z+s);
                if p.re > 0.0 && p.im > 0.0 && p.re < 1.0 && p.im < 1.0 {
                    p.re *= (sample_pic.width()-1) as f64;
                    p.im *= (sample_pic.height()-1) as f64;
                    pixel = *sample_pic.get_pixel((p.re ) as u32, p.im as u32);
                }
                
                if pixel != image::Rgb([0,0,0]) {break;}
            }
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }
    let path = format!("{}{}", name, ".png");
    let os_path = Path::new(&path);
    img.save(os_path).unwrap();
    
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
