//! What an independent source's omitted field asks for.
//!
//! An omitted field asks for the default of the simulator the deck was
//! written for, so the default belongs to that simulator: the deck's
//! `.options rspice_source=` (the program a drawn deck came from) names it,
//! else the dialect (`ngspice` or `xyce`). With neither, each field takes the
//! default of the highest-precedence simulator that has one established, in
//! oracle order: QSPICE, LTspice, Xyce, ngspice.
//!
//! A default is established by a measurement or by the simulator's own
//! documentation. Where a simulator's behaviour contradicts its own
//! documentation the documentation is its default: a defect is not a default.
//! A simulator with neither has no entry, and a named simulator without one
//! falls back to the precedence rule.
//!
//! Sources of each entry:
//! - ngspice: the ngspice manual, §4.1 "Independent Sources for Voltage or
//!   Current" (<https://ngspice.sourceforge.io/docs/ngspice-html-manual/manual.xhtml>,
//!   version 47+; the ngspice-46 manual's §4.1 reads the same), and batch runs
//!   of ngspice-47.
//! - Xyce: the Xyce Reference Guide 7.10, §2.3.10 "Independent Voltage
//!   Source", Transient Specifications, Tables 2-60 to 2-65
//!   (<https://xyce.sandia.gov/documentation-tutorials/>), and batch runs of
//!   Xyce 7.10.
//! - LTspice: batch runs of LTspice 26.1.1. Its help, "V. Voltage Source"
//!   (<https://ltspicehelpmanual.azurewebsites.net/VVoltageSource.htm>), names
//!   no default for any field this table covers, so what it runs is its
//!   default.
//! - QSPICE: batch runs of QSPICE64 (build of 6 Nov 2026); QSPICE publishes
//!   no help online, so what it runs is its default.

use crate::Value;
use crate::config::SpiceDialect;

use super::SourceSpec;

/// A simulator whose source defaults RSpice can take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceSimulator {
    Qspice,
    Ltspice,
    Xyce,
    Ngspice,
}

impl SourceSimulator {
    /// Oracle precedence, highest first.
    pub const PRECEDENCE: [Self; 4] = [Self::Qspice, Self::Ltspice, Self::Xyce, Self::Ngspice];

    /// The simulator's own name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Qspice => "QSPICE",
            Self::Ltspice => "LTspice",
            Self::Xyce => "Xyce",
            Self::Ngspice => "ngspice",
        }
    }
}

/// How a run reads its independent sources: the dialect's evaluation rules
/// (what an authored zero means, which values a card reads) and whose
/// defaults fill the fields a card leaves out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceRules {
    pub dialect: SpiceDialect,
    /// `None` takes each default from the highest-precedence simulator that
    /// has one established.
    pub defaults: Option<SourceSimulator>,
}

impl SourceRules {
    /// The rules for a deck read under `dialect` that says it was written
    /// for `deck_source` (`.options rspice_source=`). The program a deck came
    /// from is the simulator it was written for, so it outranks the dialect.
    #[must_use]
    pub const fn new(dialect: SpiceDialect, deck_source: Option<SourceSimulator>) -> Self {
        let defaults = match (deck_source, dialect) {
            (Some(simulator), _) => Some(simulator),
            (None, SpiceDialect::Ngspice) => Some(SourceSimulator::Ngspice),
            (None, SpiceDialect::Xyce) => Some(SourceSimulator::Xyce),
            (None, SpiceDialect::BestAvailable) => None,
        };
        Self { dialect, defaults }
    }

    /// The default an omitted `field` takes under these rules.
    #[must_use]
    pub fn default_for(self, field: OmittedField) -> DefaultRule {
        self.chosen(field).0
    }

    fn chosen(self, field: OmittedField) -> (DefaultRule, SourceSimulator) {
        self.defaults
            .and_then(|simulator| established(field, simulator).map(|rule| (rule, simulator)))
            .or_else(|| {
                SourceSimulator::PRECEDENCE
                    .into_iter()
                    .find_map(|simulator| {
                        established(field, simulator).map(|rule| (rule, simulator))
                    })
            })
            .expect("every omitted field has an ngspice default")
    }
}

