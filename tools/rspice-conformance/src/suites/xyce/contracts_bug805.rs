use super::*;
use rspice_core::netlist::{BjtType, SaveSignal, SimulationOptions, SourceSpec};

const LABEL: &str = "BUG_805 legacy BJT model-alias relation";
const FAMILY_PATH: &str = "Netlists/Certification_Tests/BUG_805";

const CANONICAL_MODEL_PARAMS: [(&str, Value); 40] = [
    ("IS", 3.97589e-14),
    ("BF", 195.3412),
    ("NF", 1.0040078),
    ("VAF", 53.081),
    ("IKF", 0.976),
    ("ISE", 1.60241e-14),
    ("NE", 1.4791931),
    ("BR", 1.1107942),
    ("NR", 0.9928261),
    ("VAR", 11.3571702),
    ("IKR", 2.4993953),
    ("ISC", 1.88505e-12),
    ("NC", 1.1838278),
    ("RB", 56.5826472),
    ("IRB", 1.50459e-4),
    ("RBM", 5.2592283),
    ("RE", 0.0402974),
    ("RC", 0.4208),
    ("CJE", 2.56e-11),
    ("VJE", 0.682256),
    ("MJE", 0.3358856),
    ("TF", 3.3e-10),
    ("XTF", 6.0),
    ("VTF", 0.574),
    ("ITF", 0.32),
    ("PTF", 25.832),
    ("CJC", 1.40625e-11),
    ("VJC", 0.5417393),
    ("MJC", 0.4547893),
    ("XCJC", 1.0),
    ("TR", 3.2e-7),
    ("CJS", 0.0),
    ("VJS", 0.75),
    ("MJS", 0.0),
    ("XTB", 1.6486),
    ("EG", 1.11),
    ("XTI", 5.8315),
    ("KF", 0.0),
    ("AF", 1.0),
    ("FC", 0.83),
];

type CanonicalModel = BTreeMap<String, u64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Bug805Representation {
    Owner,
    Canonical,
    PspiceAliases,
    HspiceAliases,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Bug805Role(Bug805Representation);

impl Bug805Role {
    const ALL: [Self; 4] = [
        Self(Bug805Representation::Owner),
        Self(Bug805Representation::Canonical),
        Self(Bug805Representation::PspiceAliases),
        Self(Bug805Representation::HspiceAliases),
    ];
    const WORKERS: [Self; 3] = [
        Self(Bug805Representation::Canonical),
        Self(Bug805Representation::PspiceAliases),
        Self(Bug805Representation::HspiceAliases),
    ];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    fn file_name(self) -> &'static str {
        match self.0 {
            Bug805Representation::Owner => "bug_805.cir",
            Bug805Representation::Canonical => "colpitts_osc1.cir",
            Bug805Representation::PspiceAliases => "colpitts_osc2.cir",
            Bug805Representation::HspiceAliases => "colpitts_osc3.cir",
        }
    }

    fn path(self) -> String {
        format!("{FAMILY_PATH}/{}", self.file_name())
    }

    fn record(self) -> String {
        XyceTestRunner::normalize_manifest_key(&self.path())
    }

    pub(super) fn contract(self) -> &'static str {
        match self.0 {
            Bug805Representation::Owner => "bug805_bjt_model_alias_wrapper_owner",
            Bug805Representation::Canonical => "bug805_bjt_model_canonical_worker",
            Bug805Representation::PspiceAliases => "bug805_bjt_model_pspice_alias_worker",
            Bug805Representation::HspiceAliases => "bug805_bjt_model_hspice_alias_worker",
        }
    }

    fn raw_model_keys(self) -> &'static [&'static str] {
        const CANONICAL: [&str; 40] = [
            "IS", "BF", "NF", "VAF", "IKF", "ISE", "NE", "BR", "NR", "VAR", "IKR", "ISC", "NC",
            "RB", "IRB", "RBM", "RE", "RC", "CJE", "VJE", "MJE", "TF", "XTF", "VTF", "ITF", "PTF",
            "CJC", "VJC", "MJC", "XCJC", "TR", "CJS", "VJS", "MJS", "XTB", "EG", "XTI", "KF", "AF",
            "FC",
        ];
        const PSPICE: [&str; 40] = [
            "IS", "BFM", "NF", "VA", "IK", "JLE", "NLE", "BRM", "NR", "VB", "JBR", "JLC", "NC",
            "RB", "JRB", "RBM", "RE", "RC", "CJE", "PE", "ME", "TF", "XTF", "VTF", "JTF", "PTF",
            "CJC", "PC", "MC", "CDIS", "TR", "CCS", "PS", "MS", "TB", "EG", "PT", "KF", "AF", "FC",
        ];
        const HSPICE: [&str; 40] = [
            "IS", "BFM", "NF", "VBF", "JBF", "JLE", "NLE", "BRM", "NR", "VRB", "JBR", "JLC", "NC",
            "RB", "IOB", "RBM", "RE", "RC", "CJE", "PE", "ME", "TF", "XTF", "VTF", "JTF", "PTF",
            "CJC", "PC", "MC", "CDIS", "TR", "CSUB", "PSUB", "ESUB", "TCB", "EG", "PT", "KF", "AF",
            "FC",
        ];
        match self.0 {
            Bug805Representation::Canonical => &CANONICAL,
            Bug805Representation::PspiceAliases => &PSPICE,
            Bug805Representation::HspiceAliases => &HSPICE,
            Bug805Representation::Owner => &[],
        }
    }
}

