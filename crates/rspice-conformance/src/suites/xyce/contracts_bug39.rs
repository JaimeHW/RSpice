use super::*;
use std::fmt::Write as _;

impl XyceTestRunner {
    pub(super) fn bug39_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG39_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG39_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG39_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug39_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug39_historical_oracle_provenance_records();
        let provenance_hash = blake3::hash(records.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let required_records = records
            .iter()
            .filter(|record| !record.contains("\tNetlists/Certification_Tests/BUG_39_SON/README\t"))
            .cloned()
            .collect::<Vec<_>>();
        let required_stream = required_records.join("\n");
        let required_sha256 = format!("{:x}", Sha256::digest(required_stream.as_bytes()));
        let required_blake3 = blake3::hash(required_stream.as_bytes())
            .to_hex()
            .to_string();
        if required_records.len() != XYCE_BUG39_REQUIRED_ORACLE_RECORD_COUNT
            || required_stream.len() != XYCE_BUG39_REQUIRED_ORACLE_BYTES
            || required_sha256 != XYCE_BUG39_REQUIRED_ORACLE_SHA256
            || required_blake3 != XYCE_BUG39_REQUIRED_ORACLE_BLAKE3
        {
            return Err(format!(
                "BUG_39_SON required Release-7.10 anchors/wrappers/exclude/Manifest provenance changed: records={}/{}/{required_sha256}/{required_blake3}",
                required_records.len(),
                required_stream.len()
            ));
        }
        if records.len() != XYCE_BUG39_HISTORICAL_ORACLE_RECORD_COUNT
            || provenance_hash != XYCE_BUG39_HISTORICAL_ORACLE_BLAKE3
        {
            return Err(format!(
                "BUG_39_SON Release-7.10 required oracle plus README provenance changed: records={}/{provenance_hash}",
                records.len()
            ));
        }
        if UPSTREAM_EXCLUSIONS_SOURCE_COMMIT != XYCE_BUG39_PRETRIM_COMMIT
            || UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE
                != "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d"
        {
            return Err("BUG_39_SON pre-trim manifest commit/tree provenance changed".to_string());
        }
        Ok(())
    }

    pub(super) fn bug39_gaussian_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug39GaussianContract, String>> {
        let role = XyceBug39GaussianRole::for_record(&deck.relative_path)?;
        Some((|| {
            const LABEL: &str = "BUG_39_SON generated Gaussian mean/sigma family";
            if deck.section != XyceDeckSection::Netlists
                || Self::normalize_manifest_key(&self.relative_key(&deck.path))
                    != Self::normalize_manifest_key(&deck.relative_path)
            {
                return Err(format!(
                    "recognized {LABEL} record '{}' is not backed by its exact Netlists path",
                    deck.relative_path
                ));
            }
            let anchor_path = self.root.join(role.path());
            if !Self::same_path(&deck.path, &anchor_path) {
                return Err(format!(
                    "recognized {LABEL} role {role:?} is not backed by its canonical path"
                ));
            }
            let contract = XyceBug39GaussianContract { anchor_path, role };
            self.validate_bug39_provenance(&contract)?;
            Ok(contract)
        })())
    }

