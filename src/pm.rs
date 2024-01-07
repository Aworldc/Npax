use std::{
    env::args,
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::Write,
    process::Command
};

use toml::{map::Map, Table, Value};

struct PackageInfo {
    pub name: String,
    pub version: String
}

fn run_script(scripts: Map<String, Value>, script: String) -> Option<i32> {
    match scripts.get(&script) {
        Some(script_data) => {
            println!("> Found script '{}'", script);
            match script_data.get("command") {
                Some(cmd) => {
                    match cmd {
                        Value::String(command) => {   
                            println!("> Running '{}'", command);

                            match if cfg!(target_os = "windows") {
                                Command::new("cmd")
                                    .args(["/C", command])
                                    .spawn()
                            } else {
                                Command::new("sh")
                                    .arg("-c")
                                    .arg(command)
                                    .spawn()
                            } {
                                Ok(mut proc) => {
                                    match proc.wait() {
                                        Ok(exitcode) => {
                                            match exitcode.code() {
                                                Some(code) => {
                                                    println!("> Script '{}' exited with code {}", script, code);
                                                    Some(code)
                                                },
                                                None => {
                                                    println!("> Script '{}' terminated", script );
                                                    None
                                                },
                                            }
                                        },
                                        Err(_) => {
                                            println!("> Failed to start process");
                                            None
                                        },
                                    } 
                                },
                                Err(err) => {
                                    println!("{}", err.to_string());
                                    None
                                },
                            }
                        },
                        _ => {
                            println!("[Npax] 'command' property of script '{}' is not a string. Have you made a mistake?", script);
                            None
                        }
                    }
                },
                None => {
                    println!("[Npax] Failed to get 'command' property of script '{}'. Does it exist?", script);
                    None
                },
            }
        },
        None => {
            println!("[Npax] Failed to get script '{}' from npax.toml. Does it exist?", script);
            None
        },
    }
}

fn validate_package_section(config: Table) -> Option<PackageInfo> {
    match config.get("package") {
        Some(package) => {
            match package {
                Value::Table(_) => {
                    match package.get("name") {
                        Some(name) => {
                            match name {
                                Value::String(name) => {
                                    match package.get("version") {
                                        Some(version) => {
                                            match version {
                                                Value::String(version) => Some(PackageInfo {
                                                    name: name.clone(),
                                                    version: version.clone(),
                                                }),
                                                _ => {
                                                    println!("[Npax] Property 'version of package is not a string. Have you made a typo?");
                                                    None
                                                }
                                            }
                                        },
                                        None => {
                                            println!("[Npax] Property 'version' of package does not exist. Your package needs a version.");
                                            None
                                        },
                                    }
                                }
                                _ => {
                                    println!("[Npax] Property 'name' of package is not a string. Have you made a typo?");
                                    None
                                }
                            }
                        },
                        None => {
                            println!("[Npax] Property 'name' of package does not exist. Name ur damn package.");
                            None
                        },
                    }
                },
                _ => {
                    println!("[Npax] Property 'package' of npax.toml is not a table. Have you made a typo?");
                    None
                }
            }
        },
        None => {
            println!("[Npax] Failed to get property 'package' of npax.toml. Does it exist?");
            None
        }
    }
}

pub fn exec() {
    match read_to_string("npax.toml") {
        Ok(data) => match data.parse::<Table>() {
            Ok(config) => {
                match validate_package_section(config.clone()) {
                    Some(package) => {
                        println!("> Found version {} of package '{}'", package.version, package.name);

                        let args = args().collect::<Vec<String>>();
                        let script;
        
                        if args.len() > 2 {
                            script = args[2].clone()
                        } else {
                            script = "main".to_owned()
                        }
        
                        match config.get("scripts") {
                            Some(scripts) => match scripts {
                                toml::Value::Table(scripts) => {
                                    run_script(scripts.clone(), script);
                                },
                                _ => {
                                    println!("[Npax] Property 'scripts' of npax.toml is not a table. Have you made a typo?")
                                }
                            },
                            None => {
                                println!("[Npax] Failed to get property 'scripts' of npax.toml. Does it exist?")
                            }
                        }
                    },
                    None => {},
                }
            }
            Err(_) => println!("[Npax] Failed to parse npax.toml. Is it valid toml?"),
        },
        Err(_) => {
            println!("[Npax] Failed to open npax.toml. Does it exist?")
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
