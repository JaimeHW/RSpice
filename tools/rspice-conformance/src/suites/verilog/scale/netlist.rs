//! A tiny structural-netlist IR, its Verilog emitter, and the structural
//! metrics the scale suite pins.
//!
//! The IR exists so that the *same* object is both what gets written to disk
//! and what the metrics are computed from. Measuring a netlist by re-parsing
//! the Verilog would put a second Verilog parser in the harness, and a bug in
//! it would be indistinguishable from a bug in the circuit; measuring the
//! structure the emitter was handed cannot drift from what was emitted.
//!
//! Three restrictions keep both the emitter and the flattener honest, and are
//! asserted rather than assumed:
//!
//! * Child modules have **scalar ports only**, and an instance connects each
//!   port to a whole declared net by name. This is what the RSpice front end
//!   accepts (a bit-select in port position is refused by name, IEEE
//!   1364-2005 section 12.3.9), and it is what makes flattening a substitution
//!   rather than a bit-blast.
//! * Vector ports appear only on a top module, and are split into scalar nets
//!   with `buf` and rejoined the same way. That is also where the suite gets
//!   its buffer coverage.
//! * Every net has at most one driver. Multi-driver resolution is the corpus's
//!   job, not this suite's.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Write as _;

/// The eight combinational gate primitives of IEEE 1364-2005 section 7.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gate {
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
    Buf,
    Not,
}

impl Gate {
    /// Every primitive, in the order the suite reports them.
    pub const ALL: [Gate; 8] = [
        Gate::And,
        Gate::Nand,
        Gate::Or,
        Gate::Nor,
        Gate::Xor,
        Gate::Xnor,
        Gate::Buf,
        Gate::Not,
    ];

    pub const fn keyword(self) -> &'static str {
        match self {
            Gate::And => "and",
            Gate::Nand => "nand",
            Gate::Or => "or",
            Gate::Nor => "nor",
            Gate::Xor => "xor",
            Gate::Xnor => "xnor",
            Gate::Buf => "buf",
            Gate::Not => "not",
        }
    }

    /// Whether the primitive takes exactly one input.
    pub const fn is_unary(self) -> bool {
        matches!(self, Gate::Buf | Gate::Not)
    }

    /// Evaluate the primitive over two-valued inputs.
    ///
    /// Used by nothing in the emitted design — it is here so the generator's
    /// own unit tests can check a hand-built cell against its truth table
    /// without going near the simulator.
    pub fn eval(self, inputs: &[bool]) -> bool {
        match self {
            Gate::And => inputs.iter().all(|bit| *bit),
            Gate::Nand => !inputs.iter().all(|bit| *bit),
            Gate::Or => inputs.iter().any(|bit| *bit),
            Gate::Nor => !inputs.iter().any(|bit| *bit),
            Gate::Xor => inputs.iter().filter(|bit| **bit).count() % 2 == 1,
            Gate::Xnor => inputs.iter().filter(|bit| **bit).count() % 2 == 0,
            Gate::Buf => inputs[0],
            Gate::Not => !inputs[0],
        }
    }
}

/// One gate instance: `<keyword> <name> (<output>, <input>, ...);`
#[derive(Debug, Clone)]
pub struct GateInst {
    pub kind: Gate,
    pub name: String,
    pub output: String,
    pub inputs: Vec<String>,
}

/// One child-module instance, connected by name.
#[derive(Debug, Clone)]
pub struct ModInst {
    pub module: String,
    pub name: String,
    /// `(port, net)` in the child's declaration order.
    pub conns: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    In,
    Out,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub name: String,
    pub width: u32,
    pub dir: Dir,
}

/// One Verilog module.
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    /// Comment lines emitted immediately above the module header.
    pub doc: Vec<String>,
    pub ports: Vec<Port>,
    pub wires: Vec<(String, u32)>,
    /// `assign <name> = 1'b<0|1>;` constant sources.
    pub ties: Vec<(String, bool)>,
    pub gates: Vec<GateInst>,
    pub insts: Vec<ModInst>,
}

/// A whole design: child modules first, top module last.
#[derive(Debug, Clone)]
pub struct Design {
    pub top: String,
    /// File-level comment lines, emitted before the first module.
    pub header: Vec<String>,
    pub modules: Vec<Module>,
}

impl Design {
    pub fn module(&self, name: &str) -> Option<&Module> {
        self.modules.iter().find(|module| module.name == name)
    }

