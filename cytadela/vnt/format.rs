pub struct VntPackage {
    pub manifest: Vec<u8>,
    pub payload: Vec<u8>, // wasm / binary
}
