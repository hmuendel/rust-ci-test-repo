use std::env;
use std::fs::File;
use std::process::{Command, Stdio};

fn main() {
    // we only want to do the heavy lifting on release builds
    if let Ok(_) = env::var("releasing") {
        // tagging the git repo with the version from cargo if the tag doesn't
        // already exist
        if !get_git_tags().contains(&concat!("v", env!("CARGO_PKG_VERSION")).to_string()) {
            tag_git_repo();
            run_git_cliff();
            force_tag_repo();
        }
    }
}

fn run_git_cliff() {
    let file_name = "../CHANGELOG.md";
    let file = File::create(file_name).unwrap();
    let stdio = Stdio::from(file);
    let status = Command::new("git-cliff")
        .stdout(stdio)
        .current_dir("../")
        .status()
        .expect("Failed to run git cliff");
    print!("{:?}", status);
    add_to_commit(file_name);
    add_to_commit("../Cargo.lock");
    ammend_commit()
}

fn add_to_commit(file: &str) {
    let output = Command::new("git")
        .args(["add", file])
        .output()
        .expect("Failed to execute git add");
    println!("{:?}", output);
}

fn ammend_commit() {
    let output = Command::new("git")
        .args(["commit", "--amend", "--no-edit"])
        .output()
        .expect("Failed to execute git commit ammend");
    println!("{:?}", output);
}

// tagging the git repo with the version from cargo
fn tag_git_repo() {
    let output = Command::new("git")
        .args([
            "tag",
            "-s",
            "-m",
            concat!("new version: ", env!("CARGO_PKG_VERSION")),
            concat!("v", env!("CARGO_PKG_VERSION")),
        ])
        .output()
        .expect("Failed to execute git tag");
    println!("{:?}", output);
}

// tagging the git repo with the version from cargo
fn force_tag_repo() {
    let output = Command::new("git")
        .args([
            "tag",
            "-s",
            "-f",
            "-m",
            concat!("new version: ", env!("CARGO_PKG_VERSION")),
            concat!("v", env!("CARGO_PKG_VERSION")),
        ])
        .output()
        .expect("Failed to execute git tag");
    println!("{:?}", output);
}

// get the git tags of the current repo from the git command line
fn get_git_tags() -> Vec<String> {
    let output = Command::new("git")
        .args(["tag", "-l"])
        .output()
        .expect("Failed to execute git rev-parse");
    let git_tags = String::from_utf8_lossy(&output.stdout).trim().to_string();
    git_tags.split('\n').map(|s| s.to_string()).collect()
}
