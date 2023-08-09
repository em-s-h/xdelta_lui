# Data          <!-- {{{ -->

```rust
pub struct Files {
    /// 'source' refers to the original ROM
    source: String,

    /// 'target' refers to the patched ROM
    target: String,

    /// 'output' refers to the patch
    output: String,
}

pub struct Cli {
    buffer_size: u32,
    flags: String,
    files: Files,
}

pub enum Mode {
    /// Create patch
    Create,

    /// Apply patch
    Apply,
}
```

<!-- }}} -->

# Functions     <!-- {{{ -->

## lib.rs       <!-- {{{ -->

```rust
pub fn run() {}

fn apply_patch(cli: &Cli) {}

fn create_patch(cli: &Cli) {}

// After 'get_files'
fn get_buffer_size(cli: &Files) -> u32 {}
```

<!-- }}} -->

## ui.rs        <!-- {{{ -->

```rust
pub fn start_ui() {}

fn build_window() {}
fn build_buttons() {}

pub fn get_files(mode: Mode) -> Files {}
```

<!-- }}} -->

<!-- }}} -->
