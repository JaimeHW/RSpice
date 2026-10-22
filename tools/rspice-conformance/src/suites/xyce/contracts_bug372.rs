use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "BUG_372 native MOS multiplicity wrapper family";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_372";

type Bug372NumericParameters = Vec<(String, u64)>;
type Bug372StringParameters = Vec<(String, String)>;
type Bug372ModelFingerprintEntry = (
    String,
    String,
    Bug372NumericParameters,
    Bug372StringParameters,
);
type Bug372ModelFingerprint = Vec<Bug372ModelFingerprintEntry>;

#[cfg(test)]
const QUALIFIED: [Bug372Role; 4] = [
    Bug372Role::Level2,
    Bug372Role::Level3,
    Bug372Role::Bsim3,
    Bug372Role::Bsim4,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug372Role {
    Level2,
    Level3,
    Bsim3,
    Bsim4,
}

impl Bug372Role {
    pub(super) fn for_record(record: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(record).as_str() {
            "netlists/certification_tests/bug_372/invert2.cir" => Some(Self::Level2),
            "netlists/certification_tests/bug_372/invert3.cir" => Some(Self::Level3),
            "netlists/certification_tests/bug_372/invert_bsim3.cir" => Some(Self::Bsim3),
            "netlists/certification_tests/bug_372/invert_bsim4.cir" => Some(Self::Bsim4),
            _ => None,
        }
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::Level2 => "bug372_level2_multiplicity_wrapper_owner",
            Self::Level3 => "bug372_level3_multiplicity_wrapper_owner",
            Self::Bsim3 => "bug372_bsim3_multiplicity_wrapper_owner",
            Self::Bsim4 => "bug372_bsim4_multiplicity_wrapper_owner",
        }
    }

    const fn owner_name(self) -> &'static str {
        match self {
            Self::Level2 => "invert2.cir",
            Self::Level3 => "invert3.cir",
            Self::Bsim3 => "invert_bsim3.cir",
            Self::Bsim4 => "invert_bsim4.cir",
        }
    }

    const fn control_name(self) -> &'static str {
        match self {
            Self::Level2 => "invert2-sim.cir",
            Self::Level3 => "invert3-sim.cir",
            Self::Bsim3 => "invert_bsim3-sim.cir",
            Self::Bsim4 => "invert_bsim4-sim.cir",
        }
    }

    fn owner_path(self) -> String {
        format!("{FAMILY_DIRECTORY}/{}", self.owner_name())
    }

    fn control_path(self) -> String {
        format!("{FAMILY_DIRECTORY}/{}", self.control_name())
    }

    #[cfg(test)]
    fn owner_record(self) -> String {
        XyceTestRunner::normalize_manifest_key(&self.owner_path())
    }

    const fn multiplicity(self) -> usize {
        match self {
            Self::Level2 | Self::Level3 => 2,
            Self::Bsim3 | Self::Bsim4 => 32,
        }
    }

    const fn model_level(self) -> u8 {
        match self {
            Self::Level2 => 2,
            Self::Level3 => 3,
            Self::Bsim3 => 9,
            Self::Bsim4 => 14,
        }
    }

    const fn print_precision(self) -> usize {
        match self {
            Self::Level2 | Self::Level3 => XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            Self::Bsim3 | Self::Bsim4 => 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug372Representation {
    ExplicitParallel,
    Multiplicity,
    UnitMultiplicityCounterfactual,
}

impl Bug372Representation {
    const fn expected_m(self, role: Bug372Role) -> Option<Value> {
        match self {
            Self::ExplicitParallel => None,
            Self::Multiplicity => Some(role.multiplicity() as Value),
            Self::UnitMultiplicityCounterfactual => Some(1.0),
        }
    }

    const fn expected_mos(self, role: Bug372Role) -> usize {
        match self {
            Self::ExplicitParallel => 2 * role.multiplicity(),
            Self::Multiplicity | Self::UnitMultiplicityCounterfactual => 2,
        }
    }
}

#[derive(Debug, Clone)]
struct Bug372Worker {
    plan: XyceStaticTranPlan,
    netlist: Netlist,
    representation: Bug372Representation,
}

#[derive(Debug)]
struct Bug372Run {
    table: XycePrnTable,
    result: TransientResult,
}

impl XyceTestRunner {
    fn bug372_nodes(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn bug372_param(params: &[(String, Value)], name: &str) -> Option<Value> {
        let values = params
            .iter()
            .filter(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
            .collect::<Vec<_>>();
        match values.as_slice() {
            [value] => Some(*value),
            _ => None,
        }
    }

    fn bug372_pulse(
        spec: &SourceSpec,
    ) -> Option<(Value, Value, Value, Value, Value, Value, Value)> {
        match spec {
            SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                pulse_count,
                width_defaults_to_zero,
            } if pulse_count.to_bits() == 0.0f64.to_bits() && !width_defaults_to_zero => {
                Some((*v1, *v2, *delay, *rise, *fall, *width, *period))
            }
            SourceSpec::DcTransient { transient, .. }
            | SourceSpec::AcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => Self::bug372_pulse(transient),
            _ => None,
        }
    }

    fn bug372_supply(role: Bug372Role) -> Value {
        if role == Bug372Role::Bsim4 { 3.3 } else { 5.0 }
    }

    fn bug372_period(role: Bug372Role) -> Value {
        if matches!(role, Bug372Role::Level2) {
            3.0e-6
        } else {
            3.01e-6
        }
    }

    fn bug372_model_names(role: Bug372Role) -> (&'static str, &'static str) {
        match role {
            Bug372Role::Level2 | Bug372Role::Level3 => ("CD4012_NMOS", "CD4012_PMOS"),
            Bug372Role::Bsim3 => ("CMOSN", "CMOSP"),
            Bug372Role::Bsim4 => ("N1", "P1"),
        }
    }

    fn validate_bug372_models(role: Bug372Role, netlist: &Netlist) -> Result<(), String> {
        if netlist.models.len() != 2 {
            return Err(format!(
                "{LABEL} {role:?} lost its exact two-model card census"
            ));
        }
        let mut polarities = BTreeSet::new();
        for model in &netlist.models {
            let polarity = if model.model_type.eq_ignore_ascii_case("NMOS") {
                "nmos"
            } else if model.model_type.eq_ignore_ascii_case("PMOS") {
                "pmos"
            } else {
                return Err(format!(
                    "{LABEL} {role:?} model polarity changed: {model:?}"
                ));
            };
            polarities.insert(polarity);
            let (nmos_model, pmos_model) = Self::bug372_model_names(role);
            let expected_name = match polarity {
                "nmos" => nmos_model,
                "pmos" => pmos_model,
                _ => unreachable!(),
            };
            let level = Self::bug372_param(&model.params, "LEVEL");
            if !model.name.eq_ignore_ascii_case(expected_name)
                || level.map(Value::to_bits) != Some((role.model_level() as Value).to_bits())
                || model.params.iter().any(|(_, value)| !value.is_finite())
                || !model.expr_params.is_empty()
                || !model.string_vector_params.is_empty()
                || !model.real_vector_params.is_empty()
                || !model.real_vector_expr_params.is_empty()
                || !model.integer_vector_params.is_empty()
            {
                return Err(format!(
                    "{LABEL} {role:?} exact native model changed: {model:?}"
                ));
            }
            if role == Bug372Role::Bsim4 {
                if !matches!(model.string_params.as_slice(), [(name, version)]
                    if name.eq_ignore_ascii_case("VERSION") && version == "4.6.1")
                {
                    return Err(format!("{LABEL} BSIM4 VERSION=4.6.1 identity changed"));
                }
            } else if !model.string_params.is_empty() {
                return Err(format!(
                    "{LABEL} {role:?} acquired string-valued model fields"
                ));
            }
        }
        if polarities != BTreeSet::from(["nmos", "pmos"]) {
            return Err(format!("{LABEL} {role:?} model polarity census changed"));
        }
        Ok(())
    }

    fn bug372_model_fingerprint(netlist: &Netlist) -> Bug372ModelFingerprint {
        let mut models = netlist
            .models
            .iter()
            .map(|model| {
                let mut numeric = model
                    .params
                    .iter()
                    .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
                    .collect::<Vec<_>>();
                numeric.sort();
                let mut strings = model
                    .string_params
                    .iter()
                    .map(|(name, value)| (name.to_ascii_lowercase(), value.clone()))
                    .collect::<Vec<_>>();
                strings.sort();
                (
                    model.name.to_ascii_lowercase(),
                    model.model_type.to_ascii_lowercase(),
                    numeric,
                    strings,
                )
            })
            .collect::<Vec<_>>();
        models.sort();
        models
    }

    fn bug372_mos_signatures(netlist: &Netlist) -> BTreeSet<String> {
        let elements = netlist
            .subcircuits
            .first()
            .map(|subckt| subckt.elements.as_slice())
            .unwrap_or(netlist.elements.as_slice());
        elements
            .iter()
            .filter_map(|element| {
                let ElementKind::Mosfet {
                    model,
                    instance_params,
                    deferred_params,
                    ..
                } = &element.kind
                else {
                    return None;
                };
                let mut params = instance_params
                    .iter()
                    .filter(|(name, _)| !name.eq_ignore_ascii_case("M"))
                    .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
                    .collect::<Vec<_>>();
                params.sort();
                let nodes = element
                    .nodes
                    .iter()
                    .map(|node| node.to_ascii_lowercase())
                    .collect::<Vec<_>>();
                Some(format!(
                    "{}|{nodes:?}|{params:?}|{deferred_params:?}",
                    model.to_ascii_lowercase()
                ))
            })
            .collect()
    }

    fn validate_bug372_topology(
        role: Bug372Role,
        representation: Bug372Representation,
        netlist: &Netlist,
    ) -> Result<(), String> {
        let hierarchical = matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4);
        let expected_top_elements = if hierarchical {
            6
        } else {
            5 + representation.expected_mos(role)
        };
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != expected_top_elements
            || netlist.models.len() != 2
            || netlist.subcircuits.len() != usize::from(hierarchical)
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} typed envelope changed"
            ));
        }
        Self::validate_bug372_models(role, netlist)?;
        let mos_elements = if hierarchical {
            let subckt = &netlist.subcircuits[0];
            if !subckt.name.eq_ignore_ascii_case("INVERTER")
                || !Self::bug372_nodes(&subckt.ports, &["IN", "OUT", "VDD", "GND"])
                || subckt.elements.len() != representation.expected_mos(role)
                || !subckt.initial_conditions.is_empty()
                || !subckt.node_sets.is_empty()
                || !subckt.params.is_empty()
                || !subckt.expr_params.is_empty()
                || !subckt.string_params.is_empty()
                || !subckt.body_params.is_empty()
                || !subckt.body_expr_params.is_empty()
                || !subckt.body_string_params.is_empty()
                || !subckt.body_functions.is_empty()
                || !subckt.local_options.is_empty()
                || subckt.library_ref.is_some()
                || !subckt.nested_subcircuits.is_empty()
            {
                return Err(format!(
                    "{LABEL} {role:?} exact INVERTER definition changed"
                ));
            }
            subckt.elements.iter().collect::<Vec<_>>()
        } else {
            netlist
                .elements
                .iter()
                .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
                .collect::<Vec<_>>()
        };
        let mut nmos = 0usize;
        let mut pmos = 0usize;
        for element in mos_elements {
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                return Err(format!(
                    "{LABEL} {role:?} subcircuit acquired a non-MOS element"
                ));
            };
            let (nmos_model, pmos_model) = Self::bug372_model_names(role);
            let is_n = model.eq_ignore_ascii_case(nmos_model);
            let is_p = model.eq_ignore_ascii_case(pmos_model);
            let expected_nodes: &[&str] = if is_n {
                if matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4) {
                    &["OUT", "IN", "GND", "GND"]
                } else {
                    &["VOUT", "IN", "0", "0"]
                }
            } else if hierarchical {
                &["OUT", "IN", "VDD", "VDD"]
            } else {
                &["VOUT", "IN", "VDD", "VDD"]
            };
            let m = Self::bug372_param(instance_params, "M");
            if element.provenance != ElementProvenance::Authored
                || *compact_syntax
                || !deferred_params.is_empty()
                || !(is_n ^ is_p)
                || !Self::bug372_nodes(&element.nodes, expected_nodes)
                || m.map(Value::to_bits) != representation.expected_m(role).map(Value::to_bits)
                || instance_params.iter().any(|(_, value)| !value.is_finite())
            {
                return Err(format!(
                    "{LABEL} {role:?} MOS topology/multiplicity changed: {element:?}"
                ));
            }
            nmos += usize::from(is_n);
            pmos += usize::from(is_p);
        }
        let expected_per_polarity = representation.expected_mos(role) / 2;
        if nmos != expected_per_polarity || pmos != expected_per_polarity {
            return Err(format!("{LABEL} {role:?} NMOS/PMOS census changed"));
        }

        let find = |name: &str| {
            netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("{LABEL} {role:?} lost top-level {name}"))
        };
        let supply = Self::bug372_supply(role);
        let vdd = find("VDDdev")?;
        let rin = find("RIN")?;
        let vin = find("VIN1")?;
        let r1 = find("R1")?;
        let c2 = find("C2")?;
        let xinv = hierarchical.then(|| find("XINV1")).transpose()?;
        if !Self::bug372_nodes(&vdd.nodes, &["VDD", "0"])
            || !matches!(vdd.kind, ElementKind::VoltageSource(SourceSpec::Dc(value)) if value.to_bits() == supply.to_bits())
            || !Self::bug372_nodes(&rin.nodes, &["IN", "1"])
            || !matches!(&rin.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 1_000.0f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            || !Self::bug372_nodes(&vin.nodes, &["1", "0"])
            || !Self::bug372_nodes(&r1.nodes, &["VOUT", "0"])
            || !matches!(&r1.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 10_000.0f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            || !Self::bug372_nodes(&c2.nodes, &["VOUT", "0"])
            || !matches!(&c2.kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 0.1e-12f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            || xinv.is_some_and(|xinv| {
                !Self::bug372_nodes(&xinv.nodes, &["IN", "VOUT", "VDD", "0"])
                    || !matches!(&xinv.kind, ElementKind::Subcircuit { subckt_name, params }
                        if subckt_name.eq_ignore_ascii_case("INVERTER") && params.is_empty())
            })
        {
            return Err(format!("{LABEL} {role:?} exact top-level topology changed"));
        }
        let pulse = match &vin.kind {
            ElementKind::VoltageSource(spec) => Self::bug372_pulse(spec),
            _ => None,
        };
        let expected_pulse = (
            supply,
            0.0,
            1.5e-6,
            5.0e-9,
            5.0e-9,
            1.5e-6,
            Self::bug372_period(role),
        );
        if pulse.map(|values| {
            [
                values.0, values.1, values.2, values.3, values.4, values.5, values.6,
            ]
            .map(Value::to_bits)
        }) != Some(
            [
                expected_pulse.0,
                expected_pulse.1,
                expected_pulse.2,
                expected_pulse.3,
                expected_pulse.4,
                expected_pulse.5,
                expected_pulse.6,
            ]
            .map(Value::to_bits),
        ) {
            return Err(format!(
                "{LABEL} {role:?} ideal VIN1 pulse changed: {pulse:?}"
            ));
        }
        Ok(())
    }

    fn bug372_expected_census(
        role: Bug372Role,
        representation: Bug372Representation,
    ) -> (usize, usize, usize, usize, &'static str) {
        let owner = representation == Bug372Representation::ExplicitParallel;
        match (role, owner) {
            (Bug372Role::Level2 | Bug372Role::Level3, true) => (12, 14, 17, 4, "MOSFET"),
            (Bug372Role::Level2 | Bug372Role::Level3, false) => (8, 10, 11, 2, "MOSFET"),
            (Bug372Role::Bsim3, true) => (4, 6, 69, 64, "BSIM3"),
            (Bug372Role::Bsim3, false) => (4, 6, 7, 2, "BSIM3"),
            (Bug372Role::Bsim4, true) => (68, 70, 133, 64, "BSIM4"),
            (Bug372Role::Bsim4, false) => (6, 8, 9, 2, "BSIM4"),
        }
    }

    fn validate_bug372_worker(
        &self,
        role: Bug372Role,
        representation: Bug372Representation,
        plan: XyceStaticTranPlan,
        expected_source: &str,
        path: &Path,
    ) -> Result<Bug372Worker, String> {
        let canonical_plan_source =
            Self::canonical_lf_text_identity(LABEL, plan.source.as_bytes())?;
        let expected_probes: &[&str] = if role == Bug372Role::Level3 {
            &["v(vout)", "v(in)"]
        } else {
            &["v(vout)", "v(in)", "v(1)"]
        };
        if plan.deck_path != path
            || canonical_plan_source.as_slice() != expected_source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !plan.steps.is_empty()
            || plan.tran.step.to_bits() != 20.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 30.0e-6f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print
                    .probes
                    .iter()
                    .map(String::as_str)
                    .ne(expected_probes.iter().copied())
            })
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} transient plan changed: {plan:?}"
            ));
        }
        let netlist = Self::parse_xyce_netlist(expected_source, path)
            .map_err(|error| format!("{LABEL} {role:?} Xyce parse failed: {error}"))?;
        Self::validate_bug372_topology(role, representation, &netlist)?;
        let expected_reltol: Value = match role {
            Bug372Role::Level2 => 1.0e-4,
            Bug372Role::Level3 => 5.0e-5,
            Bug372Role::Bsim3 | Bug372Role::Bsim4 => 5.0e-3,
        };
        let expected_abstol =
            matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4).then_some(1.0e-3f64.to_bits());
        let expected_newlte = (role == Bug372Role::Level3
            && representation != Bug372Representation::ExplicitParallel)
            .then_some(1);
        let expected_tr_partition = (role == Bug372Role::Bsim3
            && representation == Bug372Representation::ExplicitParallel)
            .then_some(false);
        if netlist.options.timeint_reltol.map(Value::to_bits) != Some(expected_reltol.to_bits())
            || netlist.options.timeint_abstol.map(Value::to_bits) != expected_abstol
            || netlist
                .options
                .transient_lte_reference
                .and_then(TransientLteReference::xyce_selector)
                != expected_newlte
            || netlist.options.linsol_tr_partition != expected_tr_partition
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} TIMEINT options changed: {:?}",
                netlist.options
            ));
        }
        let request = &netlist.output_requests[0];
        let expected_precision =
            matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4).then_some(10);
        let expected_width = matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4).then_some(19);
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || request.print_precision != expected_precision
            || request.print_width != expected_width
            || request
                .operands
                .iter()
                .map(String::as_str)
                .ne(expected_probes.iter().copied())
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} exact PRINT changed: {request:?}"
            ));
        }
        let circuit = self
            .create_xyce_engine()
            .build_circuit(&netlist)
            .map_err(|error| format!("{LABEL} {role:?} assembly failed: {error}"))?;
        let (nodes, matrix, devices, op_count, kind) =
            Self::bug372_expected_census(role, representation);
        let branches = circuit.branch_names_sorted();
        let report = circuit.device_op_report();
        let names = report
            .entries
            .iter()
            .map(|entry| entry.name.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        if circuit.has_generated_veriloga_devices()
            || circuit.num_nodes() != nodes
            || circuit.matrix_size() != matrix
            || circuit.device_count() != devices
            || branches.len() != 2
            || branches.iter().map(String::as_str).ne(["VDDDEV", "VIN1"])
            || report.entries.len() != op_count
            || names.len() != op_count
            || !report.labels_resolve()
            || report
                .entries
                .iter()
                .any(|entry| !entry.device_kind.eq_ignore_ascii_case(kind))
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} native-route census changed: nodes={}/{nodes}, matrix={}/{matrix}, devices={}/{devices}, branches={branches:?}, report={report:?}",
                circuit.num_nodes(),
                circuit.matrix_size(),
                circuit.device_count()
            ));
        }
        Ok(Bug372Worker {
            plan,
            netlist,
            representation,
        })
    }

    fn bug372_unit_source(role: Bug372Role, control: &str) -> Result<String, String> {
        let authored = format!("M={}", role.multiplicity());
        let mut occurrences = 0usize;
        let mut lines = Vec::new();
        for line in control.lines() {
            let trimmed = line.trim_start();
            let rewritten = if !trimmed.starts_with('*') {
                occurrences += line
                    .split_ascii_whitespace()
                    .filter(|token| *token == authored)
                    .count();
                line.replace(&format!(" {authored}"), " M=1")
            } else {
                line.to_string()
            };
            lines.push(rewritten);
        }
        if occurrences != 2 {
            return Err(format!(
                "{LABEL} {role:?} cannot derive exact M=1 counterfactual: found {occurrences} {authored} assignments"
            ));
        }
        let mut source = lines.join("\n");
        source.push('\n');
        Ok(source)
    }

    fn validate_bug372_pair_structure(
        role: Bug372Role,
        owner: &Netlist,
        control: &Netlist,
        unit: &Netlist,
    ) -> Result<(), String> {
        let owner_models = Self::bug372_model_fingerprint(owner);
        if owner_models != Self::bug372_model_fingerprint(control)
            || owner_models != Self::bug372_model_fingerprint(unit)
        {
            return Err(format!("{LABEL} {role:?} owner/control model cards differ"));
        }
        let owner_mos = Self::bug372_mos_signatures(owner);
        if owner_mos.len() != 2
            || owner_mos != Self::bug372_mos_signatures(control)
            || owner_mos != Self::bug372_mos_signatures(unit)
        {
            return Err(format!(
                "{LABEL} {role:?} explicit/M/control device semantics differ after removing M"
            ));
        }
        Ok(())
    }

    fn bug372_tolerances(role: Bug372Role) -> Vec<XyceVerifyTransientTolerance> {
        let default = XyceVerifyTransientTolerance::release_7_10_default();
        match role {
            Bug372Role::Level2 => vec![
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 5.0e-7,
                    ..default
                },
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 5.0e-7,
                    zero: 1.0e-8,
                    ..default
                },
                default,
            ],
            Bug372Role::Level3 => vec![
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 1.0e-6,
                    ..default
                },
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 1.0e-6,
                    zero: 1.0e-6,
                    absolute_difference: 1.0e-6,
                    ..default
                },
            ],
            Bug372Role::Bsim3 | Bug372Role::Bsim4 => vec![default; 3],
        }
    }

    fn validate_bug372_comp_source(role: Bug372Role, source: &str) -> Result<(), String> {
        let probes = if role == Bug372Role::Level3 {
            vec!["v(vout)".to_string(), "v(in)".to_string()]
        } else {
            vec![
                "v(vout)".to_string(),
                "v(in)".to_string(),
                "v(1)".to_string(),
            ]
        };
        let expected = Self::bug372_tolerances(role);
        if matches!(role, Bug372Role::Bsim3 | Bug372Role::Bsim4) {
            if Self::source_has_comp_directive(source) {
                return Err(format!(
                    "{LABEL} {role:?} unexpectedly acquired *COMP overrides"
                ));
            }
        } else {
            let actual = Self::xyce_verify_comp_tolerances(source, &probes)
                .map_err(|error| format!("{LABEL} {role:?} *COMP parsing failed: {error}"))?;
            if actual != expected {
                return Err(format!(
                    "{LABEL} {role:?} exact Release *COMP tolerances changed: {actual:?}"
                ));
            }
        }
        Ok(())
    }

    fn bug372_node_trace<'a>(
        role: Bug372Role,
        result: &'a TransientResult,
        node: &str,
    ) -> Result<&'a [Value], String> {
        let index = result
            .node_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(node))
            .ok_or_else(|| format!("{LABEL} {role:?} result lost node {node}"))?;
        result
            .voltages
            .get(index)
            .map(Vec::as_slice)
            .filter(|trace| trace.len() == result.time.len())
            .ok_or_else(|| format!("{LABEL} {role:?} node {node} trace is incomplete"))
    }

    fn bug372_pulse_value(role: Bug372Role, time: Value) -> Value {
        let v1 = Self::bug372_supply(role);
        let v2 = 0.0;
        let delay = 1.5e-6;
        let rise = 5.0e-9;
        let fall = 5.0e-9;
        let width = 1.5e-6;
        let period = Self::bug372_period(role);
        if time <= delay {
            return v1;
        }
        let phase = (time - delay).rem_euclid(period);
        if phase < rise {
            v1 + (v2 - v1) * phase / rise
        } else if phase < rise + width {
            v2
        } else if phase < rise + width + fall {
            v2 + (v1 - v2) * (phase - rise - width) / fall
        } else {
            v1
        }
    }

    fn validate_bug372_nontrivial_columns(
        role: Bug372Role,
        table: &XycePrnTable,
    ) -> Result<(), String> {
        if !(100..=20_000).contains(&table.rows.len()) {
            return Err(format!("{LABEL} {role:?} output row envelope changed"));
        }
        let mut ranges = [(Value::INFINITY, Value::NEG_INFINITY); 2];
        for row in &table.rows {
            if row.len() != table.columns.len() || row.iter().any(|value| !value.is_finite()) {
                return Err(format!("{LABEL} {role:?} output contains malformed values"));
            }
            for (range, value) in ranges.iter_mut().zip(&row[2..4]) {
                range.0 = range.0.min(*value);
                range.1 = range.1.max(*value);
            }
        }
        let supply = Self::bug372_supply(role);
        let bounded = ranges
            .iter()
            .all(|(min, max)| *min >= -1.0 && *max <= supply + 1.0);
        // The BSIM4 control represents 32 devices behind the authored 1 kOhm
        // input resistor, so its physical gate loading limits V(IN) to about
        // 0.229*VDD even while the printed ideal V(1) pulse spans full VDD.
        let vin_span_fraction = if role == Bug372Role::Bsim4 {
            0.20
        } else {
            0.55
        };
        if !bounded
            || ranges[0].1 - ranges[0].0 < 0.55 * supply
            || ranges[1].1 - ranges[1].0 < vin_span_fraction * supply
        {
            return Err(format!(
                "{LABEL} {role:?} waveform became shared-zero, invariant, or out of range: {ranges:?}"
            ));
        }
        Ok(())
    }

    fn validate_bug372_result(
        role: Bug372Role,
        representation: Bug372Representation,
        table: &XycePrnTable,
        result: &TransientResult,
    ) -> Result<(), String> {
        let expected_columns: &[&str] = if role == Bug372Role::Level3 {
            &["Index", "TIME", "V(VOUT)", "V(IN)"]
        } else {
            &["Index", "TIME", "V(VOUT)", "V(IN)", "V(1)"]
        };
        if table.columns.len() != expected_columns.len()
            || table
                .columns
                .iter()
                .zip(expected_columns)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() != result.time.len()
            || result.step_sizes.len() != result.time.len()
        {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} PRN/result shape changed"
            ));
        }
        Self::validate_bug372_nontrivial_columns(role, table)?;
        let vout = Self::bug372_node_trace(role, result, "VOUT")?;
        let vin = Self::bug372_node_trace(role, result, "IN")?;
        let v1 = (expected_columns.len() == 5)
            .then(|| Self::bug372_node_trace(role, result, "1"))
            .transpose()?;
        let mut previous = None;
        for (index, (row, time)) in table.rows.iter().zip(&result.time).enumerate() {
            let expected_source = Self::bug372_pulse_value(role, *time);
            if row[0].to_bits() != (index as Value).to_bits()
                || row[1].to_bits() != time.to_bits()
                || row[2].to_bits() != vout[index].to_bits()
                || row[3].to_bits() != vin[index].to_bits()
                || v1.is_some_and(|trace| {
                    row[4].to_bits() != trace[index].to_bits()
                        || (trace[index] - expected_source).abs() > 1.0e-8
                })
                || previous.is_some_and(|prior| *time <= prior)
            {
                return Err(format!(
                    "{LABEL} {role:?} {representation:?} raw solution/ideal-source linkage failed at row {index}: {row:?}"
                ));
            }
            previous = Some(*time);
        }
        let first = table.rows.first().expect("validated nonempty");
        let last = table.rows.last().expect("validated nonempty");
        if first[1].abs() > 1.0e-18 || (last[1] - 30.0e-6).abs() > 1.0e-13 {
            return Err(format!(
                "{LABEL} {role:?} {representation:?} transient horizon changed: first={first:?}, last={last:?}"
            ));
        }
        Ok(())
    }

    fn run_bug372_worker(
        &self,
        role: Bug372Role,
        worker: &Bug372Worker,
        start: Instant,
    ) -> Result<Bug372Run, String> {
        let max_step = Self::transient_family_max_step(&worker.netlist, &worker.plan.tran)
            .map_err(|error| format!("{LABEL} {role:?} max-step validation failed: {error}"))?;
        let engine = self.create_xyce_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let result = engine
            .run_tran_with_startup_mode_and_abort(
                &worker.netlist,
                worker.plan.tran.stop,
                max_step,
                TransientStartupMode::from_uic(worker.plan.tran.uic),
                &abort,
            )
            .map_err(|error| {
                format!(
                    "{LABEL} {role:?} {:?} execution failed: {error}",
                    worker.representation
                )
            })?;
        let quality = engine.convergence_quality();
        if quality.gmin_stepping_count != 0
            || quality.source_stepping_count != 0
            || quality.force_accepted_points != 0
            || !quality.force_accepted_indices.is_empty()
            || quality.failure_diagnostic.is_some()
        {
            return Err(format!(
                "{LABEL} {role:?} {:?} returned a numerically unqualified waveform: {quality:?}",
                worker.representation
            ));
        }
        let table =
            Self::transient_family_result_to_prn_table(&worker.plan, &worker.netlist, &result)?;
        Self::validate_bug372_result(role, worker.representation, &table, &result)?;
        Ok(Bug372Run { table, result })
    }

    fn validate_bug372_relation(
        &self,
        role: Bug372Role,
        owner: &Bug372Run,
        control: &Bug372Run,
        unit: &Bug372Run,
    ) -> Result<(), String> {
        if std::ptr::eq(owner, control)
            || std::ptr::eq(owner, unit)
            || std::ptr::eq(control, unit)
            || owner.result.time.as_ptr() == control.result.time.as_ptr()
            || owner.result.time.as_ptr() == unit.result.time.as_ptr()
        {
            return Err(format!("{LABEL} {role:?} independent runs became aliased"));
        }
        let tolerances = Self::bug372_tolerances(role);
        let mismatches = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
            &owner.table,
            &control.table,
            &tolerances,
            role.print_precision(),
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} {role:?} M-control TEST differs from explicit-parallel GOOD: {mismatches:?}"
            ));
        }
        let ignored_m = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
            &owner.table,
            &unit.table,
            &tolerances,
            role.print_precision(),
        )?;
        if ignored_m.is_empty() {
            return Err(format!(
                "{LABEL} {role:?} M=1 counterfactual still passes, so multiplicity is not independently observed"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug372_oracle(
        &self,
        role: Bug372Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let members = [role.owner_name(), role.control_name()]
            .into_iter()
            .map(|name| {
                let path = self.root.join(FAMILY_DIRECTORY).join(name);
                Ok((
                    name.to_ascii_lowercase(),
                    Self::read_corpus_canonical(LABEL, &path)?,
                ))
            })
            .collect::<Result<BTreeMap<_, _>, String>>()?;
        let source = |name: &str| -> Result<&str, String> {
            std::str::from_utf8(
                members
                    .get(&name.to_ascii_lowercase())
                    .ok_or_else(|| format!("{LABEL} lost retained member {name}"))?,
            )
            .map_err(|error| format!("{LABEL} member {name} is not UTF-8: {error}"))
        };
        let owner_source = source(role.owner_name())?;
        let control_source = source(role.control_name())?;
        Self::validate_bug372_comp_source(role, owner_source)?;
        let unit_source = Self::bug372_unit_source(role, control_source)?;
        let purpose = XyceStaticTranPlanPurpose::Bug372MultiplicityRelationalFamily;
        let owner_path = self.root.join(role.owner_path());
        let control_path = self.root.join(role.control_path());
        let owner_plan = self.static_tran_plan_for_sealed_source_with_purpose(
            &owner_path,
            owner_source,
            purpose,
        )?;
        let control_plan = self.static_tran_plan_for_sealed_source_with_purpose(
            &control_path,
            control_source,
            purpose,
        )?;
        let unit_path = self.root.join(FAMILY_DIRECTORY).join(format!(
            "__rspice_{}_unit_m_counterfactual.cir",
            role.owner_name()
        ));
        let mut unit_plan = control_plan.clone();
        unit_plan.deck_path = unit_path.clone();
        unit_plan.source = unit_source.clone();
        let owner = self.validate_bug372_worker(
            role,
            Bug372Representation::ExplicitParallel,
            owner_plan,
            owner_source,
            &owner_path,
        )?;
        let control = self.validate_bug372_worker(
            role,
            Bug372Representation::Multiplicity,
            control_plan,
            control_source,
            &control_path,
        )?;
        let unit = self.validate_bug372_worker(
            role,
            Bug372Representation::UnitMultiplicityCounterfactual,
            unit_plan,
            &unit_source,
            &unit_path,
        )?;
        Self::validate_bug372_pair_structure(
            role,
            &owner.netlist,
            &control.netlist,
            &unit.netlist,
        )?;
        let owner_run = self.run_bug372_worker(role, &owner, start)?;
        let control_run = self.run_bug372_worker(role, &control, start)?;
        let unit_run = self.run_bug372_worker(role, &unit, start)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} {role:?} execution exceeded deadline"));
        }
        self.validate_bug372_relation(role, &owner_run, &control_run, &unit_run)
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
    fn bug372_routes_exactly_four_owner_records() {
        for role in QUALIFIED {
            assert_eq!(Bug372Role::for_record(&role.owner_record()), Some(role));
            assert_eq!(Bug372Role::for_record(&role.control_path()), None);
        }
        for unsupported in ["invert1.cir", "invert6.cir", "invert_b3soi.cir"] {
            assert_eq!(
                Bug372Role::for_record(&format!("{FAMILY_DIRECTORY}/{unsupported}")),
                None
            );
        }
    }

    #[test]
    fn bug372_level2_checked_in_owner_executes_complete_oracle() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let result = runner.run_test(root.join(Bug372Role::Level2.owner_path()));
        assert!(result.passed, "BUG372 LEVEL2 failed: {:?}", result.error);
        assert_eq!(result.contract, Bug372Role::Level2.contract());
    }

    #[test]
    fn bug372_unit_counterfactual_rewrites_only_the_two_authored_m_assignments() {
        let root = corpus_root();
        for role in QUALIFIED {
            let control = fs::read_to_string(root.join(role.control_path()))
                .expect("checked-in BUG372 control exists");
            let unit = XyceTestRunner::bug372_unit_source(role, &control)
                .expect("exact M=1 counterfactual derives");
            assert_eq!(
                unit.split_ascii_whitespace()
                    .filter(|token| *token == "M=1")
                    .count(),
                2
            );
            assert_eq!(
                unit.split_ascii_whitespace()
                    .filter(|token| *token == format!("M={}", role.multiplicity()))
                    .count(),
                0
            );
            assert_eq!(
                unit.matches("TNOM=27").count(),
                control.matches("TNOM=27").count(),
                "counterfactual must not rewrite model parameters containing the digit 2"
            );
            let path = root.join(role.control_path());
            let netlist = XyceTestRunner::parse_xyce_netlist(&unit, &path).unwrap();
            XyceTestRunner::validate_bug372_topology(
                role,
                Bug372Representation::UnitMultiplicityCounterfactual,
                &netlist,
            )
            .unwrap();
        }
    }

    #[test]
    fn bug372_optional_parameter_lookup_is_non_panicking_and_exact() {
        let params = vec![("L".to_string(), 5.0e-6)];
        assert_eq!(XyceTestRunner::bug372_param(&params, "M"), None);
        assert_eq!(XyceTestRunner::bug372_param(&params, "LEVEL"), None);
        let one = vec![("M".to_string(), 2.0)];
        assert_eq!(XyceTestRunner::bug372_param(&one, "m"), Some(2.0));
        let repeated = vec![("M".to_string(), 2.0), ("m".to_string(), 2.0)];
        assert_eq!(XyceTestRunner::bug372_param(&repeated, "M"), None);
    }

    #[test]
    fn bug372_sealed_source_planner_does_not_reopen_the_deck_path() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let source = fs::read_to_string(runner.root.join(Bug372Role::Level2.owner_path())).unwrap();
        let missing_path = runner
            .root
            .join(FAMILY_DIRECTORY)
            .join("__rspice_bug372_missing_owner.cir");
        assert!(!missing_path.exists());
        let deck = XyceDeck {
            path: missing_path.clone(),
            section: XyceDeckSection::Netlists,
            relative_path: Bug372Role::Level2.owner_path(),
        };
        let plan = runner
            .static_tran_plan_for_deck_with_sealed_source_and_purpose(
                &deck,
                &source,
                XyceStaticTranPlanPurpose::Bug372MultiplicityRelationalFamily,
            )
            .expect("authenticated source plans without reading the nonexistent deck path");
        assert_eq!(plan.deck_path, missing_path);
        assert_eq!(plan.source, source);
    }

    #[test]
    fn bug372_topology_rejects_a_wrong_native_model_route() {
        let root = corpus_root();
        let path = root.join(Bug372Role::Level2.owner_path());
        let source = fs::read_to_string(&path).expect("checked-in BUG372 owner exists");
        let netlist = XyceTestRunner::parse_xyce_netlist(&source, &path).unwrap();
        XyceTestRunner::validate_bug372_topology(
            Bug372Role::Level2,
            Bug372Representation::ExplicitParallel,
            &netlist,
        )
        .unwrap();

        let wrong_level = source.replacen("LEVEL = 2", "LEVEL = 9", 1);
        assert_ne!(wrong_level, source);
        let wrong = XyceTestRunner::parse_xyce_netlist(&wrong_level, &path).unwrap();
        assert!(
            XyceTestRunner::validate_bug372_topology(
                Bug372Role::Level2,
                Bug372Representation::ExplicitParallel,
                &wrong,
            )
            .is_err()
        );
    }

    #[test]
    fn bug372_shared_zero_can_pass_history_but_not_the_independent_waveform_gate() {
        let columns = vec![
            "Index".into(),
            "TIME".into(),
            "V(VOUT)".into(),
            "V(IN)".into(),
            "V(1)".into(),
        ];
        let rows = (0..100)
            .map(|index| vec![index as Value, index as Value * 1.0e-9, 0.0, 0.0, 0.0])
            .collect::<Vec<_>>();
        let shared_zero = XycePrnTable { columns, rows };
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &shared_zero,
                    &shared_zero,
                    &XyceTestRunner::bug372_tolerances(Bug372Role::Level2),
                    Bug372Role::Level2.print_precision(),
                )
                .unwrap()
                .is_empty(),
            "the historical comparator alone admits identical shared-wrong streams"
        );
        assert!(
            XyceTestRunner::validate_bug372_nontrivial_columns(Bug372Role::Level2, &shared_zero,)
                .is_err()
        );
    }

    #[test]
    fn bug372_release_direction_and_threshold_are_fail_closed() {
        let table = |value: Value| XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(X)".into()],
            rows: vec![vec![0.0, 0.0, value], vec![1.0, 1.0, value]],
        };
        let good = table(1.0);
        let threshold = table(0.0);
        let over_threshold = table(-0.01);
        let tolerance = XyceVerifyTransientTolerance {
            relative: 1.0,
            absolute: 1.0e-12,
            zero: 0.0,
            absolute_difference: 0.0,
            offset: 0.0,
        };
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &good,
                    &threshold,
                    &[tolerance],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )
                .unwrap()
                .is_empty(),
            "GOOD-denominator comparison must accept the exact Release threshold"
        );
        assert!(
            !runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &threshold,
                    &good,
                    &[tolerance],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )
                .unwrap()
                .is_empty(),
            "reversing GOOD and TEST must not silently change the denominator"
        );
        assert!(
            !runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &good,
                    &over_threshold,
                    &[tolerance],
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )
                .unwrap()
                .is_empty(),
            "integrated normalized RMS values above one must fail"
        );
    }
}
