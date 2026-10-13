//! The meters a build put on Verilog-A instance terminals, read back.
//!
//! The builder inserts a zero-volt source in series with each terminal whose
//! current a deck names (see `engine::builder::veriloga_terminal_meters`).
//! The source's branch unknown is that current. These are the reads every
//! analysis publishes it through, under the instance's own names.

use super::*;

/// One metered terminal of one Verilog-A instance.
///
/// Only a build that can elaborate a Verilog-A instance makes one.
#[cfg_attr(
    not(any(feature = "veriloga", feature = "veriloga-builtins-base")),
    allow(dead_code)
)]
#[derive(Debug, Clone)]
pub(crate) struct VerilogATerminalMeter {
    /// Flattened instance name.
    pub(crate) instance: String,
    /// Current label, `i` and the port name in lower case: `@x1[ia]`.
    pub(crate) label: String,
    /// The card's other labels for this terminal's current, lower case: a
    /// diode card routed to a generated model reads its anode as `@d1[id]`.
    pub(crate) aliases: Vec<String>,
    /// Whether this is the first terminal of an instance with at most two and
    /// the deck asks for `I(instance)`, which therefore names it.
    pub(crate) instance_current: bool,
    /// A mixed module's discrete port: its converter is the instance's, so it
    /// sits on the private side of the meter.
    pub(crate) discrete: bool,
    /// The meter's source name; its branch carries the current.
    pub(crate) source: String,
    /// The node the instance's terminal connects to instead of the deck's.
    pub(crate) private_node: String,
    /// The deck's net the terminal was connected to.
    pub(crate) external_node: String,
    /// One-based MNA branch ordinal of the meter, set once the circuit is built.
    pub(crate) branch: usize,
}

impl VerilogATerminalMeter {
    /// Every label the meter's current is published under: its own, then the
    /// card's aliases.
    pub(crate) fn labels(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.label.as_str()).chain(self.aliases.iter().map(String::as_str))
    }

    /// `x1:ia`, the spelling an `@x1[ia]` probe resolves through, for each of
    /// [`Self::labels`].
    pub(crate) fn parameter_names(&self) -> impl Iterator<Item = String> + '_ {
        self.labels()
            .map(|label| format!("{}:{label}", self.instance))
    }
}

impl CircuitData {
    /// Give a metered discrete port's net the deck's name back.
    ///
    /// The meter puts the instance and its converter on a private node, so
    /// the bit the instance drives would be recorded under that node's name
    /// rather than the net the deck connected it to. The two nodes are one
    /// net through a zero-volt source, equal in every voltage, so the names
    /// trade places once the circuit is built: the deck's name marks the node
    /// that carries the bit, and the deck's other connections keep the
    /// private one. A net that is ground keeps its name.
    pub(crate) fn name_metered_discrete_ports_after_their_nets(&mut self) {
        for meter in &self.veriloga_terminal_meters {
            if !meter.discrete {
                continue;
            }
            let (Some(&private), Some(&external)) = (
                self.node_map.get(&meter.private_node),
                self.node_map.get(&meter.external_node),
            ) else {
                continue;
            };
            if external == 0 || private == 0 {
                continue;
            }
            self.node_map.insert(meter.external_node.clone(), private);
            self.node_map.insert(meter.private_node.clone(), external);
        }
    }

    /// The meters on Verilog-A instance terminals, in build order.
    #[inline]
    pub(crate) fn veriloga_terminal_meters(&self) -> &[VerilogATerminalMeter] {
        &self.veriloga_terminal_meters
    }

    /// Current entering a metered terminal at `solution`, positive into the
    /// instance.
    pub(crate) fn veriloga_terminal_meter_current(
        &self,
        solution: &[Value],
        meter: &VerilogATerminalMeter,
    ) -> Result<Value, String> {
        meter
            .branch
            .checked_sub(1)
            .and_then(|branch| solution.get(self.num_nodes() + branch))
            .copied()
            .ok_or_else(|| {
                format!(
                    "the current meter of terminal '{}' of the Verilog-A instance '{}' is \
                     outside the solution",
                    meter.label, meter.instance
                )
            })
    }

    /// Report each metered terminal of a generated instance at the meter's
    /// value: the current in the solved system, rather than the flow the
    /// device stamped at the iterate the last solve started from.
    ///
    /// A runtime-compiled instance has no entry in the report — its port
    /// names are the module's, outside the persisted label vocabulary — so its
    /// meters publish through the observables of the analysis instead.
    pub(crate) fn apply_veriloga_terminal_meters(
        &self,
        report: &mut DeviceOpReport,
        solution: &[Value],
    ) -> Result<(), String> {
        for meter in &self.veriloga_terminal_meters {
            let Some(entry) = report
                .entries
                .iter_mut()
                .find(|entry| entry.name.eq_ignore_ascii_case(&meter.instance))
            else {
                continue;
            };
            let current = self.veriloga_terminal_meter_current(solution, meter)?;
            for (parameter, value) in &mut entry.params {
                if meter
                    .labels()
                    .any(|label| parameter.eq_ignore_ascii_case(label))
                {
                    *value = current;
                }
            }
        }
        Ok(())
    }
}
