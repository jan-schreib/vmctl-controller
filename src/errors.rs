use std::num::ParseIntError;

#[derive(Debug)]
pub enum VmctlControllerError {
    Parsing,
    Shell,
    Start,
    Stop,
    VmNotFound,
    Vmctl,
}

impl From<ParseIntError> for VmctlControllerError {
    fn from(_: ParseIntError) -> VmctlControllerError {
        VmctlControllerError::Parsing
    }
}

impl From<std::io::Error> for VmctlControllerError {
    fn from(_: std::io::Error) -> VmctlControllerError {
        VmctlControllerError::Shell
    }
}