impl From<SpiceDialect> for SourceRules {
    fn from(dialect: SpiceDialect) -> Self {
        Self::new(dialect, None)
    }
}

/// A source field whose omission asks for a default that depends on the
/// analysis or that the simulators state differently. Fields every simulator
/// defaults alike to a constant (`PULSE` TD, `SIN` PHASE, the `SFFM`/`AM`
/// delays and phases, `PWL` TD and R) are filled where the card is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmittedField {
    PulseRise,
    PulseFall,
    /// PW left out with TF: `PULSE(V1 V2 TD TR)` and shorter.
    PulseWidth,
    /// PW left out after both edges: `PULSE(V1 V2 TD TR TF)`.
    PulseWidthAfterEdges,
    PulsePeriod,
    SinFrequency,
    ExpRiseDelay,
    ExpRiseTau,
    ExpFallDelay,
    ExpFallTau,
    SffmCarrier,
    SffmIndex,
    SffmSignal,
    AmModulating,
    AmCarrier,
}

impl OmittedField {
    /// The field's name on the card.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::PulseRise => "TR",
            Self::PulseFall => "TF",
            Self::PulseWidth | Self::PulseWidthAfterEdges => "PW",
            Self::PulsePeriod => "PER",
            Self::SinFrequency => "FREQ",
            Self::ExpRiseDelay => "TD1",
            Self::ExpRiseTau => "TAU1",
            Self::ExpFallDelay => "TD2",
            Self::ExpFallTau => "TAU2",
            Self::SffmCarrier | Self::AmCarrier => "FC",
            Self::SffmIndex => "MDI",
            Self::SffmSignal => "FS",
            Self::AmModulating => "FM",
        }
    }

    const fn unit(self) -> &'static str {
        match self {
            Self::SinFrequency
            | Self::SffmCarrier
            | Self::SffmSignal
            | Self::AmModulating
            | Self::AmCarrier => "Hz",
            Self::SffmIndex => "",
            _ => "s",
        }
    }
}

/// A default as a simulator states it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DefaultRule {
    Value(Value),
    Tstep,
    Tstop,
    /// So many cycles over the stop time: `cycles / TSTOP`.
    CyclesOverTstop(Value),
    /// `TD1 + TSTEP`, the `EXP` fall delay.
    RiseDelayPlusTstep,
    /// `TD1`: the `EXP` fall starts with the rise.
    RiseDelay,
    /// The `EXP` fall never comes: an infinite TD2 or TAU2.
    NoFall,
}

impl DefaultRule {
    /// The value under a transient of `tstep` and `tstop`; `rise_delay` is
    /// the resolved `EXP` TD1.
    #[must_use]
    pub fn value(self, tstep: Value, tstop: Value, rise_delay: Value) -> Value {
        match self {
            Self::Value(value) => value,
            Self::Tstep => tstep,
            Self::Tstop => tstop,
            Self::CyclesOverTstop(cycles) => cycles / tstop,
            Self::RiseDelayPlusTstep => rise_delay + tstep,
            Self::RiseDelay => rise_delay,
            Self::NoFall => Value::INFINITY,
        }
    }

    fn describe(self) -> String {
        match self {
            Self::Value(value) => format!("{value}"),
            Self::Tstep => "TSTEP".to_owned(),
            Self::Tstop => "TSTOP".to_owned(),
            Self::CyclesOverTstop(cycles) => format!("{cycles}/TSTOP"),
            Self::RiseDelayPlusTstep => "TD1 + TSTEP".to_owned(),
            Self::RiseDelay => "TD1".to_owned(),
            Self::NoFall => "no fall".to_owned(),
        }
    }

    /// Whether the value moves with the analysis, so a note can state it.
    const fn follows_analysis(self) -> bool {
        !matches!(self, Self::Value(_) | Self::RiseDelay | Self::NoFall)
    }
}

