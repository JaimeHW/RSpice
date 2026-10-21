//! The converters the build inserted on mixed boundaries, as data.
//!
//! A boundary is a deck net where an analog node meets a discrete port, and
//! the build bridges each one with a converter: the built-in connect-rule
//! converter, ngspice's stock bridge, an authored connect module or the card
//! an auto-bridge template generates. The build states each in its log; a
//! [`ConverterInsertion`] is the same statement in a shape a reader can act
//! on, kept on the circuit ([`CircuitData::converter_insertions`]). The log
//! line is rendered from the record, so the two never disagree.

use std::fmt;

use super::CircuitData;
use crate::xspice::PortDirection;

/// The rule set whose defaults stand where no declaration states one.
pub(crate) const BUILTIN_RULE_SET: &str = "rspice_builtin";

/// One converter the build inserted on a mixed boundary, and where each of
/// its values came from.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[non_exhaustive]
pub struct ConverterInsertion {
    /// The boundary net, as the deck names it.
    pub boundary: String,
    /// The instance and port whose discrete side meets the net, `XCTRL.sda`:
    /// an HDL instance's port, or the first XSPICE port on the net.
    pub port: Option<String>,
    /// The direction of that discrete port: an input reads the net through
    /// an analog-to-digital converter.
    pub direction: PortDirection,
    pub kind: ConverterKind,
    /// What converts: `a2d`, an authored connect module, `adc_bridge`, or the
    /// model or subcircuit a template's card names.
    pub module: String,
    /// The converter's instance.
    pub instance: String,
    /// What chose the converter: a connect rule, the rule set an HDL port no
    /// connect rule reaches converts with, an auto-bridge template, or
    /// ngspice's auto-bridge.
    pub rules: ConverterSource,
    /// Where the connect statement that selected it is written, `file:line`.
    pub location: Option<String>,
    /// The supply it converts against. `None` for real-valued traffic, which
    /// carries a number rather than a logic level, and for an authored module
    /// whose statement states none.
    pub supply: Option<ConverterSupply>,
    /// Every parameter, with its value and where the value came from.
    pub parameters: Vec<ConverterParameter>,
}

/// Which converter bridges a boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ConverterKind {
    /// The built-in connect-rule analog-to-digital converter.
    A2d,
    /// The built-in connect-rule digital-to-analog converter.
    D2a,
    /// The built-in connect-rule bidirectional converter.
    Bidir,
    /// ngspice's stock `adc_bridge`, on an XSPICE boundary no connect rule
    /// reaches.
    AdcBridge,
    DacBridge,
    BidiBridge,
    RealToV,
    VToReal,
    /// An authored connect module, executing its own body.
    Authored,
    /// The card an auto-bridge template generates.
    Template,
}

/// Where a value, or a converter, came from.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ConverterSource {
    /// The connect statement's own override (`connect d2a #(.rhi(10));`).
    ConnectStatement,
    /// What chose a converter: a connect statement in the `connectrules`
    /// block named. A module that copies the shipped declaration's earlier
    /// signature declares only some parameters, and the built-in rule set
    /// supplies the rest.
    ConnectRule {
        block: String,
        earlier_signature: bool,
    },
    /// The selected declaration's default.
    ModuleDefault,
    /// The named rule set's default.
    RuleSet(String),
    /// The deck's parameter (`.param vcc`).
    Parameter(String),
    /// The one rail that reaches the boundary, at its level in volts.
    Inferred { source: String, level: f64 },
    /// No supply reaches the boundary; the default level, in volts.
    Default(f64),
    /// ngspice's auto-bridge default.
    AutoBridge,
    /// The named auto-bridge template.
    Template(String),
}

/// The supply a converter converts against.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[non_exhaustive]
pub struct ConverterSupply {
    /// Volts; for a supply bound live, its nominal level.
    pub level: f64,
    /// The net it is bound to live, if it is.
    pub net: Option<String>,
    pub source: ConverterSource,
}

/// One resolved parameter of a converter.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[non_exhaustive]
pub struct ConverterParameter {
    pub name: String,
    /// In `unit`; for a level that tracks a live supply, its value at the
    /// supply's nominal level. Infinite where the parameter allows it (`txdel`,
    /// `rz`), which JSON writes as `null`.
    pub value: f64,
    /// `V`, `s` or `Ω`.
    pub unit: String,
    /// The default's rule, as the declaration writes it (`vsup/2`).
    pub rule: Option<String>,
    pub source: ConverterSource,
    /// A default level that follows a live supply as a fraction of it.
    pub tracks: bool,
}

impl ConverterSupply {
    pub(crate) fn new(level: f64, net: Option<String>, source: ConverterSource) -> Self {
        Self { level, net, source }
    }
}

impl ConverterParameter {
    pub(crate) fn new(
        name: &str,
        value: f64,
        unit: &str,
        rule: Option<&str>,
        source: ConverterSource,
        tracks: bool,
    ) -> Self {
        Self {
            name: name.to_owned(),
            value,
            unit: unit.to_owned(),
            rule: rule.map(str::to_owned),
            source,
            tracks,
        }
    }
}

