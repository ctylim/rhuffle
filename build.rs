use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_commit_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", git_commit_hash);
    let output = Command::new("git")
        .args(&["log", "-1", "--date=iso-strict", "--format=%cd"])
        .output()
        .unwrap();
    let git_commit_date = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_COMMIT_DATE={}", git_commit_date);
}
