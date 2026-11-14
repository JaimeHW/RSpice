//! Build-time patches to vendored model releases.
//!
//! A vendored release stays byte-identical to the archive it came from. A
//! correction to it is a unified diff named after the release directory and
//! kept beside it: `cmc/vacode_102.5.0.patch` patches the files of
//! `cmc/vacode_102.5.0/`, its `a/` and `b/` paths relative to that directory.
//! Every reader of the model tree takes its files through
//! [`patch_shipped_source`]: the file-system source providers for each file
//! they load, root or include, and a reader holding the tree in memory with
//! its own lookup.
//!
//! The applier is strict. A hunk must match at the line it names, context and
//! removed lines exactly; there is no fuzz and no offset. A patch that no
//! longer applies, as after a release is re-vendored, is refused with the
//! patch and the hunk named.

use std::borrow::Cow;
use std::path::Path;

/// The compile profile the model tree carries at its root. It marks a
/// directory as a model tree, so a `.patch` file beside a directory of the
/// user's own is never read.
pub const VERILOGA_COMPILE_PROFILE_FILE_NAME: &str = ".rspice-veriloga-profile";

/// A release patch that does not apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleasePatchError {
    /// The patch, by its path in the model tree.
    pub patch: String,
    /// The one-based hunk of `patch` that failed, when one did.
    pub hunk: Option<usize>,
    pub reason: String,
}

impl std::fmt::Display for ReleasePatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.hunk {
            Some(hunk) => write!(
                f,
                "release patch {}, hunk {hunk}: {}",
                self.patch, self.reason
            ),
            None => write!(f, "release patch {}: {}", self.patch, self.reason),
        }
    }
}

impl std::error::Error for ReleasePatchError {}

/// `source`, the text of the model-tree file at `path`, as the tree ships it.
///
/// `path` is `/`-separated and relative to the tree's root. A directory above
/// the file that has a release patch beside it has that patch's hunks for the
/// file applied; `read` returns the text of a tree file by such a path, or
/// `None` when there is no file there.
pub fn patch_shipped_source<'a>(
    path: &str,
    source: &'a str,
    read: impl Fn(&str) -> std::io::Result<Option<String>>,
) -> Result<Cow<'a, str>, ReleasePatchError> {
    let mut patched = Cow::Borrowed(source);
    for (index, _) in path.match_indices('/') {
        let release = &path[..index];
        let patch_name = format!("{release}.patch");
        let patch = read(&patch_name).map_err(|error| ReleasePatchError {
            patch: patch_name.clone(),
            hunk: None,
            reason: error.to_string(),
        })?;
        if let Some(patch) = patch
            && let Cow::Owned(text) =
                apply_release_patch(&patch_name, &patch, &path[index + 1..], &patched)?
        {
            patched = Cow::Owned(text);
        }
    }
    Ok(patched)
}

/// [`patch_shipped_source`] for a file the source providers read from disk at
/// the canonical `path`. A file outside a model tree is returned unchanged.
pub(crate) fn patch_shipped_file(
    path: &Path,
    source: String,
) -> Result<String, super::PreprocessorError> {
    let Some(root) = path
        .ancestors()
        .skip(1)
        .filter(|directory| directory.join(VERILOGA_COMPILE_PROFILE_FILE_NAME).is_file())
        .last()
    else {
        return Ok(source);
    };
    let Some(relative) = path
        .strip_prefix(root)
        .ok()
        .and_then(Path::to_str)
        .map(|relative| relative.replace('\\', "/"))
    else {
        return Ok(source);
    };
    let read = |file: &str| match std::fs::read_to_string(root.join(file)) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    };
    match patch_shipped_source(&relative, &source, read) {
        Ok(Cow::Borrowed(_)) => Ok(source),
        Ok(Cow::Owned(patched)) => Ok(patched),
        Err(error) => Err(super::PreprocessorError::new(
            error.to_string(),
            Some(path.to_path_buf()),
            0,
        )),
    }
}

