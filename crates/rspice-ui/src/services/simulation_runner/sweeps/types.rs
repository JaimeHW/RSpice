//! Sweep result and configuration types.

use rspice_core::Value;

pub(crate) const REFERENCE_MODEL_BINDING_BEGIN: &str = "* RSPICE REFERENCE MODEL BINDING BEGIN";
pub(crate) const REFERENCE_MODEL_BINDING_END: &str = "* RSPICE REFERENCE MODEL BINDING END";

/// Parametric sweep data.
#[derive(Debug, Clone)]
pub struct ParametricData {
    pub target: String,
    pub sweep_values: Vec<Value>,
    pub voltages: Vec<(String, Vec<Value>)>,
    pub num_failures: usize,
}

/// Explicit configuration for temperature sweep execution.
#[derive(Debug, Clone)]
pub struct TempRunConfig {
    pub temperatures_c: Vec<Value>,
    pub base_mode: CornerBaseMode,
}

impl Default for TempRunConfig {
    fn default() -> Self {
        Self {
            temperatures_c: vec![25.0],
            base_mode: CornerBaseMode::Op,
        }
    }
}

impl TempRunConfig {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.temperatures_c.is_empty() {
            return Err("Temperature sweep requires at least one temperature point".to_string());
        }
        if self.temperatures_c.iter().any(|t| !t.is_finite()) {
            return Err("Temperature sweep points must be finite values".to_string());
        }
        validate_base_mode("Temperature sweep", &self.base_mode)
    }
}

/// Process-corner designation for UI corner sweeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CornerProcess {
    TT,
    SS,
    FF,
    SF,
    FS,
}

/// The comment every materialized model block is sealed under in a deck.
///
/// One spelling, in one place, because three things depend on it agreeing: the
/// corner materializer that writes it into the deck an engine reads, the model
/// execution plan that writes it into a reference deck, and the executed-deck
/// archive that reads it back to say which model sources a completed run was
/// actually given. Two spellings would mean a deck a reader can see and a
/// workspace cannot describe.
pub const SEALED_MODEL_SOURCE_MARKER: &str = "* RSpice sealed model source: ";

/// One explicit foundry/library model binding for a process point.
///
/// Model cards are fully materialized from an authenticated in-memory source
/// snapshot before a worker request is created. Workers never receive a model
/// path that could be reopened after verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CornerModelBinding {
    pub process: CornerProcess,
    pub source_label: String,
    pub section: Option<String>,
    pub materialized_model_cards: String,
}

impl CornerModelBinding {
    pub(crate) fn validate(&self) -> Result<(), String> {
        let label = self.source_label.trim();
        if label.is_empty() {
            return Err("Corner model binding requires a source label".to_owned());
        }
        if label.chars().any(char::is_control) {
            return Err(format!(
                "Corner model source label contains a control character and cannot be represented safely: {label:?}"
            ));
        }
        if let Some(section) = self.section.as_deref() {
            let section = section.trim();
            if section.is_empty() {
                return Err("Corner model section cannot be empty".to_owned());
            }
            if section.chars().any(|character| {
                character.is_whitespace()
                    || character == '"'
                    || character == '\''
                    || character.is_control()
            }) {
                return Err(format!(
                    "Corner model section contains an unsupported character: {section}"
                ));
            }
        }
        if self.materialized_model_cards.trim().is_empty() {
            return Err(format!(
                "Corner model binding '{label}' contains no materialized model cards"
            ));
        }
        for line in self.materialized_model_cards.lines() {
            if line.trim().eq_ignore_ascii_case(".end") {
                return Err(format!(
                    "Corner model binding '{label}' contains a terminal .end card"
                ));
            }
            if rspice_core::netlist::parse_include_directive(line).is_some()
                || rspice_core::netlist::parse_lib_directive(line).is_some()
            {
                return Err(format!(
                    "Corner model binding '{label}' contains an unresolved include/library directive"
                ));
            }
        }
        Ok(())
    }
}

impl CornerProcess {
    pub(crate) fn as_keyword(self) -> &'static str {
        match self {
            Self::TT => "TT",
            Self::SS => "SS",
            Self::FF => "FF",
            Self::SF => "SF",
            Self::FS => "FS",
        }
    }
}

