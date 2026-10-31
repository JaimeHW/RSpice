//! `.options scale`: the unit the drawn geometry on an instance line is in.
//!
//! The factor multiplies the lengths an instance line gives and enters the
//! areas squared, before binning or any device reads them, the way ngspice's
//! parameter setters (`mos1par.c`, `b4par.c`, ...) and Xyce's `setParam` do.
//! Model-card values (`LD`, `DEFW`, a Xyce card's own `L`/`W`) are never
//! scaled. Which elements take it:
//! - MOS1-3, BSIM3, BSIM4, B4SOI and HiSIM cards, as in ngspice; its MOS6,
//!   MOS9, BSIM1/2, BSIM3-SOI and VDMOS setters never read the factor.
//!   QSPICE agrees on every family it implements.
//! - The `W`/`L` of a resistor or capacitor in the ngspice dialect only
//!   (`resparam.c`, `capparam.c`); QSPICE leaves them alone.
//! - In the Xyce dialect, Xyce's MOS levels 1, 2, 3, 6, 9, 49, 10, 14, 54
//!   and 18, and nothing else.
//!
//! The LEVEL=3 diode derives its area from the factor itself.

use super::*;

const MOS_LENGTHS: &[&str] = &["W", "L", "PS", "PD"];
const MOS_AREAS: &[&str] = &["AS", "AD"];
const PASSIVE_LENGTHS: &[&str] = &["W", "L", "WIDTH", "LENGTH"];

/// Apply `.options scale` to the instance geometry of flattened elements.
///
/// Runs once per flattening, on the flattener's fresh copy, so a geometry is
/// never scaled twice however often a study rebuilds from the same deck.
pub(super) fn scale_instance_geometry(
    elements: &mut [Element],
    netlist: &Netlist,
    dialect: SpiceDialect,
) {
    let Some(scale) = netlist
        .options
        .scale
        .filter(|scale| scale.is_finite() && *scale > 0.0 && *scale != 1.0)
    else {
        return;
    };
    for element in elements {
        match &mut element.kind {
            ElementKind::Mosfet {
                model,
                instance_params,
                ..
            } if mos_card_takes_scale(netlist, model, dialect) => {
                scale_named(instance_params, MOS_LENGTHS, scale);
                scale_named(instance_params, MOS_AREAS, scale * scale);
            }
            ElementKind::Resistor {
                instance_params, ..
            }
            | ElementKind::Capacitor {
                instance_params, ..
            } if dialect == SpiceDialect::Ngspice => {
                scale_named(instance_params, PASSIVE_LENGTHS, scale);
            }
            _ => {}
        }
    }
}

fn scale_named(params: &mut [(String, f64)], names: &[&str], factor: f64) {
    for (name, value) in params {
        if names
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
        {
            *value *= factor;
        }
    }
}

/// Whether the family the instance's card selects reads scaled geometry.
///
/// Every bin of a binned family shares one LEVEL, so the first bin answers
/// for the family before the scaled geometry picks the bin.
fn mos_card_takes_scale(netlist: &Netlist, model: &str, dialect: SpiceDialect) -> bool {
    let prefix_len = model.len() + 1;
    let card = find_model_def(netlist, model).or_else(|| {
        netlist.models.iter().find(|card| {
            card.name.len() > prefix_len
                && card.name.as_bytes()[model.len()] == b'.'
                && card.name[..model.len()].eq_ignore_ascii_case(model)
        })
    });
    let (model_type, params, expr_params, string_params) = match card {
        Some(card) => (
            card.model_type.as_str(),
            model_params_upper_map(&card.params),
            card.expr_params.as_slice(),
            card.string_params.as_slice(),
        ),
        None => match foundation_model_card(model) {
            Some(card) => (
                card.model_type.as_str(),
                card.params.clone(),
                &[][..],
                &[][..],
            ),
            None => (model, HashMap::new(), &[][..], &[][..]),
        },
    };
    if is_vdmos_model_type(model_type) {
        return false;
    }
    let level = params.get("LEVEL").copied().unwrap_or(1.0);
    if level.fract() != 0.0 {
        return false;
    }
    let level = level as i64;
    if dialect == SpiceDialect::Xyce {
        return matches!(level, 1..=3 | 6 | 9 | 49 | 10 | 14 | 54 | 18);
    }
    matches!(level, 0..=3 | 8 | 49 | 14 | 54 | 10 | 58 | 68 | 73)
        || level == 9 && level9_selects_bsim3(&params, expr_params, string_params, dialect)
}

