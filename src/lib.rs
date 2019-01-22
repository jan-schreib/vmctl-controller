extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
pub mod errors;
pub mod status;

use errors::VmctlControllerError;
use status::Status;
use std::process::Command;
use std::str;

fn vm_id(name: &str) -> Result<u64, VmctlControllerError> {
    let status = match Status::new() {
        Ok(v) => v,
        Err(_) => return Err(VmctlControllerError::Vmctl),
    };
    let vm = match status.iter().find(|ref x| x.name == name) {
        Some(vm) => Ok(vm.id),
        None => Err(VmctlControllerError::VmNotFound),
    };
    vm
}

pub fn start(name: &str) -> Result<(), VmctlControllerError> {
    vm_id(name)?;
    let vmctl = Command::new("sh")
        .arg("-c")
        .arg(&format!("vmctl start {}", name))
        .output()?;

    let out = String::from_utf8_lossy(&vmctl.stderr);

    if out.contains("vmctl: started vm") {
        return Ok(());
    }
    Err(VmctlControllerError::Start)
}

pub fn stop(name: &str) -> Result<(), VmctlControllerError> {
    let id = vm_id(name)?;
    let vmctl = Command::new("sh")
        .arg("-c")
        .arg(&format!("vmctl stop {}", id))
        .output()?;

    let out = String::from_utf8_lossy(&vmctl.stderr);

    if out.contains("stopping vm: requested to shutdown vm") {
        return Ok(());
    }
    Err(VmctlControllerError::Stop)
}

pub fn pause(name: &str) -> Result<(), VmctlControllerError> {
    let id = vm_id(name)?;
    let vmctl = Command::new("sh")
        .arg("-c")
        .arg(&format!("vmctl pause {}", id))
        .output()?;

    let out = String::from_utf8_lossy(&vmctl.stderr);

    if out.contains("vmctl: paused vm") {
        return Ok(());
    }
    Err(VmctlControllerError::Stop)
}

pub fn unpause(name: &str) -> Result<(), VmctlControllerError> {
    let id = vm_id(name)?;
    let vmctl = Command::new("sh")
        .arg("-c")
        .arg(&format!("vmctl unpause {}", id))
        .output()?;

    let out = String::from_utf8_lossy(&vmctl.stderr);

    if out.contains("vmctl: unpaused vm") {
        return Ok(());
    }
    Err(VmctlControllerError::Stop)
}
