use std::{
    path::PathBuf,
    process::{Command, Output, Stdio},
};

fn main() {
    println!("cargo:rustc-env=FJ_VERSION_STRING={}", version_string());
}

fn version_string() -> String {
    let pkg_version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let commit = git_description();

    let official_release = std::env::var("FJ_OFFICIAL_RELEASE").is_ok();
    println!("cargo:rerun-if-env-changed=FJ_OFFICIAL_RELEASE");

    match (commit, official_release) {
        (Some(commit), true) => format!("{pkg_version} ({commit})"),
        (Some(commit), false) => {
            format!("{pkg_version} ({commit}, unreleased)")
        }
        (None, true) => pkg_version,
        (None, false) => format!("{pkg_version} (unreleased)"),
    }
}

/// Try to get the current git commit.
///
/// This may fail if `git` isn't installed (unlikely) or if the `.git/` folder
/// isn't accessible (more likely than you think). This typically happens when
/// we're building just the `fj-app` crate in a Docker container or when
/// someone is installing from crates.io via `cargo install`.
fn git_description() -> Option<String> {
    let mut cmd = Command::new("git");
    cmd.args(["describe", "--always", "--dirty=-modified"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let Output {
        status,
        stdout,
        stderr,
    } = cmd.output().ok()?;

    let stdout = String::from_utf8_lossy(&stdout);

    if !status.success() {
        // Not sure if anyone will ever see this, but it could be helpful for
        // troubleshooting.
        eprintln!("Command failed: {cmd:?}");
        let stderr = String::from_utf8_lossy(&stderr);
        eprintln!("---- Stdout ----");
        eprintln!("{stdout}");
        eprintln!("---- Stderr ----");
        eprintln!("{stderr}");
        return None;
    }

    // Make sure we re-run whenever the current commit changes
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let project_root = crate_dir.ancestors().nth(2).unwrap();
    let head_file = project_root.join(".git").join("HEAD");
    println!("cargo:rerun-if-changed={}", head_file.display());

    if let Ok(contents) = std::fs::read_to_string(&head_file) {
        // Most of the time the HEAD file will be `ref: refs/heads/$branch`, but
        // when it's a detached head we'll only get the commit hash and can skip
        // the rerun-if-changed logic.

        if let Some((_, branch)) = contents.split_once(' ') {
            let commit_hash_file =
                project_root.join(".git").join(branch.trim());
            println!("cargo:rerun-if-changed={}", commit_hash_file.display());
        }
    }

    Some(stdout.trim().to_string())
}
