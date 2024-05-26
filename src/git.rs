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
        match self
            .branch
            .split_once('/')
            .map(|t| t.0)
            .unwrap_or(&self.branch)
        {
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
        let head = repo.head().ok();
        let head_oid = head.as_ref().and_then(|h| h.target());
        let (Some(head), Some(head_oid)) = (head, head_oid) else {
            return Ok(Git {
                repo_path,
                branch: repo
                    .config()
                    .and_then(|c| c.get_string("init.defaultbranch"))
                    .unwrap_or(String::from("master")),
                ahead: 0,
                behind: 0,
            });
        };

        let branch = if head.is_branch() {
            head.shorthand()
                .unwrap_or("(HEAD name is not valid utf-8)")
                .to_string()
        } else {
            head_oid.to_string().split_at(7).0.to_string()
        };
        let (ahead, behind) = repo
            .revparse_ext("@{upstream}")
            .and_then(|(upstream, _)| repo.graph_ahead_behind(head_oid, upstream.id()))
            .unwrap_or_default();

        Ok(Git {
            repo_path,
            branch,
            ahead,
            behind,
        })
    }
}
