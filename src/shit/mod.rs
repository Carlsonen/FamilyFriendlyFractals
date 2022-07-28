#[allow(unused_must_use)]

pub fn make_folder(name: &String) {
    let path = std::env::current_exe().unwrap().join(name);
    std::fs::create_dir(path);
}
