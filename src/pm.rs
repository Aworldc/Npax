use std::{
    env::args,
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::Write,
};

use toml::{map::Map, Table, Value};

fn run_script(scripts: Map<String, Value>, script: String) {
    match scripts.get(&script) {
        Some(script_data) => {
            dbg!(script_data);
        },
        None => {
            println!("Failed to get script '{}' from npax.toml. Does it exist?", script)
        },
    }
}

pub fn exec() {
    match read_to_string("npax.toml") {
        Ok(data) => match data.parse::<Table>() {
            Ok(config) => {
                let args = args().collect::<Vec<String>>();
                let script;

                if args.len() > 2 {
                    script = args[2].clone()
                } else {
                    script = "main".to_owned()
                }

                match config.get("scripts") {
                    Some(scripts) => match scripts {
                        toml::Value::Table(scripts) => run_script(scripts.clone(), script),
                        _ => {
                            println!("Property 'scripts' of npax.toml is not a table. Have you made a typo?")
                        }
                    },
                    None => {
                        println!("Failed to get property 'scripts' of npax.toml. Does it exist?")
                    }
                }
            }
            Err(_) => println!("Failed to parse npax.toml. Is it valid toml?"),
        },
        Err(_) => {
            println!("Failed to open npax.toml. Does it exist?")
        }
    }
}

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

pub fn global_exec() {}
