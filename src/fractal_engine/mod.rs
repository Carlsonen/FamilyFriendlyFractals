extern crate image;
use image::{open, ImageBuffer, RgbImage, RgbaImage};

extern crate num;
use num::complex::Complex;

use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg64;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::coloring::*;
use crate::shit::*;

pub struct FractalEngine {
    rng: Pcg64,
    width: u32,
    height: u32,
    messiness: i32,
    coloring: ColorConfig,
    sample_path: String,
    image_fractal_hue: f32,
    zoom: f64,
}
impl FractalEngine {
    pub fn new() -> Self {
        FractalEngine {
            rng: Pcg64::seed_from_u64(0),
            width: 3072,
            height: 3072,
            messiness: 30,
            coloring: ColorConfig::new(),
            sample_path: String::from(""),
            image_fractal_hue: 0.0,
            zoom: 3.8,
        }
    }
    pub fn set_seed(&mut self, seed: &String) {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let hash_val = hasher.finish();
        self.rng = Pcg64::seed_from_u64(hash_val);
    }
    pub fn set_coloring(&mut self, coloring: ColoringMethod) {
        self.coloring.set_coloring(coloring);
    }
    pub fn set_shape(&mut self, shape: i32) {
        self.messiness = shape.max(0).min(99);
    }
    pub fn set_res(&mut self, w: u32, h: u32) -> (u32, u32) {
        self.width = w.max(1).min(9999);
        self.height = h.max(1).min(9999);
        (self.width, self.height)
    }
    pub fn set_zoom(&mut self, z: f64) {
        self.zoom = z;
    }
    pub fn set_sample_path(&mut self, path: &String) -> Result<&str, &str> {
        use std::path::Path;
        if Path::new(&path).exists() {
            self.sample_path = String::from(path);
            return Ok("");
        } else {
            return Err("");
        }
    }
    pub fn set_image_fractal_hue(&mut self, hue: u32) -> u32 {
        let tmp = hue.max(0).min(359);
        self.image_fractal_hue = tmp as f32;
        tmp
    }
    pub fn gen_julia(&mut self, name: &String) {
        // good shit
        let width = self.width;
        let height = self.height;
        let max_iterations = 10_000;
        // rng shit important order
        let angle: f64 = self.rng.gen_range(-3.14..3.14);
        let seed_coordinate = find_good_julia(angle, self.messiness);
        self.coloring.update_color_modifiers(&mut self.rng);
        // image shit
        let mut img: RgbImage = ImageBuffer::new(width, height);
        let zoom = self.zoom;
        let aspect_ratio = width as f64 / height as f64;
        for x in 0..(width as f32 / 2.0).ceil() as u32 {
            for y in 0..height {
                let co_x = aspect_ratio * zoom * (x as f64 / width as f64 - 0.5);
                let co_y = zoom * (y as f64 / height as f64 - 0.5);
                let coordinate = Complex::new(co_x, co_y);
                let iterations = julia_pixel(coordinate, max_iterations, seed_coordinate);
                let color = self.coloring.get_color(iterations);
                img.put_pixel(x as u32, y as u32, color);
                img.put_pixel(width - x - 1, height - y - 1, color);
            }
        }
        make_folder(&String::from("fractals"));
        let result = img.save(format!("fractals/{name}.png"));
        match result {
            Ok(_) => {}
            Err(e) => println!("error: {:?}", e),
        }
    }
    pub fn gen_image_fractal(&mut self, name: &String) {
        use colors_transform::{Color, Rgb};
        // good shit
        let width = self.width;
        let height = self.height;
        // rng shit important order
        let angle: f64 = self.rng.gen_range(-3.14..3.14);
        let c = find_good_julia(angle, self.messiness);
        // image shit
        let sample_pic = open(format!("{}", self.sample_path)).unwrap().into_rgba8();
        let mut img: RgbaImage = ImageBuffer::new(width, height);
        let zoom = self.zoom;
        let aspect_ratio = width as f64 / height as f64;
        for x in 0..(width as f32 / 2.0).ceil() as u32 {
            for y in 0..height {
                let co_x = aspect_ratio * zoom * (x as f64 / width as f64 - 0.5);
                let co_y = zoom * (y as f64 / height as f64 - 0.5);
                let mut z = Complex::new(co_x, co_y);

                let mut pixel = image::Rgba([0, 0, 0, 0]);
                let mut i = 0;
                while i < 10_000 {
                    if z.norm_sqr() > 4.0 {
                        break;
                    }
                    z = z * z + c;

                    //let box_start = Complex::new(0.0, 0.0);
                    let box_start = c;
                    let box_size = c.norm_sqr();

                    let mut p = (z - box_start) / box_size;
                    if p.re > 0.0 && p.im > 0.0 && p.re < 1.0 && p.im < 1.0 {
                        p.re *= (sample_pic.width() - 1) as f64;
                        p.im *= (sample_pic.height() - 1) as f64;
                        pixel = *sample_pic.get_pixel((p.re) as u32, p.im as u32);
                        if pixel[3] != 0 {
                            let mut rgb =
                                Rgb::from(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
                            rgb = rgb.adjust_hue((i as f32 * self.image_fractal_hue) % 360.0);
                            pixel = image::Rgba([
                                rgb.get_red() as u8,
                                rgb.get_green() as u8,
                                rgb.get_blue() as u8,
                                255,
                            ]);
                        }
                    }
                    if pixel[3] != 0 {
                        break;
                    }
                    i += 1;
                }
                if pixel[3] == 0 {
                    pixel = image::Rgba([255, 255, 255, 255]); // white
                                                               //pixel = image::Rgba([0,0,0,0]);         // invis
                }
                img.put_pixel(x as u32, y as u32, pixel);
                img.put_pixel(width - x - 1, height - y - 1, pixel);
            }
        }
        make_folder(&String::from("fractals"));
        let result = img.save(format!("fractals/{name}.png"));
        match result {
            Ok(_) => {}
            Err(e) => println!("error: {:?}", e),
        }
    }
}

fn julia_pixel(coordinate: Complex<f64>, max_iterations: i32, c: Complex<f64>) -> f64 {
    let mut z = coordinate;
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

fn mandel_pixel(coordinate: Complex<f64>, max_iterations: i32) -> i32 {
    let mut z = Complex::new(0.0, 0.0);
    let c = coordinate;
    let mut iteration = 0;
    while (iteration < max_iterations) & (z.norm_sqr() <= 4.0) {
        z = z * z + c;
        iteration += 1;
    }
    return iteration;
}

fn find_good_julia(angle: f64, messi: i32) -> Complex<f64> {
    let x = angle.cos() * 2.0;
    let y = angle.sin() * 2.0;
    let mut coord = Complex::new(x, y);
    let mut step = coord / 2.0;
    let mut sign = -1.0;
    loop {
        coord += step * sign;
        let i = mandel_pixel(coord, 100);
        if i < messi {
            sign = -1.0;
        } else if i > messi {
            sign = 1.0;
        } else {
            return coord;
        }
        step *= 0.51;
    }
}