    pub fn top_module(&self) -> &Module {
        self.module(&self.top).expect("the top module is present")
    }
}

// ===========================================================================
// Building
// ===========================================================================

/// Incremental construction of one module.
///
/// Net names are generated from a per-prefix counter rather than one global
/// counter, so inserting a cell in one part of a circuit does not renumber
/// every net after it. That keeps a generator edit's diff proportional to the
/// change.
#[derive(Debug)]
pub struct Builder {
    module: Module,
    counters: BTreeMap<String, usize>,
    gate_serial: usize,
}

impl Builder {
    pub fn new(name: &str) -> Self {
        Self {
            module: Module {
                name: name.to_string(),
                doc: Vec::new(),
                ports: Vec::new(),
                wires: Vec::new(),
                ties: Vec::new(),
                gates: Vec::new(),
                insts: Vec::new(),
            },
            counters: BTreeMap::new(),
            gate_serial: 0,
        }
    }

    pub fn doc(&mut self, line: &str) {
        self.module.doc.push(line.to_string());
    }

    pub fn input(&mut self, name: &str, width: u32) {
        self.module.ports.push(Port {
            name: name.to_string(),
            width,
            dir: Dir::In,
        });
    }

    pub fn output(&mut self, name: &str, width: u32) {
        self.module.ports.push(Port {
            name: name.to_string(),
            width,
            dir: Dir::Out,
        });
    }

    /// Declare a named wire and return its name.
    pub fn wire(&mut self, name: &str) -> String {
        self.module.wires.push((name.to_string(), 1));
        name.to_string()
    }

    /// Declare a fresh wire whose name starts with `prefix`.
    pub fn fresh(&mut self, prefix: &str) -> String {
        let counter = self.counters.entry(prefix.to_string()).or_insert(0);
        let name = format!("{prefix}_{counter}");
        *counter += 1;
        self.module.wires.push((name.clone(), 1));
        name
    }

    /// A constant net.
    pub fn tie(&mut self, name: &str, value: bool) -> String {
        let net = self.wire(name);
        self.module.ties.push((net.clone(), value));
        net
    }

    /// Drive an existing net from a gate.
    pub fn drive(&mut self, kind: Gate, output: &str, inputs: &[String]) {
        assert!(
            !inputs.is_empty(),
            "{}: gate {} has no inputs",
            self.module.name,
            self.gate_serial
        );
        assert!(
            !kind.is_unary() || inputs.len() == 1,
            "{}: {} takes one input, got {}",
            self.module.name,
            kind.keyword(),
            inputs.len()
        );
        let name = format!("g{}", self.gate_serial);
        self.gate_serial += 1;
        self.module.gates.push(GateInst {
            kind,
            name,
            output: output.to_string(),
            inputs: inputs.to_vec(),
        });
    }

    /// Drive an existing net from a one-input gate.
    ///
    /// The common shape at a port: a `buf` or a `not` between the net a
    /// circuit computed on and the port it is observed through.
    pub fn drive_from(&mut self, kind: Gate, output: &str, input: &str) {
        let inputs = [input.to_string()];
        self.drive(kind, output, &inputs);
    }

    /// Drive a fresh wire from a gate and return it.
    pub fn gate(&mut self, kind: Gate, prefix: &str, inputs: &[String]) -> String {
        let output = self.fresh(prefix);
        self.drive(kind, &output, inputs);
        output
    }

    pub fn not(&mut self, prefix: &str, a: &str) -> String {
        self.gate(Gate::Not, prefix, &[a.to_string()])
    }

    pub fn buffer(&mut self, prefix: &str, a: &str) -> String {
        self.gate(Gate::Buf, prefix, &[a.to_string()])
    }

    pub fn two(&mut self, kind: Gate, prefix: &str, a: &str, b: &str) -> String {
        self.gate(kind, prefix, &[a.to_string(), b.to_string()])
    }