/// The table: each simulator's established default for an omitted field.
const fn established(field: OmittedField, simulator: SourceSimulator) -> Option<DefaultRule> {
    use DefaultRule::{
        CyclesOverTstop, NoFall, RiseDelay, RiseDelayPlusTstep, Tstep, Tstop, Value,
    };
    use OmittedField as F;
    use SourceSimulator::{Ltspice, Ngspice, Qspice, Xyce};
    match (field, simulator) {
        // PULSE. ngspice manual §4.1.1: TR and TF TSTEP, PW and PER TSTOP;
        // ngspice-47 runs the same except that it runs `PULSE(V1 V2 TD TR
        // TF)` as one triangle, PW 0 (vsrcload.c, "if PW not given, but TR and
        // TF --> PW = 0"), against that table, so the manual's TSTOP is
        // ngspice's default there too. ngspice-46 resolved an omitted PER to
        // TR + TF + PW, also against its manual. Xyce RG Table 2-60 and Xyce
        // 7.10 runs: TR and TF Δt (the time step), PW and PER T_F (the end
        // time). LTspice 26 refuses a PULSE of fewer than four values, so it
        // has no TR default, and a fall it cannot run shows no TF default; it
        // holds V2 to the end when PW is left out with TF, runs `PULSE(V1 V2
        // TD TR TF)` as one triangle (PW 0), and fires once when PER is left
        // out. QSPICE holds V2 to the end when PW is left out, after both
        // edges too, and fires once when PER is left out. It has no TR
        // default: it runs `PULSE(V1 V2)` and `PULSE(V1 V2 TD)` alike as an
        // ideal step at 10 ns, dropping the authored TD, and a defect is not a
        // default. No simulator can show a TF default, since a card that leaves
        // TF out leaves PW out too and holds V2.
        (F::PulseRise | F::PulseFall, Xyce | Ngspice) => Some(Tstep),
        (F::PulseWidth | F::PulsePeriod, Qspice | Ltspice | Xyce | Ngspice) => Some(Tstop),
        (F::PulseWidthAfterEdges, Qspice | Xyce | Ngspice) => Some(Tstop),
        (F::PulseWidthAfterEdges, Ltspice) => Some(Value(0.0)),
        // SIN. ngspice manual §4.1.2 and ngspice-47: FREQ 1/TSTOP. Xyce RG
        // Table 2-61 asks for FREQ and Xyce 7.10 refuses a card without it,
        // so Xyce has no default; QSPICE refuses a SIN of fewer than three
        // values too. LTspice 26 runs `SIN(0 1)` at 0 Hz, a constant VO.
        (F::SinFrequency, Ngspice) => Some(CyclesOverTstop(1.0)),
        (F::SinFrequency, Ltspice) => Some(Value(0.0)),
        // EXP. ngspice manual §4.1.3, Xyce RG Table 2-62 and Xyce 7.10 runs
        // agree: TD1 0, TAU1 and TAU2 TSTEP, TD2 TD1 + TSTEP. ngspice-47 starts
        // an omitted TD1 at TSTEP instead, against its manual. LTspice 26 runs
        // an omitted TD1, TAU1 or TAU2 as 0, a zero time constant being an
        // ideal step, and starts an omitted TD2's fall at TD1, so `EXP(V1 V2
        // TD1 TAU1)` rises and falls back at TD1, leaving V1 - (V2 - V1) times
        // the rise's decay. (Its fall is not held back to TD1 otherwise:
        // `EXP(0 1 10u 5u 4u 10u)` falls from 4 us.) QSPICE refuses an EXP of
        // fewer than four values, so it has no TD1 or TAU1 default; with TD2
        // or TAU2 left out the fall never comes, and `EXP(V1 V2 TD1 TAU1)`
        // rises and stays at V2.
        (F::ExpRiseDelay, Ltspice | Xyce | Ngspice) => Some(Value(0.0)),
        (F::ExpRiseTau | F::ExpFallTau, Xyce | Ngspice) => Some(Tstep),
        (F::ExpFallDelay, Xyce | Ngspice) => Some(RiseDelayPlusTstep),
        (F::ExpRiseTau | F::ExpFallTau, Ltspice) => Some(Value(0.0)),
        (F::ExpFallDelay, Ltspice) => Some(RiseDelay),
        (F::ExpFallDelay | F::ExpFallTau, Qspice) => Some(NoFall),
        // SFFM. QSPICE64 (recorded with commit bc326808c8, pinned by
        // `sffm_defaults_follow_qspice`, and measured again here): FC and FS
        // one cycle over the stop time, MDI 0. LTspice 26: FC, MDI and FS all 0, so `SFFM(0 1)` is a
        // constant VO and `SFFM(0 1 100k 2)` an unmodulated carrier. Xyce RG
        // Table 2-65 and Xyce 7.10 runs: FC and FS 1/TSTOP, MDI 0. ngspice
        // manual §4.1.5: FC 500/TSTOP, MDI 90, FM 5/TSTOP. ngspice-47 swaps
        // the two frequency defaults (vsrcload.c gives FC 5/TSTOP and FM
        // 500/TSTOP), against its manual.
        (F::SffmCarrier | F::SffmSignal, Qspice | Xyce) => Some(CyclesOverTstop(1.0)),
        (F::SffmCarrier | F::SffmSignal, Ltspice) => Some(Value(0.0)),
        (F::SffmIndex, Qspice | Ltspice | Xyce) => Some(Value(0.0)),
        (F::SffmCarrier, Ngspice) => Some(CyclesOverTstop(500.0)),
        (F::SffmIndex, Ngspice) => Some(Value(90.0)),
        (F::SffmSignal, Ngspice) => Some(CyclesOverTstop(5.0)),
        // AM is ngspice's alone (LTspice 26 and Xyce 7.10 refuse the card).
        // ngspice manual §4.1.6 and ngspice-47: FM 5/TSTOP, FC 500/TSTOP.
        (F::AmModulating, Ngspice) => Some(CyclesOverTstop(5.0)),
        (F::AmCarrier, Ngspice) => Some(CyclesOverTstop(500.0)),
        _ => None,
    }
}