/// Explicit configuration for corner sweep execution.
#[derive(Debug, Clone)]
pub struct CornerRunConfig {
    pub process_corners: Vec<CornerProcess>,
    pub voltages: Vec<Value>,
    /// Exact independent voltage-source instances forming the supply domain.
    /// Voltage corners never infer these from grounding or magnitude.
    pub supply_source_names: Vec<String>,
    pub temperatures_c: Vec<Value>,
    pub full_matrix: bool,
    pub nominal_voltage: Option<Value>,
    pub base_mode: CornerBaseMode,
    /// Explicit model sources keyed to process. Non-typical corners must be
    /// backed by a real library section; RSpice never fabricates a foundry
    /// corner by scaling model parameters heuristically.
    pub model_bindings: Vec<CornerModelBinding>,
    /// The exact points to run, when the declared space is not an axis
    /// composition.
    ///
    /// Empty means the axes compose as `full_matrix` says. Non-empty is the
    /// whole space: expansion returns this list unchanged, because a space that
    /// had points removed cannot be recovered from the axes that produced it.
    /// The axes still describe which values the points draw on, and the process
    /// axis still decides which model sections are materialized.
    pub points: Vec<CornerPoint>,
}

/// Frequency sweep type used by corner AC base analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerFrequencySweep {
    Decade,
    Octave,
    Linear,
}

/// Base analysis executed at each corner point.
#[derive(Debug, Clone, Default)]
pub enum CornerBaseMode {
    /// Run DC operating point directly at each corner.
    #[default]
    Op,
    /// Run DC sweep and record the final converged point at each corner.
    DcSweep {
        source_name: String,
        start: Value,
        stop: Value,
        step: Value,
    },
    /// Run an authored two-source nested DC sweep at each point.
    DcSweepNested {
        source_name: String,
        start: Value,
        stop: Value,
        step: Value,
        source2: String,
        start2: Value,
        stop2: Value,
        step2: Value,
    },
    /// Run transient analysis and record the terminal sample at each corner.
    Transient { stop_time: Value, step_time: Value },
    /// Run the complete authored transient window at each point.
    TransientWindow {
        stop_time: Value,
        step_time: Value,
        start_time: Value,
        max_timestep: Option<Value>,
        uic: bool,
    },
    /// Run AC analysis and record terminal-frequency magnitude at each corner.
    Ac {
        start_freq: Value,
        stop_freq: Value,
        points_per_unit: usize,
        sweep: CornerFrequencySweep,
    },
}

impl CornerBaseMode {
    pub(crate) fn metric_label(&self) -> CornerMetricLabel {
        match self {
            Self::Ac { .. } => CornerMetricLabel::AcMagnitude,
            _ => CornerMetricLabel::Voltage,
        }
    }
}