/// `source` with the hunks `patch` holds for `file` applied, or borrowed
/// unchanged when it holds none.
fn apply_release_patch<'a>(
    patch_name: &str,
    patch: &str,
    file: &str,
    source: &'a str,
) -> Result<Cow<'a, str>, ReleasePatchError> {
    let refuse = |hunk: Option<usize>, reason: String| ReleasePatchError {
        patch: patch_name.to_owned(),
        hunk,
        reason,
    };
    let source_lines: Vec<&str> = source.split_inclusive('\n').collect();
    let mut output = String::new();
    // Source lines already copied to `output`.
    let mut copied = 0usize;
    let mut applied = false;
    let mut current_file: Option<&str> = None;
    let mut hunk_number = 0usize;
    let mut lines = patch.split_inclusive('\n').peekable();
    while let Some(line) = lines.next() {
        let text = line.strip_suffix('\n').unwrap_or(line);
        if let Some(old) = text.strip_prefix("--- ") {
            let new = lines
                .next()
                .map(|line| line.strip_suffix('\n').unwrap_or(line))
                .and_then(|line| line.strip_prefix("+++ "));
            let (Some(old), Some(new)) = (
                old.strip_prefix("a/"),
                new.and_then(|new| new.strip_prefix("b/")),
            ) else {
                return Err(refuse(
                    None,
                    format!("`{text}` is not followed by a `+++ b/` header for the same file"),
                ));
            };
            if old != new {
                return Err(refuse(
                    None,
                    format!("the headers name `{old}` and `{new}`"),
                ));
            }
            current_file = Some(old);
            continue;
        }
        let Some(range) = text.strip_prefix("@@ -") else {
            if current_file.is_none() {
                // The plain-text preamble above the first header.
                continue;
            }
            return Err(refuse(
                Some(hunk_number),
                format!("unexpected line `{text}` outside a hunk"),
            ));
        };
        hunk_number += 1;
        let Some((old_start, old_count, new_count)) = parse_hunk_range(range) else {
            return Err(refuse(
                Some(hunk_number),
                format!("malformed hunk header `{text}`"),
            ));
        };
        let mut body = Vec::new();
        let (mut old_seen, mut new_seen) = (0usize, 0usize);
        while old_seen < old_count || new_seen < new_count {
            let Some(line) = lines.next() else {
                return Err(refuse(
                    Some(hunk_number),
                    "the hunk ends before its line counts".into(),
                ));
            };
            match line.as_bytes().first() {
                Some(b' ') => {
                    old_seen += 1;
                    new_seen += 1;
                }
                Some(b'-') => old_seen += 1,
                Some(b'+') => new_seen += 1,
                _ => {
                    let text = line.strip_suffix('\n').unwrap_or(line);
                    return Err(refuse(
                        Some(hunk_number),
                        format!("unexpected line `{text}` inside the hunk"),
                    ));
                }
            }
            body.push(line);
        }
        if old_seen != old_count || new_seen != new_count {
            return Err(refuse(
                Some(hunk_number),
                "the hunk's lines do not match its header's counts".into(),
            ));
        }
        if current_file != Some(file) {
            continue;
        }
        // A hunk that removes nothing names the line it follows.
        let start = if old_count == 0 {
            old_start
        } else {
            old_start.saturating_sub(1)
        };
        if (old_count > 0 && old_start == 0)
            || start < copied
            || start + old_count > source_lines.len()
        {
            return Err(refuse(
                Some(hunk_number),
                format!("line {old_start} is outside `{file}` or before the previous hunk"),
            ));
        }
        output.extend(source_lines[copied..start].iter().copied());
        let mut at = start;
        for line in body {
            let (marker, content) = line.split_at(1);
            let content = content.strip_suffix('\n').unwrap_or(content);
            if marker != "+" {
                let original = source_lines[at];
                if original.strip_suffix('\n').unwrap_or(original) != content {
                    return Err(refuse(
                        Some(hunk_number),
                        format!("line {} of `{file}` is not `{content}`", at + 1),
                    ));
                }
                at += 1;
            }
            match marker {
                " " => output.push_str(source_lines[at - 1]),
                "+" => {
                    output.push_str(content);
                    output.push('\n');
                }
                _ => {}
            }
        }
        copied = at;
        applied = true;
    }
    if !applied {
        return Ok(Cow::Borrowed(source));
    }
    output.extend(source_lines[copied..].iter().copied());
    Ok(Cow::Owned(output))
}

