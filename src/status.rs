use std::str;
use std::str::FromStr;

use std::process::{Command, Stdio};
use errors::VmctlControllerError;

#[derive(Debug)]
pub struct Status {
    pub id: u64,
    pub pid: String,
    pub vcpus: u64,
    pub max_mem: String,
    pub cur_mem: String,
    pub tty: String,
    pub owner: String,
    pub name: String,
}

impl FromStr for Status {
    type Err = VmctlControllerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "" {
            return Err(VmctlControllerError::Parsing);
        }

        let status_dirty_str: Vec<&str> = s.split(' ').collect();
        let status_str: Vec<&str> = status_dirty_str.into_iter().filter(|&s| s != "").collect();

        if status_str.len() != 8 {
            return Err(VmctlControllerError::Parsing);
        }

        let id = status_str.get(0).unwrap().parse::<u64>()?;
        let pid = status_str.get(1).unwrap().to_string();
        let vcpus = status_str.get(2).unwrap().parse::<u64>()?;
        let max_mem = status_str.get(3).unwrap().to_string();
        let cur_mem = status_str.get(4).unwrap().to_string();
        let tty = status_str.get(5).unwrap().to_string();
        let owner = status_str.get(6).unwrap().to_string();
        let name = status_str.get(7).unwrap().to_string();

        Ok(Status {
            id: id,
            pid: pid,
            vcpus: vcpus,
            max_mem: max_mem,
            cur_mem: cur_mem,
            tty: tty,
            owner: owner,
            name: name,
        })
    }
}

impl Status {
    fn from_vec(vms: Vec<&str>) -> Vec<Result<Status, VmctlControllerError>> {
        vms.into_iter().map(|vm| Status::from_str(&vm)).collect()
    }

    fn from_shell(input: &str) -> Vec<Result<Status, VmctlControllerError>> {
        let strings_dirty: Vec<&str> = input.split('\n').collect();
        let strings: Vec<&str> = strings_dirty.into_iter().filter(|&s| s != "").collect();
        Status::from_vec(strings)
    }

    /// Executes "vmctl status | tail -n +2" and returns the result.
    /// Will not work then vmctl or tail are not installed.
    pub fn new() -> Result<Vec<Status>, VmctlControllerError> {
        let vmctl = match Command::new("sh")
            .arg("-c")
            .arg("vmctl status")
            .stdout(Stdio::piped())
            .spawn() {
                Ok(v) => v,
                Err(_) => return Err(VmctlControllerError::Shell),
            };

        let out = match vmctl.stdout {
            Some(v) => v,
            None => return Err(VmctlControllerError::Shell),
        };

        //remove the first line of the 'vmctl status' output
        let tail = match Command::new("tail")
            .arg("-n")
            .arg("+2")
            .stdin(Stdio::from(out))
            .output() {
                Ok(v) => v,
                Err(_) => return Err(VmctlControllerError::Shell),
            };

        let ret = match str::from_utf8(&tail.stdout) {
            Ok(v) => v,
            Err(_) => return Err(VmctlControllerError::Parsing),
        };

        let stats = Status::from_shell(ret);
        Ok(stats.into_iter().map(|x| x.unwrap()).collect())
    }
}

#[test]
fn empty() {
    let result = Status::from_str("");
    assert!(result.is_err())
}

#[test]
fn garbage() {
    let result = Status::from_str("garbage");
    assert!(result.is_err())
}

#[test]
fn broken() {
    let vm1 = "  a     -     1    2.0G       -      äöü";
    let result = Status::from_str(vm1);
    assert!(result.is_err())
}
#[test]
fn from_str() {
    let vm1 = "  1     -     1    2.0G       -       -          user one";
    let result = Status::from_str(vm1);
    let status = result.unwrap();
    assert_eq!(status.id, 1)
}

#[test]
fn from_vec() {
    let vm1 = "1     -     1    2.0G       -       -          user one";
    let vm2 = "2     -     1    512M       -       -          user two";
    let vm3 = "3     -     1    2.0G       -       -          user three";
    let vm4 = "4     -     1    4.0G       -       -          user four";

    let vm_vec = vec![vm1, vm2, vm3, vm4];

    let result = Status::from_vec(vm_vec);
    let stati: Vec<Status> = result.into_iter().map(|x| x.unwrap()).collect();

    assert_eq!(stati.get(0).unwrap().id, 1);
    assert_eq!(stati.get(1).unwrap().owner, "user");
    assert_eq!(stati.get(2).unwrap().name, "three");
    assert_eq!(stati.get(3).unwrap().max_mem, "4.0G");
}

#[test]
fn from_shell() {
    let shell_output = r#"
    1     -     1    2.0G       -       -          user one
    2     -     1    512M       -       -          user two
    3     -     1    2.0G       -       -          user three
    4     -     1    4.0G       -       -          user four"#;

    let result = Status::from_shell(shell_output);

    let stati: Vec<Status> = result.into_iter().map(|x| x.unwrap()).collect();

    assert_eq!(stati.get(0).unwrap().id, 1);
    assert_eq!(stati.get(1).unwrap().owner, "user");
    assert_eq!(stati.get(2).unwrap().name, "three");
    assert_eq!(stati.get(3).unwrap().max_mem, "4.0G");
}