/// The fields `spec` leaves for the analysis to fill, in card order.
#[must_use]
pub fn omitted_fields(spec: &SourceSpec) -> Vec<OmittedField> {
    let mut fields = Vec::new();
    let mut omitted = |value: Value, field: OmittedField| {
        if value.is_nan() {
            fields.push(field);
        }
    };
    match spec {
        SourceSpec::Distortion { inner, .. }
        | SourceSpec::RfPort { inner, .. }
        | SourceSpec::DcTransient {
            transient: inner, ..
        }
        | SourceSpec::AcTransient {
            transient: inner, ..
        }
        | SourceSpec::DcAcTransient {
            transient: inner, ..
        } => return omitted_fields(inner),
        SourceSpec::Pulse {
            rise,
            fall,
            width,
            period,
            ..
        } => {
            omitted(*rise, OmittedField::PulseRise);
            omitted(*fall, OmittedField::PulseFall);
            omitted(
                *width,
                if fall.is_nan() {
                    OmittedField::PulseWidth
                } else {
                    OmittedField::PulseWidthAfterEdges
                },
            );
            omitted(*period, OmittedField::PulsePeriod);
        }
        SourceSpec::Sin { frequency, .. } => omitted(*frequency, OmittedField::SinFrequency),
        SourceSpec::Exp {
            td1,
            tau1,
            td2,
            tau2,
            ..
        } => {
            omitted(*td1, OmittedField::ExpRiseDelay);
            omitted(*tau1, OmittedField::ExpRiseTau);
            omitted(*td2, OmittedField::ExpFallDelay);
            omitted(*tau2, OmittedField::ExpFallTau);
        }
        SourceSpec::Sffm {
            carrier_freq,
            modulation_index,
            signal_freq,
            ..
        } => {
            omitted(*carrier_freq, OmittedField::SffmCarrier);
            omitted(*modulation_index, OmittedField::SffmIndex);
            omitted(*signal_freq, OmittedField::SffmSignal);
        }
        SourceSpec::Am {
            modulating_freq,
            carrier_freq,
            ..
        } => {
            omitted(*modulating_freq, OmittedField::AmModulating);
            omitted(*carrier_freq, OmittedField::AmCarrier);
        }
        _ => {}
    }
    fields
}