    /// Reduce `inputs` with `kind`, in a balanced tree of at most `fan_in`
    /// inputs per gate.
    ///
    /// `kind` must be associative in the sense the reduction needs: `and`,
    /// `or`, and `xor` are. Inverting reductions are built as their
    /// non-inverting tree followed by one inverter, because a tree of `nand`
    /// gates does not compute a reduction NAND.
    pub fn reduce(&mut self, kind: Gate, prefix: &str, inputs: &[String], fan_in: usize) -> String {
        assert!(fan_in >= 2, "a reduction tree needs a fan-in of at least 2");
        let (base, invert) = match kind {
            Gate::And | Gate::Nand => (Gate::And, kind == Gate::Nand),
            Gate::Or | Gate::Nor => (Gate::Or, kind == Gate::Nor),
            Gate::Xor | Gate::Xnor => (Gate::Xor, kind == Gate::Xnor),
            Gate::Buf | Gate::Not => panic!("a buffer is not a reduction"),
        };
        assert!(!inputs.is_empty(), "an empty reduction has no value");
        let mut level = inputs.to_vec();
        while level.len() > 1 {
            let mut next = Vec::with_capacity(level.len().div_ceil(fan_in));
            for chunk in level.chunks(fan_in) {
                if chunk.len() == 1 {
                    next.push(chunk[0].clone());
                } else {
                    next.push(self.gate(base, prefix, chunk));
                }
            }
            level = next;
        }
        let value = level.remove(0);
        if invert {
            self.not(prefix, &value)
        } else {
            value
        }
    }

    /// A buffer chain `stages` deep on one net, returning the last stage.
    pub fn buffer_chain(&mut self, prefix: &str, net: &str, stages: usize) -> String {
        let mut current = net.to_string();
        for _ in 0..stages {
            current = self.buffer(prefix, &current);
        }
        current
    }

    /// Split a vector port into scalar nets with `buf`, MSB-first index order
    /// preserved by the returned vector's index.
    pub fn split(&mut self, port: &str, width: u32) -> Vec<String> {
        (0..width)
            .map(|bit| {
                let net = self.fresh(&format!("{port}b"));
                self.drive(Gate::Buf, &net, &[format!("{port}[{bit}]")]);
                net
            })
            .collect()
    }

    /// Join scalar nets onto a vector port with `buf`.
    pub fn join(&mut self, port: &str, bits: &[String]) {
        for (index, net) in bits.iter().enumerate() {
            self.drive_from(Gate::Buf, &format!("{port}[{index}]"), net);
        }
    }

    pub fn instance(&mut self, module: &str, name: &str, conns: &[(&str, String)]) {
        self.module.insts.push(ModInst {
            module: module.to_string(),
            name: name.to_string(),
            conns: conns
                .iter()
                .map(|(port, net)| ((*port).to_string(), net.clone()))
                .collect(),
        });
    }

    /// Insert buffer trees until no net drives more than `limit` gate inputs.
    ///
    /// This is the fanout-limiting pass a technology mapper runs, and it is
    /// why the emitted netlists carry buffers at all. Nets named in `exempt`
    /// keep their full fanout: one broadcast net in the suite is deliberately
    /// left unbuffered because a net driving a hundred gate inputs is itself
    /// the thing being stressed.
    ///
    /// Nets a module instance touches are never buffered, because this pass
    /// cannot see which side of the child's port a connection is on.
    pub fn limit_fanout(&mut self, limit: usize, exempt: &[&str]) {
        assert!(limit >= 2, "a fanout limit below two cannot terminate");
        let exempt: BTreeSet<&str> = exempt.iter().copied().collect();
        let instance_nets: BTreeSet<String> = self
            .module
            .insts
            .iter()
            .flat_map(|inst| inst.conns.iter().map(|(_, net)| net.clone()))
            .collect();

        loop {
            let mut load: BTreeMap<String, usize> = BTreeMap::new();
            for gate in &self.module.gates {
                for input in &gate.inputs {
                    *load.entry(input.clone()).or_insert(0) += 1;
                }
            }
            let Some((net, count)) = load
                .into_iter()
                .filter(|(net, count)| {
                    *count > limit && !exempt.contains(net.as_str()) && !instance_nets.contains(net)
                })
                .max_by(|left, right| left.1.cmp(&right.1).then(right.0.cmp(&left.0)))
            else {
                return;
            };

            // One buffer per `limit` loads, then the loads are dealt out to
            // them in the order they appear. A second pass buffers the
            // buffers if the tree is still too wide, which is what makes this
            // a loop rather than a single rewrite.
            let branches = count.div_ceil(limit);
            let buffers: Vec<String> = (0..branches)
                .map(|_| {
                    let out = self.fresh("fob");
                    self.drive_from(Gate::Buf, &out, &net);
                    out
                })
                .collect();

            let mut seen = 0usize;
            for gate in &mut self.module.gates {
                if gate.kind == Gate::Buf && buffers.contains(&gate.output) {
                    continue;
                }
                for input in &mut gate.inputs {
                    if *input == net {
                        *input = buffers[seen / limit].clone();
                        seen += 1;
                    }
                }
            }
        }
    }

