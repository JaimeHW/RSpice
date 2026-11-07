//! QSPICE's ¥-device logic gates and Ã-device Gm amplifiers as behavioural
//! subcircuits RSpice runs.
//!
//! Each is written for one placed device, with the settings it states, and
//! goes into the imported project's model library. Pins are the symbol's,
//! in its order, which is QSPICE's order on the device line; a pin nothing
//! reaches is QSPICE's unused `¥` pin and takes no part.
//!
//! A gate's output is QSPICE's: a source between the Vss and Vdd pins, set
//! by the logic state and ramped over `Trise`/`Tfall`, behind `Rout` (100 Ω
//! unless stated). Its inputs switch at `Ref` above Vss, the supply midpoint
//! unless stated. A Gm amplifier sources
//! `Gm·V(MULT+,MULT−)·V(min(IN+,IN++), max(IN−,IN−−))` from Vdd and sinks it
//! into Vss, within the current and output-voltage limits it states.

use std::collections::BTreeMap;

use super::mapping::{spice_text, spice_tokens};

/// A device's subcircuit and what the notice says of it.
pub(super) struct Behaviour {
    /// The subcircuit's `.subckt` text.
    pub card: String,
    /// What the device is, in words.
    pub kind: &'static str,
    /// Stated settings the subcircuit does not model.
    pub unmodelled: Vec<String>,
}

/// The settings every device takes and the subcircuit leaves to QSPICE's
/// defaults without changing what the circuit does: temperature, timing
/// tolerance, and parallel count 1.
const INERT: [&str; 2] = ["temp", "ttol"];

/// The subcircuit `model` for a QSPICE `device` (`¥` or `Ã`) whose value text
/// is `value` and whose pins something reaches as `reached` says, or `None`
/// when it is not a gate or Gm amplifier.
pub(super) fn qspice_behaviour(
    device: char,
    value: &str,
    reached: &[bool],
    model: &str,
) -> Option<Behaviour> {
    // QSPICE's drawings may letter settings in angle brackets
    // (`Gm=<10m>`, `Gm=<1 Ref=.5>`); the settings are what they hold.
    let tokens = spice_tokens(&spice_text(value).replace(['<', '>'], ""));
    let kind = tokens
        .iter()
        .find(|word| !word.contains('='))?
        .to_ascii_uppercase();
    let mut settings = tokens
        .iter()
        .filter_map(|word| word.split_once('='))
        .map(|(key, value)| (key.to_ascii_lowercase(), value.to_owned()))
        .collect::<BTreeMap<_, _>>();
    let pin = |index: usize| reached.get(index).copied().unwrap_or(false);
    let mut body = Body {
        lines: Vec::new(),
        ports: reached.len(),
    };
    let kind = match (device, kind.as_str()) {
        ('¥', "AND" | "OR" | "XOR") => {
            let inputs = (4..9).filter(|index| pin(*index)).collect::<Vec<_>>();
            if inputs.is_empty() {
                return None;
            }
            let threshold = body.threshold(&mut settings);
            let high = inputs
                .iter()
                .map(|index| format!("u(V(p{})-V(p2)-{threshold})", index + 1))
                .collect::<Vec<_>>();
            let state = match kind.as_str() {
                "AND" => high.join("*"),
                "OR" => format!(
                    "1-{}",
                    high.iter()
                        .map(|input| format!("(1-{input})"))
                        .collect::<Vec<_>>()
                        .join("*")
                ),
                _ => high
                    .into_iter()
                    .reduce(|a, b| format!("(({a})+({b})-2*({a})*({b}))"))
                    .unwrap_or_default(),
            };
            body.gate(&mut settings, &state, None);
            match kind.as_str() {
                "AND" => "AND gate",
                "OR" => "OR gate",
                _ => "XOR gate",
            }
        }
        ('¥', "HMITT") => {
            let threshold = body.threshold(&mut settings);
            let negative = if pin(5) { "V(p6)" } else { "V(p2)" };
            let vt = settings.remove("vt").unwrap_or_else(|| "0".to_owned());
            let vh = settings.remove("vh").unwrap_or_else(|| "0".to_owned());
            // The trip point moves from VT+VH to VT−VH once the output is
            // high: the ramped state holds the memory.
            let state = format!("u(V(p5)-{negative}-({vt})-({vh})*(1-2*V(r)))");
            let enable = pin(6).then(|| (7, threshold));
            body.gate(&mut settings, &state, enable);
            "Schmitt trigger"
        }
        ('Ã', "MULTGMAMP") => {
            body.amplifier(&mut settings, pin);
            "multiplying Gm amplifier"
        }
        _ => return None,
    };
    if settings.get("m").is_some_and(|count| count.trim() == "1") {
        settings.remove("m");
    }
    let unmodelled = settings
        .into_keys()
        .filter(|key| !INERT.contains(&key.as_str()))
        .collect();
    let ports = (1..=reached.len())
        .map(|index| format!("p{index}"))
        .collect::<Vec<_>>()
        .join(" ");
    let card = format!(
        "* QSPICE {kind} ({device}-device {value})\n.subckt {model} {ports}\n{}.ends {model}",
        body.lines.concat(),
        value = spice_text(value),
    );
    Some(Behaviour {
        card,
        kind,
        unmodelled,
    })
}

