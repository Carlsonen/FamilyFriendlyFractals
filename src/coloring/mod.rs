use image;
use rand::Rng;
use rand_pcg::Pcg64;

pub struct ColorConfig {
    color_method: ColoringMethod,
    a: f64,
    b: f64,
    c: f64,
}
impl ColorConfig {
    pub fn new() -> Self {
        ColorConfig {
            color_method: ColoringMethod::Default,
            a: 0.0,
            b: 0.0,
            c: 0.0,
        }
    }
    pub fn set_coloring(&mut self, coloring: ColoringMethod) {
        self.color_method = coloring;
    }
    pub fn update_color_modifiers(&mut self, rng: &mut Pcg64) {
        match self.color_method {
            ColoringMethod::Default => self.set_modifiers(0.0, 0.2, rng),
            ColoringMethod::Dark => self.set_modifiers(0.05, 0.1, rng),
            ColoringMethod::Colorful => self.set_modifiers(0.0, 1.0, rng),
            ColoringMethod::Gray => self.set_modifiers(0.2, 0.22, rng),
            ColoringMethod::Gold => self.set_modifiers(0.0, 0.2, rng),
            ColoringMethod::Rainbow => self.set_modifiers(0.0, 0.2, rng),
        }
    }
    fn set_modifiers(&mut self, min: f64, max: f64, rng: &mut Pcg64) {
        self.a = rng.gen_range(min..max);
        self.b = rng.gen_range(min..max);
        self.c = rng.gen_range(min..max);
    }
    pub fn get_color(&self, i: f64) -> image::Rgb<u8> {
        match self.color_method {
            ColoringMethod::Default => return self.original_color(i),
            ColoringMethod::Dark => return self.original_color(i),
            ColoringMethod::Colorful => return self.original_color(i),
            ColoringMethod::Gray => return self.original_color(i),
            ColoringMethod::Gold => return self.gold_color(i),
            ColoringMethod::Rainbow => return self.rainbow_color(i),
        }
    }
    fn original_color(&self, i: f64) -> image::Rgb<u8> {
        let red = (i * self.a).sin() * 255.0;
        let green = (i * self.b).sin() * 255.0;
        let blue = (i * self.c).sin() * 255.0;
        let color = [red as u8, green as u8, blue as u8];
        image::Rgb(color)
    }
    fn gold_color(&self, i: f64) -> image::Rgb<u8> {
        let red = 255.0 - i.sin() * 10.0;
        let green = 200.0 - i.cos() * 10.0;
        let blue = 12;
        let color = [red as u8, green as u8, blue as u8];
        image::Rgb(color)
    }
    fn rainbow_color(&self, i: f64) -> image::Rgb<u8> {
        let hue = i * 10.0;
        hue_to_rgb(hue % 360.0)
    }
}
fn hue_to_rgb(hue: f64) -> image::Rgb<u8> {
    let h = hue / 60.0;
    let i = h as i32 % 6;
    let f = h - i as f64;
    let p = (255.0 * (1.0 - f)) as u8;
    let q = (255.0 * (1.0 - (f * (1.0 - (h % 2.0))))) as u8;
    let t = (255.0 * (1.0 - (f * f))) as u8;
    let r = 255;
    let g = 255;
    let b = 255;
    match i {
        0 => image::Rgb([r, g, b]),
        1 => image::Rgb([q, r, b]),
        2 => image::Rgb([p, r, b]),
        3 => image::Rgb([p, q, b]),
        4 => image::Rgb([p, p, b]),
        5 => image::Rgb([r, p, b]),
        _ => image::Rgb([r, g, b]),
    }
}
#[derive(Copy, Clone, Debug)]
pub enum ColoringMethod {
    Default,
    Dark,
    Colorful,
    Gray,
    Gold,
    Rainbow,
}
