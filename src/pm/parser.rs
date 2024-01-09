#[cfg(test)]
mod tests {
    use crate::pm::parser::{
        parse_dep_string, DepInfo, Dependency, DependencyType, DependencyVersion, GitRepo, Runtime,
    };

    #[test]
    fn it_recognises_local_script_dependencies() {
        assert_eq!(
            parse_dep_string("build".to_owned()),
            Some(DepInfo {
                package: Dependency::This,
                item: DependencyType::Script("build".to_owned()),
                version: DependencyVersion::Any
            })
        );

        assert_eq!(
            parse_dep_string("main".to_owned()),
            Some(DepInfo {
                package: Dependency::This,
                item: DependencyType::Script("main".to_owned()),
                version: DependencyVersion::Any
            })
        );
    }

    #[test]
    fn it_recognises_npm_deps() {
        assert_eq!(
            parse_dep_string("express#npm".to_owned()),
            Some(DepInfo {
                package: Dependency::Npm("express".to_owned()),
                item: DependencyType::Module,
                version: DependencyVersion::Any
            })
        );

        assert_eq!(
            parse_dep_string("@material/web#npm".to_owned()),
            Some(DepInfo {
                package: Dependency::Npm("@material/web".to_owned()),
                item: DependencyType::Module,
                version: DependencyVersion::Any
            })
        );
    }

    #[test]
    fn it_recognises_major_versions() {
        assert_eq!(
            parse_dep_string("express=4#npm".to_owned()),
            Some(DepInfo {
                package: Dependency::Npm("express".to_owned()),
                item: DependencyType::Module,
                version: DependencyVersion::Major(4)
            })
        );
    }

    #[test]
    fn it_recognises_specific_versions() {
        assert_eq!(
            parse_dep_string("express=4.18.2#npm".to_owned()),
            Some(DepInfo {
                package: Dependency::Npm("express".to_owned()),
                item: DependencyType::Module,
                version: DependencyVersion::Specific("4.18.2".to_owned())
            })
        );
    }

    #[test]
    fn it_recognises_pypi_deps() {
        assert_eq!(
            parse_dep_string("tqdm#pypi".to_owned()),
            Some(DepInfo {
                package: Dependency::PyPi("tqdm".to_owned()),
                item: DependencyType::Module,
                version: DependencyVersion::Any
            })
        );
    }

    #[test]
    fn it_recognises_subpackage_deps() {
        assert_eq!(
            parse_dep_string("thing#sub".to_owned()),
            Some(DepInfo {
                package: Dependency::SubPackage("thing".to_owned()),
                item: DependencyType::Module,
                version: DependencyVersion::Any
            })
        );

        assert_eq!(
            parse_dep_string("subpackage#sub".to_owned()),
            Some(DepInfo {
                package: Dependency::SubPackage("subpackage".to_owned()),
                item: DependencyType::Module,
                version: DependencyVersion::Any
            })
        );
    }

    #[test]
    fn it_recognises_subpackage_scripts() {
        assert_eq!(
            parse_dep_string("thing:build#sub".to_owned()),
            Some(DepInfo {
                package: Dependency::SubPackage("thing".to_owned()),
                item: DependencyType::Script("build".to_owned()),
                version: DependencyVersion::Any
            })
        );

        assert_eq!(
            parse_dep_string("thing:dosomething#sub".to_owned()),
            Some(DepInfo {
                package: Dependency::SubPackage("thing".to_owned()),
                item: DependencyType::Script("dosomething".to_owned()),
                version: DependencyVersion::Any
            })
        );
    }

    #[test]
    fn it_recognises_git_deps() {
        assert_eq!(
            parse_dep_string("Aworldc/Something#git:github.com".to_owned()),
            Some(DepInfo {
                package: Dependency::Git(GitRepo {
                    base: "github.com".to_owned(),
                    repo: "Aworldc/Something".to_owned()
                }),
                item: DependencyType::Module,
                version: DependencyVersion::Any
            })
        );
    }

    #[test]
    fn it_recognises_runtime_deps() {
        assert_eq!(
            parse_dep_string("node#runtime".to_owned()),
            Some(DepInfo {
                package: Dependency::Runtime(Runtime::Node),
                item: DependencyType::Module,
                version: DependencyVersion::Any
            })
        );

        assert_eq!(
            parse_dep_string("python#runtime".to_owned()),
            Some(DepInfo {
                package: Dependency::Runtime(Runtime::Python),
                item: DependencyType::Module,
                version: DependencyVersion::Any
            })
        );

        assert_eq!(
            parse_dep_string("node=18#runtime".to_owned()),
            Some(DepInfo {
                package: Dependency::Runtime(Runtime::Node),
                item: DependencyType::Module,
                version: DependencyVersion::Major(18)
            })
        );
    }

    #[test]
    fn it_returns_none_for_invalid_depstrings() {
        assert_eq!(parse_dep_string("example#yomomma".to_owned()), None);
        assert_eq!(parse_dep_string("express##npm".to_owned()), None);
        assert_eq!(parse_dep_string("express=4=3#npm".to_owned()), None);
        assert_eq!(parse_dep_string("#".to_owned()), None);
    }
}

#[derive(Debug, PartialEq)]
pub struct DepInfo {
    pub package: Dependency,
    pub item: DependencyType,
    pub version: DependencyVersion,
}

#[derive(Debug, PartialEq)]
pub enum Dependency {
    Npm(String),
    PyPi(String),
    SubPackage(String),
    This,
    Git(GitRepo),
    Runtime(Runtime),
}

#[derive(Debug, PartialEq)]
pub enum DependencyType {
    Module,
    Script(String),
}

#[derive(Debug, PartialEq)]
pub enum DependencyVersion {
    Any,
    Major(u32),
    Specific(String),
}

#[derive(Debug, PartialEq)]
pub struct GitRepo {
    pub base: String,
    pub repo: String,
}

#[derive(Debug, PartialEq)]
pub enum Runtime {
    Node,
    Python,
}

pub fn parse_dep_string(dep_string: String) -> Option<DepInfo> {
    let string = dep_string.split("#").collect::<Vec<&str>>();

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
                            Some(dep) => Some(DepInfo {
                                package: dep,
                                item,
                                version,
                            }),
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
