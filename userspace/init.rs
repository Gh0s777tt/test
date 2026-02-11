mod shell;

use std::process::Command;

fn main() {
    // Best-effort Wraith mode setup. Keep booting even if host capabilities differ.
    match Command::new("mount")
        .args(["-t", "tmpfs", "tmpfs", "/"])
        .status()
    {
        Ok(status) if status.success() => {}
        Ok(_) => eprintln!("[VANTIS] tmpfs remount skipped"),
        Err(err) => eprintln!("[VANTIS] tmpfs remount unavailable: {err}"),
    }

    if let Err(err) = Command::new("swapoff").arg("-a").status() {
        eprintln!("[VANTIS] swapoff unavailable: {err}");
    }

    println!("[VANTIS] WRAITH MODE ACTIVE");
    shell::start();
}
