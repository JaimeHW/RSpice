use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    error::PackError,
    limits::Limits,
    manifest::ManifestTemplate,
    pack::build_pack,
    path::is_reserved_path,
    signing::{SIGNING_KEY_BYTES, SigningKey},
};

/// Generates fresh signing key material from the platform entropy source.
///
/// This is the only function in the crate that needs randomness, and it exists
/// only behind the `packer` feature so a verifying client — native or
/// WebAssembly — never links a random-number source at all.
pub fn generate_signing_key() -> Result<SigningKey, PackError> {
    let mut secret = [0_u8; SIGNING_KEY_BYTES];
    getrandom::fill(&mut secret).map_err(|error| PackError::Io {
        path: "platform entropy".to_owned(),
        detail: error.to_string(),
    })?;
    Ok(SigningKey::from_bytes(&secret))
}

/// Builds and signs a pack from a directory tree.
///
/// The walk resolves nothing: symbolic links, devices, and anything else that
/// is not a plain file are refused rather than followed, so the file set in the
/// archive is exactly the file set on disk under `directory`.
pub fn build_pack_from_dir(
    directory: &Path,
    template: ManifestTemplate,
    key: &SigningKey,
) -> Result<Vec<u8>, PackError> {
    let files = collect_files(directory)?;
    build_pack(&files, template, key)
}

fn collect_files(root: &Path) -> Result<Vec<(String, Vec<u8>)>, PackError> {
    let limits = Limits::default();
    let mut pending = vec![(root.to_path_buf(), String::new())];
    let mut files = Vec::new();
    let mut expanded = 0_u64;
    while let Some((directory, prefix)) = pending.pop() {
        let mut children: Vec<(PathBuf, String, bool)> = Vec::new();
        for entry in fs::read_dir(&directory).map_err(|error| io(&directory, &error))? {
            let entry = entry.map_err(|error| io(&directory, &error))?;
            let path = entry.path();
            let kind = fs::symlink_metadata(&path)
                .map_err(|error| io(&path, &error))?
                .file_type();
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                return Err(PackError::IllegalPath(path.display().to_string()));
            };
            let relative = if prefix.is_empty() {
                name.to_owned()
            } else {
                format!("{prefix}/{name}")
            };
            if kind.is_dir() {
                children.push((path, relative, true));
            } else if kind.is_file() {
                children.push((path, relative, false));
            } else {
                return Err(PackError::IllegalPath(relative));
            }
        }
        children.sort_by(|left, right| left.1.as_bytes().cmp(right.1.as_bytes()));
        for (path, relative, is_directory) in children {
            if is_directory {
                pending.push((path, relative));
                continue;
            }
            if is_reserved_path(&relative) {
                return Err(PackError::ReservedFileName(relative));
            }
            if files.len() >= limits.max_files {
                return Err(PackError::TooManyFiles {
                    limit: limits.max_files,
                    actual: files.len() + 1,
                });
            }
            let content = fs::read(&path).map_err(|error| io(&path, &error))?;
            expanded += content.len() as u64;
            if expanded > limits.max_expanded_bytes as u64 {
                return Err(PackError::ExpandedTooLarge {
                    limit: limits.max_expanded_bytes,
                });
            }
            files.push((relative, content));
        }
    }
    Ok(files)
}

fn io(path: &Path, error: &std::io::Error) -> PackError {
    PackError::Io {
        path: path.display().to_string(),
        detail: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{limits::Limits, pack::Pack, testing};

    fn scratch(name: &str) -> PathBuf {
        let directory = std::env::temp_dir().join(format!("rspice-pack-{name}"));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir_all(directory.join("models")).expect("scratch directory");
        directory
    }

    #[test]
    fn a_directory_tree_builds_the_same_pack_as_its_in_memory_file_set() {
        let root = scratch("tree");
        fs::write(root.join("models/opamps.lib"), testing::OPAMP_LIBRARY).expect("writes");
        fs::write(root.join("LICENSE"), testing::LICENSE_TEXT).expect("writes");
        let key = crate::signing::signing_key(&testing::SECRET);

        let from_dir =
            build_pack_from_dir(&root, testing::template(), &key).expect("builds from disk");
        let from_memory =
            build_pack(&testing::files(), testing::template(), &key).expect("builds in memory");
        assert_eq!(from_dir, from_memory);
        Pack::verify(&from_dir, &key.verifying_key(), &Limits::default()).expect("verifies");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn a_container_owned_name_on_disk_is_refused() {
        let root = scratch("reserved");
        fs::write(root.join("manifest.json"), b"{}").expect("writes");
        fs::write(root.join("models/opamps.lib"), testing::OPAMP_LIBRARY).expect("writes");
        let key = crate::signing::signing_key(&testing::SECRET);
        assert!(matches!(
            build_pack_from_dir(&root, testing::template(), &key),
            Err(PackError::ReservedFileName(name)) if name == "manifest.json"
        ));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn generated_keys_differ_and_sign_their_own_manifests() {
        let first = generate_signing_key().expect("generates");
        let second = generate_signing_key().expect("generates");
        assert_ne!(first.to_bytes(), second.to_bytes());
        let archive = build_pack(&testing::files(), testing::template(), &first).expect("builds");
        Pack::verify(&archive, &first.verifying_key(), &Limits::default()).expect("verifies");
    }
}
