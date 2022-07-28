use family_friendly_fractals::shit::*;
use image::{ImageBuffer, RgbImage};
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct WaveSprite {
    width: u32,
    height: u32,
    res: u32,
    rng: Pcg64,
    board: Vec<Option<[ColorBlueprint; 3]>>,
}
impl WaveSprite {
    pub fn new(width: u32, height: u32, res: u32, seed: &String) -> Self {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let hash_val = hasher.finish();
        let rng = Pcg64::seed_from_u64(hash_val);

        WaveSprite {
            width: width,
            height: height,
            res: res,
            rng: rng,
            board: vec![None; (width * height) as usize],
        }
    }
    pub fn make_img(&mut self) {
        let mut img: RgbImage = ImageBuffer::new(self.res * self.width, self.res * self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                let north = self.get_color_shit(x, y - 1);
                let west = self.get_color_shit(x - 1, y);
                let mut yellow = ColorBlueprint {
                    north: false,
                    east: false,
                    south: false,
                    west: false,
                    horizontal_ix: 0,
                    vertical_ix: 0,
                    color: [255, 255, 0],
                };
                let mut cyan = ColorBlueprint {
                    north: false,
                    east: false,
                    south: false,
                    west: false,
                    horizontal_ix: 0,
                    vertical_ix: 0,
                    color: [0, 255, 255],
                };
                let mut magenta = ColorBlueprint {
                    north: false,
                    east: false,
                    south: false,
                    west: false,
                    horizontal_ix: 0,
                    vertical_ix: 0,
                    color: [255, 0, 255],
                };
                match north {
                    Some(colors) => {
                        if colors[0].south {
                            yellow.north = true;
                            yellow.horizontal_ix = colors[0].horizontal_ix;
                        }
                        if colors[1].south {
                            cyan.north = true;
                            cyan.horizontal_ix = colors[1].horizontal_ix;
                        }
                        if colors[2].south {
                            magenta.north = true;
                            magenta.horizontal_ix = colors[2].horizontal_ix;
                        }
                        if !colors[0].south {
                            let mut num = self.rng.gen_range(0..3);
                            while (num == cyan.horizontal_ix) || (num == magenta.horizontal_ix) {
                                num = self.rng.gen_range(0..3);
                            }
                            yellow.horizontal_ix = num;
                        }
                        if !colors[1].south {
                            let mut num = self.rng.gen_range(0..3);
                            while (num == yellow.horizontal_ix) || (num == magenta.horizontal_ix) {
                                num = self.rng.gen_range(0..3);
                            }
                            cyan.horizontal_ix = num;
                        }
                        if !colors[2].south {
                            let mut num = self.rng.gen_range(0..3);
                            while (num == cyan.horizontal_ix) || (num == yellow.horizontal_ix) {
                                num = self.rng.gen_range(0..3);
                            }
                            magenta.horizontal_ix = num;
                        }
                    }
                    _ => {
                        let mut shit = [0, 1, 2];
                        shit.shuffle(&mut self.rng);
                        yellow.horizontal_ix = shit[0];
                        cyan.horizontal_ix = shit[1];
                        magenta.horizontal_ix = shit[2];
                        yellow.north = self.rng.gen::<bool>();
                        cyan.north = self.rng.gen::<bool>();
                        magenta.north = self.rng.gen::<bool>();
                    }
                }
                match west {
                    Some(colors) => {
                        if colors[0].east {
                            yellow.west = true;
                            yellow.vertical_ix = colors[0].vertical_ix;
                        }
                        if colors[1].east {
                            cyan.west = true;
                            cyan.vertical_ix = colors[1].vertical_ix;
                        }
                        if colors[2].east {
                            magenta.west = true;
                            magenta.vertical_ix = colors[2].vertical_ix;
                        }
                        if !colors[0].east {
                            let mut num = self.rng.gen_range(0..3);
                            while (num == cyan.vertical_ix) || (num == magenta.vertical_ix) {
                                num = self.rng.gen_range(0..3);
                            }
                            yellow.vertical_ix = num;
                        }
                        if !colors[1].east {
                            let mut num = self.rng.gen_range(0..3);
                            while (num == yellow.vertical_ix) || (num == magenta.vertical_ix) {
                                num = self.rng.gen_range(0..3);
                            }
                            cyan.vertical_ix = num;
                        }
                        if !colors[2].east {
                            let mut num = self.rng.gen_range(0..3);
                            while (num == cyan.vertical_ix) || (num == yellow.vertical_ix) {
                                num = self.rng.gen_range(0..3);
                            }
                            magenta.vertical_ix = num;
                        }
                    }
                    _ => {
                        let mut shit = [0, 1, 2];
                        shit.shuffle(&mut self.rng);
                        yellow.vertical_ix = shit[0];
                        cyan.vertical_ix = shit[1];
                        magenta.vertical_ix = shit[2];
                        yellow.west = self.rng.gen::<bool>();
                        cyan.west = self.rng.gen::<bool>();
                        magenta.west = self.rng.gen::<bool>();
                    }
                }
                yellow.east = self.rng.gen::<bool>();
                yellow.south = self.rng.gen::<bool>();
                while (yellow.north as u8
                    + yellow.east as u8
                    + yellow.south as u8
                    + yellow.west as u8)
                    < 2
                {
                    yellow.east = self.rng.gen::<bool>();
                    yellow.south = self.rng.gen::<bool>();
                }
                cyan.east = self.rng.gen::<bool>();
                cyan.south = self.rng.gen::<bool>();
                while (cyan.north as u8 + cyan.east as u8 + cyan.south as u8 + cyan.west as u8) < 2
                {
                    cyan.east = self.rng.gen::<bool>();
                    cyan.south = self.rng.gen::<bool>();
                }
                magenta.east = self.rng.gen::<bool>();
                magenta.south = self.rng.gen::<bool>();
                while (magenta.north as u8
                    + magenta.east as u8
                    + magenta.south as u8
                    + magenta.west as u8)
                    < 2
                {
                    magenta.east = self.rng.gen::<bool>();
                    magenta.south = self.rng.gen::<bool>();
                }
                draw_one_color(&mut img, x, y, self.res, yellow);
                draw_one_color(&mut img, x, y, self.res, cyan);
                draw_one_color(&mut img, x, y, self.res, magenta);
                self.board[(x + y * self.width) as usize] = Some([yellow, cyan, magenta]);
            }
        }

