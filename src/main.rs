mod cwd;
mod git;
mod python;
mod utils;

use std::{env, io};

fn main() -> io::Result<()> {
    let mut args = env::args();
    let exe = args.next().unwrap();
    match args.next().unwrap().as_str() {
        "run" => run(
            args.next().unwrap().as_str(),
            args.next().unwrap_or("\n".into()).as_str(),
        )?,
        "init" => {
            let shell_script = match args.next().unwrap().as_str() {
                "nu" => include_str!("shell/init.nu"),
                "bash" => include_str!("shell/init.bash"),
                shell => panic!("unknown shell: {shell}"),
            }
            .replace("::SMPT::", exe.as_str());
            println!("{shell_script}");
        }
        _ => panic!("unknown command"),
    }
    Ok(())
}

fn run(exit_status: &str, new_line: &str) -> io::Result<()> {
    let exit_status = exit_status == "0";

    let git = git::Git::new()?;
    let python = python::Python::new();
    let cwd = cwd::Cwd::new(git.repo_path.parent())?;

    print!(
        "{reset}{new_line}{prompt_color}┃ {reset}{cwd}{git}{python}{new_line}{prompt_color}┃ {reset}",
        reset = utils::RESET,
        prompt_color = if exit_status {
            utils::GREEN
        } else {
            utils::RED
        }
    );
    Ok(())
}
