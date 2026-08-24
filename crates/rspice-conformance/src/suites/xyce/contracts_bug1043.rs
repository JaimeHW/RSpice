use super::*;

const LABEL: &str = "BUG_1043_SON AC DATA parameter family";

impl XyceTestRunner {
    pub(super) fn bug1043_ac_data_parameter_ac_comparator_tolerance() -> XyceAcComparatorTolerance {
        XyceAcComparatorTolerance::new(6.0e-5, 1.0e-4, 1.0e-6, 1.0e-6)
            .expect("Release 7.10 BUG_1043_SON ACComparator tolerance is valid")
    }

    pub(super) fn bug1043_ac_data_parameter_grid_matches(frequencies: &[Value]) -> bool {
        frequencies.len() == XYCE_BUG1043_FREQUENCY_GRID.len()
            && frequencies
                .iter()
                .copied()
                .zip(XYCE_BUG1043_FREQUENCY_GRID)
                .all(|(actual, expected)| {
                    actual.is_finite()
                        && (actual.to_bits() == expected.to_bits()
                            || (actual - expected).abs()
                                <= expected.abs() * XYCE_BUG1043_FREQUENCY_GRID_RELATIVE_ROUNDOFF)
                })
    }

    pub(super) fn bug1043_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG1043_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG1043_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG1043_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1043_historical_oracle_records(
        records: &[String],
    ) -> Result<(), String> {
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if XYCE_BUG1043_PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || records.len() != XYCE_BUG1043_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG1043_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG1043_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG1043_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release-7.10 wrapper/exclude/ACComparator provenance changed: pretrim={XYCE_BUG1043_PRETRIM_COMMIT}, records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug1043_historical_oracle_provenance() -> Result<(), String> {
        Self::validate_bug1043_historical_oracle_records(
            &Self::bug1043_historical_oracle_provenance_records(),
        )
    }

    fn validate_bug1043_artifact_directory(
        directory: &Path,
        expected_artifacts: &[(&str, usize, &str, &str)],
        purpose: &str,
    ) -> Result<(), String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {LABEL} {purpose} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} {purpose} directory must be a regular non-symlink directory"
            ));
        }
        let expected = expected_artifacts
            .iter()
            .copied()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        if expected.len() != expected_artifacts.len() {
            return Err(format!("{LABEL} {purpose} artifact specification collides"));
        }

        let mut observed = BTreeSet::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to enumerate {LABEL} {purpose}: {error}"))?
        {
            let entry = entry
                .map_err(|error| format!("failed to inspect {LABEL} {purpose} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {LABEL} {purpose} member: {error}"))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} {purpose} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} {purpose} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!("{LABEL} {purpose} contains a case collision"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{LABEL} acquired unexpected {purpose} member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{LABEL} {purpose} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {LABEL} {purpose} member: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} {purpose} member {name:?} content changed: bytes={}, sha256={sha256}, blake3={content_blake3}",
                    canonical.len()
                ));
            }
        }
        let expected_names = expected.keys().cloned().collect::<BTreeSet<_>>();
        if observed != expected_names {
            return Err(format!(
                "{LABEL} {purpose} census changed: expected {expected_names:?}, got {observed:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn bug1043_ac_data_parameter_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug1043AcDataParameterFamilyContract, String>> {
        let (spec, role) = XyceBug1043AcDataParameterRole::for_record(&deck.relative_path)?;
        Some((|| {
            if deck.section != XyceDeckSection::Netlists
                || Self::normalize_manifest_key(&self.relative_key(&deck.path))
                    != Self::normalize_manifest_key(&deck.relative_path)
            {
                return Err(format!(
                    "recognized {LABEL} record '{}' is not backed by its exact Netlists path",
                    deck.relative_path
                ));
            }
            let owner_path = self.root.join(spec.owner_path);
            let baseline_path = self.root.join(spec.baseline_path);
            let expected_path = match role {
                XyceBug1043AcDataParameterRole::DataWrapperOwner => &owner_path,
                XyceBug1043AcDataParameterRole::ExpressionBaseline => &baseline_path,
            };
            if !Self::same_path(&deck.path, expected_path) {
                return Err(format!(
                    "recognized {LABEL} role {role:?} is not backed by its canonical path"
                ));
            }
            let contract = XyceBug1043AcDataParameterFamilyContract {
                relational: XyceBaselineFamilyContract {
                    kind: XyceBaselineFamilyKind::Bug1043AcDataParameters,
                    comparison: XyceBaselineFamilyComparison::AcComparator(
                        Self::bug1043_ac_data_parameter_ac_comparator_tolerance(),
                    ),
                    family: spec.family.to_string(),
                    baseline_path: baseline_path.clone(),
                    member_paths: vec![owner_path.clone(), baseline_path.clone()],
                    target_path: Some(expected_path.clone()),
                },
                owner_path,
                baseline_path,
                spec,
                role,
            };
            self.validate_bug1043_ac_data_parameter_provenance(&contract)?;
            Ok(contract)
        })())
    }

    pub(super) fn validate_bug1043_ac_data_parameter_provenance(
        &self,
        contract: &XyceBug1043AcDataParameterFamilyContract,
    ) -> Result<(), String> {
        Self::validate_bug1043_historical_oracle_provenance()?;
        let expected_target = match contract.role {
            XyceBug1043AcDataParameterRole::DataWrapperOwner => &contract.owner_path,
            XyceBug1043AcDataParameterRole::ExpressionBaseline => &contract.baseline_path,
        };
        if contract.relational.kind != XyceBaselineFamilyKind::Bug1043AcDataParameters
            || contract.relational.comparison
                != XyceBaselineFamilyComparison::AcComparator(
                    Self::bug1043_ac_data_parameter_ac_comparator_tolerance(),
                )
            || contract.relational.family != contract.spec.family
            || !Self::same_path(&contract.relational.baseline_path, &contract.baseline_path)
            || contract.relational.member_paths.len() != 2
            || !Self::same_path(&contract.relational.member_paths[0], &contract.owner_path)
            || !Self::same_path(
                &contract.relational.member_paths[1],
                &contract.baseline_path,
            )
            || !contract
                .relational
                .target_path
                .as_ref()
                .is_some_and(|path| Self::same_path(path, expected_target))
            || contract
                .relational
                .kind
                .ac_comparator_member_is_good_waveform()
        {
            return Err(format!(
                "{LABEL} contract is not the exact expression-GOODFILE/DATA-owner-TESTFILE pair"
            ));
        }

        let prefix = "netlists/certification_tests/bug_1043_son/";
        let wrapper_records = Self::load_upstream_wrapper_decks(&self.root);
        let family_wrappers = wrapper_records
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let expected_wrappers =
            BTreeSet::from([XYCE_BUG1043_OWNER_RECORD, XYCE_BUG1043_ANALYTIC_RECORD]);
        let analytic_path = self.root.join(XYCE_BUG1043_ANALYTIC_PATH);
        if family_wrappers != expected_wrappers
            || Self::normalize_manifest_key(&self.relative_key(&analytic_path))
                != XYCE_BUG1043_ANALYTIC_RECORD
        {
            return Err(format!(
                "{LABEL} wrapper ownership changed: {family_wrappers:?}"
            ));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let family_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(prefix))
            .collect::<BTreeMap<_, _>>();
        let baseline_exclusion = family_exclusions
            .get(&XYCE_BUG1043_EXPRESSION_BASELINE_RECORD.to_string())
            .copied();
        if family_exclusions.len() != 1
            || baseline_exclusion.is_none_or(|exclusion| {
                Self::normalize_manifest_key(&exclusion.source)
                    != Self::normalize_manifest_key(XYCE_BUG1043_HISTORICAL_EXCLUDE_PATH)
                    || !matches!(
                        &exclusion.disposition,
                        XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                            expected_contract,
                        } if expected_contract
                            == XYCE_BUG1043_AC_DATA_PARAMETER_EXPRESSION_BASELINE_CONTRACT
                    )
            })
        {
            return Err(format!(
                "{LABEL} expression baseline lost its exact independent qualification: {family_exclusions:?}"
            ));
        }

        if contract.spec.owner_content_blake3 != XYCE_BUG1043_RETAINED_SOURCE_ARTIFACTS[0].3
            || contract.spec.baseline_content_blake3 != XYCE_BUG1043_RETAINED_SOURCE_ARTIFACTS[2].3
        {
            return Err(format!(
                "{LABEL} role/source identity specification changed"
            ));
        }
        Self::validate_bug1043_artifact_directory(
            &self.root.join(XYCE_BUG1043_FAMILY_DIR),
            &XYCE_BUG1043_RETAINED_SOURCE_ARTIFACTS,
            "source",
        )?;
        let output_directory = self
            .root
            .join("OutputData/Certification_Tests/BUG_1043_SON");
        Self::validate_bug1043_artifact_directory(
            &output_directory,
            &[XYCE_BUG1043_ANALYTIC_ORACLE_ARTIFACT],
            "analytic-oracle",
        )?;
        if !Self::same_path(
            &output_directory.join(XYCE_BUG1043_ANALYTIC_ORACLE_ARTIFACT.0),
            &self.root.join(XYCE_BUG1043_ANALYTIC_ORACLE_PATH),
        ) {
            return Err(format!("{LABEL} analytic oracle path changed"));
        }
        for path in [&contract.owner_path, &contract.baseline_path] {
            self.reject_wrapper_output_artifacts(path)
                .map_err(|error| format!("{LABEL} relational record {error}"))?;
        }

        let owner_plan = self
            .bug1043_relational_ac_plan_for_path(&contract.owner_path)
            .map_err(|error| format!("{LABEL} DATA owner plan failed: {error}"))?;
        let baseline_plan = self
            .bug1043_relational_ac_plan_for_path(&contract.baseline_path)
            .map_err(|error| format!("{LABEL} expression baseline plan failed: {error}"))?;
        Self::validate_bug1043_ac_data_parameter_ac_plan(&owner_plan)?;
        Self::validate_bug1043_ac_data_parameter_ac_plan(&baseline_plan)?;
        if !Self::bug1043_ac_data_parameter_grid_matches(&owner_plan.ac.frequencies)
            || !Self::bug1043_ac_data_parameter_grid_matches(&baseline_plan.ac.frequencies)
        {
            return Err(format!("{LABEL} six-row frequency grid changed"));
        }
        let owner_netlist = Self::relational_ac_plan_netlist_for_kind(
            XyceBaselineFamilyKind::Bug1043AcDataParameters,
            &owner_plan,
        )?;
        let baseline_netlist = Self::relational_ac_plan_netlist_for_kind(
            XyceBaselineFamilyKind::Bug1043AcDataParameters,
            &baseline_plan,
        )?;
        let owner_snapshot =
            Self::bug1043_ac_data_parameter_family_snapshot(&owner_plan, &owner_netlist)?;
        let baseline_snapshot =
            Self::bug1043_ac_data_parameter_family_snapshot(&baseline_plan, &baseline_netlist)?;
        Self::compare_bug1043_ac_data_parameter_snapshots(&baseline_snapshot, &owner_snapshot)
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

    fn canonical_deck(relative_path: &str) -> XyceDeck {
        XyceDeck {
            path: corpus_root().join(relative_path),
            section: XyceDeckSection::Netlists,
            relative_path: relative_path.to_string(),
        }
    }

    #[test]
    fn bug1043_historical_provenance_is_exact_and_rejects_drift() {
        let records = XyceTestRunner::bug1043_historical_oracle_provenance_records();
        XyceTestRunner::validate_bug1043_historical_oracle_records(&records)
            .expect("Release-7.10 BUG1043 provenance remains exact");
        let mut changed = records;
        changed[0].push('x');
        assert!(XyceTestRunner::validate_bug1043_historical_oracle_records(&changed).is_err());
    }

    #[test]
    fn bug1043_canonical_pair_has_directional_typed_contract() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for (relative_path, expected_role) in [
            (
                XYCE_BUG1043_OWNER_PATH,
                XyceBug1043AcDataParameterRole::DataWrapperOwner,
            ),
            (
                XYCE_BUG1043_EXPRESSION_BASELINE_PATH,
                XyceBug1043AcDataParameterRole::ExpressionBaseline,
            ),
        ] {
            let contract = runner
                .bug1043_ac_data_parameter_family_contract(&canonical_deck(relative_path))
                .expect("BUG1043 pair member is recognized")
                .unwrap_or_else(|error| panic!("canonical BUG1043 member failed: {error}"));
            assert_eq!(contract.role, expected_role);
            assert_eq!(
                contract.role.result_contract(),
                expected_role.result_contract()
            );
            assert!(
                !contract
                    .relational
                    .kind
                    .ac_comparator_member_is_good_waveform()
            );
        }
        assert!(
            runner
                .bug1043_ac_data_parameter_family_contract(&canonical_deck(
                    XYCE_BUG1043_ANALYTIC_PATH
                ))
                .is_none()
        );
    }
}
