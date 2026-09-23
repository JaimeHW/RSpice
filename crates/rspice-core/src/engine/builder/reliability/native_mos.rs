//! Aging coordinates on native bulk BSIM devices. Mobility retains the units
//! inferred from the fresh geometry-binned model even if an aged value crosses
//! the native U0 unit-selection threshold. Every rewritten card is per-instance.

use super::*;
use crate::device::Bsim3v3EquationSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Family {
    Classic,
    Bsim3(Bsim3v3EquationSet),
    Bsim4,
    Unsupported,
}

impl Family {
    pub(super) fn select(model: &ModelDef, dialect: SpiceDialect) -> Self {
        let level = model_param(&model.params, &["LEVEL"]).unwrap_or(1.0);
        let params = upper(model);
        match level {
            1.0 | 2.0 | 3.0 => Self::Classic,
            8.0 => Self::Bsim3(Bsim3v3EquationSet::NgspiceV330),
            49.0 => Self::Bsim3(if dialect == SpiceDialect::Xyce {
                Bsim3v3EquationSet::XyceV322
            } else {
                Bsim3v3EquationSet::NgspiceV330
            }),
            9.0 if level9_selects_bsim3(
                &params,
                &model.expr_params,
                &model.string_params,
                dialect,
            ) =>
            {
                Self::Bsim3(Bsim3v3EquationSet::XyceV322)
            }
            14.0 | 54.0 => Self::Bsim4,
            _ => Self::Unsupported,
        }
    }

    pub(super) fn supports(self, parameter: &str, model: &ModelDef) -> bool {
        match self {
            Self::Classic => matches!(parameter, "VTO" | "KP" | "GAMMA" | "PHI"),
            Self::Bsim3(_) => matches!(parameter, "VTH0" | "U0" | "VSAT" | "RDSW"),
            Self::Bsim4 => match parameter {
                "VTH0" | "U0" | "VSAT" => true,
                "RDSW" | "RDSWMIN" => model_param(&model.params, &["RDSMOD"]).unwrap_or(0.0) == 0.0,
                "RDW" | "RSW" | "RDWMIN" | "RSWMIN" => {
                    model_param(&model.params, &["RDSMOD"]).unwrap_or(0.0) == 1.0
                }
                _ => false,
            },
            Self::Unsupported => false,
        }
    }
}

fn upper(model: &ModelDef) -> HashMap<String, f64> {
    model
        .params
        .iter()
        .map(|(k, v)| (k.to_ascii_uppercase(), *v))
        .collect()
}

pub(super) struct NativeContext {
    inverse: [f64; 3],
    fresh_mobility_scale: f64,
}

pub(super) fn resolve(
    family: Family,
    model: &ModelDef,
    element: &Element,
    netlist: &Netlist,
    temperature: f64,
) -> Result<Option<NativeContext>, SimulationError> {
    let ElementKind::Mosfet {
        instance_params, ..
    } = &element.kind
    else {
        return Ok(None);
    };
    let params = upper(model);
    let pmos = model.model_type.eq_ignore_ascii_case("PMOS");
    let nominal = crate::constants::celsius_to_kelvin(netlist.options.tnom.unwrap_or(27.0));
    let (leff, weff, bin_unit, u0) = match family {
        Family::Bsim3(equations) => {
            use crate::device::mosfet::bsim3v3::{
                Bsim3v3Geometry, Bsim3v3Model, Bsim3v3ModelTemp, Bsim3v3SizeDep,
            };
            let params = native_bsim3_model_params_upper_map(
                &element.name,
                &model.name,
                &params,
                &model.expr_params,
                &model.string_params,
            )?;
            let m =
                Bsim3v3Model::try_from_params_with_equation_set(&params, pmos, nominal, equations)
                    .map_err(refusal)?;
            let defaults = Bsim3v3Geometry::default();
            let xyce = equations == Bsim3v3EquationSet::XyceV322;
            let l = instance_param(instance_params, &["L"])
                .or(if xyce {
                    m.instance_length_default
                } else {
                    None
                })
                .unwrap_or(defaults.l);
            let w = instance_param(instance_params, &["W"])
                .or(if xyce { m.instance_width_default } else { None })
                .unwrap_or(defaults.w);
            let size = Bsim3v3SizeDep::new(&m, &Bsim3v3ModelTemp::new(&m, temperature), l, w)
                .map_err(refusal)?;
            (
                size.leff,
                size.weff,
                m.bin_unit,
                [m.u0.v, m.u0.l, m.u0.w, m.u0.p],
            )
        }
        Family::Bsim4 => {
            use crate::device::mosfet::bsim4v8::{
                Bsim4v8Geometry, Bsim4v8Model, Bsim4v8ModelTemp, Bsim4v8SizeDep,
            };
            let params = native_bsim4_model_params_upper_map(
                &element.name,
                &model.name,
                &params,
                &model.expr_params,
                &model.string_params,
            )?;
            let m = Bsim4v8Model::try_from_params(&params, pmos, nominal).map_err(refusal)?;
            let defaults = Bsim4v8Geometry::default();
            let l = instance_param(instance_params, &["L"]).unwrap_or(defaults.l);
            let w = instance_param(instance_params, &["W"]).unwrap_or(defaults.w);
            let nf = instance_param(instance_params, &["NF"])
                .filter(|v| v.is_finite() && *v >= 1.0)
                .unwrap_or(defaults.nf);
            let size = Bsim4v8SizeDep::new(&m, &Bsim4v8ModelTemp::new(&m, temperature), l, w, nf)
                .map_err(refusal)?;
            (
                size.leff,
                size.weff,
                m.bin_unit,
                [m.u0.v, m.u0.l, m.u0.w, m.u0.p],
            )
        }
        _ => return Ok(None),
    };
    let inverse = if bin_unit == 1 {
        [1e-6 / leff, 1e-6 / weff, 1e-12 / (leff * weff)]
    } else {
        [1.0 / leff, 1.0 / weff, 1.0 / (leff * weff)]
    };
    let raw = evaluate(u0, inverse);
    if !raw.is_finite() || raw <= 0.0 || inverse.iter().any(|v| !v.is_finite()) {
        return Err(refusal(
            "Fresh BSIM mobility or geometry is outside its native domain",
        ));
    }
    Ok(Some(NativeContext {
        inverse,
        fresh_mobility_scale: if raw > 1.0 { 1e-4 } else { 1.0 },
    }))
}

