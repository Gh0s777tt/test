use super::manifest::VntManifest;

pub fn load(path: &str) -> VntManifest {
    let data = std::fs::read(path).expect("vnt read failed");
    // parse manifest from archive
    let manifest_text = std::str::from_utf8(&data).expect("manifest is not valid UTF-8");
    toml::from_str(manifest_text).expect("invalid manifest")
}