struct Body {
    lines: Vec<String>,
    /// How many pins the symbol has: the subcircuit's ports.
    ports: usize,
}

impl Body {
    fn line(&mut self, line: impl Into<String>) {
        let mut line = line.into();
        line.push('\n');
        self.lines.push(line);
    }

    /// The logic threshold above Vss: `Ref`, else the supply midpoint.
    fn threshold(&self, settings: &mut BTreeMap<String, String>) -> String {
        settings.remove("ref").map_or_else(
            || "(V(p1)-V(p2))/2".to_owned(),
            |reference| format!("({reference})"),
        )
    }

    /// Holds each input pin from 1-based `first` to Vss through 1 GΩ, so an
    /// input nothing drives is no floating node.
    fn hold_inputs(&mut self, first: usize) {
        for input in first..=self.ports {
            self.line(format!("R{input} p{input} p2 1G"));
        }
    }

    /// A gate's state `s`, enabled by the pin `enable` names where it is
    /// used, delayed by `Td`, ramped into `r` over `Trise`/`Tfall`, and
    /// driven onto Q and Q̅ between the supply pins.
    fn gate(
        &mut self,
        settings: &mut BTreeMap<String, String>,
        state: &str,
        enable: Option<(usize, String)>,
    ) {
        self.hold_inputs(5);
        let state = match enable {
            Some((pin, threshold)) => format!("({state})*u(V(p{pin})-V(p2)-{threshold})"),
            None => state.to_owned(),
        };
        self.line(format!("Bs s 0 V={state}"));
        let delay = settings
            .remove("td")
            .or_else(|| settings.remove("td1"))
            .filter(|delay| !is_zero(delay));
        let target = match delay {
            Some(delay) => {
                self.line(format!("Bd d 0 V=delay(V(s),{delay})"));
                "V(d)"
            }
            None => "V(s)",
        };
        let rise = settings.remove("trise").filter(|time| !is_zero(time));
        let fall = settings.remove("tfall").filter(|time| !is_zero(time));
        // With no stated edge the state still settles through `r`, a
        // nanosecond integrator, so the Schmitt trigger's memory has a
        // capacitor to live on.
        let settle = match (&rise, &fall) {
            (Some(rise), Some(fall)) => format!("min({rise},{fall})/1000"),
            (Some(edge), None) | (None, Some(edge)) => format!("({edge})/1000"),
            (None, None) => "1n".to_owned(),
        };
        let slope = |edge: &Option<String>| {
            edge.as_ref()
                .map_or_else(|| format!("1/({settle})"), |edge| format!("1/({edge})"))
        };
        self.line(format!(
            "Br 0 r I=limit(({target}-V(r))/({settle}),-{},{})",
            slope(&fall),
            slope(&rise)
        ));
        self.line("Cr r 0 1");
        let rout = settings
            .remove("rout")
            .or_else(|| settings.remove("rsrc"))
            .unwrap_or_else(|| "100".to_owned());
        self.line("Bq q p2 V=V(r)*(V(p1)-V(p2))");
        self.line(format!("Rq q p3 {rout}"));
        let mut outputs = vec!["p3"];
        if self.ports >= 4 {
            self.line("Bqb qb p2 V=(1-V(r))*(V(p1)-V(p2))");
            self.line(format!("Rqb qb p4 {rout}"));
            outputs.push("p4");
        }
        self.output_capacitance(settings, &outputs);
    }