/// The one note a source earns when a field it leaves out takes a default
/// the simulators disagree on, naming the value used and who uses it, or
/// `None` when they all agree. `analysis` is the transient's `(TSTEP,
/// TSTOP)` when the deck has one.
#[must_use]
pub fn omitted_field_note(
    source: &str,
    spec: &SourceSpec,
    rules: SourceRules,
    analysis: Option<(Value, Value)>,
) -> Option<String> {
    let parts = omitted_fields(spec)
        .into_iter()
        .filter_map(|field| disagreement(field, rules, analysis))
        .collect::<Vec<_>>();
    if parts.is_empty() {
        return None;
    }
    Some(format!("{source} {}: {}", kind(spec)?, parts.join("; ")))
}

/// The notes a parsed deck's sources earn, one per authored card at most,
/// resolved against the deck's first transient.
pub(crate) fn omitted_source_field_notes(
    netlist: &super::Netlist,
    dialect: SpiceDialect,
) -> Vec<super::ParseDiagnostic> {
    let rules = SourceRules::new(dialect, netlist.options.source_simulator);
    let analysis = netlist.analyses.iter().find_map(|analysis| match analysis {
        super::AnalysisCommand::Tran { step, stop, .. } if *step > 0.0 && *stop > 0.0 => {
            Some((*step, *stop))
        }
        _ => None,
    });
    let top = netlist.elements.iter().map(|element| (None, element));
    let nested = netlist.subcircuits.iter().flat_map(|subcircuit| {
        subcircuit
            .elements
            .iter()
            .map(move |element| (Some(subcircuit.name.as_str()), element))
    });
    top.chain(nested)
        .filter_map(|(subcircuit, element)| {
            let (super::ElementKind::VoltageSource(spec)
            | super::ElementKind::CurrentSource(spec)) = &element.kind
            else {
                return None;
            };
            let message = omitted_field_note(&element.name, spec, rules, analysis)?;
            let key = super::element_origin_key(subcircuit, &element.name);
            Some(match netlist.element_origins.get(&key) {
                Some(origin) => super::ParseDiagnostic {
                    line: origin.line,
                    origin: Some(origin.clone()),
                    code: "omitted-source-default".to_owned(),
                    message,
                    severity: super::DiagnosticSeverity::Note,
                },
                None => super::ParseDiagnostic {
                    line: 0,
                    origin: None,
                    code: "omitted-source-default".to_owned(),
                    message,
                    severity: super::DiagnosticSeverity::Note,
                },
            })
        })
        .collect()
}

fn kind(spec: &SourceSpec) -> Option<&'static str> {
    Some(match spec {
        SourceSpec::Distortion { inner, .. }
        | SourceSpec::RfPort { inner, .. }
        | SourceSpec::DcTransient {
            transient: inner, ..
        }
        | SourceSpec::AcTransient {
            transient: inner, ..
        }
        | SourceSpec::DcAcTransient {
            transient: inner, ..
        } => return kind(inner),
        SourceSpec::Pulse { .. } => "PULSE",
        SourceSpec::Sin { .. } => "SIN",
        SourceSpec::Exp { .. } => "EXP",
        SourceSpec::Sffm { .. } => "SFFM",
        SourceSpec::Am { .. } => "AM",
        _ => return None,
    })
}