#[cfg(test)]
mod tests {
    use crate::config::SpiceDialect;
    use crate::engine::{Engine, SimulationConfig};
    use crate::netlist::Netlist;

    fn op_voltages(deck: &str, dialect: SpiceDialect, nodes: &[&str]) -> Vec<f64> {
        let netlist = Netlist::parse(deck).expect("deck parses");
        let result = Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
            .run_dc_op(&netlist)
            .expect("operating point solves");
        nodes
            .iter()
            .map(|node| {
                result
                    .try_voltage_named(node)
                    .unwrap_or_else(|| panic!("missing voltage for node {node}"))
            })
            .collect()
    }

    /// `m1` conducts through its channel; `m2` is off and its forward-biased
    /// drain junction carries `JS*AD`.
    fn mos_pair(scale: &str, channel: &str, junction: &str, level: u32) -> String {
        format!(
            "scaled geometry
{scale}
vdd vdd 0 5
vg g 0 2
vb b 0 0.65
             m1 d g 0 0 nm {channel}
rd vdd d 10k
             m2 e 0 0 b nm {channel} {junction}
re e 0 1k
             .model nm nmos level={level} vto=1 kp=50u ld=0.3u js=1e-2
.end
"
        )
    }

    /// ngspice-46 and QSPICE agree on the drawn deck and the one in metres
    /// (level 1: V(d) 3.214286 V, V(e) 0.192471 V): `W`/`L` scale linearly and
    /// `AD` squared.
    #[test]
    fn options_scale_multiplies_drawn_mos_geometry() {
        for dialect in [
            SpiceDialect::BestAvailable,
            SpiceDialect::Ngspice,
            SpiceDialect::Xyce,
        ] {
            for level in [1, 2, 3] {
                let drawn = op_voltages(
                    &mos_pair(".options scale=1e-6", "w=10 l=2", "ad=400 as=1", level),
                    dialect,
                    &["d", "e"],
                );
                let metres = op_voltages(
                    &mos_pair("", "w=10u l=2u", "ad=400p as=1p", level),
                    dialect,
                    &["d", "e"],
                );
                for (drawn, metres) in drawn.iter().zip(&metres) {
                    assert!(
                        (drawn - metres).abs() <= 1e-9,
                        "{dialect:?} level {level}: drawn {drawn} V, metres {metres} V"
                    );
                }
            }
        }
    }

    /// ngspice's MOS9 setter never reads the variable, so the drawn numbers
    /// stay in metres (ngspice-46: 3.75 V either way; QSPICE has no MOS9).
    #[test]
    fn mos9_ignores_options_scale_as_ngspice_does() {
        let deck = |scale: &str| mos_pair(scale, "w=10 l=2", "", 9);
        for dialect in [SpiceDialect::BestAvailable, SpiceDialect::Ngspice] {
            let drawn = op_voltages(&deck(".options scale=1e-6"), dialect, &["d"]);
            let unscaled = op_voltages(&deck(""), dialect, &["d"]);
            assert_eq!(drawn, unscaled, "{dialect:?}");
        }
    }

    fn divider(scale: &str, geometry: &str) -> String {
        format!(
            "scaled resistor
{scale}
vdd vdd 0 5
r1 vdd d rm {geometry}
r2 d 0 10k
             .model rm r rsh=100 narrow=0.5u
.end
"
        )
    }

    /// ngspice scales a resistor's drawn `W`/`L` (2.5 V); QSPICE does not
    /// (3.333 V), and outside the ngspice dialect RSpice follows QSPICE.
    #[test]
    fn options_scale_reaches_resistor_geometry_only_in_the_ngspice_dialect() {
        let scaled = divider(".options scale=1e-6", "l=100 w=2");
        let metres = op_voltages(&divider("", "l=100u w=2u"), SpiceDialect::Ngspice, &["d"]);
        let drawn = op_voltages(&scaled, SpiceDialect::Ngspice, &["d"]);
        assert!(
            (drawn[0] - metres[0]).abs() <= 1e-9,
            "{drawn:?} V vs {metres:?} V"
        );
        let unscaled = op_voltages(
            &divider("", "l=100 w=2"),
            SpiceDialect::BestAvailable,
            &["d"],
        );
        let drawn = op_voltages(&scaled, SpiceDialect::BestAvailable, &["d"]);
        assert_eq!(drawn, unscaled);
    }
}
