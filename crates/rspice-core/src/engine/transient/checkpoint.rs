//! Transient checkpoint/restore.
//!
//! A checkpoint captures the integrator state at an accepted time point:
//! the full MNA solution plus the capacitor and inductor companion-model
//! histories. Restoring injects that state into a freshly built circuit and
//! continues integration from the checkpoint time with absolute-time source
//! evaluation — the same numerical regime as a breakpoint restart, which
//! the integrator already performs at every source discontinuity.
//!
//! Scope, stated precisely: accepted linear-reactive state and histories are
//! captured bit-exactly. Nonlinear charge histories and accepted-step timing
//! provenance are not serialized, so continuation deliberately takes one
//! order-one breakpoint-restart step before higher-order integration resumes.
//! Nonlinear iteration memories and transmission-line delay histories likewise
//! re-derive from the restored solution. Decks dominated by transmission-line
//! delays should prefer unsegmented runs (a warning is logged at capture).
//!
//! The on-disk format is a versioned, line-oriented text format using
//! Rust's shortest-round-trip float formatting, so save/load reproduces
//! every `f64` bit-exactly with no serialization dependencies (core stays
//! lean for the wasm build).

use crate::Value;
use crate::analysis::LteEstimator;
use crate::circuit::Circuit;
use crate::netlist::{ElementKind, Netlist, TransientLteReference};
use crate::xspice::{CmContextCheckpoint, XspiceInstanceCheckpoint};

/// Format version written to and required from checkpoint files.
const FORMAT_VERSION: u32 = 6;

/// Snapshot of transient-integration state at an accepted time point.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientCheckpoint {
    /// Simulation time of the snapshot (s).
    pub time: Value,
    /// Full MNA solution (node voltages then branch currents).
    pub solution: Vec<Value>,
    /// Fingerprint of the netlist this state belongs to; restore refuses a
    /// mismatch rather than silently continuing a different circuit.
    pub netlist_fingerprint: u64,

    cap_v_prev: Vec<Value>,
    cap_v_prev_prev: Vec<Value>,
    cap_v_prev_prev_prev: Vec<Value>,
    cap_i_prev: Vec<Value>,
    cap_i_eq: Vec<Value>,
    ind_i_prev: Vec<Value>,
    ind_i_prev_prev: Vec<Value>,
    ind_v_prev: Vec<Value>,
    lte_signal_global_reference: Value,
    lte_signal_local_reference: Vec<Value>,
    lte_reference_history_available: bool,
    lte_reference_mode: Option<TransientLteReference>,
    xspice_instances: Vec<String>,
    xspice_resume_blockers: Vec<String>,
    xspice_instance_states: Vec<XspiceInstanceCheckpoint>,
}

/// Stable fingerprint of the netlist identity (FNV-1a over the source text
/// when available, else over the flattened element signature).
pub fn netlist_fingerprint(netlist: &Netlist) -> u64 {
    const OFFSET: u64 = 0xCBF2_9CE4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01B3;
    let mut hash = OFFSET;
    let mut feed = |bytes: &[u8]| {
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
    };

    if let Some(text) = &netlist.source_text {
        feed(text.as_bytes());
    } else {
        for element in &netlist.elements {
            feed(element.name.as_bytes());
            for node in &element.nodes {
                feed(node.as_bytes());
            }
        }
    }
    hash
}

fn parse_count_header(line: &str, name: &str) -> Result<usize, String> {
    let mut fields = line.split_whitespace();
    let section = fields
        .next()
        .ok_or_else(|| format!("malformed '{name}' header: '{line}'"))?;
    if section != name {
        return Err(format!("malformed '{name}' header: '{line}'"));
    }
    let count = fields
        .next()
        .ok_or_else(|| format!("malformed '{name}' header: '{line}'"))?
        .parse::<usize>()
        .map_err(|_| format!("malformed '{name}' header: '{line}'"))?;
    if let Some(extra) = fields.next() {
        return Err(format!("malformed '{name}' header: extra field '{extra}'"));
    }
    Ok(count)
}

struct CheckpointLines<'a> {
    inner: std::str::Lines<'a>,
    remaining: usize,
}

impl<'a> CheckpointLines<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            inner: text.lines(),
            remaining: text.lines().count(),
        }
    }

    fn remaining(&self) -> usize {
        self.remaining
    }
}

impl<'a> Iterator for CheckpointLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.inner.next()?;
        self.remaining -= 1;
        Some(line)
    }
}

fn allocate_checkpoint_rows<T>(
    lines: &CheckpointLines<'_>,
    count: usize,
    name: &str,
) -> Result<Vec<T>, String> {
    let remaining_rows = lines.remaining();
    if count > remaining_rows {
        return Err(format!(
            "'{name}' declares {count} rows but only {remaining_rows} checkpoint rows remain"
        ));
    }

    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| format!("'{name}' count {count} exceeds checkpoint allocation limits"))?;
    Ok(values)
}