fn disagreement(
    field: OmittedField,
    rules: SourceRules,
    analysis: Option<(Value, Value)>,
) -> Option<String> {
    let (used, _) = rules.chosen(field);
    let mut agreeing = Vec::new();
    let mut others: Vec<(DefaultRule, Vec<&str>)> = Vec::new();
    for simulator in SourceSimulator::PRECEDENCE {
        let Some(rule) = established(field, simulator) else {
            continue;
        };
        if rule == used {
            agreeing.push(simulator.name());
        } else if let Some((_, names)) = others.iter_mut().find(|(other, _)| *other == rule) {
            names.push(simulator.name());
        } else {
            others.push((rule, vec![simulator.name()]));
        }
    }
    if others.is_empty() {
        return None;
    }
    let value = match analysis {
        Some((tstep, tstop)) if used.follows_analysis() => {
            // A note on TD2 states the rule; the value rides on TD1.
            (used != DefaultRule::RiseDelayPlusTstep).then(|| {
                format!(
                    " = {}",
                    engineering(used.value(tstep, tstop, 0.0), field.unit())
                )
            })
        }
        _ => None,
    }
    .unwrap_or_default();
    let others = others
        .into_iter()
        .map(|(rule, names)| format!("{}: {}", names.join(", "), rule.describe()))
        .collect::<Vec<_>>()
        .join("; ");
    Some(format!(
        "{} omitted, using {}{value} ({}; {others})",
        field.label(),
        used.describe(),
        agreeing.join(", ")
    ))
}

