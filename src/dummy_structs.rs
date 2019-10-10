use std::path::PathBuf;

pub struct HawktracerInstance {}

pub enum HawktracerListenerType {
    ToFile {
        file_path: PathBuf,
        buffer_size: usize,
    },
    TCP {
        port: u32,
        buffer_size: usize,
    },
}

impl HawktracerInstance {
    pub fn new() -> HawktracerInstance {
        HawktracerInstance {}
    }

    pub fn create_listener(&self, _listener_type: HawktracerListenerType) {}
}
