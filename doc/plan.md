# Structs
<!-- {{{ -->

```rust
pub struct Files {
    input: String,
    target: String,
    output: String,
}

pub struct Data {

}
```

<!-- }}} -->

# Functions
<!-- {{{ -->

## lib.rs
<!-- {{{ -->

```rust
pub fn run() {}

fn apply_patch(files: &Files) {}

fn create_patch(files: &Files) {}
```

<!-- }}} -->

## ui.rs
<!-- {{{ -->

```rust
pub fn start_ui() {}
fn build_ui() {}

fn build_window() {}
fn build_buttons() {}

pub fn get_files(mode: u8) -> Files {}
```

<!-- }}} -->

<!-- }}} -->
