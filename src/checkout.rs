use std::process::Command;

use crate::branch::Branch;

pub fn checkout_branch(branch: Branch) {
    if branch.is_remote {
        checkout_remote_branch(branch);
    } else {
        checkout_local_branch(branch);
    }
}

pub fn checkout_local_branch(branch: Branch) {
    Command::new("git")
        .args(&["checkout", branch.name.as_str()])
        .status()
        .expect("git could not be executed");
}

pub fn checkout_remote_branch(branch: Branch) {
    let new_branch_name = branch.name.trim_start_matches("/origin/");
    Command::new("git")
        .args(&["switch", "-c", new_branch_name, branch.name.as_str()])
        .status()
        .expect("git could not be executed");
}
