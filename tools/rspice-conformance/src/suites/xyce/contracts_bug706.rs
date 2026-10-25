use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "BUG_706_SON RC include/inline Release relation";
const OWNER_CONTRACT: &str = "bug706_rc_include_relational_wrapper_owner";
const INLINE_CONTRACT: &str = "bug706_rc_inline_release_reference";
#[cfg(test)]
const FLAT_ELEMENT_COUNT: usize = 6_003;
const FLAT_NODE_COUNT: usize = 3_002;
const FLAT_BRANCH_COUNT: usize = 1;
#[cfg(test)]
const FLAT_SIGNATURE_BYTES: usize = 378_564;
#[cfg(test)]
const FLAT_SIGNATURE_SHA256: &str =
    "8dbc70b9f3b92103ea3740c64f4b5d60c2be848a145207e3735042cfb559dc68";
#[cfg(test)]
const FLAT_SIGNATURE_BLAKE3: &str =
    "698e3a5133d9de1486a06865a545120c87a5e4fc6ad22a88212b7997c2b67613";
const MAX_ROWS: usize = 20_000;
const MIN_ROWS: usize = 50;
const MAX_RESULT_SCALARS: usize = 3 * MAX_ROWS;
const ANALYTIC_TIMES: [Value; 5] = [0.0, 0.25e-3, 0.5e-3, 0.75e-3, 1.0e-3];
const ANALYTIC_ABSOLUTE_TOLERANCE: Value = 2.0e-5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug706Role {
    IncludeOwner,
    InlineGood,
}

impl Bug706Role {
    const ALL: [Self; 2] = [Self::IncludeOwner, Self::InlineGood];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::IncludeOwner => OWNER_CONTRACT,
            Self::InlineGood => INLINE_CONTRACT,
        }
    }

    const fn path(self) -> &'static str {
        match self {
            Self::IncludeOwner => "Netlists/Certification_Tests/BUG_706_SON/rc_simple_lib.cir",
            Self::InlineGood => "Netlists/Certification_Tests/BUG_706_SON/rc_simple_xyce.cir",
        }
    }

    const fn record(self) -> &'static str {
        match self {
            Self::IncludeOwner => "netlists/certification_tests/bug_706_son/rc_simple_lib.cir",
            Self::InlineGood => "netlists/certification_tests/bug_706_son/rc_simple_xyce.cir",
        }
    }

    const fn file_name(self) -> &'static str {
        match self {
            Self::IncludeOwner => "rc_simple_lib.cir",
            Self::InlineGood => "rc_simple_xyce.cir",
        }
    }
}

impl XyceTestRunner {
    fn bug706_plan_from_validated_netlist(
        role: Bug706Role,
        path: &Path,
        source: &str,
        netlist: &Netlist,
    ) -> Result<XyceStaticTranPlan, String> {
        let purpose = match role {
            Bug706Role::IncludeOwner => {
                XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
            }
            Bug706Role::InlineGood => XyceStaticTranPlanPurpose::RelationalFamily,
        };
        if Self::contains_control_block(source) {
            return Err(format!("{LABEL} must not acquire a control block"));
        }
        Self::reject_unsupported_source_directives(source)?;
        let output = Self::single_tran_print_output_request(source)?;
        let print = XycePrintRequest {
            probes: output.probes,
        };
        let plan = XyceStaticTranPlan {
            deck_path: path.to_path_buf(),
            oracle: XyceStaticTranOracle::None,
            source: source.to_string(),
            print: Some(print),
            output_override: false,
            timeint_conststep: Self::source_enables_constant_time_step_output(source),
            tran: Self::single_tran_analysis(netlist)?,
            steps: Self::step_commands(netlist)?,
            contract: match role {
                Bug706Role::IncludeOwner => XyceStaticTranContract::WrapperStatic,
                Bug706Role::InlineGood => XyceStaticTranContract::PlainStatic,
            },
            wrapper_tolerance: None,
            comparison_mode: XyceStaticTranComparisonMode::Pointwise,
        };
        Self::validate_bug706_plan(role, &plan)?;
        plan.validate_oracle_contract(purpose, role == Bug706Role::IncludeOwner)?;
        Ok(plan)
    }

