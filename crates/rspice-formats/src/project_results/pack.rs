//! A project and its kept results as one archive to share, and that archive opened as a project.
//!
//! A pack is a ZIP archive: the project file at its root, and the kept segments of its results
//! folder under `results/<stem>/`, as they sit beside the file. Unpacked into a folder of its
//! own, it is a project whose runs read their results from beside it, wherever that folder is.
//! The segments are stored, not deflated: their blocks are encoded already, so a pack of
//! gigabytes is read and written once, a block at a time, and never held whole.

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

/// The extension of a pack.
pub const PACK_EXTENSION: &str = "rspicepack";

/// The extension of the project file a pack holds.
const PROJECT_EXTENSION: &str = "rspiceproj";

/// The bytes copied between two reports of progress and two looks at whether to stop.
const BLOCK: usize = 1 << 20;

/// Whether `name` is the name a kept segment goes by: its digest in hex, then `.rsr`.
fn is_kept_segment_name(name: &str) -> bool {
    name.strip_suffix(".rsr").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

/// Copy `from` into `to` a block at a time, telling `progress` the bytes done so far of
/// `total` after each block. `Ok(false)` when `progress` answered `false`.
fn copy_blocks(
    from: &mut dyn Read,
    to: &mut dyn Write,
    done: &mut u64,
    total: u64,
    progress: &mut dyn FnMut(u64, u64) -> bool,
) -> std::io::Result<bool> {
    let mut block = vec![0; BLOCK];
    loop {
        let read = match from.read(&mut block) {
            Ok(0) => return Ok(true),
            Ok(read) => read,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        };
        to.write_all(&block[..read])?;
        *done += read as u64;
        if !progress(*done, total) {
            return Ok(false);
        }
    }
}

/// Write the project file `project` and the kept segments of its results folder as the pack
/// `archive`, telling `progress` the bytes packed of all of them. Nothing is under `archive`
/// until the pack is whole. `Ok(None)` when `progress` answered `false`: the pack stopped and
/// left nothing.
pub fn pack_project(
    project: &Path,
    archive: &Path,
    progress: &mut dyn FnMut(u64, u64) -> bool,
) -> Result<Option<()>, String> {
    let name = project
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("{} is not a project file", project.display()))?;
    let stem = name
        .strip_suffix(&format!(".{PROJECT_EXTENSION}"))
        .ok_or_else(|| format!("{} is not a project file", project.display()))?;
    let folder = super::segment_folder(project);
    let mut segments: Vec<(String, PathBuf)> = match std::fs::read_dir(&folder) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                (is_kept_segment_name(&name) && entry.path().is_file())
                    .then(|| (name, entry.path()))
            })
            .collect(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => return Err(format!("{}: {error}", folder.display())),
    };
    segments.sort();
    let size = |path: &Path| std::fs::metadata(path).map_or(0, |metadata| metadata.len());
    let total = size(project) + segments.iter().map(|(_, path)| size(path)).sum::<u64>();

    let mut partial = archive.as_os_str().to_owned();
    partial.push(".partial");
    let partial = PathBuf::from(partial);
    let written = (|| -> Result<bool, String> {
        let io = |path: &Path, error: std::io::Error| format!("{}: {error}", path.display());
        let file = File::create(&partial).map_err(|error| io(&partial, error))?;
        let mut writer = zip::ZipWriter::new(BufWriter::new(file));
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .large_file(true);
        let mut done = 0;
        let entries = std::iter::once((name.to_owned(), project.to_path_buf())).chain(
            segments
                .iter()
                .map(|(name, path)| (format!("results/{stem}/{name}"), path.clone())),
        );
        for (entry, path) in entries {
            writer
                .start_file(entry.as_str(), options)
                .map_err(|error| format!("{}: {error}", partial.display()))?;
            let mut source = File::open(&path).map_err(|error| io(&path, error))?;
            if !copy_blocks(&mut source, &mut writer, &mut done, total, progress)
                .map_err(|error| io(&path, error))?
            {
                return Ok(false);
            }
        }
        let file = writer
            .finish()
            .map_err(|error| format!("{}: {error}", partial.display()))?
            .into_inner()
            .map_err(|error| io(&partial, error.into_error()))?;
        file.sync_all().map_err(|error| io(&partial, error))?;
        Ok(true)
    })();
    match written {
        Ok(true) => std::fs::rename(&partial, archive).map_err(|error| {
            let _ = std::fs::remove_file(&partial);
            format!("{}: {error}", archive.display())
        })?,
        Ok(false) => {
            let _ = std::fs::remove_file(&partial);
            return Ok(None);
        }
        Err(error) => {
            let _ = std::fs::remove_file(&partial);
            return Err(error);
        }
    }
    Ok(Some(()))
}