impl ConverterInsertion {
    /// A record with no supply, parameters or location; the build adds what
    /// the converter has.
    pub(crate) fn new(
        boundary: String,
        port: Option<String>,
        direction: PortDirection,
        kind: ConverterKind,
        module: String,
        instance: String,
        rules: ConverterSource,
    ) -> Self {
        Self {
            boundary,
            port,
            direction,
            kind,
            module,
            instance,
            rules,
            location: None,
            supply: None,
            parameters: Vec::new(),
        }
    }

    fn is_built_in(&self) -> bool {
        matches!(
            self.kind,
            ConverterKind::A2d | ConverterKind::D2a | ConverterKind::Bidir
        )
    }

    /// What it converts, as the stock bridges' log line names it.
    fn conversion(&self) -> &'static str {
        let digital = self.supply.is_some();
        match (self.direction, digital) {
            (PortDirection::In, true) => "analog-to-digital",
            (PortDirection::Out, true) => "digital-to-analog",
            (PortDirection::InOut, true) => "bidirectional digital/analog",
            (PortDirection::Out, false) => "real-to-voltage",
            (PortDirection::In, false) => "voltage-to-real",
            (PortDirection::InOut, false) => "bidirectional real/electrical",
        }
    }
}

impl CircuitData {
    /// Every converter the build inserted on a mixed boundary, in the order
    /// it inserted them.
    pub fn converter_insertions(&self) -> &[ConverterInsertion] {
        &self.converter_insertions
    }
}

fn direction_label(direction: PortDirection) -> &'static str {
    match direction {
        PortDirection::In => "input",
        PortDirection::Out => "output",
        PortDirection::InOut => "inout",
    }
}

/// `value` with an SI prefix and `unit`: `1.65 V`, `2 ns`, `inf`.
fn si(value: f64, unit: &str) -> String {
    if value.is_infinite() {
        return if value > 0.0 {
            "inf".into()
        } else {
            "-inf".into()
        };
    }
    if value == 0.0 {
        return format!("0 {unit}");
    }
    const PREFIXES: [(f64, &str); 9] = [
        (1e9, "G"),
        (1e6, "M"),
        (1e3, "k"),
        (1.0, ""),
        (1e-3, "m"),
        (1e-6, "u"),
        (1e-9, "n"),
        (1e-12, "p"),
        (1e-15, "f"),
    ];
    let magnitude = value.abs();
    let (scale, prefix) = PREFIXES
        .iter()
        .copied()
        .find(|(scale, _)| magnitude >= *scale * (1.0 - 1e-12))
        .unwrap_or((1e-15, "f"));
    let scaled = format!("{:.6}", value / scale);
    let scaled = scaled.trim_end_matches('0').trim_end_matches('.');
    format!("{scaled} {prefix}{unit}")
}

impl fmt::Display for ConverterSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectStatement => f.write_str("connect statement"),
            Self::ConnectRule {
                earlier_signature: false,
                ..
            } => f.write_str("connect rule"),
            Self::ConnectRule {
                earlier_signature: true,
                ..
            } => write!(
                f,
                "connect rule, earlier signature, defaults from rule set {BUILTIN_RULE_SET}"
            ),
            Self::ModuleDefault => f.write_str("module default"),
            Self::RuleSet(name) => write!(f, "rule set {name}"),
            Self::Parameter(name) => write!(f, ".param {name}"),
            Self::Inferred { source, level } => write!(f, "inferred from {source} = {level} V"),
            Self::Default(level) => write!(f, "default {level} V"),
            Self::AutoBridge => f.write_str("ngspice auto-bridge default"),
            Self::Template(key) => write!(f, "template {key}"),
        }
    }
}

/// `rhi=10 Ω (connect statement)`; the alternate form is the value alone,
/// `rhi=10 Ω`.
impl fmt::Display for ConverterParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, si(self.value, &self.unit))?;
        if f.alternate() {
            return Ok(());
        }
        f.write_str(" (")?;
        match (&self.source, &self.rule) {
            (ConverterSource::ConnectStatement, _) | (_, None) => write!(f, "{}", self.source)?,
            (source, Some(rule)) => write!(f, "{source} {rule}")?,
        }
        if self.tracks {
            f.write_str(", tracks")?;
        }
        f.write_str(")")
    }
}

/// `vsup=net vdd, live (connect statement)`; the alternate form is the
/// supply alone, `vdd, live` or `3.3 V`.
impl fmt::Display for ConverterSupply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.net, f.alternate()) {
            (Some(net), true) => write!(f, "{net}, live"),
            (None, true) => f.write_str(&si(self.level, "V")),
            (Some(net), false) => write!(f, "vsup=net {net}, live ({})", self.source),
            (None, false) => write!(f, "vsup={} ({}, static)", si(self.level, "V"), self.source),
        }
    }
}

/// The log record of one converter.
impl fmt::Display for ConverterInsertion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&record_text(std::slice::from_ref(self)))
    }
}

