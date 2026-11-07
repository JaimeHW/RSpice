//! LTspice parts that RSpice carries as its own functional parts.
//!
//! LTspice's behavioral gates, buffers and Schmitt triggers are one
//! eight-terminal device each: five inputs, an inverted and a true output,
//! and a common. A terminal on the common's net, or on none, is not part of
//! the circuit. A part whose common is ground and which drives one output
//! is exactly one of RSpice's functional gates; any other stays LTspice's
//! own device.

use std::collections::{BTreeSet, HashMap};

use rspice_core_netlist::netlist::lexer::parse_spice_value_complete;
use rspice_design_model::Point;

use super::ForeignPin;
use super::import::strictly_inside;
use crate::schematic::functional_catalog::FUNCTIONAL_INPUTS_SETTING;

/// The nets a drawing's wires and flags make: wires join at their ends and
/// where an end meets another wire's middle, and flags of one name join.
#[derive(Debug, Default)]
pub(super) struct SourceNets {
    parent: Vec<usize>,
    points: HashMap<Point, usize>,
    segments: Vec<(Point, Point)>,
    ground: Option<usize>,
}

impl SourceNets {
    pub(super) fn new(segments: &[(Point, Point)], flags: &[(Point, &str)]) -> Self {
        let mut nets = Self {
            segments: segments.to_vec(),
            ..Self::default()
        };
        for &(a, b) in segments {
            let (a, b) = (nets.node(a), nets.node(b));
            nets.join(a, b);
        }
        let tees = segments
            .iter()
            .flat_map(|&(a, b)| [a, b])
            .filter_map(|end| Some((end, nets.segment_through(end)?)))
            .collect::<Vec<_>>();
        for (end, start) in tees {
            let (end, start) = (nets.node(end), nets.node(start));
            nets.join(end, start);
        }
        let mut named = HashMap::<&str, usize>::new();
        for &(point, name) in flags {
            let node = match nets.segment_through(point) {
                Some(start) => nets.node(start),
                None => nets.node(point),
            };
            match named.get(name) {
                Some(&other) => nets.join(node, other),
                None => {
                    named.insert(name, node);
                }
            }
        }
        nets.ground = named.get("0").copied();
        nets
    }

    /// The net at `point`; `None` when nothing drawn reaches it.
    pub(super) fn net(&self, point: Point) -> Option<usize> {
        let node = match self.points.get(&point) {
            Some(node) => *node,
            None => self.points[&self.segment_through(point)?],
        };
        Some(self.root(node))
    }

    pub(super) fn is_ground(&self, net: usize) -> bool {
        self.ground.is_some_and(|ground| self.root(ground) == net)
    }

    /// The start of a wire whose middle `point` lies on.
    fn segment_through(&self, point: Point) -> Option<Point> {
        self.segments
            .iter()
            .find(|(a, b)| strictly_inside(*a, *b, point))
            .map(|(a, _)| *a)
    }

    fn node(&mut self, point: Point) -> usize {
        let next = self.parent.len();
        let node = *self.points.entry(point).or_insert(next);
        if node == next {
            self.parent.push(next);
        }
        node
    }

    fn root(&self, mut node: usize) -> usize {
        while self.parent[node] != node {
            node = self.parent[node];
        }
        node
    }

    fn join(&mut self, a: usize, b: usize) {
        let (a, b) = (self.root(a), self.root(b));
        if a != b {
            self.parent[a] = b;
        }
    }
}

/// A functional part placed for a source part: the source pin, by
/// instance-line index, each of its pins lands on, in the part's terminal
/// order, and its sheet settings.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct FunctionalPlan {
    pub stable_id: &'static str,
    pub pins: Vec<usize>,
    pub params: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Functional {
    Part(FunctionalPlan),
    /// The source's own device stays, for this reason.
    Kept(String),
}

