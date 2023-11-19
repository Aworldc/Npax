use platform_dirs::AppDirs;
use std::fs::create_dir_all;

pub struct Dirs {
    pub index_dir: String,
    pub config_dir: String,
    pub tarball_dir: String,
    pub extracted_dir: String,
}

impl Dirs {
    pub fn new() -> Dirs {
        let dirs = AppDirs::new(Some("name"), false).unwrap();

        return Dirs {
            index_dir: dirs.data_dir.join("index").to_str().unwrap().to_owned(),
            config_dir: dirs.data_dir.join("config").to_str().unwrap().to_owned(),
            tarball_dir: dirs.data_dir.join("tarballs").to_str().unwrap().to_owned(),
            extracted_dir: dirs.data_dir.join("extracted").to_str().unwrap().to_owned(),
        };
    }

    pub fn create(&self) -> Option<&Dirs> {
        let index = create_dir_all(&self.index_dir);
        let config = create_dir_all(&self.index_dir);
        let tarball = create_dir_all(&self.index_dir);
        let extracted = create_dir_all(&self.index_dir);

        match index {
            Ok(_) => match config {
                Ok(_) => match tarball {
                    Ok(_) => match extracted {
                        Ok(_) => Some(self),
                        Err(_) => None,
                    },
                    Err(_) => None,
                },
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}
