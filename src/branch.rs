use std::collections::HashMap;

use crate::command_output;

pub fn branches() -> Vec<Branch> {
    let tracked_remote_branches = tracked_remote_branches();
    let branch_list: Vec<Branch> = command_output(
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
    .filter(|s| !s.is_empty())
    .map(str::trim)
    .map(str::to_string)
    .map(|s| parse_branch(s, &tracked_remote_branches))
    .filter(|b| !b.is_remote || b.tracked_by.is_none())
    .collect::<Vec<_>>();

    branch_list
}

fn tracked_remote_branches() -> HashMap<String, String> {
    let tracked_remote_branch_map: HashMap<_, _> = command_output(
        "git",
        &[
            "for-each-ref",
            "--format=%(refname:short) <- %(upstream:short)",
            "refs/heads",
        ],
    )
    .split("\n")
    .map(str::to_string)
    .map(|str| {
        let pair = str
            .split("<-")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        pair
    })
    .filter(|pair| pair.len() == 2)
    .map(|pair| (pair[1].to_string(), pair[0].to_string()))
    .into_iter()
    .collect();

    tracked_remote_branch_map
}

fn parse_branch(branch: String, tracked_remote_branches: &HashMap<String, String>) -> Branch {
    println!("branch {:?}", branch);

    let is_current = branch.starts_with('*');
    let is_remote = branch.starts_with("remotes/");
    let name = branch
        .trim_start_matches("remotes/")
        .trim_start_matches("*")
        .trim()
        .to_string();
    let tracked_by = tracked_remote_branches.get(&name).cloned();

    Branch::new(name, is_current, is_remote, tracked_by)
}

#[derive(Clone, Debug)]
pub struct Branch {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub tracked_by: Option<String>,
}

impl Branch {
    fn new(name: String, is_current: bool, is_remote: bool, tracked_by: Option<String>) -> Self {
        Self {
            name,
            is_current,
            is_remote,
            tracked_by,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_branch() {
        let remote_tracked_by_map = HashMap::new();
        let branch = parse_branch(String::from("feature/project-1"), &remote_tracked_by_map);
        assert_eq!(branch.name, "feature/project-1");
        assert_eq!(branch.is_current, false);
        assert_eq!(branch.is_remote, false);
        assert_eq!(branch.tracked_by, None);
    }

    #[test]
    fn it_parses_the_current_branch() {
        let remote_tracked_by_map = HashMap::new();
        let branch = parse_branch(String::from("* feature/project-1"), &remote_tracked_by_map);
        assert_eq!(branch.name, "feature/project-1");
        assert_eq!(branch.is_current, true);
        assert_eq!(branch.is_remote, false);
        assert_eq!(branch.tracked_by, None);
    }

    #[test]
    fn it_parses_a_remote_branch() {
        let remote_tracked_by_map = HashMap::new();
        let branch = parse_branch(
            String::from("remotes/origin/feature/project-1"),
            &remote_tracked_by_map,
        );
        assert_eq!(branch.name, "origin/feature/project-1");
        assert_eq!(branch.is_current, false);
        assert_eq!(branch.is_remote, true);
        assert_eq!(branch.tracked_by, None);
    }

    #[test]
    fn it_parses_a_tracked_branch() {
        let remote_tracked_by_map = HashMap::from([(
            String::from("origin/feature/project-1"),
            String::from("feature/project-1"),
        )]);
        let branch = parse_branch(
            String::from("remotes/origin/feature/project-1"),
            &remote_tracked_by_map,
        );
        assert_eq!(branch.name, "origin/feature/project-1");
        assert_eq!(branch.is_current, false);
        assert_eq!(branch.is_remote, true);
        assert_eq!(branch.tracked_by, Some(String::from("feature/project-1")));
    }
}