impl Default for CornerRunConfig {
    fn default() -> Self {
        Self {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![1.0],
            supply_source_names: Vec::new(),
            temperatures_c: vec![25.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::default(),
            model_bindings: Vec::new(),
            points: Vec::new(),
        }
    }
}

impl CornerRunConfig {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.process_corners.is_empty() {
            return Err("Corner analysis requires at least one process corner".to_string());
        }
        if self.voltages.is_empty() {
            return Err("Corner analysis requires at least one voltage corner".to_string());
        }
        if self.voltages.iter().any(|v| !v.is_finite() || *v <= 0.0) {
            return Err(
                "Corner analysis voltage corners must be positive finite values".to_string(),
            );
        }
        let mut distinct_supply_values: Vec<u64> = if self.points.is_empty() {
            self.voltages.iter().map(|value| value.to_bits()).collect()
        } else {
            self.points
                .iter()
                .map(|point| point.voltage.to_bits())
                .collect()
        };
        distinct_supply_values.sort_unstable();
        distinct_supply_values.dedup();
        if distinct_supply_values.len() > 1 && self.supply_source_names.is_empty() {
            return Err(
                "Corner voltage sweeping requires explicitly bound supply-source instances"
                    .to_owned(),
            );
        }
        let mut seen_supply_sources = std::collections::BTreeSet::new();
        for source in &self.supply_source_names {
            if source.trim().is_empty()
                || source != source.trim()
                || source.chars().any(char::is_control)
                || !seen_supply_sources.insert(source.to_ascii_lowercase())
            {
                return Err(format!(
                    "Corner supply source binding {source:?} is empty, malformed, or duplicated"
                ));
            }
        }
        if self.temperatures_c.is_empty() {
            return Err("Corner analysis requires at least one temperature corner".to_string());
        }
        if self.temperatures_c.iter().any(|t| !t.is_finite()) {
            return Err("Corner analysis temperature corners must be finite values".to_string());
        }
        if let Some(vnom) = self.nominal_voltage
            && (!vnom.is_finite() || vnom <= 0.0)
        {
            return Err(
                "Corner analysis nominal voltage must be a positive finite value".to_string(),
            );
        }
        for point in &self.points {
            if !point.voltage.is_finite() || point.voltage <= 0.0 {
                return Err(
                    "Explicit corner points must carry positive finite voltages".to_string()
                );
            }
            if !point.temperature_c.is_finite() {
                return Err("Explicit corner points must carry finite temperatures".to_string());
            }
            // Model sections are materialized per process from the process
            // axis, so a point naming a process the axis omits would be solved
            // against whichever section was already loaded.
            if !self.process_corners.contains(&point.process) {
                return Err(format!(
                    "Explicit corner point uses process {} which the process axis does not declare",
                    point.process.as_keyword()
                ));
            }
        }
        // An explicit list is already paired: it states each point in full, so
        // there are no axes left to align and nothing that could be cycled.
        if !self.full_matrix && self.points.is_empty() {
            // A diagonal sweep pairs the axes index by index. Axes of
            // different non-scalar lengths have no such pairing: continuing
            // would silently cycle the shorter one, producing a point set the
            // declaration never asked for and a manifest that could not
            // justify its own labels. A single-valued axis is shared by every
            // point, which is a pairing and is allowed.
            let mut lengths: Vec<usize> = [
                self.process_corners.len(),
                self.voltages.len(),
                self.temperatures_c.len(),
            ]
            .into_iter()
            .filter(|length| *length != 1)
            .collect();
            lengths.sort_unstable();
            lengths.dedup();
            if lengths.len() > 1 {
                return Err(format!(
                    "Diagonal corner sweeps require equal non-scalar axis lengths; this \
                     configuration declares {}. Implicit cycling is not performed.",
                    lengths
                        .iter()
                        .map(usize::to_string)
                        .collect::<Vec<_>>()
                        .join(" and ")
                ));
            }
        }
        for binding in &self.model_bindings {
            binding.validate()?;
        }
        // Every process that is actually solved needs a real library section.
        // With an explicit list that is the list's own processes: a value the
        // axis declares but no point uses is never materialized, and demanding
        // a PDK section for it would refuse a corner that does not run.
        if self.points.is_empty() {
            for process in &self.process_corners {
                require_model_section(*process, &self.model_bindings)?;
            }
        } else {
            for point in &self.points {
                require_model_section(point.process, &self.model_bindings)?;
            }
        }
        validate_base_mode("Corner", &self.base_mode)?;
        Ok(())
    }
}

/// Refuse a non-typical process that no real library section backs.
///
/// RSpice never fabricates a foundry corner by scaling model parameters, so a
/// process the deck cannot resolve through a section is a corner that cannot be
/// run at all.
fn require_model_section(
    process: CornerProcess,
    bindings: &[CornerModelBinding],
) -> Result<(), String> {
    if process == CornerProcess::TT
        || bindings
            .iter()
            .any(|binding| binding.process == process && binding.section.is_some())
    {
        return Ok(());
    }
    Err(format!(
        "{} corner requires an explicit PDK model-library section",
        process.as_keyword()
    ))
}