/// `-l[,s] +l[,s] @@...` as the old start and the old and new line counts.
fn parse_hunk_range(range: &str) -> Option<(usize, usize, usize)> {
    let (old, rest) = range.split_once(" +")?;
    let (new, _) = rest.split_once(" @@")?;
    let side = |side: &str| -> Option<(usize, usize)> {
        match side.split_once(',') {
            Some((start, count)) => Some((start.parse().ok()?, count.parse().ok()?)),
            None => Some((side.parse().ok()?, 1)),
        }
    };
    let (old_start, old_count) = side(old)?;
    let (_, new_count) = side(new)?;
    Some((old_start, old_count, new_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "one\ntwo\nthree\nfour\nfive\n";

    fn patch(context: &str) -> String {
        format!(
            "Replaces the third line.\n\
             --- a/dir/file.va\n\
             +++ b/dir/file.va\n\
             @@ -2,3 +2,3 @@\n \
             {context}\n\
             -three\n\
             +THREE\n \
             four\n"
        )
    }

    #[test]
    fn a_hunk_applies_at_the_line_it_names() {
        let patched = apply_release_patch("x.patch", &patch("two"), "dir/file.va", SOURCE).unwrap();
        assert_eq!(patched, "one\ntwo\nTHREE\nfour\nfive\n");
        let untouched =
            apply_release_patch("x.patch", &patch("two"), "dir/other.va", SOURCE).unwrap();
        assert!(matches!(untouched, Cow::Borrowed(_)));
    }

    #[test]
    fn a_hunk_whose_context_does_not_match_is_refused() {
        let error =
            apply_release_patch("x.patch", &patch("TWO"), "dir/file.va", SOURCE).unwrap_err();
        assert_eq!(error.patch, "x.patch");
        assert_eq!(error.hunk, Some(1));
        assert_eq!(
            error.to_string(),
            "release patch x.patch, hunk 1: line 2 of `dir/file.va` is not `TWO`"
        );
    }

    #[test]
    fn the_patch_beside_a_release_directory_applies_to_its_files() {
        let read = |file: &str| {
            Ok((file == "vendor/release.patch").then(|| patch("two").replace("dir/", "")))
        };
        let patched = patch_shipped_source("vendor/release/file.va", SOURCE, read).unwrap();
        assert_eq!(patched, "one\ntwo\nTHREE\nfour\nfive\n");
        let elsewhere = patch_shipped_source("vendor/other/file.va", SOURCE, read).unwrap();
        assert!(matches!(elsewhere, Cow::Borrowed(_)));
    }

    #[test]
    fn a_file_read_from_the_model_tree_is_patched() {
        use super::super::SourceProvider as _;
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../models/veriloga/cmc/vacode_102.5.0/PSP102_module.include");
        let document = super::super::FileSystemSourceProvider
            .load_root(&path)
            .unwrap();
        assert!(document.source.contains("mig        =  1.0e-40;"));
    }

    /// Every patch the shipped tree carries applies to the release beside it,
    /// so a re-vendored release that a patch no longer fits fails here.
    #[test]
    fn every_shipped_release_patch_applies() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/veriloga");
        let mut patches = Vec::new();
        let mut pending = vec![root.clone()];
        while let Some(directory) = pending.pop() {
            for entry in std::fs::read_dir(&directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    pending.push(path);
                } else if path
                    .extension()
                    .is_some_and(|extension| extension == "patch")
                {
                    patches.push(path);
                }
            }
        }
        assert!(
            !patches.is_empty(),
            "the shipped tree carries no release patch"
        );
        for patch_path in patches {
            let release = patch_path.with_extension("");
            let patch = std::fs::read_to_string(&patch_path).unwrap();
            let files: Vec<&str> = patch
                .lines()
                .filter_map(|line| line.strip_prefix("--- a/"))
                .collect();
            assert!(
                !files.is_empty(),
                "{} patches no file",
                patch_path.display()
            );
            for file in files {
                let source = std::fs::read_to_string(release.join(file)).unwrap();
                let patched =
                    apply_release_patch(&patch_path.display().to_string(), &patch, file, &source)
                        .unwrap_or_else(|error| panic!("{error}"));
                assert!(
                    matches!(patched, Cow::Owned(_)),
                    "{} leaves {file} unchanged",
                    patch_path.display()
                );
            }
        }
    }
}
