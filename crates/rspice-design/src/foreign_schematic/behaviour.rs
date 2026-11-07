//! QSPICE's ¥-device logic gates and Ã-device Gm amplifiers as behavioural
//! subcircuits RSpice runs.
//!
//! Each is written for one placed device, with the settings it states, and
//! goes into the imported project's model library. Pins are the symbol's,
//! in its order, which is QSPICE's order on the device line; a pin nothing
//! reaches is QSPICE's unused `¥` pin and takes no part.
//!
//! A gate's output is QSPICE's: a source between the Vss and Vdd pins, set
//! by the logic state and ramped over `Trise` and `Tfall` (which defaults to
//! `Trise`), behind `Rout` (100 Ω unless stated), an impedance `Zmult` times
//! higher halfway through an edge. Its inputs switch at `Ref` above Vss, the
//! supply midpoint unless stated; the native special-function device decides
//! the state and shapes its edges.
//!
//! A Gm amplifier sources `Gm·V(MULT+,MULT−)·V(min(IN+,IN++), max(IN−,IN−−))`
//! from Vdd and sinks it into Vss. A current limit `L` bends the current `I`
//! to `I·L/(|I|+L)`, the sinking limit defaulting to the sourcing one. An
//! output-voltage limit holds the output with the amplifier's own
//! transconductance, and within `Vdsat` (0.5 V) of a rail the output stage
//! turns to a resistance, as QSPICE's does.

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
            body.gate(&mut settings, Decision::Logic(state), None);
            match kind.as_str() {
                "AND" => "AND gate",
                "OR" => "OR gate",
                _ => "XOR gate",
            }
        }
        ('¥', "HMITT") => {
            let threshold = body.threshold(&mut settings);
            let decision = Decision::Schmitt {
                negative: if pin(5) { "p6" } else { "p2" },
                vt: settings.remove("vt").unwrap_or_else(|| "0".to_owned()),
                vh: settings.remove("vh").unwrap_or_else(|| "0".to_owned()),
            };
            let enable = pin(6).then_some((7, threshold));
            body.gate(&mut settings, decision, enable);
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

    /// A gate decided by `decision`, enabled by the pin `enable` names where
    /// it is used, delayed by `Td` and ramped over `Trise`/`Tfall` into `r`,
    /// and driven onto Q and Q̅ between the supply pins.
    fn gate(
        &mut self,
        settings: &mut BTreeMap<String, String>,
        decision: Decision,
        enable: Option<(usize, String)>,
    ) {
        self.hold_inputs(5);
        let delay = settings.remove("td").or_else(|| settings.remove("td1"));
        let timing = [
            ("Td", delay),
            ("Trise", settings.remove("trise")),
            ("Tfall", settings.remove("tfall")),
        ]
        .into_iter()
        .filter_map(|(key, time)| time.map(|time| format!(" {key}={}", parameter(&time))))
        .collect::<String>();
        let device = match decision {
            Decision::Logic(state) => {
                self.line(format!("Bs s 0 V={state}"));
                "s 0 0 0 0 0 r 0 BUF".to_owned()
            }
            Decision::Schmitt { negative, vt, vh } => format!(
                "p5 {negative} 0 0 0 0 r 0 SCHMITT Vt={} Vh={}",
                parameter(&vt),
                parameter(&vh)
            ),
        };
        self.line(format!("A1 {device} Vhigh=1 Vlow=0{timing}"));
        let high = match enable {
            Some((pin, threshold)) => format!("V(r)*u(V(p{pin})-V(p2)-{threshold})"),
            None => "V(r)".to_owned(),
        };
        let rout = settings
            .remove("rout")
            .or_else(|| settings.remove("rsrc"))
            .unwrap_or_else(|| "100".to_owned());
        let zmult = settings
            .remove("zmult")
            .filter(|zmult| number(zmult) != Some(1.0));
        let mut outputs = vec![("q", "p3", high.clone())];
        if self.ports >= 4 {
            outputs.push(("qb", "p4", format!("(1-{high})")));
        }
        for (source, pin, level) in &outputs {
            match &zmult {
                Some(zmult) => self.line(format!(
                    "B{source} p2 {pin} I=(V(p2)+{level}*(V(p1)-V(p2))-V({pin}))/(({rout})*(1+(({zmult})-1)*4*{level}*(1-{level})))"
                )),
                None => {
                    self.line(format!("B{source} {source} p2 V={level}*(V(p1)-V(p2))"));
                    self.line(format!("R{source} {source} {pin} {rout}"));
                }
            }
        }
        let pins = outputs.iter().map(|(_, pin, _)| *pin).collect::<Vec<_>>();
        self.output_capacitance(settings, &pins);
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
        let drive = format!("({gm})*({positive}-{negative}{offset}){multiplier}");
        // `Ft` is the transconductance's single-pole bandwidth.
        let mut current = match settings.remove("ft") {
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
        let gain = format!("abs(({gm}){multiplier})");
        if let Some(high) = settings.remove("voutmax") {
            current = format!("min({current},{gain}*(({high})-V(p3)+V(p2)))");
        }
        if let Some(low) = settings.remove("voutmin") {
            current = format!("max({current},{gain}*(({low})-V(p3)+V(p2)))");
        }
        let current = format!("{current}{enable}");
        let source = settings.remove("isrc").or_else(|| settings.remove("iout"));
        let sink = settings.remove("isnk").or_else(|| source.clone());
        let current = if source.is_some() || sink.is_some() {
            self.line(format!("Bc c 0 V={current}"));
            let bent = |sign: &str, limit: &Option<String>| match limit {
                Some(limit) => format!("uramp({sign}V(c))*({limit})/(uramp({sign}V(c))+({limit}))"),
                None => format!("uramp({sign}V(c))"),
            };
            format!("{}-{}", bent("", &source), bent("-", &sink))
        } else {
            current
        };
        self.line(format!("Bi i 0 V={current}"));
        let vdsat = settings.remove("vdsat").unwrap_or_else(|| "0.5".to_owned());
        let top = settings.remove("vdsat1").unwrap_or_else(|| vdsat.clone());
        let bottom = settings.remove("vdsat2").unwrap_or(vdsat);
        self.line(format!(
            "Bsrc p1 p3 I=uramp(V(i))*{}",
            saturation("V(p1)-V(p3)", &top)
        ));
        self.line(format!(
            "Bsnk p3 p2 I=uramp(-V(i))*{}",
            saturation("V(p3)-V(p2)", &bottom)
        ));
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

/// What decides a gate's output state.
enum Decision {
    /// A logic expression, 1 while the output is high.
    Logic(String),
    /// A Schmitt trigger on `V(p5) − V(negative)`, high above `vt + vh` and
    /// low below `vt − vh`.
    Schmitt {
        negative: &'static str,
        vt: String,
        vh: String,
    },
}

/// The fraction of its current an output transistor carries `across` volts
/// from its rail: all of it beyond `vdsat`, `x·(2 − x)` of it inside, with
/// `x = across/vdsat`, and a resistance's current past the rail.
fn saturation(across: &str, vdsat: &str) -> String {
    let x = format!("min(({across})/({vdsat}),1)");
    format!("{x}*(2-max({x},0))")
}

/// A setting as a special-function device parameter: a number as written,
/// anything else as an expression.
fn parameter(text: &str) -> String {
    if number(text).is_some() || text.starts_with('{') {
        text.to_owned()
    } else {
        format!("{{{text}}}")
    }
}

/// A setting written as a number, SPICE's scale suffixes read.
fn number(text: &str) -> Option<f64> {
    rspice_vocabulary::quantity::parse_engineering_value(text.trim()).ok()
}
