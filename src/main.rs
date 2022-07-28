use rand::{distributions::Alphanumeric, Rng};
use text_io::read;

mod wave_function;
use family_friendly_fractals::coloring::*;
use family_friendly_fractals::fractal_engine::*;

fn print_available_commands() {
    use owo_colors::OwoColorize;
    println!("========================================================");
    println!("{}", "available commands:".green());
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
    println!("orbit hue <value>                | set hue difference (0-359)");
    println!();
    println!("gpu make <name>                  | generates fractal on GPU");
    println!("gpu r                            | generates random on GPU");
    println!();
    println!("stop                             | stops the program");
    println!("========================================================");
}
enum ImgType {
    Fractal,
    ImgFractal,
}
fn main() {
    let mut fractal_engine = FractalEngine::new();
    let mut img_type = ImgType::Fractal;
    match fractal_engine.set_sample_path(&String::from("sample/amogus.png")) {
        Ok(_) => {}
        Err(_) => {
            println!("No default sample image found")
        }
    }
    print_available_commands();
    loop {
        println!();
        //println!("input command: ");
        let command: String = read!();
        if command == "stop" {
            println!("program stopped");
            break;
        } else if command == "make" {
            let name: String = read!();
            fractal_engine.set_seed(&name);
            match img_type {
                ImgType::Fractal => fractal_engine.gen_julia(&name),
                ImgType::ImgFractal => fractal_engine.gen_image_fractal(&name),
            }
            println!("=> Fractal \"{}\" saved!", name);
        } else if command == "r" {
            let seed: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect();
            fractal_engine.set_seed(&seed);
            match img_type {
                ImgType::Fractal => fractal_engine.gen_julia(&String::from("random")),
                ImgType::ImgFractal => fractal_engine.gen_image_fractal(&String::from("random")),
            }
            println!("=> Fractal \"{}\" saved!", seed);
        } else if command == "set" {
            let func: String = read!();
            match func.as_str() {
                "orbit" => img_type = ImgType::ImgFractal,
                &_ => img_type = ImgType::Fractal,
            }
        } else if command == "orbit" {
            let func: String = read!();
            if func == "set" {
                let pic: String = read!();
                let path = format!("sample/{}", pic);
                match fractal_engine.set_sample_path(&path) {
                    Ok(_) => println!("=> Sample picture set to {}", pic),
                    Err(_) => println!("=> the file \"{}\" does not exist", path),
                }
            } else if func == "hue" {
                let hue: u32 = read!();
                println!(
                    "=> Orbit hue set to {}",
                    fractal_engine.set_image_fractal_hue(hue)
                );
            } else {
                println!("=> invalid command");
            }
        } else if command == "wave" {
            let name: String = read!();
            //let mut wave = wave_function::WaveWorld::new(128, 128, &name);
            //wave.evaluate();
            //wave.capture();
            let mut wave = wave_function::WaveSprite::new(16, 16, 24, &name);
            wave.make_img();
        } else if command == "noise" {
            wave_function::noise(256, 256);
        } else if command == "color" {
            let color: String = read!();
            match color.as_str() {
                "dark" => {
                    fractal_engine.set_coloring(ColoringMethod::Dark);
                    println!("=> color set to dark")
                }
                "gray" => {
                    fractal_engine.set_coloring(ColoringMethod::Gray);
                    println!("=> color set to gray")
                }
                "colorful" => {
                    fractal_engine.set_coloring(ColoringMethod::Colorful);
                    println!("=> color set to colorful")
                }
                "rainbow" => {
                    fractal_engine.set_coloring(ColoringMethod::Rainbow);
                    println!("=> color set to rainbow")
                }
                &_ => {
                    fractal_engine.set_coloring(ColoringMethod::Default);
                    println!("=> color set to default")
                }
            }
        } else if command == "shape" {
            let messi: i32 = read!();
            fractal_engine.set_shape(messi);
            println!("=> messiness set to {}", messi);
        } else if command == "res" {
            let width: u32 = read!();
            let height: u32 = read!();
            println!(
                "=> screen set to {:?}",
                fractal_engine.set_res(width, height)
            );
        } else if command == "zoom" {
            let zoom: f64 = read!();
            fractal_engine.set_zoom(zoom);
            println!("=> Zoom set to {}", zoom);
        } else {
            print_available_commands();
        }
    }
}
