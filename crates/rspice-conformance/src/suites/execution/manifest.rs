//! Per-deck contract lookup from `execution-manifest.tsv`.
//!
//! One tab-separated row per adjudicated deck:
//!
//! ```text
//! <corpus-relative path>\t<contract>[\t<note>]
//! ```
//!
//! A trailing `!extended` in the note marks a deck too expensive for a
//! routine run. Decks absent from the manifest are required to execute, so
//! the file records only exceptions and stays readable as a list of known
//! gaps rather than an inventory of everything vendored.

use super::*;

pub(super) const FILE_NAME: &str = "execution-manifest.tsv";

/// Marks a deck whose cost puts it outside a per-commit run.
const EXTENDED_MARKER: &str = "!extended";

pub(super) fn load(root: &Path) -> HashMap<String, ManifestEntry> {
    let Ok(content) = std::fs::read_to_string(root.join(FILE_NAME)) else {
        return HashMap::new();
    };
    parse(&content, root)
}

fn parse(content: &str, root: &Path) -> HashMap<String, ManifestEntry> {
    let mut manifest = HashMap::new();
    for (index, raw) in content.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut fields = line.splitn(3, '\t');
        let (Some(path), Some(token)) = (fields.next(), fields.next()) else {
            continue;
        };
        let note = fields.next().unwrap_or_default();

        let Some(contract) = ExecutionContract::parse(token) else {
            // Loud but non-fatal: a typo in one row must not silently exempt
            // the deck it names, and the deck then falls back to `Executes`.
            eprintln!(
                "{}/{FILE_NAME}:{}: unknown contract '{token}'; deck must execute",
                root.display(),
                index + 1
            );
            continue;
        };

        manifest.insert(
            normalize_key(path),
            ManifestEntry {
                contract,
                extended: note.contains(EXTENDED_MARKER),
            },
        );
    }
    manifest
}

pub(super) fn normalize_key(path: &str) -> String {
    path.trim().replace('\\', "/")
}

impl ExecutionRunner {
    /// The contract for a deck, defaulting to [`ExecutionContract::Executes`].
    pub fn contract_for(&self, key: &str) -> ExecutionContract {
        self.manifest
            .get(&normalize_key(key))
            .map_or(ExecutionContract::Executes, |entry| entry.contract)
    }

    /// Whether the manifest marks this deck as costing more than a routine run.
    pub fn is_extended(&self, key: &str) -> bool {
        self.manifest
            .get(&normalize_key(key))
            .is_some_and(|entry| entry.extended)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_parses_contracts_notes_and_extended_marker() {
        let manifest = parse(
            "# comment\n\
             \n\
             85/c432/c432.net\texecutes\n\
             85/c7552/c7552_ann.net\texecutes\t20k devices !extended\n\
             cider\\diode.cir\texpected_unsupported\tCIDER numerical devices\n\
             bad.cir\tnot_a_contract\tignored\n",
            Path::new("corpus"),
        );

        assert_eq!(manifest.len(), 3);
        assert_eq!(
            manifest["85/c432/c432.net"].contract,
            ExecutionContract::Executes
        );
        assert!(!manifest["85/c432/c432.net"].extended);
        assert!(manifest["85/c7552/c7552_ann.net"].extended);
        // Backslash-separated keys normalize, so a manifest stays portable
        // between the platform that wrote it and the one that reads it.
        assert_eq!(
            manifest["cider/diode.cir"].contract,
            ExecutionContract::ExpectedUnsupported
        );
        // An unparsable contract must not become an exemption.
        assert!(!manifest.contains_key("bad.cir"));
    }
}
