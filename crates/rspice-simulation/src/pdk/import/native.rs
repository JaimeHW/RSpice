//! Native archive reads, background validation, and stale import rejection.

use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

use super::*;

pub struct NativePackageImport {
    base: PdkConfig,
    result: Result<PackageImportCandidate, String>,
}

static NATIVE_PACKAGE_IMPORTS: OnceLock<Mutex<VecDeque<NativePackageImport>>> = OnceLock::new();

fn native_package_imports() -> &'static Mutex<VecDeque<NativePackageImport>> {
    NATIVE_PACKAGE_IMPORTS.get_or_init(|| Mutex::new(VecDeque::new()))
}

fn prepare_native_package_import(
    base: &PdkConfig,
    bytes: &[u8],
    authority: &PdkAdministrativeAuthority,
    reason: &str,
) -> Result<PackageImportCandidate, String> {
    let mut config = base.clone();
    let receipt = config
        .technology_registry
        .install_archive_bytes(bytes, &config.publisher_trust_store, authority, reason)
        .map_err(|error| error.to_string())?;
    Ok(PackageImportCandidate {
        config,
        package_id: receipt.target.package_id,
        revision: receipt.target.revision,
        sequence: receipt.sequence,
    })
}

impl NativePackageImport {
    pub fn resolve(self, current: &PdkConfig) -> Result<PackageImportCandidate, String> {
        if *current != self.base {
            return Err(
                "PDK configuration changed while the signed package was being validated; the stale candidate was discarded without mutation."
                    .to_owned(),
            );
        }
        self.result
    }
}

pub fn take_native_package_imports() -> Vec<NativePackageImport> {
    native_package_imports()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .drain(..)
        .collect()
}

pub fn start_native_package_import(
    base: PdkConfig,
    path: std::path::PathBuf,
    authority: PdkAdministrativeAuthority,
    reason: String,
    wake: impl FnOnce() + Send + 'static,
) {
    std::thread::spawn(move || {
        let result = (|| {
            let metadata = std::fs::metadata(&path).map_err(|error| error.to_string())?;
            if metadata.len() > MAX_PDK_ARCHIVE_BYTES as u64 {
                return Err(format!(
                    "{} exceeds the {}-byte package limit",
                    path.display(),
                    MAX_PDK_ARCHIVE_BYTES
                ));
            }
            let bytes = std::fs::read(&path).map_err(|error| error.to_string())?;
            if bytes.len() > MAX_PDK_ARCHIVE_BYTES {
                return Err(format!(
                    "{} grew beyond the {}-byte package limit while it was being read",
                    path.display(),
                    MAX_PDK_ARCHIVE_BYTES
                ));
            }
            prepare_native_package_import(&base, &bytes, &authority, &reason)
        })();
        native_package_imports()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push_back(NativePackageImport { base, result });
        wake();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_package_candidate_preserves_base_and_rejects_stale_completion() {
        let (bytes, trust, authority) = crate::pdk::test_fixtures::fixture_archive();
        let mut base = PdkConfig {
            publisher_trust_store: trust,
            ..PdkConfig::default()
        };
        let before = base.clone();

        let candidate = NativePackageImport {
            base: base.clone(),
            result: prepare_native_package_import(
                &base,
                &bytes,
                &authority,
                "background import test",
            ),
        }
        .resolve(&base)
        .expect("background candidate validates");

        assert_eq!(base, before);
        assert_eq!(candidate.package_id, "demo180");
        assert_eq!(candidate.revision, "2.3.1");
        assert_eq!(candidate.sequence, 1);
        assert_eq!(
            candidate
                .config
                .technology_registry
                .validated_packages()
                .len(),
            1
        );

        let completion = NativePackageImport {
            base: base.clone(),
            result: Ok(candidate),
        };
        base.set_env_var("PDK_IMPORT_TEST", "changed during validation");
        assert_eq!(
            completion.resolve(&base).err().as_deref(),
            Some(
                "PDK configuration changed while the signed package was being validated; the stale candidate was discarded without mutation."
            ),
        );
    }
}