    pub fn finish(self) -> Module {
        self.module
    }
}

// ===========================================================================
// Emission
// ===========================================================================

/// Render a design as Verilog.
///
/// `\n` is written explicitly and nothing platform-dependent reaches the
/// string, so the bytes are the same on every host. That matters because the
/// vendored copies are byte-compared against a fresh run.
pub fn emit(design: &Design) -> String {
    let mut out = String::new();
    for line in &design.header {
        if line.is_empty() {
            out.push_str("//\n");
        } else {
            let _ = writeln!(out, "// {line}");
        }
    }
    for module in &design.modules {
        out.push('\n');
        emit_module(&mut out, module);
    }
    out
}

fn emit_module(out: &mut String, module: &Module) {
    for line in &module.doc {
        if line.is_empty() {
            out.push_str("//\n");
        } else {
            let _ = writeln!(out, "// {line}");
        }
    }
    let names: Vec<&str> = module.ports.iter().map(|port| port.name.as_str()).collect();
    let _ = writeln!(out, "module {} ({});", module.name, wrap(&names, 2));

    for dir in [Dir::In, Dir::Out] {
        let keyword = match dir {
            Dir::In => "input",
            Dir::Out => "output",
        };
        // Grouped by width so a scalar-ported circuit declares one line per
        // run rather than one line per port.
        let mut widths: Vec<u32> = module
            .ports
            .iter()
            .filter(|port| port.dir == dir)
            .map(|port| port.width)
            .collect();
        widths.dedup();
        widths.sort_unstable();
        widths.dedup();
        for width in widths {
            let group: Vec<&str> = module
                .ports
                .iter()
                .filter(|port| port.dir == dir && port.width == width)
                .map(|port| port.name.as_str())
                .collect();
            if group.is_empty() {
                continue;
            }
            let range = if width == 1 {
                String::new()
            } else {
                format!(" [{}:0]", width - 1)
            };
            let _ = writeln!(out, "  {keyword}{range} {};", wrap(&group, 4));
        }
    }

    if !module.wires.is_empty() {
        let names: Vec<&str> = module.wires.iter().map(|(name, _)| name.as_str()).collect();
        let _ = writeln!(out, "  wire {};", wrap(&names, 4));
    }
    for (name, value) in &module.ties {
        let _ = writeln!(out, "  assign {name} = 1'b{};", u8::from(*value));
    }
    if !module.ties.is_empty() || !module.wires.is_empty() {
        out.push('\n');
    }

    for gate in &module.gates {
        let mut terminals = vec![gate.output.clone()];
        terminals.extend(gate.inputs.iter().cloned());
        let refs: Vec<&str> = terminals.iter().map(String::as_str).collect();
        let _ = writeln!(
            out,
            "  {} {} ({});",
            gate.kind.keyword(),
            gate.name,
            wrap(&refs, 4)
        );
    }
    for inst in &module.insts {
        let conns: Vec<String> = inst
            .conns
            .iter()
            .map(|(port, net)| format!(".{port}({net})"))
            .collect();
        let refs: Vec<&str> = conns.iter().map(String::as_str).collect();
        let _ = writeln!(out, "  {} {} ({});", inst.module, inst.name, wrap(&refs, 4));
    }
    out.push_str("endmodule\n");
}

/// Join `items` with `, `, folding onto continuation lines past 76 columns.
fn wrap(items: &[&str], indent: usize) -> String {
    let mut out = String::new();
    let mut column = indent;
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            out.push(',');
            column += 1;
            if column + item.len() + 1 > 76 {
                out.push('\n');
                out.push_str(&" ".repeat(indent + 2));
                column = indent + 2;
            } else {
                out.push(' ');
                column += 1;
            }
        }
        out.push_str(item);
        column += item.len();
    }
    out
}

// ===========================================================================
// Flattening and metrics
// ===========================================================================

