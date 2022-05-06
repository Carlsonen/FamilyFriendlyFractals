#[allow(unused_must_use)]

pub fn make_folder(name: &String) {
    let path = std::env::current_dir().unwrap().join(name);
    std::fs::create_dir(path);
}