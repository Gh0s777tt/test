#!/bin/bash

# Create Cargo.toml files for all modules

# Userspace drivers
cat > userspace/drivers/direct_metal/Cargo.toml << 'TOML'
[package]
name = "vantis-direct-metal"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true
ash = { workspace = true, optional = true }
metal-rs = { workspace = true, optional = true }

[features]
default = ["hw-accel"]
hw-accel = []
vulkan = ["ash"]
metal = ["metal-rs"]
all-backends = ["vulkan", "metal"]
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/drivers/network/Cargo.toml << 'TOML'
[package]
name = "vantis-network"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

# Userspace security
cat > userspace/security/vault/Cargo.toml << 'TOML'
[package]
name = "vantis-vault"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true
aes.workspace = true
twofish.workspace = true
serpent.workspace = true
cbc.workspace = true
cipher.workspace = true
rand.workspace = true
rand_core.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/security/sentinel/Cargo.toml << 'TOML'
[package]
name = "vantis-sentinel"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/security/compliance/Cargo.toml << 'TOML'
[package]
name = "vantis-compliance"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

# Userspace AI
cat > userspace/ai/cortex_ai/Cargo.toml << 'TOML'
[package]
name = "vantis-cortex-ai"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/ai/semantic_search/Cargo.toml << 'TOML'
[package]
name = "vantis-semantic-search"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/ai/automation/Cargo.toml << 'TOML'
[package]
name = "vantis-automation"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

# Userspace multimedia
cat > userspace/multimedia/audio_mixer/Cargo.toml << 'TOML'
[package]
name = "vantis-audio-mixer"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/multimedia/babel_protocol/Cargo.toml << 'TOML'
[package]
name = "vantis-babel-protocol"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/multimedia/flux_engine/Cargo.toml << 'TOML'
[package]
name = "vantis-flux-engine"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true
ash = { workspace = true, optional = true }
metal-rs = { workspace = true, optional = true }

[features]
default = ["hw-accel"]
hw-accel = []
vulkan = ["ash"]
metal = ["metal-rs"]
all-backends = ["vulkan", "metal"]
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

# Userspace accessibility
cat > userspace/accessibility/spectrum_2_0/Cargo.toml << 'TOML'
[package]
name = "vantis-spectrum-2-0"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/accessibility/voice_assistant/Cargo.toml << 'TOML'
[package]
name = "vantis-voice-assistant"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/accessibility/bci_interface/Cargo.toml << 'TOML'
[package]
name = "vantis-bci-interface"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/accessibility/braille_display/Cargo.toml << 'TOML'
[package]
name = "vantis-braille-display"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/accessibility/haptic_language/Cargo.toml << 'TOML'
[package]
name = "vantis-haptic-language"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

# Userspace compatibility
cat > userspace/compatibility/vnt_apps/Cargo.toml << 'TOML'
[package]
name = "vantis-vnt-apps"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/compatibility/android_subsystem/Cargo.toml << 'TOML'
[package]
name = "vantis-android-subsystem"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/compatibility/legacy_airlock/Cargo.toml << 'TOML'
[package]
name = "vantis-legacy-airlock"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

# Userspace profiles
cat > userspace/profiles/profiles/Cargo.toml << 'TOML'
[package]
name = "vantis-profiles"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/profiles/interfaces/Cargo.toml << 'TOML'
[package]
name = "vantis-interfaces"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/profiles/permission_cards/Cargo.toml << 'TOML'
[package]
name = "vantis-permission-cards"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

# Userspace UI
cat > userspace/ui/flux/Cargo.toml << 'TOML'
[package]
name = "vantis-flux"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true
ash = { workspace = true, optional = true }
metal-rs = { workspace = true, optional = true }

[features]
default = ["hw-accel"]
hw-accel = []
vulkan = ["ash"]
metal = ["metal-rs"]
all-backends = ["vulkan", "metal"]
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

cat > userspace/ui/shells/Cargo.toml << 'TOML'
[package]
name = "vantis-shells"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
anyhow.workspace = true

[features]
default = ["hw-accel"]
hw-accel = []
verus = ["dep:builtin", "dep:builtin_macros", "dep:vstd"]
verus-full = ["verus"]
kani = []
TOML

echo "All Cargo.toml files generated successfully!"
