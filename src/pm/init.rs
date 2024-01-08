use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
};

pub fn create() {
    OpenOptions::new()
        .write(true)
        .create(true)
        .open("npax.toml")
        .unwrap()
        .write_all(include_bytes!("npax.default.toml"))
        .unwrap();

    create_dir_all("src").unwrap();

    OpenOptions::new()
        .write(true)
        .create(true)
        .open("src/main.js")
        .unwrap()
        .write_all(include_bytes!("main.default.js"))
        .unwrap();
}