    /// A Gm amplifier's output current, from pins VDD VSS OUT IN− IN+ MULT+
    /// MULT− IN−− IN++ EN.
    fn amplifier(&mut self, settings: &mut BTreeMap<String, String>, pin: impl Fn(usize) -> bool) {
        self.hold_inputs(4);
        let positive = if pin(8) { "min(V(p5),V(p9))" } else { "V(p5)" };
        let negative = if pin(7) { "max(V(p4),V(p8))" } else { "V(p4)" };
        let offset = settings
            .remove("vos1")
            .map(|offset| format!("+({offset})"))
            .unwrap_or_default();
        let multiplier = if pin(5) || pin(6) {
            "*(V(p6)-V(p7))"
        } else {
            ""
        };
        let gm = settings.remove("gm").unwrap_or_else(|| "1".to_owned());
        let enable = if pin(9) {
            let threshold = self.threshold(settings);
            format!("*u(V(p10)-V(p2)-{threshold})")
        } else {
            settings.remove("ref");
            String::new()
        };
        let drive = format!("({gm})*({positive}-{negative}{offset}){multiplier}{enable}");
        // `Ft` is the transconductance's single-pole bandwidth.
        let current = match settings.remove("ft") {
            Some(bandwidth) => {
                self.line(format!("Bx x 0 V={drive}"));
                self.line("Rx x xf 1");
                let capacitance = number(&bandwidth).map_or_else(
                    || format!("{{1/(2*pi*({bandwidth}))}}"),
                    |hertz| format!("{:e}", 1.0 / (2.0 * std::f64::consts::PI * hertz)),
                );
                self.line(format!("Cx xf 0 {capacitance}"));
                "V(xf)".to_owned()
            }
            None => drive,
        };
        let source = settings
            .remove("isrc")
            .or_else(|| settings.remove("iout"))
            .map_or_else(|| "1e30".to_owned(), |limit| format!("({limit})"));
        let sink = settings
            .remove("isnk")
            .map_or_else(|| "1e30".to_owned(), |limit| format!("({limit})"));
        self.line(format!("Bi i 0 V=limit({current},-{sink},{source})"));
        self.line("Bsrc p1 p3 I=uramp(V(i))");
        self.line("Bsnk p3 p2 I=uramp(-V(i))");
        // An output voltage limit, from Vss, holds the output as a clamp.
        if let Some(high) = settings.remove("voutmax") {
            self.line(format!("Bmax p3 p2 I=1k*uramp(V(p3)-V(p2)-({high}))"));
        }
        if let Some(low) = settings.remove("voutmin") {
            self.line(format!("Bmin p2 p3 I=1k*uramp(({low})-V(p3)+V(p2))"));
        }
        if let Some(rout) = settings.remove("rout") {
            let twice = number(&rout).map_or_else(
                || format!("{{2*({rout})}}"),
                |ohms| format!("{:e}", 2.0 * ohms),
            );
            self.line(format!("Rup p3 p1 {twice}"));
            self.line(format!("Rdown p3 p2 {twice}"));
        }
        self.output_capacitance(settings, &["p3"]);
    }

    /// `CapVdd` and `CapVss` from each output to its supply pin.
    fn output_capacitance(&mut self, settings: &mut BTreeMap<String, String>, outputs: &[&str]) {
        for (key, supply) in [("capvdd", "p1"), ("capvss", "p2")] {
            if let Some(capacitance) = settings.remove(key) {
                for output in outputs {
                    self.line(format!("C{key}{output} {output} {supply} {capacitance}"));
                }
            }
        }
    }
}

/// A setting written as a number, SPICE's scale suffixes read.
fn number(text: &str) -> Option<f64> {
    rspice_vocabulary::quantity::parse_engineering_value(text.trim()).ok()
}

fn is_zero(text: &str) -> bool {
    number(text) == Some(0.0)
}