/// A folder beside the pack `archive` to open it into that does not exist yet: named for the
/// pack, and numbered when that name is taken.
pub fn folder_to_unpack(archive: &Path) -> PathBuf {
    let parent = archive.parent().unwrap_or_else(|| Path::new("."));
    let stem = archive.file_stem().map_or_else(
        || "project".into(),
        |stem| stem.to_string_lossy().into_owned(),
    );
    let mut folder = parent.join(&stem);
    let mut number = 2;
    while folder.exists() {
        folder = parent.join(format!("{stem} {number}"));
        number += 1;
    }
    folder
}

/// Open the pack `archive` into `folder`, which it makes and must not exist yet, telling
/// `progress` the bytes unpacked of all of them; the project file it holds. A pack is refused
/// unless it holds one project file at its root and nothing but that project's kept segments
/// beside it. `Ok(None)` when `progress` answered `false`. Stopped or refused, it leaves nothing.
pub fn unpack_project(
    archive: &Path,
    folder: &Path,
    progress: &mut dyn FnMut(u64, u64) -> bool,
) -> Result<Option<PathBuf>, String> {
    let not_a_pack = |reason: String| format!("{} is not a pack: {reason}", archive.display());
    let file = File::open(archive).map_err(|error| format!("{}: {error}", archive.display()))?;
    let mut zip = zip::ZipArchive::new(BufReader::new(file))
        .map_err(|error| not_a_pack(error.to_string()))?;
    let names = zip
        .file_names()
        .map(|name| name.map(std::borrow::Cow::into_owned))
        .collect::<Result<Vec<String>, _>>()
        .map_err(|error| not_a_pack(error.to_string()))?;
    // A plain name, so the file goes nowhere but the folder it is opened into.
    let plain = |name: &str| {
        !name.contains(['/', char::from(0x5c), ':']) && !name.starts_with('.') && name != ".."
    };
    let projects: Vec<&String> = names
        .iter()
        .filter(|name| plain(name) && name.ends_with(&format!(".{PROJECT_EXTENSION}")))
        .collect();
    let [project] = projects.as_slice() else {
        return Err(not_a_pack(format!(
            "it holds {} project files, not one",
            projects.len()
        )));
    };
    let stem = project.trim_end_matches(&format!(".{PROJECT_EXTENSION}"));
    let results = format!("results/{stem}/");
    let mut entries = Vec::new();
    let mut total = 0;
    for index in 0..zip.len() {
        let entry = zip
            .by_index(index)
            .map_err(|error| not_a_pack(error.to_string()))?;
        let name = entry
            .name()
            .map_err(|error| not_a_pack(error.to_string()))?
            .into_owned();
        if entry.is_dir() {
            if name == "results/" || name == results {
                continue;
            }
        } else if name == **project
            || name
                .strip_prefix(&results)
                .is_some_and(is_kept_segment_name)
        {
            total += entry.size();
            entries.push((index, name));
            continue;
        }
        return Err(not_a_pack(format!(
            "it holds {name}, which a pack never holds"
        )));
    }

    std::fs::create_dir(folder).map_err(|error| format!("{}: {error}", folder.display()))?;
    let unpacked = (|| -> Result<bool, String> {
        std::fs::create_dir_all(folder.join(&results))
            .map_err(|error| format!("{}: {error}", folder.display()))?;
        let mut done = 0;
        for (index, name) in entries {
            let mut entry = zip
                .by_index(index)
                .map_err(|error| not_a_pack(error.to_string()))?;
            let target = folder.join(&name);
            let io = |error: std::io::Error| format!("{}: {error}", target.display());
            let mut out = BufWriter::new(File::create(&target).map_err(io)?);
            if !copy_blocks(&mut entry, &mut out, &mut done, total, progress).map_err(io)? {
                return Ok(false);
            }
            out.into_inner()
                .map_err(|error| io(error.into_error()))?
                .sync_all()
                .map_err(io)?;
        }
        Ok(true)
    })();
    match unpacked {
        Ok(true) => Ok(Some(folder.join(project.as_str()))),
        Ok(false) => {
            let _ = std::fs::remove_dir_all(folder);
            Ok(None)
        }
        Err(error) => {
            let _ = std::fs::remove_dir_all(folder);
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("rspice-pack-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    /// A pack that holds anything but one project and its kept segments is refused, and opening
    /// it leaves no folder.
    #[test]
    fn a_pack_holding_anything_else_is_refused_and_leaves_nothing() {
        let root = scratch("refused");
        let segment = format!("results/amp/{}.rsr", "ab".repeat(32));
        for (entries, reason) in [
            (vec!["amp.rspiceproj", "../evil.rsr"], "../evil.rsr"),
            (
                vec!["amp.rspiceproj", "results/other/x.rsr"],
                "results/other/x.rsr",
            ),
            (vec!["amp.rspiceproj", "b.rspiceproj"], "2 project files"),
            (vec![segment.as_str()], "0 project files"),
        ] {
            let archive = root.join("bad.rspicepack");
            let mut writer = zip::ZipWriter::new(File::create(&archive).unwrap());
            for entry in entries {
                writer
                    .start_file(entry, zip::write::SimpleFileOptions::default())
                    .unwrap();
                writer.write_all(b"x").unwrap();
            }
            writer.finish().unwrap();
            let folder = root.join("opened");
            let refused = unpack_project(&archive, &folder, &mut |_, _| true).unwrap_err();
            assert!(refused.contains(reason), "{refused}");
            assert!(!folder.exists(), "{reason}");
        }
        let _ = std::fs::remove_dir_all(&root);
    }

    /// A stopped pack leaves no archive, and a stopped open no folder.
    #[test]
    fn a_stopped_pack_or_open_leaves_nothing() {
        let root = scratch("stopped");
        let project = root.join("amp.rspiceproj");
        std::fs::write(&project, vec![b'{'; 3 * BLOCK]).unwrap();
        let archive = root.join("amp.rspicepack");
        assert_eq!(
            pack_project(&project, &archive, &mut |done, _| done < BLOCK as u64),
            Ok(None)
        );
        assert!(
            std::fs::read_dir(&root).unwrap().count() == 1,
            "only the project"
        );
        assert_eq!(
            pack_project(&project, &archive, &mut |_, _| true),
            Ok(Some(()))
        );
        let folder = folder_to_unpack(&archive);
        assert_eq!(folder, root.join("amp"));
        assert_eq!(
            unpack_project(&archive, &folder, &mut |done, _| done < BLOCK as u64),
            Ok(None)
        );
        assert!(!folder.exists());
        let opened = unpack_project(&archive, &folder, &mut |_, _| true)
            .unwrap()
            .unwrap();
        assert_eq!(
            std::fs::read(opened).unwrap(),
            std::fs::read(&project).unwrap()
        );
        assert_eq!(folder_to_unpack(&archive), root.join("amp 2"));
        let _ = std::fs::remove_dir_all(&root);
    }
}
