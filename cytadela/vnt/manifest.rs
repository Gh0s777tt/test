use serde::Deserialize;

#[derive(Deserialize)]
pub struct VntManifest {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub permissions: Permissions,
}

#[derive(Deserialize)]
pub struct Permissions {
    pub filesystem: bool,
    pub network: bool,
    pub devices: bool,
}
