//! The results store segments a saved project file names, read from its JSON without decoding
//! the project: what the browser's store keeps for a file, a checkpoint or a version a save
//! replaced.

use std::collections::HashSet;

/// The project a file names (its id) and every segment it reads: the `segment` a stored window or
/// event history names, and the `segments` an archived run lists. Other digests a file spells,
/// a snapshot's or a revision's, name no segment. `None` for bytes that are not a project file.
pub fn segments_named_in(bytes: &[u8]) -> Option<(String, HashSet<[u8; 32]>)> {
    use serde_json::Value;

    let file: Value = serde_json::from_slice(bytes).ok()?;
    let project = file.pointer("/workspace/project/id")?.as_str()?.to_owned();
    let mut digests = HashSet::new();
    let mut add = |value: &Value| {
        if let Some(digest) = value
            .as_str()
            .and_then(|text| rspice_results::store_reader::parse_digest(text).ok())
        {
            digests.insert(digest.0);
        }
    };
    let mut pending = vec![&file];
    while let Some(value) = pending.pop() {
        match value {
            Value::Array(values) => pending.extend(values),
            Value::Object(fields) => {
                for (key, field) in fields {
                    match (key.as_str(), field) {
                        ("segment", _) => add(field),
                        ("segments", Value::Array(listed)) => {
                            for item in listed {
                                if item.is_string() {
                                    add(item);
                                } else {
                                    pending.push(item);
                                }
                            }
                        }
                        _ => pending.push(field),
                    }
                }
            }
            _ => {}
        }
    }
    Some((project, digests))
}

/// Every segment the project `project` reads in the project files `files` (its retained
/// checkpoints): a file of another project names none of its segments.
pub fn segments_of_project<'a>(
    project: &str,
    files: impl IntoIterator<Item = &'a [u8]>,
) -> HashSet<[u8; 32]> {
    files
        .into_iter()
        .filter_map(segments_named_in)
        .filter(|(id, _)| id == project)
        .flat_map(|(_, digests)| digests)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUN: &str = "11111111111111111111111111111111111111111111111111111111111111aa";
    const ARCHIVED: &str = "22222222222222222222222222222222222222222222222222222222222222bb";
    const SNAPSHOT: &str = "33333333333333333333333333333333333333333333333333333333333333cc";

    fn file(project: &str) -> Vec<u8> {
        serde_json::json!({
            "workspace": {"project": {"id": project}},
            "snapshot_digest": SNAPSHOT,
            "simulation_results": {"runs": [{"analyses": [{"waveforms": [
                {"x": {"segment": RUN, "column": 0}, "y": {"segment": RUN, "column": 1}}
            ]}]}]},
            "documents": [{"archived": {"segments": [ARCHIVED]}}],
        })
        .to_string()
        .into_bytes()
    }

    fn digest(hex: &str) -> [u8; 32] {
        rspice_results::store_reader::parse_digest(hex).unwrap().0
    }

    #[test]
    fn a_file_names_the_segments_its_runs_and_archives_read_and_no_other_digest() {
        let (project, named) = segments_named_in(&file("p-1")).unwrap();
        assert_eq!(project, "p-1");
        assert_eq!(named, HashSet::from([digest(RUN), digest(ARCHIVED)]));
    }

    #[test]
    fn a_projects_checkpoints_name_their_segments_and_another_projects_name_none() {
        let mine = file("p-1");
        let theirs = file("p-2");
        let unreadable = b"not a project".to_vec();
        let files = [mine.as_slice(), theirs.as_slice(), unreadable.as_slice()];
        assert_eq!(
            segments_of_project("p-1", files),
            HashSet::from([digest(RUN), digest(ARCHIVED)])
        );
        assert!(segments_of_project("p-3", files).is_empty());
    }
}
