pub mod vnt;
pub mod sandbox;
pub mod legacy;

#[cfg(test)]
mod tests {
    use super::sandbox::caps::Capabilities;
    use super::vnt::manifest::VntManifest;

    #[test]
    fn test_manifest_deserialization() {
        let raw = r#"
name = "demo-app"
version = "1.0.0"
entry = "main.wasm"

[permissions]
filesystem = false
network = false
devices = false
"#;
        let manifest: VntManifest = toml::from_str(raw).unwrap();
        assert_eq!(manifest.name, "demo-app");
        assert!(!manifest.permissions.network);
    }

    #[test]
    fn test_capabilities_clone() {
        let caps = Capabilities {
            fs: true,
            net: false,
            devices: false,
        };
        let copy = caps.clone();
        assert!(copy.fs);
        assert!(!copy.net);
    }
}
