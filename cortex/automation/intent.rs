pub enum Intent {
    OpenFile(String),
    Search(String),
    Shutdown,
    Unknown,
}

pub fn parse(prompt: &str) -> Intent {
    if prompt.contains("wyłącz") {
        Intent::Shutdown
    } else {
        Intent::Unknown
    }
}