/// `value` with an SI prefix, as a note states it: `100us`, `10kHz`.
fn engineering(value: Value, unit: &str) -> String {
    const PREFIXES: [(Value, &str); 9] = [
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
        .into_iter()
        .find(|(scale, _)| magnitude >= *scale * (1.0 - 1e-12))
        .unwrap_or((1.0, ""));
    let scaled = value / scale;
    let digits = format!("{scaled:.4}");
    let digits = digits.trim_end_matches('0').trim_end_matches('.');
    format!("{digits}{prefix}{unit}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pulse(rise: Value, fall: Value, width: Value, period: Value) -> SourceSpec {
        SourceSpec::Pulse {
            v1: 0.0,
            v2: 5.0,
            delay: 1e-6,
            rise,
            fall,
            width,
            period,
            pulse_count: 0.0,
            width_defaults_to_zero: rise.is_finite() && fall.is_finite() && width.is_nan(),
        }
    }

    #[test]
    fn a_named_simulator_takes_its_own_default_and_best_takes_precedence() {
        let ngspice = SourceRules::new(SpiceDialect::Ngspice, None);
        let xyce = SourceRules::new(SpiceDialect::Xyce, None);
        let best = SourceRules::new(SpiceDialect::BestAvailable, None);
        assert_eq!(
            ngspice.default_for(OmittedField::SffmCarrier),
            DefaultRule::CyclesOverTstop(500.0)
        );
        assert_eq!(
            xyce.default_for(OmittedField::SffmCarrier),
            DefaultRule::CyclesOverTstop(1.0)
        );
        // QSPICE outranks Xyce and ngspice.
        assert_eq!(
            best.default_for(OmittedField::SffmIndex),
            DefaultRule::Value(0.0)
        );
        // LTspice 26 runs `SIN(0 1)` at 0 Hz, and outranks ngspice's
        // 1/TSTOP.
        assert_eq!(
            best.default_for(OmittedField::SinFrequency),
            DefaultRule::Value(0.0)
        );
        assert_eq!(
            ngspice.default_for(OmittedField::SinFrequency),
            DefaultRule::CyclesOverTstop(1.0)
        );
        // Xyce asks for FREQ, so a Xyce reading falls back to the rule.
        assert_eq!(
            xyce.default_for(OmittedField::SinFrequency),
            DefaultRule::Value(0.0)
        );
        // ngspice-47 runs `PULSE(V1 V2 TD TR TF)` with PW 0; its manual says
        // TSTOP, and the manual is ngspice's default. LTspice 26 runs the
        // same card with PW 0 too, but QSPICE holds V2, and outranks it.
        for rules in [ngspice, xyce, best] {
            assert_eq!(
                rules.default_for(OmittedField::PulseWidthAfterEdges),
                DefaultRule::Tstop
            );
        }
        // With the fall left out too, every simulator holds V2 to the end.
        for rules in [ngspice, xyce, best] {
            assert_eq!(
                rules.default_for(OmittedField::PulseWidth),
                DefaultRule::Tstop
            );
        }
        // LTspice refuses a PULSE without TR, so the rise comes from Xyce.
        assert_eq!(
            best.default_for(OmittedField::PulseRise),
            DefaultRule::Tstep
        );
        // QSPICE never falls when TD2 is left out, LTspice 26 falls at TD1,
        // Xyce and ngspice at TD1 + TSTEP.
        assert_eq!(
            best.default_for(OmittedField::ExpFallDelay),
            DefaultRule::NoFall
        );
        assert_eq!(
            SourceRules::new(SpiceDialect::BestAvailable, Some(SourceSimulator::Ltspice))
                .default_for(OmittedField::ExpFallDelay),
            DefaultRule::RiseDelay
        );
        // QSPICE refuses an EXP without TD1, so TD1 comes from LTspice: 0.
        assert_eq!(
            best.default_for(OmittedField::ExpRiseDelay),
            DefaultRule::Value(0.0)
        );
        assert_eq!(
            ngspice.default_for(OmittedField::ExpFallDelay),
            DefaultRule::RiseDelayPlusTstep
        );
    }

    #[test]
    fn the_program_a_deck_came_from_outranks_the_dialect() {
        let ltspice = SourceRules::new(SpiceDialect::Ngspice, Some(SourceSimulator::Ltspice));
        assert_eq!(ltspice.defaults, Some(SourceSimulator::Ltspice));
        // LTspice runs SFFM unmodulated by default.
        assert_eq!(
            ltspice.default_for(OmittedField::SffmIndex),
            DefaultRule::Value(0.0)
        );
        // LTspice 26 runs an omitted carrier at 0 Hz, though QSPICE outranks
        // it with one cycle over the stop time.
        assert_eq!(
            ltspice.default_for(OmittedField::SffmCarrier),
            DefaultRule::Value(0.0)
        );
        // LTspice refuses a PULSE without TR: the precedence rule fills it.
        assert_eq!(
            ltspice.default_for(OmittedField::PulseRise),
            DefaultRule::Tstep
        );
    }

    #[test]
    fn a_default_every_simulator_shares_earns_no_note() {
        let best = SourceRules::new(SpiceDialect::BestAvailable, None);
        for spec in [
            pulse(Value::NAN, Value::NAN, Value::NAN, Value::NAN),
            pulse(1e-9, Value::NAN, Value::NAN, Value::NAN),
            pulse(1e-9, 1e-9, 1e-9, Value::NAN),
        ] {
            assert_eq!(
                omitted_field_note("V1", &spec, best, Some((1e-7, 1e-4))),
                None
            );
        }
    }

    #[test]
    fn a_width_left_out_after_both_edges_earns_a_note() {
        let spec = pulse(1e-9, 1e-9, Value::NAN, Value::NAN);
        assert_eq!(
            omitted_fields(&spec),
            [
                OmittedField::PulseWidthAfterEdges,
                OmittedField::PulsePeriod
            ]
        );
        assert_eq!(
            omitted_field_note(
                "V1",
                &spec,
                SourceRules::new(SpiceDialect::BestAvailable, None),
                Some((1e-7, 1e-4))
            )
            .as_deref(),
            Some("V1 PULSE: PW omitted, using TSTOP = 100us (QSPICE, Xyce, ngspice; LTspice: 0)")
        );
        assert_eq!(
            omitted_field_note(
                "V1",
                &spec,
                SourceRules::new(SpiceDialect::Ngspice, None),
                Some((1e-7, 1e-4))
            )
            .as_deref(),
            Some("V1 PULSE: PW omitted, using TSTOP = 100us (QSPICE, Xyce, ngspice; LTspice: 0)")
        );
        assert_eq!(
            omitted_field_note(
                "V1",
                &spec,
                SourceRules::new(SpiceDialect::BestAvailable, Some(SourceSimulator::Ltspice)),
                Some((1e-7, 1e-4))
            )
            .as_deref(),
            Some("V1 PULSE: PW omitted, using 0 (LTspice; QSPICE, Xyce, ngspice: TSTOP)")
        );
    }

    /// `EXP(V1 V2 TD1 TAU1)`: QSPICE never falls, LTspice 26 starts the
    /// omitted fall at TD1 as an ideal step, Xyce and ngspice at TD1 + TSTEP
    /// with a TSTEP time constant. A TD2 note names the rule, not a value,
    /// since it rides on TD1.
    #[test]
    fn an_exp_with_its_fall_left_out_earns_a_note() {
        let spec = SourceSpec::Exp {
            v1: 0.0,
            v2: 1.0,
            td1: 1e-6,
            tau1: 1e-6,
            td2: Value::NAN,
            tau2: Value::NAN,
        };
        assert_eq!(
            omitted_field_note(
                "V3",
                &spec,
                SourceRules::new(SpiceDialect::BestAvailable, None),
                Some((1e-7, 1e-4))
            )
            .as_deref(),
            Some(
                "V3 EXP: TD2 omitted, using no fall (QSPICE; LTspice: TD1; Xyce, ngspice: \
                 TD1 + TSTEP); TAU2 omitted, using no fall (QSPICE; LTspice: 0; Xyce, \
                 ngspice: TSTEP)"
            )
        );
        assert_eq!(
            omitted_field_note(
                "V3",
                &spec,
                SourceRules::new(SpiceDialect::Xyce, None),
                Some((1e-7, 1e-4))
            )
            .as_deref(),
            Some(
                "V3 EXP: TD2 omitted, using TD1 + TSTEP (Xyce, ngspice; QSPICE: no fall; \
                 LTspice: TD1); TAU2 omitted, using TSTEP = 100ns (Xyce, ngspice; QSPICE: \
                 no fall; LTspice: 0)"
            )
        );
    }

    #[test]
    fn a_disputed_default_earns_one_note_naming_the_value() {
        let best = SourceRules::new(SpiceDialect::BestAvailable, None);
        let sffm = SourceSpec::Sffm {
            offset: 0.0,
            amplitude: 1.0,
            carrier_freq: Value::NAN,
            modulation_index: Value::NAN,
            signal_freq: 1e3,
            delay: 0.0,
            phase_modulation: 0.0,
            phase_carrier: 0.0,
        };
        assert_eq!(
            omitted_field_note("V2", &sffm, best, Some((1e-7, 1e-4))).as_deref(),
            Some(
                "V2 SFFM: FC omitted, using 1/TSTOP = 10kHz (QSPICE, Xyce; LTspice: 0; \
                 ngspice: 500/TSTOP); MDI omitted, using 0 (QSPICE, LTspice, Xyce; ngspice: 90)"
            )
        );
        let ngspice = SourceRules::new(SpiceDialect::Ngspice, None);
        assert_eq!(
            omitted_field_note("V2", &sffm, ngspice, None).as_deref(),
            Some(
                "V2 SFFM: FC omitted, using 500/TSTOP (ngspice; QSPICE, Xyce: 1/TSTOP; \
                 LTspice: 0); MDI omitted, using 90 (ngspice; QSPICE, LTspice, Xyce: 0)"
            )
        );
    }

    #[test]
    fn engineering_values_read_as_a_note_states_them() {
        assert_eq!(engineering(1e-4, "s"), "100us");
        assert_eq!(engineering(1e4, "Hz"), "10kHz");
        assert_eq!(engineering(5e6, "Hz"), "5MHz");
        assert_eq!(engineering(2.5e-9, "s"), "2.5ns");
    }
}
