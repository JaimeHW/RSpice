//! Case discovery over the vendored GF180MCU corpus.

use super::*;

impl DeviceRunner {
    /// Every case in the corpus, sorted.
    ///
    /// A case is a `<name>.spice` deck with a `<name>.tsv` reference beside
    /// it. A deck without a reference is not discovered, and that asymmetry
    /// is deliberate: the two files are written together by vendoring, so a
    /// lone deck means the reference failed to capture, and running it would
    /// report a comparison failure for a case that was never comparable.
    pub fn discover(&self) -> Vec<String> {
        let mut cases = Vec::new();
        collect(&self.root.join("cases"), &mut cases);
        cases.sort();
        cases
    }

    /// Absolute path to a case's deck.
    pub fn deck_path(&self, case: &str) -> PathBuf {
        self.case_path(case, "spice")
    }

    /// Absolute path to a case's reference curve.
    pub fn reference_path(&self, case: &str) -> PathBuf {
        self.case_path(case, "tsv")
    }

    fn case_path(&self, case: &str, extension: &str) -> PathBuf {
        // Cases are named uniquely across groups, so the group directory is
        // recovered by search rather than encoded into the manifest key —
        // which keeps manifest rows readable as device descriptions.
        let cases_root = self.root.join("cases");
        if let Ok(entries) = std::fs::read_dir(&cases_root) {
            for entry in entries.flatten() {
                let candidate = entry.path().join(format!("{case}.{extension}"));
                if candidate.is_file() {
                    return candidate;
                }
            }
        }
        cases_root.join(format!("{case}.{extension}"))
    }
}

fn collect(dir: &Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "spice") {
            continue;
        }
        if !path.with_extension("tsv").is_file() {
            continue;
        }
        if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
            out.push(stem.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_pairs_decks_with_references_and_skips_unpaired_decks() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after the Unix epoch")
            .as_nanos();
        let tests_dir = std::env::temp_dir().join(format!("rspice_gf180_discovery_{unique}"));
        let group = tests_dir.join("gf180mcu").join("cases").join("diode_iv");
        std::fs::create_dir_all(&group).expect("create corpus fixture");

        std::fs::write(group.join("paired.spice"), "deck\n.end\n").expect("write deck");
        std::fs::write(group.join("paired.tsv"), "0\t1\n").expect("write reference");
        std::fs::write(group.join("orphan.spice"), "deck\n.end\n").expect("write lone deck");

        let runner = DeviceRunner::new(&tests_dir, DeviceConfig::default());
        assert_eq!(runner.discover(), vec!["paired".to_string()]);
        assert!(runner.deck_path("paired").is_file());
        assert!(runner.reference_path("paired").is_file());

        std::fs::remove_dir_all(tests_dir).expect("remove corpus fixture");
    }
}
