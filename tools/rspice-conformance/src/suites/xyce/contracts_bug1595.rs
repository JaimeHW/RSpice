use super::*;
use std::io::Read as _;

impl XyceTestRunner {
    pub(super) fn read_bug1595_source_bounded(path: &Path) -> Result<Vec<u8>, String> {
        const LABEL: &str = "BUG_1595 hierarchical mutual-inductor expected failure";
        let expected_bytes = XYCE_BUG1595_RETAINED_ARTIFACTS[0].1;
        let physical_cap = expected_bytes
            .checked_mul(2)
            .and_then(|value| value.checked_add(2))
            .ok_or_else(|| format!("{LABEL} source-size bound overflowed"))?;
        let metadata = fs::symlink_metadata(path)
            .map_err(|error| format!("failed to inspect {LABEL} source: {error}"))?;
        if !metadata.file_type().is_file()
            || metadata.file_type().is_symlink()
            || metadata.len() > physical_cap as u64
        {
            return Err(format!(
                "{LABEL} source must be a bounded regular non-symlink file"
            ));
        }
        let file = fs::File::open(path)
            .map_err(|error| format!("failed to open {LABEL} source: {error}"))?;
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        file.take((physical_cap + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|error| format!("failed to read {LABEL} source: {error}"))?;
        if bytes.len() > physical_cap {
            return Err(format!("{LABEL} source exceeded its bounded read envelope"));
        }
        Ok(bytes)
    }

    pub(super) fn bug1595_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG1595_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG1595_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG1595_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1595_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug1595_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG1595_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG1595_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG1595_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG1595_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "BUG_1595 Release-7.10 error-wrapper provenance changed: records={}/{}, bytes={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                XYCE_BUG1595_HISTORICAL_RECORD_COUNT,
                stream.len(),
                XYCE_BUG1595_HISTORICAL_RECORD_BYTES,
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug1595_complete_family_provenance(
        &self,
        family_dir: &Path,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG_1595 hierarchical mutual-inductor expected failure";
        Self::validate_bug1595_historical_oracle_provenance()?;

        let expected_family = self.root.join("Netlists/Certification_Tests/BUG_1595");
        if !Self::same_path(family_dir, &expected_family) {
            return Err(format!(
                "{LABEL} resolved outside its exact canonical corpus directory: {}",
                family_dir.display()
            ));
        }
        let family_metadata = fs::symlink_metadata(family_dir)
            .map_err(|error| format!("failed to inspect {LABEL} family: {error}"))?;
        if !family_metadata.file_type().is_dir() || family_metadata.file_type().is_symlink() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }

        let expected = XYCE_BUG1595_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        for entry in fs::read_dir(family_dir)
            .map_err(|error| format!("failed to read {LABEL} family: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "{LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!(
                    "{LABEL} family has a case-colliding member {name:?}"
                ));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{LABEL} family acquired unexpected member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }

            let maximum_raw_bytes = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(2))
                .ok_or_else(|| format!("{LABEL} retained-size bound overflowed"))?;
            if metadata.len() > maximum_raw_bytes as u64 {
                return Err(format!(
                    "{LABEL} retained member {name:?} exceeds its bounded read envelope: {} > {maximum_raw_bytes}",
                    metadata.len()
                ));
            }
            let file = fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} member {name:?}: {error}"))?;
            let mut bytes = Vec::with_capacity(metadata.len() as usize);
            file.take((maximum_raw_bytes + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} member {name:?}: {error}"))?;
            if bytes.len() > maximum_raw_bytes {
                return Err(format!(
                    "{LABEL} retained member {name:?} exceeded its bounded read envelope"
                ));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} retained member {name:?} changed: expected {expected_bytes}/{expected_sha256}/{expected_blake3}, got {}/{sha256}/{content_blake3}",
                    canonical.len()
                ));
            }
            if name == "options" && canonical != b"timelimit=30\n" {
                return Err(format!(
                    "{LABEL} options no longer binds the historical 30-second outer timeout"
                ));
            }
        }
        let source_names = observed.iter().cloned().collect::<Vec<_>>();
        let source_census = blake3::hash(source_names.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if observed.len() != expected.len()
            || source_census != XYCE_BUG1595_SOURCE_DIRECTORY_CENSUS_BLAKE3
        {
            return Err(format!(
                "{LABEL} retained family census changed: expected {}/{}, got {}/{}",
                expected.len(),
                XYCE_BUG1595_SOURCE_DIRECTORY_CENSUS_BLAKE3,
                observed.len(),
                source_census
            ));
        }

        let family_prefix = "netlists/certification_tests/bug_1595/";
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if owners != [XYCE_BUG1595_EXPECTED_FAILURE_RECORD.to_string()] {
            return Err(format!(
                "{LABEL} requires its exact one live wrapper owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG1595_EXPECTED_FAILURE_RECORD) {
            return Err(format!(
                "{LABEL} must not be classified by an upstream exclude sentinel"
            ));
        }

        let output_family = self.root.join("OutputData/Certification_Tests/BUG_1595");
        match fs::symlink_metadata(&output_family) {
            Ok(_) => {
                return Err(format!(
                    "{LABEL} acquired an invented output family at {}",
                    output_family.display()
                ));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "failed to inspect {LABEL} OutputData path {}: {error}",
                    output_family.display()
                ));
            }
        }
        let empty_output_hash = blake3::hash(b"").to_hex().to_string();
        if empty_output_hash != XYCE_BUG1595_EMPTY_OUTPUT_CENSUS_BLAKE3 {
            return Err(format!("{LABEL} empty OutputData census identity changed"));
        }
        self.reject_wrapper_output_artifacts(&family_dir.join("bug1595.cir"))
            .map_err(|error| format!("{LABEL} {error}"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce")
    }

    fn canonical_deck(root: &Path) -> XyceDeck {
        XyceDeck {
            path: root.join(XYCE_BUG1595_EXPECTED_FAILURE_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1595_EXPECTED_FAILURE_PATH.to_string(),
        }
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let source_root = corpus_root();
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1595-{label}-"))
            .tempdir()
            .expect("create BUG1595 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_1595");
        fs::create_dir_all(&family).expect("create BUG1595 fixture family");
        for name in ["bug1595.cir", "options"] {
            fs::copy(
                source_root
                    .join("Netlists/Certification_Tests/BUG_1595")
                    .join(name),
                family.join(name),
            )
            .expect("copy canonical BUG1595 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG1595_EXPECTED_FAILURE_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG1595 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG1595 exclusion manifest");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug1595_historical_and_retained_provenance_are_exact() {
        XyceTestRunner::validate_bug1595_historical_oracle_provenance()
            .expect("Release-7.10 BUG1595 provenance remains exact");
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug1595_complete_family_provenance(
                &root.join("Netlists/Certification_Tests/BUG_1595"),
            )
            .expect("retained BUG1595 family remains exact");
    }

    #[test]
    fn bug1595_canonical_oracle_and_expired_deadline_are_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = canonical_deck(&root);
        runner
            .validate_expected_failure_oracle(
                &deck,
                XyceExpectedFailureKind::Bug1595HierarchicalMutualInductorReference,
                Instant::now(),
            )
            .expect("canonical BUG1595 typed failure qualifies");
        let error = runner
            .validate_expected_failure_oracle(
                &deck,
                XyceExpectedFailureKind::Bug1595HierarchicalMutualInductorReference,
                Instant::now() - Duration::from_secs(31),
            )
            .expect_err("expired BUG1595 deadline must fail closed");
        assert!(error.contains("bounded"), "unexpected error: {error}");
    }

    #[test]
    fn bug1595_typed_observer_rejects_identity_and_origin_mutations() {
        use rspice_core::abort_signal::ImmediateAbort;

        let path = corpus_root().join(XYCE_BUG1595_EXPECTED_FAILURE_PATH);
        let source = fs::read_to_string(&path).expect("read canonical BUG1595 source");
        let observation =
            XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                &source,
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .expect("canonical BUG1595 typed payload is observed");
        assert_eq!(
            observation,
            XyceExpectedFailureKind::Bug1595HierarchicalMutualInductorReference
                .expected_observation()
        );

        let renamed = source.replace("K1 X1:L1 X1:L2 0.75", "K2 X1:L1 X1:L2 0.75");
        assert!(
            XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                &renamed,
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err(),
            "a renamed coupling must not satisfy the typed payload"
        );

        let shifted = source.replacen("\nK1 X1:L1", "\n\nK1 X1:L1", 1);
        assert!(
            XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                &shifted,
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err(),
            "a shifted physical origin must not satisfy the typed payload"
        );

        for mutation in [
            source.replace("K1 X1:L1 X1:L2 0.75", "K1 L1 L2 0.75"),
            source.replace("K1 X1:L1 X1:L2 0.75", "K1 X1:L2 X1:L1 0.75"),
            source.replace("K1 X1:L1 X1:L2 0.75", "K1 x1:l1 X1:L2 0.75"),
        ] {
            assert!(
                XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                    &mutation,
                    &path,
                    &rspice_core::abort_signal::NoAbort,
                )
                .is_err(),
                "corrected, reordered, and recased references must fail closed"
            );
        }

        let wrong_path = path.with_file_name("renamed.cir");
        assert!(
            XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                &source,
                &wrong_path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err(),
            "the observer must bind the authored filename"
        );
        let aborted =
            XyceTestRunner::observe_bug1595_hierarchical_mutual_inductor_reference_failure(
                &source,
                &path,
                &ImmediateAbort,
            )
            .expect_err("the typed parse must honor cooperative cancellation");
        assert!(
            aborted.contains("bounded"),
            "unexpected abort error: {aborted}"
        );
    }

    #[test]
    fn bug1595_provenance_rejects_family_role_exclusion_and_output_drift() {
        let (_temporary, deck, runner) = fixture("source-drift");
        fs::write(&deck.path, "* changed\n").expect("mutate BUG1595 source");
        assert!(
            runner
                .validate_bug1595_complete_family_provenance(
                    deck.path.parent().expect("BUG1595 deck has parent")
                )
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("extra-member");
        fs::write(
            deck.path
                .parent()
                .expect("BUG1595 deck has parent")
                .join("unexpected.out"),
            "stale output\n",
        )
        .expect("add unexpected BUG1595 member");
        assert!(
            runner
                .validate_bug1595_complete_family_provenance(
                    deck.path.parent().expect("BUG1595 deck has parent")
                )
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("options-drift");
        fs::write(
            deck.path
                .parent()
                .expect("BUG1595 deck has parent")
                .join("options"),
            "timelimit=31\n",
        )
        .expect("mutate BUG1595 timeout");
        assert!(
            runner
                .validate_bug1595_complete_family_provenance(
                    deck.path.parent().expect("BUG1595 deck has parent")
                )
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("oversized-member");
        fs::write(&deck.path, vec![b'*'; 2_000]).expect("write oversized BUG1595 source");
        assert!(
            runner
                .validate_bug1595_complete_family_provenance(
                    deck.path.parent().expect("BUG1595 deck has parent")
                )
                .is_err()
        );
        assert!(
            XyceTestRunner::read_bug1595_source_bounded(&deck.path).is_err(),
            "the direct observation read must also stay bounded"
        );

        let (_temporary, deck, runner) = fixture("manifest-drift");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove BUG1595 owner");
        assert!(
            runner
                .validate_bug1595_complete_family_provenance(
                    deck.path.parent().expect("BUG1595 deck has parent")
                )
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("exclusion-drift");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG1595_EXPECTED_FAILURE_PATH}\tNetlists/Certification_Tests/BUG_1595/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG1595 exclusion");
        assert!(
            runner
                .validate_bug1595_complete_family_provenance(
                    deck.path.parent().expect("BUG1595 deck has parent")
                )
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("output-drift");
        let output = runner.root.join("OutputData/Certification_Tests/BUG_1595");
        fs::create_dir_all(&output).expect("create forbidden BUG1595 OutputData");
        assert!(
            runner
                .validate_bug1595_complete_family_provenance(
                    deck.path.parent().expect("BUG1595 deck has parent")
                )
                .is_err()
        );
    }
}
