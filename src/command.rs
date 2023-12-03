use serde::{Deserialize, Serialize};
use std::io;
use std::process::{Child, Command};

// TODO: make this more secure, impl From and parse it
pub type BinaryPath = String;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomCommand {
    Binary(BinaryPath),
}

pub struct CommandDriver {}

impl CommandDriver {
    // TODO: implement cmd with args (this is os dependant)
    pub fn run_binary(path: BinaryPath) -> io::Result<Child> {
        Command::new(path).spawn()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_binary() {
        CommandDriver::run_binary("".to_string()).expect("fail to run binary");
    }
}
