mod shell;

use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

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

    if Path::new("/etc/fstab").exists() {
        if let Err(err) = Command::new("swapoff").arg("-a").status() {
            eprintln!("[VANTIS] swapoff unavailable: {err}");
        }
    }

    println!("[VANTIS] WRAITH MODE ACTIVE");
    loop {
        shell::start();
        eprintln!("[VANTIS] shell session ended, restarting...");
        thread::sleep(Duration::from_millis(500));
    }
}