pub(super) fn validate_base_mode(context: &str, base_mode: &CornerBaseMode) -> Result<(), String> {
    match base_mode {
        CornerBaseMode::Op => {}
        CornerBaseMode::DcSweep {
            source_name,
            start,
            stop,
            step,
        } => {
            if source_name.trim().is_empty() {
                return Err(format!(
                    "{} DC sweep base mode requires a non-empty source name",
                    context
                ));
            }
            if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
                return Err(format!(
                    "{} DC sweep base mode requires finite start/stop/step values",
                    context
                ));
            }
            if *step == 0.0 {
                return Err(format!(
                    "{} DC sweep base mode step cannot be zero",
                    context
                ));
            }
            if (stop - start).abs() > 0.0 && (stop - start).signum() != step.signum() {
                return Err(format!(
                    "{} DC sweep base mode step direction must match start/stop range",
                    context
                ));
            }
        }
        CornerBaseMode::DcSweepNested {
            source_name,
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
        } => {
            validate_dc_base(context, source_name, *start, *stop, *step)?;
            validate_dc_base(context, source2, *start2, *stop2, *step2)?;
            if source_name.eq_ignore_ascii_case(source2) {
                return Err(format!(
                    "{context} nested DC base mode requires two distinct sources"
                ));
            }
        }
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => {
            if !stop_time.is_finite() || *stop_time <= 0.0 {
                return Err(format!(
                    "{} transient base mode stop_time must be a positive finite value",
                    context
                ));
            }
            if !step_time.is_finite() || *step_time <= 0.0 {
                return Err(format!(
                    "{} transient base mode step_time must be a positive finite value",
                    context
                ));
            }
            if step_time > stop_time {
                return Err(format!(
                    "{} transient base mode step_time must be <= stop_time",
                    context
                ));
            }
        }
        CornerBaseMode::TransientWindow {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            ..
        } => {
            validate_transient_base(context, *stop_time, *step_time)?;
            if !start_time.is_finite() || *start_time < 0.0 || start_time >= stop_time {
                return Err(format!(
                    "{context} transient start_time must be finite, non-negative, and below stop_time"
                ));
            }
            if let Some(max_timestep) = max_timestep
                && (!max_timestep.is_finite() || *max_timestep <= 0.0)
            {
                return Err(format!(
                    "{context} transient max_timestep must be a positive finite value"
                ));
            }
        }
        CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            ..
        } => {
            if !start_freq.is_finite() || !stop_freq.is_finite() {
                return Err(format!(
                    "{} AC base mode requires finite start/stop frequencies",
                    context
                ));
            }
            if *start_freq <= 0.0 || *stop_freq <= 0.0 {
                return Err(format!(
                    "{} AC base mode frequencies must be positive values",
                    context
                ));
            }
            if stop_freq < start_freq {
                return Err(format!(
                    "{} AC base mode stop frequency must be >= start frequency",
                    context
                ));
            }
            if *points_per_unit == 0 {
                return Err(format!(
                    "{} AC base mode points_per_unit must be greater than zero",
                    context
                ));
            }
        }
    }
    Ok(())
}

fn validate_dc_base(
    context: &str,
    source_name: &str,
    start: Value,
    stop: Value,
    step: Value,
) -> Result<(), String> {
    if source_name.trim().is_empty() {
        return Err(format!(
            "{context} DC sweep base mode requires a non-empty source name"
        ));
    }
    if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
        return Err(format!(
            "{context} DC sweep base mode requires finite start/stop/step values"
        ));
    }
    if step == 0.0 {
        return Err(format!("{context} DC sweep base mode step cannot be zero"));
    }
    if (stop - start).abs() > 0.0 && (stop - start).signum() != step.signum() {
        return Err(format!(
            "{context} DC sweep base mode step direction must match start/stop range"
        ));
    }
    Ok(())
}

fn validate_transient_base(
    context: &str,
    stop_time: Value,
    step_time: Value,
) -> Result<(), String> {
    if !stop_time.is_finite() || stop_time <= 0.0 {
        return Err(format!(
            "{context} transient base mode stop_time must be a positive finite value"
        ));
    }
    if !step_time.is_finite() || step_time <= 0.0 {
        return Err(format!(
            "{context} transient base mode step_time must be a positive finite value"
        ));
    }
    if step_time > stop_time {
        return Err(format!(
            "{context} transient base mode step_time must be <= stop_time"
        ));
    }
    Ok(())
}

