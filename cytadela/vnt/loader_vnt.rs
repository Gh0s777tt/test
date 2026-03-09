use super::manifest::VntManifest;

pub fn load(path: &str) -> VntManifest {
    let data = std::fs::read(path).expect("vnt read failed");
    // parse manifest from archive
    toml::from_slice(&data).expect("invalid manifest")
}
