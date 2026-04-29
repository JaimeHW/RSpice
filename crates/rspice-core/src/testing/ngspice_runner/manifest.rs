//! Validation manifest loading and per-deck contract lookup.

use super::*;

impl TestRunner {
    pub(super) fn validation_manifest_path(test_dir: &Path) -> Option<PathBuf> {
        let mut candidate_dir = Some(test_dir);
        while let Some(dir) = candidate_dir {
            let candidate = dir.join("validation-manifest.tsv");
            if candidate.is_file() {
                return Some(candidate);
            }
            candidate_dir = dir.parent();
        }

        None
    }

    pub(super) fn load_validation_manifest(
        test_dir: &Path,
    ) -> (PathBuf, HashMap<String, ValidationContract>) {
        let Some(manifest_path) = Self::validation_manifest_path(test_dir) else {
            return (test_dir.to_path_buf(), HashMap::new());
        };
        let manifest_root = manifest_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| test_dir.to_path_buf());
        let Ok(content) = fs::read_to_string(&manifest_path) else {
            return (manifest_root, HashMap::new());
        };

        let mut manifest = HashMap::new();
        for (line_number, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.splitn(3, '\t');
            let Some(path) = parts.next() else {
                continue;
            };
            let Some(mode) = parts.next() else {
                continue;
            };
            let Some(contract) = ValidationContract::parse(mode) else {
                eprintln!(
                    "Ignoring invalid validation contract '{}' in {}:{}",
                    mode,
                    manifest_path.display(),
                    line_number + 1
                );
                continue;
            };

            manifest.insert(Self::normalize_manifest_key(path), contract);
        }

        (manifest_root, manifest)
    }

    pub(super) fn normalize_manifest_key(path: &str) -> String {
        path.trim().replace('\\', "/").to_ascii_lowercase()
    }

    pub(super) fn manifest_key_for_path(&self, cir_path: &Path) -> Option<String> {
        let relative = cir_path
            .strip_prefix(&self.validation_manifest_root)
            .or_else(|_| cir_path.strip_prefix(&self.test_dir))
            .ok()?;
        Some(Self::normalize_manifest_key(
            &relative.to_string_lossy().replace('\\', "/"),
        ))
    }

    pub(super) fn validation_contract_for(&self, cir_path: &Path) -> Option<ValidationContract> {
        let key = self.manifest_key_for_path(cir_path)?;
        self.validation_manifest.get(&key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn validation_manifest_is_resolved_from_ancestor_directory() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after the Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("rspice_manifest_test_{unique}"));
        let subdir = root.join("general");
        let circuit = subdir.join("rca3040.cir");

        fs::create_dir_all(&subdir).expect("create temporary test directory");
        fs::write(
            root.join("validation-manifest.tsv"),
            "general/rca3040.cir\tsmoke\n",
        )
        .expect("write validation manifest");
        fs::write(&circuit, "rca3040\n.end\n").expect("write circuit");

        let runner = TestRunner::new(&subdir, TestRunnerConfig::default());
        assert_eq!(
            runner.validation_contract_for(&circuit),
            Some(ValidationContract::Smoke)
        );

        fs::remove_dir_all(root).expect("remove temporary test directory");
    }
}