/// What an LTspice gate, buffer or Schmitt trigger becomes, or `None` for
/// any other part. `net(index)` is the net the pin at that instance-line
/// index lands on, `None` when it lands on nothing; `words` are its
/// instance parameters.
pub(super) fn ltspice_gate(
    model: &str,
    pins: &[&ForeignPin],
    net: &dyn Fn(usize) -> Option<usize>,
    is_ground: &dyn Fn(usize) -> bool,
    words: &[String],
) -> Option<Functional> {
    let (positive, negative, schmitt) = match model.to_ascii_uppercase().as_str() {
        "AND" => ("and", Some("nand"), false),
        "OR" => ("or", Some("nor"), false),
        "XOR" => ("xor", None, false),
        "BUF" => ("buffer", Some("inverter"), false),
        "SCHMITT" => ("schmitt_buffer", Some("schmitt_inverter"), true),
        _ => return None,
    };
    let at = |order: usize| pins.iter().position(|pin| pin.spice_order == order);
    let orders = pins
        .iter()
        .map(|pin| pin.spice_order)
        .collect::<BTreeSet<_>>();
    if orders.len() != pins.len() || orders.iter().any(|order| !(1..=8).contains(order)) {
        return None;
    }
    let kept = |reason: &str| Some(Functional::Kept(reason.to_owned()));
    // An unconnected common is ground, as LTspice writes it.
    let common = at(8).and_then(net);
    if common.is_some_and(|common| !is_ground(common)) {
        return kept(
            "has its common on a net other than ground, which RSpice's functional gates take as their reference, so it keeps LTspice's own device",
        );
    }
    let used = |index: usize| net(index).is_some_and(|net| !is_ground(net));
    let inputs = (1..=5)
        .filter_map(at)
        .filter(|index| used(*index))
        .collect::<Vec<_>>();
    let (inverted, output) = match (
        at(7).filter(|index| used(*index)),
        at(6).filter(|index| used(*index)),
    ) {
        (Some(output), None) => (false, output),
        (None, Some(output)) => (true, output),
        (Some(_), Some(_)) => {
            return kept(
                "drives both its true and its inverted output, where RSpice's functional part has one, so it keeps LTspice's own device",
            );
        }
        (None, None) => return kept("drives no output, so it keeps LTspice's own device"),
    };
    // A gate reading one input is a buffer.
    let (positive, negative) = match inputs.len() {
        0 => return kept("reads no input, so it keeps LTspice's own device"),
        1 if !schmitt => ("buffer", Some("inverter")),
        1 => (positive, negative),
        _ if schmitt || positive == "buffer" => {
            return kept("reads more than one input, so it keeps LTspice's own device");
        }
        _ => (positive, negative),
    };
    let Some(part) = (if inverted { negative } else { Some(positive) }) else {
        return kept(
            "drives its inverted output, and RSpice has no inverting part of its kind, so it keeps LTspice's own device",
        );
    };
    // LTspice's own defaults, which RSpice's parts do not share.
    let mut settings = vec![
        ("VHIGH", "1".to_owned()),
        ("VLOW", "0".to_owned()),
        ("TD", "0".to_owned()),
        ("TRISE", "0".to_owned()),
        ("ROUT", "1".to_owned()),
    ];
    let mut fall = None;
    let (mut threshold, mut hysteresis) = (None, None);
    for word in words {
        let Some((key, value)) = word
            .split_once('=')
            .filter(|(key, value)| !key.is_empty() && !value.is_empty())
        else {
            return Some(Functional::Kept(format!(
                "has `{word}` on its line, which RSpice's functional part has no setting for, so it keeps LTspice's own device"
            )));
        };
        let field = match key.to_ascii_lowercase().as_str() {
            "vhigh" => "VHIGH",
            "vlow" => "VLOW",
            "td" => "TD",
            "trise" => "TRISE",
            "rout" => "ROUT",
            "tfall" => {
                fall = Some(value.to_owned());
                continue;
            }
            "vt" if schmitt => {
                threshold = Some(value.to_owned());
                continue;
            }
            "vh" if schmitt => {
                hysteresis = Some(match parse_spice_value_complete(value) {
                    Ok(half) => (2.0 * half).to_string(),
                    Err(_) => format!("{{2*({})}}", value.trim_matches(['{', '}'])),
                });
                continue;
            }
            _ => {
                return Some(Functional::Kept(format!(
                    "sets `{word}`, which RSpice's functional part has no setting for, so it keeps LTspice's own device"
                )));
            }
        };
        if let Some(setting) = settings.iter_mut().find(|(name, _)| *name == field) {
            setting.1 = value.to_owned();
        }
    }
    // LTspice's fall time follows its rise time unless set.
    let fall = fall.unwrap_or_else(|| settings[3].1.clone());
    settings.push(("TFALL", fall));
    if schmitt {
        let (Some(threshold), Some(hysteresis)) = (threshold, hysteresis) else {
            return kept(
                "leaves its trip points (Vt, Vh) at LTspice's defaults, so it keeps LTspice's own device",
            );
        };
        settings.push(("VT", threshold));
        settings.push(("HYST", hysteresis));
    }
    let mut params = Vec::new();
    if matches!(part, "and" | "nand" | "or" | "nor" | "xor") {
        params.push((
            FUNCTIONAL_INPUTS_SETTING.to_owned(),
            inputs.len().to_string(),
        ));
    }
    params.extend(
        settings
            .into_iter()
            .map(|(name, value)| (name.to_owned(), value)),
    );
    let stable_id = match part {
        "and" => "rspice.functional.and",
        "nand" => "rspice.functional.nand",
        "or" => "rspice.functional.or",
        "nor" => "rspice.functional.nor",
        "xor" => "rspice.functional.xor",
        "buffer" => "rspice.functional.buffer",
        "inverter" => "rspice.functional.inverter",
        "schmitt_buffer" => "rspice.functional.schmitt_buffer",
        _ => "rspice.functional.schmitt_inverter",
    };
    let mut pins = inputs;
    pins.push(output);
    Some(Functional::Part(FunctionalPlan {
        stable_id,
        pins,
        params,
    }))
}

