mod shell;

use std::process::Command;

fn main() {
    // Wraith Mode
    Command::new("mount")
        .args(&["-t", "tmpfs", "tmpfs", "/"])
        .status()
        .expect("tmpfs failed");

    Command::new("swapoff").arg("-a").status().ok();

    println!("[VANTIS] WRAITH MODE ACTIVE");
    shell::start();
}
