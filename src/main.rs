use clap::Parser;
use inquire::Select;
use itertools::{Either, Itertools};
use std::{
    process::{exit, Command, Stdio},
    str::from_utf8,
};

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Cli {
    #[clap(long, action)]
    version: bool,
}

fn main() {
    parse_and_handle_cli_arguments();

    inside_git_worktree_or_panic();

    let (_, branch_list) = branches();

    let chosen_branch = show_branch_selector(branch_list);

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

fn show_branch_selector(branch_list: Vec<String>) -> String {
    let chosen_branch = Select::new("Select branch to checkout", branch_list)
        .prompt()
        .unwrap();
    chosen_branch
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

fn parse_and_handle_cli_arguments() {
    let cli = Cli::parse();

    if cli.version {
        print_version_and_exit();
    }
}

fn print_version_and_exit() {
    println!("Build timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
    println!("git revision: {}", env!("VERGEN_GIT_DESCRIBE"));
    exit(0);
}
