//! The built-in connect-rule converters' parameters: which ones a boundary
//! accepts, what each resolves to, and where each value came from.
//!
//! One precedence, highest first:
//!
//! 1. the connect statement's override (`connect d2a #(.rhi(10));`);
//! 2. the selected declaration's default, for the full shipped signature;
//! 3. the built-in rule set `rspice_builtin`, for everything else: a legacy
//!    copy of a shipped signature, and an HDL port no connect rule reaches.
//!
//! The supply is the one exception: it comes from the statement, else from the
//! deck (`.param vcc`, the rail that reaches the net, or 3.3 V), and the caller
//! resolves it.
//!
//! Every value is checked here, before any converter exists, and a refusal
//! names the parameter.

use rspice_veriloga::connect::library::BuiltinSignature;

use super::super::XspiceAutoBridgeKind as Kind;
use crate::circuit::{ConverterParameter, ConverterSource as Source, ConverterSupply as Supply};

/// The rule set whose defaults stand where no declaration states one.
pub(in crate::engine::builder) const RULE_SET: &str = crate::circuit::BUILTIN_RULE_SET;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unit {
    Volt,
    Second,
    Ohm,
    /// A fraction of the supply.
    Fraction,
}

impl Unit {
    fn symbol(self) -> &'static str {
        match self {
            Self::Volt => "V",
            Self::Second => "s",
            Self::Ohm => "Ω",
            Self::Fraction => "",
        }
    }
}

/// One resolved parameter.
#[derive(Debug, Clone)]
pub(in crate::engine::builder) struct Parameter {
    pub(in crate::engine::builder) name: &'static str,
    /// Volts, seconds or ohms; for a level that tracks a live supply, its
    /// value at the supply's nominal level.
    pub(in crate::engine::builder) value: f64,
    unit: Unit,
    /// A default level that follows a live supply as a fraction of it.
    pub(in crate::engine::builder) tracks: bool,
    /// A default the converter derives from other parameters (`vx`, `vz`,
    /// `rx`, `rsup`), so it is not stamped; the converter derives it the same
    /// way.
    derived: bool,
    /// The default's rule, as the declaration writes it (`vsup/2`).
    rule: Option<&'static str>,
    pub(in crate::engine::builder) source: Source,
}

impl Parameter {
    /// The parameter as the insertion record keeps it.
    fn record(&self) -> ConverterParameter {
        ConverterParameter::new(
            self.name,
            self.value,
            self.unit.symbol(),
            self.rule,
            self.source.clone(),
            self.tracks,
        )
    }
}

/// Everything one built-in converter is instantiated from.
#[derive(Debug, Clone)]
pub(in crate::engine::builder) struct ConverterLaw {
    pub(in crate::engine::builder) supply: Supply,
    pub(in crate::engine::builder) parameters: Vec<Parameter>,
}

const A2D_NAMES: &[&str] = &[
    "vsup", "vsupmin", "fthi", "ftlo", "vthi", "vtlo", "txdel", "tdrise", "tdfall",
];
const D2A_NAMES: &[&str] = &[
    "vsup", "vsupmin", "vlo", "vhi", "vx", "vz", "rlo", "rhi", "rx", "rz", "rsup", "rpull",
    "rweak", "trise", "tfall", "tdelay",
];
const BIDIR_NAMES: &[&str] = &[
    "vsup", "vsupmin", "fthi", "ftlo", "vthi", "vtlo", "txdel", "tdrise", "tdfall", "vlo", "vhi",
    "vx", "vz", "rlo", "rhi", "rx", "rz", "rsup", "rpull", "rweak", "trise", "tfall", "tdelay",
];
/// The first-shipped full signatures, whose thresholds are volts only.
const ABSOLUTE_A2D_NAMES: &[&str] = &[
    "vsup", "vsupmin", "vthi", "vtlo", "txdel", "tdrise", "tdfall",
];
const ABSOLUTE_BIDIR_NAMES: &[&str] = &[
    "vsup", "vsupmin", "vthi", "vtlo", "txdel", "tdrise", "tdfall", "vlo", "vhi", "vx", "vz",
    "rlo", "rhi", "rx", "rz", "rsup", "rpull", "rweak", "trise", "tfall", "tdelay",
];
/// A delegating module: its name, its signature, and its connect statement's
/// folded parameters.
pub(in crate::engine::builder) type Selected<'a> = (&'a str, BuiltinSignature, &'a [(String, f64)]);

