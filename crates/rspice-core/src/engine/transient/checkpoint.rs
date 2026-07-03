//! Transient checkpoint/restore.
//!
//! A checkpoint captures the integrator state at an accepted time point:
//! the full MNA solution plus the capacitor and inductor companion-model
//! histories. Restoring injects that state into a freshly built circuit and
//! continues integration from the checkpoint time with absolute-time source
//! evaluation — the same numerical regime as a breakpoint restart, which
//! the integrator already performs at every source discontinuity.
//!
//! Scope, stated precisely: linear-reactive state (C/L histories) resumes
//! exactly; nonlinear-device iteration memories and transmission-line delay
//! histories re-derive from the restored solution on the first step, just
//! as they do after any breakpoint. Decks dominated by transmission-line
//! delays should prefer unsegmented runs (a warning is logged at capture).
//!
//! The on-disk format is a versioned, line-oriented text format using
//! Rust's shortest-round-trip float formatting, so save/load reproduces
//! every `f64` bit-exactly with no serialization dependencies (core stays
//! lean for the wasm build).

use crate::Value;
use crate::circuit::Circuit;
use crate::netlist::{ElementKind, Netlist};
use crate::xspice::{CmContextCheckpoint, XspiceInstanceCheckpoint};

/// Format version written to and required from checkpoint files.
const FORMAT_VERSION: u32 = 5;

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

