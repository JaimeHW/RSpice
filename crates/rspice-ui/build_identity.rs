//! Source commit for diagnostics, including incremental and linked-worktree builds.

use std::path::{Path, PathBuf};
use std::process::Command;

fn git(manifest_dir: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(manifest_dir)
        // A caller's Git environment must not attribute this source to another checkout.
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_COMMON_DIR")
        .env_remove("GIT_INDEX_FILE")
        .output()
        .ok()?;
    output.status.success().then_some(())?;
    let text = String::from_utf8(output.stdout).ok()?.trim().to_owned();
    (!text.is_empty()).then_some(text)
}

fn git_path(manifest_dir: &Path, name: &str) -> Option<PathBuf> {
    git(manifest_dir, &["rev-parse", "--git-path", name]).map(|path| manifest_dir.join(path))
}

fn watch(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());
}

pub fn emit(manifest_dir: &Path) {
    // An extracted archive inside an unrelated repository has no source identity.
    let tracked = git(
        manifest_dir,
        &["ls-files", "--error-unmatch", "--", "Cargo.toml"],
    )
    .is_some();
    let hash = tracked
        .then(|| git(manifest_dir, &["rev-parse", "--short=9", "HEAD"]))
        .flatten()
        .unwrap_or_else(|| "unknown".to_owned());
    println!("cargo:rustc-env=RSPICE_BUILD_HASH={hash}");
    if !tracked {
        return;
    }

    // Git resolves per-worktree HEAD and shared refs, including .git indirection.
    for name in ["HEAD", "packed-refs", "reftable"] {
        if let Some(path) = git_path(manifest_dir, name)
            && path.exists()
        {
            watch(&path);
        }
    }
    if let Some(reference) = git(manifest_dir, &["symbolic-ref", "--quiet", "HEAD"])
        && let Some(mut path) = git_path(manifest_dir, &reference)
    {
        // A packed branch has no loose ref yet. Watch its nearest existing ref
        // directory so the next commit's loose ref triggers Cargo as well. Do
        // not register absent files: those make Cargo rerun on every build.
        while !path.exists() && path.pop() {}
        if path.exists() {
            watch(&path);
        }
    }
}