/// Every gate in a design, with hierarchy resolved away.
#[derive(Debug, Clone)]
pub struct Flat {
    pub gates: Vec<GateInst>,
    /// Nets that carry a constant.
    pub ties: BTreeSet<String>,
    /// The top module's primary input nets, including per-bit selects.
    pub primary_inputs: BTreeSet<String>,
    /// The top module's primary output nets, including per-bit selects.
    pub primary_outputs: Vec<String>,
    pub instances: usize,
}

/// Inline every module instance into one gate list.
pub fn flatten(design: &Design) -> Flat {
    let mut flat = Flat {
        gates: Vec::new(),
        ties: BTreeSet::new(),
        primary_inputs: BTreeSet::new(),
        primary_outputs: Vec::new(),
        instances: 0,
    };
    let top = design.top_module();
    for port in &top.ports {
        for bit in 0..port.width {
            let net = if port.width == 1 {
                port.name.clone()
            } else {
                format!("{}[{bit}]", port.name)
            };
            match port.dir {
                Dir::In => {
                    flat.primary_inputs.insert(net);
                }
                Dir::Out => flat.primary_outputs.push(net),
            }
        }
    }
    inline(design, top, "", &BTreeMap::new(), &mut flat);
    flat
}

fn inline(
    design: &Design,
    module: &Module,
    prefix: &str,
    subst: &BTreeMap<String, String>,
    flat: &mut Flat,
) {
    let resolve = |net: &str| -> String {
        if let Some(mapped) = subst.get(net) {
            return mapped.clone();
        }
        format!("{prefix}{net}")
    };

    for (net, _) in &module.ties {
        flat.ties.insert(resolve(net));
    }
    for gate in &module.gates {
        flat.gates.push(GateInst {
            kind: gate.kind,
            name: format!("{prefix}{}", gate.name),
            output: resolve(&gate.output),
            inputs: gate.inputs.iter().map(|net| resolve(net)).collect(),
        });
    }
    for inst in &module.insts {
        flat.instances += 1;
        let child = design
            .module(&inst.module)
            .unwrap_or_else(|| panic!("instance {} names an absent module", inst.name));
        for port in &child.ports {
            assert_eq!(
                port.width, 1,
                "child module {} has a vector port; the suite keeps instance ports scalar",
                child.name
            );
        }
        let mut child_subst = BTreeMap::new();
        for (port, net) in &inst.conns {
            child_subst.insert(port.clone(), resolve(net));
        }
        assert_eq!(
            child_subst.len(),
            child.ports.len(),
            "instance {} connects {} of {}'s {} ports",
            inst.name,
            child_subst.len(),
            child.name,
            child.ports.len()
        );
        let child_prefix = format!("{prefix}{}/", inst.name);
        inline(design, child, &child_prefix, &child_subst, flat);
    }
}

/// The structural facts the suite asserts about a circuit.
#[derive(Debug, Clone)]
pub struct Metrics {
    /// Gates after flattening, which is the number a simulator evaluates.
    pub gates: usize,
    /// Module instances after flattening.
    pub instances: usize,
    /// Gate primitives actually used.
    pub kinds: BTreeSet<Gate>,
    /// The widest gate in the design.
    pub max_fan_in: usize,
    /// The heaviest net, and how many gate inputs it drives.
    pub max_fan_out: usize,
    pub max_fan_out_net: String,
    /// Longest path from a primary input to a primary output, in gate levels.
    pub depth: usize,
    /// Scalar port bits: inputs, outputs, and the total.
    pub input_bits: usize,
    pub output_bits: usize,
    pub ports: usize,
    /// Declared ports, whatever their width.
    pub declared_ports: usize,
    /// Whether every declared port is one bit wide.
    pub scalar_ported: bool,
}

