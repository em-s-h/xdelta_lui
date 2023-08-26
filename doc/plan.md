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
    flags: String,
    files: Files,
}

pub enum Mode {
    /// Create patch
    Create,

    /// Apply patch
    Apply,
}

const BUFFER_SIZE: u32 = 1000000000;
```

<!-- }}} -->