/// The log record of `group`: one converter, or the one instance a template
/// generates for several boundaries. This is the only place a converter's
/// log text is written.
pub(crate) fn record_text(group: &[ConverterInsertion]) -> String {
    let Some(first) = group.first() else {
        return String::new();
    };
    if first.is_built_in() {
        let port = first
            .port
            .as_deref()
            .map(|port| format!("{port}, "))
            .unwrap_or_default();
        let rules = match &first.rules {
            ConverterSource::RuleSet(name) => format!("no connect rule, rule set {name}"),
            rules => rules.to_string(),
        };
        let mut text = format!(
            "boundary {} ({port}{}): {} as {} [{rules}]\n  ",
            first.boundary,
            direction_label(first.direction),
            first.module,
            first.instance
        );
        let mut items = first
            .supply
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        items.extend(first.parameters.iter().map(ToString::to_string));
        text.push_str(&items.join("; "));
        return text;
    }
    match (&first.kind, &first.rules) {
        (ConverterKind::Authored, _) => format!(
            "Node '{}' bridges through connect module '{}' as instance '{}'",
            first.boundary, first.module, first.instance
        ),
        (ConverterKind::Template, ConverterSource::Template(key)) => {
            let nodes = group
                .iter()
                .map(|record| record.boundary.as_str())
                .collect::<Vec<_>>();
            let nodes = format!(
                "{} {}",
                if nodes.len() == 1 { "node" } else { "nodes" },
                nodes.join(" ")
            );
            let subcircuit = first
                .instance
                .chars()
                .next()
                .is_some_and(|initial| initial.eq_ignore_ascii_case(&'x'));
            let mut text = if subcircuit {
                format!(
                    "Inserted {} bridge subcircuit for {nodes} from bridge template '{key}'",
                    first.conversion()
                )
            } else {
                format!(
                    "Inserted {} bridge {} for {nodes} from bridge template '{key}'",
                    first.conversion(),
                    first.instance
                )
            };
            for record in group {
                let level = record.supply.as_ref().map_or(0.0, |supply| supply.level);
                text.push_str(&format!(
                    "\nboundary {}: template {key}, static at {level} V",
                    record.boundary
                ));
            }
            text
        }
        _ => format!(
            "Inserted {} bridge {} for node {}",
            first.conversion(),
            first.instance,
            first.boundary
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The built-in converter's record is the multi-line statement of every
    /// value and its source, after the boundary, port and module.
    #[test]
    fn a_built_in_record_states_each_value_with_its_source() {
        let mut record = ConverterInsertion::new(
            "sda".into(),
            Some("XCTRL.sda".into()),
            PortDirection::InOut,
            ConverterKind::Bidir,
            "bidir".into(),
            "__auto_bridge_bidi_7".into(),
            ConverterSource::RuleSet(BUILTIN_RULE_SET.into()),
        );
        record.supply = Some(ConverterSupply::new(
            3.3,
            None,
            ConverterSource::Parameter("vcc".into()),
        ));
        record.parameters = vec![
            ConverterParameter::new(
                "vthi",
                1.65,
                "V",
                Some("vsup/2"),
                ConverterSource::RuleSet(BUILTIN_RULE_SET.into()),
                false,
            ),
            ConverterParameter::new(
                "rz",
                f64::INFINITY,
                "Ω",
                None,
                ConverterSource::RuleSet(BUILTIN_RULE_SET.into()),
                false,
            ),
            ConverterParameter::new(
                "trise",
                2e-9,
                "s",
                None,
                ConverterSource::ConnectStatement,
                false,
            ),
        ];
        assert_eq!(
            record.to_string(),
            "boundary sda (XCTRL.sda, inout): bidir as __auto_bridge_bidi_7 [no connect rule, \
             rule set rspice_builtin]\n  vsup=3.3 V (.param vcc, static); vthi=1.65 V (rule set \
             rspice_builtin vsup/2); rz=inf (rule set rspice_builtin); trise=2 ns (connect \
             statement)"
        );
        assert_eq!(format!("{:#}", record.parameters[2]), "trise=2 ns");
    }

    /// A template's instance bridging two nodes is one sentence naming both,
    /// and each node says it is static at the template's level.
    #[test]
    fn a_template_group_names_every_node_and_says_static() {
        let record = |node: &str| {
            let mut record = ConverterInsertion::new(
                node.into(),
                None,
                PortDirection::Out,
                ConverterKind::Template,
                "tdac".into(),
                "atmpl3".into(),
                ConverterSource::Template("auto_bridge_dac_out".into()),
            );
            record.supply = Some(ConverterSupply::new(
                5.0,
                None,
                ConverterSource::Inferred {
                    source: "V1".into(),
                    level: 5.0,
                },
            ));
            record
        };
        assert_eq!(
            record_text(&[record("a"), record("b")]),
            "Inserted digital-to-analog bridge atmpl3 for nodes a b from bridge template \
             'auto_bridge_dac_out'\nboundary a: template auto_bridge_dac_out, static at 5 V\n\
             boundary b: template auto_bridge_dac_out, static at 5 V"
        );
    }
}
