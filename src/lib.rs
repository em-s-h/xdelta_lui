use std::process::Command;

const BUFFER_SIZE: u32 = 1000000000;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Create,
    Apply,
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
