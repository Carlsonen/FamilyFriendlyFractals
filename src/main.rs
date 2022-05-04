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
mod wave_function;

fn print_available_commands() {
    println!("========================================================");
    println!("available commands:");
    println!("make <name>                      | generates fractal");
    println!("color <default/dark/gray/random> | changes the coloring");
    println!("r                                | generates random fractal");
    println!();
    println!("shape <number>                   | changes how messy (1-99, default 30)");
    println!("res <width> <height>             | changes resolution");
    println!();
    println!("orbit set <path>                 | name sample image in ./sample/");
    println!("orbit make <name>                | generates fractal from image");
    println!("orbit r                          | generates random from image");
    println!("orbit color <value>              | set hue difference (0-359)");
    println!();
    println!("gpu make <name>                  | generates fractal on GPU");
    println!("gpu r                            | generates random on GPU");
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
    let mut orbit_path: String = "amogus.png".to_string();
    let mut orbit_hue_diff = 50.0;
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
        else if command == "wave" {
            //println!("input name: ");
            let name: String = read!();
            let mut wave = wave_function::WaveWorld::new(128, 128, &name);
            wave.evaluate();
            wave.capture();
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
                julia_orbit_trap(&name, &name, &config, &orbit_path, orbit_hue_diff);
                println!("=> Fractal \"{}\" saved!", name);
                
            }
            else if func == "r" {
                let name: String = "random".to_string();
                let seed: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect();
                julia_orbit_trap(&name, &seed, &config, &orbit_path, orbit_hue_diff);
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
            else if func == "color" { 
                let mut c: u32 = read!();
                c = u32::max(c, 0);
                c = u32::min(c, 359);
                orbit_hue_diff = c as f32;
                println!("=> color diff set to {}", c);
            }
            else {
                println!("=> invalid command");
            }
        }
        else if command == "gpu" {
            let func: String = read!();
            if func == "make" {
                let name: String = read!();
                opencl_julia(&name, &name, &config);
                println!("=> Fractal \"{}\" saved!", name);
            }
            else if func == "r" {
                let name: String = "random".to_string();
                let seed: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect();
                opencl_julia(&name, &seed, &config);
                println!("=> Fractal \"{}\" saved!", seed);
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
    let zoom = 0.8;
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
        for _ in 0..3 {
            z = z * z + c;
            iteration += 1;
        }
        let onedivbylntwo = 1.44269504089;

        return iteration as f64 + 1.0 - (z.norm().ln().ln() * onedivbylntwo) as f64;
    }
    return iteration as f64;
}
fn julia_orbit_trap(name: &String, seed: &String, config: &Config, path: &String, hue_diff: f32) {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash_val = hasher.finish();
    let mut rng = Pcg64::seed_from_u64(hash_val);

    let width = config.screen.width;
    let height = config.screen.height;

    let mut img: RgbaImage = RgbaImage::new(width as u32, height as u32);
    use colors_transform::{Hsl, Rgb, Color};
    let sample_pic = open(format!("sample/{}", path)).unwrap().into_rgba8();
    
    let zoom = 2.8;
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
                let mut p = 4.0*(z+s);
                if p.re > 0.0 && p.im > 0.0 && p.re < 1.0 && p.im < 1.0 {
                    p.re *= (sample_pic.width()-1) as f64;
                    p.im *= (sample_pic.height()-1) as f64;
                    pixel = *sample_pic.get_pixel((p.re ) as u32, p.im as u32);
                    if pixel[3] != 0 {
                        let mut rgb = Rgb::from(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
                        rgb = rgb.adjust_hue((i as f32 * hue_diff) % 360.0);
                        pixel = image::Rgba([
                        rgb.get_red() as u8, 
                        rgb.get_green() as u8,
                        rgb.get_blue() as u8, 255]);
                    }
                }
                if pixel[3] != 0 {break;}
            }
            if pixel[3] == 0 {pixel = image::Rgba([255,255,255,255]);}
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

fn opencl_julia(name: &String, seed: &String, config: &Config) {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash_val = hasher.finish();
    let mut rng = Pcg64::seed_from_u64(hash_val);
    let angle: f64 = rng.gen_range(-3.14..3.14);
    let seed_coordinate = find_good_julia(angle, config.shape.messiness_factor);
    let rmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);
    let gmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);
    let bmul: f64 = rng.gen_range(config.coloring.min..config.coloring.max);
    let w = config.screen.width;
    let h = config.screen.height;
    let height = (h as f32 /2.0).ceil() as u32;
    let max_iterations = config.shape.iterations;
    let mut img = image::RgbImage::new(w as u32, h as u32);
    // This is the kernel that we will be executed by each worker:
    //     buffer: output buffer where to store the number of iterations before the f(c) diverges.
    //             Accessing this memory is expensive, so it should be used sparingly. Since the
    //             output buffer is 1-dimensional, we will need to flatten the output by mapping
    //             a pixel at coordinates (r, c) to (r * width + c).
    //     width: image width.
    //     height: image height.
    //     max_iterations: maximum number of iterations of f(c).
    let src = r#"
        __kernel void julia(__global uint* buffer, uint width, uint height, uint max_iterations, float cx, float cy, float rmul, float gmul, float bmul) {
            // Get the x coordinate of this worker. We can get x and y coordinates because the kernel
            // operates over a 2-dimensional data structure, as specified in the Rust code below.
            int x = get_global_id(0);
            // Get the y coordinate of this worker.
            int y = get_global_id(1);
            // The code below is an almost line-by-line port of the naive implementation, which has
            // been optimized to have only 3 multiplications in the inner loop.
            float zx = ((float)x / width - 0.5) * 3.8;
            float zy = ((float)y / height) * 3.8 * 0.5;
            float _zx = 0.0;
            float _zy = 0.0;
            float zx2 = zx * zx;
            float zy2 = zy * zy;
            uint iteration = 0;
            float smooth = 0.0;
            while (((zx2 + zy2) <= 4.0) && (iteration < max_iterations)) {
                zy = (zx + zx) * zy + cy;
                zx = zx2 - zy2 + cx;
                zx2 = zx * zx;
                zy2 = zy * zy;
                iteration = iteration + 1;
            }
            if (iteration < max_iterations) {
                zy = (zx + zx) * zy + cy;
                zx = zx2 - zy2 + cx;
                zx2 = zx * zx;
                zy2 = zy * zy;

                zy = (zx + zx) * zy + cy;
                zx = zx2 - zy2 + cx;
                zx2 = zx * zx;
                zy2 = zy * zy;

                zy = (zx + zx) * zy + cy;
                zx = zx2 - zy2 + cx;
                zx2 = zx * zx;
                zy2 = zy * zy;

                iteration = iteration + 3;
                float onedivbylntwo = 1.44269504089;
        
                float znorm = sqrt(zx2+zy2);
                smooth = (float)iteration + 1.0 - log(log(znorm)) * onedivbylntwo;
                //smooth = convert_float(iteration);
            }
            // Store the number of iterations computed by this worker.
            uint red = (uint)convert_uchar(sin(0.1 * smooth * rmul)*255.0);
            uint green = (uint)convert_uchar(sin(0.1 * smooth * gmul)*255.0);
            uint blue = (uint)convert_uchar(sin(0.1 * smooth * bmul)*255.0);

            buffer[width * y + x] = (red << 16) | (green << 8) | blue;
        }
    "#;
    // Build an OpenCL context, make it run the OpenCL C code defined above, and set the
    // data structure to operate on as a 2-dimensional w by h structure.
    let pro_que = ocl::ProQue::builder().src(src).dims((w, height)).build().unwrap();
    // Let buffer be the output buffer accessible by workers. This memory lives on the
    // hardware accelerator (e.g.: the GPU).
    let buffer = pro_que.create_buffer::<u32>().unwrap();
    //let buffer = ocl::Buffer::builder().len(w*height*3).build();
    // Build the OpenCL program, make it run the kernel called `mandelbrot` and bind actual
    // values to the arguments of `mandelbrot`.
    let kernel = pro_que
        .kernel_builder("julia")
        .arg(&buffer)
        .arg(w)
        .arg(height)
        .arg(max_iterations as u32)
        .arg(seed_coordinate.re as f32)
        .arg(seed_coordinate.im as f32)
        .arg(rmul as f32)
        .arg(gmul as f32)
        .arg(bmul as f32)
        .build()
        .unwrap();
    // Run the OpenCL kernel
    unsafe { kernel.enq().unwrap() };
    let mut vec = vec![0u32; buffer.len()];
    // Copy the OpenCL buffer back to a traditional Vec.
    buffer.read(&mut vec).enq().unwrap();
    for i in 0..buffer.len() {
        let x = i as u32 % w;
        let y = i as u32 / w;
        let val = vec[i];
        let rgb = [
            ((val >> 16) & 0xff) as u8,
            ((val >> 8) & 0xff) as u8,
            ((val >> 0) & 0xff) as u8
        ];
        let color = image::Rgb(rgb);
        img.put_pixel(x, y+(h >> 1), color);
        img.put_pixel(h - x - 1, height - y - 1, color);
    }
    let path = format!("{}.png", name);
    img.save(path).unwrap();
}

