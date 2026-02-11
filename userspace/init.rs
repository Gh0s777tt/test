mod shell;

use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let cmdline = fs::read_to_string("/proc/cmdline").unwrap_or_default();
    let installer_mode = cmdline
        .split_whitespace()
        .any(|token| token == "vantis.mode=installer");

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

    if installer_mode {
        println!("[VANTIS] INSTALLER MODE ACTIVE");
        println!("[VANTIS] To install: install /dev/vda --yes");
    }
    println!("[VANTIS] WRAITH MODE ACTIVE");
    loop {
        shell::start();
        eprintln!("[VANTIS] shell session ended, restarting...");
        thread::sleep(Duration::from_millis(500));
    }
}
