pub struct Config {
    pub coloring: Coloring,
    pub shape: Shape,
    pub screen: Screen,
}
impl Config {
    pub fn new(coloring: Coloring, shape: Shape, screen: Screen) -> Self {
        Config {
            coloring: coloring,
            shape: shape,
            screen: screen,
        }
    }
}
pub struct Coloring {
    pub min: f64,
    pub max: f64,
}
impl Coloring {
    pub fn new(min: f64, max: f64) -> Self {
        Coloring { min: min, max: max }
    }
    pub fn gray() -> Self {
        Coloring { min: 2.0, max: 2.2 }
    }
    pub fn default() -> Self {
        Coloring { min: 0.0, max: 2.0 }
    }
    pub fn dark() -> Self {
        Coloring { min: 0.5, max: 1.0 }
    }
    pub fn random() -> Self {
        Coloring { min: 0.0, max: 9.9 }
    }
}

pub struct Shape {
    pub messiness_factor: i32,
    pub iterations: i32,
}
impl Shape {
    pub fn new(messiness_factor: i32, iterations: i32) -> Self {
        Shape {
            messiness_factor: messiness_factor,
            iterations: iterations,
        }
    }
    pub fn default() -> Self {
        Shape {
            messiness_factor: 30,
            iterations: 10000,
        }
    }
}

pub struct Screen {
    pub width: u32,
    pub height: u32,
}
impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        Screen {
            width: width,
            height: height,
        }
    }
    pub fn default() -> Self {
        Screen {
            width: 1024 * 3,
            height: 1024 * 3,
        }
    }
    pub fn macbook() -> Self {
        Screen {
            width: 2560 * 2,
            height: 1600 * 2,
        }
    }
    pub fn pc() -> Self {
        Screen {
            width: 1920 * 2,
            height: 1080 * 2,
        }
    }
}