    fn validate_bug706_plan(role: Bug706Role, plan: &XyceStaticTranPlan) -> Result<(), String> {
        let probes = plan
            .print
            .as_ref()
            .map(|print| {
                print
                    .probes
                    .iter()
                    .map(|probe| Self::normalize_probe(probe))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if plan.deck_path.file_name().and_then(|name| name.to_str()) != Some(role.file_name())
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract
                != match role {
                    Bug706Role::IncludeOwner => XyceStaticTranContract::WrapperStatic,
                    Bug706Role::InlineGood => XyceStaticTranContract::PlainStatic,
                }
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || !plan.steps.is_empty()
            || probes != ["v(2)"]
            || plan.tran.step.to_bits() != 0.1e-6f64.to_bits()
            || plan.tran.stop.to_bits() != 0.001f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        let outputs = Self::print_output_requests(&plan.source, "TRAN")?;
        if !matches!(outputs.as_slice(), [request]
            if request.format.is_none()
                && request.file.is_none()
                && matches!(request.probes.as_slice(), [probe] if probe.eq_ignore_ascii_case("V(2)")))
        {
            return Err(format!("{LABEL} default PRN request changed"));
        }
        Ok(())
    }

    fn bug706_plain_resistor(element: &rspice_core::netlist::Element) -> bool {
        element.provenance == ElementProvenance::Authored
            && matches!(&element.kind, ElementKind::Resistor {
                value, value_expr: None, model: None, instance_params, deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
    }

    fn bug706_plain_capacitor(element: &rspice_core::netlist::Element) -> bool {
        element.provenance == ElementProvenance::Authored
            && matches!(&element.kind, ElementKind::Capacitor {
                value, value_expr: None, model: None, initial_voltage: None,
                instance_params, deferred_params,
            } if value.to_bits() == 1.0e-9f64.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
    }

    fn bug706_source_matches(element: &rspice_core::netlist::Element) -> bool {
        element.provenance == ElementProvenance::Authored
            && element.name.eq_ignore_ascii_case("V1")
            && Self::nodes_match(&element.nodes, &["1", "0"])
            && matches!(&element.kind, ElementKind::VoltageSource(SourceSpec::DcTransient {
                dc_value,
                transient,
            }) if dc_value.to_bits() == 0.0f64.to_bits()
                && matches!(transient.as_ref(), SourceSpec::Sin {
                    offset, amplitude, frequency, delay, damping, phase,
                } if offset.to_bits() == 0.0f64.to_bits()
                    && amplitude.to_bits() == 1.0f64.to_bits()
                    && frequency.to_bits() == 1_000.0f64.to_bits()
                    && delay.to_bits() == 0.0f64.to_bits()
                    && damping.to_bits() == 0.0f64.to_bits()
                    && phase.to_bits() == 0.0f64.to_bits()))
    }

    fn bug706_subckt_metadata_is_empty(definition: &SubcircuitDef) -> bool {
        definition.initial_conditions.is_empty()
            && definition.node_sets.is_empty()
            && definition.params.is_empty()
            && definition.expr_params.is_empty()
            && definition.string_params.is_empty()
            && definition.body_params.is_empty()
            && definition.body_expr_params.is_empty()
            && definition.body_string_params.is_empty()
            && definition.body_functions.is_empty()
            && definition.local_options.is_empty()
            && definition.library_ref.is_none()
            && definition.nested_subcircuits.is_empty()
    }

    fn validate_bug706_chain_definition(
        definition: &SubcircuitDef,
        name: &str,
        child: &str,
        count: usize,
    ) -> Result<(), String> {
        if !definition.name.eq_ignore_ascii_case(name)
            || !Self::nodes_match(
                &definition.ports,
                &["1", if count == 3 { "4" } else { "11" }],
            )
            || definition.elements.len() != count
            || !Self::bug706_subckt_metadata_is_empty(definition)
        {
            return Err(format!("{LABEL} subcircuit {name} envelope changed"));
        }
        for (index, element) in definition.elements.iter().enumerate() {
            let ordinal = index + 1;
            let left = ordinal.to_string();
            let right = (ordinal + 1).to_string();
            if element.provenance != ElementProvenance::Authored
                || !element.name.eq_ignore_ascii_case(&format!("X{ordinal}"))
                || !Self::nodes_match(&element.nodes, &[&left, &right])
                || !matches!(&element.kind, ElementKind::Subcircuit { subckt_name, params }
                    if subckt_name.eq_ignore_ascii_case(child) && params.is_empty())
            {
                return Err(format!(
                    "{LABEL} subcircuit {name} chain element {ordinal} changed: {element:?}"
                ));
            }
        }
        Ok(())
    }

    fn validate_bug706_netlist(role: Bug706Role, netlist: &Netlist) -> Result<(), String> {
        if netlist.title.trim_end() != "Simple RC test"
            || netlist.elements.len() != 4
            || netlist.subcircuits.len() != 7
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
                step, stop, start: None, max_step: None, uic: false,
            } if step.to_bits() == 0.1e-6f64.to_bits()
                && stop.to_bits() == 0.001f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let [source, resistor, capacitor, instance] = netlist.elements.as_slice() else {
            unreachable!("BUG706 element count was checked")
        };
        if !Self::bug706_source_matches(source)
            || !resistor.name.eq_ignore_ascii_case("R1")
            || !Self::nodes_match(&resistor.nodes, &["1", "2"])
            || !Self::bug706_plain_resistor(resistor)
            || !capacitor.name.eq_ignore_ascii_case("C1")
            || !Self::nodes_match(&capacitor.nodes, &["2", "0"])
            || !Self::bug706_plain_capacitor(capacitor)
            || instance.provenance != ElementProvenance::Authored
            || !instance.name.eq_ignore_ascii_case("X1")
            || !Self::nodes_match(&instance.nodes, &["2", "3"])
            || !matches!(&instance.kind, ElementKind::Subcircuit { subckt_name, params }
                if subckt_name.eq_ignore_ascii_case("rc3000") && params.is_empty())
        {
            return Err(format!(
                "{LABEL} {} top-level topology changed",
                role.file_name()
            ));
        }

        let definitions = netlist
            .subcircuits
            .iter()
            .map(|definition| (definition.name.to_ascii_lowercase(), definition))
            .collect::<BTreeMap<_, _>>();
        if definitions
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>()
            != BTreeSet::from([
                "rc", "rc10", "rc100", "rc1000", "rc10000", "rc3000", "rc100000",
            ])
        {
            return Err(format!("{LABEL} subcircuit inventory changed"));
        }
        let rc = definitions["rc"];
        if !rc.name.eq_ignore_ascii_case("rc")
            || !Self::nodes_match(&rc.ports, &["1", "2"])
            || rc.elements.len() != 2
            || !Self::bug706_subckt_metadata_is_empty(rc)
            || !rc.elements[0].name.eq_ignore_ascii_case("R")
            || !Self::nodes_match(&rc.elements[0].nodes, &["1", "2"])
            || !Self::bug706_plain_resistor(&rc.elements[0])
            || !rc.elements[1].name.eq_ignore_ascii_case("C")
            || !Self::nodes_match(&rc.elements[1].nodes, &["2", "0"])
            || !Self::bug706_plain_capacitor(&rc.elements[1])
        {
            return Err(format!("{LABEL} leaf rc definition changed"));
        }
        for (name, child, count) in [
            ("rc10", "rc", 10),
            ("rc100", "rc10", 10),
            ("rc1000", "rc100", 10),
            ("rc10000", "rc1000", 10),
            ("rc3000", "rc1000", 3),
            ("rc100000", "rc10000", 10),
        ] {
            Self::validate_bug706_chain_definition(definitions[name], name, child, count)?;
        }

        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || request.operands != ["V(2)"]
            || !request.expressions.is_empty()
            || request.dependencies.len() != 1
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || request.dependencies[0].expression
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("2")
        {
            return Err(format!("{LABEL} typed PRINT changed: {request:?}"));
        }
        Ok(())
    }

    #[cfg(test)]
    fn bug706_flat_signature(
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<(usize, String, String), String> {
        let flattened = flatten_netlist_with_models_with_abort(netlist, abort)
            .map_err(|error| format!("{LABEL} flatten failed: {error}"))?;
        if flattened.elements.len() != FLAT_ELEMENT_COUNT
            || !flattened.scoped_models.is_empty()
            || !flattened.scoped_initial_conditions.is_empty()
            || !flattened.scoped_node_sets.is_empty()
            || !flattened.scoped_startup_directives.is_empty()
            || !flattened.xspice_auto_bridge_node_hints.is_empty()
        {
            return Err(format!(
                "{LABEL} flattened envelope changed: elements={}",
                flattened.elements.len()
            ));
        }
        let mut sha256_hasher = Sha256::new();
        let mut blake3_hasher = blake3::Hasher::new();
        let mut signature_bytes = 0usize;
        let mut resistors = 0usize;
        let mut capacitors = 0usize;
        let mut sources = 0usize;
        let mut nodes = BTreeSet::new();
        for (index, element) in flattened.elements.iter().enumerate() {
            if abort.is_aborted() {
                return Err(format!(
                    "{LABEL} exceeded its shared deadline during flatten audit"
                ));
            }
            if element.provenance != ElementProvenance::Authored || element.nodes.len() != 2 {
                return Err(format!(
                    "{LABEL} flattened element {index} changed provenance/arity"
                ));
            }
            for node in &element.nodes {
                if node != "0" {
                    nodes.insert(node.to_ascii_lowercase());
                }
            }
            let kind = if Self::bug706_plain_resistor(element) {
                resistors += 1;
                format!("R:{:016x}", 1.0f64.to_bits())
            } else if Self::bug706_plain_capacitor(element) {
                capacitors += 1;
                format!("C:{:016x}", 1.0e-9f64.to_bits())
            } else if Self::bug706_source_matches(element) {
                sources += 1;
                format!(
                    "VSIN:{:016x}:{:016x}:{:016x}:{:016x}:{:016x}:{:016x}",
                    0.0f64.to_bits(),
                    1.0f64.to_bits(),
                    1_000.0f64.to_bits(),
                    0.0f64.to_bits(),
                    0.0f64.to_bits(),
                    0.0f64.to_bits()
                )
            } else {
                return Err(format!(
                    "{LABEL} flattened element {index} changed: {element:?}"
                ));
            };
            let line = format!(
                "{index}\t{}\t{}\t{kind}",
                element.name,
                element.nodes.join(",")
            );
            if index != 0 {
                sha256_hasher.update(b"\n");
                blake3_hasher.update(b"\n");
                signature_bytes = signature_bytes
                    .checked_add(1)
                    .ok_or_else(|| format!("{LABEL} flattened signature size overflowed"))?;
            }
            sha256_hasher.update(line.as_bytes());
            blake3_hasher.update(line.as_bytes());
            signature_bytes = signature_bytes
                .checked_add(line.len())
                .ok_or_else(|| format!("{LABEL} flattened signature size overflowed"))?;
        }
        if resistors != 3_001
            || capacitors != 3_001
            || sources != 1
            || nodes.len() != FLAT_NODE_COUNT
        {
            return Err(format!(
                "{LABEL} flattened census changed: R={resistors}, C={capacitors}, V={sources}, nodes={}",
                nodes.len()
            ));
        }
        let sha256 = format!("{:x}", sha256_hasher.finalize());
        let content_blake3 = blake3_hasher.finalize().to_hex().to_string();
        if signature_bytes != FLAT_SIGNATURE_BYTES
            || sha256 != FLAT_SIGNATURE_SHA256
            || content_blake3 != FLAT_SIGNATURE_BLAKE3
        {
            return Err(format!(
                "{LABEL} flattened signature changed: bytes={}, sha256={sha256}, blake3={content_blake3}",
                signature_bytes
            ));
        }
        Ok((signature_bytes, sha256, content_blake3))
    }

    fn bug706_projected_table(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
        abort: &dyn AbortSignal,
    ) -> Result<XycePrnTable, String> {
        Self::validate_transient_result_time_grid(result)?;
        if result.num_nodes != FLAT_NODE_COUNT
            || result.node_names.len() != FLAT_NODE_COUNT
            || result.branch_names.len() != FLAT_BRANCH_COUNT
            || result
                .voltages
                .iter()
                .filter(|values| !values.is_empty())
                .count()
                != 1
            || result
                .branch_currents
                .iter()
                .any(|values| !values.is_empty())
            || !result.digital_traces.is_empty()
            || !result.real_traces.is_empty()
            || !result.device_op_traces.is_empty()
            || !result.store_traces.is_empty()
        {
            return Err(format!("{LABEL} retained result topology changed"));
        }
        let projected = rspice_core::analysis::evaluate_tran_output_requests_with_abort(
            netlist,
            result,
            rspice_core::ResourceLimits::default(),
            abort,
        )
        .map_err(|error| format!("{LABEL} production output projection failed: {error}"))?;
        let [(name, physical_type, values)] = projected.as_slice() else {
            return Err(format!(
                "{LABEL} projection produced {} columns",
                projected.len()
            ));
        };
        if name != "V(2)" || *physical_type != "voltage" || values.len() != result.time.len() {
            return Err(format!("{LABEL} projection column/order/shape changed"));
        }
        let scalar_count = result
            .time
            .len()
            .checked_add(result.step_sizes.len())
            .and_then(|count| count.checked_add(values.len()))
            .ok_or_else(|| format!("{LABEL} result scalar count overflowed"))?;
        if result.time.len() < MIN_ROWS
            || result.time.len() > MAX_ROWS
            || scalar_count > MAX_RESULT_SCALARS
        {
            return Err(format!(
                "{LABEL} result exceeds its bounded envelope: rows={}, scalars={scalar_count}",
                result.time.len()
            ));
        }

        let time_scale = Self::tran_print_time_scale_factor(&plan.source)?;
        let output_times = Self::xyce_verify_transient_output_times(plan, netlist, result)?;
        let mut cursor = 0usize;
        let mut rows = Vec::with_capacity(output_times.len());
        for (index, time) in output_times.into_iter().enumerate() {
            if abort.is_aborted() {
                return Err(format!(
                    "{LABEL} exceeded its shared deadline during projection"
                ));
            }
            while result
                .time
                .get(cursor)
                .is_some_and(|candidate| candidate.to_bits() != time.to_bits())
            {
                cursor += 1;
            }
            let value = values
                .get(cursor)
                .copied()
                .ok_or_else(|| format!("{LABEL} projection lost TIME={time:e}"))?;
            rows.push(vec![index as Value, time * time_scale, value]);
            cursor += 1;
        }
        let table = XycePrnTable {
            columns: ["Index", "TIME", "V(2)"].map(str::to_string).to_vec(),
            rows,
        };
        if result.time.len() != table.rows.len() {
            return Err(format!("{LABEL} default PRN omitted accepted rows"));
        }
        Ok(table)
    }

    fn validate_bug706_table(role: Bug706Role, table: &XycePrnTable) -> Result<(), String> {
        if table.columns != ["Index", "TIME", "V(2)"]
            || table.rows.len() < MIN_ROWS
            || table.rows.len() > MAX_ROWS
        {
            return Err(format!("{LABEL} {} table shape changed", role.file_name()));
        }
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != 3
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || row[1] < 0.0
                || row[1] > 0.001
                || row[2].abs() > 1.000_01
            {
                return Err(format!(
                    "{LABEL} {} malformed/passivity-violating row {index}",
                    role.file_name()
                ));
            }
        }
        if table.rows.windows(2).any(|pair| pair[0][1] >= pair[1][1])
            || table
                .rows
                .first()
                .is_none_or(|row| row[1].to_bits() != 0.0f64.to_bits() || row[2].abs() > 1.0e-12)
            || table
                .rows
                .last()
                .is_none_or(|row| row[1].to_bits() != 0.001f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} time/endpoints changed",
                role.file_name()
            ));
        }
        let minimum = table
            .rows
            .iter()
            .map(|row| row[2])
            .fold(Value::INFINITY, Value::min);
        let maximum = table
            .rows
            .iter()
            .map(|row| row[2])
            .fold(Value::NEG_INFINITY, Value::max);
        let variation = table
            .rows
            .windows(2)
            .map(|pair| (pair[1][2] - pair[0][2]).abs())
            .sum::<Value>();
        if minimum >= -0.9 || maximum <= 0.9 || !(3.5..=4.5).contains(&variation) {
            return Err(format!(
                "{LABEL} {} waveform is trivial/non-passive: min={minimum:e}, max={maximum:e}, variation={variation:e}",
                role.file_name()
            ));
        }
        Ok(())
    }

    // Exact finite-ladder modal solution. The homogeneous path has a
    // Dirichlet boundary at the source and a Neumann boundary at the open far
    // end, hence theta_m=(2m+1)pi/(2N+1). This oracle is independent of the
    // transient integrator and uses only the structurally revalidated R, C,
    // source frequency, and 3001 capacitor-node count.
    fn bug706_analytic_v2(time: Value) -> Value {
        const N: usize = 3_001;
        const RC: Value = 1.0e-9;
        let omega = 2.0 * std::f64::consts::PI * 1_000.0;
        let mut value = 0.0;
        for mode in 0..N {
            let theta = (2 * mode + 1) as Value * std::f64::consts::PI / (2 * N + 1) as Value;
            let lambda = -4.0 * (0.5 * theta).sin().powi(2) / RC;
            let weight = 4.0 * theta.sin().powi(2) / (2 * N + 1) as Value / RC;
            let response = (omega * (lambda * time).exp()
                - omega * (omega * time).cos()
                - lambda * (omega * time).sin())
                / (lambda * lambda + omega * omega);
            value += weight * response;
        }
        value
    }

    fn validate_bug706_analytic_samples(
        role: Bug706Role,
        table: &XycePrnTable,
    ) -> Result<(), String> {
        for target_time in ANALYTIC_TIMES {
            let row = table
                .rows
                .iter()
                .min_by(|left, right| {
                    (left[1] - target_time)
                        .abs()
                        .total_cmp(&(right[1] - target_time).abs())
                })
                .ok_or_else(|| format!("{LABEL} has no analytic sample rows"))?;
            let time = row[1];
            let actual = row[2];
            let expected = Self::bug706_analytic_v2(time);
            if (actual - expected).abs() > ANALYTIC_ABSOLUTE_TOLERANCE {
                return Err(format!(
                    "{LABEL} {} failed analytic V(2) near target TIME={target_time:e} at accepted TIME={time:e}: actual={actual:e}, expected={expected:e}, tolerance={ANALYTIC_ABSOLUTE_TOLERANCE:e}",
                    role.file_name()
                ));
            }
        }
        Ok(())
    }

    fn validate_bug706_comparison_frame(side: &str, table: &XycePrnTable) -> Result<(), String> {
        if table.columns != ["Index", "TIME", "V(2)"]
            || table.rows.len() < 2
            || table.rows.len() > MAX_ROWS
        {
            return Err(format!(
                "{LABEL} {side} comparison frame changed: columns={:?}, rows={}",
                table.columns,
                table.rows.len()
            ));
        }
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != table.columns.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || (index > 0 && row[1] <= table.rows[index - 1][1])
            {
                return Err(format!(
                    "{LABEL} {side} comparison row {index} is malformed"
                ));
            }
        }
        Ok(())
    }

