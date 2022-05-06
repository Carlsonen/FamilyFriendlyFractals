use std::collections::hash_map::DefaultHasher;
use image::{ImageBuffer, RgbImage};
use std::hash::{Hash, Hasher};
use rand::prelude::*;
use rand_pcg::Pcg64;
use family_friendly_fractals::shit::*;

pub fn noise(w: u32, h: u32) {
    let mut img: RgbImage = ImageBuffer::new(w, h);
    let mut rng = Pcg64::seed_from_u64(0);
    for x in 0..w {
        for y in 0..h {
            let r: u8 = rng.gen_range(0..255);
            let g: u8 = rng.gen_range(0..255);
            let b: u8 = rng.gen_range(0..255);

            img.put_pixel(x, y, image::Rgb([r,g,b]));
        }
    }
    make_folder(&String::from("fractals"));
    img.save("fractals/noise.png").unwrap();
}

pub struct WaveWorld {
    width: u32,
    height: u32,
    grid: Vec<Cell>,
    rng: Pcg64
}
impl WaveWorld {
    pub fn new(width: u32, height: u32, seed: &String) -> Self {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let hash_val = hasher.finish();
        let rng = Pcg64::seed_from_u64(hash_val);
        let grid = vec![Cell{lowest: 0, heighest: 36, value: -1}; (width * height) as usize];

        WaveWorld {width: width, height: height, grid: grid, rng: rng}
    }

    pub fn evaluate(&mut self) {
        self.collapse(self.width / 2, self.height / 2);
        for x in 0..self.width {
            for y in 0..self.height {
                self.collapse(x, y);
            }
        }
    }

    pub fn capture(&self) {
        use colors_transform::{Hsl, Color};
        let mut img: RgbImage = ImageBuffer::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let value = self.get_value(x, y) as f32;
                let c = Hsl::from(value * 10.0, 95.0, 50.0).to_rgb();
                    
                img.put_pixel(x, y, image::Rgb([c.get_red() as u8, c.get_green() as u8, c.get_blue() as u8]));
            }
        }
        make_folder(&String::from("fractals"));
        img.save("fractals/wave.png").unwrap();
    }

    fn collapse(&mut self, x: u32, y: u32) {
        if x >= self.width || y >= self.height {return} 
        if self.get_value(x, y) != -1 {return}
        let lowest = self.get_lowest(x, y);
        let heighest = self.get_heighest(x, y);
        let val = self.rng.gen_range(lowest..heighest+1);
        self.set(x, y, val);
        for _x in 0..self.width {
            for _y in 0..self.height {
                let d = dist(x, y, _x, _y) as i32;
                self.update_range(_x, _y, val - d, val + d);
            }
        }
        let d = self.rng.gen_range(0..2);
        if d == 0 {
            self.collapse(x+1, y);
            self.collapse(x-1, y);
        }
        else {
            self.collapse(x, y+1);
            self.collapse(x, y-1);
        }
    }
}
fn dist(x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as u32
}
impl WaveWorld {
    fn ix(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }
    fn set(&mut self, x: u32, y: u32, value: i32) {
        let ix = self.ix(x,y);
        self.grid[ix].value = value;
    }
    fn update_range(&mut self, x: u32, y: u32, low: i32, high: i32) {
        let ix = self.ix(x, y);
        self.grid[ix].lowest = self.grid[ix].lowest.max(low);
        self.grid[ix].heighest = self.grid[ix].heighest.min(high);
    }
    fn get_value(&self, x: u32, y: u32) -> i32 {
        self.grid[self.ix(x,y)].value
    }
    fn get_lowest(&self, x: u32, y: u32) -> i32 {
        self.grid[self.ix(x,y)].lowest
    }
    fn get_heighest(&self, x: u32, y: u32) -> i32 {
        self.grid[self.ix(x,y)].heighest
    }
}

#[derive(Clone)]
struct Cell {
    pub lowest: i32,
    pub heighest: i32,
    pub value: i32
}