/// One executable corner point.
///
/// Public because a filtered run space is delivered to the executor as its
/// points rather than as axes; the type is a plain PVT triple and exposes no
/// execution machinery.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CornerPoint {
    pub process: CornerProcess,
    pub voltage: Value,
    pub temperature_c: Value,
}

impl CornerPoint {
    pub(super) fn label(&self) -> String {
        format!(
            "{}_{:.6}V_{:.6}C",
            self.process.as_keyword(),
            self.voltage,
            self.temperature_c
        )
    }
}

/// One scalar per node at a single swept point, in a node order the caller
/// keeps identical across points: the mappers pair names and values by index.
#[derive(Debug, Clone)]
pub(crate) struct SweepPointResult {
    pub(crate) node_names: Vec<String>,
    pub(crate) node_values: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CornerMetricLabel {
    Voltage,
    AcMagnitude,
}

impl CornerMetricLabel {
    pub(super) fn format_trace_name(self, node_name: &str) -> String {
        match self {
            Self::Voltage => format!("V({})", node_name),
            Self::AcMagnitude => format!("|V({})|", node_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_typical_corner_requires_real_model_section() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::FF],
            ..CornerRunConfig::default()
        };

        let error = config
            .validate()
            .expect_err("synthetic process scaling must not be accepted");

        assert!(error.contains("explicit PDK model-library section"));
    }

    #[test]
    fn explicit_points_still_require_a_real_model_section_for_every_process_they_run() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::TT, CornerProcess::SS],
            voltages: vec![1.0],
            temperatures_c: vec![25.0],
            points: vec![
                CornerPoint {
                    process: CornerProcess::TT,
                    voltage: 1.0,
                    temperature_c: 25.0,
                },
                CornerPoint {
                    process: CornerProcess::SS,
                    voltage: 1.0,
                    temperature_c: 25.0,
                },
            ],
            ..CornerRunConfig::default()
        };

        let error = config
            .validate()
            .expect_err("an explicit list is not a way around the PDK binding rule");

        assert!(error.contains("SS corner requires an explicit PDK model-library section"));
    }

    #[test]
    fn a_process_no_explicit_point_runs_needs_no_binding() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::TT, CornerProcess::SS],
            voltages: vec![1.0],
            temperatures_c: vec![25.0],
            points: vec![CornerPoint {
                process: CornerProcess::TT,
                voltage: 1.0,
                temperature_c: 25.0,
            }],
            ..CornerRunConfig::default()
        };

        config
            .validate()
            .expect("a declared-but-unreached corner binds nothing");
    }

    #[test]
    fn an_explicit_point_naming_an_undeclared_process_is_refused() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![1.0],
            temperatures_c: vec![25.0],
            points: vec![CornerPoint {
                process: CornerProcess::FF,
                voltage: 1.0,
                temperature_c: 25.0,
            }],
            ..CornerRunConfig::default()
        };

        let error = config.validate().expect_err(
            "a point outside the process axis would be solved against the wrong models",
        );

        assert!(
            error.contains("the process axis does not declare"),
            "{error}"
        );
    }

    #[test]
    fn an_explicit_list_is_exempt_from_the_diagonal_pairing_rule() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![0.9, 1.0, 1.1],
            temperatures_c: vec![-40.0, 125.0],
            full_matrix: false,
            points: vec![CornerPoint {
                process: CornerProcess::TT,
                voltage: 0.9,
                temperature_c: 125.0,
            }],
            ..CornerRunConfig::default()
        };

        config
            .validate()
            .expect("an explicit list states each point in full, so there is nothing to pair");
    }

    #[test]
    fn binding_rejects_unsafe_labels_and_unresolved_directives() {
        let binding = CornerModelBinding {
            process: CornerProcess::FF,
            source_label: "models.lib\n.end".to_owned(),
            section: Some("ff".to_owned()),
            materialized_model_cards: ".model fast D (IS=1e-12)".to_owned(),
        };

        assert!(binding.validate().is_err());

        let unresolved = CornerModelBinding {
            process: CornerProcess::FF,
            source_label: "models.lib [ff]".to_owned(),
            section: Some("ff".to_owned()),
            materialized_model_cards: ".include \"late.inc\"".to_owned(),
        };
        assert!(unresolved.validate().is_err());
    }
}
