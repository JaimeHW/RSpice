use super::*;

const BUG307_LABEL: &str = "BUG_307 subcircuit-local model scope";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug307Role {
    ScopedCollisionOwner,
    ActiveOnlyControl,
}

impl Bug307Role {
    const ALL: [Self; 2] = [Self::ScopedCollisionOwner, Self::ActiveOnlyControl];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn path(self) -> &'static str {
        match self {
            Self::ScopedCollisionOwner => XYCE_BUG307_OWNER_PATH,
            Self::ActiveOnlyControl => XYCE_BUG307_CONTROL_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::ScopedCollisionOwner => XYCE_BUG307_OWNER_RECORD,
            Self::ActiveOnlyControl => XYCE_BUG307_CONTROL_RECORD,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::ScopedCollisionOwner => XYCE_BUG307_OWNER_CONTRACT,
            Self::ActiveOnlyControl => XYCE_BUG307_CONTROL_CONTRACT,
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::ScopedCollisionOwner => "bug_307_a.cir",
            Self::ActiveOnlyControl => "bug_307_b.cir",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::ScopedCollisionOwner => "scoped-collision owner",
            Self::ActiveOnlyControl => "active-only control",
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug307_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG307_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG307_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG307_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug307_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug307_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG307_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG307_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG307_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG307_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "BUG_307 Release-7.10 A/B wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug307_directory(
        directory: &Path,
        expected_artifacts: &[(&str, usize, &str, &str)],
        label: &str,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {label} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{label} directory {} must be a regular non-symlink directory",
                directory.display()
            ));
        }
        let expected = expected_artifacts
            .iter()
            .copied()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to read {label} directory: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {label} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{label} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{label} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if observed.contains_key(&key) {
                return Err(format!(
                    "{label} directory contains case-colliding member {name:?}"
                ));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{label} directory acquired unexpected member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{label} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {label} member {name:?}: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(label, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{label} member {name:?} changed: expected {expected_bytes}/{expected_sha256}/{expected_blake3}, got {}/{sha256}/{content_blake3}",
                    canonical.len()
                ));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{label} directory census changed: expected {} members, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug307_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug307Role,
    ) -> Result<BTreeMap<Bug307Role, Vec<u8>>, String> {
        Self::validate_bug307_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {BUG307_LABEL} {} is not backed by its canonical path",
                role.label()
            ));
        }

        let family_prefix = "netlists/certification_tests/bug_307/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let expected_owners = BTreeSet::from([
            XYCE_BUG307_OWNER_RECORD,
            "netlists/certification_tests/bug_307/bug_307_d.cir",
        ]);
        if owners != expected_owners {
            return Err(format!(
                "{BUG307_LABEL} wrapper ownership changed: {owners:?}"
            ));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG307_LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG307_OWNER_RECORD)
            || exclusions.contains_key("netlists/certification_tests/bug_307/bug_307_d.cir")
        {
            return Err(format!(
                "{BUG307_LABEL} wrapper owners must not be excluded"
            ));
        }
        let family_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(family_prefix))
            .collect::<BTreeMap<_, _>>();
        let expected_records = BTreeSet::from([
            XYCE_BUG307_CONTROL_RECORD,
            "netlists/certification_tests/bug_307/bug_307_e.cir",
            "netlists/certification_tests/bug_307/bug_307_f.cir",
            "netlists/certification_tests/bug_307/bug_307_g.cir",
            "netlists/certification_tests/bug_307/bug_307_h.cir",
            "netlists/certification_tests/bug_307/bug_307_i.cir",
        ]);
        if family_exclusions
            .keys()
            .map(|key| key.as_str())
            .collect::<BTreeSet<_>>()
            != expected_records
        {
            return Err(format!(
                "{BUG307_LABEL} exclusion-family census changed: {:?}",
                family_exclusions.keys().collect::<Vec<_>>()
            ));
        }
        for (record, exclusion) in family_exclusions {
            let expected_contract = if record.as_str() == XYCE_BUG307_CONTROL_RECORD {
                XYCE_BUG307_CONTROL_CONTRACT
            } else {
                "static_prn_tran"
            };
            if exclusion.source != XYCE_BUG307_EXCLUSION_SOURCE
                || !matches!(&exclusion.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract: actual }
                        if actual == expected_contract)
            {
                return Err(format!(
                    "{BUG307_LABEL} qualification changed for {record}: {exclusion:?}"
                ));
            }
        }

        let source_dir = self.root.join("Netlists/Certification_Tests/BUG_307");
        let retained = Self::validate_bug307_directory(
            &source_dir,
            &XYCE_BUG307_RETAINED_ARTIFACTS,
            "BUG_307 retained source family",
        )?;
        let output_dir = self.root.join("OutputData/Certification_Tests/BUG_307");
        Self::validate_bug307_directory(
            &output_dir,
            &XYCE_BUG307_RETAINED_OUTPUTS,
            "BUG_307 retained output family",
        )?;
        for member_role in Bug307Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member_role.path()))
                .map_err(|error| format!("{BUG307_LABEL} {} {error}", member_role.label()))?;
        }

        Bug307Role::ALL
            .into_iter()
            .map(|member_role| {
                retained
                    .get(&member_role.file_name().to_ascii_lowercase())
                    .cloned()
                    .map(|bytes| (member_role, bytes))
                    .ok_or_else(|| format!("{BUG307_LABEL} lost {}", member_role.file_name()))
            })
            .collect()
    }

    fn validate_bug307_model(
        model: &rspice_core::netlist::ModelDef,
        expected_name: &str,
        expected_params: &[(&str, Value)],
    ) -> bool {
        if !model.name.eq_ignore_ascii_case(expected_name)
            || !model.model_type.eq_ignore_ascii_case("R")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
            || model.params.len() != expected_params.len()
        {
            return false;
        }
        expected_params
            .iter()
            .all(|(expected_name, expected_value)| {
                model.params.iter().any(|(name, value)| {
                    name.eq_ignore_ascii_case(expected_name)
                        && value.to_bits() == expected_value.to_bits()
                })
            })
    }

    fn validate_bug307_subcircuit(
        subcircuit: &SubcircuitDef,
        expected_name: &str,
        expected_model: &str,
    ) -> bool {
        if !subcircuit.name.eq_ignore_ascii_case(expected_name)
            || subcircuit.ports.len() != 2
            || !subcircuit.ports[0].eq_ignore_ascii_case("a")
            || !subcircuit.ports[1].eq_ignore_ascii_case("b")
            || subcircuit.elements.len() != 1
            || !subcircuit.initial_conditions.is_empty()
            || !subcircuit.node_sets.is_empty()
            || !subcircuit.params.is_empty()
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.string_params.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.body_string_params.is_empty()
            || !subcircuit.body_functions.is_empty()
            || !subcircuit.local_options.is_empty()
            || subcircuit.library_ref.is_some()
            || !subcircuit.nested_subcircuits.is_empty()
        {
            return false;
        }
        let resistor = &subcircuit.elements[0];
        resistor.name.eq_ignore_ascii_case("R1")
            && resistor.nodes.len() == 2
            && resistor.nodes[0].eq_ignore_ascii_case("a")
            && resistor.nodes[1].eq_ignore_ascii_case("b")
            && resistor.provenance == ElementProvenance::Authored
            && matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: Some(model),
                instance_params,
                deferred_params,
            } if value.to_bits() == 0.0f64.to_bits()
                && model.eq_ignore_ascii_case(expected_model)
                && deferred_params.is_empty()
                && instance_params.len() == 2
                && instance_params.iter().any(|(name, value)|
                    name.eq_ignore_ascii_case("L") && value.to_bits() == 1.0f64.to_bits())
                && instance_params.iter().any(|(name, value)|
                    name.eq_ignore_ascii_case(rspice_core::netlist::XYCE_DEFAULT_RESISTOR_VALUE_MARKER)
                        && value.to_bits() == 1.0f64.to_bits()))
    }

    fn validate_bug307_worker_contract(
        &self,
        role: Bug307Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticDcPlan, String> {
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        let expected_diagnostic_lines: &[usize] = match role {
            Bug307Role::ScopedCollisionOwner => &[8, 12],
            Bug307Role::ActiveOnlyControl => &[8],
        };
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VIN")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.print.probes != ["V(1)", "I(VMON)"]
            || plan.diagnostics.len() != expected_diagnostic_lines.len()
            || !plan.diagnostics.iter().zip(expected_diagnostic_lines).all(
                |(diagnostic, expected_line)| {
                    diagnostic.line == *expected_line
                        && diagnostic.code == "xyce_resistor_model_missing_value"
                        && diagnostic.severity == rspice_core::netlist::DiagnosticSeverity::Warning
                },
            )
        {
            return Err(format!(
                "{BUG307_LABEL} {} static DC plan changed: {plan:?}",
                role.label()
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG307_LABEL} {} no longer parses: {error}", role.label())
        })?;
        let expected_subcircuits = match role {
            Bug307Role::ScopedCollisionOwner => 2,
            Bug307Role::ActiveOnlyControl => 1,
        };
        if netlist.title != "Semiconductor Resistor Circuit Netlist"
            || netlist.elements.len() != 3
            || netlist.subcircuits.len() != expected_subcircuits
            || netlist.models.len() != expected_subcircuits
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || netlist.diagnostics.len() != expected_diagnostic_lines.len()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.params.numeric_parameters().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{BUG307_LABEL} {} typed envelope changed",
                role.label()
            ));
        }

        let instance = &netlist.elements[0];
        if !instance.name.eq_ignore_ascii_case("XR1")
            || instance.nodes != ["2", "0"]
            || instance.provenance != ElementProvenance::Authored
            || !matches!(&instance.kind, ElementKind::Subcircuit { subckt_name, params }
                if subckt_name.eq_ignore_ascii_case("myRMOD") && params.is_empty())
        {
            return Err(format!("{BUG307_LABEL} {} X-line changed", role.label()));
        }
        let exact_source = |element: &rspice_core::netlist::Element,
                            name: &str,
                            nodes: [&str; 2],
                            expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && element.nodes == nodes
                && element.provenance == ElementProvenance::Authored
                && matches!(&element.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                        if value.to_bits() == expected.to_bits())
        };
        if !exact_source(&netlist.elements[1], "VIN", ["1", "0"], 5.0)
            || !exact_source(&netlist.elements[2], "VMON", ["1", "2"], 0.0)
        {
            return Err(format!("{BUG307_LABEL} {} sources changed", role.label()));
        }

        let active_model = [
            ("RSH", 0.031),
            ("NARROW", 0.0),
            ("DEFW", 1.0),
            ("TC1", 0.001),
            ("TC2", -0.001),
        ];
        match role {
            Bug307Role::ScopedCollisionOwner => {
                let unused_model = [("RSH", 1.0), ("TC1", 0.001), ("TC2", -0.001)];
                if !Self::validate_bug307_subcircuit(
                    &netlist.subcircuits[0],
                    "myRMOD2",
                    "myRMOD2::RMOD",
                ) || !Self::validate_bug307_subcircuit(
                    &netlist.subcircuits[1],
                    "myRMOD",
                    "myRMOD::RMOD",
                ) || !Self::validate_bug307_model(
                    &netlist.models[0],
                    "myRMOD2::RMOD",
                    &unused_model,
                ) || !Self::validate_bug307_model(
                    &netlist.models[1],
                    "myRMOD::RMOD",
                    &active_model,
                ) {
                    return Err(format!(
                        "{BUG307_LABEL} owner lexical model scopes changed: subcircuits={:?}, models={:?}",
                        netlist.subcircuits, netlist.models
                    ));
                }
            }
            Bug307Role::ActiveOnlyControl => {
                if !Self::validate_bug307_subcircuit(
                    &netlist.subcircuits[0],
                    "myRMOD",
                    "myRMOD::RMOD",
                ) || !Self::validate_bug307_model(
                    &netlist.models[0],
                    "myRMOD::RMOD",
                    &active_model,
                ) {
                    return Err(format!(
                        "{BUG307_LABEL} control active model scope changed: subcircuits={:?}, models={:?}",
                        netlist.subcircuits, netlist.models
                    ));
                }
            }
        }

        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode: DcSweepMode::Linear,
            sweep2: None,
        } if source.eq_ignore_ascii_case("VIN")
            && start.to_bits() == 0.0f64.to_bits()
            && stop.to_bits() == 5.0f64.to_bits()
            && step.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!(
                "{BUG307_LABEL} {} typed DC command changed",
                role.label()
            ));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || !request.expressions.is_empty()
            || request.dependencies.len() != 2
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("1")
            || request.dependencies[1].kind != OutputSymbolKind::Device
            || !request.dependencies[1].operator.eq_ignore_ascii_case("I")
            || !request.dependencies[1].symbol.eq_ignore_ascii_case("VMON")
        {
            return Err(format!(
                "{BUG307_LABEL} {} typed .PRINT request changed",
                role.label()
            ));
        }

        let flattened = flatten_netlist_with_models(&netlist)
            .map_err(|error| format!("{BUG307_LABEL} {} flatten failed: {error}", role.label()))?;
        if !flattened.scoped_models.is_empty()
            || !flattened.scoped_initial_conditions.is_empty()
            || !flattened.scoped_node_sets.is_empty()
            || !flattened.scoped_startup_directives.is_empty()
            || !flattened.xspice_auto_bridge_node_hints.is_empty()
            || flattened.elements.len() != 3
        {
            return Err(format!(
                "{BUG307_LABEL} {} flattened envelope changed",
                role.label()
            ));
        }
        let resistor = &flattened.elements[0];
        if !resistor.name.eq_ignore_ascii_case("XR1.R1")
            || resistor.nodes != ["2", "0"]
            || resistor.provenance != ElementProvenance::Authored
            || !matches!(&resistor.kind, ElementKind::Resistor { model: Some(model), .. }
                if model.eq_ignore_ascii_case("myRMOD::RMOD"))
        {
            return Err(format!(
                "{BUG307_LABEL} {} selected the wrong local resistor model",
                role.label()
            ));
        }
        let engine =
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
        let resolved = engine
            .resolved_resistor_parameters(&netlist, "XR1.R1")
            .map_err(|error| {
                format!(
                    "{BUG307_LABEL} {} resistor resolution failed: {error}",
                    role.label()
                )
            })?
            .ok_or_else(|| format!("{BUG307_LABEL} {} lost XR1.R1", role.label()))?;
        if resolved.resistance.to_bits() != 0.031f64.to_bits()
            || resolved.reported_resistance.to_bits() != 0.031f64.to_bits()
        {
            return Err(format!(
                "{BUG307_LABEL} {} active resistance changed: {resolved:?}",
                role.label()
            ));
        }
        Ok(plan)
    }

    fn run_bug307_worker(
        &self,
        role: Bug307Role,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|error| format!("{BUG307_LABEL} {} failed: {error}", role.label()))?;
        if results.len() != 6 {
            return Err(format!(
                "{BUG307_LABEL} {} produced {} points instead of 6",
                role.label(),
                results.len()
            ));
        }
        for (index, point) in results.iter().enumerate() {
            let expected_sweep = index as Value;
            let node_index = point
                .result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("1"))
                .ok_or_else(|| format!("{BUG307_LABEL} {} lost node 1", role.label()))?;
            let node_voltage = point.result.node_voltages[node_index];
            let branch_current = point
                .result
                .branch_current_named("VMON")
                .ok_or_else(|| format!("{BUG307_LABEL} {} lost I(VMON)", role.label()))?;
            let expected_current = expected_sweep / 0.031;
            let tolerance = expected_current.abs().max(1.0) * 1.0e-11;
            let finite = point.sweep_value.is_finite()
                && point
                    .result
                    .node_voltages
                    .iter()
                    .chain(point.result.branch_currents.iter())
                    .all(|value| value.is_finite())
                && point
                    .result
                    .dc_observables
                    .iter()
                    .all(|(_, value)| value.is_finite())
                && point
                    .device_op_report
                    .entries
                    .iter()
                    .flat_map(|entry| entry.params.iter().map(|(_, value)| value))
                    .all(|value| value.is_finite());
            if point.sweep_value.to_bits() != expected_sweep.to_bits()
                || node_voltage.to_bits() != expected_sweep.to_bits()
                || (branch_current - expected_current).abs() > tolerance
                || !finite
                || !point.device_op_report.labels_resolve()
            {
                return Err(format!(
                    "{BUG307_LABEL} {} point {index} violates the 0.031-ohm analytic contract",
                    role.label()
                ));
            }
        }
        self.dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|error| format!("{BUG307_LABEL} {} PRN failed: {error}", role.label()))
    }

    pub(super) fn validate_bug307_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug307Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG307_LABEL} deadline expired before validation"));
        }
        let sources = self.validate_bug307_provenance(deck, role)?;
        let mut outputs = BTreeMap::new();
        for worker_role in Bug307Role::ALL {
            let bytes = sources
                .get(&worker_role)
                .ok_or_else(|| format!("{BUG307_LABEL} lost {}", worker_role.label()))?;
            let source = std::str::from_utf8(bytes).map_err(|error| {
                format!(
                    "{BUG307_LABEL} {} is not UTF-8: {error}",
                    worker_role.label()
                )
            })?;
            let path = self.root.join(worker_role.path());
            let plan = self.validate_bug307_worker_contract(worker_role, source, &path)?;
            outputs.insert(
                worker_role,
                self.run_bug307_worker(worker_role, &plan, start)?,
            );
        }
        let owner = outputs
            .get(&Bug307Role::ScopedCollisionOwner)
            .expect("both BUG307 workers ran");
        let control = outputs
            .get(&Bug307Role::ActiveOnlyControl)
            .expect("both BUG307 workers ran");
        let mismatches = self
            .compare_serialized_default_prn_tables(owner, control)
            .map_err(|error| format!("{BUG307_LABEL} raw-PRN relation failed: {error}"))?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG307_LABEL} owner and control PRNs differ: {mismatches:?}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!(
                "{BUG307_LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        self.validate_bug307_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG307_LABEL} final provenance exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
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

    fn bug307_exclusion_manifest(control_disposition: &str) -> String {
        let mut rows = vec![format!(
            "{XYCE_BUG307_CONTROL_PATH}\t{XYCE_BUG307_EXCLUSION_SOURCE}\t{control_disposition}"
        )];
        for suffix in ["e", "f", "g", "h", "i"] {
            rows.push(format!(
                "Netlists/Certification_Tests/BUG_307/bug_307_{suffix}.cir\t{XYCE_BUG307_EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\tstatic_prn_tran"
            ));
        }
        format!(
            "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\n",
            rows.join("\n")
        )
    }

    fn bug307_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug307-{label}-"))
            .tempdir()
            .expect("create BUG307 fixture root");
        let root = temporary.path().to_path_buf();
        let source_family = root.join("Netlists/Certification_Tests/BUG_307");
        let output_family = root.join("OutputData/Certification_Tests/BUG_307");
        fs::create_dir_all(&source_family).expect("create BUG307 source family");
        fs::create_dir_all(&output_family).expect("create BUG307 output family");
        let canonical = corpus_root();
        for (name, ..) in XYCE_BUG307_RETAINED_ARTIFACTS {
            fs::copy(
                canonical
                    .join("Netlists/Certification_Tests/BUG_307")
                    .join(name),
                source_family.join(name),
            )
            .expect("copy canonical BUG307 source member");
        }
        for (name, ..) in XYCE_BUG307_RETAINED_OUTPUTS {
            fs::copy(
                canonical
                    .join("OutputData/Certification_Tests/BUG_307")
                    .join(name),
                output_family.join(name),
            )
            .expect("copy canonical BUG307 output member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{XYCE_BUG307_OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\nNetlists/Certification_Tests/BUG_307/bug_307_d.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"
            ),
        )
        .expect("write BUG307 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            bug307_exclusion_manifest(&format!(
                "{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{XYCE_BUG307_CONTROL_CONTRACT}"
            )),
        )
        .expect("write BUG307 exclusion manifest");
        let deck = XyceDeck {
            path: root.join(XYCE_BUG307_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG307_OWNER_PATH.to_string(),
        };
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug307_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug307_historical_oracle_provenance()
            .expect("Release-7.10 BUG307 A/B provenance remains exact");
    }

    #[test]
    fn bug307_workers_preserve_lexically_scoped_resistor_models() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug307Role::ALL {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG307 worker");
            runner
                .validate_bug307_worker_contract(role, &source, &path)
                .expect("canonical BUG307 worker passes");
            let mutation = source.replacen("RSH=.031", "RSH=.032", 1);
            assert!(
                runner
                    .validate_bug307_worker_contract(role, &mutation, &path)
                    .is_err(),
                "active model mutation must fail closed"
            );
        }
        let owner_path = root.join(XYCE_BUG307_OWNER_PATH);
        let owner = fs::read_to_string(&owner_path).expect("read BUG307 owner");
        assert!(
            runner
                .validate_bug307_worker_contract(
                    Bug307Role::ScopedCollisionOwner,
                    &owner.replacen("RSH=1", "RSH=2", 1),
                    &owner_path,
                )
                .is_err(),
            "unused conflicting model mutation must fail closed"
        );
    }

    #[test]
    fn bug307_oracle_runs_both_exact_roles() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug307Role::ALL {
            let deck = XyceDeck {
                path: root.join(role.path()),
                section: XyceDeckSection::Netlists,
                relative_path: role.path().to_string(),
            };
            runner
                .validate_bug307_oracle(&deck, role, Instant::now())
                .expect("canonical BUG307 relational oracle passes");
        }
    }

    #[test]
    fn bug307_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let deck = XyceDeck {
            path: root.join(XYCE_BUG307_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG307_OWNER_PATH.to_string(),
        };
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired deadline");
        assert!(
            runner
                .validate_bug307_oracle(&deck, Bug307Role::ScopedCollisionOwner, start)
                .is_err()
        );
    }

    #[test]
    fn bug307_provenance_rejects_source_output_and_qualification_drift() {
        let (_temporary, deck, runner) = bug307_fixture("source-drift");
        runner
            .validate_bug307_provenance(&deck, Bug307Role::ScopedCollisionOwner)
            .expect("canonical BUG307 fixture provenance passes");
        fs::write(
            deck.path
                .parent()
                .expect("BUG307 deck has parent")
                .join("unexpected.out"),
            "stale wrapper output\n",
        )
        .expect("write stale BUG307 artifact");
        assert!(
            runner
                .validate_bug307_provenance(&deck, Bug307Role::ScopedCollisionOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug307_fixture("output-drift");
        fs::write(
            runner
                .root
                .join("OutputData/Certification_Tests/BUG_307/bug_307_a.cir.prn"),
            "invented numerical gold\n",
        )
        .expect("write forbidden BUG307 A gold");
        assert!(
            runner
                .validate_bug307_provenance(&deck, Bug307Role::ScopedCollisionOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug307_fixture("qualification-drift");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            bug307_exclusion_manifest(UPSTREAM_EXCLUDED_DISPOSITION),
        )
        .expect("demote BUG307 control");
        let refreshed = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(
            refreshed
                .validate_bug307_provenance(&deck, Bug307Role::ScopedCollisionOwner)
                .is_err()
        );
    }
}
