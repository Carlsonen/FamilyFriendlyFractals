use delaunator::{triangulate, Point};
use image::{open, ImageBuffer, RgbImage};
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem::swap;

pub fn polygon(source_path: &String, seed: &String) {
    let src_img = open(format!("{}", source_path)).unwrap().into_rgb8();
    let width = src_img.width();
    let height = src_img.height();

    let mut poly_img: RgbImage = open(format!("{}", source_path)).unwrap().into_rgb8();

    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash_val = hasher.finish();
    let mut rng = Pcg64::seed_from_u64(hash_val);
    get_edge_value(&src_img, 0, 0, 5);
    let mut points: Vec<Point> = vec![];
    let mut max_edge_val = 0.0f32;
    for x in 0..src_img.width() {
        for y in 0..src_img.height() {
            let edge_value = get_edge_value(&src_img, x, y, 2);
            max_edge_val = max_edge_val.max(edge_value);
            poly_img.put_pixel(x, y, image::Rgb([(edge_value * 0.2) as u8, 0, 0]));
            if rng.gen_range(0..20) == 0u8 {
                if edge_value > 3000.0 {
                    //draw_circle(&mut poly_img, posx, posy, 5.5, [255, 255, 255]);

                    points.push(Point {
                        x: x as f64,
                        y: y as f64,
                    });
                }
            } else if rng.gen_range(0..20000) == 0u16 {
                points.push(Point {
                    x: x as f64,
                    y: y as f64,
                });
            }
        }
    }
    println!("max edge {max_edge_val}");
    /*
    let triangles = triangulate(&points).triangles;
    for i in (0..triangles.len()).step_by(3) {
        let a = &points[triangles[i]];
        let b = &points[triangles[i + 1]];
        let c = &points[triangles[i + 2]];
        draw_line(
            &mut poly_img,
            a.x as u32,
            a.y as u32,
            b.x as u32,
            b.y as u32,
        );
        draw_line(
            &mut poly_img,
            a.x as u32,
            a.y as u32,
            c.x as u32,
            c.y as u32,
        );
        draw_line(
            &mut poly_img,
            c.x as u32,
            c.y as u32,
            b.x as u32,
            b.y as u32,
        );
    }*/

    poly_img.save("fractals/poly.png");
}

pub fn circles(source_path: &String) {
    let radius_multiplier: f64 = 0.99995;
    let radius_limit_multiplier = 0.05;
    let src_pic = open(format!("{}", source_path)).unwrap().into_rgb8();

    let width = src_pic.width();
    let height = src_pic.height();
    let mut radius: f64 = (u32::min(width, height) / 30) as f64;
    let radius_limit = radius * radius_limit_multiplier;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let mut rng = Pcg64::from_entropy();
    let mut iteration_count = 0;
    while radius >= radius_limit {
        let posx = rng.gen_range(0..width);
        let posy = rng.gen_range(0..height);
        let color = get_avg_from_circle(&src_pic, posx, posy, radius as f32);
        //println!("{:?}", color);
        draw_circle(&mut img, posx, posy, radius as f32, color);
        radius *= radius_multiplier;
        iteration_count += 1;
    }
    img.save("fractals/circle_approx.png");
}

fn draw_circle(img: &mut RgbImage, posx: u32, posy: u32, radius: f32, color: [u8; 3]) {
    let r = radius.ceil() as i32;
    for x in posx as i32 - r..posx as i32 + r {
        for y in posy as i32 - r..posy as i32 + r {
            if x < img.width() as i32
                && y < img.height() as i32
                && x >= 0
                && y >= 0
                && dist(x, y, posx as i32, posy as i32) <= radius
            {
                img.put_pixel(x as u32, y as u32, image::Rgb(color));
            }
        }
    }
}

