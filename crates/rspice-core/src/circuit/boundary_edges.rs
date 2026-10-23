//! Naming the digital edge a terminal transient failure followed.
//!
//! A D/A converter turns an accepted digital edge into an analog step. When
//! the step is ideal, or faster than the circuit it drives can follow, the
//! first Newton solve after it is where a run fails, and the plain failure —
//! "convergence failed at t" — names neither the edge nor the converter that
//! made it. Both are in the accepted event state: the time each digital net
//! last changed, its value, who drives it, and the converter reading it. A
//! mixed module drives the circuit without a converter too, through a D/A
//! bridge or a discrete value its analog block reads, and keeps the last
//! accepted edge of those itself.
//!
//! Read only after recovery is exhausted, so nothing here can change a
//! result.

use crate::Value;
use crate::circuit::{CircuitData, ConverterInsertion};
use crate::xspice::{DigitalState, DigitalValue, EventInputKind, SharedXspiceInstance};

/// The accepted value a net held before an instant, from the run's own
/// record, when it keeps one for that net.
pub(crate) type ValueBefore<'a> = &'a dyn Fn(&str, Value) -> Option<DigitalValue>;

/// One accepted digital edge into a D/A converter.
struct BoundaryEdge<'a> {
    node: usize,
    time: Value,
    converter: &'a SharedXspiceInstance,
}