    pub(super) fn validate_bug39_provenance(
        &self,
        contract: &XyceBug39GaussianContract,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG_39_SON generated Gaussian mean/sigma family";
        if !Self::same_path(&contract.anchor_path, &self.root.join(contract.role.path())) {
            return Err(format!("{LABEL} contract target changed"));
        }
        let family_dir = self.validate_bug39_retained_family_provenance()?;

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let wrapper_records = Self::load_upstream_wrapper_decks(&self.root);
        let mut candidates = Vec::with_capacity(XYCE_BUG39_CANDIDATE_COUNT);
        let mut candidate_content = Vec::with_capacity(XYCE_BUG39_CANDIDATE_COUNT);
        let mut owner_rows = Vec::with_capacity(XYCE_BUG39_CANDIDATE_COUNT);
        let mut historical_exclusion_rows = Vec::with_capacity(XYCE_BUG39_CANDIDATE_COUNT);
        for role in [
            XyceBug39GaussianRole::AgaussAbsolute,
            XyceBug39GaussianRole::GaussRelative,
        ] {
            let path = self.root.join(role.path());
            let relative = self.relative_key(&path);
            if Self::normalize_manifest_key(&relative) != role.record() {
                return Err(format!(
                    "{LABEL} canonical path '{}' is unavailable",
                    role.record()
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {LABEL} anchor '{relative}': {error}"))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let content_hash = blake3::hash(&canonical).to_hex().to_string();
            if !canonical.is_empty() || content_hash != XYCE_BUG39_EMPTY_CONTENT_BLAKE3 {
                return Err(format!(
                    "{LABEL} anchor '{relative}' must remain the exact zero-byte wrapper placeholder"
                ));
            }
            let key = Self::normalize_manifest_key(&relative);
            if !self.requires_upstream_wrapper(&relative) || !wrapper_records.contains(&key) {
                return Err(format!(
                    "{LABEL} anchor '{relative}' lost removed-wrapper ownership"
                ));
            }
            let exclusion = exclusions.get(&key).ok_or_else(|| {
                format!("{LABEL} anchor '{relative}' lost historical exclusion provenance")
            })?;
            if !matches!(
                &exclusion.disposition,
                XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                    if expected_contract == role.result_contract()
            ) || exclusion.source != XYCE_BUG39_HISTORICAL_EXCLUDE_PATH
            {
                return Err(format!(
                    "{LABEL} anchor '{relative}' lacks its exact independent qualification contract"
                ));
            }

            candidates.push(relative.clone());
            candidate_content.push(format!("{relative}\t{content_hash}"));
            owner_rows.push(format!("{relative}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}"));
            historical_exclusion_rows.push(format!(
                "{relative}\t{}\t{UPSTREAM_EXCLUDED_DISPOSITION}",
                exclusion.source
            ));
            self.reject_wrapper_output_artifacts(&path)
                .map_err(|error| format!("{LABEL} anchor '{relative}' {error}"))?;
            self.reject_wrapper_output_artifacts(&family_dir.join(role.generated_file_name()))
                .map_err(|error| {
                    format!(
                        "{LABEL} historical generated deck '{}' {error}",
                        role.generated_file_name()
                    )
                })?;
            Self::bug39_generated_source(role)?;
        }

        candidates.sort();
        candidate_content.sort();
        owner_rows.sort();
        historical_exclusion_rows.sort();
        let candidate_hash = blake3::hash(candidates.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(candidate_content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let owner_hash = blake3::hash(owner_rows.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let exclusion_hash = blake3::hash(historical_exclusion_rows.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if candidates.len() != XYCE_BUG39_CANDIDATE_COUNT
            || candidate_hash != XYCE_BUG39_CANDIDATE_BLAKE3
            || content_hash != XYCE_BUG39_CANDIDATE_CONTENT_BLAKE3
            || owner_hash != XYCE_BUG39_OWNER_MANIFEST_BLAKE3
            || exclusion_hash != XYCE_BUG39_HISTORICAL_EXCLUSION_BLAKE3
        {
            return Err(format!(
                "{LABEL} ownership provenance changed: candidates={}/{candidate_hash}/{content_hash}, owners={owner_hash}, exclusions={exclusion_hash}",
                candidates.len()
            ));
        }
        Ok(())
    }

    fn validate_bug39_retained_family_provenance(&self) -> Result<PathBuf, String> {
        const LABEL: &str = "BUG_39_SON retained family";
        Self::validate_bug39_historical_oracle_provenance()?;
        let family_dir = self.root.join("Netlists/Certification_Tests/BUG_39_SON");
        let family_metadata = fs::symlink_metadata(&family_dir)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if family_metadata.file_type().is_symlink() || !family_metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} requires a physical, non-symlinked family directory"
            ));
        }

        let expected_artifacts = XYCE_BUG39_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut retained_records = Vec::new();
        let mut physical_names = Vec::new();
        for entry in fs::read_dir(&family_dir)
            .map_err(|error| format!("failed to enumerate {LABEL}: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to read {LABEL} entry: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                format!(
                    "failed to inspect {LABEL} family entry '{}': {error}",
                    path.display()
                )
            })?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} family entry '{}' must be a regular non-symlink file",
                    path.display()
                ));
            }
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| format!("{LABEL} family entry name is not UTF-8"))?;
            let key = file_name.to_ascii_lowercase();
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected_artifacts.get(&key).copied()
            else {
                return Err(format!(
                    "{LABEL} complete physical family census has unexpected file '{file_name}'"
                ));
            };
            if file_name != expected_name {
                return Err(format!(
                    "{LABEL} retained artifact spelling changed: expected '{expected_name}', got '{file_name}'"
                ));
            }
            let bytes = fs::read(&path).map_err(|error| {
                format!("failed to read {LABEL} artifact '{file_name}': {error}")
            })?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} retained artifact '{file_name}' identity changed: bytes={}/{expected_bytes}, sha256={sha256}, blake3={content_blake3}",
                    canonical.len()
                ));
            }
            physical_names.push(key);
            retained_records.push(format!(
                "{expected_name}\t{expected_bytes}\t{expected_sha256}\t{expected_blake3}"
            ));
        }
        physical_names.sort();
        retained_records.sort_by_key(|record| record.to_ascii_lowercase());
        let expected_names = expected_artifacts.keys().cloned().collect::<Vec<_>>();
        let retained_hash = blake3::hash(retained_records.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if physical_names != expected_names
            || retained_records.len() != XYCE_BUG39_RETAINED_RECORD_COUNT
            || retained_hash != XYCE_BUG39_RETAINED_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} retained physical/content census changed: records={}/{retained_hash}, files={physical_names:?}",
                retained_records.len()
            ));
        }

        Ok(family_dir)
    }

    pub(super) fn bug39_deterministic_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug39DeterministicContract, String>> {
        let role = XyceBug39DeterministicRole::for_record(&deck.relative_path)?;
        Some((|| {
            const LABEL: &str = "BUG_39_SON deterministic expression family";
            let deck_path = self.root.join(role.path());
            if deck.section != XyceDeckSection::Netlists
                || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
                || !Self::same_path(&deck.path, &deck_path)
            {
                return Err(format!(
                    "recognized {LABEL} role {role:?} is not backed by its exact canonical Netlists path"
                ));
            }
            let family_dir = self.validate_bug39_deterministic_provenance()?;
            let reference_path = family_dir.join(role.reference_file_name());
            let contract = XyceBug39DeterministicContract {
                deck_path,
                reference_path,
                role,
            };
            let (plan, netlist, reference) = self.bug39_deterministic_plan(&contract)?;
            Self::validate_bug39_deterministic_semantics(role, &plan, &netlist, &reference)?;
            Ok(contract)
        })())
    }

    pub(super) fn validate_bug39_deterministic_provenance(&self) -> Result<PathBuf, String> {
        const LABEL: &str = "BUG_39_SON deterministic expression family";
        let family_dir = self.validate_bug39_retained_family_provenance()?;
        let wrappers = Self::load_upstream_wrapper_decks(&self.root);
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let mut candidates = Vec::with_capacity(XYCE_BUG39_DETERMINISTIC_CANDIDATE_COUNT);
        let mut content_records = Vec::with_capacity(XYCE_BUG39_DETERMINISTIC_CANDIDATE_COUNT);
        let mut owner_rows = Vec::with_capacity(XYCE_BUG39_DETERMINISTIC_CANDIDATE_COUNT);
        for role in XyceBug39DeterministicRole::ALL {
            let path = self.root.join(role.path());
            let relative = self.relative_key(&path);
            let key = Self::normalize_manifest_key(&relative);
            if key != role.record() || !Self::same_path(&path, &self.root.join(role.path())) {
                return Err(format!(
                    "{LABEL} canonical path '{}' is unavailable",
                    role.path()
                ));
            }
            if !self.requires_upstream_wrapper(&relative) || !wrappers.contains(&key) {
                return Err(format!(
                    "{LABEL} deck '{relative}' lost removed-wrapper ownership"
                ));
            }
            if exclusions.contains_key(&key) {
                return Err(format!(
                    "{LABEL} deck '{relative}' must not acquire upstream-exclusion provenance"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {LABEL} deck '{relative}': {error}"))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let content_hash = blake3::hash(&canonical).to_hex().to_string();
            candidates.push(relative.clone());
            content_records.push(format!("{relative}\t{content_hash}"));
            owner_rows.push(format!("{relative}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}"));
        }
        candidates.sort();
        content_records.sort();
        owner_rows.sort();
        let candidate_hash = blake3::hash(candidates.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(content_records.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let owner_hash = blake3::hash(owner_rows.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if candidates.len() != XYCE_BUG39_DETERMINISTIC_CANDIDATE_COUNT
            || candidate_hash != XYCE_BUG39_DETERMINISTIC_CANDIDATE_BLAKE3
            || content_hash != XYCE_BUG39_DETERMINISTIC_CONTENT_BLAKE3
            || owner_hash != XYCE_BUG39_DETERMINISTIC_OWNER_BLAKE3
        {
            return Err(format!(
                "{LABEL} ownership/content provenance changed: records={}/{candidate_hash}/{content_hash}/{owner_hash}",
                candidates.len()
            ));
        }
        Ok(family_dir)
    }

    pub(super) fn bug39_deterministic_plan(
        &self,
        contract: &XyceBug39DeterministicContract,
    ) -> Result<(XyceStaticDcPlan, Netlist, XycePrnTable), String> {
        let plan = self.static_dc_plan_for_path(&contract.deck_path, ExpressionDialect::Xyce)?;
        let netlist = Self::parse_xyce_netlist(&plan.source, &contract.deck_path)
            .map_err(|error| format!("BUG_39_SON deterministic deck parse failed: {error}"))?;
        let reference = Self::parse_dc_reference_file(
            XyceStaticDcContract::WrapperDefault,
            &contract.reference_path,
        )
        .map_err(|error| format!("BUG_39_SON deterministic gold parse failed: {error}"))?;
        Ok((plan, netlist, reference))
    }

    pub(super) fn validate_bug39_deterministic_semantics(
        role: XyceBug39DeterministicRole,
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        reference: &XycePrnTable,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG_39_SON deterministic expression family";
        let expected = role.expected_resistance();
        let resolved_parameter = netlist.params.get("RES").ok_or_else(|| {
            format!("{LABEL} {role:?} did not resolve the authored RES parameter")
        })?;
        let serialized_parameter = Self::xyce_default_prn_roundtrip(resolved_parameter)?;
        if plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || plan.print_format.is_some()
            || !plan.diagnostics.is_empty()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, DcSweepMode::Linear)
            || !plan.dc.source.eq_ignore_ascii_case("I1")
            || plan.dc.start.to_bits() != (-1.0f64).to_bits()
            || plan.dc.stop.to_bits() != (-1.0f64).to_bits()
            || plan.dc.step.to_bits() != (-0.1f64).to_bits()
            || plan.print.probes != ["I(I1)", "V(1)"]
        {
            return Err(format!(
                "{LABEL} requires the exact one-point I1 sweep and ordered I(I1), V(1) default PRN plan"
            ));
        }
        if netlist.params.statistical_mode() != StatisticalParamMode::Nominal
            || netlist.params.expression_dialect() != ExpressionDialect::Xyce
            || !resolved_parameter.is_finite()
            || serialized_parameter.to_bits() != expected.to_bits()
            || !netlist.diagnostics.is_empty()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || netlist.elements.len() != 2
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
        {
            return Err(format!(
                "{LABEL} typed parameter/topology/directive census changed for {role:?}"
            ));
        }
        let current = &netlist.elements[0];
        if !current.name.eq_ignore_ascii_case("I1")
            || current.nodes != ["1", "0"]
            || !matches!(
                &current.kind,
                ElementKind::CurrentSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.to_bits() == (-1.0f64).to_bits()
            )
        {
            return Err(format!("{LABEL} current-source topology changed"));
        }
        let resistor = &netlist.elements[1];
        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            deferred_params,
        } = &resistor.kind
        else {
            return Err(format!("{LABEL} R1 is no longer a resistor"));
        };
        if !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.nodes != ["1", "0"]
            || value.to_bits() != resolved_parameter.to_bits()
            || Self::xyce_default_prn_roundtrip(*value)?.to_bits() != expected.to_bits()
            || value_expr.is_some()
            || model.is_some()
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
        {
            return Err(format!(
                "{LABEL} resolved R1 semantics changed for {role:?}: value={value}, expression={value_expr:?}"
            ));
        }
        if reference.columns.len() != 3
            || !reference
                .columns
                .iter()
                .zip(["Index", "I(I1)", "V(1)"])
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
            || reference.rows.as_slice() != [[0.0, -1.0, expected]]
        {
            return Err(format!(
                "{LABEL} {role:?} gold must remain the exact one-row [0,-1,{expected}] PRN"
            ));
        }
        validate_output_symbols(netlist)
            .map_err(|error| format!("{LABEL} output dependency changed: {error}"))?;
        Ok(())
    }

    pub(super) fn run_bug39_deterministic_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug39DeterministicContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        let (plan, qualified_netlist, reference) = match self.bug39_deterministic_plan(&contract) {
            Ok(qualified) => qualified,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG_39_SON deterministic plan qualification failed: {error}"),
                    Vec::new(),
                );
            }
        };
        if let Err(error) = Self::validate_bug39_deterministic_semantics(
            contract.role,
            &plan,
            &qualified_netlist,
            &reference,
        ) {
            return self.failure_result(deck, start, result_contract, error, Vec::new());
        }
        let (netlist, results) = match self.run_static_dc_results(&plan, start) {
            Ok(run) => run,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG_39_SON deterministic DC execution failed: {error}"),
                    Vec::new(),
                );
            }
        };
        if results.len() != 1 {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!(
                    "BUG_39_SON deterministic DC produced {} rows instead of one",
                    results.len()
                ),
                Vec::new(),
            );
        }
        let actual = match self.dc_results_to_prn_table(&plan, &netlist, &results) {
            Ok(table) => table,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG_39_SON deterministic PRN conversion failed: {error}"),
                    Vec::new(),
                );
            }
        };
        let expected = contract.role.expected_resistance();
        let strengthened_physics = (|| {
            let [row] = actual.rows.as_slice() else {
                return Err("actual PRN must contain exactly one row".to_string());
            };
            if actual.columns != ["Index", "I(I1)", "V(1)"]
                || row.len() != 3
                || row.iter().any(|value| !value.is_finite())
                || Self::xyce_default_prn_roundtrip(row[0])?.to_bits() != 0.0f64.to_bits()
                || Self::xyce_default_prn_roundtrip(row[1])?.to_bits() != (-1.0f64).to_bits()
                || Self::xyce_default_prn_roundtrip(row[2])?.to_bits() != expected.to_bits()
            {
                return Err(format!(
                    "actual one-point PRN does not preserve exact [0,-1,{expected}] circuit physics: {:?}/{row:?}",
                    actual.columns
                ));
            }
            Ok(())
        })();
        if let Err(error) = strengthened_physics {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("BUG_39_SON strengthened physical contract failed: {error}"),
                Vec::new(),
            );
        }
        if let Err(error) = self.validate_bug39_deterministic_provenance() {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("BUG_39_SON provenance changed during execution: {error}"),
                Vec::new(),
            );
        }
        let mismatches = match self.compare_release_7_10_xyce_verify_dc_tables(
            "BUG_39_SON deterministic expression",
            &reference,
            &actual,
            &results,
            &results,
        ) {
            Ok(mismatches) => mismatches,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG_39_SON xyce_verify adapter failed: {error}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, result_contract)
        } else {
            self.failure_result(
                deck,
                start,
                result_contract,
                format!("{} BUG_39_SON mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn bug39_generated_source(role: XyceBug39GaussianRole) -> Result<String, String> {
        let mut source = String::with_capacity(role.generated_source_bytes());
        match role {
            XyceBug39GaussianRole::AgaussAbsolute => {
                source.push_str("*TEST of gaussian random deviates\n")
            }
            XyceBug39GaussianRole::GaussRelative => {
                source.push_str("*TEST of gaussian random deviates with GAUSS instead of AGAUSS\n")
            }
        }
        source.push_str("I1 1 0 DC -1\n");
        let expression = match role {
            XyceBug39GaussianRole::AgaussAbsolute => "agauss(100,10,10)",
            XyceBug39GaussianRole::GaussRelative => "gauss(100,0.1,10)",
        };
        for index in 1..=XYCE_BUG39_SAMPLE_COUNT {
            writeln!(source, "R{index} 1 0 {{{expression}}}")
                .expect("writing to a String cannot fail");
        }
        source.push_str("\n.DC I1 -1 -1 -.1\n.print DC FORMAT=NOINDEX PRECISION=19 ");
        for index in 1..=XYCE_BUG39_SAMPLE_COUNT {
            write!(source, "R{index}:R ").expect("writing to a String cannot fail");
        }
        source.push_str("\n.end\n");

        Self::validate_bug39_generated_source_identity(role, &source)?;
        Ok(source)
    }

    pub(super) fn validate_bug39_generated_source_identity(
        role: XyceBug39GaussianRole,
        source: &str,
    ) -> Result<(), String> {
        let source_sha256 = format!("{:x}", Sha256::digest(source.as_bytes()));
        let source_hash = blake3::hash(source.as_bytes()).to_hex().to_string();
        if source.len() != role.generated_source_bytes()
            || source_sha256 != role.generated_source_sha256()
            || source_hash != role.generated_source_blake3()
        {
            return Err(format!(
                "BUG_39_SON {:?} generated source identity changed: bytes={}/{}, sha256={source_sha256}, blake3={source_hash}",
                role,
                source.len(),
                role.generated_source_bytes()
            ));
        }
        Ok(())
    }

    pub(super) fn bug39_sampled_plan(
        &self,
        contract: &XyceBug39GaussianContract,
    ) -> Result<(XyceStaticDcPlan, Netlist), String> {
        const LABEL: &str = "BUG_39_SON generated Gaussian mean/sigma family";
        let source = Self::bug39_generated_source(contract.role)?;
        let generated_path = contract
            .anchor_path
            .with_file_name(contract.role.generated_file_name());
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Sample,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            parameter_redefinition_diagnostic_policy:
                rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Silent,
            ..NetlistParseOptions::default()
        };
        let netlist = Netlist::parse_with_path_and_options(&source, &generated_path, options)
            .map_err(|error| format!("failed to parse {LABEL} generated source: {error}"))?;
        Self::validate_bug39_sampled_netlist(contract.role, &netlist)?;
        let dc = Self::single_dc_sweep(&netlist)?;
        let print = XycePrintRequest {
            probes: (1..=XYCE_BUG39_SAMPLE_COUNT)
                .map(|index| format!("R{index}:R"))
                .collect(),
        };
        Self::validate_bug39_print_plan(&print)?;
        let diagnostics = netlist.diagnostics.clone();
        let plan = XyceStaticDcPlan {
            deck_path: generated_path,
            execution_dir: None,
            source,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            parameter_redefinition_diagnostic_policy:
                rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Silent,
            print,
            print_format: Some("NOINDEX".to_string()),
            dc,
            dc_data: None,
            steps: Vec::new(),
            diagnostics,
        };
        Ok((plan, netlist))
    }

    pub(super) fn validate_bug39_print_plan(print: &XycePrintRequest) -> Result<(), String> {
        const LABEL: &str = "BUG_39_SON generated Gaussian mean/sigma family";
        if print.probes.len() != XYCE_BUG39_SAMPLE_COUNT {
            return Err(format!(
                "{LABEL} generated print plan has {} probes, expected {XYCE_BUG39_SAMPLE_COUNT}",
                print.probes.len()
            ));
        }
        for (offset, probe) in print.probes.iter().enumerate() {
            let index = offset + 1;
            if probe != &format!("R{index}:R") {
                return Err(format!(
                    "{LABEL} generated print probe {index} changed: {probe:?}"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_bug39_sampled_netlist(
        role: XyceBug39GaussianRole,
        netlist: &Netlist,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG_39_SON generated Gaussian mean/sigma family";
        if netlist.params.statistical_mode() != StatisticalParamMode::Sample
            || netlist.params.expression_dialect() != ExpressionDialect::Xyce
            || netlist.options.seed.is_some()
        {
            return Err(format!(
                "{LABEL} requires the unseeded wrapper source under explicit Xyce Sample mode"
            ));
        }
        if !netlist.diagnostics.is_empty()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || netlist.elements.len() != XYCE_BUG39_SAMPLE_COUNT + 1
        {
            return Err(format!(
                "{LABEL} generated topology changed: diagnostics={}, models={}, subcircuits={}, elements={}",
                netlist.diagnostics.len(),
                netlist.models.len(),
                netlist.subcircuits.len(),
                netlist.elements.len()
            ));
        }
        let current_source = &netlist.elements[0];
        if !current_source.name.eq_ignore_ascii_case("I1")
            || current_source.nodes != ["1", "0"]
            || !matches!(
                &current_source.kind,
                ElementKind::CurrentSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.to_bits() == (-1.0_f64).to_bits()
            )
        {
            return Err(format!("{LABEL} generated current-source topology changed"));
        }
        let expected_expression = match role {
            XyceBug39GaussianRole::AgaussAbsolute => "agauss(100,10,10)",
            XyceBug39GaussianRole::GaussRelative => "gauss(100,0.1,10)",
        };
        for (offset, element) in netlist.elements[1..].iter().enumerate() {
            let index = offset + 1;
            let ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
            } = &element.kind
            else {
                return Err(format!(
                    "{LABEL} generated element {index} is not a resistor"
                ));
            };
            if element.name != format!("R{index}")
                || element.nodes != ["1", "0"]
                || !value.is_nan()
                || value_expr.as_deref() != Some(expected_expression)
                || model.is_some()
                || !instance_params.is_empty()
                || !deferred_params.is_empty()
            {
                return Err(format!(
                    "{LABEL} generated resistor R{index} topology/expression changed: name={:?}, nodes={:?}, value={}, value_expr={value_expr:?}, model={model:?}, instance_params={instance_params:?}, deferred_params={deferred_params:?}",
                    element.name, element.nodes, value
                ));
            }
        }
        match netlist.analyses.as_slice() {
            [
                AnalysisCommand::Dc {
                    source,
                    start,
                    stop,
                    step,
                    mode: DcSweepMode::Linear,
                    sweep2: None,
                },
            ] if source.eq_ignore_ascii_case("I1")
                && start.to_bits() == (-1.0_f64).to_bits()
                && stop.to_bits() == (-1.0_f64).to_bits()
                && step.to_bits() == (-0.1_f64).to_bits() => {}
            _ => return Err(format!("{LABEL} generated one-point DC analysis changed")),
        }
        if netlist.saves.signals.len() != XYCE_BUG39_SAMPLE_COUNT
            || netlist.output_requests.len() != 1
            || netlist.output_requests[0].directive != OutputDirectiveKind::Print
            || netlist.output_requests[0].analysis
                != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || netlist.output_requests[0].name.is_some()
            || netlist.output_requests[0].print_delimiter
                != Some(rspice_core::netlist::PrintDelimiter::Whitespace)
            || !netlist.output_requests[0].dependencies.is_empty()
        {
            return Err(format!(
                "{LABEL} generated ordered print request changed: saves={}, requests={}, request={:?}",
                netlist.saves.signals.len(),
                netlist.output_requests.len(),
                netlist.output_requests.first()
            ));
        }
        for (offset, signal) in netlist.saves.signals.iter().enumerate() {
            let index = offset + 1;
            if !matches!(
                signal,
                rspice_core::netlist::SaveSignal::Raw(raw)
                    if raw == &format!("r{index}:r")
            ) {
                return Err(format!(
                    "{LABEL} generated print probe {index} is not ordered R{index}:R"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn bug39_population_moments(
        samples: &[Value],
    ) -> Result<XyceBug39GaussianMoments, String> {
        if samples.len() != XYCE_BUG39_SAMPLE_COUNT {
            return Err(format!(
                "BUG_39_SON Gaussian wrapper produced {} samples, expected {}",
                samples.len(),
                XYCE_BUG39_SAMPLE_COUNT
            ));
        }
        if samples.iter().any(|sample| !sample.is_finite()) {
            return Err("BUG_39_SON Gaussian wrapper produced a non-finite sample".to_string());
        }
        let (sum, sum_squares) = samples
            .iter()
            .fold((0.0, 0.0), |(sum, sum_squares), sample| {
                (sum + sample, sum_squares + sample * sample)
            });
        let count = samples.len() as Value;
        let mean = sum / count;
        let mean_square = sum_squares / count;
        let variance = mean_square - mean * mean;
        if !mean.is_finite() || !variance.is_finite() || variance < 0.0 {
            return Err(format!(
                "BUG_39_SON Gaussian wrapper moments are invalid: mean={mean}, variance={variance}"
            ));
        }
        Ok(XyceBug39GaussianMoments {
            mean,
            population_standard_deviation: variance.sqrt(),
        })
    }

    pub(super) fn validate_bug39_moment_predicate(
        moments: XyceBug39GaussianMoments,
    ) -> Result<(), String> {
        let mean_error = (moments.mean - XYCE_BUG39_MEAN).abs();
        let sigma_error = (moments.population_standard_deviation - XYCE_BUG39_EXPECTED_SIGMA).abs();
        if Self::bug39_moment_errors_pass(mean_error, sigma_error) {
            Ok(())
        } else {
            Err(format!(
                "BUG_39_SON historical strict mean/sigma predicate failed: mean={} (error {mean_error}), population sigma={} (error {sigma_error}), required both errors < {}",
                moments.mean, moments.population_standard_deviation, XYCE_BUG39_MOMENT_TOLERANCE
            ))
        }
    }

    pub(super) fn bug39_moment_errors_pass(mean_error: Value, sigma_error: Value) -> bool {
        mean_error.is_finite()
            && sigma_error.is_finite()
            && mean_error < XYCE_BUG39_MOMENT_TOLERANCE
            && sigma_error < XYCE_BUG39_MOMENT_TOLERANCE
    }

    pub(super) fn run_bug39_gaussian_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug39GaussianContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        let (plan, netlist) = match self.bug39_sampled_plan(&contract) {
            Ok(plan) => plan,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG_39_SON generated plan qualification failed: {error}"),
                    Vec::new(),
                );
            }
        };
        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let results = match engine.run_dc_sweep2_spec_with_report_and_abort(
            &netlist,
            &plan.dc.source,
            &plan.dc.primary_spec(),
            None,
            &abort,
        ) {
            Ok(results) => results,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!(
                        "BUG_39_SON generated DC execution exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG_39_SON generated DC execution failed: {error}"),
                    Vec::new(),
                );
            }
        };
        let table = match self.dc_results_to_prn_table(&plan, &netlist, &results) {
            Ok(table) => table,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG_39_SON generated DC output conversion failed: {error}"),
                    Vec::new(),
                );
            }
        };
        let expected_columns = std::iter::once("Index".to_string())
            .chain(plan.print.probes.iter().cloned())
            .collect::<Vec<_>>();
        if table.columns != expected_columns || table.rows.len() != 1 {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!(
                    "BUG_39_SON generated DC output census changed: columns={}, rows={}",
                    table.columns.len(),
                    table.rows.len()
                ),
                Vec::new(),
            );
        }
        let row = &table.rows[0];
        if row.len() != XYCE_BUG39_SAMPLE_COUNT + 1 || row[0].to_bits() != 0.0_f64.to_bits() {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!(
                    "BUG_39_SON generated NOINDEX projection changed: row columns={}",
                    row.len()
                ),
                Vec::new(),
            );
        }
        let moments = match Self::bug39_population_moments(&row[1..]) {
            Ok(moments) => moments,
            Err(error) => {
                return self.failure_result(deck, start, result_contract, error, Vec::new());
            }
        };
        match Self::validate_bug39_moment_predicate(moments) {
            Ok(()) => self.passed_result(deck, start, result_contract),
            Err(error) => self.failure_result(deck, start, result_contract, error, Vec::new()),
        }
    }
}
