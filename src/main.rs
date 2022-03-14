extern crate image;
use image::{ImageBuffer, RgbImage, RgbaImage, open};

extern crate num;
use num::complex::Complex;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::{distributions::Alphanumeric, Rng};
use rand::prelude::*;
use rand_pcg::Pcg64;

use text_io::read;

mod configstructs;
use configstructs::Coloring;
use configstructs::Config;
use configstructs::Screen;
use configstructs::Shape;

use std::path::Path;

fn print_available_commands() {
    println!("========================================================");
    println!("available commands:");
    println!("make <name>                      | generates a fractal");
    println!("color <default/dark/gray/random> | changes the coloring");
    println!("r                                | makes a random fractal");
    println!();
    println!("shape <number>                   | changes how messy (1-99, default 30)");
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
    let mut orbit_path: String = "bear.ico".to_string();
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
            let mut messi: i32 = read!();
            messi = i32::max(messi, 1);
            messi = i32::min(messi, 99);
            config.shape.messiness_factor = messi;
            println!("=> messiness set to {}", messi);
        }
        else if command == "res" {
            let mut width: u32 = read!();
            let mut height: u32 = read!();
            width = u32::max(width, 1);
            height = u32::max(height, 1);
            width = u32::min(width, 9999);
            height = u32::min(height, 9999);
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
                let pic: String = read!();
                let path = format!("sample/{}", pic);
                if Path::new(&path).exists() {
                    println!("=> Sample picture set to {}", pic);
                    orbit_path = pic;
                }
                else {
                    println!("=> the file \"{}\" does not exist", path);
                }
            }
            else {
                println!("=> invalid command");
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
    
    let width: u32 = config.screen.width;
    let height: u32 = config.screen.height;
    let max_iterations = config.shape.iterations;
    
    let angle: f64 = rng.gen_range(-3.14..3.14);
    let seed_coordinate = find_good_julia(angle, config.shape.messiness_factor);
    
    let rmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);
    let gmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);
    let bmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);
    let zoom = 3.8;
    let aspect_ratio = width as f64 / height as f64;
    for x in 0..(width as f32 /2.0).ceil() as u32 {
        for y in 0..height {
            let co_x = aspect_ratio * zoom * (x as f64 / width as f64 - 0.5);
            let co_y = zoom * (y as f64 / height as f64 - 0.5);
            let coordinate = Complex::new(co_x, co_y);
            
            let iterations = julia(coordinate, max_iterations, seed_coordinate);
            let red = (0.1 * iterations * rmul).sin() * 255.0;
            let green = (0.1 * iterations * gmul).sin() * 255.0;
            let blue = (0.1 * iterations * bmul).sin() * 255.0;
            let color = [red as u8, green as u8, blue as u8];
        
            img.put_pixel(x as u32, y as u32, image::Rgb(color));
            img.put_pixel(width - x - 1, height - y - 1, image::Rgb(color));
        }
    }
    
    let path = format!("{}.png", name);
    img.save(path).unwrap();
}
fn julia(coordinate: Complex<f64>, max_iterations: i32, seed: Complex<f64>) -> f64 {
    let mut z = coordinate;
    let c = seed;
    let mut iteration = 0;
    while iteration < max_iterations && z.norm_sqr() < 4.0 {
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
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash_val = hasher.finish();
    let mut rng = Pcg64::seed_from_u64(hash_val);

    let width = config.screen.width;
    let height = config.screen.height;

    let mut img: RgbaImage = RgbaImage::new(width as u32, height as u32);
    let sample_pic = open(format!("sample/{}", path)).unwrap().into_rgba8();
    
    let zoom = 3.8;
    let aspect_ratio = width as f64 / height as f64;

    let angle: f64 = rng.gen_range(-3.14..3.14);
    let c = find_good_julia(angle, config.shape.messiness_factor);
    
    for x in 0..(width as f32 /2.0).ceil() as u32 {
        for y in 0..height {
            let co_x = aspect_ratio * zoom * (x as f64 / width as f64 - 0.5);
            let co_y = zoom * (y as f64 / height as f64 - 0.5);

            let mut z = Complex::new(co_x, co_y);
            
            let s = Complex::new(0.0, 0.0);
            let mut pixel = image::Rgba([0,0,0,0]);
            for i in 0..1000 {
                if z.norm_sqr() > 4.0 {break;}
                z = z*z+c;
                let mut p = 0.4*(z+s);
                if p.re > 0.0 && p.im > 0.0 && p.re < 1.0 && p.im < 1.0 {
                    p.re *= (sample_pic.width()-1) as f64;
                    p.im *= (sample_pic.height()-1) as f64;
                    pixel = *sample_pic.get_pixel((p.re ) as u32, p.im as u32);
                }
                if pixel[3] != 0 {break;}
            }
            if pixel[3] == 0 {pixel = image::Rgba([0,0,16,255]);}
            img.put_pixel(x as u32, y as u32, pixel);
            img.put_pixel(width - x - 1, height - y - 1, pixel);
        }
    }
    let path = format!("{}.png", name);
    img.save(path).unwrap();
    
}
fn find_good_julia(angle: f64, messi: i32) -> Complex<f64>{
    let x = angle.cos() * 2.0;
    let y = angle.sin() * 2.0;
    let mut coord = Complex::new(x, y);
    let mut step = coord / 2.0;
    let mut sign = -1.0;
    loop {
        coord += step * sign;
        let i = mandel(coord, 100);
        if i < messi {sign = -1.0;}
        else if i > messi {sign = 1.0;}
        else {return coord;}
        step *= 0.51;
    }
}
fn mandel(coordinate: Complex<f64>, max_iterations: i32) -> i32 {
    let mut z = Complex::new(0.0, 0.0);
    let c = coordinate;
    let mut iteration = 0;
    while (iteration < max_iterations) & (z.norm_sqr() <= 4.0) {
        z = z * z + c;
        iteration += 1;
    }
    return iteration;
}