fn write_value_vector(out: &mut String, name: &str, values: &[Value]) {
    out.push_str(&format!("{name} {}\n", values.len()));
    for value in values {
        out.push_str(&value.to_string());
        out.push('\n');
    }
}

fn write_i64_vector(out: &mut String, name: &str, values: &[i64]) {
    out.push_str(&format!("{name} {}\n", values.len()));
    for value in values {
        out.push_str(&value.to_string());
        out.push('\n');
    }
}

fn read_value_vector(lines: &mut CheckpointLines<'_>, name: &str) -> Result<Vec<Value>, String> {
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' vector"))?;
    let count = parse_count_header(header, name)?;
    let mut values = allocate_checkpoint_rows(lines, count, name)?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'{name}' vector truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        let field = fields
            .next()
            .ok_or_else(|| format!("'{name}' row {row} is empty"))?;
        let value = field
            .parse::<Value>()
            .map_err(|_| format!("'{name}' row {row}: bad value '{field}'"))?;
        if let Some(extra) = fields.next() {
            return Err(format!("'{name}' row {row}: extra field '{extra}'"));
        }
        values.push(value);
    }
    Ok(values)
}

fn read_value_section(
    lines: &mut CheckpointLines<'_>,
    name: &str,
    columns: usize,
) -> Result<Vec<Vec<Value>>, String> {
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' section"))?;
    let count = parse_count_header(header, name)?;
    if columns == 0 {
        return Err(format!("'{name}' section must have at least one column"));
    }
    let mut cols = Vec::new();
    cols.try_reserve_exact(columns)
        .map_err(|_| format!("'{name}' column count {columns} exceeds allocation limits"))?;
    for _ in 0..columns {
        cols.push(allocate_checkpoint_rows(lines, count, name)?);
    }
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'{name}' truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        for col in &mut cols {
            let field = fields
                .next()
                .ok_or_else(|| format!("'{name}' row {row} is short"))?;
            let value = field
                .parse::<Value>()
                .map_err(|_| format!("'{name}' row {row}: bad value '{field}'"))?;
            col.push(value);
        }
        if let Some(extra) = fields.next() {
            return Err(format!("'{name}' row {row}: extra field '{extra}'"));
        }
    }
    Ok(cols)
}

fn read_i64_vector(lines: &mut CheckpointLines<'_>, name: &str) -> Result<Vec<i64>, String> {
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' vector"))?;
    let count = parse_count_header(header, name)?;
    let mut values = allocate_checkpoint_rows(lines, count, name)?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'{name}' vector truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        let field = fields
            .next()
            .ok_or_else(|| format!("'{name}' row {row} is empty"))?;
        let value = field
            .parse::<i64>()
            .map_err(|_| format!("'{name}' row {row}: bad value '{field}'"))?;
        if let Some(extra) = fields.next() {
            return Err(format!("'{name}' row {row}: extra field '{extra}'"));
        }
        values.push(value);
    }
    Ok(values)
}

fn read_nonempty_line_vector(
    lines: &mut CheckpointLines<'_>,
    name: &str,
) -> Result<Vec<String>, String> {
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' section"))?;
    let count = parse_count_header(header, name)?;
    let mut values = allocate_checkpoint_rows(lines, count, name)?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'{name}' truncated at row {row}"))?;
        let value = line.trim();
        if value.is_empty() {
            return Err(format!("'{name}' row {row} is empty"));
        }
        values.push(value.to_string());
    }
    Ok(values)
}

fn read_xspice_instance_states(
    lines: &mut CheckpointLines<'_>,
    version: u32,
) -> Result<Vec<XspiceInstanceCheckpoint>, String> {
    let header = lines
        .next()
        .ok_or_else(|| "missing 'xspice_states' section".to_string())?;
    let count = parse_count_header(header, "xspice_states")?;
    let mut states = allocate_checkpoint_rows(lines, count, "xspice_states")?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'xspice_states' truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        if fields.next() != Some("xspice_state") {
            return Err(format!("malformed 'xspice_state' header: '{line}'"));
        }
        let name = fields
            .next()
            .ok_or_else(|| format!("'xspice_state' row {row} is missing instance name"))?;
        let model = fields
            .next()
            .ok_or_else(|| format!("'xspice_state' row {row} is missing model name"))?;
        if let Some(extra) = fields.next() {
            return Err(format!("'xspice_state' row {row}: extra field '{extra}'"));
        }
        let (time, time_prev) = if version >= 5 {
            let times = read_value_vector(lines, "context_time")?;
            if times.len() != 2 {
                return Err(format!(
                    "'context_time' for XSPICE state row {row} must contain 2 values, got {}",
                    times.len()
                ));
            }
            (times[0], times[1])
        } else {
            (0.0, 0.0)
        };
        states.push(XspiceInstanceCheckpoint {
            name: name.to_string(),
            model: model.to_string(),
            context: CmContextCheckpoint {
                time,
                time_prev,
                state: read_value_vector(lines, "state")?,
                state_prev: read_value_vector(lines, "state_prev")?,
                int_state: read_i64_vector(lines, "int_state")?,
            },
        });
    }
    Ok(states)
}