fn get_avg_from_circle(img: &RgbImage, posx: u32, posy: u32, radius: f32) -> [u8; 3] {
    let mut count = 0;
    let mut color: [u32; 3] = [0, 0, 0];

    let r = (0.5 * radius).ceil() as i32;
    for x in posx as i32 - r..posx as i32 + r {
        for y in posy as i32 - r..posy as i32 + r {
            if x < img.width() as i32 && y < img.height() as i32 && x >= 0 && y >= 0 {
                let pixel = *img.get_pixel(x as u32, y as u32);
                //println!("{:?}", pixel);
                color[0] += pixel[0] as u32;
                color[1] += pixel[1] as u32;
                color[2] += pixel[2] as u32;
                count += 1;
            }
        }
    }
    if count > 0 {
        color[0] /= count;
        color[1] /= count;
        color[2] /= count;
    }

    [color[0] as u8, color[1] as u8, color[2] as u8]
}

fn dist(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    f32::sqrt(
        (x1 as f32 - x2 as f32) * (x1 as f32 - x2 as f32)
            + (y1 as f32 - y2 as f32) * (y1 as f32 - y2 as f32),
    )
}

fn get_edge_value(img: &RgbImage, posx: u32, posy: u32, radius: i32) -> f32 {
    let r = radius;

    let mut horizontal_kernel: Vec<Vec<f32>> = vec![];
    let mut vertical_kernel: Vec<Vec<f32>> = vec![];

    for x in -r..r + 1 {
        let mut horizontal_row: Vec<f32> = vec![];
        let mut vertical_row: Vec<f32> = vec![];
        for y in -r..r + 1 {
            let mut d = dist(x, y, 0, 0);
            d = 1.0 / (f32::exp(d / radius as f32));
            if x == 0 {
                horizontal_row.push(0.0);
            } else if x < 0 {
                horizontal_row.push(-d)
            } else {
                horizontal_row.push(d);
            }
            if y == 0 {
                vertical_row.push(0.0);
            } else if y < 0 {
                vertical_row.push(-d)
            } else {
                vertical_row.push(d);
            }
        }
        horizontal_kernel.push(horizontal_row);
        vertical_kernel.push(vertical_row);
    }
    let mut h = 0.0;
    let mut v = 0.0;
    for x in 0..2 * radius + 1 {
        for y in 0..2 * radius + 1 {
            let img_x = (x + posx as i32 - radius) as u32;
            let img_y = (y + posy as i32 - radius) as u32;
            if (img_x as i32) < radius
                || (img_y as i32) < radius
                || img_x as i32 > img.width() as i32 - radius - 1
                || img_y as i32 > img.height() as i32 - radius - 1
            {
                return 0.0;
            }
            let hv = horizontal_kernel[x as usize][y as usize];
            let vv = vertical_kernel[x as usize][y as usize];
            let pixel = img.get_pixel(img_x, img_y);
            let red = pixel[0] as f32;
            let green = pixel[1] as f32;
            let blue = pixel[2] as f32;
            let shit = (red * red + green * green + blue * blue).sqrt();
            h += hv * shit;
            v += vv * shit;
        }
    }
    f32::sqrt(h * h + v * v)
}

fn draw_line(img: &mut RgbImage, mut x1: u32, mut y1: u32, mut x2: u32, mut y2: u32) {
    let dx = x1 as f32 - x2 as f32;
    let dy = y1 as f32 - y2 as f32;

    if dx.abs() > dy.abs() {
        let mut y: f32 = y1 as f32;
        if x1 > x2 {
            y = y2 as f32;
            swap(&mut x1, &mut x2);
        }
        for x in x1..x2 {
            img.put_pixel(x, y as u32, image::Rgb([0, 0, 0]));
            y += dy / dx;
        }
    } else {
        let mut x: f32 = x1 as f32;
        if y1 > y2 {
            x = x2 as f32;
            swap(&mut y1, &mut y2);
        }
        for y in y1..y2 {
            img.put_pixel(x as u32, y, image::Rgb([0, 0, 0]));
            x += dx / dy;
        }
    }
}
