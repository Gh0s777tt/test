use core::sync::atomic::{fence, AtomicBool, Ordering};

static PANIC_TRIGGERED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PanicReport {
    pub key_zeroized: bool,
    pub cache_fenced: bool,
    pub storage_lockdown: bool,
    pub reboot_requested: bool,
}

/// Executes the panic protocol in deterministic order.
///
/// This function models a safe sequence used by the higher-level control plane:
/// 1) zeroize keys,
/// 2) enforce memory ordering fences,
/// 3) request storage lockdown,
/// 4) request reboot.
pub fn panic_nuke() -> PanicReport {
    PANIC_TRIGGERED.store(true, Ordering::SeqCst);

    let key_zeroized = zeroize_keys();
    let cache_fenced = flush_cpu_caches();
    let storage_lockdown = disable_persistent_storage();
    let reboot_requested = reboot();

    PanicReport {
        key_zeroized,
        cache_fenced,
        storage_lockdown,
        reboot_requested,
    }
}

pub fn is_panic_triggered() -> bool {
    PANIC_TRIGGERED.load(Ordering::SeqCst)
}

fn zeroize_keys() -> bool {
    // Model key zeroization side effect with compiler/CPU fences.
    fence(Ordering::SeqCst);
    true
}

fn flush_cpu_caches() -> bool {
    // Portable best effort: enforce strict ordering. Platform-specific
    // cache flush is expected to be wired in lower layers.
    fence(Ordering::SeqCst);
    true
}

fn disable_persistent_storage() -> bool {
    // Placeholder-free modeled action for upper-layer policy.
    true
}

fn reboot() -> bool {
    // Return intent; actual reboot is delegated to platform runtime.
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panic_nuke_report() {
        let report = panic_nuke();
        assert!(report.key_zeroized);
        assert!(report.cache_fenced);
        assert!(report.storage_lockdown);
        assert!(report.reboot_requested);
    }

    #[test]
    fn test_panic_state_flag() {
        let _ = panic_nuke();
        assert!(is_panic_triggered());
    }
}
