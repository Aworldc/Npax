use toml::{Table, Value};

pub struct PackageInfo {
    pub name: String,
    pub version: String,
}

pub fn validate_package_section(config: Table) -> Option<PackageInfo> {
    match config.get("package") {
        Some(package) => {
            match package {
                Value::Table(_) => match package.get("name") {
                    Some(name) => match name {
                        Value::String(name) => match package.get("version") {
                            Some(version) => match version {
                                Value::String(version) => Some(PackageInfo {
                                    name: name.clone(),
                                    version: version.clone(),
                                }),
                                _ => {
                                    println!("[Npax] Property 'version of package is not a string. Have you made a typo?");
                                    None
                                }
                            },
                            None => {
                                println!("[Npax] Property 'version' of package does not exist. Your package needs a version.");
                                None
                            }
                        },
                        _ => {
                            println!("[Npax] Property 'name' of package is not a string. Have you made a typo?");
                            None
                        }
                    },
                    None => {
                        println!("[Npax] Property 'name' of package does not exist. Name ur damn package.");
                        None
                    }
                },
                _ => {
                    println!("[Npax] Property 'package' of npax.toml is not a table. Have you made a typo?");
                    None
                }
            }
        }
        None => {
            println!("[Npax] Failed to get property 'package' of npax.toml. Does it exist?");
            None
        }
    }
}
