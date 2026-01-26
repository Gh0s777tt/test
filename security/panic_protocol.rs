pub fn panic_nuke() {
    zeroize_keys();
    flush_cpu_caches();
    disable_persistent_storage();
    reboot();
}

fn zeroize_keys() {
    // overwrite memory regions holding secrets
}
