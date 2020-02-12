use std::env;
use std::env::current_exe;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::process::Child;
use std::process::Command;
use std::ffi::OsString;

fn main() {
    let current_executable: PathBuf =
        current_exe().expect("Error: could not find the tmc-cli install location.");
    let install_folder: &Path = current_executable
        .as_path()
        .parent()
        .expect("Error: could not find the tmc-cli install location.");
    let jar_path_string: PathBuf = install_folder.join("tmc-cli.jar");
    let jar_path: OsString = jar_path_string.into_os_string();
    let cmd_args: Vec<String> = env::args().into_iter().skip(1).collect();

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
        .arg("java")
        .arg("-jar")
        .arg(jar_path)
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