/// Accepted alias spellings, each with the parameters it sets.
type Aliases = &'static [(&'static str, &'static [&'static str])];

const LEGACY_A2D_NAMES: &[&str] = &["vsup", "tdrise", "tdfall"];
const LEGACY_D2A_NAMES: &[&str] = &["vsup", "trise", "tfall"];
const R2E_NAMES: &[&str] = &["rout", "trise", "tfall"];
const E2R_NAMES: &[&str] = &["vdelta", "ttol"];
const RBIDIR_NAMES: &[&str] = &["rout", "trise", "tfall", "vdelta", "ttol"];

/// The parameters a converter of `kind` accepts under `signature`, and the
/// aliases it accepts for them: Cadence's `tr`/`tf`, and `tdelay` for both
/// A/D delays.
fn accepted(kind: Kind, signature: BuiltinSignature) -> (&'static [&'static str], Aliases) {
    const A2D_ALIASES: &[(&str, &[&str])] = &[("tdelay", &["tdrise", "tdfall"])];
    const D2A_ALIASES: &[(&str, &[&str])] = &[("tr", &["trise"]), ("tf", &["tfall"])];
    match (signature, kind) {
        (BuiltinSignature::Legacy, Kind::Adc) => (LEGACY_A2D_NAMES, &[]),
        (BuiltinSignature::Legacy, _) => (LEGACY_D2A_NAMES, &[]),
        (BuiltinSignature::Absolute, Kind::Adc) => (ABSOLUTE_A2D_NAMES, A2D_ALIASES),
        (BuiltinSignature::Absolute, Kind::Bidi) => (ABSOLUTE_BIDIR_NAMES, D2A_ALIASES),
        (_, Kind::Adc) => (A2D_NAMES, A2D_ALIASES),
        (_, Kind::Dac) => (D2A_NAMES, D2A_ALIASES),
        (_, Kind::RealToV) => (R2E_NAMES, D2A_ALIASES),
        (_, Kind::VToReal) => (E2R_NAMES, &[]),
        (_, Kind::RealBidi) => (RBIDIR_NAMES, D2A_ALIASES),
        (_, Kind::Bidi) => (BIDIR_NAMES, D2A_ALIASES),
    }
}

/// The parameters given by a connect statement, aliases resolved.
#[derive(Default)]
struct Given(Vec<(&'static str, f64)>);

impl Given {
    fn get(&self, name: &str) -> Option<f64> {
        self.0
            .iter()
            .find(|(given, _)| *given == name)
            .map(|(_, value)| *value)
    }
}

fn read_statement(
    kind: Kind,
    signature: BuiltinSignature,
    module: &str,
    statement: &[(String, f64)],
) -> Result<Given, String> {
    let (names, aliases) = accepted(kind, signature);
    let mut given = Given::default();
    let mut spelled: Vec<(&'static str, &str)> = Vec::new();
    for (written, value) in statement {
        let lower = written.to_ascii_lowercase();
        if lower == "vsup" {
            continue;
        }
        let targets: Vec<&'static str> = if let Some(name) = names.iter().find(|n| **n == lower) {
            vec![*name]
        } else if let Some((_, targets)) = aliases.iter().find(|(alias, _)| *alias == lower) {
            targets.to_vec()
        } else {
            let mut list: Vec<&str> = names.to_vec();
            list.extend(aliases.iter().map(|(alias, _)| *alias));
            return Err(format!(
                "the connect statement for '{module}' passes parameter '{written}', which the \
                 built-in {module}{} does not take; it takes {}",
                if signature == BuiltinSignature::Legacy {
                    " (its earlier signature)"
                } else {
                    ""
                },
                list.join(", ")
            ));
        };
        for target in targets {
            if let Some((_, first)) = spelled.iter().find(|(name, _)| *name == target) {
                return Err(format!(
                    "the connect statement for '{module}' sets '{target}' twice, as '{first}' \
                     and as '{written}'; give it once"
                ));
            }
            spelled.push((target, written.as_str()));
            given.0.push((target, *value));
        }
    }
    Ok(given)
}

/// The parameters resolved so far.
struct Resolver<'a> {
    given: &'a Given,
    default_source: Source,
    live: bool,
    parameters: Vec<Parameter>,
}

impl Resolver<'_> {
    /// The parameter `name`: the statement's value, else `default`.
    /// `tracks` marks a default that follows a live supply; `derived` one the
    /// converter derives from other parameters itself.
    fn take(
        &mut self,
        name: &'static str,
        unit: Unit,
        default: f64,
        rule: Option<&'static str>,
        tracks: bool,
        derived: bool,
    ) -> f64 {
        let parameter = match self.given.get(name) {
            Some(value) => Parameter {
                name,
                value,
                unit,
                tracks: false,
                derived: false,
                rule: None,
                source: Source::ConnectStatement,
            },
            None => Parameter {
                name,
                value: default,
                unit,
                tracks: tracks && self.live,
                derived,
                rule,
                source: self.default_source.clone(),
            },
        };
        let value = parameter.value;
        self.parameters.push(parameter);
        value
    }
}