impl TransientCheckpoint {
    fn validate_numeric_state(&self) -> Result<(), String> {
        if !self.time.is_finite() || self.time < 0.0 {
            return Err("checkpoint time must be finite and non-negative".to_string());
        }
        if self.solution.iter().any(|value| !value.is_finite()) {
            return Err("checkpoint solution values must be finite".to_string());
        }

        let capacitor_len = self.cap_v_prev.len();
        if [
            self.cap_v_prev_prev.len(),
            self.cap_v_prev_prev_prev.len(),
            self.cap_i_prev.len(),
            self.cap_i_eq.len(),
        ]
        .into_iter()
        .any(|len| len != capacitor_len)
        {
            return Err(
                "checkpoint capacitor history vectors have inconsistent lengths".to_string(),
            );
        }
        let inductor_len = self.ind_i_prev.len();
        if [self.ind_i_prev_prev.len(), self.ind_v_prev.len()]
            .into_iter()
            .any(|len| len != inductor_len)
        {
            return Err(
                "checkpoint inductor history vectors have inconsistent lengths".to_string(),
            );
        }
        if self
            .cap_v_prev
            .iter()
            .chain(&self.cap_v_prev_prev)
            .chain(&self.cap_v_prev_prev_prev)
            .chain(&self.cap_i_prev)
            .chain(&self.cap_i_eq)
            .chain(&self.ind_i_prev)
            .chain(&self.ind_i_prev_prev)
            .chain(&self.ind_v_prev)
            .any(|value| !value.is_finite())
        {
            return Err("checkpoint reactive history values must be finite".to_string());
        }
        if !self.lte_signal_global_reference.is_finite()
            || self.lte_signal_global_reference < 0.0
            || self
                .lte_signal_local_reference
                .iter()
                .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(
                "checkpoint LTE reference values must be finite and non-negative".to_string(),
            );
        }
        if self.xspice_instance_states.iter().any(|instance| {
            !instance.context.time.is_finite()
                || instance.context.time < 0.0
                || !instance.context.time_prev.is_finite()
                || instance.context.time_prev < 0.0
                || instance
                    .context
                    .state
                    .iter()
                    .chain(&instance.context.state_prev)
                    .any(|value| !value.is_finite())
        }) {
            return Err("checkpoint XSPICE floating-point state must be finite".to_string());
        }
        Ok(())
    }

    /// Capture the integrator state from a circuit at time `time` with the
    /// current accepted `solution`.
    pub(crate) fn capture(
        fingerprint: u64,
        time: Value,
        solution: &[Value],
        circuit: &Circuit,
        lte_estimator: Option<&LteEstimator>,
    ) -> Self {
        if !circuit.tlines.is_empty() || !circuit.coupled_tlines.is_empty() {
            log::warn!(
                "transient checkpoint at t={time:.6e}: transmission-line delay \
                 histories are re-derived on resume (breakpoint-restart \
                 semantics); prefer unsegmented runs for delay-dominated decks"
            );
        }
        let xspice_instances: Vec<String> = circuit
            .xspice_instances
            .iter()
            .map(|instance| format!("{}({})", instance.name, instance.model_name()))
            .collect();
        let xspice_resume_blockers = circuit.xspice_checkpoint_resume_blockers();
        let xspice_instance_states = if xspice_resume_blockers.is_empty() {
            circuit.xspice_checkpoint_instance_states()
        } else {
            Vec::new()
        };
        if !xspice_resume_blockers.is_empty() {
            log::warn!(
                "transient checkpoint at t={time:.6e}: XSPICE code-model \
                 state is not fully serialized; this checkpoint will be refused \
                 for resume: {}",
                xspice_resume_blockers.join("; ")
            );
        }

        let (lte_signal_global_reference, lte_signal_local_reference) = lte_estimator
            .map(LteEstimator::signal_reference_snapshot)
            .map_or((0.0, Vec::new()), |(global, local)| {
                (global, local.to_vec())
            });

        Self {
            time,
            solution: solution.to_vec(),
            netlist_fingerprint: fingerprint,
            cap_v_prev: circuit.capacitors.v_prev.clone(),
            cap_v_prev_prev: circuit.capacitors.v_prev_prev.clone(),
            cap_v_prev_prev_prev: circuit.capacitors.v_prev_prev_prev.clone(),
            cap_i_prev: circuit.capacitors.i_prev.clone(),
            cap_i_eq: circuit.capacitors.i_eq.clone(),
            ind_i_prev: circuit.inductors.i_prev.clone(),
            ind_i_prev_prev: circuit.inductors.i_prev_prev.clone(),
            ind_v_prev: circuit.inductors.v_prev.clone(),
            lte_signal_global_reference,
            lte_signal_local_reference,
            lte_reference_history_available: lte_estimator.is_some(),
            lte_reference_mode: lte_estimator.map(LteEstimator::reference_mode),
            xspice_instances,
            xspice_resume_blockers,
            xspice_instance_states,
        }
    }

    /// Inject the captured reactive-state histories into a freshly built
    /// circuit. Lengths must match the capture exactly.
    pub(crate) fn inject(&self, circuit: &mut Circuit) -> Result<(), String> {
        self.validate_numeric_state()?;
        if circuit.capacitors.v_prev.len() != self.cap_v_prev.len()
            || circuit.inductors.i_prev.len() != self.ind_i_prev.len()
        {
            return Err(format!(
                "checkpoint shape mismatch: {} capacitors / {} inductors captured, \
                 circuit has {} / {}",
                self.cap_v_prev.len(),
                self.ind_i_prev.len(),
                circuit.capacitors.v_prev.len(),
                circuit.inductors.i_prev.len()
            ));
        }

        circuit.capacitors.v_prev.copy_from_slice(&self.cap_v_prev);
        circuit
            .capacitors
            .v_prev_prev
            .copy_from_slice(&self.cap_v_prev_prev);
        circuit
            .capacitors
            .v_prev_prev_prev
            .copy_from_slice(&self.cap_v_prev_prev_prev);
        circuit.capacitors.i_prev.copy_from_slice(&self.cap_i_prev);
        circuit.capacitors.i_eq.copy_from_slice(&self.cap_i_eq);
        circuit.inductors.i_prev.copy_from_slice(&self.ind_i_prev);
        circuit
            .inductors
            .i_prev_prev
            .copy_from_slice(&self.ind_i_prev_prev);
        circuit.inductors.v_prev.copy_from_slice(&self.ind_v_prev);
        circuit.restore_xspice_checkpoint_instance_states(&self.xspice_instance_states)?;
        Ok(())
    }

    /// Restore accepted-solution LTE reference history for a resumed run.
    pub(crate) fn restore_lte_references(
        &self,
        estimator: &mut LteEstimator,
    ) -> Result<(), String> {
        if estimator.requires_signal_reference_history() && !self.lte_reference_history_available {
            return Err(
                "legacy transient checkpoint does not contain NEWLTE signal-history state"
                    .to_string(),
            );
        }
        if self.lte_reference_history_available
            && self.lte_reference_mode != Some(estimator.reference_mode())
        {
            return Err(format!(
                "transient checkpoint LTE reference mode {:?} does not match resumed mode {:?}",
                self.lte_reference_mode,
                estimator.reference_mode()
            ));
        }
        estimator.restore_signal_reference_snapshot(
            self.lte_signal_global_reference,
            &self.lte_signal_local_reference,
        )
    }

    /// Validate this checkpoint against a netlist before resuming.
    pub fn validate_for(&self, netlist: &Netlist) -> Result<(), String> {
        self.validate_numeric_state()?;
        let fingerprint = netlist_fingerprint(netlist);
        if fingerprint != self.netlist_fingerprint {
            return Err(format!(
                "checkpoint was captured from a different netlist \
                 (fingerprint {:#018x}, this deck is {:#018x}); refusing to \
                 resume mismatched state",
                self.netlist_fingerprint, fingerprint
            ));
        }
        if !self.xspice_resume_blockers.is_empty() {
            return Err(format!(
                "transient checkpoint resume cannot restore unsupported XSPICE \
                 state: {}. Run XSPICE transient decks unsegmented.",
                self.xspice_resume_blockers.join("; ")
            ));
        }
        if self.xspice_instances.is_empty() && netlist_has_xspice(netlist) {
            return Err(
                "transient checkpoint resume cannot verify XSPICE state for this \
                 legacy checkpoint format; the target netlist contains XSPICE \
                 code-model instances. Run XSPICE transient decks unsegmented."
                    .to_string(),
            );
        }
        Ok(())
    }

    //=========================================================================
    // Text serialization (versioned; exact f64 round-trip via shortest
    // round-trip Display formatting)
    //=========================================================================

    /// Serialize to the versioned text format.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("RSPICE-CHECKPOINT {FORMAT_VERSION}\n"));
        out.push_str(&format!("fingerprint {:#018x}\n", self.netlist_fingerprint));
        out.push_str(&format!("time {}\n", self.time));

        let section = |out: &mut String, name: &str, rows: &[&[Value]]| {
            let len = rows.first().map_or(0, |r| r.len());
            out.push_str(&format!("{name} {len}\n"));
            for i in 0..len {
                let line: Vec<String> = rows.iter().map(|r| r[i].to_string()).collect();
                out.push_str(&line.join(" "));
                out.push('\n');
            }
        };

        section(&mut out, "solution", &[&self.solution]);
        let lte_mode = match self.lte_reference_mode {
            None => "none".to_string(),
            Some(TransientLteReference::PredictorLocal) => "predictor-local".to_string(),
            Some(mode) => mode
                .xyce_selector()
                .expect("Xyce LTE mode has a selector")
                .to_string(),
        };
        out.push_str(&format!("lte_reference_mode {lte_mode}\n"));
        write_value_vector(
            &mut out,
            "lte_signal_global",
            &[self.lte_signal_global_reference],
        );
        write_value_vector(
            &mut out,
            "lte_signal_local",
            &self.lte_signal_local_reference,
        );
        section(
            &mut out,
            "capacitors",
            &[
                &self.cap_v_prev,
                &self.cap_v_prev_prev,
                &self.cap_v_prev_prev_prev,
                &self.cap_i_prev,
                &self.cap_i_eq,
            ],
        );
        section(
            &mut out,
            "inductors",
            &[&self.ind_i_prev, &self.ind_i_prev_prev, &self.ind_v_prev],
        );
        out.push_str(&format!("xspice {}\n", self.xspice_instances.len()));
        for instance in &self.xspice_instances {
            out.push_str(instance);
            out.push('\n');
        }
        out.push_str(&format!(
            "xspice_blockers {}\n",
            self.xspice_resume_blockers.len()
        ));
        for blocker in &self.xspice_resume_blockers {
            out.push_str(blocker);
            out.push('\n');
        }
        out.push_str(&format!(
            "xspice_states {}\n",
            self.xspice_instance_states.len()
        ));
        for instance in &self.xspice_instance_states {
            out.push_str(&format!(
                "xspice_state {} {}\n",
                instance.name, instance.model
            ));
            write_value_vector(
                &mut out,
                "context_time",
                &[instance.context.time, instance.context.time_prev],
            );
            write_value_vector(&mut out, "state", &instance.context.state);
            write_value_vector(&mut out, "state_prev", &instance.context.state_prev);
            write_i64_vector(&mut out, "int_state", &instance.context.int_state);
        }
        out
    }

    /// Parse the versioned text format.
    pub fn from_text(text: &str) -> Result<Self, String> {
        let mut lines = CheckpointLines::new(text);

        let header = lines.next().ok_or("empty checkpoint file")?;
        let version: u32 = header
            .strip_prefix("RSPICE-CHECKPOINT ")
            .and_then(|v| v.trim().parse().ok())
            .ok_or_else(|| format!("not a checkpoint file (header: '{header}')"))?;
        if !(1..=FORMAT_VERSION).contains(&version) {
            return Err(format!(
                "unsupported checkpoint version {version} (this build reads {FORMAT_VERSION})"
            ));
        }

        let fingerprint_line = lines.next().ok_or("missing fingerprint line")?;
        let netlist_fingerprint = fingerprint_line
            .strip_prefix("fingerprint ")
            .map(str::trim)
            .and_then(|v| v.strip_prefix("0x"))
            .and_then(|v| u64::from_str_radix(v, 16).ok())
            .ok_or_else(|| format!("malformed fingerprint line: '{fingerprint_line}'"))?;

        let time_line = lines.next().ok_or("missing time line")?;
        let time: Value = time_line
            .strip_prefix("time ")
            .and_then(|v| v.trim().parse().ok())
            .ok_or_else(|| format!("malformed time line: '{time_line}'"))?;

        let mut solution_cols = read_value_section(&mut lines, "solution", 1)?;
        if solution_cols[0].iter().any(|value| !value.is_finite()) {
            return Err("checkpoint solution values must be finite".to_string());
        }
        let (lte_reference_mode, lte_signal_global_reference, lte_signal_local_reference) =
            if version >= 6 {
                let mode_line = lines
                    .next()
                    .ok_or_else(|| "missing 'lte_reference_mode' line".to_string())?;
                let mode = match mode_line.strip_prefix("lte_reference_mode ").map(str::trim) {
                    Some("none") => None,
                    Some("predictor-local") => Some(TransientLteReference::PredictorLocal),
                    Some(selector) => {
                        let selector = selector.parse::<u8>().map_err(|_| {
                            format!("malformed LTE reference mode line: '{mode_line}'")
                        })?;
                        Some(
                            TransientLteReference::from_xyce_selector(selector).ok_or_else(
                                || format!("unsupported LTE reference mode in line: '{mode_line}'"),
                            )?,
                        )
                    }
                    None => {
                        return Err(format!("malformed LTE reference mode line: '{mode_line}'"));
                    }
                };
                let global = read_value_vector(&mut lines, "lte_signal_global")?;
                if global.len() != 1 || !global[0].is_finite() || global[0] < 0.0 {
                    return Err(
                        "'lte_signal_global' must contain one finite non-negative value"
                            .to_string(),
                    );
                }
                let local = read_value_vector(&mut lines, "lte_signal_local")?;
                if local.iter().any(|value| !value.is_finite() || *value < 0.0) {
                    return Err(
                        "'lte_signal_local' values must be finite and non-negative".to_string()
                    );
                }
                (mode, global[0], local)
            } else {
                (None, 0.0, Vec::new())
            };
        let cap_cols = read_value_section(&mut lines, "capacitors", 5)?;
        let ind_cols = read_value_section(&mut lines, "inductors", 3)?;
        let xspice_instances = if version >= 2 {
            read_nonempty_line_vector(&mut lines, "xspice")?
        } else {
            Vec::new()
        };
        let mut xspice_resume_blockers = if version >= 3 {
            read_nonempty_line_vector(&mut lines, "xspice_blockers")?
        } else {
            Vec::new()
        };
        if version == 2 && !xspice_instances.is_empty() {
            xspice_resume_blockers.extend(xspice_instances.iter().map(|instance| {
                format!("{instance}: legacy checkpoint did not record model checkpoint support")
            }));
        }
        let xspice_instance_states = if version >= 4 {
            read_xspice_instance_states(&mut lines, version)?
        } else {
            Vec::new()
        };
        if let Some(extra) = lines.find(|line| !line.trim().is_empty()) {
            return Err(format!("checkpoint has trailing content: '{extra}'"));
        }

        let mut cap_iter = cap_cols.into_iter();
        let mut ind_iter = ind_cols.into_iter();
        let checkpoint = Self {
            time,
            solution: solution_cols.swap_remove(0),
            netlist_fingerprint,
            cap_v_prev: cap_iter.next().unwrap(),
            cap_v_prev_prev: cap_iter.next().unwrap(),
            cap_v_prev_prev_prev: cap_iter.next().unwrap(),
            cap_i_prev: cap_iter.next().unwrap(),
            cap_i_eq: cap_iter.next().unwrap(),
            ind_i_prev: ind_iter.next().unwrap(),
            ind_i_prev_prev: ind_iter.next().unwrap(),
            ind_v_prev: ind_iter.next().unwrap(),
            lte_signal_global_reference,
            lte_signal_local_reference,
            lte_reference_history_available: lte_reference_mode.is_some(),
            lte_reference_mode,
            xspice_instances,
            xspice_resume_blockers,
            xspice_instance_states,
        };
        checkpoint.validate_numeric_state()?;
        Ok(checkpoint)
    }

    /// Write the checkpoint to a file.
    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
        self.validate_numeric_state()?;
        std::fs::write(path, self.to_text())
            .map_err(|e| format!("cannot write checkpoint '{}': {e}", path.display()))
    }

    /// Read a checkpoint from a file.
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read checkpoint '{}': {e}", path.display()))?;
        Self::from_text(&text)
    }
}

