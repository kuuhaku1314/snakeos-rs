use std::{
    process::Command,
};

fn main() {
// read env variables that were set in build script
    let bios_path = env!("BIOS_PATH");


    let mut cmd = Command::new("wsl");
    cmd
        .arg("qemu-system-x86_64")
        .arg("-drive")
        .arg(format!("format=raw,file={}", convert_windows_to_linux_path(bios_path)))
        .arg("-serial")
        .arg("stdio");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}

fn convert_windows_to_linux_path(windows_path: &str) -> String {
    // Replace backslashes with forward slashes
    let mut linux_path = windows_path.replace("\\", "/");

    // Handle drive letter (e.g., C:\ -> /mnt/c/)
    if let Some((drive, rest)) = linux_path.split_once(":") {
        linux_path = format!("/mnt/{}/{}", drive.to_lowercase(), rest.trim_start_matches('/'));
    }

    linux_path
}