fn is_time(name: &str) -> bool {
    matches!(
        name,
        "txdel" | "tdrise" | "tdfall" | "trise" | "tfall" | "tdelay"
    )
}

fn is_resistance(name: &str) -> bool {
    name.starts_with('r')
}

impl ConverterLaw {
    /// Resolve and check every parameter of one converter.
    ///
    /// `selected` is the delegating module's name, signature and statement
    /// parameters; `None` is an HDL port no connect rule reaches, which takes
    /// the rule set's defaults.
    pub(in crate::engine::builder) fn resolve(
        kind: Kind,
        selected: Option<Selected<'_>>,
        supply: Supply,
    ) -> Result<Self, String> {
        let (module, signature, statement) =
            selected.unwrap_or((module_name(kind), BuiltinSignature::Legacy, &[]));
        let given = read_statement(kind, signature, module, statement)?;
        let live = supply.net.is_some();
        let vsup = supply.level;
        if !vsup.is_finite() {
            return Err(format!(
                "the connect statement for '{module}' gives vsup = {vsup}; a supply is a finite \
                 level or a net"
            ));
        }
        let mut resolver = Resolver {
            given: &given,
            default_source: if selected.is_some() && signature != BuiltinSignature::Legacy {
                Source::ModuleDefault
            } else {
                Source::RuleSet(RULE_SET.into())
            },
            live,
            parameters: Vec::new(),
        };
        let refuse = |name: &str, value: f64, why: &str| -> Result<Self, String> {
            Err(format!(
                "the connect statement for '{module}' gives {name} = {value}; {why}"
            ))
        };

        let vsupmin = resolver.take("vsupmin", Unit::Volt, 0.0, None, false, false);
        let mut band = None;
        if matches!(kind, Kind::Adc | Kind::Bidi) {
            let half = Some("vsup/2");
            let (vthi, vtlo) = if signature == BuiltinSignature::Full && selected.is_some() {
                // The thresholds as fractions of the supply, which a live
                // supply carries with it. A threshold in volts beside its
                // fraction would be two answers to one question.
                for (fraction, volts) in [("fthi", "vthi"), ("ftlo", "vtlo")] {
                    if let (Some(value), Some(_)) = (given.get(fraction), given.get(volts)) {
                        return refuse(
                            fraction,
                            value,
                            &format!(
                                "{volts} is given too, and a threshold is either volts or a \
                                 fraction of the supply; give one of them"
                            ),
                        );
                    }
                }
                // A static supply stamps the thresholds in volts, so only a
                // live one needs the fractions themselves.
                let fthi = resolver.take("fthi", Unit::Fraction, 0.5, None, false, !live);
                let ftlo = resolver.take("ftlo", Unit::Fraction, 0.5, None, false, !live);
                for (name, value) in [("fthi", fthi), ("ftlo", ftlo)] {
                    if !(0.0..=1.0).contains(&value) {
                        return refuse(name, value, "a threshold fraction lies in [0, 1]");
                    }
                }
                if ftlo > fthi {
                    return refuse(
                        "ftlo",
                        ftlo,
                        &format!(
                            "it exceeds fthi = {fthi}; the falling threshold may not lie above the rising one"
                        ),
                    );
                }
                let rule =
                    |given: bool, stated: &'static str| Some(if given { stated } else { "vsup/2" });
                let vthi = resolver.take(
                    "vthi",
                    Unit::Volt,
                    fthi * vsup,
                    rule(given.get("fthi").is_some(), "fthi*vsup"),
                    true,
                    false,
                );
                let vtlo = resolver.take(
                    "vtlo",
                    Unit::Volt,
                    ftlo * vsup,
                    rule(given.get("ftlo").is_some(), "ftlo*vsup"),
                    true,
                    false,
                );
                (vthi, vtlo)
            } else {
                (
                    resolver.take("vthi", Unit::Volt, vsup / 2.0, half, true, false),
                    resolver.take("vtlo", Unit::Volt, vsup / 2.0, half, true, false),
                )
            };
            resolver.take("txdel", Unit::Second, f64::INFINITY, None, false, false);
            resolver.take("tdrise", Unit::Second, 0.0, None, false, false);
            resolver.take("tdfall", Unit::Second, 0.0, None, false, false);
            band = Some((vtlo, vthi));
        }
        let mut strengths = None;
        if matches!(kind, Kind::Dac | Kind::Bidi) {
            let high_tracks = given.get("vhi").is_none();
            let vlo = resolver.take("vlo", Unit::Volt, 0.0, None, false, false);
            let vhi = resolver.take("vhi", Unit::Volt, vsup, Some("vsup"), true, false);
            let vx = resolver.take(
                "vx",
                Unit::Volt,
                (vlo + vhi) / 2.0,
                Some("(vlo+vhi)/2"),
                high_tracks,
                true,
            );
            resolver.take(
                "vz",
                Unit::Volt,
                vx,
                Some("vx"),
                high_tracks && given.get("vx").is_none(),
                true,
            );
            let rlo = resolver.take("rlo", Unit::Ohm, 20.0, None, false, false);
            let rhi = resolver.take("rhi", Unit::Ohm, 20.0, None, false, false);
            let strong = rlo.min(rhi);
            resolver.take("rx", Unit::Ohm, strong, Some("min(rlo, rhi)"), false, true);
            let rz = resolver.take("rz", Unit::Ohm, f64::INFINITY, None, false, false);
            let rsup = resolver.take(
                "rsup",
                Unit::Ohm,
                strong / 10.0,
                Some("min(rlo, rhi)/10"),
                false,
                true,
            );
            let rpull = resolver.take("rpull", Unit::Ohm, 10e3, None, false, false);
            let rweak = resolver.take("rweak", Unit::Ohm, 100e3, None, false, false);
            resolver.take("trise", Unit::Second, 0.0, None, false, false);
            resolver.take("tfall", Unit::Second, 0.0, None, false, false);
            resolver.take("tdelay", Unit::Second, 0.0, None, false, false);
            strengths = Some((rlo, rhi, rz, rsup, rpull, rweak));
        }

        for parameter in &resolver.parameters {
            let (name, value) = (parameter.name, parameter.value);
            let infinite_allowed = matches!(name, "txdel" | "rz");
            if value.is_nan() || (value.is_infinite() && !infinite_allowed) {
                return refuse(name, value, "it must be finite");
            }
            if is_time(name) && value < 0.0 {
                return refuse(name, value, "a time or delay must be nonnegative");
            }
            if is_resistance(name) && value <= 0.0 {
                return refuse(name, value, "a resistance must be positive");
            }
        }
        if vsupmin < 0.0 {
            return refuse(
                "vsupmin",
                vsupmin,
                "it must be nonnegative; zero turns it off",
            );
        }
        if !live && vsupmin > vsup {
            return refuse(
                "vsupmin",
                vsupmin,
                &format!(
                    "it exceeds the supply vsup = {vsup} V, so the converter is never powered"
                ),
            );
        }
        if let Some((vtlo, vthi)) = band
            && vtlo > vthi
        {
            return refuse(
                "vtlo",
                vtlo,
                &format!("the falling threshold must not exceed the rising one, vthi = {vthi}"),
            );
        }
        if let Some((rlo, rhi, rz, rsup, rpull, rweak)) = strengths {
            let given_or = |preferred: &'static str, otherwise: &'static str| {
                if given.get(preferred).is_some() {
                    preferred
                } else {
                    otherwise
                }
            };
            let stiffest = if rlo <= rhi { "rlo" } else { "rhi" };
            let softest = if rlo >= rhi { "rlo" } else { "rhi" };
            let broken = if rsup > rlo.min(rhi) {
                Some(given_or("rsup", stiffest))
            } else if rlo.max(rhi) > rpull {
                Some(given_or("rpull", softest))
            } else if rpull > rweak {
                Some(given_or("rweak", "rpull"))
            } else if rweak > rz {
                Some(given_or("rz", "rweak"))
            } else {
                None
            };
            if let Some(name) = broken {
                let value = resolver
                    .parameters
                    .iter()
                    .find(|parameter| parameter.name == name)
                    .map_or(f64::NAN, |parameter| parameter.value);
                return refuse(
                    name,
                    value,
                    "the drive strengths must order rsup <= min(rlo, rhi) and max(rlo, rhi) <= \
                     rpull <= rweak <= rz, or a weaker IEEE 1364 strength would drive stiffer \
                     than a stronger one",
                );
            }
        }

        Ok(Self {
            supply,
            parameters: resolver.parameters,
        })
    }

