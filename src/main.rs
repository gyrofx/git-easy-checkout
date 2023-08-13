mod branch;
mod checkout;

use branch::{branches, Branch};
use checkout::checkout_branch;
use clap::Parser;
use core::panic;
use inquire::Select;

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

    let branch_list = branches();

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

fn show_branch_selector(branch_list: Vec<Branch>) -> Branch {
    let branch_list_as_strings = branch_list
        .iter()
        .map(|b| b.name.clone())
        .collect::<Vec<_>>();

    let chosen_branch_as_str: String =
        Select::new("Select branch to checkout", branch_list_as_strings)
            .prompt()
            .unwrap();

    let chosen_branch = branch_list
        .into_iter()
        .find(|b| b.name.eq(&chosen_branch_as_str))
        .unwrap();
    chosen_branch
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
