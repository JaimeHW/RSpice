//! Batch text of XSPICE event nodes: `eprint` and `edisplay` (ngspice
//! xspice/evt/evtprint.c), read from the newest analysis's event data, as
//! ngspice reads its circuit's.

use super::*;
use crate::analysis::transient::{DigitalTrace, RealTrace};
use crate::engine::control::shell::{percent_e, words};

/// One event node's committed values, as ngspice keeps them (`Evt_Node_t`).
enum EventNode<'a> {
    Digital(&'a DigitalTrace),
    Real(&'a RealTrace),
}

impl EventNode<'_> {
    fn name(&self) -> &str {
        match self {
            Self::Digital(trace) => &trace.node_name,
            Self::Real(trace) => &trace.node_name,
        }
    }

    /// The user-defined node type's name (`g_evt_udn_info[]->name`).
    fn kind(&self) -> &'static str {
        match self {
            Self::Digital(_) => "d",
            Self::Real(_) => "real",
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Digital(trace) => trace.points.len(),
            Self::Real(trace) => trace.points.len(),
        }
    }

    fn time(&self, index: usize) -> Value {
        match self {
            Self::Digital(trace) => trace.points[index].time,
            Self::Real(trace) => trace.points[index].time,
        }
    }

    /// The type's `print_val` of one value: xspice/idn/idndig.c:265 for a
    /// digital node, icm/xtraevt/real/udnfunc.c:166 `%15.6e` for a real one.
    fn value(&self, index: usize) -> String {
        match self {
            Self::Digital(trace) => trace.points[index].value.to_ngspice_token(),
            Self::Real(trace) => format!("{:>15}", percent_e(trace.points[index].value, 6)),
        }
    }
}

impl ControlCircuit {
    /// The newest analysis's event nodes, digital first.
    fn event_nodes(&self, line: usize) -> Result<(&str, Vec<EventNode<'_>>), ControlError> {
        let Some(dataset) = self.datasets.last() else {
            return Err(command_error(
                line,
                "no event node data: no simulation has run",
            ));
        };
        let ControlAnalysisResult::Transient(result) = &dataset.result else {
            return Err(command_error(
                line,
                format!(
                    "no event node data: the newest analysis, {}, is not a transient analysis",
                    dataset.name
                ),
            ));
        };
        let nodes = result
            .digital_traces
            .iter()
            .map(EventNode::Digital)
            .chain(result.real_traces.iter().map(EventNode::Real))
            .collect();
        Ok((&dataset.name, nodes))
    }

    /// `edisplay` (evtprint.c:399 `EVTdisplay`): each event node with its
    /// type and number of events.
    pub(in crate::engine) fn event_display(
        &self,
        command: &ControlCommand,
    ) -> Result<String, ControlError> {
        let (plot, nodes) = self.event_nodes(command.line)?;
        if nodes.is_empty() {
            return Ok("No event node available!\n".into());
        }
        let mut text = format!(
            "\nList of event nodes in plot {plot}\n    {:<20}: {:<5}, number of events\n\n",
            "node name", "type"
        );
        for node in &nodes {
            // ngspice prints the names its netlist lowercased.
            text.push_str(&format!(
                "    {:<20}: {:<5}, {:>5}\n",
                node.name().to_ascii_lowercase(),
                node.kind(),
                node.len()
            ));
        }
        Ok(text)
    }

    /// `eprint node...` (evtprint.c:101 `EVTprint`): a row at each step where
    /// a named node has an event, carrying every node's newest value. A node
    /// with two events at one step gives two rows, as ngspice's merge does.
    /// ngspice ends with its event-port messages, which this host has none
    /// of, and with its own solver's load-call counts, which it leaves out.
    pub(in crate::engine) fn event_print(
        &self,
        command: &ControlCommand,
    ) -> Result<String, ControlError> {
        let line = command.line;
        let words = words(&command.arguments);
        if words.is_empty() {
            return Err(command_error(line, "usage: eprint <node1> <node2> ..."));
        }
        let (_, nodes) = self.event_nodes(line)?;
        let mut selected = Vec::with_capacity(words.len());
        for word in &words {
            let node = nodes
                .iter()
                .find(|node| node.name().eq_ignore_ascii_case(&word.text))
                .filter(|node| node.len() > 0)
                .ok_or_else(|| {
                    command_error(line, format!("node {} is not an event node", word.text))
                })?;
            selected.push(node);
        }
        let mut text = String::from("\n**** Results Data ****\n\nTime or Step\n");
        for word in &words {
            text.push_str(&word.text);
            text.push('\n');
        }
        text.push_str("\n\n");
        let mut next = vec![0_usize; selected.len()];
        let mut values = vec![String::new(); selected.len()];
        // ngspice prints `cp_numdgt` decimals, 9 unless `numdgt` is set.
        let row = |step: Value, values: &[String], text: &mut String| {
            text.push_str(&percent_e(step, 9));
            for value in values {
                text.push_str("    ");
                text.push_str(value);
            }
            text.push('\n');
        };
        // The first row takes every node's first event at the last node's
        // step (evtprint.c:197).
        let mut step = 0.0;
        for (index, node) in selected.iter().enumerate() {
            step = node.time(0);
            values[index] = node.value(0);
            next[index] = 1;
        }
        row(step, &values, &mut text);
        let pending = |next: &[usize]| {
            selected
                .iter()
                .zip(next)
                .filter(|(node, next)| **next < node.len())
                .map(|(node, next)| node.time(*next))
                .reduce(Value::min)
        };
        let mut rows = 1_usize;
        while let Some(step) = pending(&next) {
            rows += 1;
            if rows > self.value_limit {
                return Err(ControlError::new(
                    line,
                    ControlErrorKind::ResourceLimit,
                    format!("result values limit {} exceeded", self.value_limit),
                ));
            }
            for (index, node) in selected.iter().enumerate() {
                if next[index] < node.len() && node.time(next[index]) == step {
                    values[index] = node.value(next[index]);
                    next[index] += 1;
                }
            }
            row(step, &values, &mut text);
        }
        text.push_str("\n\n\n**** Messages ****\n\n");
        Ok(text)
    }
}