    /// One parameter's resolved value.
    #[cfg(test)]
    pub(in crate::engine::builder) fn get(&self, name: &str) -> Option<&Parameter> {
        self.parameters
            .iter()
            .find(|parameter| parameter.name == name)
    }

    /// The converter instance's parameters. A level that tracks a live
    /// supply is left to the converter, which reads it as a fraction of the
    /// supply, and so is a default the converter derives itself. An infinite
    /// `txdel` or `rz` is the converter's own default, and an instance
    /// parameter is finite, so it is left to the default.
    pub(in crate::engine::builder) fn model_parameters(&self) -> Vec<(String, f64)> {
        self.parameters
            .iter()
            .filter(|parameter| {
                !parameter.tracks && !parameter.derived && parameter.value.is_finite()
            })
            .map(|parameter| (parameter.name.to_string(), parameter.value))
            .collect()
    }

    /// Every parameter with its value and source, as the insertion record
    /// keeps them.
    pub(in crate::engine::builder) fn record(&self) -> Vec<ConverterParameter> {
        self.parameters.iter().map(Parameter::record).collect()
    }
}

/// Everything one real-valued converter (`r2e`, `e2r`, `rbidir`) is
/// instantiated from. A real boundary carries a number, so there is no supply.
#[derive(Debug, Clone)]
pub(in crate::engine::builder) struct RealConverterLaw {
    parameters: Vec<Parameter>,
}

