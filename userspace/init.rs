mod shell;

use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

fn parse_cmdline() -> (String, String) {
    let cmdline = fs::read_to_string("/proc/cmdline").unwrap_or_default();
    let mut mode = String::from("live");
    let mut persist = String::new();

    for token in cmdline.split_whitespace() {
        if let Some(value) = token.strip_prefix("vantis.mode=") {
            mode = value.to_string();
        } else if let Some(value) = token.strip_prefix("vantis.persist=") {
            persist = value.to_string();
        }
    }

    (mode, persist)
}

fn mount_persistent_storage(persist_spec: &str) -> bool {
    if persist_spec.is_empty() {
        return false;
    }

    let persist_dev = match Command::new("findfs").arg(persist_spec).output() {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => String::new(),
    };
    if persist_dev.is_empty() {
        eprintln!("[VANTIS] persistent target not found: {persist_spec}");
        return false;
    }

    let _ = fs::create_dir_all("/persist");
    let _ = fs::create_dir_all("/home");
    let _ = fs::create_dir_all("/var");

    let auto_mount = Command::new("mount")
        .args([persist_dev.as_str(), "/persist"])
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    let vfat_mount = if auto_mount {
        true
    } else {
        Command::new("mount")
            .args(["-t", "vfat", persist_dev.as_str(), "/persist"])
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    };
    let ext_mount = if auto_mount || vfat_mount {
        true
    } else {
        Command::new("mount")
            .args(["-t", "ext4", persist_dev.as_str(), "/persist"])
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    };
    if !(auto_mount || vfat_mount || ext_mount) {
        eprintln!("[VANTIS] failed to mount persistent device: {persist_dev}");
        return false;
    }

    let _ = fs::create_dir_all("/persist/home");
    let _ = fs::create_dir_all("/persist/var");
    let _ = fs::create_dir_all("/persist/vantis");

    let home_bind = Command::new("mount")
        .args(["--bind", "/persist/home", "/home"])
        .status();
    if home_bind.as_ref().map(|s| s.success()).unwrap_or(false) {
        let _ = Command::new("mount")
            .args(["--bind", "/persist/var", "/var"])
            .status();
        println!("[VANTIS] persistent storage active: {persist_dev}");
        true
    } else {
        eprintln!("[VANTIS] failed to bind persistent home/var mounts");
        false
    }
}

fn run_first_boot_setup(persistent_active: bool) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let hostname = format!("vantis-{suffix:05}", suffix = now % 100_000);
    let profile = format!(
        "profile=core\nuser=vantis\nhostname={hostname}\nmode=installed\nfirst_boot_unix_utc={now}\nnotes=generated_by_vantis_first_boot\n"
    );
    let welcome = format!(
        "Welcome to VantisOS installed mode.\nFirst boot unix timestamp: {now}\nUser: vantis\nHost: {hostname}\n"
    );

    let _ = fs::create_dir_all("/home");
    let _ = fs::write("/home/.vantis_system_profile.conf", &profile);
    let _ = fs::write("/home/.vantis_welcome.txt", welcome);
    let _ = fs::write("/home/.vantis_first_boot_done", format!("completed_unix_utc={now}\n"));

    if persistent_active {
        let _ = fs::create_dir_all("/persist/vantis");
        let _ = fs::write("/persist/vantis/system_profile.conf", profile);
        let _ = fs::write("/persist/vantis/first_boot_done", format!("completed_unix_utc={now}\n"));
    }

    println!("[VANTIS] FIRST BOOT SETUP COMPLETE");
}

fn main() {
    let (mode, mut persist_spec) = parse_cmdline();
    let installer_mode = mode == "installer";
    let installed_mode = mode == "installed";

    if installed_mode && persist_spec.is_empty() {
        persist_spec = String::from("LABEL=VANTIS_DATA");
    }

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

    let mut persistent_active = mount_persistent_storage(&persist_spec);
    if installed_mode && !persistent_active && persist_spec != "LABEL=VANTIS_BOOT" {
        persistent_active = mount_persistent_storage("LABEL=VANTIS_BOOT");
        if persistent_active {
            println!("[VANTIS] persistent fallback active: LABEL=VANTIS_BOOT");
        }
    }

    if installer_mode {
        println!("[VANTIS] INSTALLER MODE ACTIVE");
        println!("[VANTIS] To install: install /dev/vda --yes");
    }

    if installed_mode {
        let _ = fs::create_dir_all("/home");
        if Path::new("/home/.vantis_first_boot_done").exists() {
            println!("[VANTIS] FIRST BOOT SETUP ALREADY COMPLETE");
        } else {
            run_first_boot_setup(persistent_active);
        }
        if !persistent_active {
            println!("[VANTIS] persistent storage unavailable; setup is volatile");
        }
    }

    println!("[VANTIS] WRAITH MODE ACTIVE");
    loop {
        shell::start();
        eprintln!("[VANTIS] shell session ended, restarting...");
        thread::sleep(Duration::from_millis(500));
    }
}
