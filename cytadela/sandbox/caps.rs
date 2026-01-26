#[derive(Clone)]
pub struct Capabilities {
    pub fs: bool,
    pub net: bool,
    pub devices: bool,
}