fn netlist_has_xspice(netlist: &Netlist) -> bool {
    netlist
        .elements
        .iter()
        .any(|element| matches!(element.kind, ElementKind::Xspice { .. }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> TransientCheckpoint {
        TransientCheckpoint {
            time: 1.2345678901234567e-6,
            solution: vec![0.5, -3.25, 1.0e-15, f64::MIN_POSITIVE, -0.0],
            netlist_fingerprint: 0xDEAD_BEEF_0123_4567,
            cap_v_prev: vec![0.1, -0.2],
            cap_v_prev_prev: vec![0.09, -0.19],
            cap_v_prev_prev_prev: vec![0.08, -0.18],
            cap_i_prev: vec![1e-3, -2e-3],
            cap_i_eq: vec![5e-4, -6e-4],
            ind_i_prev: vec![7e-3],
            ind_i_prev_prev: vec![6.5e-3],
            ind_v_prev: vec![0.02],
            lte_signal_global_reference: 3.25,
            lte_signal_local_reference: Vec::new(),
            lte_reference_history_available: true,
            lte_reference_mode: Some(TransientLteReference::SignalGlobal),
            xspice_instances: Vec::new(),
            xspice_resume_blockers: Vec::new(),
            xspice_instance_states: Vec::new(),
        }
    }

    fn legacy_text(checkpoint: &TransientCheckpoint, version: u32) -> String {
        let text = checkpoint.to_text().replace(
            &format!("RSPICE-CHECKPOINT {FORMAT_VERSION}"),
            &format!("RSPICE-CHECKPOINT {version}"),
        );
        let mut output = String::new();
        let mut lines = text.lines();
        while let Some(line) = lines.next() {
            if line.starts_with("lte_reference_mode ") {
                continue;
            }
            if line.starts_with("lte_signal_global ") || line.starts_with("lte_signal_local ") {
                let count = line
                    .split_whitespace()
                    .nth(1)
                    .expect("LTE checkpoint vector count")
                    .parse::<usize>()
                    .expect("numeric LTE checkpoint vector count");
                for _ in 0..count {
                    lines.next().expect("complete LTE checkpoint vector");
                }
                continue;
            }
            output.push_str(line);
            output.push('\n');
        }
        output
    }

    #[test]
    fn text_round_trip_is_bit_exact() {
        let original = sample();
        let restored = TransientCheckpoint::from_text(&original.to_text()).unwrap();
        assert_eq!(original, restored);
        // Bit-level check on the touchy values (subnormals, negative zero).
        for (a, b) in original.solution.iter().zip(&restored.solution) {
            assert_eq!(a.to_bits(), b.to_bits());
        }
    }

    #[test]
    fn signal_history_lte_references_round_trip_and_legacy_resume_fails_closed() {
        let checkpoint = sample();
        let restored = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect("current checkpoint format parses");
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            1.0e-3,
            1.0e-6,
            crate::netlist::TransientLteReference::SignalGlobal,
        );
        estimator.seed_reference_prefix(&restored.solution, restored.solution.len());
        restored
            .restore_lte_references(&mut estimator)
            .expect("current checkpoint restores signal history");
        let (global, local) = estimator.signal_reference_snapshot();
        assert_eq!(global.to_bits(), 3.25f64.to_bits());
        assert!(local.is_empty());

        let legacy = TransientCheckpoint::from_text(&legacy_text(&checkpoint, 5))
            .expect("version-five checkpoint remains readable");
        let err = legacy
            .restore_lte_references(&mut estimator)
            .expect_err("legacy checkpoint cannot resume signal-history NEWLTE exactly");
        assert!(
            err.contains("NEWLTE signal-history"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn checkpoint_lte_reference_mode_mismatch_fails_closed() {
        let checkpoint = sample();
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            1.0e-3,
            1.0e-6,
            TransientLteReference::SignalLocal,
        );
        estimator.seed_reference_prefix(&checkpoint.solution, checkpoint.solution.len());

        let err = checkpoint
            .restore_lte_references(&mut estimator)
            .expect_err("checkpoint mode provenance must match the resumed solver");
        assert!(err.contains("does not match"), "unexpected error: {err}");
    }

    #[test]
    fn legacy_checkpoint_upgrade_does_not_invent_signal_history() {
        let legacy = TransientCheckpoint::from_text(&legacy_text(&sample(), 5))
            .expect("version-five checkpoint remains readable");
        assert!(!legacy.lte_reference_history_available);

        let upgraded = TransientCheckpoint::from_text(&legacy.to_text())
            .expect("legacy checkpoint can be re-serialized in the current format");
        assert!(!upgraded.lte_reference_history_available);
        assert_eq!(upgraded.lte_reference_mode, None);

        let mut estimator = LteEstimator::with_tolerances_and_reference(
            1.0e-3,
            1.0e-6,
            TransientLteReference::SignalGlobal,
        );
        estimator.seed_reference_prefix(&upgraded.solution, upgraded.solution.len());
        let err = upgraded
            .restore_lte_references(&mut estimator)
            .expect_err("upgrading a legacy file cannot synthesize signal history");
        assert!(
            err.contains("NEWLTE signal-history"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn checkpoint_rejects_non_finite_solution_and_lte_reference_values() {
        for non_finite in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
            let mut checkpoint = sample();
            checkpoint.solution[0] = non_finite;
            let err = TransientCheckpoint::from_text(&checkpoint.to_text())
                .expect_err("non-finite accepted solutions must fail closed");
            assert!(err.contains("solution values"), "unexpected error: {err}");

            let mut checkpoint = sample();
            checkpoint.lte_signal_global_reference = non_finite;
            let err = TransientCheckpoint::from_text(&checkpoint.to_text())
                .expect_err("non-finite signal history must fail closed");
            assert!(err.contains("lte_signal_global"), "unexpected error: {err}");
        }

        let mut checkpoint = sample();
        checkpoint.time = -1.0;
        let err = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect_err("negative checkpoint time must fail closed");
        assert!(err.contains("checkpoint time"), "unexpected error: {err}");

        let mut checkpoint = sample();
        checkpoint.cap_v_prev[0] = Value::NAN;
        let err = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect_err("non-finite reactive history must fail closed");
        assert!(err.contains("reactive history"), "unexpected error: {err}");

        let mut checkpoint = sample();
        checkpoint.xspice_instance_states = vec![XspiceInstanceCheckpoint {
            name: "a1".to_string(),
            model: "stateful".to_string(),
            context: CmContextCheckpoint {
                time: 1.0,
                time_prev: 0.5,
                state: vec![Value::INFINITY],
                state_prev: vec![0.0],
                int_state: Vec::new(),
            },
        }];
        let err = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect_err("non-finite XSPICE state must fail closed");
        assert!(err.contains("XSPICE"), "unexpected error: {err}");
    }

    #[test]
    fn version_one_checkpoint_files_still_load_without_xspice_state() {
        let version_one =
            legacy_text(&sample(), 1).replace("xspice 0\nxspice_blockers 0\nxspice_states 0\n", "");
        let restored = TransientCheckpoint::from_text(&version_one)
            .expect("v1 checkpoint without XSPICE section still loads");
        assert!(restored.xspice_instances.is_empty());
        assert!(restored.xspice_resume_blockers.is_empty());
        assert!(restored.xspice_instance_states.is_empty());
        assert!(!restored.lte_reference_history_available);
    }

    #[test]
    fn xspice_blockers_round_trip_and_legacy_v2_refuses_resume() {
        let mut original = sample();
        original.xspice_instances = vec!["a1(gain)".to_string()];
        original.xspice_resume_blockers = vec!["a1(gain): model owns pending state".to_string()];
        let restored = TransientCheckpoint::from_text(&original.to_text()).unwrap();
        assert_eq!(original, restored);

        let version_two = legacy_text(&original, 2).replace(
            "xspice_blockers 1\na1(gain): model owns pending state\nxspice_states 0\n",
            "",
        );
        let restored = TransientCheckpoint::from_text(&version_two)
            .expect("v2 checkpoint with XSPICE instance list still loads");
        assert_eq!(restored.xspice_instances, vec!["a1(gain)"]);
        assert!(
            restored.xspice_resume_blockers[0].contains("legacy checkpoint"),
            "legacy v2 checkpoints must remain blocked, got {:?}",
            restored.xspice_resume_blockers
        );
    }

    #[test]
    fn xspice_instance_states_round_trip_and_v3_loads_without_state_section() {
        let mut original = sample();
        original.xspice_instances = vec!["a1(int)".to_string()];
        original.xspice_instance_states = vec![XspiceInstanceCheckpoint {
            name: "a1".to_string(),
            model: "int".to_string(),
            context: CmContextCheckpoint {
                time: 1.25,
                time_prev: 1.0,
                state: vec![1.0, -0.0],
                state_prev: vec![0.5, f64::MIN_POSITIVE],
                int_state: vec![42, -7],
            },
        }];
        let restored = TransientCheckpoint::from_text(&original.to_text()).unwrap();
        assert_eq!(original, restored);

        let version_three = legacy_text(&sample(), 3).replace("xspice_states 0\n", "");
        let restored = TransientCheckpoint::from_text(&version_three)
            .expect("v3 checkpoint without serialized XSPICE state still loads");
        assert!(restored.xspice_instance_states.is_empty());

        let version_four = legacy_text(&original, 4).replace("context_time 2\n1.25\n1\n", "");
        let restored = TransientCheckpoint::from_text(&version_four)
            .expect("v4 XSPICE state checkpoint without context times still loads");
        assert_eq!(restored.xspice_instance_states[0].context.time, 0.0);
        assert_eq!(restored.xspice_instance_states[0].context.time_prev, 0.0);
    }

    #[test]
    fn malformed_input_fails_with_a_clear_message() {
        assert!(TransientCheckpoint::from_text("").is_err());
        assert!(
            TransientCheckpoint::from_text("RSPICE-CHECKPOINT 999\nfingerprint 0x0\ntime 0\n")
                .unwrap_err()
                .contains("version")
        );
        // Cut mid-file so a whole section is missing rows — trimming only
        // trailing digits would still be a syntactically valid file.
        let text = sample().to_text();
        let truncated = &text[..text.len() / 2];
        assert!(TransientCheckpoint::from_text(truncated).is_err());
    }

    #[test]
    fn declared_section_counts_are_bounded_before_allocation() {
        let count = usize::MAX;

        let text = format!("state {count}\n");
        let mut lines = CheckpointLines::new(&text);
        let err = read_value_vector(&mut lines, "state")
            .expect_err("oversized floating-point vectors must fail closed");
        assert!(err.contains("rows remain"), "unexpected error: {err}");

        let text = format!("solution {count}\n");
        let mut lines = CheckpointLines::new(&text);
        let err = read_value_section(&mut lines, "solution", 1)
            .expect_err("oversized table sections must fail closed");
        assert!(err.contains("rows remain"), "unexpected error: {err}");

        let text = format!("int_state {count}\n");
        let mut lines = CheckpointLines::new(&text);
        let err = read_i64_vector(&mut lines, "int_state")
            .expect_err("oversized integer vectors must fail closed");
        assert!(err.contains("rows remain"), "unexpected error: {err}");

        let text = format!("xspice {count}\n");
        let mut lines = CheckpointLines::new(&text);
        let err = read_nonempty_line_vector(&mut lines, "xspice")
            .expect_err("oversized string sections must fail closed");
        assert!(err.contains("rows remain"), "unexpected error: {err}");

        let text = format!("xspice_states {count}\n");
        let mut lines = CheckpointLines::new(&text);
        let err = read_xspice_instance_states(&mut lines, FORMAT_VERSION)
            .expect_err("oversized nested XSPICE sections must fail closed");
        assert!(err.contains("rows remain"), "unexpected error: {err}");
    }

    #[test]
    fn malformed_input_rejects_extra_row_fields_and_trailing_data() {
        let text = sample().to_text();
        let extra_row_field = text.replacen("0.5\n", "0.5 99\n", 1);
        let err = TransientCheckpoint::from_text(&extra_row_field)
            .expect_err("extra checkpoint row fields must be rejected");
        assert!(
            err.contains("extra field"),
            "expected extra-field diagnostic, got {err}"
        );

        let trailing = format!("{text}unexpected trailer\n");
        let err = TransientCheckpoint::from_text(&trailing)
            .expect_err("trailing checkpoint content must be rejected");
        assert!(
            err.contains("trailing content"),
            "expected trailing-content diagnostic, got {err}"
        );
    }
}