        img.save("fractals/wave.png").unwrap();
    }
    fn get_color_shit(&self, x: u32, y: u32) -> Option<[ColorBlueprint; 3]> {
        if x >= self.width {
            return None;
        }
        if y >= self.height {
            return None;
        }

        return self.board[(x + y * self.width) as usize];
    }
}
#[derive(Clone, Copy, std::fmt::Debug)]
struct ColorBlueprint {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    horizontal_ix: u32,
    vertical_ix: u32,
    color: [u8; 3],
}

fn draw_one_color(img: &mut RgbImage, x: u32, y: u32, res: u32, color: ColorBlueprint) {
    if !color.north && !color.east && !color.south && !color.west {
        return;
    }
    let line_width = res / 6;
    let startx = color.horizontal_ix * 2 * line_width + (res - 5 * line_width) / 2;
    let stopx = startx + line_width - 1;
    let starty = color.vertical_ix * 2 * line_width + (res - 5 * line_width) / 2;
    let stopy = starty + line_width - 1;

    let mut xmin = 0;
    if !color.west {
        xmin = startx
    }
    let mut xmax = res - 1;
    if !color.east {
        xmax = stopx;
    }
    let mut ymin = 0;
    if !color.north {
        ymin = starty;
    }
    let mut ymax = res - 1;
    if !color.south {
        ymax = stopy;
    }
    let darker = [color.color[0] / 2, color.color[1] / 2, color.color[2] / 2];
    for i in xmin..xmax + 1 {
        for j in starty..stopy + line_width / 2 + 1 {
            if &image::Rgb([255, 255, 0]) != img.get_pixel(x * res + i, y * res + j)
                || color.color != [255, 0, 255]
            {
                img.put_pixel(x * res + i, y * res + j, image::Rgb(darker));
            }
        }
        for j in starty..stopy + 1 {
            if &image::Rgb([255, 255, 0]) != img.get_pixel(x * res + i, y * res + j)
                || color.color != [255, 0, 255]
            {
                img.put_pixel(x * res + i, y * res + j, image::Rgb(color.color));
            }
        }
    }
    for j in ymin..ymax + 1 {
        for i in startx..stopx + 1 {
            if (&image::Rgb([255, 255, 0]) != img.get_pixel(x * res + i, y * res + j)
                && &image::Rgb([127, 127, 0]) != img.get_pixel(x * res + i, y * res + j))
                || color.color != [255, 0, 255]
            {
                img.put_pixel(x * res + i, y * res + j, image::Rgb(color.color));
            }
        }
    }
}

