use std::process::Command;

pub fn enter_wraith() {
    // rootfs = tmpfs
    Command::new("mount")
        .args(&["-t", "tmpfs", "tmpfs", "/"])
        .status()
        .expect("tmpfs mount failed");

    // disable swap
    let _ = Command::new("swapoff").arg("-a").status();
}