fn read_value_vector<'a, I>(lines: &mut I, name: &str) -> Result<Vec<Value>, String>
where
    I: Iterator<Item = &'a str>,
{
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' vector"))?;
    let count = parse_count_header(header, name)?;
    let mut values = Vec::with_capacity(count);
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

fn read_i64_vector<'a, I>(lines: &mut I, name: &str) -> Result<Vec<i64>, String>
where
    I: Iterator<Item = &'a str>,
{
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' vector"))?;
    let count = parse_count_header(header, name)?;
    let mut values = Vec::with_capacity(count);
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

fn read_xspice_instance_states<'a, I>(
    lines: &mut I,
    version: u32,
) -> Result<Vec<XspiceInstanceCheckpoint>, String>
where
    I: Iterator<Item = &'a str>,
{
    let header = lines
        .next()
        .ok_or_else(|| "missing 'xspice_states' section".to_string())?;
    let count = parse_count_header(header, "xspice_states")?;
    let mut states = Vec::with_capacity(count);
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
    /// Capture the integrator state from a circuit at time `time` with the
    /// current accepted `solution`.
    pub(crate) fn capture(
        fingerprint: u64,
        time: Value,
        solution: &[Value],
        circuit: &Circuit,
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
            xspice_instances,
            xspice_resume_blockers,
            xspice_instance_states,
        }
    }

    /// Inject the captured reactive-state histories into a freshly built
    /// circuit. Lengths must match the capture exactly.
    pub(crate) fn inject(&self, circuit: &mut Circuit) -> Result<(), String> {
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

    /// Validate this checkpoint against a netlist before resuming.
    pub fn validate_for(&self, netlist: &Netlist) -> Result<(), String> {
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
        let mut lines = text.lines();

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

        let mut read_section = |name: &str, columns: usize| -> Result<Vec<Vec<Value>>, String> {
            let header = lines
                .next()
                .ok_or_else(|| format!("missing '{name}' section"))?;
            let count: usize = header
                .strip_prefix(name)
                .map(str::trim)
                .and_then(|v| v.parse().ok())
                .ok_or_else(|| format!("malformed '{name}' header: '{header}'"))?;
            let mut cols = vec![Vec::with_capacity(count); columns];
            for row in 0..count {
                let line = lines
                    .next()
                    .ok_or_else(|| format!("'{name}' truncated at row {row}"))?;
                let mut fields = line.split_whitespace();
                for col in cols.iter_mut() {
                    let field = fields
                        .next()
                        .ok_or_else(|| format!("'{name}' row {row} is short"))?;
                    let value: Value = field
                        .parse()
                        .map_err(|_| format!("'{name}' row {row}: bad value '{field}'"))?;
                    col.push(value);
                }
                if let Some(extra) = fields.next() {
                    return Err(format!("'{name}' row {row}: extra field '{extra}'"));
                }
            }
            Ok(cols)
        };

        let mut solution_cols = read_section("solution", 1)?;
        let cap_cols = read_section("capacitors", 5)?;
        let ind_cols = read_section("inductors", 3)?;
        let xspice_instances = if version >= 2 {
            let header = lines
                .next()
                .ok_or_else(|| "missing 'xspice' section".to_string())?;
            let count: usize = header
                .strip_prefix("xspice")
                .map(str::trim)
                .and_then(|value| value.parse().ok())
                .ok_or_else(|| format!("malformed 'xspice' header: '{header}'"))?;
            let mut instances = Vec::with_capacity(count);
            for row in 0..count {
                let line = lines
                    .next()
                    .ok_or_else(|| format!("'xspice' truncated at row {row}"))?;
                if line.trim().is_empty() {
                    return Err(format!("'xspice' row {row} is empty"));
                }
                instances.push(line.trim().to_string());
            }
            instances
        } else {
            Vec::new()
        };
        let mut xspice_resume_blockers = if version >= 3 {
            let header = lines
                .next()
                .ok_or_else(|| "missing 'xspice_blockers' section".to_string())?;
            let count: usize = header
                .strip_prefix("xspice_blockers")
                .map(str::trim)
                .and_then(|value| value.parse().ok())
                .ok_or_else(|| format!("malformed 'xspice_blockers' header: '{header}'"))?;
            let mut blockers = Vec::with_capacity(count);
            for row in 0..count {
                let line = lines
                    .next()
                    .ok_or_else(|| format!("'xspice_blockers' truncated at row {row}"))?;
                if line.trim().is_empty() {
                    return Err(format!("'xspice_blockers' row {row} is empty"));
                }
                blockers.push(line.trim().to_string());
            }
            blockers
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
        Ok(Self {
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
            xspice_instances,
            xspice_resume_blockers,
            xspice_instance_states,
        })
    }

    /// Write the checkpoint to a file.
    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
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
            xspice_instances: Vec::new(),
            xspice_resume_blockers: Vec::new(),
            xspice_instance_states: Vec::new(),
        }
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
    fn version_one_checkpoint_files_still_load_without_xspice_state() {
        let version_one = sample()
            .to_text()
            .replace("RSPICE-CHECKPOINT 5", "RSPICE-CHECKPOINT 1")
            .replace("xspice 0\nxspice_blockers 0\nxspice_states 0\n", "");
        let restored = TransientCheckpoint::from_text(&version_one)
            .expect("v1 checkpoint without XSPICE section still loads");
        assert!(restored.xspice_instances.is_empty());
        assert!(restored.xspice_resume_blockers.is_empty());
        assert!(restored.xspice_instance_states.is_empty());
    }

    #[test]
    fn xspice_blockers_round_trip_and_legacy_v2_refuses_resume() {
        let mut original = sample();
        original.xspice_instances = vec!["a1(gain)".to_string()];
        original.xspice_resume_blockers = vec!["a1(gain): model owns pending state".to_string()];
        let restored = TransientCheckpoint::from_text(&original.to_text()).unwrap();
        assert_eq!(original, restored);

        let version_two = original
            .to_text()
            .replace("RSPICE-CHECKPOINT 5", "RSPICE-CHECKPOINT 2")
            .replace(
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

        let version_three = sample()
            .to_text()
            .replace("RSPICE-CHECKPOINT 5", "RSPICE-CHECKPOINT 3")
            .replace("xspice_states 0\n", "");
        let restored = TransientCheckpoint::from_text(&version_three)
            .expect("v3 checkpoint without serialized XSPICE state still loads");
        assert!(restored.xspice_instance_states.is_empty());

        let version_four = original
            .to_text()
            .replace("RSPICE-CHECKPOINT 5", "RSPICE-CHECKPOINT 4")
            .replace("context_time 2\n1.25\n1\n", "");
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
