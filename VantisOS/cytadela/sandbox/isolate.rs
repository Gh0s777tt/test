use super::caps::Capabilities;

pub fn apply(caps: Capabilities) {
    if !caps.net {
        // block network syscalls
    }
    if !caps.fs {
        // mount tmpfs only
    }
}
