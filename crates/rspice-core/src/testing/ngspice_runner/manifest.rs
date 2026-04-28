//! Validation manifest loading and per-deck contract lookup.

use super::*;

impl TestRunner {
    pub(super) fn validation_manifest_path(test_dir: &Path) -> PathBuf {
        test_dir.join("validation-manifest.tsv")
    }

    pub(super) fn load_validation_manifest(test_dir: &Path) -> HashMap<String, ValidationContract> {
        let manifest_path = Self::validation_manifest_path(test_dir);
        let Ok(content) = fs::read_to_string(&manifest_path) else {
            return HashMap::new();
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

        manifest
    }

    pub(super) fn normalize_manifest_key(path: &str) -> String {
        path.trim().replace('\\', "/").to_ascii_lowercase()
    }

    pub(super) fn manifest_key_for_path(&self, cir_path: &Path) -> Option<String> {
        let relative = cir_path.strip_prefix(&self.test_dir).ok()?;
        Some(Self::normalize_manifest_key(
            &relative.to_string_lossy().replace('\\', "/"),
        ))
    }

    pub(super) fn validation_contract_for(&self, cir_path: &Path) -> Option<ValidationContract> {
        let key = self.manifest_key_for_path(cir_path)?;
        self.validation_manifest.get(&key).copied()
    }
}
