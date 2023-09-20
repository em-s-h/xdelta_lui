use std::process::Command;

const BUFFER_SIZE: u32 = 1000000000;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Create,
    Apply,
}

#[derive(Default, Debug)]
pub struct Files {
    pub source: String,
    pub target: String,
    pub output: String,
}

impl Files {
    pub fn new() -> Self {
        Self {
            source: "".to_string(),
            target: "".to_string(),
            output: "".to_string(),
        }
    }
}

pub fn call_xdelta(args: &[&str]) {
    Command::new("xdelta3")
        .args(args)
        .spawn()
        .expect("Unable to run 'xdelta3 {args}'");
}

// fn get_file_name(path: &str) -> &str {
//     unimplemented!()
// }

#[cfg(test)]
mod test {}