/// A converter's timing and drive, as the parameters its code model reads.
struct ConverterLaw {
    delay: Value,
    rise: Value,
    fall: Value,
    /// Every parameter worth naming beside the edge, `(name, value)` in the
    /// model's own vocabulary.
    named: Vec<(&'static str, Value)>,
}

impl ConverterLaw {
    fn of(converter: &SharedXspiceInstance) -> Self {
        // The connect-rule vocabulary (`connect_bridges`), or the XSPICE
        // bridges' (`dac_bridge`, `bidi_bridge`), which has no delay.
        let (rise, fall, delay, drive): (_, _, _, &[&'static str]) =
            if converter.model_name().starts_with("__rspice_connect") {
                ("trise", "tfall", Some("tdelay"), &["rlo", "rhi"])
            } else {
                ("t_rise", "t_fall", None, &["out_low", "out_high"])
            };
        let named: Vec<_> = [rise, fall]
            .into_iter()
            .chain(delay)
            .chain(drive.iter().copied())
            .map(|name| (name, converter.param(name)))
            .collect();
        Self {
            delay: delay.map_or(0.0, |name| converter.param(name)),
            rise: converter.param(rise),
            fall: converter.param(fall),
            named,
        }
    }

    /// How long after its input edge the converter's output is still
    /// moving: the step a solve at that time is asked to follow.
    fn settling(&self) -> Value {
        let span = self.delay + self.rise.max(self.fall);
        if span.is_finite() { span.max(0.0) } else { 0.0 }
    }

    /// Whether the transition toward `state` takes no time at all.
    fn is_ideal(&self, state: Option<DigitalState>) -> bool {
        let transition = match state {
            Some(DigitalState::One | DigitalState::OneR | DigitalState::OneZ) => self.rise,
            Some(DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ) => self.fall,
            _ => self.rise.min(self.fall),
        };
        transition == 0.0
    }
}

impl CircuitData {
    /// The accepted digital edge a terminal failure at `time` followed, as a
    /// sentence, when one D/A converter's input changed no more than its own
    /// delay and transition plus two steps before it, or a mixed module's own
    /// drive no more than two steps before it.
    ///
    /// `step` is the failed step's size, and never less than the minimum
    /// step: the retry ladder ends a step or so above the floor, so the
    /// failure lands that far after the edge rather than a floor's width.
    /// `before` reads the value the net held before the edge from the run's
    /// accepted record; a net it has none for is named without it.
    pub(crate) fn boundary_edge_before_failure(
        &self,
        time: Value,
        step: Value,
        before: ValueBefore<'_>,
    ) -> Option<String> {
        let values = &self.scheduler.xspice_event_values;
        let mut found: Option<BoundaryEdge<'_>> = None;
        for converter in &self.xspice_instances {
            if !converter.drives_continuous_outputs() {
                continue;
            }
            let window = ConverterLaw::of(converter).settling() + 2.0 * step;
            converter.for_each_event_input_net(|kind, node| {
                if !matches!(kind, EventInputKind::Digital) {
                    return;
                }
                let Some(&edge) = values.digital_event_times.get(&node) else {
                    return;
                };
                // A time-zero value is the initial state, not an edge.
                if edge <= 0.0 || edge > time || time - edge > window {
                    return;
                }
                if found.as_ref().is_none_or(|latest| edge > latest.time) {
                    found = Some(BoundaryEdge {
                        node,
                        time: edge,
                        converter,
                    });
                }
            });
        }
        // A mixed module's own drive — a D/A bridge, or a discrete value its
        // analog block reads — has no event net; each module keeps its last
        // accepted edge (`xspice/verilog/mixed/drive_edges.rs`). The later
        // edge is the one the failure followed.
        #[cfg(feature = "veriloga")]
        if let Some(internal) = self
            .mixed_signal_hosts
            .iter()
            .filter_map(|host| host.drive_edge_before_failure(time, step))
            .reduce(|latest, next| {
                if next.time > latest.time {
                    next
                } else {
                    latest
                }
            })
            .filter(|internal| found.as_ref().is_none_or(|edge| internal.time > edge.time))
        {
            return Some(internal_edge_sentence(time, &internal));
        }
        let edge = found?;
        let after = values.digital_values.get(&edge.node);
        let (net_node, drivers) = self.edge_origin(edge.node);
        let net = self.node_name_by_id(net_node).unwrap_or("?");
        let before = before(net, edge.time);
        let mut sentence = format!(
            "the failed step to t={time:e}s follows an accepted digital edge on net `{net}`"
        );
        match (before, after) {
            (Some(before), Some(after)) => sentence.push_str(&format!(" {before} -> {after}")),
            (None, Some(after)) => sentence.push_str(&format!(" -> {after}")),
            _ => {}
        }
        sentence.push_str(&format!(" at t={:e}s", edge.time));
        if !drivers.is_empty() {
            sentence.push_str(&format!(" driven by {}", drivers.join(", ")));
        }
        let law = ConverterLaw::of(edge.converter);
        sentence.push_str(&format!(
            ", into converter {}",
            self.converter_label(edge.converter, &law)
        ));
        if law.is_ideal(after.map(|value| value.state)) {
            sentence.push_str(
                "; the converter's step is ideal, which no solver timestep can resolve: give it \
                 a nonzero trise and tfall",
            );
        }
        Some(sentence)
    }
}

impl CircuitData {
    /// The net an edge on event node `node` is named by, and who drove it: a
    /// code model's own output, or an HDL port bit, which elaboration moved
    /// onto a private event node and whose deck net records the same bit.
    fn edge_origin(&self, node: usize) -> (usize, Vec<String>) {
        #[cfg(feature = "veriloga")]
        if let Some(origin) = self
            .mixed_signal_hosts
            .iter()
            .find_map(|host| host.converted_bit_origin(node))
        {
            return origin;
        }
        let drivers = self
            .scheduler
            .xspice_event_values
            .digital_drivers
            .get(&node)
            .map(|bank| {
                bank.keys()
                    .map(|(instance, _, _)| instance.clone())
                    .collect()
            })
            .unwrap_or_default();
        (node, drivers)
    }
}

impl CircuitData {
    /// The insertion record of converter instance `name`, when the build
    /// inserted it on a boundary; an authored or deck-instanced converter has
    /// none.
    pub(crate) fn converter_record(&self, name: &str) -> Option<&ConverterInsertion> {
        self.converter_insertions
            .iter()
            .find(|record| record.instance.eq_ignore_ascii_case(name))
    }

    /// A converter as an edge diagnostic names it, `instance (module: trise=0,
    /// ...)`: the module and the values its law reads, from the insertion
    /// record where the build kept one, and from the code model elsewhere.
    fn converter_label(&self, converter: &SharedXspiceInstance, law: &ConverterLaw) -> String {
        let record = self.converter_record(&converter.name);
        let parameters: Vec<String> = law
            .named
            .iter()
            .map(|(name, value)| {
                let value = record
                    .and_then(|record| {
                        record
                            .parameters
                            .iter()
                            .find(|parameter| parameter.name == *name)
                    })
                    .map_or(*value, |parameter| parameter.value);
                format!("{name}={}", compact(value))
            })
            .collect();
        let module = record.map_or_else(|| converter.model_name(), |record| &record.module);
        format!("{} ({module}: {})", converter.name, parameters.join(", "))
    }
}

/// The sentence for a failure at `time` after a mixed module's own drive edge,
/// worded as an inserted converter's is: the net with its values before and
/// after, the edge time, who drove it, and what carried it into the analog
/// circuit.
#[cfg(feature = "veriloga")]
fn internal_edge_sentence(time: Value, edge: &crate::xspice::verilog::DriveEdgeReport) -> String {
    let mut sentence = format!(
        "the failed step to t={time:e}s follows an accepted digital edge on net `{}` {} -> {} \
         at t={:e}s driven by {}, into {}",
        edge.net,
        edge.before,
        edge.after,
        edge.time,
        edge.drivers.join(", "),
        edge.into,
    );
    if edge.ideal {
        sentence.push_str("; the bridge's step is ideal, which no solver timestep can resolve");
    }
    sentence
}

/// A parameter value as an author would write it: `20`, `3.3`, `1e-12`.
fn compact(value: Value) -> String {
    if value == 0.0 || (1.0e-3..1.0e6).contains(&value.abs()) {
        format!("{value}")
    } else {
        format!("{value:e}")
    }
}