impl XyceTestRunner {
    fn bug805_canonical_param_name(name: &str) -> Option<&'static str> {
        Some(match name.to_ascii_uppercase().as_str() {
            "BF" | "BFM" => "BF",
            "VAF" | "VA" | "VBF" => "VAF",
            "IKF" | "IK" | "JBF" => "IKF",
            "ISE" | "JLE" => "ISE",
            "NE" | "NLE" => "NE",
            "BR" | "BRM" => "BR",
            "VAR" | "VB" | "VRB" => "VAR",
            "IKR" | "JBR" => "IKR",
            "ISC" | "JLC" => "ISC",
            "IRB" | "JRB" | "IOB" => "IRB",
            "VJE" | "PE" => "VJE",
            "MJE" | "ME" => "MJE",
            "ITF" | "JTF" => "ITF",
            "VJC" | "PC" => "VJC",
            "MJC" | "MC" => "MJC",
            "XCJC" | "CDIS" => "XCJC",
            "CJS" | "CCS" | "CSUB" => "CJS",
            "VJS" | "PS" | "PSUB" => "VJS",
            "MJS" | "MS" | "ESUB" => "MJS",
            "XTB" | "TB" | "TCB" => "XTB",
            "XTI" | "PT" => "XTI",
            "IS" => "IS",
            "NF" => "NF",
            "NR" => "NR",
            "NC" => "NC",
            "RB" => "RB",
            "RBM" => "RBM",
            "RE" => "RE",
            "RC" => "RC",
            "CJE" => "CJE",
            "TF" => "TF",
            "XTF" => "XTF",
            "VTF" => "VTF",
            "PTF" => "PTF",
            "CJC" => "CJC",
            "TR" => "TR",
            "EG" => "EG",
            "KF" => "KF",
            "AF" => "AF",
            "FC" => "FC",
            _ => return None,
        })
    }

    fn validate_bug805_model(
        role: Bug805Role,
        model: &rspice_core::netlist::ModelDef,
    ) -> Result<CanonicalModel, String> {
        if role.0 == Bug805Representation::Owner
            || !model.name.eq_ignore_ascii_case("Q222200")
            || !model.model_type.eq_ignore_ascii_case("NPN")
            || model.params.len() != CANONICAL_MODEL_PARAMS.len()
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} {} model shape changed", role.file_name()));
        }
        let raw_keys = model
            .params
            .iter()
            .map(|(name, _)| name.to_ascii_uppercase())
            .collect::<Vec<_>>();
        if raw_keys
            != role
                .raw_model_keys()
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>()
        {
            return Err(format!(
                "{LABEL} {} raw alias spellings changed: {raw_keys:?}",
                role.file_name()
            ));
        }
        let mut normalized = BTreeMap::new();
        for (name, value) in &model.params {
            let canonical = Self::bug805_canonical_param_name(name)
                .ok_or_else(|| format!("{LABEL} unrecognized model parameter {name:?}"))?;
            if !value.is_finite()
                || normalized
                    .insert(canonical.to_string(), value.to_bits())
                    .is_some()
            {
                return Err(format!(
                    "{LABEL} {} model aliases became ambiguous",
                    role.file_name()
                ));
            }
        }
        let expected = CANONICAL_MODEL_PARAMS
            .into_iter()
            .map(|(name, value)| (name.to_string(), value.to_bits()))
            .collect::<BTreeMap<_, _>>();
        if normalized != expected {
            return Err(format!(
                "{LABEL} {} normalized physical model changed",
                role.file_name()
            ));
        }
        Ok(normalized)
    }

    fn validate_bug805_options(options: &SimulationOptions) -> Result<(), String> {
        if options.device_voltage_limiting != Some(true)
            || options.device_debug_level != Some(0)
            || options.timeint_reltol.map(Value::to_bits) != Some(1.0e-4f64.to_bits())
            || options.timeint_abstol.map(Value::to_bits) != Some(1.0e-9f64.to_bits())
        {
            return Err(format!("{LABEL} active options changed: {options:?}"));
        }
        let mut remaining = options.clone();
        remaining.device_voltage_limiting = None;
        remaining.device_debug_level = None;
        remaining.timeint_reltol = None;
        remaining.timeint_abstol = None;
        if format!("{remaining:?}") != format!("{:?}", SimulationOptions::default()) {
            return Err(format!("{LABEL} acquired an extra option: {remaining:?}"));
        }
        Ok(())
    }

    fn validate_bug805_passive(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
        value: Value,
        kind: char,
    ) -> Result<(), String> {
        let matches_kind = match kind {
            'R' => matches!(&element.kind, ElementKind::Resistor {
                value: actual, value_expr: None, model: None, instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty()),
            'C' => matches!(&element.kind, ElementKind::Capacitor {
                value: actual, value_expr: None, initial_voltage: None, model: None,
                instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty()),
            'L' => matches!(&element.kind, ElementKind::Inductor {
                value: actual, value_expr: None, initial_current: None, model: None,
                instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty()),
            _ => false,
        };
        if !element.name.eq_ignore_ascii_case(name)
            || element.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&element.nodes, &nodes)
            || !matches_kind
        {
            return Err(format!("{LABEL} passive {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn validate_bug805_netlist(
        role: Bug805Role,
        netlist: &Netlist,
    ) -> Result<CanonicalModel, String> {
        if role.0 == Bug805Representation::Owner
            || netlist.title != "Colpitts oscilator"
            || netlist.elements.len() != 20
            || netlist.models.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !Self::netlist_is_native_bug805_bjt_envelope(netlist)
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        Self::validate_bug805_options(&netlist.options)?;
        let names = netlist
            .elements
            .iter()
            .map(|element| element.name.to_ascii_uppercase())
            .collect::<Vec<_>>();
        let expected_names = [
            "R6", "R5", "R4", "L1", "R7", "C20", "C8", "C10", "C9", "C2", "C3", "C7", "C6", "C5",
            "C4", "R16", "R15", "V2", "R17", "Q2",
        ];
        if names != expected_names {
            return Err(format!("{LABEL} authored element order changed: {names:?}"));
        }
        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        for (name, nodes, value) in [
            ("R6", ["0", "2"], 15.0e3),
            ("R5", ["0", "3"], 15.0e3),
            ("R4", ["0", "4"], 100.0e3),
            ("R7", ["1", "5"], 0.37),
            ("R16", ["0", "q2b"], 22.0e3),
            ("R15", ["q2b", "q2c"], 47.0e3),
            ("R17", ["q2e", "0"], 2.2e3),
        ] {
            Self::validate_bug805_passive(
                elements
                    .get(&name.to_ascii_lowercase())
                    .ok_or_else(|| format!("{LABEL} lost {name}"))?,
                name,
                nodes,
                value,
                'R',
            )?;
        }
        for (name, nodes, value) in [
            ("C20", ["4", "0"], 30.0e-12),
            ("C8", ["1", "4"], 82.0e-12),
            ("C10", ["3", "0"], 270.0e-12),
            ("C9", ["1", "3"], 10.0e-12),
            ("C2", ["2", "0"], 47.0e-12),
            ("C3", ["1", "2"], 10.0e-12),
            ("C7", ["1", "0"], 82.0e-12),
            ("C6", ["1", "q2b"], 3.3e-9),
            ("C5", ["q2e", "0"], 2.7e-9),
            ("C4", ["q2b", "q2e"], 2.7e-9),
        ] {
            Self::validate_bug805_passive(
                elements
                    .get(&name.to_ascii_lowercase())
                    .ok_or_else(|| format!("{LABEL} lost {name}"))?,
                name,
                nodes,
                value,
                'C',
            )?;
        }
        Self::validate_bug805_passive(
            elements
                .get("l1")
                .ok_or_else(|| format!("{LABEL} lost L1"))?,
            "L1",
            ["5", "0"],
            2.5e-6,
            'L',
        )?;
        let source = elements
            .get("v2")
            .ok_or_else(|| format!("{LABEL} lost V2"))?;
        let expected_points: [(Value, Value); 2] = [(0.0, 0.0), (5.0e-9, 7.4)];
        if source.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&source.nodes, &["q2c", "0"])
            || !matches!(&source.kind, ElementKind::VoltageSource(SourceSpec::Pwl {
                points, delay, repeat_from,
            }) if points.len() == expected_points.len()
                && points.iter().zip(expected_points).all(|((time, value), (expected_time, expected_value))|
                    time.to_bits() == expected_time.to_bits()
                        && value.to_bits() == expected_value.to_bits())
                && delay.to_bits() == 0.0f64.to_bits()
                && repeat_from.is_none())
        {
            return Err(format!("{LABEL} source V2 changed: {source:?}"));
        }
        let transistor = elements
            .get("q2")
            .ok_or_else(|| format!("{LABEL} lost Q2"))?;
        if transistor.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&transistor.nodes, &["q2c", "q2b", "q2e"])
            || !matches!(&transistor.kind, ElementKind::Bjt {
                model, bjt_type: BjtType::Npn, instance_params, deferred_params,
            } if model.eq_ignore_ascii_case("Q222200")
                && instance_params.is_empty() && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} transistor Q2 changed: {transistor:?}"));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step, stop, start: None, max_step: None, uic: false,
        } if step.to_bits() == 0.0f64.to_bits()
            && stop.to_bits() == 80.0e-6f64.to_bits())
        {
            return Err(format!("{LABEL} transient analysis changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != 2
            || request
                .dependencies
                .iter()
                .zip(["q2b", "q2e"])
                .any(|(dependency, expected)| {
                    dependency.kind != OutputSymbolKind::Node
                        || !dependency.operator.eq_ignore_ascii_case("V")
                        || !dependency.symbol.eq_ignore_ascii_case(expected)
                        || dependency.expression
                })
        {
            return Err(format!("{LABEL} typed PRINT request changed: {request:?}"));
        }
        if !matches!(netlist.saves.signals.as_slice(),
            [SaveSignal::Voltage(left), SaveSignal::Voltage(right)]
                if left.eq_ignore_ascii_case("q2b") && right.eq_ignore_ascii_case("q2e"))
        {
            return Err(format!(
                "{LABEL} SaveSet changed: {:?}",
                netlist.saves.signals
            ));
        }
        Self::validate_bug805_model(role, &netlist.models[0])
    }

    fn bug805_plan(&self, role: Bug805Role) -> Result<XyceStaticTranPlan, String> {
        if role.0 == Bug805Representation::Owner {
            return Err(format!("{LABEL} owner is a wrapper anchor, not a worker"));
        }
        let plan = self.static_tran_plan_for_path_with_purpose(
            &self.root.join(role.path()),
            XyceStaticTranPlanPurpose::Bug805RelationalFamily,
        )?;
        let print = plan
            .print
            .as_ref()
            .ok_or_else(|| format!("{LABEL} {} lost PRINT", role.file_name()))?;
        if !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != 80.0e-6f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || print.probes.len() != 2
            || !print.probes[0].eq_ignore_ascii_case("V(q2b)")
            || !print.probes[1].eq_ignore_ascii_case("V(q2e)")
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(plan)
    }

    fn validate_bug805_table(role: Bug805Role, table: &XycePrnTable) -> Result<(), String> {
        if table.columns.len() != 4
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(q2b)")
            || !table.columns[3].eq_ignore_ascii_case("V(q2e)")
            || table.rows.len() < 20
        {
            return Err(format!("{LABEL} {} output shape changed", role.file_name()));
        }
        let mut prior_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != 4
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || prior_time.is_some_and(|prior| row[1] <= prior)
            {
                return Err(format!(
                    "{LABEL} {} output row {index} is malformed",
                    role.file_name()
                ));
            }
            prior_time = Some(row[1]);
        }
        let first = &table.rows[0];
        let last = table.rows.last().expect("BUG805 table is nonempty");
        let ranges = [2usize, 3].map(|column| {
            let minimum = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::INFINITY, Value::min);
            let maximum = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::NEG_INFINITY, Value::max);
            maximum - minimum
        });
        if first[1].to_bits() != 0.0f64.to_bits()
            || (last[1] - 80.0e-6).abs() > 1.0e-14
            || ranges.iter().any(|range| *range < 0.1)
        {
            return Err(format!(
                "{LABEL} {} output became vacuous",
                role.file_name()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug805_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let mut tables = BTreeMap::new();
        let mut physical_model = None;
        // Preserve the Release wrapper's three independent-run order.
        for worker in Bug805Role::WORKERS {
            if abort.is_aborted() {
                return Err(format!("{LABEL} deadline expired between independent runs"));
            }
            let plan = self.bug805_plan(worker)?;
            let parsed = Self::parse_xyce_netlist(&plan.source, &plan.deck_path)
                .map_err(|error| format!("{LABEL} {} parse failed: {error}", worker.file_name()))?;
            let normalized = Self::validate_bug805_netlist(worker, &parsed)?;
            if physical_model
                .as_ref()
                .is_some_and(|expected| expected != &normalized)
            {
                return Err(format!(
                    "{LABEL} aliases no longer describe one physical model"
                ));
            }
            physical_model = Some(normalized);
            let (netlist, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                    other => format!("{LABEL} {} execution failed: {other}", worker.file_name()),
                })?;
            Self::validate_bug805_netlist(worker, &netlist)?;
            let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)?;
            Self::validate_bug805_table(worker, &table)?;
            tables.insert(worker, table);
        }
        let canonical = tables
            .get(&Bug805Role(Bug805Representation::Canonical))
            .expect("canonical BUG805 worker ran");
        for worker in [
            Bug805Role(Bug805Representation::PspiceAliases),
            Bug805Role(Bug805Representation::HspiceAliases),
        ] {
            let mismatches = self.compare_serialized_default_prn_tables(
                canonical,
                tables.get(&worker).expect("all BUG805 workers ran"),
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {} default-PRN relation failed: {mismatches:?}",
                    worker.file_name()
                ));
            }
        }
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

    #[test]
    fn bug805_worker_models_normalize_to_one_exact_physical_card() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let mut normalized = None;
        for role in Bug805Role::WORKERS {
            let plan = runner.bug805_plan(role).expect("canonical BUG805 plan");
            let netlist = XyceTestRunner::parse_xyce_netlist(&plan.source, &plan.deck_path)
                .expect("canonical BUG805 netlist");
            let model = XyceTestRunner::validate_bug805_netlist(role, &netlist)
                .expect("canonical BUG805 typed contract");
            assert!(
                normalized
                    .as_ref()
                    .is_none_or(|expected| expected == &model)
            );
            normalized = Some(model);
        }
    }

    #[test]
    fn bug805_owner_executes_all_three_independent_alias_workers() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug805_oracle(Instant::now())
            .expect("BUG805 native alias relation");
    }

    #[test]
    fn bug805_typed_and_numeric_counterfactuals_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let role = Bug805Role(Bug805Representation::PspiceAliases);
        let plan = runner.bug805_plan(role).expect("canonical BUG805 plan");
        let changed = plan.source.replacen("BFM = 195.3412", "BF = 195.3412", 1);
        let netlist = XyceTestRunner::parse_xyce_netlist(&changed, &plan.deck_path)
            .expect("mutated BUG805 model still parses");
        assert!(XyceTestRunner::validate_bug805_netlist(role, &netlist).is_err());

        let table = XycePrnTable {
            columns: vec![
                "Index".to_string(),
                "TIME".to_string(),
                "V(q2b)".to_string(),
                "V(q2e)".to_string(),
            ],
            rows: vec![vec![0.0, 0.0, 0.0, 0.0], vec![1.0, 1.0e-6, 1.0, 0.5]],
        };
        let mut wrong = table.clone();
        wrong.rows[1][2] += 1.0e-3;
        assert!(
            !runner
                .compare_serialized_default_prn_tables(&table, &wrong)
                .expect("counterfactual default-PRN comparison")
                .is_empty()
        );
    }

    #[test]
    fn bug805_expired_deadline_rejects_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        assert!(
            runner
                .validate_bug805_oracle(Instant::now() - Duration::from_millis(10))
                .is_err()
        );
    }
}
