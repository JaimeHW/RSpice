//! Captured expected state per deck family.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn validate_measure_step_find_when_provenance(
        &self,
        deck: &XyceDeck,
        member: XyceMeasureStepFindWhenMember,
    ) -> Result<Vec<(XyceMeasureStepFindWhenMember, Vec<u8>)>, String> {
        let expected_record = member.record();
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != expected_record
            || self.requires_upstream_wrapper(&deck.relative_path) != member.is_owner()
        {
            return Err(format!(
                "MEASURE STEP FIND/WHEN record '{expected_record}' lost exact owner/control ownership"
            ));
        }
        let canonical_deck = deck.path.canonicalize().map_err(|error| {
            format!("failed to canonicalize MEASURE STEP FIND/WHEN record: {error}")
        })?;
        let canonical_expected = self
            .root
            .join(member.source_relative_path())
            .canonicalize()
            .map_err(|error| {
                format!("canonical MEASURE STEP FIND/WHEN record is missing: {error}")
            })?;
        if canonical_deck != canonical_expected {
            return Err(
                "MEASURE STEP FIND/WHEN record resolved outside its canonical corpus path".into(),
            );
        }

        let base = self.root.join("Netlists/MEASURE/STEP");
        let mut family_names = Vec::new();
        for entry in fs::read_dir(&base)
            .map_err(|error| format!("failed to read {}: {error}", base.display()))?
        {
            let path = entry
                .map_err(|error| format!("failed to inspect {}: {error}", base.display()))?
                .path();
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                return Err("MEASURE STEP FIND/WHEN member name is not UTF-8".to_string());
            };
            if name.starts_with("FindWhenTest") {
                let metadata = fs::symlink_metadata(&path)
                    .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
                if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                    return Err(format!(
                        "MEASURE STEP FIND/WHEN member {} must be a regular non-symlink file",
                        path.display()
                    ));
                }
                family_names.push(name.to_string());
            }
        }
        family_names.sort();
        let expected_names = [
            "FindWhenTest.cir",
            "FindWhenTest.s0.cir",
            "FindWhenTest.s1.cir",
            "FindWhenTest.s2.cir",
            "FindWhenTest.s3.cir",
        ];
        if family_names != expected_names {
            return Err(format!(
                "MEASURE STEP FIND/WHEN case-sensitive family census changed: {family_names:?}"
            ));
        }

        let manifest_path = self.root.join(HARNESS_MANIFEST_FILE);
        let manifest_bytes = fs::read(&manifest_path)
            .map_err(|error| format!("failed to read {}: {error}", manifest_path.display()))?;
        let canonical_manifest =
            Self::canonical_lf_text_identity("MEASURE STEP FIND/WHEN manifest", &manifest_bytes)?;
        let manifest_text = std::str::from_utf8(&canonical_manifest)
            .map_err(|error| format!("MEASURE STEP FIND/WHEN manifest is not UTF-8: {error}"))?;
        let manifest_rows = manifest_text
            .lines()
            .filter(|line| line.starts_with("Netlists/MEASURE/STEP/FindWhenTest"))
            .collect::<Vec<_>>();
        if manifest_rows != ["Netlists/MEASURE/STEP/FindWhenTest.cir\trequires_upstream_wrapper"] {
            return Err(format!(
                "MEASURE STEP FIND/WHEN manifest ownership changed: {manifest_rows:?}"
            ));
        }

        let mut sources = Vec::with_capacity(XyceMeasureStepFindWhenMember::ALL.len());
        for candidate in XyceMeasureStepFindWhenMember::ALL {
            let bytes = Self::validate_measure_cont_regular_text_identity(
                &self.root.join(candidate.source_relative_path()),
                candidate.source_identity(),
                "MEASURE STEP FIND/WHEN source",
            )?;
            sources.push((candidate, bytes));
        }
        Ok(sources)
    }

    pub(super) fn validate_measure_cont_step_tran_provenance(
        &self,
        deck: &XyceDeck,
        member: XyceMeasureContStepTranMember,
    ) -> Result<Vec<(XyceMeasureContStepTranMember, Vec<u8>)>, String> {
        let expected_record = member.record();
        let owns_wrapper = member.role == XyceMeasureContStepTranRole::Main;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != expected_record
            || self.requires_upstream_wrapper(&deck.relative_path) != owns_wrapper
        {
            return Err(format!(
                "MEASURE_CONT STEP record '{expected_record}' lost exact owner/control ownership"
            ));
        }
        let canonical_deck = deck
            .path
            .canonicalize()
            .map_err(|error| format!("failed to canonicalize MEASURE_CONT STEP record: {error}"))?;
        let canonical_expected = self
            .root
            .join(member.source_relative_path())
            .canonicalize()
            .map_err(|error| format!("canonical MEASURE_CONT STEP record is missing: {error}"))?;
        if canonical_deck != canonical_expected {
            return Err(
                "MEASURE_CONT STEP record resolved outside its canonical corpus path".into(),
            );
        }

        self.validate_measure_cont_manifest_family()?;
        self.validate_measure_cont_family_census(
            "Netlists/MEASURE_CONT",
            XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_COUNT,
            XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_NAMES_BLAKE3,
            XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_CONTENT_BLAKE3,
        )?;
        self.validate_measure_cont_family_census(
            "OutputData/MEASURE_CONT",
            XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_COUNT,
            XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_NAMES_BLAKE3,
            XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_CONTENT_BLAKE3,
        )?;
        self.validate_measure_cont_step_case_sensitive_census()?;

        let mut paths = Vec::new();
        let mut content = Vec::new();
        let mut sources = Vec::new();
        for candidate in XyceMeasureContStepTranMember::ALL {
            let bytes = Self::validate_measure_cont_regular_text_identity(
                &self.root.join(candidate.source_relative_path()),
                candidate.source_identity(),
                "MEASURE_CONT STEP source",
            )?;
            paths.push(candidate.source_relative_path().to_string());
            content.push(format!(
                "{}\t{}",
                candidate.source_relative_path(),
                blake3::hash(&bytes).to_hex()
            ));
            sources.push((candidate, bytes));
        }
        paths.sort();
        content.sort();
        let path_hash = blake3::hash(paths.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(content.join("\n").as_bytes())
            .to_hex()
            .to_string();

        let manifest_path = self.root.join(HARNESS_MANIFEST_FILE);
        let manifest_bytes = fs::read(&manifest_path)
            .map_err(|error| format!("failed to read {}: {error}", manifest_path.display()))?;
        let canonical_manifest =
            Self::canonical_lf_text_identity("MEASURE_CONT STEP manifest", &manifest_bytes)?;
        let manifest_text = std::str::from_utf8(&canonical_manifest)
            .map_err(|error| format!("MEASURE_CONT STEP manifest is not UTF-8: {error}"))?;
        let mut manifest_rows = manifest_text
            .lines()
            .filter(|line| {
                line.starts_with("Netlists/MEASURE_CONT/STEP/DerivTestTran")
                    || line.starts_with("Netlists/MEASURE_CONT/STEP/FindWhenTestTran")
                    || line.starts_with("Netlists/MEASURE_CONT/STEP/TrigTargTestTran")
            })
            .map(str::to_string)
            .collect::<Vec<_>>();
        manifest_rows.sort();
        let manifest_hash = blake3::hash(manifest_rows.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if paths.len() != XyceMeasureContStepTranMember::ALL.len()
            || path_hash != XYCE_MEASURE_CONT_STEP_TRAN_CANDIDATE_BLAKE3
            || content_hash != XYCE_MEASURE_CONT_STEP_TRAN_CANDIDATE_CONTENT_BLAKE3
            || manifest_rows.len() != 3
            || manifest_rows
                .iter()
                .any(|line| line.contains(".s0.") || line.contains(".s1."))
            || manifest_hash != XYCE_MEASURE_CONT_STEP_TRAN_MANIFEST_BLAKE3
        {
            return Err(format!(
                "MEASURE_CONT STEP candidate/manifest provenance changed: paths={}/{path_hash}/{content_hash}, manifest={}/{manifest_hash}",
                paths.len(),
                manifest_rows.len()
            ));
        }

        Self::validate_measure_cont_step_historical_provenance()?;
        Ok(sources)
    }

    pub(super) fn validate_measure_cont_step_historical_provenance() -> Result<(), String> {
        let mut records = Vec::new();
        for (kind, path, content_blake3) in [
            (
                XyceMeasureContStepTranKind::Derivative,
                "Netlists/MEASURE_CONT/STEP/DerivTestTran.cir.sh",
                "7aab6d2e80f805d2509a7e33c673abc1fcee09aaae69d2b3f0e03a152f8bcaf6",
            ),
            (
                XyceMeasureContStepTranKind::FindWhen,
                "Netlists/MEASURE_CONT/STEP/FindWhenTestTran.cir.sh",
                "7aab6d2e80f805d2509a7e33c673abc1fcee09aaae69d2b3f0e03a152f8bcaf6",
            ),
            (
                XyceMeasureContStepTranKind::TriggerTarget,
                "Netlists/MEASURE_CONT/STEP/TrigTargTestTran.cir.sh",
                "b01705bf8909d2698b1569d99e621510f40cf3dab5d6702ab8f967fef21d5f79",
            ),
        ] {
            let (bytes, sha256) = XyceMeasureContStepTranMember::main(kind)
                .historical_wrapper_identity()
                .ok_or_else(|| "MEASURE_CONT STEP owner lost its wrapper identity".to_string())?;
            records.push(format!("{path}\t{bytes}\t{sha256}\t{content_blake3}"));
        }
        for (path, bytes, sha256, content_blake3) in [
            (
                "TestScripts/MeasureCommon.pm",
                44_922,
                "a8f47987c43ac63e7954b8a89cfaddb7edc8fbff50d5bbab43a57f417dde7c0d",
                "399e6580a8a17656d85913accc466d7f452a56f00507ff085e8696a5bca5aa83",
            ),
            (
                "TestScripts/file_compare.pl",
                7_465,
                "a700143baddab265ca2e74d69541432fb27ae66600c3fee71968797fc78efcbf0",
                "04dd69b4e4cfe543a39f663966229be877fa595a7c6c885dadf2173814f85895",
            ),
            (
                "XyceRegression/Tools.pm",
                68_108,
                "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
                "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
            ),
            (
                "TestScripts/xyce_verify.pl",
                59_566,
                "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
                "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
            ),
        ] {
            records.push(format!("{path}\t{bytes}\t{sha256}\t{content_blake3}"));
        }
        records.sort();
        let provenance_hash = blake3::hash(records.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if records.len() != 7
            || provenance_hash != XYCE_MEASURE_CONT_STEP_HISTORICAL_PROVENANCE_BLAKE3
        {
            return Err(format!(
                "MEASURE_CONT STEP Release-7.10 wrapper/tool provenance changed: records={}/{provenance_hash}",
                records.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_tran_provenance(
        &self,
        deck: &XyceDeck,
        kind: XyceMeasureContTranKind,
    ) -> Result<Vec<u8>, String> {
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != kind.record()
            || !self.requires_upstream_wrapper(&deck.relative_path)
        {
            return Err(format!(
                "MEASURE_CONT record '{}' lost exact removed-wrapper ownership",
                kind.record()
            ));
        }
        let canonical_deck = deck
            .path
            .canonicalize()
            .map_err(|error| format!("failed to canonicalize MEASURE_CONT record: {error}"))?;
        let canonical_expected = self
            .root
            .join(kind.source_relative_path())
            .canonicalize()
            .map_err(|error| format!("canonical MEASURE_CONT record is missing: {error}"))?;
        if canonical_deck != canonical_expected {
            return Err("MEASURE_CONT record resolved outside its canonical corpus path".into());
        }

        self.validate_measure_cont_manifest_family()?;
        self.validate_measure_cont_family_census(
            "Netlists/MEASURE_CONT",
            XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_COUNT,
            XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_NAMES_BLAKE3,
            XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_CONTENT_BLAKE3,
        )?;
        self.validate_measure_cont_family_census(
            "OutputData/MEASURE_CONT",
            XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_COUNT,
            XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_NAMES_BLAKE3,
            XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_CONTENT_BLAKE3,
        )?;

        let mut candidates = Vec::new();
        let mut candidate_content = Vec::new();
        let mut artifacts = Vec::new();
        let mut owner_bytes = None;
        for candidate in XyceMeasureContTranKind::ALL {
            let source_path = self.root.join(candidate.source_relative_path());
            let canonical = Self::validate_measure_cont_regular_text_identity(
                &source_path,
                candidate.source_identity(),
                "MEASURE_CONT source",
            )?;
            candidates.push(candidate.record().to_string());
            candidate_content.push(format!(
                "{}\t{}",
                candidate.record(),
                blake3::hash(&canonical).to_hex()
            ));
            if candidate == kind {
                owner_bytes = Some(canonical.clone());
            }
            for (relative, identity, label) in [
                (candidate.gs_relative_path(), candidate.gs_identity(), "GS"),
                (
                    candidate.mt0_relative_path(),
                    candidate.mt0_identity(),
                    "mt0",
                ),
            ] {
                let bytes = Self::validate_measure_cont_regular_text_identity(
                    &self.root.join(relative),
                    identity,
                    label,
                )?;
                artifacts.push(format!("{relative}\t{}", blake3::hash(&bytes).to_hex()));
            }
            if let Some((relative, identity)) = candidate.prn() {
                let bytes = Self::validate_measure_cont_regular_text_identity(
                    &self.root.join(relative),
                    identity,
                    "PRN",
                )?;
                artifacts.push(format!("{relative}\t{}", blake3::hash(&bytes).to_hex()));
            }
        }
        candidates.sort();
        candidate_content.sort();
        artifacts.sort();
        let candidate_hash = blake3::hash(candidates.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let candidate_content_hash = blake3::hash(candidate_content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let artifact_hash = blake3::hash(artifacts.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let manifest = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| candidates.binary_search(record).is_ok())
            .cloned()
            .collect::<Vec<_>>();
        let manifest_hash = blake3::hash(manifest.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if candidates.len() != XyceMeasureContTranKind::ALL.len()
            || candidate_hash != XYCE_MEASURE_CONT_TRAN_CANDIDATE_BLAKE3
            || candidate_content_hash != XYCE_MEASURE_CONT_TRAN_CANDIDATE_CONTENT_BLAKE3
            || artifact_hash != XYCE_MEASURE_CONT_TRAN_ARTIFACT_CONTENT_BLAKE3
            || manifest != candidates
            || manifest_hash != XYCE_MEASURE_CONT_TRAN_MANIFEST_BLAKE3
        {
            return Err(format!(
                "MEASURE_CONT candidate/artifact/manifest provenance changed: candidates={}/{candidate_hash}/{candidate_content_hash}, artifacts={}/{artifact_hash}, manifest={}/{manifest_hash}",
                candidates.len(),
                artifacts.len(),
                manifest.len()
            ));
        }
        Self::validate_measure_cont_historical_identities()?;
        owner_bytes.ok_or_else(|| "MEASURE_CONT owner was not covered by provenance".to_string())
    }

    pub(super) fn validate_abm_transient_provenance(
        &self,
        deck: &XyceDeck,
        kind: XyceAbmTransientKind,
    ) -> Result<Vec<u8>, String> {
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != kind.record()
            || !self.requires_upstream_wrapper(&deck.relative_path)
        {
            return Err(format!(
                "ABM transient record '{}' lost exact removed-wrapper ownership",
                kind.record()
            ));
        }
        let canonical_deck = deck
            .path
            .canonicalize()
            .map_err(|error| format!("failed to canonicalize ABM transient record: {error}"))?;
        let canonical_expected = self
            .root
            .join(Path::new(kind.source_relative_path()))
            .canonicalize()
            .map_err(|error| format!("canonical ABM transient record is missing: {error}"))?;
        if canonical_deck != canonical_expected {
            return Err("ABM transient record resolved outside its canonical corpus path".into());
        }

        for (prefix, count, names_hash, content_hash) in [
            (
                XYCE_ABM_TRANSIENT_TIME_FAMILY_PREFIX,
                XYCE_ABM_TRANSIENT_TIME_DIRECTORY_COUNT,
                XYCE_ABM_TRANSIENT_TIME_DIRECTORY_BLAKE3,
                XYCE_ABM_TRANSIENT_TIME_CONTENT_BLAKE3,
            ),
            (
                XYCE_ABM_TRANSIENT_SQRT_FAMILY_PREFIX,
                XYCE_ABM_TRANSIENT_SQRT_DIRECTORY_COUNT,
                XYCE_ABM_TRANSIENT_SQRT_DIRECTORY_BLAKE3,
                XYCE_ABM_TRANSIENT_SQRT_CONTENT_BLAKE3,
            ),
        ] {
            self.validate_abm_transient_family_census(prefix, count, names_hash, content_hash)?;
        }

        let mut candidates = BTreeSet::new();
        let mut candidate_content = BTreeSet::new();
        for candidate in XyceAbmTransientKind::ALL {
            let path = self.root.join(Path::new(candidate.source_relative_path()));
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                format!(
                    "ABM transient candidate {} is missing: {error}",
                    path.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "ABM transient candidate {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read ABM transient candidate: {error}"))?;
            let canonical_bytes = Self::canonical_lf_text_identity(
                &format!("ABM transient source {}", candidate.record()),
                &bytes,
            )?;
            Self::validate_xdm_replaceground_identity(
                "ABM transient source",
                candidate.record(),
                &canonical_bytes,
                candidate.source_identity(),
            )?;
            candidates.insert(candidate.record().to_string());
            candidate_content.insert(format!(
                "{}\t{}",
                candidate.record(),
                blake3::hash(&canonical_bytes).to_hex()
            ));
            self.reject_wrapper_output_artifacts(&path)?;
        }
        let candidates = candidates.into_iter().collect::<Vec<_>>();
        let candidate_content = candidate_content.into_iter().collect::<Vec<_>>();
        let candidate_hash = blake3::hash(candidates.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let candidate_content_hash = blake3::hash(candidate_content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let manifest = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| {
                record.starts_with(XYCE_ABM_TRANSIENT_TIME_FAMILY_PREFIX)
                    || record.starts_with(XYCE_ABM_TRANSIENT_SQRT_FAMILY_PREFIX)
            })
            .cloned()
            .collect::<Vec<_>>();
        let manifest_hash = blake3::hash(manifest.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if candidates.len() != XyceAbmTransientKind::ALL.len()
            || candidate_hash != XYCE_ABM_TRANSIENT_CANDIDATE_BLAKE3
            || candidate_content_hash != XYCE_ABM_TRANSIENT_CANDIDATE_CONTENT_BLAKE3
            || manifest != candidates
            || manifest_hash != XYCE_ABM_TRANSIENT_MANIFEST_BLAKE3
        {
            return Err(format!(
                "ABM transient candidate/manifest bijection changed: candidates={}/{candidate_hash}/{candidate_content_hash}, manifest={}/{manifest_hash}",
                candidates.len(),
                manifest.len()
            ));
        }
        Self::validate_abm_transient_historical_identities()?;
        fs::read(&deck.path).map_err(|error| format!("failed to read ABM transient owner: {error}"))
    }

    pub(super) fn validate_abm_pow_provenance(
        &self,
        deck: &XyceDeck,
        kind: XyceAbmPowKind,
    ) -> Result<Vec<u8>, String> {
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != kind.record()
            || !self.requires_upstream_wrapper(&deck.relative_path)
        {
            return Err(format!(
                "ABM_POW record '{}' lost exact removed-wrapper ownership",
                kind.record()
            ));
        }
        let canonical_deck = deck
            .path
            .canonicalize()
            .map_err(|error| format!("failed to canonicalize ABM_POW record: {error}"))?;
        let canonical_expected = self
            .root
            .join(Path::new(kind.source_relative_path()))
            .canonicalize()
            .map_err(|error| format!("canonical ABM_POW record is missing: {error}"))?;
        if canonical_deck != canonical_expected {
            return Err("ABM_POW record resolved outside its canonical corpus path".into());
        }
        let family = canonical_expected
            .parent()
            .ok_or_else(|| "ABM_POW record has no family directory".to_string())?;
        let family_metadata = fs::symlink_metadata(family)
            .map_err(|error| format!("failed to inspect ABM_POW family: {error}"))?;
        if !family_metadata.file_type().is_dir() || family_metadata.file_type().is_symlink() {
            return Err("ABM_POW family must be a regular non-symlink directory".into());
        }

        let mut complete = BTreeSet::new();
        let mut content = BTreeSet::new();
        for entry in fs::read_dir(family)
            .map_err(|error| format!("failed to read ABM_POW family: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect ABM_POW member: {error}"))?;
            let metadata = fs::symlink_metadata(entry.path()).map_err(|error| {
                format!(
                    "failed to inspect ABM_POW member {}: {error}",
                    entry.path().display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "ABM_POW member {} must be a regular non-symlink file",
                    entry.path().display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| "ABM_POW filename is not UTF-8".to_string())?
                .to_ascii_lowercase();
            if !complete.insert(name.clone()) {
                return Err(format!("ABM_POW family has case-colliding name {name:?}"));
            }
            let bytes = fs::read(entry.path())
                .map_err(|error| format!("failed to hash ABM_POW member: {error}"))?;
            content.insert(format!("{name}\0{}", blake3::hash(&bytes).to_hex()));
        }
        let complete = complete.into_iter().collect::<Vec<_>>();
        let content = content.into_iter().collect::<Vec<_>>();
        let complete_hash = blake3::hash(complete.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if complete.len() != XYCE_ABM_POW_SOURCE_DIRECTORY_COUNT
            || complete_hash != XYCE_ABM_POW_SOURCE_DIRECTORY_BLAKE3
            || content.len() != XYCE_ABM_POW_SOURCE_DIRECTORY_COUNT
            || content_hash != XYCE_ABM_POW_SOURCE_CONTENT_CENSUS_BLAKE3
        {
            return Err(format!(
                "ABM_POW family census changed: complete={}/{complete_hash}, content={}/{content_hash}",
                complete.len(),
                content.len()
            ));
        }

        let mut candidates = BTreeSet::new();
        let mut candidate_content = BTreeSet::new();
        for candidate in XyceAbmPowKind::ALL {
            let path = self.root.join(Path::new(candidate.source_relative_path()));
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                format!("ABM_POW candidate {} is missing: {error}", path.display())
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "ABM_POW candidate {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read ABM_POW candidate: {error}"))?;
            Self::validate_xdm_replaceground_identity(
                "ABM_POW source",
                candidate.record(),
                &bytes,
                candidate.source_identity(),
            )?;
            candidates.insert(candidate.record().to_string());
            candidate_content.insert(format!(
                "{}\t{}",
                candidate.record(),
                blake3::hash(&bytes).to_hex()
            ));
            self.reject_wrapper_output_artifacts(&path)?;
        }
        let candidates = candidates.into_iter().collect::<Vec<_>>();
        let candidate_content = candidate_content.into_iter().collect::<Vec<_>>();
        let candidate_hash = blake3::hash(candidates.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let candidate_content_hash = blake3::hash(candidate_content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let manifest = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(XYCE_ABM_POW_FAMILY_PREFIX))
            .cloned()
            .collect::<Vec<_>>();
        let manifest_hash = blake3::hash(manifest.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if candidates.len() != XyceAbmPowKind::ALL.len()
            || candidate_hash != XYCE_ABM_POW_CANDIDATE_BLAKE3
            || candidate_content_hash != XYCE_ABM_POW_CANDIDATE_CONTENT_BLAKE3
            || manifest != candidates
            || manifest_hash != XYCE_ABM_POW_MANIFEST_BLAKE3
        {
            return Err(format!(
                "ABM_POW candidate/manifest bijection changed: candidates={}/{candidate_hash}/{candidate_content_hash}, manifest={}/{manifest_hash}",
                candidates.len(),
                manifest.len()
            ));
        }
        Self::validate_abm_pow_historical_identities()?;
        fs::read(&deck.path).map_err(|error| format!("failed to read ABM_POW owner: {error}"))
    }

    pub(super) fn xdm_replaceground_element_snapshot(
        netlist: &Netlist,
        kind: XyceXdmReplaceGroundKind,
    ) -> Result<Vec<XyceXdmReplaceGroundElementSnapshot>, String> {
        let flattened = flatten_netlist(netlist)
            .map_err(|error| format!("failed to flatten XDM REPLACEGROUND circuit: {error}"))?;
        let mut snapshot = Vec::with_capacity(flattened.len());
        for element in flattened {
            let (kind_name, value) = match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } if value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty() =>
                {
                    ("R", *value)
                }
                ElementKind::VoltageSource(spec) if extract_dc_value(spec).is_finite() => {
                    ("V:DC", extract_dc_value(spec))
                }
                other => {
                    return Err(format!(
                        "XDM REPLACEGROUND circuit contains an element outside its resistor/DC-source envelope: {} {other:?}",
                        element.name
                    ));
                }
            };
            if element.nodes.len() != 2 {
                return Err(format!(
                    "XDM REPLACEGROUND element '{}' is not two-terminal",
                    element.name
                ));
            }
            snapshot.push(XyceXdmReplaceGroundElementSnapshot {
                name: element.name.to_ascii_lowercase(),
                nodes: element
                    .nodes
                    .iter()
                    .map(|node| node.to_ascii_lowercase())
                    .collect(),
                kind: kind_name.to_string(),
                value_bits: value.to_bits(),
            });
        }
        snapshot.sort_by(|left, right| left.name.cmp(&right.name));
        let expected_count = if kind.requires_subcircuit() { 5 } else { 3 };
        if snapshot.len() != expected_count {
            return Err(format!(
                "XDM REPLACEGROUND circuit flattened to {} elements, expected {expected_count}: {snapshot:?}",
                snapshot.len()
            ));
        }
        Ok(snapshot)
    }

    pub(super) fn validate_xdm_replaceground_provenance(
        &self,
        deck: &XyceDeck,
        kind: XyceXdmReplaceGroundKind,
    ) -> Result<PathBuf, String> {
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != kind.record()
        {
            return Err(format!(
                "XDM REPLACEGROUND deck path does not match its recognized record: {}",
                deck.relative_path
            ));
        }
        let canonical_deck = deck.path.canonicalize().map_err(|error| {
            format!(
                "failed to canonicalize XDM REPLACEGROUND deck {}: {error}",
                deck.path.display()
            )
        })?;
        let canonical_expected = self
            .root
            .join(Path::new(&deck.relative_path))
            .canonicalize()
            .map_err(|error| {
                format!(
                    "canonical XDM REPLACEGROUND corpus record '{}' is missing: {error}",
                    kind.record()
                )
            })?;
        if canonical_deck != canonical_expected {
            return Err(format!(
                "XDM REPLACEGROUND record '{}' resolved outside its canonical corpus path",
                kind.record()
            ));
        }
        if !self.requires_upstream_wrapper(&deck.relative_path) {
            return Err(format!(
                "XDM REPLACEGROUND record '{}' lost removed-wrapper manifest provenance",
                kind.record()
            ));
        }
        let family_dir = deck
            .path
            .parent()
            .ok_or_else(|| "XDM REPLACEGROUND record has no family directory".to_string())?;
        let expected_family = self
            .root
            .join("Netlists/XDM/HSPICE/OTHER_PARSING")
            .canonicalize()
            .map_err(|error| format!("XDM REPLACEGROUND family is missing: {error}"))?;
        if family_dir.canonicalize().ok() != Some(expected_family) {
            return Err(format!(
                "XDM REPLACEGROUND family resolved outside canonical OTHER_PARSING: {}",
                family_dir.display()
            ));
        }

        let mut complete = BTreeSet::new();
        let mut content_census = BTreeSet::new();
        let mut physical = BTreeSet::new();
        let mut candidates = BTreeSet::new();
        for entry in fs::read_dir(family_dir).map_err(|error| {
            format!(
                "failed to inspect XDM REPLACEGROUND family {}: {error}",
                family_dir.display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!("failed to inspect XDM REPLACEGROUND family entry: {error}")
            })?;
            let metadata = fs::symlink_metadata(entry.path()).map_err(|error| {
                format!(
                    "failed to inspect XDM REPLACEGROUND family member {}: {error}",
                    entry.path().display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "XDM REPLACEGROUND family member {} must be a regular non-symlink file",
                    entry.path().display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| {
                    format!(
                        "XDM REPLACEGROUND family filename in {} is not UTF-8",
                        family_dir.display()
                    )
                })?
                .to_ascii_lowercase();
            if !complete.insert(name.clone()) {
                return Err(format!(
                    "XDM REPLACEGROUND family contains case-colliding name {name:?}"
                ));
            }
            let member_bytes = fs::read(entry.path()).map_err(|error| {
                format!(
                    "failed to hash XDM REPLACEGROUND family member {}: {error}",
                    entry.path().display()
                )
            })?;
            content_census.insert(format!("{name}\0{}", blake3::hash(&member_bytes).to_hex()));
            if name.ends_with(".cir") {
                physical.insert(name.clone());
                let source = fs::read_to_string(entry.path()).map_err(|error| {
                    format!(
                        "failed to read XDM REPLACEGROUND family candidate {}: {error}",
                        entry.path().display()
                    )
                })?;
                if Self::validate_xdm_replaceground_directives(&source, 1, true).is_ok() {
                    let mut paired_name = entry.file_name();
                    paired_name.push(".hspice");
                    if family_dir.join(paired_name).is_file() {
                        candidates.insert(name);
                    }
                }
            }
        }
        let complete = complete.into_iter().collect::<Vec<_>>();
        let content_census = content_census.into_iter().collect::<Vec<_>>();
        let physical = physical.into_iter().collect::<Vec<_>>();
        let candidates = candidates.into_iter().collect::<Vec<_>>();
        let complete_hash = blake3::hash(complete.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(content_census.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let physical_hash = blake3::hash(physical.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let candidate_hash = blake3::hash(candidates.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if complete.len() != XYCE_XDM_REPLACEGROUND_SOURCE_DIRECTORY_COUNT
            || complete_hash != XYCE_XDM_REPLACEGROUND_SOURCE_DIRECTORY_BLAKE3
        {
            return Err(format!(
                "XDM HSPICE OTHER_PARSING source-directory census changed: expected {} / {}, got {} / {complete_hash}",
                XYCE_XDM_REPLACEGROUND_SOURCE_DIRECTORY_COUNT,
                XYCE_XDM_REPLACEGROUND_SOURCE_DIRECTORY_BLAKE3,
                complete.len()
            ));
        }
        if content_census.len() != XYCE_XDM_REPLACEGROUND_SOURCE_DIRECTORY_COUNT
            || content_hash != XYCE_XDM_REPLACEGROUND_SOURCE_CONTENT_CENSUS_BLAKE3
        {
            return Err(format!(
                "XDM HSPICE OTHER_PARSING path+content census changed: expected {} / {}, got {} / {content_hash}",
                XYCE_XDM_REPLACEGROUND_SOURCE_DIRECTORY_COUNT,
                XYCE_XDM_REPLACEGROUND_SOURCE_CONTENT_CENSUS_BLAKE3,
                content_census.len()
            ));
        }
        if physical.len() != XYCE_XDM_REPLACEGROUND_PHYSICAL_COUNT
            || physical_hash != XYCE_XDM_REPLACEGROUND_PHYSICAL_BLAKE3
        {
            return Err(format!(
                "XDM HSPICE OTHER_PARSING physical census changed: expected {} / {}, got {} / {physical_hash}",
                XYCE_XDM_REPLACEGROUND_PHYSICAL_COUNT,
                XYCE_XDM_REPLACEGROUND_PHYSICAL_BLAKE3,
                physical.len()
            ));
        }
        if candidates.len() != XYCE_XDM_REPLACEGROUND_CANDIDATE_COUNT
            || candidate_hash != XYCE_XDM_REPLACEGROUND_CANDIDATE_BLAKE3
        {
            return Err(format!(
                "XDM HSPICE REPLACEGROUND candidate census changed: expected {} / {}, got {} / {candidate_hash}: {candidates:?}",
                XYCE_XDM_REPLACEGROUND_CANDIDATE_COUNT,
                XYCE_XDM_REPLACEGROUND_CANDIDATE_BLAKE3,
                candidates.len()
            ));
        }

        let manifest_records = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(XYCE_XDM_REPLACEGROUND_FAMILY_PREFIX))
            .cloned()
            .collect::<Vec<_>>();
        let manifest_hash = blake3::hash(manifest_records.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if manifest_records.len() != XYCE_XDM_REPLACEGROUND_MANIFEST_COUNT
            || manifest_hash != XYCE_XDM_REPLACEGROUND_MANIFEST_BLAKE3
        {
            return Err(format!(
                "XDM HSPICE OTHER_PARSING manifest census changed: expected {} / {}, got {} / {manifest_hash}",
                XYCE_XDM_REPLACEGROUND_MANIFEST_COUNT,
                XYCE_XDM_REPLACEGROUND_MANIFEST_BLAKE3,
                manifest_records.len()
            ));
        }
        let manifest_names = manifest_records
            .iter()
            .map(|record| {
                record
                    .rsplit_once('/')
                    .map(|(_, name)| name.to_ascii_lowercase())
                    .ok_or_else(|| {
                        format!("XDM REPLACEGROUND manifest record {record:?} has no filename")
                    })
            })
            .collect::<Result<BTreeSet<_>, _>>()?
            .into_iter()
            .collect::<Vec<_>>();
        if manifest_names != physical {
            return Err(format!(
                "XDM HSPICE OTHER_PARSING manifest/physical census is not a bijection: physical={physical:?}, manifest={manifest_names:?}"
            ));
        }
        if !manifest_records
            .iter()
            .any(|record| record == kind.record())
        {
            return Err(format!(
                "XDM REPLACEGROUND record '{}' is absent from its pinned manifest family",
                kind.record()
            ));
        }
        if family_dir.join("options").exists() {
            return Err(
                "XDM HSPICE REPLACEGROUND family unexpectedly contains an options sidecar"
                    .to_string(),
            );
        }
        let mut hspice_os = deck.path.as_os_str().to_os_string();
        hspice_os.push(".hspice");
        let hspice_path = PathBuf::from(hspice_os);
        for path in [&deck.path, &hspice_path] {
            let metadata = fs::symlink_metadata(path).map_err(|error| {
                format!(
                    "failed to inspect XDM REPLACEGROUND source {}: {error}",
                    path.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "XDM REPLACEGROUND source {} must be a regular non-symlink file",
                    path.display()
                ));
            }
        }
        self.reject_xdm_replaceground_output_artifacts(&deck.path)?;
        Ok(hspice_path)
    }

    pub(super) fn addresistors_element_snapshot(
        element: &rspice_core::netlist::Element,
    ) -> Result<XyceAddResistorsElementSnapshot, String> {
        let (kind, value, initial, model) = match &element.kind {
            ElementKind::Resistor { value, model, .. } => {
                ("R", Some(*value), None, model.as_deref())
            }
            ElementKind::Capacitor {
                value,
                initial_voltage,
                model,
                ..
            } => ("C", Some(*value), *initial_voltage, model.as_deref()),
            ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) => {
                ("V", Some(*value), None, None)
            }
            ElementKind::Subcircuit { subckt_name, .. } => {
                ("X", None, None, Some(subckt_name.as_str()))
            }
            other => {
                return Err(format!(
                    "ADDRESISTORS bounded snapshot cannot represent {} {other:?}",
                    element.name
                ));
            }
        };
        if value.is_some_and(|value| !value.is_finite())
            || initial.is_some_and(|value| !value.is_finite())
        {
            return Err(format!(
                "ADDRESISTORS element {} contains a non-finite primary value",
                element.name
            ));
        }
        let provenance = match element.provenance {
            rspice_core::netlist::ElementProvenance::Authored => "authored".to_string(),
            rspice_core::netlist::ElementProvenance::GeneratedXyceAddResistor { mode } => {
                format!("generated:{mode:?}")
            }
            ref other => format!("unexpected:{other:?}"),
        };
        Ok(XyceAddResistorsElementSnapshot {
            name: element.name.to_ascii_lowercase(),
            nodes: element
                .nodes
                .iter()
                .map(|node| node.to_ascii_lowercase())
                .collect(),
            kind: kind.to_string(),
            value_bits: value.map(Value::to_bits),
            initial_value_bits: initial.map(Value::to_bits),
            model: model.map(str::to_ascii_lowercase),
            provenance,
        })
    }

    pub(super) fn validate_addresistors_provenance(
        &self,
        deck: &XyceDeck,
        kind: XyceAddResistorsKind,
    ) -> Result<Vec<u8>, String> {
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != kind.record()
            || !self.requires_upstream_wrapper(&deck.relative_path)
        {
            return Err(format!(
                "ADDRESISTORS record '{}' lost canonical removed-wrapper ownership",
                kind.record()
            ));
        }
        let canonical_deck = deck.path.canonicalize().map_err(|error| {
            format!(
                "failed to canonicalize ADDRESISTORS record {}: {error}",
                deck.path.display()
            )
        })?;
        let canonical_expected = self
            .root
            .join(Path::new(&deck.relative_path))
            .canonicalize()
            .map_err(|error| {
                format!(
                    "canonical ADDRESISTORS record '{}' is missing: {error}",
                    kind.record()
                )
            })?;
        if canonical_deck != canonical_expected {
            return Err(format!(
                "ADDRESISTORS record resolved outside its canonical corpus path: {}",
                deck.path.display()
            ));
        }
        let metadata = fs::symlink_metadata(&deck.path)
            .map_err(|error| format!("failed to inspect ADDRESISTORS source: {error}"))?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err("ADDRESISTORS source must be a regular non-symlink file".to_string());
        }
        let source_bytes = fs::read(&deck.path).map_err(|error| {
            format!(
                "failed to read ADDRESISTORS record {}: {error}",
                deck.path.display()
            )
        })?;
        Self::validate_xdm_replaceground_identity(
            "ADDRESISTORS source",
            kind.record(),
            &source_bytes,
            kind.source_identity(),
        )?;

        self.validate_addresistors_family_census(
            "PREPROC_ADDRES",
            XYCE_ADDRESISTORS_PREPROC_SOURCE_DIRECTORY_COUNT,
            XYCE_ADDRESISTORS_PREPROC_SOURCE_DIRECTORY_BLAKE3,
            XYCE_ADDRESISTORS_PREPROC_SOURCE_CONTENT_CENSUS_BLAKE3,
            XYCE_ADDRESISTORS_PREPROC_PHYSICAL_COUNT,
            XYCE_ADDRESISTORS_PREPROC_PHYSICAL_BLAKE3,
            XYCE_ADDRESISTORS_PREPROC_FAMILY_PREFIX,
            XYCE_ADDRESISTORS_PREPROC_MANIFEST_COUNT,
            XYCE_ADDRESISTORS_PREPROC_MANIFEST_BLAKE3,
        )?;
        self.validate_addresistors_family_census(
            "REDUND_REMOVE",
            XYCE_REMOVEUNUSED_SOURCE_DIRECTORY_COUNT,
            XYCE_REMOVEUNUSED_SOURCE_DIRECTORY_BLAKE3,
            XYCE_REMOVEUNUSED_SOURCE_CONTENT_CENSUS_BLAKE3,
            XYCE_REMOVEUNUSED_PHYSICAL_COUNT,
            XYCE_REMOVEUNUSED_PHYSICAL_BLAKE3,
            XYCE_REMOVEUNUSED_FAMILY_PREFIX,
            XYCE_REMOVEUNUSED_MANIFEST_COUNT,
            XYCE_REMOVEUNUSED_MANIFEST_BLAKE3,
        )?;

        let mut candidates = BTreeSet::new();
        let mut candidate_content = BTreeSet::new();
        for candidate in XyceAddResistorsKind::ALL {
            let record = candidate.record();
            let path = self.root.join(Path::new(candidate.source_relative_path()));
            let bytes = fs::read(&path).map_err(|error| {
                format!(
                    "ADDRESISTORS candidate {} is missing: {error}",
                    path.display()
                )
            })?;
            Self::validate_xdm_replaceground_identity(
                "ADDRESISTORS candidate",
                record,
                &bytes,
                candidate.source_identity(),
            )?;
            candidates.insert(record.to_string());
            candidate_content.insert(format!("{record}\t{}", blake3::hash(&bytes).to_hex()));
            self.reject_addresistors_output_artifacts(&path)?;
        }
        let candidates = candidates.into_iter().collect::<Vec<_>>();
        let candidate_content = candidate_content.into_iter().collect::<Vec<_>>();
        let candidate_hash = blake3::hash(candidates.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let candidate_content_hash = blake3::hash(candidate_content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if candidates.len() != XYCE_ADDRESISTORS_CANDIDATE_COUNT
            || candidate_hash != XYCE_ADDRESISTORS_CANDIDATE_BLAKE3
            || candidate_content_hash != XYCE_ADDRESISTORS_CANDIDATE_CONTENT_BLAKE3
        {
            return Err(format!(
                "ADDRESISTORS candidate census changed: names={}/{candidate_hash}, content={candidate_content_hash}",
                candidates.len()
            ));
        }
        Ok(source_bytes)
    }

    pub(super) fn removeunused_element_snapshot(
        element: &rspice_core::netlist::Element,
    ) -> Result<XyceRemoveUnusedElementSnapshot, String> {
        let (kind, value, model) = match &element.kind {
            ElementKind::Resistor { value, model, .. } => ("R", Some(*value), model.as_deref()),
            ElementKind::Capacitor { value, model, .. } => ("C", Some(*value), model.as_deref()),
            ElementKind::Inductor { value, model, .. } => ("L", Some(*value), model.as_deref()),
            ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) => {
                ("V", Some(*value), None)
            }
            ElementKind::CurrentSource(rspice_core::netlist::SourceSpec::Dc(value)) => {
                ("I", Some(*value), None)
            }
            ElementKind::Diode { model, .. } => ("D", None, Some(model.as_str())),
            ElementKind::Mosfet { model, .. } => ("M", None, Some(model.as_str())),
            ElementKind::Bjt { model, .. } => ("Q", None, Some(model.as_str())),
            ElementKind::Subcircuit { subckt_name, .. } => ("X", None, Some(subckt_name.as_str())),
            other => {
                return Err(format!(
                    "REMOVEUNUSED bounded snapshot cannot represent {} {other:?}",
                    element.name
                ));
            }
        };
        if value.is_some_and(|value| !value.is_finite()) {
            return Err(format!(
                "REMOVEUNUSED element {} has a non-finite primary value",
                element.name
            ));
        }
        Ok(XyceRemoveUnusedElementSnapshot {
            name: element.name.to_ascii_lowercase(),
            nodes: element
                .nodes
                .iter()
                .map(|node| node.to_ascii_lowercase())
                .collect(),
            kind: kind.to_string(),
            value_bits: value.map(Value::to_bits),
            model: model.map(str::to_ascii_lowercase),
        })
    }

    pub(super) fn validate_removeunused_provenance(
        &self,
        deck: &XyceDeck,
        kind: XyceRemoveUnusedKind,
    ) -> Result<(), String> {
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != kind.record()
        {
            return Err(format!(
                "REMOVEUNUSED deck path does not match its recognized record: {}",
                deck.relative_path
            ));
        }
        let canonical_deck = deck.path.canonicalize().map_err(|error| {
            format!(
                "failed to canonicalize REMOVEUNUSED deck {}: {error}",
                deck.path.display()
            )
        })?;
        let canonical_expected = self
            .root
            .join(Path::new(&deck.relative_path))
            .canonicalize()
            .map_err(|error| {
                format!(
                    "canonical REMOVEUNUSED record '{}' is missing: {error}",
                    kind.record()
                )
            })?;
        if canonical_deck != canonical_expected
            || !self.requires_upstream_wrapper(&deck.relative_path)
        {
            return Err(format!(
                "REMOVEUNUSED record '{}' lost canonical removed-wrapper provenance",
                kind.record()
            ));
        }
        let metadata = fs::symlink_metadata(&deck.path)
            .map_err(|error| format!("failed to inspect REMOVEUNUSED source: {error}"))?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err("REMOVEUNUSED source must be a regular non-symlink file".to_string());
        }
        let family_dir = deck
            .path
            .parent()
            .ok_or_else(|| "REMOVEUNUSED record has no family directory".to_string())?;
        let expected_family = self
            .root
            .join("Netlists/REDUND_REMOVE")
            .canonicalize()
            .map_err(|error| format!("REMOVEUNUSED family is missing: {error}"))?;
        if family_dir.canonicalize().ok() != Some(expected_family) {
            return Err(format!(
                "REMOVEUNUSED family resolved outside canonical REDUND_REMOVE: {}",
                family_dir.display()
            ));
        }

        let mut complete = BTreeSet::new();
        let mut content = BTreeSet::new();
        let mut physical = BTreeSet::new();
        for entry in fs::read_dir(family_dir)
            .map_err(|error| format!("failed to inspect REMOVEUNUSED family: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect REMOVEUNUSED member: {error}"))?;
            let member_metadata = fs::symlink_metadata(entry.path()).map_err(|error| {
                format!(
                    "failed to inspect REMOVEUNUSED member {}: {error}",
                    entry.path().display()
                )
            })?;
            if !member_metadata.file_type().is_file() || member_metadata.file_type().is_symlink() {
                return Err(format!(
                    "REMOVEUNUSED family member {} must be a regular non-symlink file",
                    entry.path().display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| "REMOVEUNUSED family filename is not UTF-8".to_string())?
                .to_ascii_lowercase();
            if !complete.insert(name.clone()) {
                return Err(format!(
                    "REMOVEUNUSED family contains case-colliding name {name:?}"
                ));
            }
            let bytes = fs::read(entry.path()).map_err(|error| {
                format!(
                    "failed to hash REMOVEUNUSED member {}: {error}",
                    entry.path().display()
                )
            })?;
            content.insert(format!("{name}\0{}", blake3::hash(&bytes).to_hex()));
            if name.ends_with(".cir") {
                physical.insert(name);
            }
        }
        let complete = complete.into_iter().collect::<Vec<_>>();
        let content = content.into_iter().collect::<Vec<_>>();
        let physical = physical.into_iter().collect::<Vec<_>>();
        let complete_hash = blake3::hash(complete.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let physical_hash = blake3::hash(physical.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if complete.len() != XYCE_REMOVEUNUSED_SOURCE_DIRECTORY_COUNT
            || complete_hash != XYCE_REMOVEUNUSED_SOURCE_DIRECTORY_BLAKE3
            || content.len() != XYCE_REMOVEUNUSED_SOURCE_DIRECTORY_COUNT
            || content_hash != XYCE_REMOVEUNUSED_SOURCE_CONTENT_CENSUS_BLAKE3
            || physical.len() != XYCE_REMOVEUNUSED_PHYSICAL_COUNT
            || physical_hash != XYCE_REMOVEUNUSED_PHYSICAL_BLAKE3
        {
            return Err(format!(
                "REMOVEUNUSED family census changed: complete={}/{complete_hash}, content={}/{content_hash}, physical={}/{physical_hash}",
                complete.len(),
                content.len(),
                physical.len()
            ));
        }

        let mut candidates = BTreeSet::new();
        let mut candidate_content = BTreeSet::new();
        for candidate in XyceRemoveUnusedKind::ALL {
            let name = candidate
                .record()
                .rsplit_once('/')
                .expect("REMOVEUNUSED records have a family")
                .1;
            let path = family_dir.join(name);
            let bytes = fs::read(&path).map_err(|error| {
                format!(
                    "REMOVEUNUSED candidate {} is missing: {error}",
                    path.display()
                )
            })?;
            candidates.insert(candidate.record().to_string());
            candidate_content.insert(format!(
                "{}\t{}",
                candidate.record(),
                blake3::hash(&bytes).to_hex()
            ));
        }
        let candidates = candidates.into_iter().collect::<Vec<_>>();
        let candidate_content = candidate_content.into_iter().collect::<Vec<_>>();
        let candidate_hash = blake3::hash(candidates.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let candidate_content_hash = blake3::hash(candidate_content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if candidates.len() != XYCE_REMOVEUNUSED_CANDIDATE_COUNT
            || candidate_hash != XYCE_REMOVEUNUSED_CANDIDATE_BLAKE3
            || candidate_content_hash != XYCE_REMOVEUNUSED_CANDIDATE_CONTENT_BLAKE3
        {
            return Err(format!(
                "REMOVEUNUSED candidate census changed: names={}/{candidate_hash}, content={candidate_content_hash}",
                candidates.len()
            ));
        }

        let manifest = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(XYCE_REMOVEUNUSED_FAMILY_PREFIX))
            .cloned()
            .collect::<Vec<_>>();
        let manifest_hash = blake3::hash(manifest.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if manifest.len() != XYCE_REMOVEUNUSED_MANIFEST_COUNT
            || manifest_hash != XYCE_REMOVEUNUSED_MANIFEST_BLAKE3
            || !manifest.iter().any(|record| record == kind.record())
        {
            return Err(format!(
                "REMOVEUNUSED manifest census changed: expected {XYCE_REMOVEUNUSED_MANIFEST_COUNT}/{XYCE_REMOVEUNUSED_MANIFEST_BLAKE3}, got {}/{manifest_hash}",
                manifest.len()
            ));
        }
        if family_dir.join("options").exists() {
            return Err("REMOVEUNUSED family unexpectedly contains an options sidecar".to_string());
        }
        self.reject_removeunused_output_artifacts(&deck.path)
    }

    pub(super) fn validate_startup_oracle_provenance(
        &self,
        deck: &XyceDeck,
        kind: XyceStartupOracleKind,
    ) -> Result<(), String> {
        if deck.section != XyceDeckSection::Netlists {
            return Err(format!(
                "startup-diagnostic record '{}' is not in the Netlists corpus",
                kind.record()
            ));
        }
        if Self::normalize_manifest_key(&deck.relative_path) != kind.record() {
            return Err(format!(
                "startup-diagnostic record path mismatch: expected '{}', got '{}'",
                kind.record(),
                deck.relative_path
            ));
        }
        let canonical_deck = deck.path.canonicalize().map_err(|error| {
            format!(
                "failed to canonicalize startup-diagnostic record {}: {error}",
                deck.path.display()
            )
        })?;
        let canonical_expected = self
            .root
            .join(Path::new(&deck.relative_path))
            .canonicalize()
            .map_err(|error| {
                format!(
                    "startup-diagnostic record '{}' is missing from the vendored corpus: {error}",
                    kind.record()
                )
            })?;
        if canonical_deck != canonical_expected {
            return Err(format!(
                "startup-diagnostic record '{}' resolved outside its canonical corpus path",
                kind.record()
            ));
        }
        let metadata = fs::symlink_metadata(&deck.path).map_err(|error| {
            format!(
                "failed to inspect startup-diagnostic record {}: {error}",
                deck.path.display()
            )
        })?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err(format!(
                "startup-diagnostic record '{}' must be a regular non-symlink file",
                kind.record()
            ));
        }
        if !self.requires_upstream_wrapper(&deck.relative_path) {
            return Err(format!(
                "startup-diagnostic record '{}' lost its removed-wrapper manifest provenance",
                kind.record()
            ));
        }

        let (
            family_prefix,
            physical_count,
            physical_hash,
            manifest_count,
            manifest_hash,
            complete_count,
            complete_hash,
        ) = if kind.is_message_input() {
            (
                "netlists/message/input/",
                79,
                XYCE_MESSAGE_INPUT_PHYSICAL_CENSUS_BLAKE3,
                50,
                XYCE_MESSAGE_INPUT_MANIFEST_CENSUS_BLAKE3,
                87,
                XYCE_STARTUP_MESSAGE_INPUT_SOURCE_DIRECTORY_CENSUS_BLAKE3,
            )
        } else {
            (
                "netlists/certification_tests/bug_667_son/",
                5,
                XYCE_STARTUP_BUG667_PHYSICAL_CENSUS_BLAKE3,
                3,
                XYCE_STARTUP_BUG667_MANIFEST_CENSUS_BLAKE3,
                5,
                XYCE_STARTUP_BUG667_PHYSICAL_CENSUS_BLAKE3,
            )
        };
        let family_dir = deck.path.parent().ok_or_else(|| {
            "startup-diagnostic record has no source family directory".to_string()
        })?;
        Self::validate_startup_source_family_census(
            family_dir,
            physical_count,
            physical_hash,
            complete_count,
            complete_hash,
        )?;
        let manifest_records = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .cloned()
            .collect::<Vec<_>>();
        let actual_manifest_hash = blake3::hash(manifest_records.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if manifest_records.len() != manifest_count || actual_manifest_hash != manifest_hash {
            return Err(format!(
                "startup-diagnostic manifest family '{}' changed: expected {manifest_count} / {manifest_hash}, got {} / {actual_manifest_hash}",
                family_prefix.trim_end_matches('/'),
                manifest_records.len()
            ));
        }
        if !manifest_records
            .iter()
            .any(|record| record == kind.record())
        {
            return Err(format!(
                "startup-diagnostic record '{}' is absent from its pinned manifest family",
                kind.record()
            ));
        }

        if kind.is_message_input() {
            let options = family_dir.join("options");
            let metadata = fs::symlink_metadata(&options).map_err(|error| {
                format!("failed to inspect Message/Input options file: {error}")
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err("Message/Input options must be a regular non-symlink file".to_string());
            }
            let bytes = fs::read(&options)
                .map_err(|error| format!("failed to read Message/Input options: {error}"))?;
            let hash = blake3::hash(&bytes).to_hex().to_string();
            if bytes.len() != XYCE_STARTUP_OPTIONS_BYTES || hash != XYCE_STARTUP_OPTIONS_BLAKE3 {
                return Err(format!(
                    "Message/Input options changed: expected {} / {}, got {} / {hash}",
                    XYCE_STARTUP_OPTIONS_BYTES,
                    XYCE_STARTUP_OPTIONS_BLAKE3,
                    bytes.len()
                ));
            }
        }

        Self::reject_startup_source_sidecars(&deck.path)?;
        self.reject_startup_output_artifacts(&deck.path)?;
        Ok(())
    }

    pub(super) fn validate_bug75_complete_family_provenance(
        &self,
        family_dir: &Path,
    ) -> Result<(), String> {
        let expected_family = self.root.join("Netlists/Certification_Tests/BUG_75_SON");
        if family_dir.canonicalize().ok() != expected_family.canonicalize().ok() {
            return Err(format!(
                "BUG75 family resolved outside its canonical corpus directory: {}",
                family_dir.display()
            ));
        }

        let mut source_names = BTreeSet::new();
        for entry in fs::read_dir(family_dir).map_err(|error| {
            format!(
                "failed to inspect complete BUG75 source family {}: {error}",
                family_dir.display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to inspect complete BUG75 source entry in {}: {error}",
                    family_dir.display()
                )
            })?;
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| "BUG75 source filename is not UTF-8".to_string())?
                .to_ascii_lowercase();
            if !source_names.insert(name.clone()) {
                return Err(format!(
                    "BUG75 source family has case-colliding name {name:?}"
                ));
            }
        }
        let source_names = source_names.into_iter().collect::<Vec<_>>();
        let source_hash = blake3::hash(source_names.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if source_names.len() != 3 || source_hash != XYCE_BUG75_SOURCE_DIRECTORY_CENSUS_BLAKE3 {
            return Err(format!(
                "BUG75 complete source-directory census changed: expected 3 / {}, got {} / {}",
                XYCE_BUG75_SOURCE_DIRECTORY_CENSUS_BLAKE3,
                source_names.len(),
                source_hash
            ));
        }

        for (file_name, expected_bytes, expected_hash) in [
            ("README", XYCE_BUG75_README_BYTES, XYCE_BUG75_README_BLAKE3),
            (
                "options",
                XYCE_BUG75_OPTIONS_BYTES,
                XYCE_BUG75_OPTIONS_BLAKE3,
            ),
        ] {
            let path = family_dir.join(file_name);
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                format!(
                    "failed to inspect BUG75 retained source {}: {error}",
                    path.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "BUG75 retained source {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let bytes = fs::read(&path).map_err(|error| {
                format!(
                    "failed to read BUG75 retained source {}: {error}",
                    path.display()
                )
            })?;
            let hash = blake3::hash(&bytes).to_hex().to_string();
            if bytes.len() != expected_bytes || hash != expected_hash {
                return Err(format!(
                    "BUG75 retained source {file_name} changed: expected {expected_bytes} / {expected_hash}, got {} / {hash}",
                    bytes.len()
                ));
            }
        }

        let output_dir = self.root.join("OutputData/Certification_Tests/BUG_75_SON");
        let mut output_names = BTreeSet::new();
        if output_dir.exists() {
            let metadata = fs::symlink_metadata(&output_dir).map_err(|error| {
                format!(
                    "failed to inspect BUG75 OutputData path {}: {error}",
                    output_dir.display()
                )
            })?;
            if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "BUG75 OutputData path {} must be a regular non-symlink directory",
                    output_dir.display()
                ));
            }
            for entry in fs::read_dir(&output_dir).map_err(|error| {
                format!(
                    "failed to inspect BUG75 OutputData directory {}: {error}",
                    output_dir.display()
                )
            })? {
                let entry = entry.map_err(|error| {
                    format!(
                        "failed to inspect BUG75 OutputData entry in {}: {error}",
                        output_dir.display()
                    )
                })?;
                let name = entry
                    .file_name()
                    .to_str()
                    .ok_or_else(|| "BUG75 OutputData filename is not UTF-8".to_string())?
                    .to_ascii_lowercase();
                if !output_names.insert(name.clone()) {
                    return Err(format!("BUG75 OutputData has case-colliding name {name:?}"));
                }
            }
        }
        let output_names = output_names.into_iter().collect::<Vec<_>>();
        let output_hash = blake3::hash(output_names.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if !output_names.is_empty() || output_hash != XYCE_BUG75_EMPTY_OUTPUT_CENSUS_BLAKE3 {
            return Err(format!(
                "BUG75 OutputData census changed: expected 0 / {}, got {} / {}",
                XYCE_BUG75_EMPTY_OUTPUT_CENSUS_BLAKE3,
                output_names.len(),
                output_hash
            ));
        }

        Ok(())
    }

    pub(super) fn validate_output_symbol_complete_family_provenance(
        &self,
        kind: XyceExpectedFailureKind,
        family_dir: &Path,
    ) -> Result<(), String> {
        let (label, relative_dir, expected_count, expected_census, retained) = match kind {
            XyceExpectedFailureKind::Bug1148UndefinedPrintNode => (
                "BUG1148",
                "Netlists/Certification_Tests/BUG_1148",
                3,
                XYCE_BUG1148_SOURCE_DIRECTORY_CENSUS_BLAKE3,
                vec![
                    (
                        "README",
                        XYCE_BUG1148_README_BYTES,
                        XYCE_BUG1148_README_BLAKE3,
                    ),
                    (
                        "options",
                        XYCE_OUTPUT_SYMBOL_OPTIONS_BYTES,
                        XYCE_OUTPUT_SYMBOL_OPTIONS_BLAKE3,
                    ),
                ],
            ),
            XyceExpectedFailureKind::Bug40UndefinedPrintNode => (
                "BUG40",
                "Netlists/Certification_Tests/BUG_40",
                4,
                XYCE_BUG40_SOURCE_DIRECTORY_CENSUS_BLAKE3,
                vec![
                    ("README", XYCE_BUG40_README_BYTES, XYCE_BUG40_README_BLAKE3),
                    (
                        "options",
                        XYCE_OUTPUT_SYMBOL_OPTIONS_BYTES,
                        XYCE_OUTPUT_SYMBOL_OPTIONS_BLAKE3,
                    ),
                    ("bug_40.out", XYCE_BUG40_OUT_BYTES, XYCE_BUG40_OUT_BLAKE3),
                ],
            ),
            _ => return Ok(()),
        };
        let expected_family = self.root.join(relative_dir);
        if family_dir.canonicalize().ok() != expected_family.canonicalize().ok() {
            return Err(format!(
                "{label} family resolved outside its canonical corpus directory: {}",
                family_dir.display()
            ));
        }
        let mut names = BTreeSet::new();
        for entry in fs::read_dir(family_dir)
            .map_err(|error| format!("failed to inspect complete {label} family: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {label} entry: {error}"))?;
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{label} filename is not UTF-8"))?
                .to_ascii_lowercase();
            if !names.insert(name.clone()) {
                return Err(format!("{label} family has case-colliding name {name:?}"));
            }
        }
        let names = names.into_iter().collect::<Vec<_>>();
        let hash = blake3::hash(names.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if names.len() != expected_count || hash != expected_census {
            return Err(format!(
                "{label} complete source-directory census changed: expected {expected_count} / {expected_census}, got {} / {hash}",
                names.len()
            ));
        }
        for (file_name, expected_bytes, expected_hash) in retained {
            let path = family_dir.join(file_name);
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                format!(
                    "failed to inspect {label} retained source {}: {error}",
                    path.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "{label} retained source {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let bytes = fs::read(&path).map_err(|error| {
                format!(
                    "failed to read {label} retained source {}: {error}",
                    path.display()
                )
            })?;
            let actual_hash = blake3::hash(&bytes).to_hex().to_string();
            if bytes.len() != expected_bytes || actual_hash != expected_hash {
                return Err(format!(
                    "{label} retained source {file_name} changed: expected {expected_bytes} / {expected_hash}, got {} / {actual_hash}",
                    bytes.len()
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_bug702_complete_family_provenance(
        &self,
        family_dir: &Path,
    ) -> Result<(), String> {
        let expected_family = self.root.join("Netlists/Certification_Tests/BUG_702");
        if family_dir.canonicalize().ok() != expected_family.canonicalize().ok() {
            return Err(format!(
                "BUG702 family resolved outside its canonical corpus directory: {}",
                family_dir.display()
            ));
        }
        let source_files = [
            (
                "dup-external.cir",
                6_674,
                XYCE_BUG702_DUP_EXTERNAL_SOURCE_BLAKE3,
            ),
            (
                "dup-inlined.cir",
                6_615,
                XYCE_BUG702_DUP_INLINED_SOURCE_BLAKE3,
            ),
            (
                "empty-initcond.cir",
                6_482,
                XYCE_BUG702_EMPTY_INITCOND_SOURCE_BLAKE3,
            ),
            ("external.cir", 6_564, XYCE_BUG702_EXTERNAL_SOURCE_BLAKE3),
            (
                "initcond.dat",
                XYCE_BUG702_INITCOND_DATA_BYTES,
                XYCE_BUG702_INITCOND_DATA_BLAKE3,
            ),
            (
                "inlined-multiple.cir",
                1_474,
                XYCE_BUG702_INLINED_MULTIPLE_SOURCE_BLAKE3,
            ),
            (
                "inlined-single.cir",
                6_542,
                XYCE_BUG702_INLINED_SINGLE_SOURCE_BLAKE3,
            ),
            (
                "missing-initcond.cir",
                6_596,
                XYCE_BUG702_MISSING_INITCOND_SOURCE_BLAKE3,
            ),
            (
                "noinits.dat",
                XYCE_BUG702_NOINITS_DATA_BYTES,
                XYCE_BUG702_NOINITS_DATA_BLAKE3,
            ),
            (
                "options",
                XYCE_BUG702_OPTIONS_BYTES,
                XYCE_BUG702_OPTIONS_BLAKE3,
            ),
            (
                "precedence.cir",
                6_633,
                XYCE_BUG702_PRECEDENCE_SOURCE_BLAKE3,
            ),
            (
                "README",
                XYCE_BUG702_README_BYTES,
                XYCE_BUG702_README_BLAKE3,
            ),
        ];
        let mut names = BTreeSet::new();
        for entry in fs::read_dir(family_dir).map_err(|error| {
            format!(
                "failed to inspect complete BUG702 source family {}: {error}",
                family_dir.display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to inspect complete BUG702 source entry in {}: {error}",
                    family_dir.display()
                )
            })?;
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| "BUG702 source filename is not UTF-8".to_string())?
                .to_ascii_lowercase();
            if !names.insert(name.clone()) {
                return Err(format!(
                    "BUG702 source family has case-colliding name {name:?}"
                ));
            }
        }
        let names = names.into_iter().collect::<Vec<_>>();
        let names_hash = blake3::hash(names.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if names.len() != source_files.len()
            || names_hash != XYCE_BUG702_SOURCE_DIRECTORY_CENSUS_BLAKE3
        {
            return Err(format!(
                "BUG702 complete source-directory census changed: expected {} / {}, got {} / {}",
                source_files.len(),
                XYCE_BUG702_SOURCE_DIRECTORY_CENSUS_BLAKE3,
                names.len(),
                names_hash
            ));
        }
        for (file_name, expected_bytes, expected_hash) in source_files {
            let path = family_dir.join(file_name);
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                format!(
                    "failed to inspect BUG702 source {}: {error}",
                    path.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "BUG702 source {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let bytes = fs::read(&path).map_err(|error| {
                format!("failed to read BUG702 source {}: {error}", path.display())
            })?;
            let hash = blake3::hash(&bytes).to_hex().to_string();
            if bytes.len() != expected_bytes || hash != expected_hash {
                return Err(format!(
                    "BUG702 source {file_name} changed: expected {expected_bytes} / {expected_hash}, got {} / {hash}",
                    bytes.len()
                ));
            }
        }

        let output_dir = self.root.join("OutputData/Certification_Tests/BUG_702");
        let output_files = [
            (
                "inv1xIC.cir.prn",
                XYCE_BUG702_INV1XIC_REFERENCE_BYTES,
                XYCE_BUG702_INV1XIC_REFERENCE_BLAKE3,
            ),
            (
                "nlrcs10.cir.prn",
                XYCE_BUG702_NLRCS10_REFERENCE_BYTES,
                XYCE_BUG702_NLRCS10_REFERENCE_BLAKE3,
            ),
        ];
        let mut output_names = BTreeSet::new();
        for entry in fs::read_dir(&output_dir).map_err(|error| {
            format!(
                "failed to inspect BUG702 OutputData directory {}: {error}",
                output_dir.display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to inspect BUG702 OutputData entry in {}: {error}",
                    output_dir.display()
                )
            })?;
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| "BUG702 OutputData filename is not UTF-8".to_string())?
                .to_ascii_lowercase();
            if !output_names.insert(name.clone()) {
                return Err(format!(
                    "BUG702 OutputData has case-colliding name {name:?}"
                ));
            }
        }
        let output_names = output_names.into_iter().collect::<Vec<_>>();
        let output_hash = blake3::hash(output_names.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if output_names.len() != output_files.len()
            || output_hash != XYCE_BUG702_OUTPUT_DIRECTORY_CENSUS_BLAKE3
        {
            return Err(format!(
                "BUG702 OutputData census changed: expected {} / {}, got {} / {}",
                output_files.len(),
                XYCE_BUG702_OUTPUT_DIRECTORY_CENSUS_BLAKE3,
                output_names.len(),
                output_hash
            ));
        }
        for (file_name, expected_bytes, expected_hash) in output_files {
            let path = output_dir.join(file_name);
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                format!(
                    "failed to inspect BUG702 reference {}: {error}",
                    path.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "BUG702 reference {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let bytes = fs::read(&path).map_err(|error| {
                format!(
                    "failed to read BUG702 reference {}: {error}",
                    path.display()
                )
            })?;
            let hash = blake3::hash(&bytes).to_hex().to_string();
            if bytes.len() != expected_bytes || hash != expected_hash {
                return Err(format!(
                    "BUG702 reference {file_name} changed: expected {expected_bytes} / {expected_hash}, got {} / {hash}",
                    bytes.len()
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_step_noise_deriv_provenance(
        &self,
        deck: &XyceDeck,
    ) -> Result<Vec<u8>, String> {
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path)
                != XYCE_MEASURE_CONT_STEP_NOISE_DERIV_RECORD
            || !self.requires_upstream_wrapper(&deck.relative_path)
        {
            return Err(
                "MEASURE_CONT STEP NOISE derivative record lost exact wrapper ownership".into(),
            );
        }
        let canonical_deck = deck.path.canonicalize().map_err(|error| {
            format!("failed to canonicalize MEASURE_CONT STEP NOISE record: {error}")
        })?;
        let canonical_expected = self
            .root
            .join("Netlists/MEASURE_CONT/STEP/DerivTestNoise.cir")
            .canonicalize()
            .map_err(|error| {
                format!("canonical MEASURE_CONT STEP NOISE record is missing: {error}")
            })?;
        if canonical_deck != canonical_expected {
            return Err(
                "MEASURE_CONT STEP NOISE record resolved outside its canonical corpus path".into(),
            );
        }

        self.validate_measure_cont_manifest_family()?;
        self.validate_measure_cont_family_census(
            "Netlists/MEASURE_CONT",
            XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_COUNT,
            XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_NAMES_BLAKE3,
            XYCE_MEASURE_CONT_TRAN_SOURCE_FAMILY_CONTENT_BLAKE3,
        )?;
        self.validate_measure_cont_family_census(
            "OutputData/MEASURE_CONT",
            XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_COUNT,
            XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_NAMES_BLAKE3,
            XYCE_MEASURE_CONT_TRAN_OUTPUT_FAMILY_CONTENT_BLAKE3,
        )?;
        self.validate_measure_cont_step_case_sensitive_census()?;

        let source = Self::validate_measure_cont_regular_text_identity(
            &canonical_expected,
            (
                XYCE_MEASURE_CONT_STEP_NOISE_DERIV_SOURCE_BYTES,
                XYCE_MEASURE_CONT_STEP_NOISE_DERIV_SOURCE_BLAKE3,
            ),
            "MEASURE_CONT STEP NOISE derivative source",
        )?;
        for (relative, identity, label) in [
            (
                "Netlists/MEASURE_CONT/STEP/DerivTestNoiseGSfile",
                (
                    XYCE_MEASURE_CONT_STEP_NOISE_DERIV_GS_BYTES,
                    XYCE_MEASURE_CONT_STEP_NOISE_DERIV_GS_BLAKE3,
                ),
                "MEASURE_CONT STEP NOISE derivative GS",
            ),
            (
                "OutputData/MEASURE_CONT/STEP/DerivTestNoise.cir.ma0",
                (
                    XYCE_MEASURE_CONT_STEP_NOISE_DERIV_MA0_BYTES,
                    XYCE_MEASURE_CONT_STEP_NOISE_DERIV_MA0_BLAKE3,
                ),
                "MEASURE_CONT STEP NOISE derivative ma0",
            ),
            (
                "OutputData/MEASURE_CONT/STEP/DerivTestNoise.cir.ma1",
                (
                    XYCE_MEASURE_CONT_STEP_NOISE_DERIV_MA1_BYTES,
                    XYCE_MEASURE_CONT_STEP_NOISE_DERIV_MA1_BLAKE3,
                ),
                "MEASURE_CONT STEP NOISE derivative ma1",
            ),
        ] {
            Self::validate_measure_cont_regular_text_identity(
                &self.root.join(relative),
                identity,
                label,
            )?;
        }

        let manifest_path = self.root.join(HARNESS_MANIFEST_FILE);
        let manifest_bytes = fs::read(&manifest_path)
            .map_err(|error| format!("failed to read {}: {error}", manifest_path.display()))?;
        let canonical_manifest =
            Self::canonical_lf_text_identity("MEASURE_CONT STEP NOISE manifest", &manifest_bytes)?;
        let manifest = std::str::from_utf8(&canonical_manifest)
            .map_err(|error| format!("MEASURE_CONT STEP NOISE manifest is not UTF-8: {error}"))?;
        let owner = "Netlists/MEASURE_CONT/STEP/DerivTestNoise.cir\trequires_upstream_wrapper";
        if manifest.lines().filter(|line| *line == owner).count() != 1 {
            return Err(
                "MEASURE_CONT STEP NOISE manifest lost its exact case-sensitive owner row".into(),
            );
        }

        // Release-7.10.0's 4,973-byte wrapper (SHA-256
        // f9c11614...5d605465) invokes MeasureCommon.pm and file_compare.pl.
        // The latter owns the strict 1e-5/1e-3/1e-10 comparison contract;
        // it does not invoke xyce_verify or perform a remeasure pass.
        Ok(source)
    }

    pub(super) fn validate_measure_noise_step_deriv_provenance(
        &self,
        deck: &XyceDeck,
    ) -> Result<Vec<u8>, String> {
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path)
                != XYCE_MEASURE_NOISE_STEP_DERIV_RECORD
            || !self.requires_upstream_wrapper(&deck.relative_path)
        {
            return Err("MEASURE_NOISE STEP derivative record lost exact wrapper ownership".into());
        }
        let canonical_deck = deck.path.canonicalize().map_err(|error| {
            format!("failed to canonicalize MEASURE_NOISE STEP derivative record: {error}")
        })?;
        let canonical_expected = self
            .root
            .join("Netlists/MEASURE_NOISE/STEP/DerivTestNoise.cir")
            .canonicalize()
            .map_err(|error| {
                format!("canonical MEASURE_NOISE STEP derivative record is missing: {error}")
            })?;
        if canonical_deck != canonical_expected {
            return Err(
                "MEASURE_NOISE STEP derivative record resolved outside its canonical corpus path"
                    .into(),
            );
        }

        let source = Self::validate_measure_cont_regular_text_identity(
            &canonical_expected,
            (
                XYCE_MEASURE_NOISE_STEP_DERIV_SOURCE_BYTES,
                XYCE_MEASURE_NOISE_STEP_DERIV_SOURCE_BLAKE3,
            ),
            "MEASURE_NOISE STEP derivative source",
        )?;
        for (relative, identity, label) in [
            (
                "OutputData/MEASURE_NOISE/STEP/DerivTestNoise.cir.ma0",
                (
                    XYCE_MEASURE_NOISE_STEP_DERIV_MA0_BYTES,
                    XYCE_MEASURE_NOISE_STEP_DERIV_MA0_BLAKE3,
                ),
                "MEASURE_NOISE STEP derivative ma0",
            ),
            (
                "OutputData/MEASURE_NOISE/STEP/DerivTestNoise.cir.ma1",
                (
                    XYCE_MEASURE_NOISE_STEP_DERIV_MA1_BYTES,
                    XYCE_MEASURE_NOISE_STEP_DERIV_MA1_BLAKE3,
                ),
                "MEASURE_NOISE STEP derivative ma1",
            ),
        ] {
            Self::validate_measure_cont_regular_text_identity(
                &self.root.join(relative),
                identity,
                label,
            )?;
        }

        let manifest_path = self.root.join(HARNESS_MANIFEST_FILE);
        let manifest_bytes = fs::read(&manifest_path)
            .map_err(|error| format!("failed to read {}: {error}", manifest_path.display()))?;
        let canonical_manifest =
            Self::canonical_lf_text_identity("MEASURE_NOISE STEP manifest", &manifest_bytes)?;
        let manifest = std::str::from_utf8(&canonical_manifest)
            .map_err(|error| format!("MEASURE_NOISE STEP manifest is not UTF-8: {error}"))?;
        let owner = "Netlists/MEASURE_NOISE/STEP/DerivTestNoise.cir\trequires_upstream_wrapper";
        if manifest.lines().filter(|line| *line == owner).count() != 1 {
            return Err(
                "MEASURE_NOISE STEP derivative manifest lost its exact case-sensitive owner row"
                    .into(),
            );
        }
        Ok(source)
    }

    pub(super) fn stepped_ic_snapshot(netlist: &Netlist) -> Result<XyceSteppedIcSnapshot, String> {
        if !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err(
                "stepped-IC family must be a diagnostic-free flat RC circuit without models, data, node sets, measurements, globals, or external model data"
                    .to_string(),
            );
        }
        if netlist.elements.len() != 3 {
            return Err(format!(
                "stepped-IC family requires exactly three circuit elements, found {}",
                netlist.elements.len()
            ));
        }

        let mut capacitor = None;
        let mut resistor_nodes = None;
        let mut source_nodes = None;
        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            match &element.kind {
                ElementKind::Capacitor {
                    value,
                    value_expr,
                    initial_voltage,
                    model,
                    instance_params,
                    deferred_params,
                } if element.nodes.len() == 2
                    && value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && initial_voltage.is_none()
                    && model.is_none()
                    && instance_params.len() <= 1
                    && instance_params.iter().all(|(name, parameter_value)| {
                        name.eq_ignore_ascii_case("c")
                            && parameter_value.to_bits() == value.to_bits()
                    })
                    && deferred_params.is_empty() =>
                {
                    if capacitor
                        .replace((element.name.clone(), element.nodes.clone(), *value))
                        .is_some()
                    {
                        return Err(
                            "stepped-IC family contains more than one qualified capacitor"
                                .to_string(),
                        );
                    }
                }
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } if element.nodes.len() == 2
                    && value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty() =>
                {
                    if resistor_nodes.replace(element.nodes.clone()).is_some() {
                        return Err(
                            "stepped-IC family contains more than one qualified resistor"
                                .to_string(),
                        );
                    }
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if element.nodes.len() == 2
                        && value.is_finite()
                        && value.to_bits() == 0.0f64.to_bits() =>
                {
                    if source_nodes.replace(element.nodes.clone()).is_some() {
                        return Err(
                            "stepped-IC family contains more than one qualified zero-volt source"
                                .to_string(),
                        );
                    }
                }
                _ => {
                    return Err(format!(
                        "stepped-IC family contains unqualified element '{}'",
                        element.name
                    ));
                }
            }
            let name = element.name.trim().to_ascii_lowercase();
            if name.is_empty()
                || elements
                    .insert(
                        name,
                        Self::scoped_model_element_fingerprint(element, &netlist.params)?,
                    )
                    .is_some()
            {
                return Err(
                    "stepped-IC family contains an empty or duplicate element name".to_string(),
                );
            }
        }

        let (capacitor_name, capacitor_nodes, capacitance) =
            capacitor.ok_or_else(|| "stepped-IC family has no qualified capacitor".to_string())?;
        let resistor_nodes = resistor_nodes
            .ok_or_else(|| "stepped-IC family has no qualified resistor".to_string())?;
        let source_nodes = source_nodes
            .ok_or_else(|| "stepped-IC family has no qualified zero-volt source".to_string())?;
        let capacitor_signal = capacitor_nodes
            .iter()
            .find(|node| !Self::node_name_is_ground(node))
            .ok_or_else(|| "stepped-IC capacitor has no non-ground signal node".to_string())?;
        if capacitor_nodes
            .iter()
            .filter(|node| Self::node_name_is_ground(node))
            .count()
            != 1
        {
            return Err("stepped-IC capacitor must connect one signal node to ground".to_string());
        }
        let source_signal = source_nodes
            .iter()
            .find(|node| !Self::node_name_is_ground(node))
            .ok_or_else(|| "stepped-IC zero-volt source has no non-ground node".to_string())?;
        if source_nodes
            .iter()
            .filter(|node| Self::node_name_is_ground(node))
            .count()
            != 1
            || !resistor_nodes
                .iter()
                .any(|node| node.eq_ignore_ascii_case(capacitor_signal))
            || !resistor_nodes
                .iter()
                .any(|node| node.eq_ignore_ascii_case(source_signal))
        {
            return Err(
                "stepped-IC topology must be grounded C -> R -> grounded zero-volt source"
                    .to_string(),
            );
        }

        let [initial_condition] = netlist.initial_conditions.as_slice() else {
            return Err(format!(
                "stepped-IC family requires exactly one .IC voltage, found {}",
                netlist.initial_conditions.len()
            ));
        };
        if initial_condition.voltage_expr.is_some()
            || !initial_condition.voltage.is_finite()
            || !initial_condition
                .node
                .eq_ignore_ascii_case(capacitor_signal)
        {
            return Err(
                "stepped-IC .IC must be one finite direct voltage on the capacitor signal node"
                    .to_string(),
            );
        }
        let initial_conditions = vec![(
            initial_condition.node.trim().to_ascii_lowercase(),
            initial_condition.voltage.to_bits(),
        )];

        Ok(XyceSteppedIcSnapshot {
            elements,
            initial_conditions,
            capacitor_name,
            capacitor_value_bits: capacitance.to_bits(),
        })
    }

    pub(super) fn strict_ac_family_snapshot(
        kind: XyceBaselineFamilyKind,
        netlist: &Netlist,
    ) -> Result<XyceStrictAcFamilySnapshot, String> {
        match kind {
            XyceBaselineFamilyKind::AcAnalysisExpression => {
                Self::ac_analysis_expression_snapshot(netlist)
                    .map(XyceStrictAcFamilySnapshot::AcAnalysisExpression)
            }
            other => Err(format!(
                "family kind {} has no qualified AC semantic snapshot",
                other.name()
            )),
        }
    }

    pub(super) fn strict_dc_family_snapshot(
        kind: XyceBaselineFamilyKind,
        netlist: &Netlist,
        plan: &XyceStaticDcPlan,
    ) -> Result<XyceStrictDcFamilySnapshot, String> {
        match kind {
            XyceBaselineFamilyKind::BjtExternalNode => {
                Self::bjt_external_node_family_snapshot(netlist, &plan.print)
                    .map(XyceStrictDcFamilySnapshot::BjtExternalNode)
            }
            XyceBaselineFamilyKind::DcAnalysisExpression => {
                Self::dc_analysis_expression_snapshot(netlist)
                    .map(XyceStrictDcFamilySnapshot::DcAnalysisExpression)
            }
            XyceBaselineFamilyKind::DelimitedExpression => {
                Self::delimited_expression_family_snapshot(plan, netlist)
                    .map(XyceStrictDcFamilySnapshot::DelimitedExpression)
            }
            XyceBaselineFamilyKind::PassiveResPrimaryValue => {
                Self::passive_res_primary_snapshot(netlist, &plan.print, &plan.dc.source)
                    .map(XyceStrictDcFamilySnapshot::PassivePrimaryValue)
            }
            XyceBaselineFamilyKind::SubcktParameterPrecedence => {
                Self::subckt_parameter_precedence_snapshot(netlist, &plan.print, &plan.dc.source)
                    .map(XyceStrictDcFamilySnapshot::SubcktParameterPrecedence)
            }
            XyceBaselineFamilyKind::SubcktParameterResolution => {
                Self::subckt_parameter_resolution_snapshot(netlist, &plan.print, &plan.dc.source)
                    .map(XyceStrictDcFamilySnapshot::SubcktParameterResolution)
            }
            XyceBaselineFamilyKind::NestedIncludeIdentity => {
                Self::nested_include_identity_family_snapshot(netlist, plan)
                    .map(XyceStrictDcFamilySnapshot::NestedIncludeIdentity)
            }
            other => Err(format!(
                "family kind {} has no qualified exact-DC semantic snapshot",
                other.name()
            )),
        }
    }

    pub(super) fn subckt_parameter_precedence_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
        sweep_source: &str,
    ) -> Result<XyceSubcktParameterPrecedenceSnapshot, String> {
        if print.probes.len() != 2
            || !netlist.models.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
        {
            return Err(
                "subcircuit-parameter precedence snapshot requires two probes and no model, diagnostic, data, IC, or NODESET state"
                    .to_string(),
            );
        }
        let flattened = rspice_core::netlist::flatten_netlist_with_models(netlist)
            .map_err(|error| format!("could not flatten sibling-reference deck: {error}"))?;
        if flattened.elements.len() != 3 {
            return Err(format!(
                "subcircuit-parameter precedence snapshot requires a three-element divider, found {} flattened elements",
                flattened.elements.len()
            ));
        }
        let voltage_sources = flattened
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::VoltageSource(_)))
            .collect::<Vec<_>>();
        let resistor_count = flattened
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Resistor { .. }))
            .count();
        if voltage_sources.len() != 1
            || resistor_count != 2
            || !voltage_sources[0].name.eq_ignore_ascii_case(sweep_source)
        {
            return Err(
                "subcircuit-parameter precedence snapshot requires two resistors and the swept independent voltage source"
                    .to_string(),
            );
        }

        let mut elements = flattened
            .elements
            .iter()
            .map(|element| {
                let mut fingerprint =
                    Self::scoped_model_element_fingerprint(element, &netlist.params)?;
                for node in &mut fingerprint.nodes {
                    *node = node.replace(':', ".");
                }
                Ok(fingerprint)
            })
            .collect::<Result<Vec<_>, String>>()?;
        elements.sort();
        Ok(XyceSubcktParameterPrecedenceSnapshot { elements })
    }

    pub(super) fn subckt_parameter_resolution_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
        sweep_source: &str,
    ) -> Result<XyceSubcktParameterResolutionSnapshot, String> {
        let (representation, _, snapshot) =
            Self::subckt_parameter_resolution_qualification(netlist, print, sweep_source)?;
        if representation == XyceSubcktParameterResolutionRepresentation::UndefinedBinding {
            return Err(
                "undefined-binding member cannot serve as a numeric family member".to_string(),
            );
        }
        snapshot.ok_or_else(|| {
            "numeric subcircuit-parameter member produced no semantic snapshot".to_string()
        })
    }

    pub(super) fn passive_cap_primary_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<XycePassivePrimaryValueSnapshot, String> {
        let kind = XycePassivePrimaryKind::CapacitorTran;
        Self::validate_passive_primary_common_netlist(netlist, kind)?;
        let model = &netlist.models[0];
        if !model.model_type.eq_ignore_ascii_case("C")
            || !model.params.is_empty()
            || !Self::passive_model_has_no_deferred_state(model)
        {
            return Err(format!(
                "capacitor primary-value model '{}' must be a parameter-free C model",
                model.name
            ));
        }

        let mut capacitor = None;
        let mut resistor = None;
        let mut pulse = None;
        let mut monitor = None;
        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            let key = Self::normalize_device_instance_name(&element.name);
            let nodes = element
                .nodes
                .iter()
                .map(|node| Self::canonical_passive_primary_node_name(node))
                .collect::<Vec<_>>();
            let fingerprint = match &element.kind {
                ElementKind::Capacitor {
                    value,
                    value_expr,
                    initial_voltage,
                    model: element_model,
                    instance_params,
                    deferred_params,
                } => {
                    if capacitor.is_some()
                        || element.nodes.len() != 2
                        || !value.is_finite()
                        || *value <= 0.0
                        || value_expr.is_some()
                        || initial_voltage.is_some()
                        || !instance_params.is_empty()
                        || !deferred_params.is_empty()
                    {
                        return Err(format!(
                            "capacitor '{}' is outside the explicit finite modeled primary-value envelope",
                            element.name
                        ));
                    }
                    let element_model = element_model.as_ref().ok_or_else(|| {
                        format!(
                            "capacitor '{}' must reference the unique model",
                            element.name
                        )
                    })?;
                    if !element_model.eq_ignore_ascii_case(&model.name) {
                        return Err(format!(
                            "capacitor '{}' does not reference model '{}'",
                            element.name, model.name
                        ));
                    }
                    capacitor = Some((
                        element.name.clone(),
                        nodes.clone(),
                        element_model.clone(),
                        *value,
                    ));
                    XyceRelationalElementFingerprint {
                        kind: "C:MODEL".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: vec![element_model.to_ascii_lowercase()],
                    }
                }
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    if resistor.is_some()
                        || element.nodes.len() != 2
                        || !value.is_finite()
                        || *value <= 0.0
                        || value_expr.is_some()
                        || model.is_some()
                        || !instance_params.is_empty()
                        || !deferred_params.is_empty()
                    {
                        return Err(format!(
                            "resistor '{}' is outside the direct finite two-terminal envelope",
                            element.name
                        ));
                    }
                    resistor = Some((element.name.clone(), nodes.clone(), *value));
                    XyceRelationalElementFingerprint {
                        kind: "R".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pulse {
                    v1,
                    v2,
                    delay,
                    rise,
                    fall,
                    width,
                    period,
                    phase,
                    width_defaults_to_zero,
                }) => {
                    if pulse.is_some()
                        || element.nodes.len() != 2
                        || !v1.is_finite()
                        || !v2.is_finite()
                        || v1 == v2
                        || !delay.is_finite()
                        || *delay < 0.0
                        || !rise.is_finite()
                        || *rise <= 0.0
                        || !fall.is_finite()
                        || *fall <= 0.0
                        || !width.is_finite()
                        || *width <= 0.0
                        || !period.is_nan()
                        || phase.to_bits() != 0.0f64.to_bits()
                        || *width_defaults_to_zero
                    {
                        return Err(format!(
                            "pulse source '{}' is outside the direct six-argument nontrivial waveform envelope",
                            element.name
                        ));
                    }
                    let numeric_bits = vec![
                        v1.to_bits(),
                        v2.to_bits(),
                        delay.to_bits(),
                        rise.to_bits(),
                        fall.to_bits(),
                        width.to_bits(),
                        period.to_bits(),
                        phase.to_bits(),
                        u64::from(*width_defaults_to_zero),
                    ];
                    pulse = Some((element.name.clone(), nodes.clone()));
                    XyceRelationalElementFingerprint {
                        kind: "V:PULSE6".to_string(),
                        nodes,
                        numeric_bits,
                        text: Vec::new(),
                    }
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) => {
                    if monitor.is_some()
                        || element.nodes.len() != 2
                        || value.to_bits() != 0.0f64.to_bits()
                    {
                        return Err(format!(
                            "monitor source '{}' must be the unique direct positive-zero DC source",
                            element.name
                        ));
                    }
                    monitor = Some((element.name.clone(), nodes.clone()));
                    XyceRelationalElementFingerprint {
                        kind: "V:DC".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                _ => {
                    return Err(format!(
                        "element '{}' is outside the qualified capacitor primary-value topology",
                        element.name
                    ));
                }
            };
            if elements.insert(key.clone(), fingerprint).is_some() {
                return Err(format!(
                    "capacitor primary-value parity contains duplicate element name '{key}'"
                ));
            }
        }
        if elements.len() != 4 {
            return Err(format!(
                "capacitor primary-value parity requires exactly four elements, found {}",
                elements.len()
            ));
        }
        let (capacitor_name, capacitor_nodes, capacitor_model, capacitor_value) =
            capacitor.ok_or_else(|| "no qualified modeled capacitor found".to_string())?;
        let (_, resistor_nodes, _) =
            resistor.ok_or_else(|| "no qualified ordinary resistor found".to_string())?;
        let (_, pulse_nodes) =
            pulse.ok_or_else(|| "no qualified six-argument pulse source found".to_string())?;
        let (monitor_name, monitor_nodes) =
            monitor.ok_or_else(|| "no qualified zero-volt monitor found".to_string())?;
        let [drive, ground] = pulse_nodes.as_slice() else {
            return Err("pulse source must have two terminals".to_string());
        };
        let [resistor_drive, monitor_input] = resistor_nodes.as_slice() else {
            return Err("ordinary resistor must have two terminals".to_string());
        };
        let [monitor_input_again, capacitor_node] = monitor_nodes.as_slice() else {
            return Err("monitor source must have two terminals".to_string());
        };
        let [capacitor_node_again, capacitor_ground] = capacitor_nodes.as_slice() else {
            return Err("capacitor must have two terminals".to_string());
        };
        if ground != "0"
            || capacitor_ground != "0"
            || drive == "0"
            || monitor_input == "0"
            || capacitor_node == "0"
            || drive == monitor_input
            || drive == capacitor_node
            || monitor_input == capacitor_node
            || resistor_drive != drive
            || monitor_input_again != monitor_input
            || capacitor_node_again != capacitor_node
        {
            return Err("capacitor primary-value topology must be Pulse -> R -> +0 V monitor -> modeled C with three distinct non-ground nodes".to_string());
        }
        let [voltage_text, current_text] = print.probes.as_slice() else {
            return Err(
                "capacitor primary-value parity requires exactly two ordered probes".to_string(),
            );
        };
        let voltage = Self::parse_voltage_probe(voltage_text)
            .ok_or_else(|| format!("'{voltage_text}' is not an atomic voltage probe"))?;
        let current = Self::parse_current_probe(current_text)
            .ok_or_else(|| format!("'{current_text}' is not an atomic current probe"))?;
        if voltage.accessor != XyceVoltageAccessor::Value
            || voltage.node_neg.is_some()
            || Self::canonical_passive_primary_node_name(&voltage.node_pos) != *capacitor_node
            || !Self::device_instance_names_match(&current, &monitor_name)
        {
            return Err("capacitor primary-value probes must be ordered V(capacitor-node), I(monitor-source)".to_string());
        }

        let source = netlist.source_text.as_deref().ok_or_else(|| {
            "capacitor primary-value parity requires original source text".to_string()
        })?;
        let (representation, literal_bits, active_source_fingerprint) =
            Self::passive_primary_source_contract(source, &capacitor_name, &capacitor_model, kind)?;
        if literal_bits != capacitor_value.to_bits() {
            return Err("capacitor source literal and parsed primary value differ".to_string());
        }
        let effective = Self::effective_capacitor_value(netlist, &capacitor_name)
            .filter(|value| value.is_finite() && *value > 0.0)
            .ok_or_else(|| "capacitor effective primary value did not resolve".to_string())?;
        if effective.to_bits() != capacitor_value.to_bits() {
            return Err("capacitor parsed and effective primary values differ".to_string());
        }
        Ok(XycePassivePrimaryValueSnapshot {
            title: netlist.title.trim().to_string(),
            device_kind: kind,
            representation,
            active_source_fingerprint,
            model_name: model.name.to_ascii_lowercase(),
            model_type: model.model_type.to_ascii_lowercase(),
            model_numeric_bits: Vec::new(),
            elements,
            effective_primary_bits: effective.to_bits(),
        })
    }

    pub(super) fn passive_res_primary_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
        sweep_source: &str,
    ) -> Result<XycePassivePrimaryValueSnapshot, String> {
        let kind = XycePassivePrimaryKind::ResistorDc;
        Self::validate_passive_primary_common_netlist(netlist, kind)?;
        let model = &netlist.models[0];
        if !model.model_type.eq_ignore_ascii_case("R")
            || model.params.len() != 1
            || !model.params[0].0.eq_ignore_ascii_case("RSH")
            || !model.params[0].1.is_finite()
            || model.params[0].1 <= 0.0
            || !Self::passive_model_has_no_deferred_state(model)
        {
            return Err(format!(
                "resistor primary-value model '{}' must be an R model with exactly one finite positive RSH",
                model.name
            ));
        }

        let mut resistor = None;
        let mut swept_source = None;
        let mut monitor = None;
        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            let key = Self::normalize_device_instance_name(&element.name);
            let nodes = element
                .nodes
                .iter()
                .map(|node| Self::canonical_passive_primary_node_name(node))
                .collect::<Vec<_>>();
            let fingerprint = match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model: element_model,
                    instance_params,
                    deferred_params,
                } => {
                    if resistor.is_some()
                        || element.nodes.len() != 2
                        || !value.is_finite()
                        || *value <= 0.0
                        || value_expr.is_some()
                        || !deferred_params.is_empty()
                    {
                        return Err(format!(
                            "resistor '{}' is outside the explicit finite modeled primary-value envelope",
                            element.name
                        ));
                    }
                    let element_model = element_model.as_ref().ok_or_else(|| {
                        format!(
                            "resistor '{}' must reference the unique model",
                            element.name
                        )
                    })?;
                    if !element_model.eq_ignore_ascii_case(&model.name) {
                        return Err(format!(
                            "resistor '{}' does not reference model '{}'",
                            element.name, model.name
                        ));
                    }
                    resistor = Some((
                        element.name.clone(),
                        nodes.clone(),
                        element_model.clone(),
                        *value,
                        instance_params.clone(),
                    ));
                    XyceRelationalElementFingerprint {
                        kind: "R:MODEL".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: vec![element_model.to_ascii_lowercase()],
                    }
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) => {
                    if element.nodes.len() != 2 || !value.is_finite() {
                        return Err(format!(
                            "voltage source '{}' must be a finite direct DC source",
                            element.name
                        ));
                    }
                    if Self::device_instance_names_match(&element.name, sweep_source) {
                        if swept_source.is_some() || *value == 0.0 {
                            return Err("resistor primary-value sweep requires one nonzero source"
                                .to_string());
                        }
                        swept_source = Some((element.name.clone(), nodes.clone(), *value));
                    } else {
                        if monitor.is_some() || value.to_bits() != 0.0f64.to_bits() {
                            return Err("resistor primary-value monitor must be the unique positive-zero source".to_string());
                        }
                        monitor = Some((element.name.clone(), nodes.clone()));
                    }
                    XyceRelationalElementFingerprint {
                        kind: "V:DC".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                _ => {
                    return Err(format!(
                        "element '{}' is outside the qualified resistor primary-value topology",
                        element.name
                    ));
                }
            };
            if elements.insert(key.clone(), fingerprint).is_some() {
                return Err(format!(
                    "resistor primary-value parity contains duplicate element name '{key}'"
                ));
            }
        }
        if elements.len() != 3 {
            return Err(format!(
                "resistor primary-value parity requires exactly three elements, found {}",
                elements.len()
            ));
        }
        let (resistor_name, resistor_nodes, resistor_model, resistor_value, instance_params) =
            resistor.ok_or_else(|| "no qualified modeled resistor found".to_string())?;
        let (_, sweep_nodes, _) =
            swept_source.ok_or_else(|| "qualified swept voltage source not found".to_string())?;
        let (monitor_name, monitor_nodes) =
            monitor.ok_or_else(|| "qualified zero-volt monitor not found".to_string())?;
        let [drive, ground] = sweep_nodes.as_slice() else {
            return Err("swept source must have two terminals".to_string());
        };
        let [monitor_drive, resistor_node] = monitor_nodes.as_slice() else {
            return Err("monitor source must have two terminals".to_string());
        };
        let [resistor_node_again, resistor_ground] = resistor_nodes.as_slice() else {
            return Err("resistor must have two terminals".to_string());
        };
        if ground != "0"
            || resistor_ground != "0"
            || drive == "0"
            || resistor_node == "0"
            || drive == resistor_node
            || monitor_drive != drive
            || resistor_node_again != resistor_node
        {
            return Err("resistor primary-value topology must be swept source -> +0 V monitor -> modeled R with two distinct non-ground nodes".to_string());
        }
        let [voltage_text, current_text] = print.probes.as_slice() else {
            return Err(
                "resistor primary-value parity requires exactly two ordered probes".to_string(),
            );
        };
        let voltage = Self::parse_voltage_probe(voltage_text)
            .ok_or_else(|| format!("'{voltage_text}' is not an atomic voltage probe"))?;
        let current = Self::parse_current_probe(current_text)
            .ok_or_else(|| format!("'{current_text}' is not an atomic current probe"))?;
        if voltage.accessor != XyceVoltageAccessor::Value
            || voltage.node_neg.is_some()
            || Self::canonical_passive_primary_node_name(&voltage.node_pos) != *drive
            || !Self::device_instance_names_match(&current, &monitor_name)
        {
            return Err(
                "resistor primary-value probes must be ordered V(drive), I(monitor-source)"
                    .to_string(),
            );
        }

        let source = netlist.source_text.as_deref().ok_or_else(|| {
            "resistor primary-value parity requires original source text".to_string()
        })?;
        let (representation, literal_bits, active_source_fingerprint) =
            Self::passive_primary_source_contract(source, &resistor_name, &resistor_model, kind)?;
        let instance_form_matches = match representation {
            XycePassivePrimaryRepresentation::Named => {
                matches!(
                    instance_params.as_slice(),
                    [(name, value)]
                        if name.eq_ignore_ascii_case("R")
                            && value.to_bits() == resistor_value.to_bits()
                )
            }
            XycePassivePrimaryRepresentation::Positional => instance_params.is_empty(),
        };
        if !instance_form_matches || literal_bits != resistor_value.to_bits() {
            return Err("resistor source representation, parsed primary value, or normalized R instance parameter differs".to_string());
        }
        let effective = Self::effective_resistor_value(netlist, &resistor_name)?
            .filter(|value| value.is_finite() && *value > 0.0)
            .ok_or_else(|| "resistor effective primary value did not resolve".to_string())?;
        if effective.to_bits() != resistor_value.to_bits() {
            return Err("resistor parsed and effective primary values differ".to_string());
        }
        Ok(XycePassivePrimaryValueSnapshot {
            title: netlist.title.trim().to_string(),
            device_kind: kind,
            representation,
            active_source_fingerprint,
            model_name: model.name.to_ascii_lowercase(),
            model_type: model.model_type.to_ascii_lowercase(),
            model_numeric_bits: vec![(
                model.params[0].0.to_ascii_lowercase(),
                model.params[0].1.to_bits(),
            )],
            elements,
            effective_primary_bits: effective.to_bits(),
        })
    }

    pub(super) fn strict_transient_family_snapshot(
        contract: &XyceBaselineFamilyContract,
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<XyceStrictTransientFamilySnapshot, String> {
        match contract.kind {
            XyceBaselineFamilyKind::AgeCap => Self::age_cap_family_snapshot(netlist, print)
                .map(XyceStrictTransientFamilySnapshot::AgeCap),
            XyceBaselineFamilyKind::DiodeModelAlias => {
                Self::diode_model_alias_family_snapshot(netlist, print)
                    .map(XyceStrictTransientFamilySnapshot::DiodeModelAlias)
            }
            XyceBaselineFamilyKind::SwitchStateCase => {
                Self::switch_state_case_family_snapshot(netlist, print)
                    .map(XyceStrictTransientFamilySnapshot::SwitchStateCase)
            }
            XyceBaselineFamilyKind::ScopedModel => {
                Self::scoped_model_family_snapshot(contract, netlist)?
                    .map(XyceStrictTransientFamilySnapshot::ScopedModel)
                    .ok_or_else(|| {
                        "scoped-model strict family produced no semantic snapshot".to_string()
                    })
            }
            XyceBaselineFamilyKind::SinExpression => {
                Self::sin_expression_family_snapshot(netlist, print)
                    .map(XyceStrictTransientFamilySnapshot::SinExpression)
            }
            XyceBaselineFamilyKind::ParamExpression => {
                Self::param_expression_family_snapshot(netlist, print)
                    .map(XyceStrictTransientFamilySnapshot::ParamExpression)
            }
            XyceBaselineFamilyKind::PassiveCapPrimaryValue => {
                Self::passive_cap_primary_snapshot(netlist, print)
                    .map(XyceStrictTransientFamilySnapshot::PassivePrimaryValue)
            }
            XyceBaselineFamilyKind::PassiveTemperatureOverride => {
                Self::passive_temperature_override_snapshot(netlist, print)
                    .map(XyceStrictTransientFamilySnapshot::PassiveTemperatureOverride)
            }
            XyceBaselineFamilyKind::TransientAnalysisExpression => {
                Self::transient_analysis_expression_snapshot(netlist, print)
                    .map(XyceStrictTransientFamilySnapshot::TransientAnalysisExpression)
            }
            other => Err(format!(
                "strict transient family kind {} has no semantic snapshot contract",
                other.name()
            )),
        }
    }

    pub(super) fn passive_temperature_override_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<XycePassiveTemperatureOverrideSnapshot, String> {
        const LABEL: &str = "passive temperature-coefficient override parity";
        let source = netlist
            .source_text
            .as_deref()
            .ok_or_else(|| format!("{LABEL} requires original source text"))?;
        if print.probes.is_empty()
            || !netlist.diagnostics.is_empty()
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }])
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} requires one diagnostic-free transient analysis without hierarchy, parameters, auxiliary analysis state, or external models"
            ));
        }
        let [model] = netlist.models.as_slice() else {
            return Err(format!(
                "{LABEL} requires exactly one passive model, found {}",
                netlist.models.len()
            ));
        };
        if !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!(
                "{LABEL} model '{}' must use only direct scalar numeric parameters",
                model.name
            ));
        }
        let model_tc = Self::passive_temperature_coefficient_pair(&model.params, "model")?;

        let mut passive = None;
        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            let name = Self::normalize_device_instance_name(&element.name);
            if name.is_empty() || elements.contains_key(&name) {
                return Err(format!(
                    "{LABEL} contains an empty or duplicate element name '{}'",
                    element.name
                ));
            }
            let nodes = element
                .nodes
                .iter()
                .map(|node| node.trim().to_ascii_lowercase())
                .collect::<Vec<_>>();
            if nodes.iter().any(String::is_empty) {
                return Err(format!(
                    "{LABEL} element '{name}' contains an empty node name"
                ));
            }
            let fingerprint = match &element.kind {
                ElementKind::Capacitor {
                    value,
                    value_expr,
                    initial_voltage,
                    model: element_model,
                    instance_params,
                    deferred_params,
                } => {
                    let details = XycePassiveTemperatureDeviceKind::Capacitor;
                    if passive.is_some()
                        || nodes.len() != 2
                        || !value.is_finite()
                        || *value <= 0.0
                        || value_expr.is_some()
                        || initial_voltage.is_some_and(|value| !value.is_finite())
                        || !deferred_params.is_empty()
                        || !matches!(
                            model.model_type.to_ascii_uppercase().as_str(),
                            "C" | "CAP" | "CAPACITOR"
                        )
                    {
                        return Err(format!(
                            "{LABEL} requires one direct finite positive two-terminal capacitor with no deferred state"
                        ));
                    }
                    let effective = Self::effective_capacitor_value(netlist, &element.name)
                        .ok_or_else(|| {
                            format!("{LABEL} could not resolve capacitor '{}'", element.name)
                        })?;
                    let (representation, winning_tc, temperature) =
                        Self::passive_temperature_instance_state(instance_params, model_tc)?;
                    Self::validate_passive_temperature_model_binding(
                        element_model.as_deref(),
                        model,
                    )?;
                    if !effective.is_finite() || effective <= 0.0 {
                        return Err(format!(
                            "{LABEL} capacitor '{}' resolved to invalid capacitance {effective}",
                            element.name
                        ));
                    }
                    let mut numeric_bits = vec![value.to_bits(), temperature.to_bits()];
                    let initial_marker = if let Some(initial) = initial_voltage {
                        numeric_bits.push(initial.to_bits());
                        "IC"
                    } else {
                        "NO_IC"
                    };
                    numeric_bits.extend(winning_tc.map(Value::to_bits));
                    numeric_bits.push(effective.to_bits());
                    passive = Some((details, representation, winning_tc, effective));
                    XyceRelationalElementFingerprint {
                        kind: "C:TEMP_OVERRIDE".to_string(),
                        nodes,
                        numeric_bits,
                        text: vec![model.name.to_ascii_lowercase(), initial_marker.to_string()],
                    }
                }
                ElementKind::Inductor {
                    value,
                    value_expr,
                    initial_current,
                    model: element_model,
                    instance_params,
                    deferred_params,
                } => {
                    let details = XycePassiveTemperatureDeviceKind::Inductor;
                    if passive.is_some()
                        || nodes.len() != 2
                        || !value.is_finite()
                        || *value <= 0.0
                        || value_expr.is_some()
                        || initial_current.is_some_and(|value| !value.is_finite())
                        || !deferred_params.is_empty()
                        || !matches!(
                            model.model_type.to_ascii_uppercase().as_str(),
                            "L" | "IND" | "INDUCTOR"
                        )
                    {
                        return Err(format!(
                            "{LABEL} requires one direct finite positive two-terminal inductor with no deferred state"
                        ));
                    }
                    let effective = Self::effective_inductor_value(netlist, &element.name)
                        .ok_or_else(|| {
                            format!("{LABEL} could not resolve inductor '{}'", element.name)
                        })?;
                    let (representation, winning_tc, temperature) =
                        Self::passive_temperature_instance_state(instance_params, model_tc)?;
                    Self::validate_passive_temperature_model_binding(
                        element_model.as_deref(),
                        model,
                    )?;
                    if !effective.is_finite() || effective <= 0.0 {
                        return Err(format!(
                            "{LABEL} inductor '{}' resolved to invalid inductance {effective}",
                            element.name
                        ));
                    }
                    let mut numeric_bits = vec![value.to_bits(), temperature.to_bits()];
                    let initial_marker = if let Some(initial) = initial_current {
                        numeric_bits.push(initial.to_bits());
                        "IC"
                    } else {
                        "NO_IC"
                    };
                    numeric_bits.extend(winning_tc.map(Value::to_bits));
                    numeric_bits.push(effective.to_bits());
                    passive = Some((details, representation, winning_tc, effective));
                    XyceRelationalElementFingerprint {
                        kind: "L:TEMP_OVERRIDE".to_string(),
                        nodes,
                        numeric_bits,
                        text: vec![model.name.to_ascii_lowercase(), initial_marker.to_string()],
                    }
                }
                _ => Self::passive_temperature_nonpassive_fingerprint(element, nodes)?,
            };
            elements.insert(name, fingerprint);
        }
        let Some((device_kind, representation, winning_tc, effective_primary)) = passive else {
            return Err(format!("{LABEL} contains no qualified passive device"));
        };
        if representation == XycePassiveTemperatureRepresentation::InstanceCoefficients
            && model_tc == winning_tc
        {
            return Err(format!(
                "{LABEL} instance representation must shadow a different model TC1/TC2 pair"
            ));
        }

        let option_directives = Self::logical_netlist_lines(source)
            .into_iter()
            .map(|line| Self::strip_netlist_comment(&line).trim().to_string())
            .filter(|line| {
                line.split_whitespace()
                    .next()
                    .is_some_and(|command| command.eq_ignore_ascii_case(".options"))
            })
            .map(|line| {
                line.split_whitespace()
                    .map(str::to_ascii_lowercase)
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();

        Ok(XycePassiveTemperatureOverrideSnapshot {
            title: netlist.title.trim().to_string(),
            device_kind,
            representation,
            elements,
            model_name: model.name.trim().to_ascii_lowercase(),
            model_type: model.model_type.trim().to_ascii_lowercase(),
            winning_tc_bits: winning_tc.map(Value::to_bits),
            effective_primary_bits: effective_primary.to_bits(),
            option_directives,
        })
    }

    pub(super) fn ac_analysis_expression_snapshot(
        netlist: &Netlist,
    ) -> Result<XyceAcAnalysisExpressionSnapshot, String> {
        const LABEL: &str = "AC-analysis expression parity";
        let source = netlist
            .source_text
            .as_deref()
            .ok_or_else(|| format!("{LABEL} requires original source text"))?;
        let (representation, parameter_bits) = Self::ac_analysis_source_qualification(source)?;
        let footer_suppressed = Self::logical_netlist_lines(source)
            .into_iter()
            .skip(1)
            .map(|line| Self::strip_netlist_comment(&line).trim().to_string())
            .any(|line| {
                line.split_whitespace()
                    .next()
                    .is_some_and(|command| command.eq_ignore_ascii_case(".options"))
                    && Self::ac_analysis_output_option_is_footer_suppression(&line).unwrap_or(false)
            });
        let parsed_parameter_bits = netlist
            .params
            .numeric_parameters()
            .into_iter()
            .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
            .collect::<BTreeMap<_, _>>();
        if parsed_parameter_bits != parameter_bits
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} parsed scalar parameter state differs from the qualified direct assignments"
            ));
        }
        let diagnostics_are_canonical_footer_only = footer_suppressed
            && representation == XyceAcAnalysisRepresentation::DirectNumeric
            && netlist.diagnostics.len() == 1;
        if netlist.title.trim().is_empty()
            || netlist.title.trim_start().starts_with('.')
            || !netlist.models.is_empty()
            || (!netlist.diagnostics.is_empty() && !diagnostics_are_canonical_footer_only)
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Ac { .. }])
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "{LABEL} requires one flat diagnostic-free ordinary AC analysis without models, hierarchy, auxiliary state, or external devices (models={}, diagnostics={}, analyses={}, fft={}, data={}, subckts={}, ic={}, nodesets={}, globals={}, measures={}, veriloga={}, spef={})",
                netlist.models.len(),
                netlist.diagnostics.len(),
                netlist.analyses.len(),
                netlist.fft_analyses.len(),
                netlist.data_tables.len(),
                netlist.subcircuits.len(),
                netlist.initial_conditions.len(),
                netlist.node_sets.len(),
                netlist.global_nodes.len(),
                netlist.measurements.len(),
                netlist.veriloga_includes.len(),
                netlist.spef_includes.len(),
            ));
        }
        for element in &netlist.elements {
            if element
                .nodes
                .iter()
                .any(|node| Self::xyce_ground_alias_name(node) && node.trim() != "0")
            {
                return Err(format!(
                    "{LABEL} element '{}' uses a ground alias; literal node 0 is required",
                    element.name
                ));
            }
            let valid = match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    element.nodes.len() == 2
                        && value.is_finite()
                        && *value > 0.0
                        && value_expr.is_none()
                        && model.is_none()
                        && instance_params.is_empty()
                        && deferred_params.is_empty()
                }
                ElementKind::Capacitor {
                    value,
                    value_expr,
                    initial_voltage,
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    element.nodes.len() == 2
                        && value.is_finite()
                        && *value > 0.0
                        && value_expr.is_none()
                        && initial_voltage.is_none()
                        && model.is_none()
                        && instance_params.is_empty()
                        && deferred_params.is_empty()
                }
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    let (magnitude, phase) = extract_ac_value(spec);
                    element.nodes.len() == 2 && magnitude.is_finite() && phase.is_finite()
                }
                _ => false,
            };
            if !valid {
                return Err(format!(
                    "{LABEL} contains unqualified element '{}'",
                    element.name
                ));
            }
        }

        let mut nonrepresentation_source = Vec::new();
        for line in Self::logical_netlist_lines(source).into_iter().skip(1) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let command = stripped.split_whitespace().next().unwrap_or_default();
            if command.eq_ignore_ascii_case(".param")
                || (command.eq_ignore_ascii_case(".options")
                    && Self::ac_analysis_output_option_is_footer_suppression(stripped)?)
            {
                continue;
            }
            if command.eq_ignore_ascii_case(".ac") {
                nonrepresentation_source.push(".ac <analysis-expression>".to_string());
            } else {
                nonrepresentation_source.push(
                    stripped
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .to_ascii_lowercase(),
                );
            }
        }
        Ok(XyceAcAnalysisExpressionSnapshot {
            representation,
            parameter_bits,
            nonrepresentation_source,
            footer_suppressed,
        })
    }

    pub(super) fn delimited_expression_family_snapshot(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
    ) -> Result<XyceDelimitedExpressionFamilySnapshot, String> {
        const LABEL: &str = "delimited-expression parity";
        let (representation, expression_sites) =
            Self::delimited_expression_source_qualification(&plan.source)?;
        if netlist.elements.len() != 2
            || netlist.analyses.len() != 1
            || !matches!(netlist.analyses[0], AnalysisCommand::Dc { .. })
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err(format!(
                "{LABEL} parsed netlist contains state outside the qualified R/V/DC surface"
            ));
        }
        let parameter_bits = netlist
            .params
            .all_params()
            .into_iter()
            .map(|(name, value)| {
                if !value.is_finite() {
                    return Err(format!("{LABEL} parameter '{name}' is non-finite"));
                }
                Ok((name.to_ascii_lowercase(), value.to_bits()))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        if parameter_bits.len() != 1 {
            return Err(format!("{LABEL} requires exactly one resolved parameter"));
        }

        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            let nodes = element
                .nodes
                .iter()
                .map(|node| node.trim().to_ascii_lowercase())
                .collect::<Vec<_>>();
            if nodes.len() != 2 || nodes[0].is_empty() || nodes[0] == "0" || nodes[1] != "0" {
                return Err(format!(
                    "{LABEL} requires each R/V element from one literal non-ground node to literal node 0"
                ));
            }
            let fingerprint = match &element.kind {
                ElementKind::Resistor { .. } => {
                    let value = Self::effective_resistor_value(netlist, &element.name)?
                        .ok_or_else(|| format!("{LABEL} could not resolve resistor value"))?;
                    if !value.is_finite() || value <= 0.0 {
                        return Err(format!(
                            "{LABEL} resistor value must be finite and strictly positive"
                        ));
                    }
                    XyceRelationalElementFingerprint {
                        kind: "r".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) => {
                    let value = *value;
                    if !value.is_finite() {
                        return Err(format!("{LABEL} voltage-source value is non-finite"));
                    }
                    XyceRelationalElementFingerprint {
                        kind: "v".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::VoltageSource(_) => {
                    return Err(format!(
                        "{LABEL} voltage source must be one direct SourceSpec::Dc"
                    ));
                }
                _ => return Err(format!("{LABEL} contains a non-R/V element")),
            };
            if elements
                .insert(element.name.trim().to_ascii_lowercase(), fingerprint)
                .is_some()
            {
                return Err(format!("{LABEL} contains duplicate element identity"));
            }
        }
        let mut signal_nodes = elements
            .values()
            .map(|element| element.nodes[0].as_str())
            .collect::<BTreeSet<_>>();
        if signal_nodes.len() != 1 {
            return Err(format!(
                "{LABEL} resistor and voltage source must share one non-ground node"
            ));
        }
        let _ = signal_nodes.pop_first();
        if !elements.iter().any(|(name, element)| {
            element.kind == "v" && Self::device_instance_names_match(name, &plan.dc.source)
        }) {
            return Err(format!(
                "{LABEL} .DC sweep source must be the qualified voltage source"
            ));
        }

        let print_probes = plan
            .print
            .probes
            .iter()
            .map(|probe| {
                if let Some(expression) = Self::print_expression_inner(probe) {
                    Self::parse_expression_fingerprint(expression)
                        .map(XycePrintSemanticFingerprint::Expression)
                } else {
                    Ok(XycePrintSemanticFingerprint::Atomic(Self::normalize_probe(
                        probe,
                    )))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(XyceDelimitedExpressionFamilySnapshot {
            representation,
            expression_sites,
            parameter_bits,
            elements,
            print_probes,
        })
    }

    pub(super) fn dc_analysis_expression_snapshot(
        netlist: &Netlist,
    ) -> Result<XyceDcAnalysisExpressionSnapshot, String> {
        const LABEL: &str = "DC-analysis expression parity";
        let source = netlist
            .source_text
            .as_deref()
            .ok_or_else(|| format!("{LABEL} requires original source text"))?;
        let (representation, parameter_bits) = Self::dc_analysis_source_qualification(source)?;
        let parsed_parameter_bits = netlist
            .params
            .numeric_parameters()
            .into_iter()
            .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
            .collect::<BTreeMap<_, _>>();
        if parsed_parameter_bits != parameter_bits
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} parsed scalar parameter state differs from the qualified direct assignments"
            ));
        }
        if !netlist.diagnostics.is_empty()
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Dc { .. }])
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "{LABEL} requires a flat diagnostic-free DC deck without auxiliary analyses or external model state"
            ));
        }
        for model in &netlist.models {
            if !Self::model_is_native_dc_analysis_expression_mos1(model) {
                return Err(format!(
                    "{LABEL} admits only direct native classic MOS LEVEL=1 models"
                ));
            }
        }
        for element in &netlist.elements {
            if element
                .nodes
                .iter()
                .any(|node| Self::xyce_ground_alias_name(node) && node.trim() != "0")
            {
                return Err(format!(
                    "{LABEL} element '{}' uses a ground alias; literal node 0 is required",
                    element.name
                ));
            }
            let valid = match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    element.nodes.len() == 2
                        && value.is_finite()
                        && *value > 0.0
                        && value_expr.is_none()
                        && model.is_none()
                        && instance_params.is_empty()
                        && deferred_params.is_empty()
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) => {
                    element.nodes.len() == 2 && value.is_finite()
                }
                ElementKind::Mosfet {
                    model,
                    compact_syntax,
                    instance_params,
                    deferred_params,
                    ..
                } => {
                    element.nodes.len() == 4
                        && !compact_syntax
                        && instance_params.is_empty()
                        && deferred_params.is_empty()
                        && netlist.models.iter().any(|candidate| {
                            candidate.name.eq_ignore_ascii_case(model)
                                && matches!(
                                    candidate.model_type.to_ascii_uppercase().as_str(),
                                    "NMOS" | "PMOS"
                                )
                        })
                }
                _ => false,
            };
            if !valid {
                return Err(format!(
                    "{LABEL} contains unqualified element '{}'",
                    element.name
                ));
            }
        }

        let mut nonrepresentation_source = Vec::new();
        for line in Self::logical_netlist_lines(source).into_iter().skip(1) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let command = stripped.split_whitespace().next().unwrap_or_default();
            if command.eq_ignore_ascii_case(".param") {
                continue;
            }
            if command.eq_ignore_ascii_case(".dc") {
                nonrepresentation_source.push(".dc <analysis-expression>".to_string());
            } else {
                nonrepresentation_source.push(
                    stripped
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .to_ascii_lowercase(),
                );
            }
        }
        Ok(XyceDcAnalysisExpressionSnapshot {
            representation,
            parameter_bits,
            nonrepresentation_source,
        })
    }

    pub(super) fn transient_analysis_expression_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<XyceTransientAnalysisExpressionSnapshot, String> {
        const LABEL: &str = "transient-analysis expression parity";
        let source = netlist
            .source_text
            .as_deref()
            .ok_or_else(|| format!("{LABEL} requires original source text"))?;
        let (representation, parameter_bits) =
            Self::transient_analysis_source_qualification(source)?;
        if netlist.title.trim().is_empty()
            || netlist.title.trim_start().starts_with('.')
            || print.probes.is_empty()
            || !netlist.models.is_empty()
            || !netlist.diagnostics.is_empty()
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }])
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} requires one flat diagnostic-free transient analysis without models, hierarchy, auxiliary state, or external devices"
            ));
        }
        let parsed_parameter_bits = netlist
            .params
            .all_params()
            .into_iter()
            .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
            .collect::<BTreeMap<_, _>>();
        if parsed_parameter_bits != parameter_bits {
            return Err(format!(
                "{LABEL} parsed parameters differ from the direct source qualification"
            ));
        }

        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            let key = Self::normalize_device_instance_name(&element.name);
            if key.is_empty() || elements.contains_key(&key) {
                return Err(format!(
                    "{LABEL} contains an empty or duplicate element name"
                ));
            }
            let nodes = element
                .nodes
                .iter()
                .map(|node| node.trim().to_ascii_lowercase())
                .collect::<Vec<_>>();
            if nodes.len() != 2
                || nodes.iter().any(String::is_empty)
                || nodes
                    .iter()
                    .any(|node| Self::xyce_ground_alias_name(node) && node != "0")
            {
                return Err(format!(
                    "{LABEL} element '{}' must have two explicit nodes and literal ground",
                    element.name
                ));
            }
            let fingerprint = match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } if value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty() =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "R".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::Capacitor {
                    value,
                    value_expr,
                    initial_voltage,
                    model,
                    instance_params,
                    deferred_params,
                } if value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && initial_voltage.is_none_or(Value::is_finite)
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty() =>
                {
                    let mut numeric_bits = vec![value.to_bits()];
                    let marker = if let Some(initial) = initial_voltage {
                        numeric_bits.push(initial.to_bits());
                        "IC"
                    } else {
                        "NO_IC"
                    };
                    XyceRelationalElementFingerprint {
                        kind: "C".to_string(),
                        nodes,
                        numeric_bits,
                        text: vec![marker.to_string()],
                    }
                }
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Self::transient_analysis_source_fingerprint(element, spec, nodes)?
                }
                _ => {
                    return Err(format!(
                        "{LABEL} contains unqualified element '{}'",
                        element.name
                    ));
                }
            };
            elements.insert(key, fingerprint);
        }
        if elements.is_empty() {
            return Err(format!("{LABEL} requires at least one qualified element"));
        }

        let option_directives = Self::logical_netlist_lines(source)
            .into_iter()
            .map(|line| Self::strip_netlist_comment(&line).trim().to_string())
            .filter(|line| {
                line.split_whitespace()
                    .next()
                    .is_some_and(|command| command.eq_ignore_ascii_case(".options"))
            })
            .map(|line| {
                line.split_whitespace()
                    .map(str::to_ascii_lowercase)
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();
        let mut nonrepresentation_source = Vec::new();
        for line in Self::logical_netlist_lines(source).into_iter().skip(1) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".param") {
                continue;
            }
            if command.eq_ignore_ascii_case(".tran") {
                let arity = Self::split_grouped_whitespace_fields(
                    stripped,
                    "transient-analysis .TRAN source signature",
                )?
                .len()
                .saturating_sub(1);
                nonrepresentation_source.push(format!(".tran$arity={arity}"));
                continue;
            }
            nonrepresentation_source.push(
                Self::split_grouped_whitespace_fields(
                    stripped,
                    "transient-analysis source signature",
                )?
                .into_iter()
                .map(|field| field.to_ascii_lowercase())
                .collect::<Vec<_>>()
                .join(" "),
            );
        }

        Ok(XyceTransientAnalysisExpressionSnapshot {
            title: netlist.title.trim().to_string(),
            representation,
            elements,
            option_directives,
            parameter_bits,
            nonrepresentation_source,
        })
    }

    pub(super) fn diode_model_alias_family_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<XyceDiodeModelAliasFamilySnapshot, String> {
        const LABEL: &str = "native diode model-parameter alias equivalence";
        let source = netlist
            .source_text
            .as_deref()
            .ok_or_else(|| format!("{LABEL} requires original source text"))?;
        let (representation, canonical_source) =
            Self::diode_model_alias_source_qualification(source)?;
        if !netlist.diagnostics.is_empty()
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }])
            || netlist.models.len() != 1
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "{LABEL} requires one flat, parameter-free, diagnostic-free native transient circuit without auxiliary state"
            ));
        }
        let model = &netlist.models[0];
        if !model.model_type.eq_ignore_ascii_case("D")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
            || model.params.len() != 12
        {
            return Err(format!(
                "{LABEL} requires one scalar numeric native D model"
            ));
        }
        let canonical_name = |name: &str| match name.to_ascii_uppercase().as_str() {
            "JS" => "IS".to_string(),
            "VB" => "BV".to_string(),
            "CJ" => "CJO".to_string(),
            other => other.to_string(),
        };
        let mut canonical_model_bits = model
            .params
            .iter()
            .map(|(name, value)| (canonical_name(name), value.to_bits()))
            .collect::<Vec<_>>();
        canonical_model_bits.sort_by(|left, right| left.0.cmp(&right.0));
        if canonical_model_bits
            .windows(2)
            .any(|pair| pair[0].0 == pair[1].0)
            || model.params.iter().any(|(_, value)| !value.is_finite())
        {
            return Err(format!(
                "{LABEL} model parameters are non-finite or ambiguous"
            ));
        }

        let mut elements = BTreeMap::new();
        let mut source_nodes = None;
        let mut diode_nodes = None;
        let mut resistor_nodes = None;
        for element in &netlist.elements {
            if element.nodes.iter().any(|node| {
                Self::canonical_passive_primary_node_name(node) == "0" && node.trim() != "0"
            }) {
                return Err(format!("{LABEL} requires literal node 0 for ground"));
            }
            let nodes = element
                .nodes
                .iter()
                .map(|node| Self::canonical_passive_primary_node_name(node))
                .collect::<Vec<_>>();
            let fingerprint = match &element.kind {
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pulse {
                    v1,
                    v2,
                    delay,
                    rise,
                    fall,
                    width,
                    period,
                    phase,
                    width_defaults_to_zero,
                }) if nodes.len() == 2
                    && [v1, v2, delay, rise, fall, width, period, phase]
                        .iter()
                        .all(|value| value.is_finite())
                    && *delay >= 0.0
                    && *rise > 0.0
                    && *fall > 0.0
                    && *width > 0.0
                    && *period > 0.0
                    && *rise + *width + *fall <= *period
                    && !*width_defaults_to_zero
                    && source_nodes.replace(nodes.clone()).is_none() =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "V:PULSE".to_string(),
                        nodes,
                        numeric_bits: [v1, v2, delay, rise, fall, width, period, phase]
                            .into_iter()
                            .map(|value| value.to_bits())
                            .collect(),
                        text: vec![width_defaults_to_zero.to_string()],
                    }
                }
                ElementKind::Diode {
                    model: device_model,
                    instance_params,
                    deferred_params,
                } if nodes.len() == 2
                    && device_model.eq_ignore_ascii_case(&model.name)
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
                    && diode_nodes.replace(nodes.clone()).is_none() =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "D".to_string(),
                        nodes,
                        numeric_bits: Vec::new(),
                        text: vec![device_model.to_ascii_lowercase()],
                    }
                }
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } if nodes.len() == 2
                    && value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
                    && resistor_nodes.replace(nodes.clone()).is_none() =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "R".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                _ => return Err(format!("{LABEL} contains an unqualified native element")),
            };
            let key = Self::normalize_device_instance_name(&element.name);
            if key.is_empty() || elements.insert(key, fingerprint).is_some() {
                return Err(format!("{LABEL} has an empty or duplicate element name"));
            }
        }
        let [source_pos, source_neg] = source_nodes
            .as_deref()
            .ok_or_else(|| format!("{LABEL} has no PULSE source"))?
        else {
            return Err(format!("{LABEL} source topology is invalid"));
        };
        let [diode_anode, diode_cathode] = diode_nodes
            .as_deref()
            .ok_or_else(|| format!("{LABEL} has no diode"))?
        else {
            return Err(format!("{LABEL} diode topology is invalid"));
        };
        let [resistor_pos, resistor_neg] = resistor_nodes
            .as_deref()
            .ok_or_else(|| format!("{LABEL} has no resistor"))?
        else {
            return Err(format!("{LABEL} resistor topology is invalid"));
        };
        if elements.len() != 3
            || source_neg != "0"
            || resistor_neg != "0"
            || source_pos == "0"
            || diode_cathode == "0"
            || diode_anode != source_pos
            || resistor_pos != diode_cathode
            || source_pos == diode_cathode
        {
            return Err(format!(
                "{LABEL} requires the grounded PULSE-source/diode/resistor series topology"
            ));
        }
        let [first_probe, second_probe] = print.probes.as_slice() else {
            return Err(format!("{LABEL} requires exactly two ordered probes"));
        };
        let first = Self::parse_voltage_probe(first_probe)
            .ok_or_else(|| format!("{LABEL} first probe is not an atomic voltage"))?;
        let second = Self::parse_voltage_probe(second_probe)
            .ok_or_else(|| format!("{LABEL} second probe is not an atomic voltage"))?;
        if first.accessor != XyceVoltageAccessor::Value
            || second.accessor != XyceVoltageAccessor::Value
            || first.node_neg.is_some()
            || second.node_neg.is_some()
            || Self::canonical_passive_primary_node_name(&first.node_pos) != *source_pos
            || Self::canonical_passive_primary_node_name(&second.node_pos) != *diode_cathode
        {
            return Err(format!(
                "{LABEL} probes must be ordered source-node then load-node voltage"
            ));
        }

        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        });
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|err| format!("{LABEL} circuit build failed: {err}"))?;
        let [runtime] = circuit.diode_storage().devices.as_slice() else {
            return Err(format!("{LABEL} must resolve to exactly one native diode"));
        };
        let runtime_diode = XyceNativeDiodeRuntimeFingerprint {
            name: Self::normalize_device_instance_name(&runtime.name),
            node_anode: runtime.node_anode,
            node_cathode: runtime.node_cathode,
            numeric_bits: [
                runtime.is,
                runtime.n,
                runtime.vt,
                runtime.rs,
                runtime.bv.unwrap_or(0.0),
                runtime.ibv,
                runtime.forward_knee_current,
                runtime.reverse_knee_current,
                runtime.recombination_saturation_current,
                runtime.recombination_emission_coefficient,
                runtime.sidewall_perimeter,
                runtime.sidewall_saturation_current,
                runtime.sidewall_emission_coefficient,
                runtime.cj0,
                runtime.vj,
                runtime.m,
                runtime.tt,
                runtime.fc,
                runtime.sidewall_cj0,
                runtime.sidewall_vj,
                runtime.sidewall_m,
                runtime.sidewall_fc,
                runtime.breakdown_emission_coefficient,
                runtime.xti,
                runtime.eg,
                runtime.tnom_c.unwrap_or(0.0),
                runtime.kf,
                runtime.af,
                runtime.multiplicity,
            ]
            .into_iter()
            .map(Value::to_bits)
            .collect(),
            boolean_state: vec![
                runtime.bv.is_some(),
                runtime.sidewall_current_given,
                runtime.sidewall_emission_given,
                runtime.breakdown_emission_given,
                runtime.tnom_c.is_some(),
                runtime.level == rspice_core::device::DiodeLevel::Pspice,
            ],
        };
        Ok(XyceDiodeModelAliasFamilySnapshot {
            representation,
            canonical_source,
            elements,
            model_name: model.name.to_ascii_lowercase(),
            model_type: model.model_type.to_ascii_uppercase(),
            canonical_model_bits,
            ordered_probes: print
                .probes
                .iter()
                .map(|probe| Self::normalize_probe(probe))
                .collect(),
            runtime_diode,
        })
    }

    pub(super) fn switch_state_case_family_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<XyceSwitchStateCaseFamilySnapshot, String> {
        const LABEL: &str = "generic-switch initial-state case equivalence";
        let source = netlist
            .source_text
            .as_deref()
            .ok_or_else(|| format!("{LABEL} requires original source text"))?;
        let (representation, canonical_source) =
            Self::switch_state_case_source_qualification(source)?;
        if !netlist.diagnostics.is_empty()
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }])
            || netlist.models.len() != 1
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "{LABEL} requires one flat, parameter-free, diagnostic-free native transient circuit without auxiliary state"
            ));
        }
        let model = &netlist.models[0];
        if !model.model_type.eq_ignore_ascii_case("SWITCH")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} requires one scalar numeric SWITCH model"));
        }
        let mut model_numeric_bits = model
            .params
            .iter()
            .map(|(name, value)| (name.to_ascii_uppercase(), value.to_bits()))
            .collect::<Vec<_>>();
        model_numeric_bits.sort_by(|left, right| left.0.cmp(&right.0));
        if model_numeric_bits.len() != 4
            || model_numeric_bits
                .iter()
                .map(|(name, _)| name.as_str())
                .ne(["OFF", "ON", "ROFF", "RON"])
            || model.params.iter().any(|(_, value)| !value.is_finite())
        {
            return Err(format!(
                "{LABEL} model requires finite scalar OFF, ON, ROFF, and RON parameters"
            ));
        }
        let model_value = |name: &str| {
            model
                .params
                .iter()
                .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
                .map(|(_, value)| *value)
        };
        if model_value("RON").is_none_or(|value| value <= 0.0)
            || model_value("ROFF")
                .zip(model_value("RON"))
                .is_none_or(|(roff, ron)| roff <= ron)
            || model_value("ON")
                .zip(model_value("OFF"))
                .is_none_or(|(on, off)| on == off)
        {
            return Err(format!(
                "{LABEL} requires positive switch resistances and distinct ON/OFF controls"
            ));
        }

        let mut elements = BTreeMap::new();
        let mut source_nodes = None;
        let mut resistor_nodes = None;
        let mut switch_nodes = None;
        let mut control_expression = None;
        for element in &netlist.elements {
            if element.nodes.iter().any(|node| {
                Self::canonical_passive_primary_node_name(node) == "0" && node.trim() != "0"
            }) {
                return Err(format!("{LABEL} requires literal node 0 for ground"));
            }
            let nodes = element
                .nodes
                .iter()
                .map(|node| Self::canonical_passive_primary_node_name(node))
                .collect::<Vec<_>>();
            let fingerprint = match &element.kind {
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if nodes.len() == 2 && value.is_finite() =>
                {
                    if source_nodes.replace(nodes.clone()).is_some() {
                        return Err(format!("{LABEL} requires one finite direct DC source"));
                    }
                    XyceRelationalElementFingerprint {
                        kind: "V:DC".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } if nodes.len() == 2
                    && value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
                    && resistor_nodes.replace(nodes.clone()).is_none() =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "R".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::GenericSwitch {
                    model,
                    control_expression: expression,
                    initial_state: Some(state),
                } if nodes.len() == 2
                    && model.eq_ignore_ascii_case(&netlist.models[0].name)
                    && switch_nodes.replace(nodes.clone()).is_none() =>
                {
                    let state = match state {
                        rspice_core::netlist::SwitchState::On => "ON",
                        rspice_core::netlist::SwitchState::Off => "OFF",
                    };
                    control_expression = Some(Self::parse_expression_fingerprint(expression)?);
                    XyceRelationalElementFingerprint {
                        kind: "S:GENERIC".to_string(),
                        nodes,
                        numeric_bits: Vec::new(),
                        text: vec![model.to_ascii_lowercase(), state.to_string()],
                    }
                }
                _ => return Err(format!("{LABEL} contains an unqualified native element")),
            };
            let key = Self::normalize_device_instance_name(&element.name);
            if key.is_empty() || elements.insert(key, fingerprint).is_some() {
                return Err(format!("{LABEL} has an empty or duplicate element name"));
            }
        }
        let [source_pos, source_neg] = source_nodes
            .as_deref()
            .ok_or_else(|| format!("{LABEL} has no source"))?
        else {
            return Err(format!("{LABEL} source topology is invalid"));
        };
        let [switch_pos, switch_neg] = switch_nodes
            .as_deref()
            .ok_or_else(|| format!("{LABEL} has no generic switch"))?
        else {
            return Err(format!("{LABEL} switch topology is invalid"));
        };
        let [resistor_pos, resistor_neg] = resistor_nodes
            .as_deref()
            .ok_or_else(|| format!("{LABEL} has no resistor"))?
        else {
            return Err(format!("{LABEL} resistor topology is invalid"));
        };
        if elements.len() != 3
            || source_neg != "0"
            || resistor_neg != "0"
            || source_pos == "0"
            || switch_neg == "0"
            || switch_pos != source_pos
            || resistor_pos != switch_neg
            || source_pos == switch_neg
        {
            return Err(format!(
                "{LABEL} requires the grounded DC-source/switch/resistor series topology"
            ));
        }
        let [first_probe, second_probe] = print.probes.as_slice() else {
            return Err(format!("{LABEL} requires exactly two ordered probes"));
        };
        let first = Self::parse_voltage_probe(first_probe)
            .ok_or_else(|| format!("{LABEL} first probe is not an atomic voltage"))?;
        let second = Self::parse_voltage_probe(second_probe)
            .ok_or_else(|| format!("{LABEL} second probe is not an atomic voltage"))?;
        if first.accessor != XyceVoltageAccessor::Value
            || second.accessor != XyceVoltageAccessor::Value
            || first.node_neg.is_some()
            || second.node_neg.is_some()
            || Self::canonical_passive_primary_node_name(&first.node_pos) != *source_pos
            || Self::canonical_passive_primary_node_name(&second.node_pos) != *switch_neg
        {
            return Err(format!(
                "{LABEL} probes must be ordered source-node then load-node voltage"
            ));
        }

        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        });
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|err| format!("{LABEL} circuit build failed: {err}"))?;
        let [runtime] = circuit.generic_switch_storage() else {
            return Err(format!(
                "{LABEL} must resolve to exactly one native generic switch"
            ));
        };
        let [AnalysisCommand::Tran { stop, .. }] = netlist.analyses.as_slice() else {
            unreachable!("qualified above")
        };
        if !runtime
            .time_breakpoints()
            .iter()
            .any(|breakpoint| breakpoint.is_finite() && *breakpoint > 0.0 && *breakpoint < *stop)
        {
            return Err(format!(
                "{LABEL} CONTROL must produce a finite switching breakpoint inside the transient interval"
            ));
        }
        let runtime_switch = XyceGenericSwitchRuntimeFingerprint {
            name: Self::normalize_device_instance_name(&runtime.name),
            node_pos: runtime.node_pos,
            node_neg: runtime.node_neg,
            numeric_bits: [
                runtime.ron.to_bits(),
                runtime.roff.to_bits(),
                runtime.on.to_bits(),
                runtime.off.to_bits(),
                runtime.onh.to_bits(),
                runtime.offh.to_bits(),
            ],
            hysteresis_enabled: runtime.hysteresis_enabled,
            time_breakpoint_bits: runtime
                .time_breakpoints()
                .iter()
                .map(|value| value.to_bits())
                .collect(),
        };
        Ok(XyceSwitchStateCaseFamilySnapshot {
            representation,
            canonical_source,
            elements,
            control_expression: control_expression
                .ok_or_else(|| format!("{LABEL} has no control expression"))?,
            model_name: model.name.to_ascii_lowercase(),
            model_type: model.model_type.to_ascii_uppercase(),
            model_numeric_bits,
            ordered_probes: print
                .probes
                .iter()
                .map(|probe| Self::normalize_probe(probe))
                .collect(),
            runtime_switch,
        })
    }

    pub(super) fn age_cap_family_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<XyceAgeCapFamilySnapshot, String> {
        const LABEL: &str = "native capacitor AGE/D equivalence";
        let source = netlist
            .source_text
            .as_deref()
            .ok_or_else(|| format!("{LABEL} requires original source text"))?;
        let representation = Self::age_cap_source_qualification(source)?;
        if netlist.title.trim().is_empty()
            || netlist.title.trim_start().starts_with('.')
            || !netlist.models.is_empty()
            || !netlist.diagnostics.is_empty()
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }])
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} requires one flat diagnostic-free native transient circuit without auxiliary or external state"
            ));
        }
        let numeric_params = netlist.params.all_params();
        if (representation == XyceAgeCapRepresentation::NativeAge && !numeric_params.is_empty())
            || (representation == XyceAgeCapRepresentation::ParameterExpression
                && (numeric_params.is_empty()
                    || numeric_params.iter().any(|(_, value)| !value.is_finite())))
        {
            return Err(format!(
                "{LABEL} has invalid representation parameter state"
            ));
        }

        let mut elements = BTreeMap::new();
        let mut resistors = Vec::new();
        let mut pulse = None;
        let mut monitor = None;
        let mut capacitor = None;
        let mut effective_capacitance = None;
        for element in &netlist.elements {
            let key = Self::normalize_device_instance_name(&element.name);
            if element.nodes.iter().any(|node| {
                Self::canonical_passive_primary_node_name(node) == "0" && node.trim() != "0"
            }) {
                return Err(format!("{LABEL} requires literal node 0 for ground"));
            }
            let nodes = element
                .nodes
                .iter()
                .map(|node| Self::canonical_passive_primary_node_name(node))
                .collect::<Vec<_>>();
            let fingerprint = match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } if nodes.len() == 2
                    && value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty() =>
                {
                    resistors.push(nodes.clone());
                    XyceRelationalElementFingerprint {
                        kind: "R".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::VoltageSource(spec) if nodes.len() == 2 => {
                    let (waveform, bits) = Self::scoped_model_source_fingerprint(spec)?;
                    // Exactly one bounded PULSE stimulus and one zero-DC
                    // monitor are admitted, and `replace` returning `Some`
                    // means a second of that role appeared. The DC arm stays
                    // guarded by the pulse result so its side effect keeps
                    // the original short-circuit order.
                    let bounded_pulse = waveform == "PULSE"
                        && Self::age_cap_pulse_spec_is_bounded(spec)
                        && pulse.replace(nodes.clone()).is_none();
                    let zero_dc_monitor = !bounded_pulse
                        && waveform == "DC"
                        && bits == [0.0f64.to_bits()]
                        && monitor
                            .replace((element.name.clone(), nodes.clone()))
                            .is_none();
                    if !bounded_pulse && !zero_dc_monitor {
                        return Err(format!("{LABEL} has an unqualified voltage source"));
                    }
                    XyceRelationalElementFingerprint {
                        kind: format!("V:{waveform}"),
                        nodes,
                        numeric_bits: bits,
                        text: Vec::new(),
                    }
                }
                ElementKind::Capacitor {
                    value,
                    value_expr,
                    initial_voltage,
                    model,
                    instance_params,
                    deferred_params,
                } if nodes.len() == 2
                    && value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && initial_voltage.is_none()
                    && model.is_none()
                    && deferred_params.is_empty() =>
                {
                    let aged_params = instance_params
                        .iter()
                        .filter(|(name, _)| {
                            name.eq_ignore_ascii_case("AGE") || name.eq_ignore_ascii_case("D")
                        })
                        .count();
                    if (representation == XyceAgeCapRepresentation::NativeAge
                        && (aged_params != instance_params.len() || aged_params == 0))
                        || (representation == XyceAgeCapRepresentation::ParameterExpression
                            && !instance_params.is_empty())
                        || capacitor.is_some()
                    {
                        return Err(format!("{LABEL} capacitor representation is inconsistent"));
                    }
                    let effective = Engine::new(SimulationConfig {
                        spice_dialect: SpiceDialect::Xyce,
                        ..SimulationConfig::default()
                    })
                    .resolved_capacitor_value(netlist, &element.name)
                    .map_err(|err| format!("{LABEL} could not resolve capacitance: {err}"))?
                    .ok_or_else(|| format!("{LABEL} has no resolved capacitance"))?;
                    if !effective.is_finite() || effective <= 0.0 {
                        return Err(format!("{LABEL} resolved capacitance is invalid"));
                    }
                    effective_capacitance = Some(effective);
                    capacitor = Some((element.name.clone(), nodes.clone()));
                    XyceRelationalElementFingerprint {
                        kind: "C:EFFECTIVE".to_string(),
                        nodes,
                        numeric_bits: vec![effective.to_bits()],
                        text: Vec::new(),
                    }
                }
                _ => return Err(format!("{LABEL} contains an unqualified native element")),
            };
            if key.is_empty() || elements.insert(key, fingerprint).is_some() {
                return Err(format!("{LABEL} has an empty or duplicate element name"));
            }
        }
        if elements.len() != 5 || resistors.len() != 2 {
            return Err(format!("{LABEL} requires exactly two R, two V, and one C"));
        }
        let pulse_nodes = pulse.ok_or_else(|| format!("{LABEL} has no pulse source"))?;
        let (monitor_name, monitor_nodes) =
            monitor.ok_or_else(|| format!("{LABEL} has no zero-volt monitor"))?;
        let (capacitor_name, capacitor_nodes) =
            capacitor.ok_or_else(|| format!("{LABEL} has no capacitor"))?;
        let [drive, ground] = pulse_nodes.as_slice() else {
            return Err(format!("{LABEL} pulse topology is invalid"));
        };
        let [monitor_input, cap_node] = monitor_nodes.as_slice() else {
            return Err(format!("{LABEL} monitor topology is invalid"));
        };
        if ground != "0"
            || drive == "0"
            || monitor_input == "0"
            || cap_node == "0"
            || drive == monitor_input
            || drive == cap_node
            || monitor_input == cap_node
            || capacitor_nodes != [cap_node.clone(), "0".to_string()]
            || !resistors
                .iter()
                .any(|nodes| nodes == &[drive.clone(), monitor_input.clone()])
            || !resistors
                .iter()
                .any(|nodes| nodes == &[monitor_input.clone(), "0".to_string()])
        {
            return Err(format!(
                "{LABEL} requires the bounded pulse/R/R/monitor/C topology"
            ));
        }
        let [current_text, voltage_text] = print.probes.as_slice() else {
            return Err(format!("{LABEL} requires two ordered probes"));
        };
        let current = Self::parse_current_probe(current_text)
            .ok_or_else(|| format!("{LABEL} first probe is not an atomic current"))?;
        let voltage = Self::parse_voltage_probe(voltage_text)
            .ok_or_else(|| format!("{LABEL} second probe is not an atomic voltage"))?;
        if !Self::device_instance_names_match(&current, &monitor_name)
            || voltage.accessor != XyceVoltageAccessor::Value
            || voltage.node_neg.is_some()
            || Self::canonical_passive_primary_node_name(&voltage.node_pos) != *cap_node
            || Self::normalize_device_instance_name(&capacitor_name).is_empty()
        {
            return Err(format!(
                "{LABEL} probes must be ordered monitor current then capacitor voltage"
            ));
        }
        let option_directives = Self::logical_netlist_lines(source)
            .into_iter()
            .map(|line| {
                Self::strip_netlist_comment(&line)
                    .trim()
                    .to_ascii_lowercase()
            })
            .filter(|line| line.starts_with(".options"))
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        let age_semantics = Self::age_cap_semantic_values(
            netlist,
            source,
            representation,
            effective_capacitance.expect("qualified capacitor"),
        )?;
        Ok(XyceAgeCapFamilySnapshot {
            representation,
            elements,
            ordered_probes: print
                .probes
                .iter()
                .map(|probe| Self::normalize_probe(probe))
                .collect(),
            option_directives,
            age_semantics,
        })
    }

    pub(super) fn sin_expression_family_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<XyceSinExpressionFamilySnapshot, String> {
        if !netlist.models.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(
                "exact SIN/SPICE_SIN parity contains models, hierarchy, auxiliary analysis state, or external-model state"
                    .to_string(),
            );
        }
        if !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(
                "exact SIN/SPICE_SIN parity does not admit parameters or user functions"
                    .to_string(),
            );
        }
        if !netlist.diagnostics.is_empty() {
            return Err(format!(
                "exact SIN/SPICE_SIN parity requires a diagnostic-free parse, found {} diagnostic(s)",
                netlist.diagnostics.len()
            ));
        }
        if !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }]) {
            return Err(format!(
                "exact SIN/SPICE_SIN parity requires exactly one transient analysis, found {} analysis command(s)",
                netlist.analyses.len()
            ));
        }
        if netlist.elements.len() != 2 {
            return Err(format!(
                "exact SIN/SPICE_SIN parity requires exactly one excitation and one resistor, found {} elements",
                netlist.elements.len()
            ));
        }

        let mut resistor = None;
        let mut source = None;
        for element in &netlist.elements {
            if let Some(alias) = element.nodes.iter().find(|node| {
                Self::xyce_ground_alias_name(node)
                    && !Self::sin_expression_name_is_literal_ground(node)
            }) {
                return Err(format!(
                    "element '{}' uses ground alias '{}'; exact Xyce parity requires literal node 0 without .PREPROCESS REPLACEGND",
                    element.name, alias
                ));
            }
            let nodes = element
                .nodes
                .iter()
                .map(|node| Self::canonical_sin_expression_node_name(node))
                .collect::<Vec<_>>();
            match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    if resistor.is_some() {
                        return Err(
                            "exact SIN/SPICE_SIN parity requires exactly one resistor".to_string()
                        );
                    }
                    if nodes.len() != 2
                        || !value.is_finite()
                        || *value <= 0.0
                        || value_expr.is_some()
                        || model.is_some()
                        || !instance_params.is_empty()
                        || !deferred_params.is_empty()
                    {
                        return Err(format!(
                            "resistor '{}' is outside the finite positive numeric two-terminal envelope",
                            element.name
                        ));
                    }
                    resistor = Some((
                        Self::normalize_device_instance_name(&element.name),
                        XyceRelationalElementFingerprint {
                            kind: "R".to_string(),
                            nodes,
                            numeric_bits: vec![value.to_bits()],
                            text: Vec::new(),
                        },
                    ));
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Sin {
                    offset,
                    amplitude,
                    frequency,
                    delay,
                    damping,
                    phase,
                }) => {
                    if source.is_some() {
                        return Err("exact SIN/SPICE_SIN parity requires exactly one excitation"
                            .to_string());
                    }
                    if nodes.len() != 2
                        || delay.to_bits() != 0.0f64.to_bits()
                        || damping.to_bits() != 0.0f64.to_bits()
                        || phase.to_bits() != 0.0f64.to_bits()
                    {
                        return Err(format!(
                            "independent source '{}' must be a two-terminal three-argument SIN with exact +0 delay, damping, and phase",
                            element.name
                        ));
                    }
                    let waveform_bits = Self::qualified_sin_expression_waveform_bits(
                        *offset, *amplitude, *frequency,
                    )?;
                    source = Some((
                        element.name.clone(),
                        nodes,
                        waveform_bits,
                        XyceSinExpressionRepresentation::IndependentSin,
                    ));
                }
                ElementKind::BehavioralVoltage {
                    expression,
                    tc1,
                    tc2,
                } => {
                    if source.is_some() {
                        return Err("exact SIN/SPICE_SIN parity requires exactly one excitation"
                            .to_string());
                    }
                    if nodes.len() != 2
                        || tc1.to_bits() != 0.0f64.to_bits()
                        || tc2.to_bits() != 0.0f64.to_bits()
                    {
                        return Err(format!(
                            "behavioral source '{}' must be two-terminal with exact +0 TC1 and TC2",
                            element.name
                        ));
                    }
                    let prepared = prepare_behavioral_expression(expression, &netlist.params)
                        .map_err(|err| {
                            format!(
                                "could not canonicalize behavioral expression for '{}': {err}",
                                element.name
                            )
                        })?;
                    let ast = parse_expression_strict(&prepared).map_err(|err| {
                        format!(
                            "could not parse behavioral expression for '{}': {err}",
                            element.name
                        )
                    })?;
                    let Expr::Function {
                        func: rspice_core::expr::Function::SpiceSin,
                        args,
                    } = ast
                    else {
                        return Err(format!(
                            "behavioral source '{}' is not a direct SPICE_SIN expression",
                            element.name
                        ));
                    };
                    let [
                        Expr::Const(offset),
                        Expr::Const(amplitude),
                        Expr::Const(frequency),
                    ] = args.as_slice()
                    else {
                        return Err(format!(
                            "behavioral source '{}' must use exactly three constant SPICE_SIN arguments",
                            element.name
                        ));
                    };
                    let waveform_bits = Self::qualified_sin_expression_waveform_bits(
                        *offset, *amplitude, *frequency,
                    )?;
                    source = Some((
                        element.name.clone(),
                        nodes,
                        waveform_bits,
                        XyceSinExpressionRepresentation::BehavioralSpiceSin,
                    ));
                }
                _ => {
                    return Err(format!(
                        "element '{}' is outside the resistor/SIN/SPICE_SIN envelope",
                        element.name
                    ));
                }
            }
        }

        let (resistor_name, resistor) = resistor
            .ok_or_else(|| "exact SIN/SPICE_SIN parity contains no resistor".to_string())?;
        let (source_name, source_nodes, waveform_bits, representation) = source
            .ok_or_else(|| "exact SIN/SPICE_SIN parity contains no excitation".to_string())?;
        Self::validate_sin_expression_source_form(netlist, &source_name, representation)?;
        if source_nodes[0] == "0" || source_nodes[1] != "0" {
            return Err(
                "exact SIN/SPICE_SIN excitation requires one non-ground output node and literal node 0"
                    .to_string(),
            );
        }
        if resistor.nodes != source_nodes {
            return Err(
                "exact SIN/SPICE_SIN resistor must connect across the same ordered node pair as the excitation"
                    .to_string(),
            );
        }
        let [probe] = print.probes.as_slice() else {
            return Err("exact SIN/SPICE_SIN parity requires exactly one probe".to_string());
        };
        let probe = Self::parse_voltage_probe(probe).ok_or_else(|| {
            "exact SIN/SPICE_SIN parity requires an atomic voltage probe".to_string()
        })?;
        if probe.accessor != XyceVoltageAccessor::Value
            || probe.node_neg.is_some()
            || Self::canonical_sin_expression_node_name(&probe.node_pos) != source_nodes[0]
        {
            return Err(
                "exact SIN/SPICE_SIN voltage probe must observe the excitation output node"
                    .to_string(),
            );
        }

        Ok(XyceSinExpressionFamilySnapshot {
            resistor,
            resistor_name,
            source_nodes,
            waveform_bits,
            representation,
        })
    }

    pub(super) fn param_expression_family_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<XyceParamExpressionFamilySnapshot, String> {
        const LABEL: &str = "parameter-expression parity";
        let source = netlist.source_text.as_deref().ok_or_else(|| {
            format!("{LABEL} requires original source text for representation qualification")
        })?;
        Self::validate_param_expression_direct_source_forms(source)?;
        if !netlist.models.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "{LABEL} contains models, top-level auxiliary analysis state, or external-model state"
            ));
        }
        if !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} does not admit string parameters or user functions"
            ));
        }
        let parameters = netlist.params.all_params();
        let [(parameter_name, parameter_value)] = parameters.as_slice() else {
            return Err(format!(
                "{LABEL} requires exactly one explicit numeric global parameter, found {}",
                parameters.len()
            ));
        };
        let complex_parameter = netlist
            .params
            .get_complex(parameter_name)
            .ok_or_else(|| format!("{LABEL} could not resolve its unique global parameter"))?;
        if !complex_parameter.is_real()
            || complex_parameter.re.to_bits() != parameter_value.to_bits()
            || !parameter_value.is_finite()
            || *parameter_value <= 0.0
        {
            return Err(format!(
                "{LABEL} requires one finite positive real global parameter, got {complex_parameter:?}"
            ));
        }
        if !netlist.diagnostics.is_empty() {
            return Err(format!(
                "{LABEL} requires a diagnostic-free parse, found {} diagnostic(s)",
                netlist.diagnostics.len()
            ));
        }
        if !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }]) {
            return Err(format!(
                "{LABEL} requires exactly one transient analysis, found {} analysis command(s)",
                netlist.analyses.len()
            ));
        }
        let [subcircuit] = netlist.subcircuits.as_slice() else {
            return Err(format!(
                "{LABEL} requires exactly one subcircuit definition, found {}",
                netlist.subcircuits.len()
            ));
        };
        if !subcircuit.params.is_empty()
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.string_params.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.body_string_params.is_empty()
            || !subcircuit.body_functions.is_empty()
            || !subcircuit.local_options.is_empty()
            || subcircuit.library_ref.is_some()
            || !subcircuit.nested_subcircuits.is_empty()
            || !subcircuit.initial_conditions.is_empty()
            || !subcircuit.node_sets.is_empty()
        {
            return Err(format!(
                "{LABEL} subcircuit must not contain defaults, body parameters/functions, local options, nested hierarchy, startup state, or a library reference"
            ));
        }
        if subcircuit.ports.len() != 6 {
            return Err(format!(
                "{LABEL} subcircuit requires exactly six ports, found {}",
                subcircuit.ports.len()
            ));
        }
        let subcircuit_ports = subcircuit
            .ports
            .iter()
            .map(|port| Self::canonical_param_expression_node_name(port))
            .collect::<Vec<_>>();
        let distinct_ports = subcircuit_ports.iter().cloned().collect::<BTreeSet<_>>();
        if distinct_ports.len() != 6
            || subcircuit
                .ports
                .iter()
                .any(|port| Self::xyce_ground_alias_name(port) || port.trim().is_empty())
        {
            return Err(format!(
                "{LABEL} subcircuit ports must be six distinct non-ground names"
            ));
        }
        let [behavioral] = subcircuit.elements.as_slice() else {
            return Err(format!(
                "{LABEL} subcircuit requires exactly one behavioral voltage source, found {} elements",
                subcircuit.elements.len()
            ));
        };
        let behavioral_nodes = behavioral
            .nodes
            .iter()
            .map(|node| Self::canonical_param_expression_node_name(node))
            .collect::<Vec<_>>();
        if behavioral_nodes != subcircuit_ports[..2] {
            return Err(format!(
                "{LABEL} behavioral source must connect across subcircuit ports 0 and 1"
            ));
        }
        let representation = match &behavioral.kind {
            ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
            } if tc1.to_bits() == 0.0f64.to_bits() && tc2.to_bits() == 0.0f64.to_bits() => {
                Self::qualify_raw_param_expression(
                    expression,
                    parameter_name,
                    *parameter_value,
                    &subcircuit.ports,
                )?
            }
            ElementKind::BehavioralVoltage { .. } => {
                return Err(format!(
                    "{LABEL} behavioral source requires exact +0 TC1 and TC2"
                ));
            }
            _ => {
                return Err(format!(
                    "{LABEL} subcircuit element must be a behavioral voltage source"
                ));
            }
        };

        if netlist.elements.len() != 4 {
            return Err(format!(
                "{LABEL} requires one subcircuit instance, two resistors, and one DC voltage source, found {} top-level elements",
                netlist.elements.len()
            ));
        }
        let mut instance = None;
        let mut resistors = Vec::new();
        let mut voltage_source = None;
        for element in &netlist.elements {
            if let Some(alias) = element.nodes.iter().find(|node| {
                Self::xyce_ground_alias_name(node)
                    && !Self::param_expression_name_is_literal_ground(node)
            }) {
                return Err(format!(
                    "element '{}' uses ground alias '{}'; {LABEL} requires literal node 0",
                    element.name, alias
                ));
            }
            match &element.kind {
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } => {
                    if instance.is_some() {
                        return Err(format!("{LABEL} requires exactly one subcircuit instance"));
                    }
                    if !subckt_name.eq_ignore_ascii_case(&subcircuit.name)
                        || !params.is_empty()
                        || element.nodes.len() != 6
                    {
                        return Err(format!(
                            "subcircuit instance '{}' must bind all six ports without parameter overrides",
                            element.name
                        ));
                    }
                    instance = Some(element);
                }
                ElementKind::Resistor { .. } => resistors.push(element),
                ElementKind::VoltageSource(_) => {
                    if voltage_source.replace(element).is_some() {
                        return Err(format!("{LABEL} requires exactly one voltage source"));
                    }
                }
                _ => {
                    return Err(format!(
                        "top-level element '{}' is outside the subcircuit/resistor/DC-source envelope",
                        element.name
                    ));
                }
            }
        }
        let instance =
            instance.ok_or_else(|| format!("{LABEL} contains no subcircuit instance"))?;
        if resistors.len() != 2 {
            return Err(format!(
                "{LABEL} requires exactly two resistors, found {}",
                resistors.len()
            ));
        }
        let voltage_source =
            voltage_source.ok_or_else(|| format!("{LABEL} contains no voltage source"))?;
        if !instance
            .nodes
            .iter()
            .skip(1)
            .step_by(2)
            .all(|node| Self::param_expression_name_is_literal_ground(node))
        {
            return Err(format!(
                "subcircuit instance '{}' must bind ports 1, 3, and 5 to literal node 0",
                instance.name
            ));
        }
        let signal_nodes = [
            Self::canonical_param_expression_node_name(&instance.nodes[0]),
            Self::canonical_param_expression_node_name(&instance.nodes[2]),
            Self::canonical_param_expression_node_name(&instance.nodes[4]),
        ];
        if signal_nodes
            .iter()
            .any(|node| node == "0" || Self::xyce_ground_alias_name(node))
            || signal_nodes.iter().cloned().collect::<BTreeSet<_>>().len() != 3
        {
            return Err(
                "subcircuit instance signal bindings must be three distinct non-ground nodes"
                    .to_string(),
            );
        }

        let mut resistor_signal_nodes = BTreeSet::new();
        for resistor in resistors {
            let nodes = resistor
                .nodes
                .iter()
                .map(|node| Self::canonical_param_expression_node_name(node))
                .collect::<Vec<_>>();
            let ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
            } = &resistor.kind
            else {
                unreachable!("resistor collection is type-checked above")
            };
            if nodes.len() != 2
                || nodes[1] != "0"
                || !value.is_finite()
                || *value <= 0.0
                || value_expr.is_some()
                || model.is_some()
                || !instance_params.is_empty()
                || !deferred_params.is_empty()
                || !matches!(nodes[0].as_str(), node if node == signal_nodes[0] || node == signal_nodes[1])
                || !resistor_signal_nodes.insert(nodes[0].clone())
            {
                return Err(format!(
                    "resistor '{}' must be one unique finite positive numeric connection from instance signal 0 or 2 to literal ground",
                    resistor.name
                ));
            }
        }
        if resistor_signal_nodes
            != [signal_nodes[0].clone(), signal_nodes[1].clone()]
                .into_iter()
                .collect()
        {
            return Err(format!(
                "{LABEL} resistors must terminate instance signal nodes 0 and 2"
            ));
        }

        let voltage_nodes = voltage_source
            .nodes
            .iter()
            .map(|node| Self::canonical_param_expression_node_name(node))
            .collect::<Vec<_>>();
        let ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(source_value)) =
            &voltage_source.kind
        else {
            return Err(format!(
                "voltage source '{}' must be a direct DC source",
                voltage_source.name
            ));
        };
        if voltage_nodes != [signal_nodes[2].clone(), "0".to_string()]
            || !source_value.is_finite()
            || *source_value == 0.0
        {
            return Err(format!(
                "voltage source '{}' must drive instance signal node 4 from literal ground with a finite nonzero DC value",
                voltage_source.name
            ));
        }

        let [probe] = print.probes.as_slice() else {
            return Err(format!("{LABEL} requires exactly one output probe"));
        };
        let probe = Self::parse_voltage_probe(probe)
            .ok_or_else(|| format!("{LABEL} requires an atomic voltage probe"))?;
        if probe.accessor != XyceVoltageAccessor::Value
            || probe.node_neg.is_some()
            || Self::canonical_param_expression_node_name(&probe.node_pos) != signal_nodes[0]
        {
            return Err(format!(
                "{LABEL} output probe must observe instance signal node 0"
            ));
        }

        let flattened = rspice_core::netlist::flatten_netlist_with_models(netlist)
            .map_err(|err| format!("could not flatten {LABEL} member: {err}"))?;
        if !flattened.scoped_models.is_empty()
            || !flattened.scoped_initial_conditions.is_empty()
            || !flattened.scoped_node_sets.is_empty()
            || !flattened.xspice_auto_bridge_node_hints.is_empty()
            || flattened.elements.len() != 4
        {
            return Err(format!(
                "flattened {LABEL} member contains scoped state, bridge hints, or an unexpected element count"
            ));
        }
        let mut flattened_elements = BTreeMap::new();
        let mut flattened_resistor_nodes = BTreeSet::new();
        let mut flattened_voltage_count = 0usize;
        let mut flattened_behavioral_count = 0usize;
        for element in &flattened.elements {
            if let Some(alias) = element.nodes.iter().find(|node| {
                Self::xyce_ground_alias_name(node)
                    && !Self::param_expression_name_is_literal_ground(node)
            }) {
                return Err(format!(
                    "flattened element '{}' uses unqualified ground alias '{}'",
                    element.name, alias
                ));
            }
            let nodes = element
                .nodes
                .iter()
                .map(|node| Self::canonical_param_expression_node_name(node))
                .collect::<Vec<_>>();
            let fingerprint = match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } if nodes.len() == 2
                    && nodes[1] == "0"
                    && value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
                    && matches!(nodes[0].as_str(), node if node == signal_nodes[0] || node == signal_nodes[1])
                    && flattened_resistor_nodes.insert(nodes[0].clone()) =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "R".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if nodes == [signal_nodes[2].clone(), "0".to_string()]
                        && value.is_finite()
                        && *value != 0.0
                        && flattened_voltage_count == 0 =>
                {
                    flattened_voltage_count += 1;
                    XyceRelationalElementFingerprint {
                        kind: "V:DC".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::BehavioralVoltage {
                    expression,
                    tc1,
                    tc2,
                } if nodes == [signal_nodes[0].clone(), "0".to_string()]
                    && tc1.to_bits() == 0.0f64.to_bits()
                    && tc2.to_bits() == 0.0f64.to_bits()
                    && flattened_behavioral_count == 0 =>
                {
                    Self::qualify_prepared_param_expression(
                        expression,
                        &netlist.params,
                        *parameter_value,
                        &signal_nodes[1],
                        "0",
                        &signal_nodes[2],
                        "0",
                    )?;
                    flattened_behavioral_count += 1;
                    XyceRelationalElementFingerprint {
                        kind: "BV".to_string(),
                        nodes,
                        numeric_bits: vec![tc1.to_bits(), tc2.to_bits(), parameter_value.to_bits()],
                        text: vec![
                            signal_nodes[1].clone(),
                            "0".to_string(),
                            signal_nodes[2].clone(),
                            "0".to_string(),
                        ],
                    }
                }
                _ => {
                    return Err(format!(
                        "flattened element '{}' is outside the qualified parameter-expression topology",
                        element.name
                    ));
                }
            };
            let key = Self::normalize_device_instance_name(&element.name);
            if flattened_elements
                .insert(key.clone(), fingerprint)
                .is_some()
            {
                return Err(format!(
                    "flattened {LABEL} member contains duplicate element name '{key}'"
                ));
            }
        }
        if flattened_resistor_nodes
            != [signal_nodes[0].clone(), signal_nodes[1].clone()]
                .into_iter()
                .collect()
            || flattened_voltage_count != 1
            || flattened_behavioral_count != 1
        {
            return Err(format!(
                "flattened {LABEL} member does not contain the exact two-resistor/one-DC-source/one-behavioral-source topology"
            ));
        }

        Ok(XyceParamExpressionFamilySnapshot {
            title: netlist.title.trim().to_string(),
            parameter_name: parameter_name.to_ascii_lowercase(),
            parameter_bits: parameter_value.to_bits(),
            subcircuit_name: subcircuit.name.trim().to_ascii_lowercase(),
            subcircuit_ports,
            flattened_elements,
            representation,
        })
    }

    pub(super) fn scoped_model_family_snapshot(
        contract: &XyceBaselineFamilyContract,
        netlist: &Netlist,
    ) -> Result<Option<XyceScopedModelFamilySnapshot>, String> {
        if contract.kind != XyceBaselineFamilyKind::ScopedModel {
            return Ok(None);
        }

        let flattened = rspice_core::netlist::flatten_netlist_with_models(netlist)
            .map_err(|err| format!("could not flatten scoped-model family member: {err}"))?;

        let mut elements = BTreeMap::new();
        let mut bjt_model_bits = BTreeMap::new();
        let mut diode_model_bits = BTreeMap::new();
        for element in &flattened.elements {
            let key = Self::normalize_device_instance_name(&element.name);
            let fingerprint = Self::scoped_model_element_fingerprint(element, &netlist.params)?;
            if elements.insert(key.clone(), fingerprint).is_some() {
                return Err(format!(
                    "flattened scoped-model family contains duplicate element name '{key}'"
                ));
            }

            match &element.kind {
                ElementKind::Bjt { model, .. } => {
                    let effective_model = Self::find_unique_model_in(
                        flattened
                            .scoped_models
                            .iter()
                            .chain(netlist.models.iter()),
                        model,
                    )
                    .ok_or_else(|| {
                        format!(
                            "scoped-model BJT '{key}' must reference exactly one effective model '{model}'"
                        )
                    })?;
                    if !Self::model_is_native_scoped_model_relational_bjt(effective_model) {
                        return Err(format!(
                            "scoped-model BJT '{key}' references an unresolved or unqualified effective model '{model}'"
                        ));
                    }
                    let bf = Self::numeric_param_value(&effective_model.params, "BF")
                        .expect("qualified scoped-model BJT has exactly one BF");
                    let saturation_current =
                        Self::numeric_param_value(&effective_model.params, "IS")
                            .expect("qualified scoped-model BJT has exactly one IS");
                    bjt_model_bits.insert(key, (bf.to_bits(), saturation_current.to_bits()));
                }
                ElementKind::Diode { model, .. } => {
                    let effective_model = Self::find_unique_model_in(
                        flattened
                            .scoped_models
                            .iter()
                            .chain(netlist.models.iter()),
                        model,
                    )
                    .ok_or_else(|| {
                        format!(
                            "scoped-model diode '{key}' must reference exactly one effective model '{model}'"
                        )
                    })?;
                    if !Self::model_is_native_exact_is_diode(effective_model) {
                        return Err(format!(
                            "scoped-model diode '{key}' references an unresolved or unqualified effective model '{model}'"
                        ));
                    }
                    let saturation_current =
                        Self::numeric_param_value(&effective_model.params, "IS")
                            .expect("qualified scoped-model diode has exactly one IS");
                    diode_model_bits.insert(key, saturation_current.to_bits());
                }
                _ => {}
            }
        }

        if bjt_model_bits.is_empty() && diode_model_bits.is_empty() {
            return Err(format!(
                "scoped-model family '{}' does not exercise a qualified nonlinear model",
                contract.family
            ));
        }

        Ok(Some(XyceScopedModelFamilySnapshot {
            elements,
            bjt_model_bits,
            diode_model_bits,
        }))
    }

    pub(super) fn bjt_external_node_family_snapshot(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<XyceBjtExternalNodeFamilySnapshot, String> {
        Self::validate_bjt_external_node_dc_probes(print, netlist)?;
        if !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(
                "exact BJT external-node DC contains auxiliary analysis, hierarchy, or external-model state"
                    .to_string(),
            );
        }
        if !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(
                "exact BJT external-node DC does not admit parameters or user functions"
                    .to_string(),
            );
        }
        if netlist.models.len() != 1 {
            return Err(format!(
                "exact BJT external-node DC requires exactly one model, found {}",
                netlist.models.len()
            ));
        }
        let effective_model = &netlist.models[0];
        if !Self::model_is_native_bjt_external_node_level1_npn(effective_model) {
            return Err(format!(
                "model '{}' is not the qualified implicit-Level-1 NPN with one finite positive BF",
                effective_model.name
            ));
        }

        let mut elements = BTreeMap::new();
        let mut bjt_model_bits = BTreeMap::new();
        let mut representation = None;
        for element in &netlist.elements {
            let key = Self::normalize_device_instance_name(&element.name);
            if let Some(alias) = element.nodes.iter().find(|node| {
                Self::xyce_ground_alias_name(node)
                    && !Self::bjt_external_node_name_is_literal_ground(node)
            }) {
                return Err(format!(
                    "element '{}' uses ground alias '{}'; exact Xyce parity requires literal node 0 without .PREPROCESS REPLACEGND",
                    element.name, alias
                ));
            }
            let mut nodes = element
                .nodes
                .iter()
                .map(|node| Self::canonical_bjt_external_node_name(node))
                .collect::<Vec<_>>();
            let (kind, numeric_bits, text) = match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    if element.nodes.len() != 2
                        || !value.is_finite()
                        || *value <= 0.0
                        || value_expr.is_some()
                        || model.is_some()
                        || !instance_params.is_empty()
                        || !deferred_params.is_empty()
                    {
                        return Err(format!(
                            "resistor '{}' is outside the finite numeric two-terminal envelope",
                            element.name
                        ));
                    }
                    ("R".to_string(), vec![value.to_bits()], Vec::new())
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) => {
                    if element.nodes.len() != 2 || !value.is_finite() {
                        return Err(format!(
                            "voltage source '{}' is outside the finite two-terminal DC envelope",
                            element.name
                        ));
                    }
                    ("V:DC".to_string(), vec![value.to_bits()], Vec::new())
                }
                ElementKind::Bjt {
                    model,
                    instance_params,
                    deferred_params,
                    ..
                } => {
                    if representation.is_some() {
                        return Err(
                            "exact BJT external-node DC requires exactly one BJT".to_string()
                        );
                    }
                    if !instance_params.is_empty() || !deferred_params.is_empty() {
                        return Err(format!(
                            "BJT '{}' has unqualified instance parameters",
                            element.name
                        ));
                    }
                    if !model.eq_ignore_ascii_case(&effective_model.name) {
                        return Err(format!(
                            "BJT '{}' does not reference the unique effective model '{}'",
                            element.name, effective_model.name
                        ));
                    }
                    if !element.nodes.get(2).is_some_and(|emitter| {
                        Self::bjt_external_node_name_is_literal_ground(emitter)
                    }) {
                        return Err(format!(
                            "BJT '{}' emitter must be ground in the qualified external-node envelope",
                            element.name
                        ));
                    }
                    let form = match element.nodes.as_slice() {
                        [_, _, _] => {
                            nodes.push("0".to_string());
                            XyceBjtExternalNodeRepresentation::OmittedGround
                        }
                        [_, _, _, substrate]
                            if Self::bjt_external_node_name_is_literal_ground(substrate) =>
                        {
                            XyceBjtExternalNodeRepresentation::ExplicitGround
                        }
                        [_, _, _, _] => {
                            return Err(format!(
                                "BJT '{}' explicit substrate must be ground",
                                element.name
                            ));
                        }
                        _ => {
                            return Err(format!(
                                "BJT '{}' must have exactly three or four terminals",
                                element.name
                            ));
                        }
                    };
                    representation = Some(form);
                    bjt_model_bits.insert(
                        effective_model.name.to_ascii_lowercase(),
                        effective_model.params[0].1.to_bits(),
                    );
                    (
                        "Q:NPN:L1".to_string(),
                        Vec::new(),
                        vec![model.to_ascii_lowercase()],
                    )
                }
                _ => {
                    return Err(format!(
                        "element '{}' is outside the resistor/DC-voltage-source/Level-1-NPN envelope",
                        element.name
                    ));
                }
            };
            let fingerprint = XyceRelationalElementFingerprint {
                kind,
                nodes,
                numeric_bits,
                text,
            };
            if elements.insert(key.clone(), fingerprint).is_some() {
                return Err(format!(
                    "exact BJT external-node DC contains duplicate element name '{key}'"
                ));
            }
        }
        let representation = representation
            .ok_or_else(|| "exact BJT external-node DC contains no qualified BJT".to_string())?;
        if bjt_model_bits.len() != 1 {
            return Err(
                "exact BJT external-node DC did not resolve exactly one BJT model".to_string(),
            );
        }
        Ok(XyceBjtExternalNodeFamilySnapshot {
            title: netlist.title.trim().to_string(),
            elements,
            bjt_model_bits,
            representation,
        })
    }

    pub(super) fn nested_include_identity_family_snapshot(
        netlist: &Netlist,
        plan: &XyceStaticDcPlan,
    ) -> Result<XyceNestedIncludeIdentityFamilySnapshot, String> {
        if !netlist.models.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || netlist.subcircuits.len() != 4
            || netlist.elements.len() != 3
        {
            return Err("nested-include identity requires a diagnostic-free two-parent/two-local-subcircuit R/V circuit without auxiliary state".to_string());
        }

        let mut definition_names = BTreeSet::new();
        for subcircuit in &netlist.subcircuits {
            if !definition_names.insert(subcircuit.name.to_ascii_lowercase()) {
                return Err(format!(
                    "duplicate fully-qualified subcircuit definition '{}'",
                    subcircuit.name
                ));
            }
        }
        let parents = netlist
            .subcircuits
            .iter()
            .filter(|subcircuit| !subcircuit.nested_subcircuits.is_empty())
            .collect::<Vec<_>>();
        let leaves = netlist
            .subcircuits
            .iter()
            .filter(|subcircuit| subcircuit.nested_subcircuits.is_empty())
            .collect::<Vec<_>>();
        if parents.len() != 2 || leaves.len() != 2 {
            return Err("nested-include identity requires exactly two parent and two qualified local definitions".to_string());
        }

        let mut hierarchy = Vec::new();
        let mut local_names = BTreeSet::new();
        for parent in &parents {
            Self::validate_nested_include_subcircuit_auxiliary_state(parent)?;
            let [nested] = parent.nested_subcircuits.as_slice() else {
                return Err(format!(
                    "parent subcircuit '{}' must own exactly one local definition",
                    parent.name
                ));
            };
            Self::validate_nested_include_subcircuit_auxiliary_state(nested)?;
            let expected_prefix = format!("{}.", parent.name);
            let local_name = nested.name.strip_prefix(&expected_prefix).ok_or_else(|| {
                format!(
                    "local definition '{}' is not qualified beneath parent '{}'",
                    nested.name, parent.name
                )
            })?;
            if local_name.is_empty() || local_name.contains('.') {
                return Err(format!(
                    "local definition '{}' has a non-local qualified suffix",
                    nested.name
                ));
            }
            local_names.insert(local_name.to_ascii_lowercase());
            if !leaves
                .iter()
                .any(|candidate| candidate.name.eq_ignore_ascii_case(&nested.name))
            {
                return Err(format!(
                    "parent '{}' local definition '{}' is absent from the executable definition table",
                    parent.name, nested.name
                ));
            }
            if parent.ports.len() != 2 || nested.ports.len() != 2 || nested.elements.len() != 1 {
                return Err("nested-include parent and local definitions must be two-terminal series-resistor blocks".to_string());
            }
            let nested_resistor = &nested.elements[0];
            Self::strict_nested_include_resistor_fingerprint(nested_resistor)?;
            if nested_resistor.nodes.len() != 2
                || !nested_resistor.nodes[0].eq_ignore_ascii_case(&nested.ports[0])
                || !nested_resistor.nodes[1].eq_ignore_ascii_case(&nested.ports[1])
            {
                return Err(format!(
                    "local definition '{}' must connect one direct resistor across its ordered ports",
                    nested.name
                ));
            }

            let mut child = None;
            let mut resistor = None;
            for element in &parent.elements {
                match &element.kind {
                    ElementKind::Subcircuit {
                        subckt_name,
                        params,
                    } if params.is_empty()
                        && subckt_name.eq_ignore_ascii_case(&nested.name)
                        && element.nodes.len() == 2 =>
                    {
                        if child.replace(element).is_some() {
                            return Err(format!(
                                "parent '{}' has multiple local subcircuit instances",
                                parent.name
                            ));
                        }
                    }
                    ElementKind::Resistor { .. } => {
                        Self::strict_nested_include_resistor_fingerprint(element)?;
                        if resistor.replace(element).is_some() {
                            return Err(format!(
                                "parent '{}' has multiple direct resistors",
                                parent.name
                            ));
                        }
                    }
                    _ => {
                        return Err(format!(
                            "parent '{}' contains unqualified element '{}'",
                            parent.name, element.name
                        ));
                    }
                }
            }
            let child = child.ok_or_else(|| {
                format!(
                    "parent '{}' does not bind its local definition",
                    parent.name
                )
            })?;
            let resistor = resistor
                .ok_or_else(|| format!("parent '{}' has no direct series resistor", parent.name))?;
            let internal = child
                .nodes
                .iter()
                .find(|node| {
                    !parent
                        .ports
                        .iter()
                        .any(|port| port.eq_ignore_ascii_case(node))
                })
                .ok_or_else(|| format!("parent '{}' has no private series node", parent.name))?;
            if child
                .nodes
                .iter()
                .filter(|node| node.eq_ignore_ascii_case(internal))
                .count()
                != 1
                || resistor
                    .nodes
                    .iter()
                    .filter(|node| node.eq_ignore_ascii_case(internal))
                    .count()
                    != 1
                || !parent.ports.iter().all(|port| {
                    child
                        .nodes
                        .iter()
                        .any(|node| node.eq_ignore_ascii_case(port))
                        || resistor
                            .nodes
                            .iter()
                            .any(|node| node.eq_ignore_ascii_case(port))
                })
            {
                return Err(format!(
                    "parent '{}' does not form a simple local-block/resistor series path",
                    parent.name
                ));
            }

            hierarchy.push(Self::nested_include_subcircuit_fingerprint(parent)?);
            hierarchy.push(Self::nested_include_subcircuit_fingerprint(nested)?);
        }
        if local_names.len() != 1 {
            return Err(
                "qualified local definitions do not share one lexical local name".to_string(),
            );
        }
        hierarchy.sort();

        let mut voltage_source = None;
        let mut top_instances = Vec::new();
        for element in &netlist.elements {
            match &element.kind {
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.is_finite() && element.nodes.len() == 2 =>
                {
                    if voltage_source.replace(element).is_some() {
                        return Err(
                            "nested-include identity contains multiple voltage sources".to_string()
                        );
                    }
                }
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } if params.is_empty()
                    && element.nodes.len() == 2
                    && parents
                        .iter()
                        .any(|parent| parent.name.eq_ignore_ascii_case(subckt_name)) =>
                {
                    top_instances.push(element);
                }
                _ => {
                    return Err(format!(
                        "nested-include top level contains unqualified element '{}'",
                        element.name
                    ));
                }
            }
        }
        let source = voltage_source.ok_or_else(|| {
            "nested-include identity requires one finite DC voltage source".to_string()
        })?;
        let instantiated_parents = top_instances
            .iter()
            .filter_map(|instance| match &instance.kind {
                ElementKind::Subcircuit { subckt_name, .. } => {
                    Some(subckt_name.to_ascii_lowercase())
                }
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        if top_instances.len() != 2
            || instantiated_parents.len() != 2
            || parents
                .iter()
                .any(|parent| !instantiated_parents.contains(&parent.name.to_ascii_lowercase()))
            || !source.name.eq_ignore_ascii_case(&plan.dc.source)
        {
            return Err("nested-include identity requires two parent instances swept by its sole voltage source".to_string());
        }
        let source_ground = source
            .nodes
            .iter()
            .position(|node| Self::node_name_is_ground(node))
            .ok_or_else(|| "nested-include source must have one grounded terminal".to_string())?;
        if source
            .nodes
            .iter()
            .filter(|node| Self::node_name_is_ground(node))
            .count()
            != 1
        {
            return Err(
                "nested-include source must have exactly one grounded terminal".to_string(),
            );
        }
        let driven = source.nodes[1 - source_ground].to_ascii_lowercase();
        let mut degree = BTreeMap::<String, usize>::new();
        for instance in &top_instances {
            for node in &instance.nodes {
                *degree.entry(node.to_ascii_lowercase()).or_default() += 1;
            }
        }
        let ground = degree
            .keys()
            .find(|node| Self::node_name_is_ground(node))
            .cloned()
            .ok_or_else(|| "parent instance chain is not grounded".to_string())?;
        let middle = degree
            .iter()
            .find_map(|(node, count)| (*count == 2).then_some(node.clone()))
            .ok_or_else(|| "parent instances do not form a two-block series chain".to_string())?;
        if degree.len() != 3
            || degree.get(&driven) != Some(&1)
            || degree.get(&ground) != Some(&1)
            || degree.get(&middle) != Some(&2)
        {
            return Err(
                "parent instances do not connect the driven node through one shared node to ground"
                    .to_string(),
            );
        }
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if probes
            != [
                format!("v({middle})"),
                format!("i({})", source.name.to_ascii_lowercase()),
            ]
        {
            return Err("nested-include identity requires ordered shared-node voltage and swept-source current probes".to_string());
        }

        let flattened = rspice_core::netlist::flatten_netlist_with_models(netlist)
            .map_err(|err| format!("nested-include hierarchy did not flatten: {err}"))?;
        if !flattened.scoped_models.is_empty()
            || !flattened.scoped_initial_conditions.is_empty()
            || !flattened.scoped_node_sets.is_empty()
            || flattened.elements.len() != 5
        {
            return Err(
                "nested-include flattening produced unexpected scoped or element state".to_string(),
            );
        }
        let mut flattened_elements = BTreeMap::new();
        for element in &flattened.elements {
            let fingerprint = match &element.kind {
                ElementKind::Resistor { .. } => {
                    Self::strict_nested_include_resistor_fingerprint(element)?
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.is_finite() && element.nodes.len() == 2 =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "V:DC".to_string(),
                        nodes: element
                            .nodes
                            .iter()
                            .map(|node| node.to_ascii_lowercase())
                            .collect(),
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                _ => {
                    return Err(format!(
                        "flattened nested-include circuit contains unqualified element '{}'",
                        element.name
                    ));
                }
            };
            if flattened_elements
                .insert(element.name.to_ascii_lowercase(), fingerprint)
                .is_some()
            {
                return Err(format!(
                    "flattened nested-include circuit has duplicate element '{}'",
                    element.name
                ));
            }
        }
        Ok(XyceNestedIncludeIdentityFamilySnapshot {
            title: netlist.title.trim().to_string(),
            hierarchy,
            flattened_elements,
        })
    }

    pub(super) fn nested_include_identity_provenance(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceNestedIncludeProvenance, String> {
        let parent_dir = deck_path
            .parent()
            .ok_or_else(|| "nested-include deck has no parent directory".to_string())?;
        let canonical_parent = parent_dir
            .canonicalize()
            .unwrap_or_else(|_| parent_dir.to_path_buf());
        let (canonical_source, occurrences) =
            Self::nested_include_identity_raw_provenance(source, parent_dir, &canonical_parent)?;
        let support_paths = occurrences
            .iter()
            .map(|(_, path)| path.clone())
            .collect::<BTreeSet<_>>();
        let representation = match support_paths.len() {
            1 => XyceNestedIncludeIdentityRepresentation::RepeatedCanonicalTarget,
            2 => XyceNestedIncludeIdentityRepresentation::SplitIdenticalTargets,
            _ => return Err("include target identity is ambiguous".to_string()),
        };
        let mut support_source = None::<String>;
        for path in &support_paths {
            let content = fs::read_to_string(path).map_err(|err| {
                format!("failed to read support source {}: {err}", path.display())
            })?;
            Self::validate_nested_include_support_source(&content)?;
            if support_source
                .as_ref()
                .is_some_and(|expected| expected != &content)
            {
                return Err("split include targets are not byte-identical".to_string());
            }
            support_source = Some(content);
        }
        let expanded_source = Netlist::preprocess_includes(source, deck_path)
            .map_err(|err| format!("include expansion failed: {err}"))?;
        Ok(XyceNestedIncludeProvenance {
            representation,
            canonical_source,
            expanded_source,
            support_paths,
        })
    }

    pub(super) fn nested_include_identity_raw_provenance(
        source: &str,
        parent_dir: &Path,
        canonical_parent: &Path,
    ) -> Result<(String, Vec<(String, PathBuf)>), String> {
        const INCLUDE_SENTINEL: &str = "<RSPICE_NESTED_INCLUDE_TARGET>";

        let mut open_parent = None::<String>;
        let mut parent_names = BTreeSet::new();
        let mut subckt_count = 0usize;
        let mut ends_count = 0usize;
        let mut dc_count = 0usize;
        let mut print_count = 0usize;
        let mut end_count = 0usize;
        let mut occurrences = Vec::<(String, PathBuf)>::new();
        let mut operand_spans = Vec::<(usize, usize)>::new();
        let mut source_offset = 0usize;

        for physical_line in source.split_inclusive('\n') {
            let line_start = source_offset;
            source_offset += physical_line.len();
            let line = physical_line.strip_suffix('\n').unwrap_or(physical_line);
            let line = line.strip_suffix('\r').unwrap_or(line);
            let uncommented = line.split_once(';').map_or(line, |(head, _)| head);
            let stripped = uncommented.trim();
            if stripped.is_empty() || stripped.starts_with('*') || stripped.starts_with("//") {
                continue;
            }
            let tokens = stripped.split_whitespace().collect::<Vec<_>>();
            let head = tokens.first().copied().unwrap_or("");
            if head.eq_ignore_ascii_case(".subckt") {
                subckt_count += 1;
                if open_parent.is_some() {
                    return Err("worker source contains a lexically nested .SUBCKT before include expansion".to_string());
                }
                let name = tokens
                    .get(1)
                    .ok_or_else(|| ".SUBCKT has no name".to_string())?
                    .to_ascii_lowercase();
                if !parent_names.insert(name.clone()) {
                    return Err(format!(
                        "duplicate top-level subcircuit definition '{name}'"
                    ));
                }
                open_parent = Some(name);
                continue;
            }
            if head.eq_ignore_ascii_case(".ends") {
                ends_count += 1;
                let owner = open_parent
                    .take()
                    .ok_or_else(|| ".ENDS appears outside a parent subcircuit".to_string())?;
                if tokens.len() > 2
                    || tokens
                        .get(1)
                        .is_some_and(|name| !name.eq_ignore_ascii_case(&owner))
                {
                    return Err(format!(
                        ".ENDS does not close its lexical parent subcircuit '{owner}'"
                    ));
                }
                continue;
            }
            if head.eq_ignore_ascii_case(".lib") || head.eq_ignore_ascii_case(".endl") {
                return Err("nested-include identity does not admit .LIB sections".to_string());
            }
            if matches!(
                head.to_ascii_lowercase().as_str(),
                ".include" | ".inc" | ".incl"
            ) {
                let owner = open_parent.clone().ok_or_else(|| {
                    "nested-include identity requires every include inside a parent subcircuit"
                        .to_string()
                })?;
                let (requested, operand_start, operand_end) =
                    Self::nested_include_identity_operand_span(line)?;
                let relative = rspice_core::netlist::source_path_literal_to_host_path(&requested);
                if requested.contains(['/', '\\'])
                    || requested == "."
                    || requested == ".."
                    || relative.is_absolute()
                    || relative.components().count() != 1
                {
                    return Err(
                        "nested-include identity requires one bare local include filename"
                            .to_string(),
                    );
                }
                let target = parent_dir.join(relative);
                let canonical = target.canonicalize().map_err(|err| {
                    format!("could not resolve include '{}': {err}", target.display())
                })?;
                if canonical.parent() != Some(canonical_parent) || !canonical.is_file() {
                    return Err(format!(
                        "include target '{}' escapes the family directory or is not a file",
                        canonical.display()
                    ));
                }
                let exact_directory_name = fs::read_dir(parent_dir)
                    .map_err(|err| {
                        format!(
                            "could not inspect include directory '{}': {err}",
                            parent_dir.display()
                        )
                    })?
                    .filter_map(Result::ok)
                    .any(|entry| {
                        entry.file_name().to_str() == Some(requested.as_str())
                            && entry.path().canonicalize().ok().as_ref() == Some(&canonical)
                    });
                if !exact_directory_name {
                    return Err(
                        "nested-include operand does not exactly spell its directory entry"
                            .to_string(),
                    );
                }
                occurrences.push((owner, canonical));
                operand_spans.push((line_start + operand_start, line_start + operand_end));
                continue;
            }
            if head.eq_ignore_ascii_case(".dc") {
                if open_parent.is_some() {
                    return Err("nested-include .DC analysis must be top-level".to_string());
                }
                dc_count += 1;
                continue;
            }
            if head.eq_ignore_ascii_case(".print") {
                if open_parent.is_some() {
                    return Err("nested-include .PRINT directive must be top-level".to_string());
                }
                print_count += 1;
                if tokens.len() != 4
                    || !tokens
                        .get(1)
                        .is_some_and(|analysis| analysis.eq_ignore_ascii_case("dc"))
                    || tokens.iter().skip(2).any(|token| {
                        let option = token.to_ascii_lowercase();
                        option == "file"
                            || option.starts_with("file=")
                            || option == "format"
                            || option.starts_with("format=")
                            || option == "noindex"
                            || option.starts_with("noindex=")
                    })
                {
                    return Err("nested-include identity requires default-PRN .PRINT DC without FILE, FORMAT, NOINDEX, or side-output options".to_string());
                }
                continue;
            }
            if head.eq_ignore_ascii_case(".end") {
                if open_parent.is_some() || tokens.len() != 1 || line.contains(';') {
                    return Err(
                        "nested-include identity requires one exact top-level .END card"
                            .to_string(),
                    );
                }
                end_count += 1;
                if source_offset != source.len() {
                    return Err(
                        "nested-include identity does not admit content after .END".to_string()
                    );
                }
            }
        }

        let include_owners = occurrences
            .iter()
            .map(|(owner, _)| owner.clone())
            .collect::<BTreeSet<_>>();
        if open_parent.is_some()
            || subckt_count != 2
            || ends_count != 2
            || occurrences.len() != 2
            || include_owners != parent_names
            || dc_count != 1
            || print_count != 1
            || end_count != 1
        {
            return Err("nested-include identity requires exactly two matched parent .SUBCKT/.ENDS pairs with one include each, one .DC, one default .PRINT DC, and one terminal .END".to_string());
        }

        let mut canonical_source = source.to_string();
        for (start, end) in operand_spans.into_iter().rev() {
            canonical_source.replace_range(start..end, INCLUDE_SENTINEL);
        }
        Ok((canonical_source, occurrences))
    }

    pub(super) fn numbered_redefinition_representation(
        netlist: &Netlist,
    ) -> Result<XyceNumberedRedefinitionRepresentation, String> {
        if !netlist.models.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || netlist.subcircuits.len() != 1
            || netlist.elements.len() != 3
        {
            return Err(
                "numbered redefinition member requires one three-element, one-subcircuit divider without auxiliary state"
                    .to_string(),
            );
        }
        let subcircuit = &netlist.subcircuits[0];
        let [body_element] = subcircuit.elements.as_slice() else {
            return Err("numbered redefinition subcircuit must contain one element".to_string());
        };
        let ElementKind::Resistor { value_expr, .. } = &body_element.kind else {
            return Err("numbered redefinition subcircuit element must be a resistor".to_string());
        };
        let instances = netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } => Some((subckt_name, params)),
                _ => None,
            })
            .collect::<Vec<_>>();
        let [(subckt_name, instance_params)] = instances.as_slice() else {
            return Err(
                "numbered redefinition member must contain one subcircuit instance".to_string(),
            );
        };
        if !subckt_name.eq_ignore_ascii_case(&subcircuit.name)
            || netlist
                .elements
                .iter()
                .filter(|element| matches!(element.kind, ElementKind::VoltageSource(_)))
                .count()
                != 1
            || netlist
                .elements
                .iter()
                .filter(|element| matches!(element.kind, ElementKind::Resistor { .. }))
                .count()
                != 1
        {
            return Err(
                "numbered redefinition member must contain one source, one series resistor, and one matching subcircuit instance"
                    .to_string(),
            );
        }

        let Some(load_parameter) = value_expr.as_deref() else {
            if subcircuit.params.is_empty()
                && subcircuit.expr_params.is_empty()
                && subcircuit.string_params.is_empty()
                && instance_params.is_empty()
            {
                return Ok(XyceNumberedRedefinitionRepresentation::LiteralBaseline);
            }
            return Err(
                "literal baseline must not carry formal or instance parameters".to_string(),
            );
        };
        if !Self::is_single_spice_identifier(load_parameter) {
            return Err("parameterized subcircuit load must reference one identifier".to_string());
        }
        let formal_expression = subcircuit
            .expr_params
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(load_parameter));
        let instance_expression = instance_params
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(load_parameter));
        match (formal_expression, instance_expression) {
            (true, false) => Ok(
                XyceNumberedRedefinitionRepresentation::DependentFormalExpression,
            ),
            (false, true) => Ok(
                XyceNumberedRedefinitionRepresentation::DependentInstanceExpression,
            ),
            _ => Err(
                "load parameter must be supplied by exactly one dependent formal or instance expression"
                    .to_string(),
            ),
        }
    }

    pub(super) fn numbered_redefinition_snapshot(
        netlist: &Netlist,
        sweep_source: &str,
    ) -> Result<Vec<XyceRelationalElementFingerprint>, String> {
        let flattened = rspice_core::netlist::flatten_netlist_with_models(netlist)
            .map_err(|err| format!("could not flatten numbered member: {err}"))?;
        if flattened.elements.len() != 3 {
            return Err(format!(
                "numbered member must flatten to three elements, found {}",
                flattened.elements.len()
            ));
        }
        let voltage_sources = flattened
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::VoltageSource(_)))
            .collect::<Vec<_>>();
        if voltage_sources.len() != 1
            || !voltage_sources[0].name.eq_ignore_ascii_case(sweep_source)
            || flattened
                .elements
                .iter()
                .filter(|element| matches!(element.kind, ElementKind::Resistor { .. }))
                .count()
                != 2
        {
            return Err(
                "numbered member must flatten to the swept source and two resistors".to_string(),
            );
        }
        let mut fingerprints = flattened
            .elements
            .iter()
            .map(|element| Self::scoped_model_element_fingerprint(element, &netlist.params))
            .collect::<Result<Vec<_>, _>>()?;
        fingerprints.sort();
        Ok(fingerprints)
    }

    pub(super) fn shared_stepped_dc_representation(
        netlist: &Netlist,
        plan: &XyceStaticDcPlan,
        expected_transform: Option<&str>,
    ) -> Result<(XyceSharedSteppedDcRepresentation, Option<String>), String> {
        let [step] = plan.steps.as_slice() else {
            return Err("semantic family member requires exactly one .STEP command".to_string());
        };
        if step.target != StepTarget::Param || step.name.trim().is_empty() {
            return Err("semantic family .STEP must target one named global parameter".to_string());
        }
        let globals = netlist.params.numeric_parameters();
        if globals.len() != 1
            || !globals[0].0.eq_ignore_ascii_case(&step.name)
            || !globals[0].1.is_finite()
            || !Self::source_has_sole_global_parameter_definition(&plan.source, &step.name)
            || !netlist.models.is_empty()
            || netlist.elements.len() != 2
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "semantic member must contain one true finite .GLOBAL_PARAM stepped definition and no auxiliary state outside its two-element DC harness (numeric={}, models={}, elements={}, data={}, ic={}, nodeset={}, measure={}, veriloga={}, spef={})",
                globals.len(),
                netlist.models.len(),
                netlist.elements.len(),
                netlist.data_tables.len(),
                netlist.initial_conditions.len(),
                netlist.node_sets.len(),
                netlist.measurements.len(),
                netlist.veriloga_includes.len(),
                netlist.spef_includes.len()
            ));
        }
        let top_resistor = netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor {
                    value_expr,
                    model: None,
                    instance_params,
                    deferred_params,
                    ..
                } if instance_params.is_empty() && deferred_params.is_empty() => {
                    Some((element, value_expr.as_deref()))
                }
                _ => None,
            });
        let top_instance = netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } => Some((subckt_name, params)),
                _ => None,
            });
        if netlist
            .elements
            .iter()
            .filter(|element| {
                matches!(
                    element.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(_))
                )
            })
            .count()
            != 1
        {
            return Err(
                "semantic member requires exactly one independent DC voltage source".to_string(),
            );
        }
        let functions = netlist.params.all_functions();
        if let Some((resistor, expression)) = top_resistor {
            if top_instance.is_some() || !netlist.subcircuits.is_empty() || !functions.is_empty() {
                return Err(
                    "direct semantic baseline may not contain hierarchy or functions".to_string(),
                );
            }
            if Self::source_has_bare_resistor_parameter_value(
                &plan.source,
                &resistor.name,
                &step.name,
            ) {
                return Ok((XyceSharedSteppedDcRepresentation::DirectIdentity, None));
            }
            let expression = expression.ok_or_else(|| {
                "direct transform baseline must retain a parameter expression".to_string()
            })?;
            let transform = Self::alpha_normalize_expression(expression, &[(&step.name, "$x")]);
            if transform == "$x" || transform.is_empty() {
                return Err("direct transform baseline must be non-identity".to_string());
            }
            return Ok((
                XyceSharedSteppedDcRepresentation::DirectTransform,
                Some(transform),
            ));
        }
        let (subckt_name, instance_params) = top_instance.ok_or_else(|| {
            "semantic member requires one resistor or subcircuit instance".to_string()
        })?;
        let [subckt] = netlist.subcircuits.as_slice() else {
            return Err("hierarchical semantic member requires exactly one subcircuit".to_string());
        };
        if !subckt.name.eq_ignore_ascii_case(subckt_name)
            || subckt.ports.len() != 2
            || subckt.elements.len() != 1
            || subckt.params.len() != 1
            || !subckt.params[0].1.is_finite()
            || !subckt.expr_params.is_empty()
            || !subckt.string_params.is_empty()
            || !subckt.body_params.is_empty()
            || !subckt.body_expr_params.is_empty()
            || !subckt.body_string_params.is_empty()
            || !subckt.body_functions.is_empty()
            || !subckt.local_options.is_empty()
            || !subckt.initial_conditions.is_empty()
            || !subckt.node_sets.is_empty()
            || !subckt.nested_subcircuits.is_empty()
        {
            return Err(
                "hierarchical semantic member has an unqualified subcircuit signature".to_string(),
            );
        }
        let formal_name = &subckt.params[0].0;
        let [(binding_name, ParametricValue::Expression(binding))] = instance_params.as_slice()
        else {
            return Err(
                "subcircuit instance requires one deferred formal-parameter binding".to_string(),
            );
        };
        if !binding_name.eq_ignore_ascii_case(formal_name) {
            return Err(
                "subcircuit instance binding must name its sole formal parameter".to_string(),
            );
        }
        let resistor_expression = match &subckt.elements[0].kind {
            ElementKind::Resistor {
                value_expr: Some(expression),
                model: None,
                instance_params,
                deferred_params,
                ..
            } if instance_params.is_empty() && deferred_params.is_empty() => expression.as_str(),
            _ => return Err("subcircuit must contain one expression-valued resistor".to_string()),
        };
        let binding_to_global = Self::alpha_normalize_expression(binding, &[(&step.name, "$g")]);
        let resistor_to_formal =
            Self::alpha_normalize_expression(resistor_expression, &[(formal_name, "$f")]);
        if functions.is_empty() {
            if expected_transform.is_none()
                && binding_to_global == "$g"
                && resistor_to_formal == "$f"
            {
                return Ok((
                    XyceSharedSteppedDcRepresentation::HierarchicalIdentity,
                    None,
                ));
            }
            if let Some(transform) = expected_transform {
                let body =
                    Self::alpha_normalize_expression(resistor_expression, &[(formal_name, "$x")]);
                if binding_to_global == "$g" && body == transform {
                    return Ok((
                        XyceSharedSteppedDcRepresentation::TransformInSubcircuitBody,
                        None,
                    ));
                }
            }
            return Err("unrecognized hierarchy-only transform representation".to_string());
        }
        let [function] = functions.as_slice() else {
            return Err(
                "function representation requires exactly one user-defined function".to_string(),
            );
        };
        let [argument] = function.args.as_slice() else {
            return Err("family transform function requires exactly one argument".to_string());
        };
        let transform = expected_transform.ok_or_else(|| {
            "identity family may not introduce a user-defined transform".to_string()
        })?;
        if Self::alpha_normalize_expression(&function.body, &[(argument, "$x")]) != transform {
            return Err(
                "function body is not alpha-equivalent to the direct transform".to_string(),
            );
        }
        let resistor_call = Self::alpha_normalize_expression(
            resistor_expression,
            &[(formal_name, "$f"), (&function.name, "$fn")],
        );
        let binding_call = Self::alpha_normalize_expression(
            binding,
            &[(&step.name, "$g"), (&function.name, "$fn")],
        );
        match (binding_call.as_str(), resistor_call.as_str()) {
            ("$g", "$fn($f)") => Ok((
                XyceSharedSteppedDcRepresentation::FunctionCallInSubcircuitBody,
                None,
            )),
            ("$fn($g)", "$f") => Ok((
                XyceSharedSteppedDcRepresentation::FunctionCallAtInstance,
                None,
            )),
            _ => Err("unrecognized function placement in hierarchical transform".to_string()),
        }
    }

    pub(super) fn bug754_global_parameter_snapshot(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
    ) -> Result<XyceBug754GlobalParameterSnapshot, String> {
        let [model] = netlist.models.as_slice() else {
            return Err("BUG 754 requires exactly one MOS model".into());
        };
        if !model.name.eq_ignore_ascii_case("mlev1")
            || !model.model_type.eq_ignore_ascii_case("nmos")
            || model.params.len() != 1
            || !model.params[0].0.eq_ignore_ascii_case("level")
            || model.params[0].1.to_bits() != 1.0f64.to_bits()
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("BUG 754 Level-1 NMOS model changed: {model:?}"));
        }

        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            let name = element.name.to_ascii_lowercase();
            let nodes = element
                .nodes
                .iter()
                .map(|node| node.to_ascii_lowercase())
                .collect::<Vec<_>>();
            let fingerprint = match &element.kind {
                ElementKind::Mosfet {
                    model,
                    mos_type: rspice_core::netlist::MosType::Nmos,
                    compact_syntax: false,
                    instance_params,
                    deferred_params,
                } if model.eq_ignore_ascii_case("mlev1")
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
                    && nodes == ["drain", "gate", "source", "0"] =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "M:NMOS:L1".to_string(),
                        nodes,
                        numeric_bits: Vec::new(),
                        text: vec!["mlev1".to_string()],
                    }
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.is_finite() =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "V:DC".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                _ => {
                    return Err(format!(
                        "BUG 754 element '{}' is outside the exact Level-1 NMOS/DC-source envelope",
                        element.name
                    ));
                }
            };
            if elements.insert(name.clone(), fingerprint).is_some() {
                return Err(format!("BUG 754 contains duplicate element '{name}'"));
            }
        }
        let expected_elements = BTreeMap::from([
            (
                "m1".to_string(),
                XyceRelationalElementFingerprint {
                    kind: "M:NMOS:L1".to_string(),
                    nodes: vec![
                        "drain".to_string(),
                        "gate".to_string(),
                        "source".to_string(),
                        "0".to_string(),
                    ],
                    numeric_bits: Vec::new(),
                    text: vec!["mlev1".to_string()],
                },
            ),
            (
                "vdrain".to_string(),
                XyceRelationalElementFingerprint {
                    kind: "V:DC".to_string(),
                    nodes: vec!["drain".to_string(), "0".to_string()],
                    numeric_bits: vec![1.0f64.to_bits()],
                    text: Vec::new(),
                },
            ),
            (
                "vgate".to_string(),
                XyceRelationalElementFingerprint {
                    kind: "V:DC".to_string(),
                    nodes: vec!["gate".to_string(), "0".to_string()],
                    numeric_bits: vec![0.5f64.to_bits()],
                    text: Vec::new(),
                },
            ),
            (
                "vsource".to_string(),
                XyceRelationalElementFingerprint {
                    kind: "V:DC".to_string(),
                    nodes: vec!["source".to_string(), "0".to_string()],
                    numeric_bits: vec![0.0f64.to_bits()],
                    text: Vec::new(),
                },
            ),
        ]);
        if elements != expected_elements {
            return Err(format!(
                "BUG 754 canonical Level-1 NMOS/source topology changed: {elements:?}"
            ));
        }
        Ok(XyceBug754GlobalParameterSnapshot {
            elements,
            model_name: model.name.to_ascii_lowercase(),
            model_type: model.model_type.to_ascii_lowercase(),
            model_params: vec![("level".to_string(), 1.0f64.to_bits())],
            dc_source: plan.dc.source.to_ascii_lowercase(),
            dc_start_bits: plan.dc.start.to_bits(),
            dc_stop_bits: plan.dc.stop.to_bits(),
            dc_step_bits: plan.dc.step.to_bits(),
            probes: plan
                .print
                .probes
                .iter()
                .map(|probe| Self::normalize_probe(probe))
                .collect(),
        })
    }

    pub(super) fn bug655_expected_continuation_snapshot() -> XyceBug655ContinuationSnapshot {
        let elements = BTreeMap::from([
            (
                "i1".to_string(),
                XyceRelationalElementFingerprint {
                    kind: "I:DC".to_string(),
                    nodes: vec!["2".to_string(), "0".to_string()],
                    numeric_bits: vec![0.0f64.to_bits()],
                    text: Vec::new(),
                },
            ),
            (
                "q1".to_string(),
                XyceRelationalElementFingerprint {
                    kind: "Q:NPN:L1".to_string(),
                    nodes: vec!["3".to_string(), "2".to_string(), "0".to_string()],
                    numeric_bits: Vec::new(),
                    text: vec!["2n3510".to_string()],
                },
            ),
            (
                "r1".to_string(),
                XyceRelationalElementFingerprint {
                    kind: "R".to_string(),
                    nodes: vec!["1".to_string(), "3".to_string()],
                    numeric_bits: vec![5_000.0f64.to_bits()],
                    text: Vec::new(),
                },
            ),
            (
                "r2".to_string(),
                XyceRelationalElementFingerprint {
                    kind: "R".to_string(),
                    nodes: vec!["2".to_string(), "3".to_string()],
                    numeric_bits: vec![20_000.0f64.to_bits()],
                    text: Vec::new(),
                },
            ),
            (
                "vcc".to_string(),
                XyceRelationalElementFingerprint {
                    kind: "V:DC".to_string(),
                    nodes: vec!["1".to_string(), "0".to_string()],
                    numeric_bits: vec![12.0f64.to_bits()],
                    text: Vec::new(),
                },
            ),
        ]);
        let mut model_params: Vec<(String, u64)> = [
            ("bf", 100.0),
            ("br", 1.35e-4),
            ("xtb", 1.5),
            ("is", 8.35e-14),
            ("eg", 1.11),
            ("cjc", 9.63e-12),
            ("cje", 9.47e-12),
            ("rb", 16.7),
            ("rc", 1.66),
            ("vaf", 90.0),
            ("tf", 1e-10),
            ("tr", 1.27e-4),
            ("cjs", 1e-15),
            ("vjs", 0.8),
            ("mjs", 0.5),
            ("var", 100.0),
            ("ise", 4.77e-11),
            ("isc", 1e-16),
            ("ikf", 0.18),
            ("ikr", 1000.0),
            ("irb", 1.0),
            ("rbm", 0.0),
            ("vtf", 1000.0),
        ]
        .into_iter()
        .map(|(name, value): (&str, Value)| (name.to_string(), value.to_bits()))
        .collect::<Vec<_>>();
        model_params.sort();
        XyceBug655ContinuationSnapshot {
            title: "*** Simple amplifier ***".to_string(),
            elements,
            model_name: "2n3510".to_string(),
            model_type: "npn".to_string(),
            model_params,
        }
    }

    pub(super) fn bug655_continuation_snapshot(
        netlist: &Netlist,
    ) -> Result<XyceBug655ContinuationSnapshot, String> {
        let model = netlist
            .models
            .first()
            .ok_or_else(|| "BUG 655 member has no NPN model".to_string())?;
        if !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
            || model.params.len() != 23
            || model.params.iter().any(|(_, value)| !value.is_finite())
        {
            return Err(
                "BUG 655 model must contain exactly 23 finite scalar Level-1 NPN parameters".into(),
            );
        }
        let mut model_params = model
            .params
            .iter()
            .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
            .collect::<Vec<_>>();
        model_params.sort();

        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            let name = element.name.to_ascii_lowercase();
            let nodes = element
                .nodes
                .iter()
                .map(|node| node.to_ascii_lowercase())
                .collect::<Vec<_>>();
            let fingerprint = match &element.kind {
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.is_finite() =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "V:DC".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::CurrentSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.is_finite() =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "I:DC".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if value.is_finite()
                    && *value > 0.0
                    && instance_params.is_empty()
                    && deferred_params.is_empty() =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "R".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    }
                }
                ElementKind::Bjt {
                    model,
                    instance_params,
                    deferred_params,
                    ..
                } if instance_params.is_empty()
                    && deferred_params.is_empty()
                    && model.eq_ignore_ascii_case(&netlist.models[0].name) =>
                {
                    XyceRelationalElementFingerprint {
                        kind: "Q:NPN:L1".to_string(),
                        nodes,
                        numeric_bits: Vec::new(),
                        text: vec![model.to_ascii_lowercase()],
                    }
                }
                _ => {
                    return Err(format!(
                        "BUG 655 element '{}' is outside the exact V/I/R/Level-1-NPN envelope",
                        element.name
                    ));
                }
            };
            if elements.insert(name.clone(), fingerprint).is_some() {
                return Err(format!("BUG 655 contains duplicate element name '{name}'"));
            }
        }
        Ok(XyceBug655ContinuationSnapshot {
            title: netlist.title.trim().to_string(),
            elements,
            model_name: model.name.to_ascii_lowercase(),
            model_type: model.model_type.to_ascii_lowercase(),
            model_params,
        })
    }

    pub(super) fn resistor_dtemp_snapshot(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        role: XyceResistorDtempRole,
    ) -> Result<XyceResistorDtempSnapshot, String> {
        Self::validate_resistor_dtemp_statement_envelope(&plan.source, role)?;
        if plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || !plan.diagnostics.is_empty()
            || plan.steps.len() != 1
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, rspice_core::netlist::DcSweepMode::Linear)
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || plan.print.probes.len() != 2
        {
            return Err("resistor DTEMP pair requires one diagnostic-free three-member STEP, one 0:1:5 linear DC sweep, and one default two-probe PRN output".to_string());
        }
        let dc_count = netlist
            .analyses
            .iter()
            .filter(|analysis| matches!(analysis, AnalysisCommand::Dc { .. }))
            .count();
        let step_count = netlist
            .analyses
            .iter()
            .filter(|analysis| matches!(analysis, AnalysisCommand::Step(_)))
            .count();
        if netlist.analyses.len() != 2
            || dc_count != 1
            || step_count != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 2
            || netlist.models.len() != 1
        {
            return Err("resistor DTEMP pair requires only one resistor, one independent source, one resistor model, one DC analysis, and one STEP analysis".to_string());
        }

        let step_values = match &plan.steps[0].sweep {
            StepSweep::List(values) if values.len() == 3 => values,
            _ => {
                return Err(
                    "resistor DTEMP pair requires an exact three-value LIST sweep".to_string(),
                );
            }
        };
        let canonical_grid = [-55.0f64, 25.0, 72.0];
        let effective_temperatures = match role {
            XyceResistorDtempRole::Owner => {
                if plan.steps[0].target != StepTarget::Param
                    || !plan.steps[0].name.eq_ignore_ascii_case("resDtemp")
                    || plan.steps[0].param_name.is_some()
                    || netlist.options.temp.map(Value::to_bits) != Some(27.0f64.to_bits())
                    || step_values
                        .iter()
                        .map(|value| value.to_bits())
                        .ne([-82.0f64, -2.0, 45.0].into_iter().map(Value::to_bits))
                {
                    return Err("DTEMP owner must fix device TEMP=27 C and STEP resDtemp through -82, -2, 45 C".to_string());
                }
                step_values
                    .iter()
                    .map(|dtemp| 27.0 + dtemp)
                    .collect::<Vec<_>>()
            }
            XyceResistorDtempRole::Reference => {
                if plan.steps[0].target != StepTarget::Temp
                    || !plan.steps[0].name.eq_ignore_ascii_case("TEMP")
                    || plan.steps[0].param_name.is_some()
                    || netlist.options.temp.is_some()
                    || step_values
                        .iter()
                        .map(|value| value.to_bits())
                        .ne(canonical_grid.into_iter().map(Value::to_bits))
                {
                    return Err("reference must STEP global TEMP through -55, 25, 72 C without a fixed TEMP option".to_string());
                }
                step_values.clone()
            }
        };
        if effective_temperatures
            .iter()
            .map(|value| value.to_bits())
            .ne(canonical_grid.into_iter().map(Value::to_bits))
        {
            return Err(
                "TEMP and TEMP+DTEMP effective-temperature grids are not identical".to_string(),
            );
        }

        let explicit_params = netlist
            .params
            .all_params()
            .into_iter()
            .filter(|(name, _)| {
                !matches!(name.to_ascii_uppercase().as_str(), "TEMP" | "TEMPER" | "VT")
            })
            .collect::<Vec<_>>();
        match role {
            XyceResistorDtempRole::Owner
                if explicit_params.len() == 1
                    && explicit_params[0].0.eq_ignore_ascii_case("resDtemp")
                    && explicit_params[0].1.to_bits() == (-82.0f64).to_bits() => {}
            XyceResistorDtempRole::Reference if explicit_params.is_empty() => {}
            _ => {
                return Err(format!(
                    "only the owner's resDtemp=-82 parameter is admitted in the pair, got {explicit_params:?}"
                ));
            }
        }

        let resistor = netlist
            .elements
            .iter()
            .find(|element| matches!(element.kind, ElementKind::Resistor { .. }))
            .ok_or_else(|| "resistor DTEMP pair has no resistor".to_string())?;
        let source = netlist
            .elements
            .iter()
            .find(|element| matches!(element.kind, ElementKind::VoltageSource(_)))
            .ok_or_else(|| "resistor DTEMP pair has no independent voltage source".to_string())?;
        let (resistance, model_name, instance_params) = match &resistor.kind {
            ElementKind::Resistor {
                value,
                value_expr: None,
                model: Some(model),
                instance_params,
                deferred_params,
            } if value.to_bits() == 1000.0f64.to_bits() && deferred_params.is_empty() => {
                (*value, model, instance_params)
            }
            _ => {
                return Err(
                    "resistor must retain the exact 1 kOhm modeled representation".to_string(),
                );
            }
        };
        match role {
            XyceResistorDtempRole::Owner
                if instance_params.len() == 1
                    && instance_params[0].0.eq_ignore_ascii_case("DTEMP")
                    && instance_params[0].1.to_bits() == (-82.0f64).to_bits() => {}
            XyceResistorDtempRole::Reference if instance_params.is_empty() => {}
            _ => return Err("only the owner resistor may carry DTEMP=resDtemp".to_string()),
        }
        let source_value = match &source.kind {
            ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                if value.to_bits() == 5.0f64.to_bits() =>
            {
                *value
            }
            _ => return Err("resistor DTEMP pair requires one finite 5 V DC source".to_string()),
        };
        let canonical_nodes = |nodes: &[String]| -> Result<[String; 2], String> {
            let normalized = nodes
                .iter()
                .map(|node| Self::canonical_param_expression_node_name(node))
                .collect::<Vec<_>>();
            normalized
                .try_into()
                .map_err(|_| "resistor DTEMP elements must be two-terminal".to_string())
        };
        let resistor_nodes = canonical_nodes(&resistor.nodes)?;
        let source_nodes = canonical_nodes(&source.nodes)?;
        if resistor_nodes != ["1".to_string(), "0".to_string()]
            || source_nodes != resistor_nodes
            || !plan.dc.source.eq_ignore_ascii_case(&source.name)
        {
            return Err("resistor/source topology or swept-source mapping changed".to_string());
        }

        let model = &netlist.models[0];
        if !model.name.eq_ignore_ascii_case(model_name)
            || !matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "R" | "RES" | "RESISTOR"
            )
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(
                "resistor DTEMP pair requires one native scalar resistor model".to_string(),
            );
        }
        let mut model_params = model
            .params
            .iter()
            .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
            .collect::<Vec<_>>();
        model_params.sort();
        let expected_model_params = vec![
            ("tc1".to_string(), 0.0007325f64.to_bits()),
            ("tc2".to_string(), (-2.217e-7f64).to_bits()),
        ];
        if model_params != expected_model_params {
            return Err(
                "resistor model must retain exactly TC1=0.0007325 and TC2=-2.217e-7".to_string(),
            );
        }
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if probes != ["v(1)".to_string(), "i(v1)".to_string()] {
            return Err("resistor DTEMP pair requires ordered V(1), I(V1) probes".to_string());
        }

        Ok(XyceResistorDtempSnapshot {
            resistor_name: resistor.name.to_ascii_lowercase(),
            resistor_nodes,
            resistance_bits: resistance.to_bits(),
            model_name: model_name.to_ascii_lowercase(),
            source_name: source.name.to_ascii_lowercase(),
            source_nodes,
            source_value_bits: source_value.to_bits(),
            model_type: model.model_type.to_ascii_lowercase(),
            model_params,
            dc_source: plan.dc.source.to_ascii_lowercase(),
            dc_start_bits: plan.dc.start.to_bits(),
            dc_stop_bits: plan.dc.stop.to_bits(),
            dc_step_bits: plan.dc.step.to_bits(),
            probes,
            effective_temperature_bits: effective_temperatures
                .into_iter()
                .map(Value::to_bits)
                .collect(),
        })
    }
}
