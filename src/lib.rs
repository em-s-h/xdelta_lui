mod ui;

pub struct File {
    /// 'source' refers to the original ROM
    source: String,

    /// 'target' refers to the patched ROM
    target: String,

    /// 'output' refers to the patch
    output: String,
}

pub enum Mode {
    /// Create patch
    Create,

    /// Apply patch
    Apply,
}

const MAX_BUFFER_SIZE: u32 = 1000000000; // 1GB

pub fn run() {}

fn apply_patch(files: &File) {}

fn create_patch(files: &File) {}