fn evaluate(values: [f64; 4], inverse: [f64; 3]) -> f64 {
    values[0] + values[1] * inverse[0] + values[2] * inverse[1] + values[3] * inverse[2]
}

fn coefficients(model: &ModelDef, key: &str) -> [f64; 4] {
    [String::new(), "L".into(), "W".into(), "P".into()]
        .map(|prefix| model_param(&model.params, &[&format!("{prefix}{key}")]).unwrap_or(0.0))
}

pub(super) fn apply(
    context: &NativeContext,
    model: &mut ModelDef,
    changed: &BTreeMap<String, f64>,
) -> Result<(), SimulationError> {
    for key in changed.keys() {
        let value = if matches!(key.as_str(), "RDSWMIN" | "RDWMIN" | "RSWMIN") {
            // These BSIM4 minima are scalar, unlike RDSW/RDW/RSW.
            model_param(&model.params, &[key]).unwrap_or(0.0)
        } else {
            evaluate(coefficients(model, key), context.inverse)
        };
        if !value.is_finite()
            || (matches!(key.as_str(), "U0" | "VSAT") && value <= 0.0)
            || (matches!(
                key.as_str(),
                "RDSW" | "RDW" | "RSW" | "RDSWMIN" | "RDWMIN" | "RSWMIN"
            ) && value < 0.0)
        {
            return Err(refusal(format!(
                "Aged geometry-binned {key}={value} is outside its native domain"
            )));
        }
    }
    if changed.contains_key("U0") {
        let values = coefficients(model, "U0");
        let physical = evaluate(values, context.inverse) * context.fresh_mobility_scale;
        if !physical.is_finite() || physical <= 0.0 {
            return Err(refusal("Aged mobility is not representable"));
        }
        // Keep the encoded value well away from the native threshold of 1.
        let destination_scale = if physical >= 0.5 { 1e-4 } else { 1.0 };
        let factor = context.fresh_mobility_scale / destination_scale;
        for (key, value) in ["U0", "LU0", "WU0", "PU0"].into_iter().zip(values) {
            let encoded = value * factor;
            if !encoded.is_finite() {
                return Err(refusal("Aged mobility coefficients overflow"));
            }
            model
                .params
                .retain(|(name, _)| !name.eq_ignore_ascii_case(key));
            model
                .expr_params
                .retain(|(name, _)| !name.eq_ignore_ascii_case(key));
            model.params.push((key.into(), encoded));
        }
        let encoded = evaluate(coefficients(model, "U0"), context.inverse);
        let actual = encoded * if encoded > 1.0 { 1e-4 } else { 1.0 };
        if (actual - physical).abs() > 64.0 * f64::EPSILON * physical.abs() {
            return Err(refusal(
                "Aged mobility cannot retain the fresh unit convention at this geometry",
            ));
        }
    }
    Ok(())
}