/// What LTspice's `UniversalOpAmp2` becomes: at `level2`, its single-pole
/// model with slew, output current and rail limits, RSpice's Op-Amp with the
/// settings `words` give it, pins In+ In- V+ V- OUT in that order. Any other
/// level, or a setting the Op-Amp has no field for, keeps LTspice's part.
pub(super) fn ltspice_opamp(model: &str, words: &[String]) -> Functional {
    if !model.eq_ignore_ascii_case("level2") {
        return Functional::Kept(format!(
            "is LTspice's UniversalOpAmp2 at `{model}`, which RSpice's Op-Amp does not model (it models level2), so it keeps LTspice's own part"
        ));
    }
    let mut params = Vec::new();
    for word in words {
        let Some((key, value)) = word
            .split_once('=')
            .filter(|(key, value)| !key.is_empty() && !value.is_empty())
        else {
            return Functional::Kept(format!(
                "has `{word}` on its line, which RSpice's Op-Amp has no setting for, so it keeps LTspice's own part"
            ));
        };
        let fields: &[&str] = match key.to_ascii_lowercase().as_str() {
            "avol" => &["AVOL"],
            "gbw" => &["GBW"],
            "slew" => &["SLEW"],
            "vos" => &["VOS"],
            "rin" => &["RIN"],
            "en" => &["EN"],
            "enk" => &["ENK"],
            "in" => &["IN"],
            "ink" => &["INK"],
            // LTspice's one output current limit holds both ways.
            "ilimit" => &["ISRC", "ISNK"],
            "rail" => &["HEADROOM"],
            _ => {
                return Functional::Kept(format!(
                    "sets `{word}`, which RSpice's Op-Amp has no setting for, so it keeps LTspice's own part"
                ));
            }
        };
        for field in fields {
            params.retain(|(name, _): &(String, String)| name != field);
            params.push(((*field).to_owned(), value.to_owned()));
        }
    }
    Functional::Part(FunctionalPlan {
        stable_id: "rspice.functional.opamp",
        pins: vec![0, 1, 2, 3, 4],
        params,
    })
}