/// Measure a design.
///
/// Panics if the flattened netlist has a combinational loop or a net with two
/// drivers, because both are authoring mistakes that would otherwise reach a
/// simulator as a hang or a race.
pub fn measure(design: &Design) -> Metrics {
    let flat = flatten(design);
    let top = design.top_module();

    let mut driver: BTreeMap<&str, usize> = BTreeMap::new();
    for (index, gate) in flat.gates.iter().enumerate() {
        assert!(
            driver.insert(&gate.output, index).is_none(),
            "net '{}' has two drivers",
            gate.output
        );
    }

    let mut fan_out: BTreeMap<&str, usize> = BTreeMap::new();
    let mut max_fan_in = 0usize;
    let mut kinds = BTreeSet::new();
    for gate in &flat.gates {
        kinds.insert(gate.kind);
        max_fan_in = max_fan_in.max(gate.inputs.len());
        for input in &gate.inputs {
            *fan_out.entry(input.as_str()).or_insert(0) += 1;
        }
    }
    let (max_fan_out_net, max_fan_out) = fan_out
        .iter()
        .max_by(|left, right| left.1.cmp(right.1).then(right.0.cmp(left.0)))
        .map(|(net, count)| ((*net).to_string(), *count))
        .unwrap_or_default();

    // Longest path by Kahn order over the gate graph. A net with no driver is
    // a primary input or a tie and sits at level zero.
    let mut remaining: Vec<usize> = flat
        .gates
        .iter()
        .map(|gate| {
            gate.inputs
                .iter()
                .filter(|net| driver.contains_key(net.as_str()))
                .count()
        })
        .collect();
    let mut consumers: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (index, gate) in flat.gates.iter().enumerate() {
        for input in &gate.inputs {
            consumers.entry(input.as_str()).or_default().push(index);
        }
    }
    let mut level: BTreeMap<&str, usize> = BTreeMap::new();
    let mut queue: VecDeque<usize> = remaining
        .iter()
        .enumerate()
        .filter(|(_, count)| **count == 0)
        .map(|(index, _)| index)
        .collect();
    let mut settled = 0usize;
    while let Some(index) = queue.pop_front() {
        settled += 1;
        let gate = &flat.gates[index];
        let depth = 1 + gate
            .inputs
            .iter()
            .map(|net| level.get(net.as_str()).copied().unwrap_or(0))
            .max()
            .unwrap_or(0);
        level.insert(&gate.output, depth);
        for consumer in consumers.get(gate.output.as_str()).into_iter().flatten() {
            remaining[*consumer] -= 1;
            if remaining[*consumer] == 0 {
                queue.push_back(*consumer);
            }
        }
    }
    assert_eq!(
        settled,
        flat.gates.len(),
        "the flattened netlist of '{}' has a combinational loop",
        design.top
    );
    let depth = flat
        .primary_outputs
        .iter()
        .map(|net| level.get(net.as_str()).copied().unwrap_or(0))
        .max()
        .unwrap_or(0);

    let input_bits = top
        .ports
        .iter()
        .filter(|port| port.dir == Dir::In)
        .map(|port| port.width as usize)
        .sum();
    let output_bits = top
        .ports
        .iter()
        .filter(|port| port.dir == Dir::Out)
        .map(|port| port.width as usize)
        .sum();

    Metrics {
        gates: flat.gates.len(),
        instances: flat.instances,
        kinds,
        max_fan_in,
        max_fan_out,
        max_fan_out_net,
        depth,
        input_bits,
        output_bits,
        ports: input_bits + output_bits,
        declared_ports: top.ports.len(),
        scalar_ported: top.ports.iter().all(|port| port.width == 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn half_adder() -> Design {
        let mut b = Builder::new("ha");
        b.input("a", 1);
        b.input("b", 1);
        b.output("s", 1);
        b.output("c", 1);
        b.drive(Gate::Xor, "s", &["a".into(), "b".into()]);
        b.drive(Gate::And, "c", &["a".into(), "b".into()]);
        Design {
            top: "ha".into(),
            header: Vec::new(),
            modules: vec![b.finish()],
        }
    }

    #[test]
    fn emission_is_stable_and_declares_what_it_uses() {
        let design = half_adder();
        let text = emit(&design);
        assert_eq!(text, emit(&design));
        assert!(text.contains("module ha (a, b, s, c);"), "{text}");
        assert!(text.contains("xor g0 (s, a, b);"), "{text}");
        assert!(text.contains("and g1 (c, a, b);"), "{text}");
    }

    #[test]
    fn a_reduction_tree_respects_its_fan_in() {
        let mut b = Builder::new("t");
        b.input("a", 8);
        b.output("y", 1);
        let bits: Vec<String> = (0..8).map(|bit| format!("a[{bit}]")).collect();
        let value = b.reduce(Gate::Or, "r", &bits, 3);
        b.drive(Gate::Buf, "y", &[value]);
        let module = b.finish();
        // 8 inputs at fan-in 3: three gates, then two, then one.
        assert_eq!(
            module
                .gates
                .iter()
                .filter(|gate| gate.kind == Gate::Or)
                .count(),
            4
        );
        assert!(module.gates.iter().all(|gate| gate.inputs.len() <= 3));
    }

    #[test]
    fn an_inverting_reduction_is_a_tree_plus_one_inverter() {
        let mut b = Builder::new("t");
        b.input("a", 4);
        b.output("y", 1);
        let bits: Vec<String> = (0..4).map(|bit| format!("a[{bit}]")).collect();
        let value = b.reduce(Gate::Nor, "r", &bits, 4);
        b.drive(Gate::Buf, "y", &[value]);
        let module = b.finish();
        assert_eq!(module.gates[0].kind, Gate::Or);
        assert_eq!(module.gates[1].kind, Gate::Not);
    }

    #[test]
    fn fanout_limiting_preserves_the_function_and_bounds_the_load() {
        let mut b = Builder::new("t");
        b.input("a", 1);
        b.input("d", 16);
        b.output("y", 16);
        for bit in 0..16 {
            b.drive(
                Gate::And,
                &format!("y[{bit}]"),
                &["a".into(), format!("d[{bit}]")],
            );
        }
        b.limit_fanout(4, &[]);
        let module = b.finish();

        let mut load = BTreeMap::new();
        for gate in &module.gates {
            for input in &gate.inputs {
                *load.entry(input.clone()).or_insert(0usize) += 1;
            }
        }
        assert!(
            load.values().all(|count| *count <= 4),
            "a net still drives more than four inputs: {load:?}"
        );
        assert!(module.gates.iter().any(|gate| gate.kind == Gate::Buf));
    }

    #[test]
    fn an_exempt_net_keeps_its_fanout() {
        let mut b = Builder::new("t");
        b.input("a", 1);
        b.input("d", 16);
        b.output("y", 16);
        for bit in 0..16 {
            b.drive(
                Gate::And,
                &format!("y[{bit}]"),
                &["a".into(), format!("d[{bit}]")],
            );
        }
        b.limit_fanout(4, &["a"]);
        let module = b.finish();
        assert!(module.gates.iter().all(|gate| gate.kind == Gate::And));
    }

    #[test]
    fn flattening_substitutes_child_ports_and_prefixes_child_nets() {
        let mut child = Builder::new("cell");
        child.input("a", 1);
        child.input("b", 1);
        child.output("y", 1);
        let inner = child.two(Gate::Nand, "n", "a", "b");
        child.drive(Gate::Not, "y", &[inner]);

        let mut top = Builder::new("top");
        top.input("p", 1);
        top.input("q", 1);
        top.output("z", 1);
        top.instance(
            "cell",
            "u0",
            &[("a", "p".into()), ("b", "q".into()), ("y", "z".into())],
        );

        let design = Design {
            top: "top".into(),
            header: Vec::new(),
            modules: vec![child.finish(), top.finish()],
        };
        let flat = flatten(&design);
        assert_eq!(flat.instances, 1);
        assert_eq!(flat.gates.len(), 2);
        assert_eq!(flat.gates[0].inputs, vec!["p".to_string(), "q".to_string()]);
        assert_eq!(flat.gates[0].output, "u0/n_0");
        assert_eq!(flat.gates[1].output, "z");

        let metrics = measure(&design);
        assert_eq!(metrics.depth, 2);
        assert_eq!(metrics.gates, 2);
        assert!(metrics.scalar_ported);
    }

    #[test]
    fn depth_counts_gate_levels_to_a_primary_output() {
        let mut b = Builder::new("t");
        b.input("a", 1);
        b.output("y", 1);
        let chain = b.buffer_chain("c", "a", 5);
        b.drive(Gate::Not, "y", &[chain]);
        let design = Design {
            top: "t".into(),
            header: Vec::new(),
            modules: vec![b.finish()],
        };
        assert_eq!(measure(&design).depth, 6);
    }

    #[test]
    fn gate_evaluation_matches_the_primitive_truth_tables() {
        assert!(Gate::And.eval(&[true, true, true]));
        assert!(!Gate::And.eval(&[true, false, true]));
        assert!(Gate::Nand.eval(&[true, false]));
        assert!(Gate::Or.eval(&[false, true]));
        assert!(Gate::Nor.eval(&[false, false]));
        assert!(Gate::Xor.eval(&[true, false, false]));
        assert!(Gate::Xnor.eval(&[true, true]));
        assert!(Gate::Buf.eval(&[true]));
        assert!(Gate::Not.eval(&[false]));
    }
}
