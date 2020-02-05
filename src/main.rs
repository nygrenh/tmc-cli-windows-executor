use std::env;
use std::env::current_exe;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::process::Child;
use std::process::Command;

fn main() {
    let current_executable: PathBuf =
        current_exe().expect("Error: could not find the tmc-cli install location.");
    let install_folder: &Path = current_executable
        .as_path()
        .parent()
        .expect("Error: could not find the tmc-cli install location.");
    let jar_path: PathBuf = install_folder.join("tmc-cli.jar");
    let cmd_args: Vec<String> = env::args().into_iter().skip(1).collect();
    let command_string = format!("java -jar {:?}", jar_path);

    let command;
    let first_arg;

    if cfg!(target_os = "windows") {
        command = "cmd";
        first_arg = "/C";
    } else {
        command = "sh";
        first_arg = "-c";
    };

    let mut handle: Child = Command::new(command)
        .arg(first_arg)
        .arg(command_string.as_str())
        .args(cmd_args)
        .spawn()
        .expect("Failed to execute tmc-cli");
    let result = handle.wait();

    // If there's an exit code, use it to exit.
    if let Ok(status) = result {
        if let Some(code) = status.code() {
            exit(code);
        }
    }
}
