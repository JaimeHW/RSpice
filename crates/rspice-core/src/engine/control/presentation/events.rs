//! Batch text of XSPICE event nodes: `eprint` and `edisplay` (ngspice
//! xspice/evt/evtprint.c), read from the newest analysis's event data, as
//! ngspice reads its circuit's.

use super::*;
use crate::analysis::transient::{DigitalTrace, RealTrace};
use crate::engine::control::shell::{percent_e, percent_g_with, words};

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

/// ngspice `get_vcdval` (evtprint.c:461): a digital value's VCD bit, or a
/// real value printed `%.16g` after its `%15.6e` `print_val`.
fn vcd_value(node: &EventNode<'_>, index: usize) -> String {
    match node {
        EventNode::Digital(trace) => match trace.points[index].value.to_ngspice_token().as_str() {
            "0s" | "0r" | "0u" => "0",
            "1s" | "1r" | "1u" => "1",
            "0z" | "1z" | "Uz" => "z",
            _ => "x",
        }
        .to_owned(),
        EventNode::Real(_) => {
            let printed = node.value(index);
            let value = printed.trim().parse::<Value>().unwrap_or(Value::NAN);
            format!("r{}", percent_g_with(value, 16, false))
        }
    }
}

impl ControlCircuit {
    /// `eprvcd node...` (evtprint.c:578 `EVTprintvcd`): the named event
    /// nodes as a VCD document, its time unit one decade finer than the
    /// transient's step. ngspice also dates the document and writes analog
    /// nodes and expressions; this host writes event nodes only, and no
    /// date, which would make the document differ run to run.
    pub(in crate::engine) fn event_vcd(
        &self,
        command: &ControlCommand,
    ) -> Result<String, ControlError> {
        let line = command.line;
        let words = words(&command.arguments);
        if let Some(option) = words.iter().find(|word| word.text.starts_with('-')) {
            return Err(command_error(
                line,
                format!("eprvcd option '{}' is not supported", option.text),
            ));
        }
        if words.is_empty() {
            return Err(command_error(line, "usage: eprvcd <node1> <node2> ..."));
        }
        // Identifiers run from '!' to '~' (evtprint.c:703).
        if words.len() > 94 {
            return Err(command_error(line, "eprvcd names at most 94 nodes"));
        }
        let (_, nodes) = self.event_nodes(line)?;
        let dataset = self
            .datasets
            .last()
            .ok_or_else(|| command_error(line, "no event node data: no simulation has run"))?;
        let AnalysisCommand::Tran { step, .. } = &dataset.command else {
            return Err(command_error(line, "eprvcd reads a transient analysis"));
        };
        let mut selected = Vec::with_capacity(words.len());
        for word in &words {
            let node = nodes
                .iter()
                .find(|node| node.name().eq_ignore_ascii_case(&word.text))
                .filter(|node| node.len() > 0)
                .ok_or_else(|| {
                    command_error(
                        line,
                        format!(
                            "node {} is not an event node; eprvcd writes no analog node",
                            word.text
                        ),
                    )
                })?;
            selected.push(node);
        }
        let (unit, scale) = if *step >= 1e-3 {
            ("us", 1e6)
        } else if *step >= 1e-6 {
            ("ns", 1e9)
        } else if *step >= 1e-9 {
            ("ps", 1e12)
        } else {
            ("fs", 1e15)
        };
        let ident = |index: usize| char::from(b'!' + index as u8);
        let mut text = format!(
            "$version rspice {} $end\n$timescale 1 {unit} $end\n",
            env!("CARGO_PKG_VERSION")
        );
        // Changes are found on the printed values (evtprint.c:907), so a
        // strength change writes the same bit again.
        let mut printed: Vec<String> = selected.iter().map(|node| node.value(0)).collect();
        let values: Vec<String> = selected.iter().map(|node| vcd_value(node, 0)).collect();
        for (index, word) in words.iter().enumerate() {
            let kind = if values[index].starts_with('r') {
                "real"
            } else {
                "wire"
            };
            text.push_str(&format!(
                "$var {kind} 1 {} {} $end\n",
                ident(index),
                word.text
            ));
        }
        text.push_str("$enddefinitions $end\n$dumpvars\n");
        for (index, value) in values.iter().enumerate() {
            let space = if value.starts_with('r') { " " } else { "" };
            text.push_str(&format!("{value}{space}{}\n", ident(index)));
        }
        text.push_str("$end\n");
        let mut next = vec![1_usize; selected.len()];
        let pending = |next: &[usize]| {
            selected
                .iter()
                .zip(next)
                .filter(|(node, next)| **next < node.len())
                .map(|(node, next)| node.time(*next))
                .reduce(Value::min)
        };
        let mut steps = 0_usize;
        while let Some(step) = pending(&next) {
            steps += 1;
            if steps > self.value_limit {
                return Err(ControlError::new(
                    line,
                    ControlErrorKind::ResourceLimit,
                    format!("result values limit {} exceeded", self.value_limit),
                ));
            }
            let mut stamped = false;
            for (index, node) in selected.iter().enumerate() {
                if next[index] < node.len() && node.time(next[index]) == step {
                    let value = node.value(next[index]);
                    if value != printed[index] {
                        if !stamped {
                            text.push_str(&format!("#{}\n", (step * scale) as u64));
                            stamped = true;
                        }
                        let bit = vcd_value(node, next[index]);
                        let space = if bit.starts_with('r') { " " } else { "" };
                        text.push_str(&format!("{bit}{space}{}\n", ident(index)));
                        printed[index] = value;
                    }
                    next[index] += 1;
                }
            }
        }
        text.push_str("\n\n");
        Ok(text)
    }

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
