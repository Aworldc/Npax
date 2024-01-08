#[derive(Debug)]
pub struct DepInfo {
    pub package: Dependency,
    pub item: DependencyType,
    pub version: DependencyVersion,
}

#[derive(Debug)]
pub enum Dependency {
    Npm(String),
    PyPi(String),
    SubPackage(String),
    This,
    Git(GitRepo),
    Runtime(Runtime)
}

#[derive(Debug)]
pub enum DependencyType {
    Module,
    Script(String),
}

#[derive(Debug)]
pub enum DependencyVersion {
    Any,
    Major(u32),
    Specific(String),
}

#[derive(Debug)]
pub struct GitRepo {
    pub base: String,
    pub repo: String
}

#[derive(Debug)]
pub enum Runtime {
    Node,
    Python
}

pub fn parse_dep_string(dep_string: String) -> Option<DepInfo> {
    let string = dep_string.split("@").collect::<Vec<&str>>();

    if string.len() == 1 {
        Some(DepInfo {
            package: Dependency::This,
            item: DependencyType::Script(string[0].to_owned()),
            version: DependencyVersion::Any,
        })
    } else if string.len() == 2 {
        let version_seperated = string[0].split('=').collect::<Vec<&str>>();

        let version = if version_seperated.len() == 1 {
            Some(DependencyVersion::Any)
        } else if version_seperated.len() == 2 {
            match version_seperated[1].parse::<u32>() {
                Ok(number) => Some(DependencyVersion::Major(number)),
                Err(_) => Some(DependencyVersion::Specific(version_seperated[1].to_owned())),
            }
        } else {
            None
        };

        match version {
            Some(version) => {
                let script_seperated = version_seperated[0].split(':').collect::<Vec<&str>>();
                let item;

                if script_seperated.len() == 1 {
                    item = Some(DependencyType::Module);
                } else if script_seperated.len() == 2 {
                    item = Some(DependencyType::Script(script_seperated[1].to_owned()));
                } else {
                    item = None;
                }

                match item {
                    Some(item) => {
                        let first_bit = script_seperated[0];
                        let second_bit = string[1];

                        let dependency = if second_bit == "npm" {
                            Some(Dependency::Npm(first_bit.to_owned()))
                        } else if second_bit == "pypi" {
                            Some(Dependency::PyPi(first_bit.to_owned()))
                        } else if second_bit == "sub" {
                            Some(Dependency::SubPackage(first_bit.to_owned()))
                        } else if second_bit.starts_with("git") {
                            let git_splitted = second_bit.split(":").collect::<Vec<&str>>();

                            if git_splitted.len() == 2 {
                                Some(Dependency::Git(GitRepo {
                                    base: git_splitted[1].to_owned(),
                                    repo: first_bit.to_owned(),
                                }))
                            } else {
                                None
                            }
                        } else if second_bit == "runtime" {
                            if first_bit == "node" {
                                Some(Dependency::Runtime(Runtime::Node))
                            } else if first_bit == "python" {
                                Some(Dependency::Runtime(Runtime::Python))
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        match dependency {
                            Some(dep) => Some(DepInfo { package: dep, item, version }),
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            None => None,
        }
    } else {
        None
    }
}