pub fn noise(w: u32, h: u32) {
    let mut img: RgbImage = ImageBuffer::new(w, h);
    let mut rng = Pcg64::seed_from_u64(0);
    for x in 0..w {
        for y in 0..h {
            let r: u8 = rng.gen_range(0..255);
            let g: u8 = rng.gen_range(0..255);
            let b: u8 = rng.gen_range(0..255);

            img.put_pixel(x, y, image::Rgb([r, g, b]));
        }
    }
    make_folder(&String::from("fractals"));
    img.save("fractals/noise.png").unwrap();
}

pub struct WaveWorld {
    width: u32,
    height: u32,
    grid: Vec<Cell>,
    rng: Pcg64,
}
impl WaveWorld {
    pub fn new(width: u32, height: u32, seed: &String) -> Self {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let hash_val = hasher.finish();
        let rng = Pcg64::seed_from_u64(hash_val);
        let grid = vec![
            Cell {
                lowest: 0,
                heighest: 36,
                value: -1
            };
            (width * height) as usize
        ];

        WaveWorld {
            width: width,
            height: height,
            grid: grid,
            rng: rng,
        }
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
        use colors_transform::{Color, Hsl};
        let mut img: RgbImage = ImageBuffer::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let value = self.get_value(x, y) as f32;
                let c = Hsl::from(value * 10.0, 95.0, 50.0).to_rgb();

                img.put_pixel(
                    x,
                    y,
                    image::Rgb([c.get_red() as u8, c.get_green() as u8, c.get_blue() as u8]),
                );
            }
        }
        make_folder(&String::from("fractals"));
        img.save("fractals/wave.png").unwrap();
    }

    fn collapse(&mut self, x: u32, y: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        if self.get_value(x, y) != -1 {
            return;
        }
        let lowest = self.get_lowest(x, y);
        let heighest = self.get_heighest(x, y);
        let val = self.rng.gen_range(lowest..heighest + 1);
        self.set(x, y, val);
        for _x in 0..self.width {
            for _y in 0..self.height {
                let d = dist(x, y, _x, _y) as i32;
                self.update_range(_x, _y, val - d, val + d);
            }
        }
        let d = self.rng.gen_range(0..2);
        if d == 0 {
            self.collapse(x + 1, y);
            self.collapse(x - 1, y);
        } else {
            self.collapse(x, y + 1);
            self.collapse(x, y - 1);
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
        let ix = self.ix(x, y);
        self.grid[ix].value = value;
    }
    fn update_range(&mut self, x: u32, y: u32, low: i32, high: i32) {
        let ix = self.ix(x, y);
        self.grid[ix].lowest = self.grid[ix].lowest.max(low);
        self.grid[ix].heighest = self.grid[ix].heighest.min(high);
    }
    fn get_value(&self, x: u32, y: u32) -> i32 {
        self.grid[self.ix(x, y)].value
    }
    fn get_lowest(&self, x: u32, y: u32) -> i32 {
        self.grid[self.ix(x, y)].lowest
    }
    fn get_heighest(&self, x: u32, y: u32) -> i32 {
        self.grid[self.ix(x, y)].heighest
    }
}

#[derive(Clone)]
struct Cell {
    pub lowest: i32,
    pub heighest: i32,
    pub value: i32,
}