    fn compare_bug706_relation(
        &self,
        inline_good: &XycePrnTable,
        include_test: &XycePrnTable,
    ) -> Result<(), String> {
        Self::validate_bug706_comparison_frame("inline-good", inline_good)?;
        Self::validate_bug706_comparison_frame("include-test", include_test)?;
        let good_start = inline_good.rows[0][1];
        let good_stop = inline_good.rows[inline_good.rows.len() - 1][1];
        let test_start = include_test.rows[0][1];
        let test_stop = include_test.rows[include_test.rows.len() - 1][1];
        if good_start.to_bits() != test_start.to_bits()
            || good_stop.to_bits() != test_stop.to_bits()
        {
            return Err(format!(
                "{LABEL} inline-good/include-test output horizons differ: good=[{good_start:e}, {good_stop:e}], test=[{test_start:e}, {test_stop:e}]"
            ));
        }
        let directional = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            inline_good,
            include_test,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )?;
        if !directional.is_empty() {
            return Err(format!(
                "{LABEL} failed directional Release-7.10 xyce_verify (inline good -> include test): {directional:?}"
            ));
        }
        Ok(())
    }

    fn prepare_bug706_role(
        &self,
        role: Bug706Role,
        abort: &dyn AbortSignal,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before planning"
            ));
        }
        let path = self.root.join(role.path());
        let source = fs::read_to_string(&path)
            .map_err(|error| format!("{LABEL} cannot read {}: {error}", role.file_name()))?;
        let netlist = Self::parse_xyce_netlist(&source, &path)
            .map_err(|error| format!("{LABEL} {} parse failed: {error}", role.file_name()))?;
        Self::validate_bug706_netlist(role, &netlist)?;
        let plan = Self::bug706_plan_from_validated_netlist(role, &path, &source, &netlist)?;
        Ok((plan, netlist))
    }

    fn execute_bug706_role(
        &self,
        role: Bug706Role,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        start: Instant,
        abort: &dyn AbortSignal,
    ) -> Result<XycePrnTable, String> {
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before execution"
            ));
        }
        let result = self
            .run_transient_family_netlist(plan, netlist, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!("{LABEL} exceeded its shared deadline"),
                other => format!("{LABEL} {} execution failed: {other}", role.file_name()),
            })?;
        let table = Self::bug706_projected_table(plan, netlist, &result, abort)?;
        Self::validate_bug706_table(role, &table)?;
        Self::validate_bug706_analytic_samples(role, &table)?;
        Ok(table)
    }

    pub(super) fn validate_bug706_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} shared deadline expired before execution"));
        }
        // Release wrapper direction is CIR1=inline-good followed by
        // CIR2=include-test. Both roles share this invocation's absolute
        // deadline; the expensive streaming flat-signature audit remains in
        // the dedicated structure test rather than re-elaborating each role.
        let (inline_plan, inline_netlist) =
            self.prepare_bug706_role(Bug706Role::InlineGood, &abort)?;
        let inline = self.execute_bug706_role(
            Bug706Role::InlineGood,
            &inline_plan,
            &inline_netlist,
            start,
            &abort,
        )?;
        let (owner_plan, owner_netlist) =
            self.prepare_bug706_role(Bug706Role::IncludeOwner, &abort)?;
        let owner = self.execute_bug706_role(
            Bug706Role::IncludeOwner,
            &owner_plan,
            &owner_netlist,
            start,
            &abort,
        )?;
        self.compare_bug706_relation(&inline, &owner)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} exceeded its shared deadline"));
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

    fn runner() -> XyceTestRunner {
        XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default())
    }

    fn comparison_table(times: &[Value], waveform: impl Fn(Value) -> Value) -> XycePrnTable {
        XycePrnTable {
            columns: ["Index", "TIME", "V(2)"].map(str::to_string).to_vec(),
            rows: times
                .iter()
                .enumerate()
                .map(|(index, time)| vec![index as Value, *time, waveform(*time)])
                .collect(),
        }
    }

    #[test]
    fn bug706_exact_plans_topology_and_flattened_signature_match() {
        let runner = runner();
        let abort = DeadlineAbort::new(Instant::now(), 180_000);
        let mut signatures = Vec::new();
        for role in Bug706Role::ALL {
            let path = runner.root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG706 source");
            let netlist = XyceTestRunner::parse_xyce_netlist(&source, &path).expect("parse BUG706");
            XyceTestRunner::validate_bug706_netlist(role, &netlist).expect("typed BUG706");
            XyceTestRunner::bug706_plan_from_validated_netlist(role, &path, &source, &netlist)
                .expect("BUG706 exact plan");
            signatures.push(
                XyceTestRunner::bug706_flat_signature(&netlist, &abort).expect("flatten BUG706"),
            );
        }
        assert_eq!(signatures[0], signatures[1]);
    }

    #[test]
    fn bug706_full_release_relation_executes_both_roles() {
        runner()
            .validate_bug706_oracle(Instant::now())
            .expect("execute strict BUG706 relation");
    }

    #[test]
    fn bug706_each_production_role_fits_aggregate_watchdog_budget() {
        let mut runner = runner();
        runner.config.max_time_per_test_ms = 29_000;
        let started = Instant::now();
        runner
            .validate_bug706_oracle(started)
            .unwrap_or_else(|error| panic!("BUG706 must fit aggregate budget: {error}"));
        assert!(
            started.elapsed() < Duration::from_millis(29_000),
            "BUG706 exceeded aggregate watchdog budget: {:?}",
            started.elapsed()
        );
    }

    #[test]
    fn bug706_directional_comparator_accepts_unequal_grids_for_the_same_waveform() {
        let runner = runner();
        let good = comparison_table(&[0.0, 0.0005, 0.001], |time| 1_000.0 * time);
        let test = comparison_table(&[0.0, 0.00025, 0.0005, 0.00075, 0.001], |time| {
            1_000.0 * time
        });
        assert!(
            runner.compare_bug706_relation(&good, &test).is_ok(),
            "unequal-grid relation must accept the same piecewise-linear waveform"
        );

        let mut wrong_columns = test.clone();
        wrong_columns.columns[2] = "v(2)".to_string();
        assert!(
            runner
                .compare_bug706_relation(&good, &wrong_columns)
                .is_err()
        );
        let mut wrong_index = test;
        wrong_index.rows[2][0] = 7.0;
        assert!(runner.compare_bug706_relation(&good, &wrong_index).is_err());
    }

    #[test]
    fn bug706_directional_comparator_is_order_sensitive_and_rejects_waveform_wide_error() {
        let runner = runner();
        let times = [0.0, 0.0005, 0.001];
        let inline_good = comparison_table(&times, |_| 100.0);
        let include_test = comparison_table(&times, |_| 99.005);
        assert!(
            runner
                .compare_bug706_relation(&inline_good, &include_test)
                .is_ok(),
            "Release verifier must preserve inline-good to include-test ordering"
        );
        assert!(
            runner
                .compare_bug706_relation(&include_test, &inline_good)
                .is_err(),
            "reversing the historical good/test roles must change the asymmetric result"
        );

        let waveform_times = (0..=100)
            .map(|index| index as Value * 0.001 / 100.0)
            .collect::<Vec<_>>();
        let good = comparison_table(&waveform_times, |time| {
            (2.0 * std::f64::consts::PI * 1_000.0 * time).sin()
        });
        let wrong = comparison_table(&waveform_times, |time| {
            (2.0 * std::f64::consts::PI * 1_000.0 * time).sin() + 0.1
        });
        assert!(
            runner.compare_bug706_relation(&good, &wrong).is_err(),
            "directional verifier must reject waveform-wide numerical error"
        );
    }

    #[test]
    fn bug706_analytic_gate_rejects_identical_common_mode_wrong_tables() {
        let runner = runner();
        let times = (0..=100)
            .map(|index| index as Value * 0.001 / 100.0)
            .collect::<Vec<_>>();
        let shared_wrong = comparison_table(&times, |_| 0.0);
        assert!(
            runner
                .compare_bug706_relation(&shared_wrong, &shared_wrong)
                .is_ok(),
            "the relational verifier alone cannot identify common-mode error"
        );
        assert!(
            XyceTestRunner::validate_bug706_analytic_samples(
                Bug706Role::InlineGood,
                &shared_wrong,
            )
            .is_err(),
            "independent modal oracle must reject shared wrong waveforms"
        );
    }

    #[test]
    fn bug706_resource_envelope_rejects_oversized_tables() {
        let table = comparison_table(&[0.0, 0.001], |_| 0.0);
        let mut oversized = table;
        let final_row = oversized.rows.last().cloned().expect("counterfactual row");
        oversized.rows.resize(MAX_ROWS + 1, final_row);
        assert!(XyceTestRunner::validate_bug706_table(Bug706Role::InlineGood, &oversized).is_err());
        assert!(XyceTestRunner::bug706_analytic_v2(0.25e-3) > 0.9);
        assert!(XyceTestRunner::bug706_analytic_v2(0.75e-3) < -0.9);
    }

    #[test]
    fn bug706_expired_shared_deadline_fails_before_execution() {
        let error = runner()
            .validate_bug706_oracle(Instant::now() - Duration::from_secs(181))
            .expect_err("expired BUG706 deadline");
        assert!(error.contains("deadline"));
    }
}