impl RealConverterLaw {
    /// Resolve and check every parameter of the real converter `module`
    /// delegates to: the statement's value, else the declaration's default.
    pub(in crate::engine::builder) fn resolve(
        kind: Kind,
        module: &str,
        statement: &[(String, f64)],
    ) -> Result<Self, String> {
        let given = read_statement(kind, BuiltinSignature::Full, module, statement)?;
        let mut resolver = Resolver {
            given: &given,
            default_source: Source::ModuleDefault,
            live: false,
            parameters: Vec::new(),
        };
        let drives = matches!(kind, Kind::RealToV | Kind::RealBidi);
        if drives {
            // `rbidir` shares its node with the sources that drive it while
            // it receives, so its drive is never ideal.
            let rout = if kind == Kind::RealToV { 0.0 } else { 20.0 };
            resolver.take("rout", Unit::Ohm, rout, None, false, false);
            resolver.take("trise", Unit::Second, 1e-9, None, false, false);
            resolver.take("tfall", Unit::Second, 1e-9, None, false, false);
        }
        if matches!(kind, Kind::VToReal | Kind::RealBidi) {
            resolver.take("vdelta", Unit::Volt, 0.0, None, false, false);
            resolver.take("ttol", Unit::Second, 0.0, None, false, false);
        }
        for parameter in &resolver.parameters {
            let (name, value) = (parameter.name, parameter.value);
            let why = if !value.is_finite() {
                "it must be finite"
            } else if value < 0.0 {
                "it must be nonnegative"
            } else if name == "rout" && value == 0.0 && kind != Kind::RealToV {
                "the port receives from the node it drives, so the node has other sources and \
                 an ideal drive would short them; give rout a positive resistance"
            } else {
                continue;
            };
            return Err(format!(
                "the connect statement for '{module}' gives {name} = {value}; {why}"
            ));
        }
        Ok(Self {
            parameters: resolver.parameters,
        })
    }

    /// One parameter's resolved value; every name the kind takes resolves.
    pub(in crate::engine::builder) fn value(&self, name: &str) -> f64 {
        self.parameters
            .iter()
            .find(|parameter| parameter.name == name)
            .map_or(f64::NAN, |parameter| parameter.value)
    }

    /// Every parameter with its value and source, as the insertion record
    /// keeps them.
    pub(in crate::engine::builder) fn record(&self) -> Vec<ConverterParameter> {
        self.parameters.iter().map(Parameter::record).collect()
    }
}

/// The shipped module that bridges a boundary of `kind`.
pub(in crate::engine::builder) fn module_name(kind: Kind) -> &'static str {
    match kind {
        Kind::Adc => "a2d",
        Kind::Dac => "d2a",
        _ => "bidir",
    }
}
