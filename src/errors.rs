use std::num::ParseIntError;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum VmctlControllerErrorCause {
    Parsing,
    Shell,
    Start,
    Stop,
    VmNotFound,
    Vmctl,
}

#[derive(Debug)]
pub struct VmctlControllerError {
    pub cause: VmctlControllerErrorCause
}

impl fmt::Display for VmctlControllerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VmctlControllerError!")
    }
}

impl From<ParseIntError> for VmctlControllerError {
    fn from(_: ParseIntError) -> VmctlControllerError {
        VmctlControllerError {
            cause: VmctlControllerErrorCause::Parsing
        }
    }
}

impl From<std::io::Error> for VmctlControllerError {
    fn from(_: std::io::Error) -> VmctlControllerError {
        VmctlControllerError {
            cause: VmctlControllerErrorCause::Shell
        }
    }
}

impl error::Error for VmctlControllerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.cause)
    }
}

impl fmt::Display for VmctlControllerErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmctlControllerErrorCause::Parsing => write!(f, "Parser error!"),
            VmctlControllerErrorCause::Shell => write!(f, "Shell error!"),
            VmctlControllerErrorCause::Start => write!(f, "VM not started!"),
            VmctlControllerErrorCause::Stop => write!(f, "VM not stopped!"),
            VmctlControllerErrorCause::VmNotFound => write!(f, "VM not found!"),
            VmctlControllerErrorCause::Vmctl => write!(f, "Vmctl error!"),
        }
    }
}

impl error::Error for VmctlControllerErrorCause {
    fn description(&self) -> &str {
        "Error in the vmctl controller, shell or parsing!"
    }
}


