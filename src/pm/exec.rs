use std::{
    env::args,
    fs::read_to_string,
    process::Command,
};

use toml::{map::Map, Table, Value};

use super::parser;

use super::util;

fn run_script(scripts: Map<String, Value>, script: String) -> Option<i32> {
    match scripts.get(&script) {
        Some(script_data) => {
            println!("> Found script '{}'", script);

            let deps_result = match script_data.get("dependencies") {
                Some(deps) => match deps {
                    Value::Array(dependencies) => {
                        if dependencies
                            .iter()
                            .filter(|item| match item {
                                Value::String(_) => false,
                                _ => true,
                            })
                            .count() > 0 {
                            println!("[Npax] Property 'dependencies' of script '{}' contains values that are not strings. Have you made a mistake?", script);
                            Err(())
                        } else {
                            for dep in dependencies {
                                match dep {
                                    Value::String(dep_string) => {
                                        let parsed_dep_string = parser::parse_dep_string(dep_string.to_string());

                                        dbg!(parsed_dep_string);
                                    },
                                    _ => panic!()
                                }
                            }

                            Ok(())
                        }
                    }
                    _ => {
                        println!("[Npax] Property 'dependencies' of script '{}' is not an array. Have you made a mistake?", script);
                        Err(())
                    }
                },
                None => {
                    println!("> Script '{}' has no dependencies", script);
                    Ok(())
                }
            };

            match deps_result {
                Ok(_) => {
                    println!("> Dependencies of script '{}' have been satisfied", script);
                    
                    match script_data.get("command") {
                        Some(cmd) => match cmd {
                            Value::String(command) => {
                                println!("> Running '{}'", command);
        
                                match if cfg!(target_os = "windows") {
                                    Command::new("cmd").args(["/C", command]).spawn()
                                } else {
                                    Command::new("sh").arg("-c").arg(command).spawn()
                                } {
                                    Ok(mut proc) => match proc.wait() {
                                        Ok(exitcode) => match exitcode.code() {
                                            Some(code) => {
                                                println!("> Script '{}' exited with code {}", script, code);
                                                Some(code)
                                            }
                                            None => {
                                                println!("> Script '{}' terminated", script);
                                                None
                                            }
                                        },
                                        Err(_) => {
                                            println!("> Failed to start process");
                                            None
                                        }
                                    },
                                    Err(err) => {
                                        println!("{}", err.to_string());
                                        None
                                    }
                                }
                            }
                            _ => {
                                println!("[Npax] Property 'command' of script '{}' is not a string. Have you made a mistake?", script);
                                None
                            }
                        },
                        None => {
                            println!(
                                "[Npax] Failed to get 'command' property of script '{}'. Does it exist?",
                                script
                            );
                            None
                        }
                    }
                },
                Err(_) => {
                    println!("> Failed to satisfy dependencies of script '{}'", script);
                    None
                },
            }
        }
        None => {
            println!(
                "[Npax] Failed to get script '{}' from npax.toml. Does it exist?",
                script
            );
            None
        }
    }
}

pub fn exec() {
    match read_to_string("npax.toml") {
        Ok(data) => match data.parse::<Table>() {
            Ok(config) => match util::validate_package_section(config.clone()) {
                Some(package) => {
                    println!(
                        "> Found version {} of package '{}'",
                        package.version, package.name
                    );

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
                            }
                            _ => {
                                println!("[Npax] Property 'scripts' of npax.toml is not a table. Have you made a typo?")
                            }
                        },
                        None => {
                            println!("[Npax] Failed to get property 'scripts' of npax.toml. Does it exist?")
                        }
                    }
                }
                None => {}
            },
            Err(_) => println!("[Npax] Failed to parse npax.toml. Is it valid toml?"),
        },
        Err(_) => {
            println!("[Npax] Failed to open npax.toml. Does it exist?")
        }
    }
}
