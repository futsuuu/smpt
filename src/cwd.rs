use std::{
    env,
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

use home::home_dir;

use crate::utils;

pub struct Cwd {
    path: PathBuf,
    home: PathBuf,
    parent: PathBuf,
}

impl Display for Cwd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path.display().to_string().replace('\\', "/");
        let parent = self.parent.display().to_string().replace('\\', "/") + "/";
        let a = path
            .replace(
                parent.as_str(),
                format!("{}{}", parent, utils::BOLD).as_str(),
            )
            .replacen(
                self.home.display().to_string().replace('\\', "/").as_str(),
                "~",
                1,
            );
        write!(f, "{} {}", utils::BLUE, a)
    }
}

impl Cwd {
    pub fn new(parent: Option<&Path>) -> io::Result<Cwd> {
        let cwd = env::current_dir()?;
        let parent = parent.unwrap_or(cwd.parent().unwrap_or(Path::new("/")));
        Ok(Self {
            path: cwd.clone(),
            home: home_dir().unwrap_or_default(),
            parent: parent.to_path_buf(),
        })
    }
}
