mod cwd;
mod git;
mod python;
mod utils;

use std::{env, io};

use clap::Parser;

#[derive(Parser)]
enum Args {
    Run {
        #[arg(allow_negative_numbers = true)]
        exit_status: String,
        #[arg(default_value_t = String::from("\n"))]
        new_line: String,
    },
    Init {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Clone, clap::ValueEnum)]
enum Shell {
    Bash,
    Nu,
}

fn main() -> io::Result<()> {
    match Args::parse() {
        Args::Run {
            exit_status,
            new_line,
        } => {
            run(&exit_status, &new_line)?;
        }
        Args::Init { shell } => {
            let shell_script = match shell {
                Shell::Bash => include_str!("shell/init.bash"),
                Shell::Nu => include_str!("shell/init.nu"),
            }
            .replace(
                "::SMPT::",
                &env::current_exe()?
                    .to_string_lossy()
                    .replace('\\', "\\\\"),
            );
            println!("{shell_script}");
        }
    }

    Ok(())
}

fn run(exit_status: &str, new_line: &str) -> io::Result<()> {
    let git = git::Git::new();
    let python = python::Python::new();
    let cwd = cwd::Cwd::new(git.repo_path.parent())?;

    print!(
        "{reset}{new_line}{prompt_color}┃ {reset}{cwd}{git}{python}{new_line}{prompt_color}┃ {reset}",
        reset = utils::RESET,
        prompt_color = if exit_status == "0" {
            utils::GREEN
        } else {
            utils::RED
        }
    );
    Ok(())
}
