use std::{fmt::Display, io, path::PathBuf};

use git2::Repository;

use crate::utils;

#[derive(Default)]
pub struct Git {
    pub repo_path: PathBuf,
    pub branch: String,
    pub ahead: usize,
    pub behind: usize,
}

impl Display for Git {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.branch.is_empty() {
            return Ok(());
        }

        f.write_str(utils::RESET)?;
        f.write_str(" ⠶ ")?;
        match self.branch.split_once('/').map(|t| t.0).unwrap_or(&self.branch) {
            "src" | "main" | "master" => write!(f, "{} ", utils::YELLOW)?,
            "dev" | "develop" => write!(f, "{} ", utils::MAGENTA)?,
            "feat" => write!(f, "{} ", utils::CYAN)?,
            "fix" => write!(f, "{} ", utils::RED)?,
            "release" => write!(f, "{} ", utils::GREEN)?,
            _ => write!(f, "{} ", utils::CYAN)?,
        };
        f.write_str(&self.branch)?;

        if self.ahead == 0 && self.behind == 0 {
            return Ok(());
        }
        f.write_str(" ")?;

        if self.ahead != 0 {
            write!(
                f,
                "{}{}{}",
                utils::RESET,
                utils::small_number(self.ahead, false),
                utils::RED,
            )?;
        }
        if self.behind != 0 {
            write!(
                f,
                "{}{}{}",
                utils::RED,
                utils::RESET,
                utils::small_number(self.behind, true),
            )?;
        }

        Ok(())
    }
}

impl Git {
    pub fn new() -> io::Result<Self> {
        let Ok(repo) = Repository::discover(".") else {
            return Ok(Git::default());
        };

        let repo_path = repo.workdir().unwrap_or_else(|| repo.path()).to_path_buf();
        let head_ref = repo.head().unwrap();
        let branch = head_ref
            .shorthand()
            .unwrap_or_else(|| head_ref.name().expect("HEAD name is not valid utf-8"))
            .to_string();

        let git = match repo.head() {
            Ok(head) => {
                let head_oid = head.target().unwrap();
                let (ahead, behind) = repo
                    .revparse_ext("@{upstream}")
                    .ok()
                    .and_then(|(upstream, _)| repo.graph_ahead_behind(head_oid, upstream.id()).ok())
                    .unwrap_or_default();
                Git {
                    repo_path,
                    branch,
                    ahead,
                    behind,
                }
            }
            Err(_) => Git {
                repo_path,
                ahead: 0,
                behind: 0,
                branch,
            },
        };

        Ok(git)
    }
}
