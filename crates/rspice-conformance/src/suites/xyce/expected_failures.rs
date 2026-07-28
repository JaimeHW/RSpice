//! Decks that must fail, and the failure they must produce.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn observe_startup_conflict(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let error = match Self::parse_xyce_netlist(source, deck_path) {
            Ok(_) => {
                return Err(
                    "IC/NODESET conflict unexpectedly parsed successfully in Xyce mode".to_string(),
                );
            }
            Err(error) => error,
        };
        let ParseError::StartupDirectiveConflict(conflict) = error else {
            return Err(format!(
                "IC/NODESET conflict produced the wrong typed failure (conflict must precede undefined-node validation): {error}"
            ));
        };
        if conflict.first_kind != StartupDirectiveKind::Ic
            || conflict.first.line != 16
            || conflict.conflicting_kind != StartupDirectiveKind::NodeSet
            || conflict.conflicting.line != 17
        {
            return Err(format!(
                "IC/NODESET conflict retained the wrong ordered physical origins: {conflict:?}"
            ));
        }
        for origin in [&conflict.first, &conflict.conflicting] {
            let path = origin
                .path
                .as_ref()
                .ok_or_else(|| "IC/NODESET conflict lost a physical source path".to_string())?;
            if path.canonicalize().ok() != deck_path.canonicalize().ok() {
                return Err(format!(
                    "IC/NODESET conflict origin resolved to {}, not {}",
                    path.display(),
                    deck_path.display()
                ));
            }
        }
        let record = XYCE_IC_NODESET_CONFLICT_RECORD;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::ConflictingStartupDirectives,
            identifiers: vec![format!(".IC|{record}:16"), format!(".NODESET|{record}:17")],
        })
    }

    pub(super) fn validate_expected_failure_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceExpectedFailureKind,
    ) -> Result<(), String> {
        let upstream_policy = kind.upstream_error_policy();
        if !upstream_policy.requires_nonzero_exit
            || upstream_policy.search_streams
                != XyceUpstreamErrorSearchStreams::EitherCompleteStdoutOrStderr
            || upstream_policy.ordered_patterns.is_empty()
        {
            return Err(format!(
                "expected-failure record '{}' has an incomplete removed-wrapper error policy",
                kind.record()
            ));
        }
        self.validate_expected_failure_provenance(deck, kind)?;
        let source_bytes = fs::read(&deck.path).map_err(|err| {
            format!(
                "failed to read expected-failure record {}: {err}",
                deck.path.display()
            )
        })?;
        let source_hash = blake3::hash(&source_bytes).to_hex().to_string();
        if source_hash != kind.source_blake3() {
            return Err(format!(
                "expected-failure record '{}' source digest changed: expected {}, got {}",
                kind.record(),
                kind.source_blake3(),
                source_hash
            ));
        }
        let source = std::str::from_utf8(&source_bytes).map_err(|err| {
            format!(
                "expected-failure record '{}' is not UTF-8: {err}",
                kind.record()
            )
        })?;
        let observation = match kind {
            XyceExpectedFailureKind::Bug67BehavioralExpression => {
                self.observe_bug67_behavioral_expression_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug671InvalidPwlFile => {
                self.observe_bug671_invalid_pwl_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug726AdjacentCouplings => {
                Self::observe_bug726_adjacent_coupling_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug744DcOperatingPoint => {
                self.observe_bug744_dc_operating_point_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug75UndefinedMutualInductorReference => {
                Self::observe_bug75_undefined_mutual_inductor_reference_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug1148UndefinedPrintNode
            | XyceExpectedFailureKind::Bug40UndefinedPrintNode
            | XyceExpectedFailureKind::Bug718InvalidPrintNodes
            | XyceExpectedFailureKind::MessagePrintBadNodeName
            | XyceExpectedFailureKind::MessagePrintBadVariable
            | XyceExpectedFailureKind::LeadCurrentsInvalidDevice
            | XyceExpectedFailureKind::MeasureInvalidNodes
            | XyceExpectedFailureKind::FourierBadLine3OutputSymbols => {
                Self::observe_undefined_output_symbols_failure(source, &deck.path, kind)?
            }
            XyceExpectedFailureKind::Bug387MissingLibraryEndl => {
                Self::observe_bug387_missing_library_endl_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::MessageSubcircuitMissingName => {
                Self::observe_subckt_noname_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::MessageSubcircuitMissingEndsEndCard
            | XyceExpectedFailureKind::MessageSubcircuitMissingEndsIncludeEof
            | XyceExpectedFailureKind::MessageSubcircuitMissingEndsTopLevelEof
            | XyceExpectedFailureKind::MessageSubcircuitMissingEndsTsInvEof => {
                Self::observe_missing_subcircuit_ends_failure(source, &deck.path, kind)?
            }
            XyceExpectedFailureKind::MessageSubcircuitDuplicateBindingA2
            | XyceExpectedFailureKind::MessageSubcircuitDuplicateBindingJ1 => {
                Self::observe_message_duplicate_subcircuit_binding_failure(
                    source, &deck.path, kind,
                )?
            }
            XyceExpectedFailureKind::MessageDcExcessArguments => {
                Self::observe_dc_excess_args_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::MessageAcUnsupportedSweepType => {
                Self::observe_unsupported_frequency_sweep_failure(source, &deck.path, kind)?
            }
            XyceExpectedFailureKind::MessageNoiseUnsupportedSweepType => {
                Self::observe_unsupported_frequency_sweep_failure(source, &deck.path, kind)?
            }
            XyceExpectedFailureKind::MessageMissingLibraryEndl => {
                Self::observe_message_missing_library_endl_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::MessageMissingLibraryFileUnquoted => {
                Self::observe_message_missing_library_file_failure(source, &deck.path, kind)?
            }
            XyceExpectedFailureKind::MessageMissingLibraryFileQuoted => {
                Self::observe_message_missing_library_file_failure(source, &deck.path, kind)?
            }
            XyceExpectedFailureKind::MessageDuplicateDevice => {
                Self::observe_message_duplicate_device_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::MessageMissingDeviceNodes => {
                Self::observe_message_missing_device_nodes_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug702DuplicateExternalInitcond
            | XyceExpectedFailureKind::Bug702DuplicateInlinedInitcond
            | XyceExpectedFailureKind::Bug702MalformedInitcondFile
            | XyceExpectedFailureKind::Bug702MissingInitcondFile => {
                Self::observe_bug702_expected_failure(source, &deck.path, kind)?
            }
            XyceExpectedFailureKind::Issue455DuplicateDcSourceFunction => {
                Self::observe_issue455_duplicate_dc_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug204InvalidDcSweepArity => {
                Self::observe_bug204_invalid_dc_sweep_arity(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug281InvalidDcSweepArity => {
                Self::observe_bug281_invalid_dc_sweep_arity(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug401BadDeviceLine => {
                self.observe_bug401_bad_device_line_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug401ExtraSpace => {
                self.observe_bug401_extra_space_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug401WorseDeviceLine => {
                Self::observe_bug401_worse_device_line_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug701DuplicateTopLevelDevice => {
                Self::observe_bug701_duplicate_toplevel_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug701DuplicateSubcircuitDevice => {
                Self::observe_bug701_duplicate_subcircuit_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug769ParameterNodeVoltage => {
                Self::observe_bug769_parameter_probe_failure(
                    source,
                    &deck.path,
                    XyceExpectedFailureKind::Bug769ParameterNodeVoltage,
                )?
            }
            XyceExpectedFailureKind::Bug769ParameterDeviceCurrent => {
                Self::observe_bug769_parameter_probe_failure(
                    source,
                    &deck.path,
                    XyceExpectedFailureKind::Bug769ParameterDeviceCurrent,
                )?
            }
            XyceExpectedFailureKind::Bug769ParameterLeadCurrent => {
                Self::observe_bug769_parameter_probe_failure(
                    source,
                    &deck.path,
                    XyceExpectedFailureKind::Bug769ParameterLeadCurrent,
                )?
            }
            XyceExpectedFailureKind::Bug1578InvalidDeviceType => {
                Self::observe_bug1578_invalid_device_type_failure(source, &deck.path)?
            }
            XyceExpectedFailureKind::Bug198UnrecognizedLine => {
                Self::observe_bug198_or_bug258_unrecognized_line_failure(
                    source,
                    &deck.path,
                    XyceExpectedFailureKind::Bug198UnrecognizedLine,
                )?
            }
            XyceExpectedFailureKind::Bug258UnrecognizedLine => {
                Self::observe_bug198_or_bug258_unrecognized_line_failure(
                    source,
                    &deck.path,
                    XyceExpectedFailureKind::Bug258UnrecognizedLine,
                )?
            }
            XyceExpectedFailureKind::Bug587InvalidNumericNotation => {
                Self::observe_bug587_invalid_numeric_notation_failure(source, &deck.path)?
            }
        };
        let expected = kind.expected_observation();
        if observation != expected {
            return Err(format!(
                "expected-failure record '{}' produced the wrong typed observation: expected {expected:?}, got {observation:?}",
                kind.record()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_expected_failure_provenance(
        &self,
        deck: &XyceDeck,
        kind: XyceExpectedFailureKind,
    ) -> Result<(), String> {
        if deck.section != XyceDeckSection::Netlists {
            return Err(format!(
                "expected-failure record '{}' is not in the Netlists corpus",
                kind.record()
            ));
        }
        let actual_record = Self::normalize_manifest_key(&deck.relative_path);
        if actual_record != kind.record() {
            return Err(format!(
                "expected-failure record path mismatch: expected '{}', got '{}'",
                kind.record(),
                deck.relative_path
            ));
        }
        let expected_path = self.root.join(Path::new(&deck.relative_path));
        let actual_canonical = deck.path.canonicalize().map_err(|err| {
            format!(
                "failed to canonicalize expected-failure record {}: {err}",
                deck.path.display()
            )
        })?;
        let expected_canonical = expected_path.canonicalize().map_err(|err| {
            format!(
                "expected-failure record '{}' is missing from the vendored corpus: {err}",
                kind.record()
            )
        })?;
        if actual_canonical != expected_canonical {
            return Err(format!(
                "expected-failure record '{}' resolved to {}, not {}",
                kind.record(),
                actual_canonical.display(),
                expected_canonical.display()
            ));
        }
        let metadata = fs::symlink_metadata(&deck.path).map_err(|err| {
            format!(
                "failed to inspect expected-failure record {}: {err}",
                deck.path.display()
            )
        })?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err(format!(
                "expected-failure record '{}' must be a regular non-symlink file",
                kind.record()
            ));
        }
        if !self.requires_upstream_wrapper(&deck.relative_path) {
            return Err(format!(
                "expected-failure record '{}' lost its removed-wrapper manifest provenance",
                kind.record()
            ));
        }

        let family_prefix = kind
            .record()
            .rsplit_once('/')
            .map(|(parent, _)| format!("{parent}/"))
            .ok_or_else(|| "expected-failure record has no family directory".to_string())?;
        let family_dir = deck
            .path
            .parent()
            .ok_or_else(|| "expected-failure record has no parent directory".to_string())?;
        if let Some(census) = kind.shared_family_census() {
            self.validate_expected_failure_shared_family_census(
                kind,
                family_dir,
                &family_prefix,
                census,
            )?;
        } else {
            let manifest_family = self
                .upstream_wrapper_decks
                .iter()
                .filter(|record| record.starts_with(&family_prefix))
                .collect::<Vec<_>>();
            if manifest_family.len() != 1 || manifest_family[0].as_str() != kind.record() {
                return Err(format!(
                    "expected-failure family '{}' must have exactly one manifest owner, found {manifest_family:?}",
                    family_prefix.trim_end_matches('/')
                ));
            }

            let mut circuit_siblings = Vec::new();
            for entry in fs::read_dir(family_dir).map_err(|err| {
                format!(
                    "failed to inspect expected-failure family {}: {err}",
                    family_dir.display()
                )
            })? {
                let entry = entry.map_err(|err| {
                    format!(
                        "failed to read expected-failure family entry in {}: {err}",
                        family_dir.display()
                    )
                })?;
                let path = entry.path();
                if path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                {
                    circuit_siblings.push(path);
                }
            }
            circuit_siblings.sort();
            if circuit_siblings.len() != 1
                || circuit_siblings[0].canonicalize().ok().as_ref() != Some(&actual_canonical)
            {
                return Err(format!(
                    "expected-failure family '{}' must contain exactly its one qualified .cir record",
                    family_dir.display()
                ));
            }
        }
        if kind.is_bug702_family() {
            self.validate_bug702_complete_family_provenance(family_dir)?;
        }
        if kind.is_bug75() {
            self.validate_bug75_complete_family_provenance(family_dir)?;
        }
        if kind.has_complete_output_symbol_family_envelope() {
            self.validate_output_symbol_complete_family_provenance(kind, family_dir)?;
        }
        Self::validate_expected_failure_source_sidecars(kind, &deck.path)?;

        let output_anchor = self
            .static_output_reference_path(&deck.path, "anchor")
            .ok_or_else(|| {
                format!(
                    "expected-failure record '{}' cannot be mapped into OutputData",
                    kind.record()
                )
            })?;
        let output_dir = output_anchor
            .parent()
            .ok_or_else(|| "expected-failure OutputData path has no parent".to_string())?;
        let deck_name = deck
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "expected-failure filename is not UTF-8".to_string())?;
        let artifact_prefix = format!("{deck_name}.").to_ascii_lowercase();
        let mut artifacts = Vec::new();
        if output_dir.is_dir() {
            for entry in fs::read_dir(output_dir).map_err(|err| {
                format!(
                    "failed to inspect expected-failure OutputData directory {}: {err}",
                    output_dir.display()
                )
            })? {
                let entry = entry.map_err(|err| {
                    format!(
                        "failed to read expected-failure OutputData entry in {}: {err}",
                        output_dir.display()
                    )
                })?;
                if entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.to_ascii_lowercase().starts_with(&artifact_prefix))
                {
                    artifacts.push(entry.path());
                }
            }
        }
        artifacts.sort();
        if let Some(expected_artifact) = kind.retained_non_oracle_artifact() {
            if artifacts.len() != 1 {
                return Err(format!(
                    "expected-failure record '{}' must retain exactly its one classified non-oracle artifact '{}', found {artifacts:?}",
                    kind.record(),
                    expected_artifact.file_name
                ));
            }
            let artifact = &artifacts[0];
            let artifact_name = artifact
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| "expected-failure artifact filename is not UTF-8".to_string())?;
            if artifact_name != expected_artifact.file_name {
                return Err(format!(
                    "expected-failure record '{}' retained non-oracle artifact changed name: expected '{}', got '{}'",
                    kind.record(),
                    expected_artifact.file_name,
                    artifact_name
                ));
            }
            let metadata = fs::symlink_metadata(artifact).map_err(|err| {
                format!(
                    "failed to inspect expected-failure retained artifact {}: {err}",
                    artifact.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "expected-failure retained artifact {} must be a regular non-symlink file",
                    artifact.display()
                ));
            }
            let bytes = fs::read(artifact).map_err(|err| {
                format!(
                    "failed to read expected-failure retained artifact {}: {err}",
                    artifact.display()
                )
            })?;
            let digest = blake3::hash(&bytes).to_hex().to_string();
            if bytes.len() != expected_artifact.bytes || digest != expected_artifact.blake3 {
                return Err(format!(
                    "expected-failure record '{}' retained non-oracle artifact changed: expected {} bytes / {}, got {} bytes / {}",
                    kind.record(),
                    expected_artifact.bytes,
                    expected_artifact.blake3,
                    bytes.len(),
                    digest
                ));
            }
        } else if !artifacts.is_empty() {
            return Err(format!(
                "expected-failure record '{}' must not own checked-in output artifacts: {artifacts:?}",
                kind.record()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_expected_failure_source_sidecars(
        kind: XyceExpectedFailureKind,
        deck_path: &Path,
    ) -> Result<(), String> {
        if !kind.rejects_source_directory_sidecars() {
            return Ok(());
        }
        let family_dir = deck_path
            .parent()
            .ok_or_else(|| "expected-failure record has no source directory".to_string())?;
        let deck_name = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "expected-failure filename is not UTF-8".to_string())?;
        let sidecar_prefix = format!("{deck_name}.").to_ascii_lowercase();
        let mut sidecars = Vec::new();
        for entry in fs::read_dir(family_dir).map_err(|error| {
            format!(
                "failed to inspect expected-failure source directory {}: {error}",
                family_dir.display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to inspect expected-failure source entry in {}: {error}",
                    family_dir.display()
                )
            })?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.to_ascii_lowercase().starts_with(&sidecar_prefix))
            {
                sidecars.push(entry.path());
            }
        }
        sidecars.sort();
        if let Some(expected) = kind.expected_source_sidecar() {
            if sidecars.len() != 1 {
                return Err(format!(
                    "expected-failure record '{}' must retain exactly its pinned source sidecar '{}', found {sidecars:?}",
                    kind.record(),
                    expected.file_name
                ));
            }
            let sidecar = &sidecars[0];
            let actual_name = sidecar
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| {
                    "expected-failure source sidecar filename is not UTF-8".to_string()
                })?;
            if actual_name != expected.file_name {
                return Err(format!(
                    "expected-failure record '{}' source sidecar changed name: expected '{}', got '{actual_name}'",
                    kind.record(),
                    expected.file_name
                ));
            }
            let metadata = fs::symlink_metadata(sidecar).map_err(|error| {
                format!(
                    "failed to inspect expected-failure source sidecar {}: {error}",
                    sidecar.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "expected-failure source sidecar {} must be a regular non-symlink file",
                    sidecar.display()
                ));
            }
            let bytes = fs::read(sidecar).map_err(|error| {
                format!(
                    "failed to read expected-failure source sidecar {}: {error}",
                    sidecar.display()
                )
            })?;
            let hash = blake3::hash(&bytes).to_hex().to_string();
            if bytes.len() != expected.bytes || hash != expected.blake3 {
                return Err(format!(
                    "expected-failure record '{}' source sidecar changed: expected {} bytes / {}, got {} bytes / {hash}",
                    kind.record(),
                    expected.bytes,
                    expected.blake3,
                    bytes.len()
                ));
            }
        } else if !sidecars.is_empty() {
            return Err(format!(
                "expected-failure record '{}' must not own source-directory sidecars: {sidecars:?}",
                kind.record()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_expected_failure_shared_family_census(
        &self,
        kind: XyceExpectedFailureKind,
        family_dir: &Path,
        family_prefix: &str,
        expected: XyceExpectedFailureFamilyCensus,
    ) -> Result<(), String> {
        let mut physical_names = BTreeSet::new();
        for entry in fs::read_dir(family_dir).map_err(|err| {
            format!(
                "failed to inspect expected-failure shared family {}: {err}",
                family_dir.display()
            )
        })? {
            let entry = entry.map_err(|err| {
                format!(
                    "failed to read expected-failure shared-family entry in {}: {err}",
                    family_dir.display()
                )
            })?;
            let path = entry.path();
            if !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            {
                continue;
            }
            let metadata = fs::symlink_metadata(&path).map_err(|err| {
                format!(
                    "failed to inspect expected-failure shared-family member {}: {err}",
                    path.display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "expected-failure shared-family member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| {
                    format!(
                        "expected-failure shared-family member {} is not UTF-8",
                        path.display()
                    )
                })?
                .to_ascii_lowercase();
            if !physical_names.insert(name.clone()) {
                return Err(format!(
                    "expected-failure shared family '{}' contains a case-colliding .cir name '{name}'",
                    family_dir.display()
                ));
            }
        }
        let physical_names = physical_names.into_iter().collect::<Vec<_>>();
        let physical_hash = blake3::hash(physical_names.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if physical_names.len() != expected.physical_cir_count
            || physical_hash != expected.physical_names_blake3
        {
            return Err(format!(
                "expected-failure shared family '{}' physical .cir census changed: expected {} records / {}, got {} records / {}",
                family_dir.display(),
                expected.physical_cir_count,
                expected.physical_names_blake3,
                physical_names.len(),
                physical_hash
            ));
        }

        let manifest_records = self.expected_failure_manifest_family_records(family_prefix)?;
        let manifest_hash = blake3::hash(manifest_records.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if expected.require_manifest_bijection {
            let mut manifest_names = BTreeSet::new();
            for record in &manifest_records {
                let name = record
                    .rsplit_once('/')
                    .map(|(_, name)| name)
                    .ok_or_else(|| {
                        format!(
                            "expected-failure shared-family manifest record '{record}' has no filename"
                        )
                    })?
                    .to_ascii_lowercase();
                if !manifest_names.insert(name.clone()) {
                    return Err(format!(
                        "expected-failure shared family '{}' contains a case-colliding manifest filename '{name}'",
                        family_prefix.trim_end_matches('/')
                    ));
                }
            }
            let manifest_names = manifest_names.into_iter().collect::<Vec<_>>();
            if manifest_names != physical_names {
                return Err(format!(
                    "expected-failure shared family '{}' manifest/physical .cir census is not a bijection: physical={physical_names:?}, manifest={manifest_names:?}",
                    family_prefix.trim_end_matches('/')
                ));
            }
        }
        if manifest_records.len() != expected.manifest_owner_count
            || manifest_hash != expected.manifest_records_blake3
        {
            return Err(format!(
                "expected-failure shared family '{}' manifest census changed: expected {} owners / {}, got {} owners / {}",
                family_prefix.trim_end_matches('/'),
                expected.manifest_owner_count,
                expected.manifest_records_blake3,
                manifest_records.len(),
                manifest_hash
            ));
        }
        if !manifest_records
            .iter()
            .any(|record| record == kind.record())
        {
            return Err(format!(
                "expected-failure record '{}' is absent from its pinned shared-family manifest census",
                kind.record()
            ));
        }
        Ok(())
    }

    pub(super) fn expected_failure_manifest_family_records(
        &self,
        family_prefix: &str,
    ) -> Result<Vec<String>, String> {
        let manifest_path = self.root.join(HARNESS_MANIFEST_FILE);
        let content = fs::read_to_string(&manifest_path).map_err(|error| {
            format!(
                "failed to read expected-failure manifest {}: {error}",
                manifest_path.display()
            )
        })?;
        let mut records = Vec::new();
        let mut normalized_rows = BTreeMap::<String, (usize, String)>::new();
        for (index, raw_line) in content.lines().enumerate() {
            let line_number = index + 1;
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((path, contract)) = line.split_once('\t') else {
                let normalized = Self::normalize_manifest_key(line);
                if normalized.starts_with(family_prefix) {
                    return Err(format!(
                        "expected-failure shared-family manifest row {line_number} is malformed: {raw_line:?}"
                    ));
                }
                continue;
            };
            let normalized = Self::normalize_manifest_key(path);
            if !normalized.starts_with(family_prefix) {
                continue;
            }
            if contract.trim() != REQUIRES_UPSTREAM_WRAPPER_CONTRACT {
                return Err(format!(
                    "expected-failure shared-family manifest row {line_number} has contract {:?}, expected {:?}",
                    contract.trim(),
                    REQUIRES_UPSTREAM_WRAPPER_CONTRACT
                ));
            }
            if let Some((first_line, first_path)) =
                normalized_rows.insert(normalized.clone(), (line_number, path.to_string()))
            {
                return Err(format!(
                    "expected-failure shared-family manifest contains duplicate or case-colliding record '{normalized}' at rows {first_line} ({first_path:?}) and {line_number} ({path:?})"
                ));
            }
            records.push(normalized);
        }
        records.sort();
        Ok(records)
    }

    pub(super) fn require_expected_failure_source_lines(
        label: &str,
        source: &str,
        expected_line_count: usize,
        expected_lines: &[(usize, &str)],
    ) -> Result<(), String> {
        let lines = source.lines().collect::<Vec<_>>();
        if lines.len() != expected_line_count {
            return Err(format!(
                "{label} physical line count changed: expected {expected_line_count}, got {}",
                lines.len()
            ));
        }
        for &(line_number, expected) in expected_lines {
            let actual = lines.get(line_number - 1).copied().ok_or_else(|| {
                format!("{label} is missing required physical line {line_number}")
            })?;
            if actual != expected {
                return Err(format!(
                    "{label} physical line {line_number} changed: expected {expected:?}, got {actual:?}"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn require_expected_failure_file_name(
        label: &str,
        deck_path: &Path,
        expected_file_name: &str,
    ) -> Result<(), String> {
        let actual = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("{label} deck path has no UTF-8 filename"))?;
        if actual != expected_file_name {
            return Err(format!(
                "{label} deck filename changed: expected {expected_file_name:?}, got {actual:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn observe_bug387_missing_library_endl_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "BUG 387",
            source,
            9,
            &[
                (3, ".lib nom.lib"),
                (4, "c1 1 0 1uF IC=1"),
                (7, ".print tran v(1)"),
                (8, ".tran 0 5ms"),
                (9, ".end"),
            ],
        )?;
        if source
            .lines()
            .any(|line| line.trim().eq_ignore_ascii_case(".endl"))
        {
            return Err("BUG 387 must retain the missing .ENDL condition".to_string());
        }
        Self::require_exact_syntax_failure(
            "BUG 387",
            source,
            deck_path,
            3,
            "Library section 'nom.lib' missing .ENDL",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::MissingLibraryEndl,
            identifiers: vec!["nom.lib".to_string(), "line 3".to_string()],
        })
    }

    pub(super) fn observe_subckt_noname_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "Message/Subcircuit subckt_noname",
            source,
            28,
            &[
                (10, "xsub1 0 1 2 testsub"),
                (12, ".subckt testsub a b c"),
                (15, ".ends"),
                (21, ".subckt "),
                (22, "r1 a b 1"),
                (23, ".ends"),
                (25, ".tran 0 1"),
                (26, ".print tran v(1)"),
            ],
        )?;
        Self::require_exact_syntax_failure(
            "Message/Subcircuit subckt_noname",
            source,
            deck_path,
            21,
            ".SUBCKT requires a subcircuit name",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::MissingSubcircuitName,
            identifiers: vec![".SUBCKT".to_string(), "line 21".to_string()],
        })
    }

    pub(super) fn canonical_expected_failure_source_path(
        path: &Path,
        label: &str,
    ) -> Result<PathBuf, String> {
        path.canonicalize().map_err(|error| {
            format!(
                "failed to canonicalize {label} path {}: {error}",
                path.display()
            )
        })
    }

    pub(super) fn expected_failure_location_identifier(
        location: &rspice_core::netlist::NetlistSourceLocation,
    ) -> Result<String, String> {
        let path = location
            .path
            .as_deref()
            .ok_or_else(|| "expected file-backed source location has no path".to_string())?;
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                format!(
                    "source location path {} has no UTF-8 filename",
                    path.display()
                )
            })?;
        Ok(format!("{file_name}:{}", location.line))
    }

    pub(super) fn observe_missing_subcircuit_ends_failure(
        source: &str,
        deck_path: &Path,
        kind: XyceExpectedFailureKind,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let (
            label,
            authored_name,
            canonical_name,
            qualified_name,
            expected_deck_file,
            opened_line,
            detected_line,
            boundary,
            dependency_origin,
        ) = match kind {
            XyceExpectedFailureKind::MessageSubcircuitMissingEndsEndCard => {
                Self::require_expected_failure_source_lines(
                    "Message/Subcircuit subckt_missing_ends",
                    source,
                    26,
                    &[
                        (10, "xsub1 0 1 2 testsub"),
                        (12, ".subckt testsub a b c"),
                        (17, "* it should be \".ends\" but using \".end\" here"),
                        (21, ".end"),
                        (23, ".tran 0 1"),
                        (24, ".print tran v(1)"),
                        (25, ".end"),
                    ],
                )?;
                (
                    "Message/Subcircuit subckt_missing_ends",
                    "testsub",
                    "TESTSUB",
                    "TESTSUB",
                    "subckt_missing_ends.cir",
                    12,
                    21,
                    MissingSubcircuitEndsBoundary::EndCard,
                    false,
                )
            }
            XyceExpectedFailureKind::MessageSubcircuitMissingEndsIncludeEof => {
                Self::require_expected_failure_source_lines(
                    "Message/Subcircuit subckt_missing_ends2",
                    source,
                    17,
                    &[
                        (10, "xsub1 0 1 2 testsub"),
                        (12, ".include missing.ends"),
                        (14, ".tran 0 1"),
                        (15, ".print tran v(1)"),
                        (16, ".end"),
                    ],
                )?;
                (
                    "Message/Subcircuit subckt_missing_ends2",
                    "testsub",
                    "TESTSUB",
                    "TESTSUB",
                    "subckt_missing_ends2.cir",
                    1,
                    4,
                    MissingSubcircuitEndsBoundary::EndOfSource,
                    true,
                )
            }
            XyceExpectedFailureKind::MessageSubcircuitMissingEndsTopLevelEof => {
                Self::require_expected_failure_source_lines(
                    "Message/Subcircuit subckt_missing_ends3",
                    source,
                    20,
                    &[
                        (10, "xsub1 0 1 2 testsub"),
                        (12, ".tran 0 1"),
                        (13, ".print tran v(1)"),
                        (17, ".subckt testsub a b c"),
                        (18, "r1 a b 1"),
                        (19, "r2 b c 1"),
                        (20, "*.ends"),
                    ],
                )?;
                (
                    "Message/Subcircuit subckt_missing_ends3",
                    "testsub",
                    "TESTSUB",
                    "TESTSUB",
                    "subckt_missing_ends3.cir",
                    17,
                    21,
                    MissingSubcircuitEndsBoundary::EndOfSource,
                    false,
                )
            }
            XyceExpectedFailureKind::MessageSubcircuitMissingEndsTsInvEof => {
                Self::require_expected_failure_source_lines(
                    "Message/Subcircuit subckt_missing_ends4",
                    source,
                    31,
                    &[
                        (17, "XU2 CLK CLKB TS_INV"),
                        (22, ".SUBCKT TS_INV A  YN"),
                        (24, "s1 digpower risefall digpower a Lsw_1meg"),
                        (
                            28,
                            "r1  risefall z 0.1 ; this is resistor with incorrectly labeled node \"z\"",
                        ),
                        (31, "*.ENDS TS_INV"),
                    ],
                )?;
                (
                    "Message/Subcircuit subckt_missing_ends4",
                    "TS_INV",
                    "TS_INV",
                    "TS_INV",
                    "subckt_missing_ends4.cir",
                    22,
                    32,
                    MissingSubcircuitEndsBoundary::EndOfSource,
                    false,
                )
            }
            _ => {
                return Err(format!(
                    "non-missing-.ENDS expected-failure kind passed to missing-.ENDS observer: {kind:?}"
                ));
            }
        };

        let actual_deck_file = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("{label} deck path has no UTF-8 filename"))?;
        if actual_deck_file != expected_deck_file {
            return Err(format!(
                "{label} deck filename changed: expected {expected_deck_file:?}, got {actual_deck_file:?}"
            ));
        }
        let deck_canonical = Self::canonical_expected_failure_source_path(deck_path, label)?;
        let expected_origin = if dependency_origin {
            Self::validate_missing_subcircuit_ends_dependency(deck_path)?
        } else {
            deck_canonical
        };
        let error = match Self::parse_xyce_netlist(source, deck_path) {
            Err(error) => error,
            Ok(_) => {
                return Err(format!(
                    "{label} unexpectedly parsed; the missing-.ENDS condition is absent"
                ));
            }
        };
        let ParseError::MissingSubcircuitEnds(error) = error else {
            return Err(format!(
                "{label} produced the wrong typed parse failure: {error:?}"
            ));
        };
        let MissingSubcircuitEndsError {
            authored_name: actual_authored,
            canonical_name: actual_canonical,
            qualified_name: actual_qualified,
            opened_at,
            detected_at,
            boundary: actual_boundary,
        } = *error;
        let opened_path = opened_at
            .path
            .as_deref()
            .ok_or_else(|| format!("{label} opening location lost its source path"))?;
        let detected_path = detected_at
            .path
            .as_deref()
            .ok_or_else(|| format!("{label} detection location lost its source path"))?;
        let opened_canonical =
            Self::canonical_expected_failure_source_path(opened_path, "observed opening source")?;
        let detected_canonical = Self::canonical_expected_failure_source_path(
            detected_path,
            "observed detection source",
        )?;
        if actual_authored != authored_name
            || actual_canonical != canonical_name
            || actual_qualified != qualified_name
            || opened_at.line != opened_line
            || detected_at.line != detected_line
            || actual_boundary != boundary
            || opened_canonical != expected_origin
            || detected_canonical != expected_origin
        {
            return Err(format!(
                "{label} typed missing-.ENDS observation changed: authored={actual_authored:?}, canonical={actual_canonical:?}, qualified={actual_qualified:?}, opened_at={opened_at:?}, detected_at={detected_at:?}, boundary={actual_boundary:?}"
            ));
        }
        let boundary_identifier = match actual_boundary {
            MissingSubcircuitEndsBoundary::EndCard => "END_CARD",
            MissingSubcircuitEndsBoundary::AlterCard => "ALTER_CARD",
            MissingSubcircuitEndsBoundary::EndOfSource => "END_OF_SOURCE",
        };
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::MissingSubcircuitEnds,
            identifiers: vec![
                actual_authored,
                actual_canonical,
                actual_qualified,
                Self::expected_failure_location_identifier(&opened_at)?,
                Self::expected_failure_location_identifier(&detected_at)?,
                boundary_identifier.to_string(),
            ],
        })
    }

    pub(super) fn observe_message_duplicate_subcircuit_binding_failure(
        source: &str,
        deck_path: &Path,
        kind: XyceExpectedFailureKind,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let (label, file_name, expected_error) = match kind {
            XyceExpectedFailureKind::MessageSubcircuitDuplicateBindingA2 => {
                Self::require_expected_failure_source_lines(
                    "Message/Subcircuit subckt_a2_dup_error",
                    source,
                    82,
                    &[
                        (28, ".subckt INV1 IN OUT VDD GND IN OUT VDD GND"),
                        (47, ".ends"),
                        (49, ".subckt INV2 IN OUT VDD GND IN OUT VDD GND"),
                        (68, ".ends"),
                        (70, "Xinv1 IN MID VDD 0 IN MID VDD VDD INV1"),
                        (71, "Xinv2 MID OUT VDD 0 MID OUT VDD 0 INV2"),
                        (78, ".tran 20ns 30us"),
                        (79, ".print tran PRECISION=10 WIDTH=19 v(out) v(in) v(1)"),
                        (82, ".end"),
                    ],
                )?;
                (
                    "Message/Subcircuit subckt_a2_dup_error",
                    "subckt_a2_dup_error.cir",
                    DuplicateSubcircuitPortBindingError {
                        subcircuit_name: "INV1".to_string(),
                        canonical_subcircuit_name: "INV1".to_string(),
                        instance_name: "Xinv1".to_string(),
                        canonical_instance_name: "XINV1".to_string(),
                        qualified_instance_name: "Xinv1".to_string(),
                        formal_port: "GND".to_string(),
                        first_position: 4,
                        conflicting_position: 8,
                        first_actual_node: "0".to_string(),
                        conflicting_actual_node: "VDD".to_string(),
                    },
                )
            }
            XyceExpectedFailureKind::MessageSubcircuitDuplicateBindingJ1 => {
                Self::require_expected_failure_source_lines(
                    "Message/Subcircuit subckt_j1_dup_error",
                    source,
                    93,
                    &[
                        (29, "X1 1 2 3 9 13 99 99 1 ONEBIT"),
                        (42, ".TRAN 0.5N 200N"),
                        (43, ".PRINT TRAN V(1) V(2) V(3) V(9) V(13)"),
                        (45, ".options linsol type=klu"),
                        (46, ".OPTIONS TIMEINT ABSTOL=1.0E-3 RELTOL=1.0E-3"),
                        (48, ".subckt myres 1 2"),
                        (51, ".ends"),
                        (53, ".SUBCKT ONEBIT 1 2 3 4 5 6 6 6"),
                        (71, ".SUBCKT AND 1 2 3 4 1 2 3 4"),
                        (79, ".SUBCKT XOR 1 2 3 4 1  p1=0 r=1"),
                        (
                            89,
                            "X4 3 7 9 6 3 7 9 6 AND ; these two lines of ONEBIT here to test context switching",
                        ),
                        (91, ".ENDS ONEBIT"),
                        (93, ".END   "),
                    ],
                )?;
                (
                    "Message/Subcircuit subckt_j1_dup_error",
                    "subckt_j1_dup_error.cir",
                    DuplicateSubcircuitPortBindingError {
                        subcircuit_name: "ONEBIT".to_string(),
                        canonical_subcircuit_name: "ONEBIT".to_string(),
                        instance_name: "X1".to_string(),
                        canonical_instance_name: "X1".to_string(),
                        qualified_instance_name: "X1".to_string(),
                        formal_port: "6".to_string(),
                        first_position: 6,
                        conflicting_position: 8,
                        first_actual_node: "99".to_string(),
                        conflicting_actual_node: "1".to_string(),
                    },
                )
            }
            _ => {
                return Err(format!(
                    "non-duplicate-binding expected-failure kind passed to duplicate-binding observer: {kind:?}"
                ));
            }
        };
        Self::require_expected_failure_file_name(label, deck_path, file_name)?;

        let netlist = Self::parse_xyce_netlist(source, deck_path)
            .map_err(|error| format!("{label} must parse before hierarchy validation: {error}"))?;
        let diagnostics_are_exact = match kind {
            XyceExpectedFailureKind::MessageSubcircuitDuplicateBindingA2 => {
                netlist.diagnostics.is_empty()
            }
            XyceExpectedFailureKind::MessageSubcircuitDuplicateBindingJ1 => {
                netlist.diagnostics.len() == 2
                    && netlist.diagnostics[0].line == 45
                    && netlist.diagnostics[0].code == "unknown-option"
                    && netlist.diagnostics[0].message == "unknown .options key 'LINSOL' ignored"
                    && netlist.diagnostics[1].line == 45
                    && netlist.diagnostics[1].code == "unknown-option"
                    && netlist.diagnostics[1].message == "unknown .options key 'TYPE' ignored"
            }
            _ => false,
        };
        if !diagnostics_are_exact {
            return Err(format!(
                "{label} parser diagnostics changed: {:?}",
                netlist.diagnostics
            ));
        }

        let error = match rspice_core::netlist::flatten_netlist_with_models(&netlist) {
            Err(error) => error,
            Ok(_) => {
                return Err(format!(
                    "{label} unexpectedly flattened; the conflicting duplicate-formal binding is absent"
                ));
            }
        };
        let ParseError::DuplicateSubcircuitPortBinding(actual_error) = error else {
            return Err(format!(
                "{label} produced the wrong typed hierarchy failure: {error:?}"
            ));
        };
        if *actual_error != expected_error {
            return Err(format!(
                "{label} duplicate-formal binding observation changed: expected {expected_error:?}, got {actual_error:?}"
            ));
        }

        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::CircuitBuild,
            category: XyceExpectedFailureCategory::DuplicateSubcircuitPortBinding,
            identifiers: vec![
                actual_error.subcircuit_name,
                actual_error.canonical_subcircuit_name,
                actual_error.instance_name,
                actual_error.canonical_instance_name,
                actual_error.qualified_instance_name,
                actual_error.formal_port,
                actual_error.first_position.to_string(),
                actual_error.conflicting_position.to_string(),
                actual_error.first_actual_node,
                actual_error.conflicting_actual_node,
            ],
        })
    }

    pub(super) fn observe_dc_excess_args_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "Message/Input DC_excessArgs",
            source,
            9,
            &[
                (3, "V1 1 0 1.0"),
                (4, "R1 1 0 1.0"),
                (6, ".DC V1 -8.0 -4.0 0.0 4.0"),
                (8, ".print dc V(1)"),
            ],
        )?;
        Self::require_exact_syntax_failure(
            "Message/Input DC_excessArgs",
            source,
            deck_path,
            6,
            ".DC has unexpected trailing token Number(4.0)",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::DcExcessArguments,
            identifiers: vec!["V1".to_string(), "4.0".to_string(), "line 6".to_string()],
        })
    }

    pub(super) fn observe_unsupported_frequency_sweep_failure(
        source: &str,
        deck_path: &Path,
        kind: XyceExpectedFailureKind,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let (label, file_name, line_count, sweep_line, analysis) = match kind {
            XyceExpectedFailureKind::MessageAcUnsupportedSweepType => (
                "Message/Input AC_setupSweepParam",
                "AC_setupSweepParam.cir",
                17,
                14,
                "AC",
            ),
            XyceExpectedFailureKind::MessageNoiseUnsupportedSweepType => (
                "Message/Input NOISE_setupSweepParam",
                "NOISE_setupSweepParam.cir",
                22,
                17,
                "NOISE",
            ),
            _ => {
                return Err(format!(
                    "non-AC/NOISE expected-failure kind passed to unsupported-sweep observer: {kind:?}"
                ));
            }
        };
        Self::require_expected_failure_file_name(label, deck_path, file_name)?;
        let expected_lines = if analysis == "AC" {
            vec![
                (9, "R1 b 0 2"),
                (10, "C1 a b 1u"),
                (11, "V1 a 0 DC 0V AC 1"),
                (13, ".print AC vm(b)"),
                (14, ".ac bogo 5 100 1e6"),
                (16, ".end"),
            ]
        } else {
            vec![
                (8, "v1  1 0 dc 5.0 ac 1"),
                (13, "eamp  3 0 2 0 1"),
                (15, "clp1  4 0 1.59nf"),
                (17, ".noise v(4) v1 bogo 5 100 1e6"),
                (19, ".print noise V(4) {log(onoise)} {log(inoise)}"),
                (21, ".end"),
            ]
        };
        Self::require_expected_failure_source_lines(label, source, line_count, &expected_lines)?;
        Self::require_exact_syntax_failure(
            label,
            source,
            deck_path,
            sweep_line,
            "Unknown frequency variation: BOGO",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::InvalidFrequencySweepType,
            identifiers: vec![
                analysis.to_string(),
                "BOGO".to_string(),
                format!("line {sweep_line}"),
            ],
        })
    }

    pub(super) fn observe_message_missing_library_endl_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let label = "Message/Input CircuitBlock_parseIncludeFile_2a";
        Self::require_expected_failure_file_name(
            label,
            deck_path,
            "CircuitBlock_parseIncludeFile_2a.cir",
        )?;
        Self::require_expected_failure_source_lines(
            label,
            source,
            10,
            &[
                (
                    1,
                    "demonstrates how Xyce missing the analysis statement if it can't find a library",
                ),
                (3, ".lib plugh.lib"),
                (5, "c1 1 0 1uF IC=1"),
                (8, ".print tran v(1)"),
                (9, ".tran 0 5ms"),
                (10, ".end"),
            ],
        )?;
        if source
            .lines()
            .any(|line| line.trim().eq_ignore_ascii_case(".endl"))
        {
            return Err(format!("{label} must retain the missing .ENDL condition"));
        }
        Self::require_exact_syntax_failure(
            label,
            source,
            deck_path,
            3,
            "Library section 'plugh.lib' missing .ENDL",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::MissingLibraryEndl,
            identifiers: vec![
                "plugh.lib".to_string(),
                "UNQUOTED".to_string(),
                "line 3".to_string(),
            ],
        })
    }

    pub(super) fn observe_message_missing_library_file_failure(
        source: &str,
        deck_path: &Path,
        kind: XyceExpectedFailureKind,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let (label, file_name, library_line, quoting) = match kind {
            XyceExpectedFailureKind::MessageMissingLibraryFileUnquoted => (
                "Message/Input CircuitBlock_parseIncludeFile_2b",
                "CircuitBlock_parseIncludeFile_2b.cir",
                ".lib plugh.lib x",
                "UNQUOTED",
            ),
            XyceExpectedFailureKind::MessageMissingLibraryFileQuoted => (
                "Message/Input CircuitBlock_parseIncludeFile_2c",
                "CircuitBlock_parseIncludeFile_2c.cir",
                ".lib 'plugh.lib' x",
                "SINGLE_QUOTED",
            ),
            _ => {
                return Err(format!(
                    "non-missing-library-file kind passed to dependency observer: {kind:?}"
                ));
            }
        };
        Self::require_expected_failure_file_name(label, deck_path, file_name)?;
        Self::require_expected_failure_source_lines(
            label,
            source,
            10,
            &[
                (
                    1,
                    "demonstrates how Xyce missing the analysis statement if it can't find a library",
                ),
                (3, library_line),
                (5, "c1 1 0 1uF IC=1"),
                (8, ".print tran v(1)"),
                (9, ".tran 0 5ms"),
                (10, ".end"),
            ],
        )?;
        let execution_dir = deck_path
            .parent()
            .ok_or_else(|| format!("{label} deck has no execution directory"))?;
        Self::require_missing_library_dependency_absent(label, deck_path, execution_dir)?;
        let expected_message = format!(
            "{}:3: .lib resolution failed: Include file not found: plugh.lib (searched {})",
            deck_path.display(),
            execution_dir.display()
        );
        match Self::parse_netlist_with_expression_dialect_and_execution_dir(
            source,
            deck_path,
            ExpressionDialect::Xyce,
            Some(execution_dir),
        ) {
            Err(ParseError::Syntax { line, message })
                if line == 3 && message == expected_message => {}
            Err(error) => {
                return Err(format!(
                    "{label} produced the wrong typed missing-library failure: expected line 3 / {expected_message:?}, got {error:?}"
                ));
            }
            Ok(_) => {
                return Err(format!(
                    "{label} unexpectedly parsed; the missing-library condition is absent"
                ));
            }
        }
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::MissingLibraryFile,
            identifiers: vec![
                "plugh.lib".to_string(),
                "x".to_string(),
                quoting.to_string(),
                "line 3".to_string(),
            ],
        })
    }

    pub(super) fn observe_message_duplicate_device_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let label = "Message/Device CircuitBlock_addTableData_1";
        Self::require_expected_failure_file_name(
            label,
            deck_path,
            "CircuitBlock_addTableData_1.cir",
        )?;
        Self::require_expected_failure_source_lines(
            label,
            source,
            16,
            &[
                (1, "level=1 diode circuit"),
                (4, "R 1 2 0.0001"),
                (7, "D2 1 3 DA"),
                (8, "DA 1 3 DA"),
                (9, "DA 1 3 DA"),
                (12, ".MODEL DA D (RS=1.73320090e-004 IS=1.85431192e-010)"),
                (14, ".DC V2 0.0 2.0 0.005"),
                (15, ".print DC v(2) I(V1)  "),
                (16, ".END"),
            ],
        )?;
        Self::require_exact_duplicate_device_failure(
            label,
            source,
            deck_path,
            "DA",
            "TOP_LEVEL",
            8,
            9,
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::DuplicateDeviceName,
            identifiers: vec![
                "DA".to_string(),
                "TOP_LEVEL".to_string(),
                "line 8".to_string(),
                "line 9".to_string(),
            ],
        })
    }

    pub(super) fn observe_message_missing_device_nodes_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let label = "Message/Device DeviceBlock_extractNodes_1";
        Self::require_expected_failure_file_name(
            label,
            deck_path,
            "DeviceBlock_extractNodes_1.cir",
        )?;
        Self::require_expected_failure_source_lines(
            label,
            source,
            18,
            &[
                (1, "test circuit for missing nodes on lines"),
                (4, "V1 in_1 0 5V"),
                (7, ".DC V1 1 5 1"),
                (10, ".PRINT DC v(in_1) v(out_1)"),
                (13, "R1 in_1 out_1 100K"),
                (14, "R2 out_1"),
                (17, "C2 out_1 100pF"),
            ],
        )?;
        Self::require_exact_syntax_failure(
            label,
            source,
            deck_path,
            14,
            "Expected node name, found Eof",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::MissingDeviceNodes,
            identifiers: vec!["R2".to_string(), "OUT_1".to_string(), "line 14".to_string()],
        })
    }

    pub(super) fn observe_bug702_expected_failure(
        source: &str,
        deck_path: &Path,
        kind: XyceExpectedFailureKind,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let family_dir = deck_path
            .parent()
            .ok_or_else(|| "BUG702 deck has no family execution directory".to_string())?;
        let (label, file_name, line_count, expected_lines) = match kind {
            XyceExpectedFailureKind::Bug702DuplicateExternalInitcond => (
                "BUG702 dup-external",
                "dup-external.cir",
                156,
                vec![
                    (20, ".initCOND FILE \"initcond.dat\""),
                    (25, ".tran 20ns 30us"),
                    (
                        26,
                        ".print tran PRECISION=10 WIDTH=19 v(vout) {v(in)+1.0} v(1)",
                    ),
                    (29, ".INITcond initcond.dat"),
                    (37, "XINV1 IN VOUT VDD 0 INVERTER"),
                    (156, ".END"),
                ],
            ),
            XyceExpectedFailureKind::Bug702DuplicateInlinedInitcond => (
                "BUG702 dup-inlined",
                "dup-inlined.cir",
                156,
                vec![
                    (20, ".initcond XiNv1:mn1 ic=2,0"),
                    (25, ".tran 20ns 30us"),
                    (
                        26,
                        ".print tran PRECISION=10 WIDTH=19 v(vout) {v(in)+1.0} v(1)",
                    ),
                    (29, ".initcond xinV1:MN1=2,0"),
                    (37, "XINV1 IN VOUT VDD 0 INVERTER"),
                    (156, ".END"),
                ],
            ),
            XyceExpectedFailureKind::Bug702MalformedInitcondFile => (
                "BUG702 empty-initcond",
                "empty-initcond.cir",
                152,
                vec![
                    (
                        14,
                        "MN1 OUT IN GND GND GND CMOSN w=4u  l=0.15u  AS=6p AD=6p PS=7u PD=7u ic=2,0 ",
                    ),
                    (19, ".INITCOND FILE noinits.dat"),
                    (24, ".tran 20ns 30us"),
                    (
                        25,
                        ".print tran PRECISION=10 WIDTH=19 v(vout) {v(in)+1.0} v(1)",
                    ),
                    (33, "XINV1 IN VOUT VDD 0 INVERTER"),
                    (152, ".END"),
                ],
            ),
            XyceExpectedFailureKind::Bug702MissingInitcondFile => (
                "BUG702 missing-initcond",
                "missing-initcond.cir",
                154,
                vec![
                    (21, ".initCOND file \"ic.dat\""),
                    (26, ".tran 20ns 30us"),
                    (
                        27,
                        ".print tran PRECISION=10 WIDTH=19 v(vout) {v(in)+1.0} v(1)",
                    ),
                    (35, "XINV1 IN VOUT VDD 0 INVERTER"),
                    (154, ".END"),
                ],
            ),
            _ => {
                return Err(format!(
                    "non-BUG702 failure kind passed to BUG702 observer: {kind:?}"
                ));
            }
        };
        Self::require_expected_failure_file_name(label, deck_path, file_name)?;
        Self::require_expected_failure_source_lines(label, source, line_count, &expected_lines)?;

        if matches!(
            kind,
            XyceExpectedFailureKind::Bug702DuplicateExternalInitcond
        ) {
            Self::validate_bug702_resource(
                family_dir,
                "initcond.dat",
                XYCE_BUG702_INITCOND_DATA_BYTES,
                XYCE_BUG702_INITCOND_DATA_BLAKE3,
            )?;
        }
        let expected_noinits =
            if matches!(kind, XyceExpectedFailureKind::Bug702MalformedInitcondFile) {
                Some(Self::validate_bug702_resource(
                    family_dir,
                    "noinits.dat",
                    XYCE_BUG702_NOINITS_DATA_BYTES,
                    XYCE_BUG702_NOINITS_DATA_BLAKE3,
                )?)
            } else {
                None
            };
        if matches!(kind, XyceExpectedFailureKind::Bug702MissingInitcondFile) {
            Self::require_bug702_missing_ic_dat(family_dir)?;
        }

        let error = Self::parse_netlist_with_expression_dialect_and_execution_dir(
            source,
            deck_path,
            ExpressionDialect::Xyce,
            Some(family_dir),
        )
        .expect_err("BUG702 expected-failure observer requires a parse/load failure");
        let expected_owner = Self::canonical_expected_failure_source_path(deck_path, label)?;
        match (kind, error) {
            (
                XyceExpectedFailureKind::Bug702DuplicateExternalInitcond,
                ParseError::DeviceInitialCondition(inner),
            )
            | (
                XyceExpectedFailureKind::Bug702DuplicateInlinedInitcond,
                ParseError::DeviceInitialCondition(inner),
            ) => {
                let DeviceInitialConditionError::DuplicateDirective { first, duplicate } = *inner
                else {
                    return Err(format!(
                        "{label} produced the wrong typed INITCOND failure: {inner:?}"
                    ));
                };
                let first_path = first
                    .path
                    .as_deref()
                    .ok_or_else(|| format!("{label} first directive has no source path"))?;
                let duplicate_path = duplicate
                    .path
                    .as_deref()
                    .ok_or_else(|| format!("{label} duplicate directive has no source path"))?;
                if first.line != 20
                    || duplicate.line != 29
                    || Self::canonical_expected_failure_source_path(first_path, label)?
                        != expected_owner
                    || Self::canonical_expected_failure_source_path(duplicate_path, label)?
                        != expected_owner
                {
                    return Err(format!(
                        "{label} duplicate INITCOND locations changed: first={first:?}, duplicate={duplicate:?}"
                    ));
                }
                let representation = if matches!(
                    kind,
                    XyceExpectedFailureKind::Bug702DuplicateExternalInitcond
                ) {
                    "EXTERNAL"
                } else {
                    "INLINE"
                };
                Ok(XyceExpectedFailureObservation {
                    stage: XyceExpectedFailureStage::NetlistParse,
                    category: XyceExpectedFailureCategory::DuplicateDeviceInitialCondition,
                    identifiers: vec![
                        representation.to_string(),
                        format!("{file_name}:20"),
                        format!("{file_name}:29"),
                    ],
                })
            }
            (
                XyceExpectedFailureKind::Bug702MissingInitcondFile,
                ParseError::DeviceInitialCondition(inner),
            ) => {
                let DeviceInitialConditionError::SourceUnavailable {
                    origin,
                    requested_path,
                } = *inner
                else {
                    return Err(format!(
                        "{label} produced the wrong typed INITCOND failure: {inner:?}"
                    ));
                };
                let origin_path = origin
                    .path
                    .as_deref()
                    .ok_or_else(|| format!("{label} directive has no source path"))?;
                if requested_path != "ic.dat"
                    || origin.line != 21
                    || Self::canonical_expected_failure_source_path(origin_path, label)?
                        != expected_owner
                {
                    return Err(format!(
                        "{label} missing-source observation changed: origin={origin:?}, requested={requested_path:?}"
                    ));
                }
                Ok(XyceExpectedFailureObservation {
                    stage: XyceExpectedFailureStage::ExternalDataLoad,
                    category: XyceExpectedFailureCategory::MissingDeviceInitialConditionFile,
                    identifiers: vec!["ic.dat".to_string(), format!("{file_name}:21")],
                })
            }
            (
                XyceExpectedFailureKind::Bug702MalformedInitcondFile,
                ParseError::DeviceInitialCondition(inner),
            ) => {
                let DeviceInitialConditionError::MalformedSource {
                    origin,
                    requested_path,
                    record_origin,
                    detail,
                } = *inner
                else {
                    return Err(format!(
                        "{label} produced the wrong typed INITCOND failure: {inner:?}"
                    ));
                };
                let origin_path = origin
                    .path
                    .as_deref()
                    .ok_or_else(|| format!("{label} directive has no source path"))?;
                let record_path = record_origin
                    .path
                    .as_deref()
                    .ok_or_else(|| format!("{label} malformed record has no source path"))?;
                if requested_path != "noinits.dat"
                    || origin.line != 19
                    || record_origin.line != 1
                    || detail != "source contains no device initial-condition records"
                    || Self::canonical_expected_failure_source_path(origin_path, label)?
                        != expected_owner
                    || Self::canonical_expected_failure_source_path(record_path, label)?
                        != expected_noinits.expect("malformed source path is pinned")
                {
                    return Err(format!(
                        "{label} malformed-source observation changed: origin={origin:?}, requested={requested_path:?}, record={record_origin:?}, detail={detail:?}"
                    ));
                }
                Ok(XyceExpectedFailureObservation {
                    stage: XyceExpectedFailureStage::ExternalDataLoad,
                    category: XyceExpectedFailureCategory::MalformedDeviceInitialConditionFile,
                    identifiers: vec![
                        "noinits.dat".to_string(),
                        format!("{file_name}:19"),
                        "noinits.dat:1".to_string(),
                    ],
                })
            }
            (_, error) => Err(format!(
                "{label} produced the wrong typed expected failure: {error:?}"
            )),
        }
    }

    pub(super) fn observe_issue455_duplicate_dc_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "ISSUE 455",
            source,
            11,
            &[
                (
                    2,
                    "* this test checks that V2 will trigger a useful error message",
                ),
                (4, "V2 1 0 dc 1.0 dc 0.0"),
                (5, "R2 1 0 1.0"),
                (7, ".OP"),
                (8, ".Print DC V(*)"),
            ],
        )?;
        Self::require_exact_syntax_failure(
            "ISSUE 455",
            source,
            deck_path,
            4,
            "Unexpected trailing token in source specification: DC",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::DuplicateDcSourceFunction,
            identifiers: vec!["V2".to_string(), "DC".to_string(), "line 4".to_string()],
        })
    }

    pub(super) fn observe_bug1578_invalid_device_type_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "BUG 1578",
            source,
            32,
            &[
                (
                    1,
                    "*This is a test to make sure Xyce pukes with an APPROPRIATE",
                ),
                (
                    7,
                    "* out the title line.  The title line begins with an N, so it is",
                ),
                (10, "Netlist to Test the Xyce Pulse Voltage Source Model"),
                (
                    12,
                    "* Tier No.:\t1                                           ",
                ),
                (28, "VPULSE 1 0 PULSE(0V 1V 0S 10US 10US 0.1US 20.1US)"),
                (29, "R 1 0 500"),
                (30, ".TRAN 1US 20.1US"),
                (31, ".PRINT TRAN V(1)"),
                (32, ".END"),
            ],
        )?;
        Self::require_exact_syntax_failure(
            "BUG 1578",
            source,
            deck_path,
            10,
            "Unknown element type: N",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::UnknownDeviceType,
            identifiers: vec![
                "NETLIST".to_string(),
                "N".to_string(),
                "line 10".to_string(),
            ],
        })
    }

    pub(super) fn observe_bug198_or_bug258_unrecognized_line_failure(
        source: &str,
        deck_path: &Path,
        kind: XyceExpectedFailureKind,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let label = match kind {
            XyceExpectedFailureKind::Bug198UnrecognizedLine => "BUG 198",
            XyceExpectedFailureKind::Bug258UnrecognizedLine => "BUG 258",
            _ => {
                return Err(format!(
                    "non-BUG 198/258 expected-failure kind passed to unrecognized-line observer: {kind:?}"
                ));
            }
        };
        Self::require_expected_failure_source_lines(
            label,
            source,
            41,
            &[
                (1, "N-Channel Mosfet Circuit "),
                (
                    2,
                    "**************************************************************",
                ),
                (
                    3,
                    "# Tier No.: 1                                               ",
                ),
                (
                    28,
                    "************************************************************** ",
                ),
                (29, "VDD 5 0 DC 18V "),
                (31, "R1 5 1 47MEGa"),
                (37, ".MODEL NFET NMOS(LEVEL=1 KP=0.5M VTO=2V)"),
                (38, ".DC VDD 18 18 1"),
                (39, ".PRINT DC V(3,2) V(1,2)"),
                (40, ".OPTIONS NONLIN in_forcing=0"),
                (41, ".END"),
            ],
        )?;
        Self::require_exact_syntax_failure(
            label,
            source,
            deck_path,
            3,
            "Expected identifier at start of line",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::InvalidNetlistLinePrefix,
            identifiers: vec!["#".to_string(), "line 3".to_string()],
        })
    }

    pub(super) fn observe_bug587_invalid_numeric_notation_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "BUG 587",
            source,
            81,
            &[
                (26, ".param secondary=10"),
                (27, ".param primary={13-secondary}"),
                (
                    38,
                    "* The broken code would separate value into two fields 2.0e+ and {primary}.",
                ),
                (43, "R1 1 2 2.0e+{primary}"),
                (75, "D1 3 0 DMOD"),
                (76, "VMON 2 3 0"),
                (77, ".MODEL DMOD D (IS=100FA)"),
                (78, ".DC VIN 5 5 1"),
                (79, ".PRINT DC I(VMON) V(3)"),
                (81, ".END"),
            ],
        )?;
        Self::require_exact_syntax_failure(
            "BUG 587",
            source,
            deck_path,
            43,
            "Invalid number '2.0e+' at line 1",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::InvalidNumericNotation,
            identifiers: vec![
                "R1".to_string(),
                "2.0e+".to_string(),
                "PRIMARY".to_string(),
                "line 43".to_string(),
            ],
        })
    }

    pub(super) fn observe_bug204_invalid_dc_sweep_arity(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "BUG 204",
            source,
            17,
            &[
                (9, "VIN 1 0 DC 5V"),
                (10, "R1 1 2 2K"),
                (11, "D1 3 0 DMOD"),
                (12, "VMON 2 3 0"),
                (13, ".MODEL DMOD D (IS=100FA)"),
                (14, ".DC VIN 5 5"),
                (15, ".PRINT DC V(1) I(VMON) V(3)"),
                (17, ".END"),
            ],
        )?;
        Self::require_exact_syntax_failure(
            "BUG 204",
            source,
            deck_path,
            14,
            ".DC linear sweep requires a step value",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::InvalidDcSweepArity,
            identifiers: vec!["VIN".to_string(), "STEP".to_string(), "line 14".to_string()],
        })
    }

    pub(super) fn observe_bug281_invalid_dc_sweep_arity(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "BUG 281",
            source,
            10,
            &[
                (2, "VIN 1 0 DC 5V"),
                (3, "R1 1 2 2K"),
                (4, "D1 3 0 DMOD"),
                (5, "VMON 2 3 0"),
                (6, ".MODEL DMOD D (IS=100FA)"),
                (7, ".DC VIN 5 5"),
                (8, ".PRINT DC I(VMON) V(3)"),
                (10, ".END"),
            ],
        )?;
        Self::require_exact_syntax_failure(
            "BUG 281",
            source,
            deck_path,
            7,
            ".DC linear sweep requires a step value",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::InvalidDcSweepArity,
            identifiers: vec!["VIN".to_string(), "STEP".to_string(), "line 7".to_string()],
        })
    }

    pub(super) fn observe_bug401_bad_device_line_failure(
        &self,
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "BUG 401 bad-device-line",
            source,
            11,
            &[
                (
                    5,
                    "An example of perfect nonsense that is too easy to get from a user!",
                ),
                (6, "R1 1 0 1meg"),
                (7, "V1 1 0 DC 1V"),
                (8, ".DC V1 1 1 1 "),
                (9, ".PRINT DC V(1)"),
                (10, ".end"),
            ],
        )?;
        self.observe_bug401_unknown_xspice_model_failure(
            "BUG 401 bad-device-line",
            source,
            deck_path,
            "AN",
            &[
                "EXAMPLE", "OF", "PERFECT", "NONSENSE", "THAT", "IS", "TOO", "EASY", "TO", "GET",
                "FROM", "A",
            ],
            5,
        )
    }

    pub(super) fn observe_bug401_extra_space_failure(
        &self,
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "BUG 401 extra-space",
            source,
            13,
            &[
                (2, "APerfect nonsense that is too easy to get from a user!"),
                (8, "R1 1 0 1meg"),
                (9, "V1 1 0 DC 1V"),
                (10, ".DC V1 1 1 1 "),
                (11, ".PRINT DC V(1)"),
                (12, ".end"),
            ],
        )?;
        self.observe_bug401_unknown_xspice_model_failure(
            "BUG 401 extra-space",
            source,
            deck_path,
            "APERFECT",
            &[
                "NONSENSE", "THAT", "IS", "TOO", "EASY", "TO", "GET", "FROM", "A",
            ],
            2,
        )
    }

    pub(super) fn observe_bug401_unknown_xspice_model_failure(
        &self,
        label: &str,
        source: &str,
        deck_path: &Path,
        expected_element: &str,
        expected_ports: &[&str],
        expected_line: usize,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let netlist = Self::parse_xyce_netlist(source, deck_path).map_err(|error| {
            format!("{label} must parse before XSPICE model resolution: {error}")
        })?;
        if !netlist.title.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || netlist.elements.len() != 3
        {
            return Err(format!(
                "{label} parsed structure changed: title={:?}, diagnostics={:?}, models={}, subcircuits={}, elements={}",
                netlist.title,
                netlist.diagnostics,
                netlist.models.len(),
                netlist.subcircuits.len(),
                netlist.elements.len()
            ));
        }
        let element_names = netlist
            .elements
            .iter()
            .map(|element| element.name.as_str())
            .collect::<Vec<_>>();
        if element_names != [expected_element, "R1", "V1"] {
            return Err(format!(
                "{label} element order changed: expected [{expected_element:?}, \"R1\", \"V1\"], got {element_names:?}"
            ));
        }
        let ElementKind::Xspice {
            model,
            pspice_u_timing,
            ports,
            params,
            expr_params,
            string_params,
            string_expr_params,
            string_vector_params,
            string_vector_expr_params,
            real_vector_params,
            real_vector_expr_params,
        } = &netlist.elements[0].kind
        else {
            return Err(format!(
                "{label} malformed A-prefixed line no longer parses as an XSPICE instance"
            ));
        };
        let analog_ports = ports
            .iter()
            .map(|port| match port {
                rspice_core::netlist::XspicePort::Analog(node) => Some(node.as_str()),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| format!("{label} acquired a non-analog XSPICE port"))?;
        if model != "USER!"
            || analog_ports != expected_ports
            || pspice_u_timing.is_some()
            || !params.is_empty()
            || !expr_params.is_empty()
            || !string_params.is_empty()
            || !string_expr_params.is_empty()
            || !string_vector_params.is_empty()
            || !string_vector_expr_params.is_empty()
            || !real_vector_params.is_empty()
            || !real_vector_expr_params.is_empty()
        {
            return Err(format!(
                "{label} XSPICE interpretation changed: model={model:?}, ports={analog_ports:?}"
            ));
        }
        match self.create_xyce_engine().build_circuit(&netlist) {
            Err(SimulationError::Circuit(message))
                if message.contains(&format!(
                    "Failed to resolve XSPICE model 'USER!' for element {expected_element}"
                )) && message.contains("Unknown XSPICE model 'USER!'") => {}
            Err(error) => {
                return Err(format!(
                    "{label} produced the wrong typed build failure: {error:?}"
                ));
            }
            Ok(_) => {
                return Err(format!(
                    "{label} unexpectedly built; the unknown XSPICE model condition is absent"
                ));
            }
        }
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::CircuitBuild,
            category: XyceExpectedFailureCategory::UnknownXspiceModel,
            identifiers: vec![
                expected_element.to_string(),
                "USER!".to_string(),
                format!("line {expected_line}"),
            ],
        })
    }

    pub(super) fn observe_bug401_worse_device_line_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "BUG 401 worse-device-line",
            source,
            12,
            &[
                (
                    6,
                    "Really good example of perfect nonsense that is too easy to get from a user!",
                ),
                (7, "R1 1 0 1meg"),
                (8, "V1 1 0 DC 1V"),
                (9, ".DC V1 1 1 1 "),
                (10, ".PRINT DC V(1)"),
                (11, ".end"),
            ],
        )?;
        Self::require_exact_syntax_failure(
            "BUG 401 worse-device-line",
            source,
            deck_path,
            6,
            "Unexpected trailing token in resistor specification: PERFECT",
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::MalformedResistorSpecification,
            identifiers: vec![
                "REALLY".to_string(),
                "PERFECT".to_string(),
                "line 6".to_string(),
            ],
        })
    }

    pub(super) fn observe_bug701_duplicate_toplevel_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "BUG 701 dup-toplevel",
            source,
            14,
            &[
                (1, "dup-toplevel.cir"),
                (5, "V1 22 0 4"),
                (6, "V1 1 0 5"),
                (8, "R1 1 22 1"),
                (9, "R2 22 0 1"),
                (11, ".dc V1 7 7 1"),
                (12, ".print dc V(1) V(22)"),
            ],
        )?;
        Self::require_exact_duplicate_device_failure(
            "BUG 701 dup-toplevel",
            source,
            deck_path,
            "V1",
            "TOP_LEVEL",
            5,
            6,
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::DuplicateDeviceName,
            identifiers: vec![
                "V1".to_string(),
                "TOP_LEVEL".to_string(),
                "line 5".to_string(),
                "line 6".to_string(),
            ],
        })
    }

    pub(super) fn observe_bug701_duplicate_subcircuit_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        Self::require_expected_failure_source_lines(
            "BUG 701 dup-subcircuit",
            source,
            19,
            &[
                (1, "dup-subcircuit.cir"),
                (6, ".subckt rNodes a b "),
                (7, "R1 a b 1"),
                (8, "R1 b 0 1"),
                (9, ".ends"),
                (11, "V1 22 0 4"),
                (12, "V2 1 0 5"),
                (14, ".dc V1 7 7 1"),
                (15, ".print dc V(1) V(22)"),
                (17, "XvNodes 1 22 rNodes"),
                (19, ".end"),
            ],
        )?;
        Self::require_exact_duplicate_device_failure(
            "BUG 701 dup-subcircuit",
            source,
            deck_path,
            "R1",
            "SUBCIRCUIT:RNODES",
            7,
            8,
        )?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::DuplicateDeviceName,
            identifiers: vec![
                "R1".to_string(),
                "SUBCIRCUIT:RNODES".to_string(),
                "XVNODES:R1".to_string(),
                "line 7".to_string(),
                "line 8".to_string(),
            ],
        })
    }

    pub(super) fn observe_bug769_parameter_probe_failure(
        source: &str,
        deck_path: &Path,
        kind: XyceExpectedFailureKind,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let (label, parameter_line, diagnostic, category, probe) = match kind {
            XyceExpectedFailureKind::Bug769ParameterNodeVoltage => (
                "BUG 769 node-voltage parameter",
                ".param RVAL={76K+v(3)}",
                "Node Voltage may not be used in parameter expression (RVAL): V(3)",
                XyceExpectedFailureCategory::ParameterNodeVoltage,
                "V(3)",
            ),
            XyceExpectedFailureKind::Bug769ParameterDeviceCurrent => (
                "BUG 769 device-current parameter",
                ".param RVAL={76K+i(v2)}",
                "Device Current may not be used in parameter expression (RVAL): I(V2)",
                XyceExpectedFailureCategory::ParameterDeviceCurrent,
                "I(V2)",
            ),
            XyceExpectedFailureKind::Bug769ParameterLeadCurrent => (
                "BUG 769 lead-current parameter",
                ".param RVAL={76K+i(c2)}",
                "Lead Current may not be used in parameter expression (RVAL): I(C2)",
                XyceExpectedFailureCategory::ParameterLeadCurrent,
                "I(C2)",
            ),
            _ => {
                return Err(format!(
                    "non-BUG 769 expected-failure kind passed to BUG 769 observer: {kind:?}"
                ));
            }
        };
        Self::require_bug769_one_line_delta_family(deck_path)?;
        Self::require_expected_failure_source_lines(
            label,
            source,
            82,
            &[
                (1, "UA555 Timer Circuit"),
                (66, ".ENDS"),
                (69, parameter_line),
                (71, "C1 1 0 2140P"),
                (72, "R2 3 1 {RVAL}"),
                (73, "C2 5 0 .01U"),
                (74, "X2 6 4 3 5 1 1 3 UA555"),
                (75, "V2 6 0 PULSE(0 15 0U .1U .1U 900U 1M)"),
                (78, ".TRAN 1U 5M"),
                (79, ".PRINT TRAN V(4)  V(1)  V(6)"),
                (81, ".END"),
                (82, ""),
            ],
        )?;
        Self::require_exact_syntax_failure(label, source, deck_path, 69, diagnostic)?;
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category,
            identifiers: vec!["RVAL".to_string(), probe.to_string(), "line 69".to_string()],
        })
    }

    pub(super) fn observe_bug67_behavioral_expression_failure(
        &self,
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let netlist = Self::parse_xyce_netlist(source, deck_path)
            .map_err(|err| format!("BUG 67 must parse before behavioral compilation: {err}"))?;
        if !netlist.diagnostics.is_empty() {
            return Err(format!(
                "BUG 67 parse produced unexpected diagnostics: {:?}",
                netlist.diagnostics
            ));
        }
        if netlist.elements.len() != 15
            || netlist.subcircuits.len() != 1
            || !netlist.subcircuits[0].name.eq_ignore_ascii_case("LM124N")
        {
            return Err(
                "BUG 67 must retain 15 top-level elements and one LM124N subcircuit".to_string(),
            );
        }
        let top_instances = netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::Subcircuit { subckt_name, .. }
                    if subckt_name.eq_ignore_ascii_case("LM124N") =>
                {
                    Some(element.name.to_ascii_uppercase())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if top_instances != ["X1", "X2"] {
            return Err(format!(
                "BUG 67 must instantiate LM124N in exact X1, X2 order, got {top_instances:?}"
            ));
        }
        let malformed = netlist.subcircuits[0]
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::BehavioralCurrent { expression, .. }
                    if expression.eq_ignore_ascii_case("POLY I(V6) 300u 1") =>
                {
                    Some(element.name.to_ascii_uppercase())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if malformed != ["B6"] {
            return Err(format!(
                "BUG 67 must retain exactly malformed LM124N B6 expression 'POLY I(V6) 300u 1', got {malformed:?}"
            ));
        }
        let tran = Self::single_tran_analysis(&netlist)?;
        if (tran.step - 1.0e-6).abs() > 1.0e-21
            || (tran.stop - 1.0e-3).abs() > 1.0e-18
            || tran.start != Some(0.0)
            || tran
                .max_step
                .is_none_or(|max_step| (max_step - 5.0e-6).abs() > 1.0e-20)
            || tran.uic
        {
            return Err(format!("BUG 67 .TRAN tuple changed: {tran:?}"));
        }
        let print = Self::single_tran_print_output_request(source)?;
        if print.format.is_some() || print.file.is_some() || print.probes != ["V(7)", "V(8)"] {
            return Err(format!(
                "BUG 67 ordered .PRINT TRAN contract changed: {print:?}"
            ));
        }
        let flattened = rspice_core::netlist::flatten_netlist_with_models(&netlist)
            .map_err(|err| format!("BUG 67 must flatten before behavioral compilation: {err}"))?;
        let flattened_b6 = flattened
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::BehavioralCurrent { expression, .. }
                    if element.name.ends_with(".B6") =>
                {
                    Some((element.name.clone(), expression.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if flattened_b6
            != [
                ("X1.B6".to_string(), "POLY I(X1.V6) 300u 1".to_string()),
                ("X2.B6".to_string(), "POLY I(X2.V6) 300u 1".to_string()),
            ]
        {
            return Err(format!(
                "BUG 67 flattened malformed B6 instances changed: {flattened_b6:?}"
            ));
        }

        match self.create_xyce_engine().build_circuit(&netlist) {
            Err(SimulationError::Circuit(message))
                if message.contains("Invalid behavioral expression")
                    && message.contains("POLY I(X1.V6) 300u 1")
                    && message.contains("Unknown identifier 'POLY'")
                    && message.contains("Ident(\"I\")") =>
            {
                Ok(XyceExpectedFailureObservation {
                    stage: XyceExpectedFailureStage::CircuitBuild,
                    category: XyceExpectedFailureCategory::BehavioralExpressionSyntax,
                    identifiers: vec!["X1.B6".to_string(), "X1.V6".to_string()],
                })
            }
            Err(error) => Err(format!(
                "BUG 67 produced the wrong typed circuit-build failure: {error}"
            )),
            Ok(_) => Err(
                "BUG 67 circuit built successfully; expected malformed behavioral expression"
                    .to_string(),
            ),
        }
    }

    pub(super) fn observe_bug671_invalid_pwl_failure(
        &self,
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let netlist = Self::parse_xyce_netlist(source, deck_path)
            .map_err(|err| format!("BUG 671 must parse before PWL file loading: {err}"))?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 2
            || !netlist.subcircuits.is_empty()
        {
            return Err(
                "BUG 671 must parse diagnostic-free as exactly one source and one resistor"
                    .to_string(),
            );
        }
        let source_element = &netlist.elements[0];
        let ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::PwlFile {
            path,
            time_scale,
            value_scale,
            time_offset,
            value_offset,
            delay,
            repeat_from,
        }) = &source_element.kind
        else {
            return Err("BUG 671 VPWL must remain a file-backed PWL voltage source".to_string());
        };
        if !source_element.name.eq_ignore_ascii_case("VPWL")
            || source_element.nodes != ["1", "0"]
            || time_scale.to_bits() != 1.0_f64.to_bits()
            || value_scale.to_bits() != 1.0_f64.to_bits()
            || time_offset.to_bits() != 0.0_f64.to_bits()
            || value_offset.to_bits() != 0.0_f64.to_bits()
            || delay.to_bits() != 0.0_f64.to_bits()
            || repeat_from.is_some()
        {
            return Err(format!(
                "BUG 671 VPWL source shape changed: {} {:?}",
                source_element.name, source_element.nodes
            ));
        }
        let resistor = &netlist.elements[1];
        if !resistor.name.eq_ignore_ascii_case("R")
            || resistor.nodes != ["1", "0"]
            || !matches!(&resistor.kind, ElementKind::Resistor { value, .. } if value.to_bits() == 500.0_f64.to_bits())
        {
            return Err("BUG 671 must retain its 500-ohm shunt resistor".to_string());
        }
        let tran = Self::single_tran_analysis(&netlist)?;
        if tran.step.to_bits() != 0.01_f64.to_bits()
            || tran.stop.to_bits() != 11.0_f64.to_bits()
            || tran.start.is_some()
            || tran.max_step.is_some()
            || tran.uic
        {
            return Err(format!("BUG 671 .TRAN tuple changed: {tran:?}"));
        }
        let print = Self::single_tran_print_output_request(source)?;
        if print.format.is_some() || print.file.is_some() || print.probes != ["V(1)"] {
            return Err(format!(
                "BUG 671 ordered .PRINT TRAN contract changed: {print:?}"
            ));
        }
        let sibling = deck_path
            .parent()
            .ok_or_else(|| "BUG 671 deck has no parent directory".to_string())?
            .join("vpwl-word.csv");
        let resolved = Path::new(path).canonicalize().map_err(|err| {
            format!(
                "BUG 671 parsed PWL path '{}' cannot be canonicalized: {err}",
                path
            )
        })?;
        let expected_resolved = sibling.canonicalize().map_err(|err| {
            format!(
                "BUG 671 exact sibling fixture {} is missing: {err}",
                sibling.display()
            )
        })?;
        if resolved != expected_resolved {
            return Err(format!(
                "BUG 671 PWL path resolved to {}, expected exact sibling {}",
                resolved.display(),
                expected_resolved.display()
            ));
        }
        Self::validate_bug671_binary_fixture(&sibling)?;

        match rspice_core::device::pwl_file::load_pwl_file(&sibling) {
            Err(rspice_core::device::pwl_file::PwlFileError::IoError(error))
                if error.kind() == std::io::ErrorKind::InvalidData => {}
            Err(error) => {
                return Err(format!(
                    "BUG 671 direct PWL loader produced the wrong typed failure: {error}"
                ));
            }
            Ok(_) => {
                return Err(
                    "BUG 671 binary fixture loaded successfully as PWL data; expected invalid encoding"
                        .to_string(),
                );
            }
        }
        match self.create_xyce_engine().build_circuit(&netlist) {
            Err(SimulationError::Circuit(message))
                if message.contains("source 'VPWL'")
                    && message.contains("vpwl-word.csv")
                    && message.contains("failed to load PWL file")
                    && message.contains("valid UTF-8") =>
            {
                Ok(XyceExpectedFailureObservation {
                    stage: XyceExpectedFailureStage::ExternalDataLoad,
                    category: XyceExpectedFailureCategory::InvalidPwlFileEncoding,
                    identifiers: vec!["VPWL".to_string(), "vpwl-word.csv".to_string()],
                })
            }
            Err(error) => Err(format!(
                "BUG 671 produced the wrong typed circuit-build/loader failure: {error}"
            )),
            Ok(_) => Err(
                "BUG 671 circuit built successfully; expected its external PWL load to fail"
                    .to_string(),
            ),
        }
    }

    pub(super) fn observe_bug726_adjacent_coupling_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let physical_line = source
            .lines()
            .nth(12)
            .ok_or_else(|| "BUG 726 source has fewer than 13 physical lines".to_string())?;
        if physical_line != "K1 L1 L2 0.75 K2 L1 L3 0.8" {
            return Err(format!(
                "BUG 726 physical line 13 changed: expected concatenated K1/K2 statement, got {physical_line:?}"
            ));
        }
        for statement in [
            "VS 1 0 SIN(0 169.7 60HZ)",
            "R1 1 2 1K",
            "L1 2 0 1mH",
            "R2 3 0 1K",
            "L2 3 0 1mH",
            "R3 4 0 1K",
            "L3 4 0 1mH",
            ".TRAN 100US 25MS",
            ".OPTIONS TIMEINT METHOD=2",
            ".PRINT TRAN I(VS) V(2) V(3) V(4)",
        ] {
            if !source.lines().any(|line| line == statement) {
                return Err(format!(
                    "BUG 726 required statement changed or moved: {statement}"
                ));
            }
        }

        match Self::parse_xyce_netlist(source, deck_path) {
            Err(ParseError::Syntax { line, message })
                if line == 13
                    && message == "Unexpected trailing token in coupling specification: K2" =>
            {
                Ok(XyceExpectedFailureObservation {
                    stage: XyceExpectedFailureStage::NetlistParse,
                    category: XyceExpectedFailureCategory::AdjacentCouplingSyntax,
                    identifiers: vec!["K1".to_string(), "K2".to_string(), "line 13".to_string()],
                })
            }
            Err(error) => Err(format!(
                "BUG 726 produced the wrong typed parser failure: {error}"
            )),
            Ok(_) => Err(
                "BUG 726 parsed successfully; expected the adjacent K2 token to be rejected"
                    .to_string(),
            ),
        }
    }

    pub(super) fn observe_bug744_dc_operating_point_failure(
        &self,
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let netlist = Self::parse_xyce_netlist(source, deck_path)
            .map_err(|err| format!("BUG 744 must parse before DC operating point: {err}"))?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 3
            || !netlist.subcircuits.is_empty()
        {
            return Err(
                "BUG 744 must parse diagnostic-free as exactly two sources and one resistor"
                    .to_string(),
            );
        }
        let expected_sources: [(&str, Value); 2] = [("Vsrc1", 5.0), ("Vsrc2", 2.0)];
        for (element, (name, value)) in netlist.elements[..2].iter().zip(expected_sources) {
            if !element.name.eq_ignore_ascii_case(name)
                || element.nodes != ["1", "0"]
                || !matches!(&element.kind, ElementKind::VoltageSource(spec) if extract_dc_value(spec).to_bits() == value.to_bits())
            {
                return Err(format!(
                    "BUG 744 conflicting source {name}={value} V topology changed"
                ));
            }
        }
        let resistor = &netlist.elements[2];
        if !resistor.name.eq_ignore_ascii_case("Rgrn")
            || resistor.nodes != ["1", "0"]
            || !matches!(&resistor.kind, ElementKind::Resistor { value, .. } if value.to_bits() == 1.0e6_f64.to_bits())
        {
            return Err("BUG 744 must retain its 1-megohm node-1 shunt".to_string());
        }
        let tran = Self::single_tran_analysis(&netlist)?;
        if tran.step.to_bits() != 0.0_f64.to_bits()
            || tran.stop.to_bits() != 1.0_f64.to_bits()
            || tran.start.is_some()
            || tran.max_step.is_some()
            || tran.uic
        {
            return Err(format!("BUG 744 .TRAN tuple changed: {tran:?}"));
        }
        let print = Self::single_tran_print_output_request(source)?;
        if print.format.is_some() || print.file.is_some() || print.probes != ["v(1)"] {
            return Err(format!(
                "BUG 744 ordered .PRINT TRAN contract changed: {print:?}"
            ));
        }
        let engine = self.create_xyce_engine();
        engine
            .build_circuit(&netlist)
            .map_err(|error| format!("BUG 744 must build before DC operating point: {error}"))?;
        match engine.run_dc_op(&netlist) {
            Err(SimulationError::Circuit(message))
                if message.contains("matrix is singular")
                    && message.contains("ideal voltage sources")
                    && message.contains("duplicate constraints") =>
            {
                Ok(XyceExpectedFailureObservation {
                    stage: XyceExpectedFailureStage::DcOperatingPoint,
                    category: XyceExpectedFailureCategory::ConflictingIdealVoltageConstraints,
                    identifiers: vec!["Vsrc1".to_string(), "Vsrc2".to_string(), "1".to_string()],
                })
            }
            Err(error) => Err(format!(
                "BUG 744 produced the wrong typed DC operating-point failure: {error}"
            )),
            Ok(_) => Err(
                "BUG 744 DC operating point succeeded; expected conflicting ideal constraints"
                    .to_string(),
            ),
        }
    }

    pub(super) fn observe_bug75_undefined_mutual_inductor_reference_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<XyceExpectedFailureObservation, String> {
        const LABEL: &str = "BUG 75";
        Self::require_expected_failure_file_name(LABEL, deck_path, "bug75.cir")?;
        Self::require_expected_failure_source_lines(
            LABEL,
            source,
            15,
            &[
                (
                    1,
                    "Test error message capability when a mutual inductor references an undefined",
                ),
                (2, "*inductor"),
                (4, "R1 1 2 1"),
                (5, "L1 2 0 1"),
                (6, "*L2 2 3 1"),
                (7, "L3 2 0 1"),
                (8, "V1 1 0 DC 1"),
                (10, "K1 L1 L3 0.1"),
                (11, "K2 L1 L3 0"),
                (12, "K3 L1 L2 0"),
                (14, ".DC V1 1 1 0.1"),
                (15, ".end"),
            ],
        )?;

        let error = match Self::parse_xyce_netlist(source, deck_path) {
            Err(ParseError::UndefinedMutualInductorReference(error)) => error,
            Err(error) => {
                return Err(format!(
                    "{LABEL} produced the wrong typed parser failure: {error:?}"
                ));
            }
            Ok(_) => {
                return Err(format!(
                    "{LABEL} unexpectedly parsed; the undefined L2 reference is absent"
                ));
            }
        };

        let expected_path = deck_path.canonicalize().map_err(|error| {
            format!(
                "failed to canonicalize {LABEL} deck {}: {error}",
                deck_path.display()
            )
        })?;
        let origin_path = error
            .origin
            .path
            .as_deref()
            .ok_or_else(|| format!("{LABEL} typed failure lost its source path"))?
            .canonicalize()
            .map_err(|error| format!("failed to canonicalize {LABEL} error origin: {error}"))?;
        if origin_path != expected_path
            || error.origin.line != 12
            || error.authored_coupling_name != "K3"
            || error.canonical_coupling_name != "K3"
            || error.qualified_coupling_name != "K3"
            || error.authored_inductor_name != "L2"
            || error.canonical_inductor_name != "L2"
            || error.qualified_inductor_name != "L2"
            || error.scope_name.is_some()
            || error.reference_position != 2
            || error.to_string() != "Undefined inductor L2 in mutual inductor K3 definition."
        {
            return Err(format!(
                "{LABEL} typed undefined-mutual-inductor payload changed: {error:?}"
            ));
        }

        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::UndefinedMutualInductorReference,
            identifiers: vec![
                error.authored_coupling_name.clone(),
                error.canonical_coupling_name.clone(),
                error.qualified_coupling_name.clone(),
                error.authored_inductor_name.clone(),
                error.canonical_inductor_name.clone(),
                error.qualified_inductor_name.clone(),
                "TOP_LEVEL".to_string(),
                error.reference_position.to_string(),
                format!("line {}", error.origin.line),
            ],
        })
    }

    pub(super) fn observe_undefined_output_symbols_failure(
        source: &str,
        deck_path: &Path,
        kind: XyceExpectedFailureKind,
    ) -> Result<XyceExpectedFailureObservation, String> {
        let (label, file_name, line_count, required_lines): (&str, &str, usize, &[(usize, &str)]) =
            match kind {
                XyceExpectedFailureKind::Bug1148UndefinedPrintNode => (
                    "BUG 1148",
                    "bug_1148.cir",
                    6,
                    &[(2, "v1 1 0 1"), (5, ".print tran V(2)"), (6, ".end")],
                ),
                XyceExpectedFailureKind::Bug40UndefinedPrintNode => (
                    "BUG 40",
                    "bug_40.cir",
                    41,
                    &[
                        (29, "VDD 5 0 DC 18V "),
                        (39, ".PRINT DC V(bad) V(3,2) V(1,2)"),
                        (41, ".END"),
                    ],
                ),
                XyceExpectedFailureKind::Bug718InvalidPrintNodes => (
                    "BUG 718 invalid nodes",
                    "invalidNodes.cir",
                    22,
                    &[
                        (10, ".PRINT AC "),
                        (11, "+ {V(bogo1)} {V(bogo2,GND)} "),
                        (
                            13,
                            "+ {VR(bogo7)} {VI(bogo8)} {VP(bogo9)} {VM(bogo9)} {VDB(bogo10)}",
                        ),
                        (22, ".END"),
                    ],
                ),
                XyceExpectedFailureKind::MessagePrintBadNodeName => (
                    "Message Print bad node name",
                    "bad_nodename.cir",
                    9,
                    &[
                        (
                            8,
                            ".print ac V(A,C) VM(D,A) VP(A,B) VDB(A,B) VR(A,B) VI(A,B)",
                        ),
                        (9, ".end"),
                    ],
                ),
                XyceExpectedFailureKind::MessagePrintBadVariable => (
                    "Message Print bad variable",
                    "bad_variable.cir",
                    9,
                    &[
                        (
                            8,
                            ".print ac V(A,C) VM(D,B) VP(A,B) VDB(A,B) VR(A,B) VI(A,B) VQ(A,B)",
                        ),
                        (9, ".end"),
                    ],
                ),
                XyceExpectedFailureKind::LeadCurrentsInvalidDevice => (
                    "lead currents invalid device",
                    "lead_for_invalid_device.cir",
                    22,
                    &[
                        (14, "I1 1 0 sin(0 1 1KHz)"),
                        (20, ".PRINT TRAN V(2) I(RBogo)"),
                        (22, ".end"),
                    ],
                ),
                XyceExpectedFailureKind::MeasureInvalidNodes => (
                    "MEASURE invalid nodes",
                    "invalid_nodes.cir",
                    22,
                    &[
                        (14, ".MEASURE TRAN BOGONODEV MAX V(bogoNode)"),
                        (15, ".MEASURE TRAN BOGONODEN MAX N(missingNode)"),
                        (20, ".MEASURE TRAN NOREPLACEGROUND MAX V(GND)"),
                        (22, ".END"),
                    ],
                ),
                XyceExpectedFailureKind::FourierBadLine3OutputSymbols => (
                    "FOURIER bad line 3",
                    "bad_dot_four_line3.cir",
                    25,
                    &[
                        (
                            22,
                            ".FOUR 1KHZ I(BogoDevice1) P(BogoDevice2) W(BogoDevice3) V(2) N(3) V(GND)",
                        ),
                        (24, ".END"),
                    ],
                ),
                _ => {
                    return Err(format!(
                        "{} is not an output-symbol expected failure",
                        kind.record()
                    ));
                }
            };
        Self::require_expected_failure_file_name(label, deck_path, file_name)?;
        Self::require_expected_failure_source_lines(label, source, line_count, required_lines)?;

        // Parsing and semantic output validation are intentionally separate.
        // This proves each corpus record reaches the production post-parse
        // validator instead of being recognized by a parser/path special case.
        let netlist = Self::parse_xyce_netlist(source, deck_path).map_err(|error| {
            format!("{label} failed before output-symbol validation: {error:?}")
        })?;
        let error = match validate_output_symbols(&netlist) {
            Err(ParseError::OutputSymbolValidation(error)) => error,
            Err(error) => {
                return Err(format!(
                    "{label} produced the wrong typed validation failure: {error:?}"
                ));
            }
            Ok(()) => {
                return Err(format!(
                    "{label} unexpectedly passed output-symbol validation"
                ));
            }
        };
        let expected = kind
            .expected_output_symbols()
            .ok_or_else(|| format!("{label} has no typed output-symbol contract"))?;
        if error.unresolved.len() != expected.len() {
            return Err(format!(
                "{label} unresolved output-symbol count changed: expected {}, got {}: {:?}",
                expected.len(),
                error.unresolved.len(),
                error.unresolved
            ));
        }
        let expected_path = deck_path
            .canonicalize()
            .map_err(|error| format!("failed to canonicalize {label} deck: {error}"))?;
        for (index, (actual, expected)) in error.unresolved.iter().zip(expected).enumerate() {
            let actual_path = actual
                .origin
                .path
                .as_deref()
                .ok_or_else(|| format!("{label} unresolved item {index} lost its source path"))?
                .canonicalize()
                .map_err(|error| {
                    format!("failed to canonicalize {label} item {index} origin: {error}")
                })?;
            if actual.directive != expected.directive
                || actual.operator != expected.operator
                || actual.symbol != expected.symbol
                || actual.kind != expected.kind
                || actual.origin.line != expected.line
                || actual_path != expected_path
                || actual
                    .origin
                    .path
                    .as_deref()
                    .and_then(Path::file_name)
                    .and_then(|name| name.to_str())
                    != Some(expected.file_name)
            {
                return Err(format!(
                    "{label} unresolved output-symbol item {index} changed: expected {expected:?}, got {actual:?}"
                ));
            }
        }
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::NetlistParse,
            category: XyceExpectedFailureCategory::UndefinedOutputSymbols,
            identifiers: expected
                .iter()
                .copied()
                .map(XyceExpectedOutputSymbol::identifier)
                .collect(),
        })
    }
}
