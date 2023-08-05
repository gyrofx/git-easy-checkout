use inquire::Select;
use itertools::{Either, Itertools};
use std::{
    process::{Command, Stdio},
    str::from_utf8,
};

fn main() {
    inside_git_worktree_or_panic();

    let (_, branch_list) = branches();

    let chosen_branch = Select::new("Select branch to checkout", branch_list)
        .prompt()
        .unwrap();

    checkout_branch(chosen_branch);
}

fn inside_git_worktree_or_panic() {
    let status = Command::new("git")
        .args(&["rev-parse", "--is-inside-work-tree"])
        .stdin(Stdio::null())
        .status()
        .expect("git could not be executed");

    if !status.success() {
        panic!("git repositoriy not found")
    }
}

fn branches() -> (String, Vec<String>) {
    let (current_branch, branch_listt): (Vec<String>, Vec<String>) = command_output(
        "git",
        &[
            "branch",
            "-a",
            "--no-color",
            "--no-abbrev",
            "--sort=-committerdate",
        ],
    )
    .split("\n")
    .map(str::trim)
    .map(str::to_string)
    .partition_map(|r| match r.starts_with('*') {
        true => Either::Left(r),
        false => Either::Right(r),
    });

    return (
        current_branch
            .into_iter()
            .nth(0)
            .expect("Missing current branch"),
        branch_listt,
    );
}

fn checkout_branch(chosen_branch: String) {
    Command::new("git")
        .args(&["checkout", chosen_branch.as_str()])
        .status()
        .expect("git could not be executed");
}

fn command_output(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("failed to execute git command");

    from_utf8(&output.stdout).unwrap().to_string()
}
