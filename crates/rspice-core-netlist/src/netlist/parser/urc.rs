//! ngspice's uniform distributed RC line, `Uxxx n1 n2 n3 mname L=len
//! [N=lumps]` with a `URC` model card, read as the lumped ladder ngspice
//! builds from it (`urcsetup.c`): resistors along both halves of the line,
//! each section's shunt a capacitor, or a diode when the card gives `ISPERL`.

use super::*;
use crate::netlist::ElementProvenance;

/// The model type that makes a `U` card a distributed RC line rather than a
/// PSpice digital primitive.
const URC_MODEL_TYPE: &str = "URC";

/// The name a `U` card gives in its model field.
pub(super) fn model_field(line: &str) -> Option<&str> {
    line.split_whitespace().nth(4)
}

/// The name a `.model` card declares when its type is URC.
pub(super) fn is_urc_model_card(card: &str) -> Option<String> {
    let mut words = card.split_whitespace();
    if !words.next()?.eq_ignore_ascii_case(".model") {
        return None;
    }
    let name = words.next()?;
    let model_type = words.next()?.split('(').next()?;
    model_type
        .eq_ignore_ascii_case(URC_MODEL_TYPE)
        .then(|| name.to_ascii_uppercase())
}

fn urc_model<'a>(models: &'a [ModelDef], name: &str) -> Option<&'a ModelDef> {
    models.iter().rev().find(|model| {
        model.name.eq_ignore_ascii_case(name)
            && model.model_type.eq_ignore_ascii_case(URC_MODEL_TYPE)
    })
}

/// Whether a `U` card names a URC model card the deck has already defined.
pub(super) fn names_urc_model(line: &str, models: &[ModelDef]) -> bool {
    model_field(line).is_some_and(|name| urc_model(models, name).is_some())
}

/// The error for a card shaped like a URC line, `Uxxx n1 n2 n3 mname
/// [L=len] [N=lumps]`, whose model no earlier card defines as URC. A line
/// outside any subcircuit waits for a later card; one inside a subcircuit is
/// read where it stands.
pub(super) fn unbound_line(fields: &[String], line_num: usize) -> Option<ParseError> {
    let urc_shaped = fields.len() >= 5
        && fields[5..].iter().all(|field| {
            field.split_once('=').is_some_and(|(key, value)| {
                !value.is_empty()
                    && (key.eq_ignore_ascii_case("L") || key.eq_ignore_ascii_case("N"))
            })
        });
    urc_shaped.then(|| ParseError::Syntax {
        line: line_num,
        message: format!(
            "'{}' reads as a URC line, but no .model card before it defines '{}' as URC; \
             inside a subcircuit, define the URC model before the subcircuit",
            fields[0], fields[4]
        ),
    })
}

/// Read a URC line and write the ladder ngspice builds for it.
pub(super) fn parse_urc(
    stream: &mut TokenStream,
    line_num: usize,
    elements: &mut Vec<Element>,
    models: &mut Vec<ModelDef>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let name = expect_element_name(stream, line_num)?;
    let left = expect_node(stream, line_num)?;
    let right = expect_node(stream, line_num)?;
    let ground = expect_node(stream, line_num)?;
    let model_name = expect_model_name(stream, line_num)?;
    let invalid = |message: String| ParseError::Syntax {
        line: line_num,
        message,
    };

    let mut length = None;
    let mut lumps = None;
    loop {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }
        let key = expect_ident(stream, line_num)?;
        if !stream.consume(&TokenKind::Equals) {
            return Err(invalid(format!("URC line '{name}': expected {key}=value")));
        }
        let Some(DeferrableValue::Resolved(value)) = take_deferrable_value(stream, params, false)
        else {
            return Err(invalid(format!(
                "URC line '{name}': {key} must be a number"
            )));
        };
        match key.to_ascii_uppercase().as_str() {
            "L" => length = Some(value),
            "N" => lumps = Some(value),
            _ => {
                return Err(invalid(format!(
                    "URC line '{name}' has no instance parameter '{key}'; it takes L and N"
                )));
            }
        }
    }

    let model = urc_model(models, &model_name).ok_or_else(|| {
        invalid(format!(
            "URC line '{name}' names no URC model '{model_name}'"
        ))
    })?;
    let card = |key: &str| -> Result<Option<f64>, ParseError> {
        if model
            .expr_params
            .iter()
            .any(|(param, _)| param.eq_ignore_ascii_case(key))
        {
            return Err(invalid(format!(
                "URC model '{model_name}' parameter {key} must be a number"
            )));
        }
        Ok(model
            .params
            .iter()
            .rev()
            .find(|(param, _)| param.eq_ignore_ascii_case(key))
            .map(|(_, value)| *value))
    };
    // ngspice's defaults (`urcsetup.c`).
    let k = card("K")?.unwrap_or(1.5);
    let fmax = card("FMAX")?.unwrap_or(1e9);
    let rperl = card("RPERL")?.unwrap_or(1000.0);
    let cperl = card("CPERL")?.unwrap_or(1e-12);
    let isperl = card("ISPERL")?;
    let rsperl = card("RSPERL")?.unwrap_or(0.0);

    let positive = |value: f64| value.is_finite() && value > 0.0;
    let nonnegative = |value: f64| value.is_finite() && value >= 0.0;
    let length = length
        .filter(|length| positive(*length))
        .ok_or_else(|| invalid(format!("URC line '{name}' needs a positive length L")))?;
    for (key, valid) in [
        ("K", k.is_finite() && k > 1.0),
        ("FMAX", positive(fmax)),
        ("RPERL", positive(rperl)),
        ("CPERL", nonnegative(cperl)),
        ("ISPERL", isperl.is_none_or(nonnegative)),
        ("RSPERL", nonnegative(rsperl)),
    ] {
        if !valid {
            return Err(invalid(format!(
                "URC model '{model_name}' has an invalid {key}; K must exceed 1, FMAX and \
                 RPERL must be positive, CPERL, ISPERL and RSPERL nonnegative"
            )));
        }
    }

    let r0 = length * rperl;
    let c0 = length * cperl;
    let lumps = match lumps {
        Some(lumps) if lumps >= 1.0 && lumps.fract() == 0.0 && lumps <= f64::from(i32::MAX) => {
            lumps as i32
        }
        Some(_) => {
            return Err(invalid(format!(
                "URC line '{name}' needs a whole number of lumps N of at least 1"
            )));
        }
        None => {
            let wnorm = fmax * r0 * c0 * 2.0 * std::f64::consts::PI;
            if wnorm < 35.0 {
                3
            } else {
                let ratio = (k - 1.0) / k;
                ((wnorm * ratio * ratio).ln() / k.ln()).max(3.0) as i32
            }
        }
    };

    let r1 = r0 * (k - 1.0) / (2.0 * k.powi(lumps) - 2.0);
    let shunt_denominator = k.powi(lumps - 1) * (k + 1.0) - 2.0;
    let c1 = c0 * (k - 1.0) / shunt_denominator;
    let tag = name.to_ascii_uppercase();
    let diode_model = isperl.map(|isperl| {
        let diode_model = format!("{tag}#URC{}", models.len());
        models.push(ModelDef {
            name: diode_model.clone(),
            model_type: "D".to_owned(),
            params: vec![
                ("CJO".to_owned(), c1),
                ("RS".to_owned(), length * f64::from(lumps) * rsperl),
                (
                    "IS".to_owned(),
                    length * isperl * (k - 1.0) / shunt_denominator,
                ),
            ],
            expr_params: Vec::new(),
            string_params: Vec::new(),
            string_vector_params: Vec::new(),
            real_vector_params: Vec::new(),
            real_vector_expr_params: Vec::new(),
            integer_vector_params: Vec::new(),
        });
        diode_model
    });

    let element = |name: String, kind: ElementKind, nodes: Vec<String>| Element {
        name,
        kind,
        nodes,
        provenance: ElementProvenance::Authored,
    };
    let resistor = |value: f64| ElementKind::Resistor {
        value,
        value_expr: None,
        model: None,
        instance_params: Vec::new(),
        deferred_params: Vec::new(),
    };
    let shunt = |side: &str, index: i32, node: &str, scale: f64| match &diode_model {
        Some(model) => element(
            format!("D{tag}#{side}{index}"),
            ElementKind::Diode {
                model: model.clone(),
                instance_params: vec![("AREA".to_owned(), scale)],
                deferred_params: Vec::new(),
            },
            vec![node.to_owned(), ground.clone()],
        ),
        None => element(
            format!("C{tag}#{side}{index}"),
            ElementKind::Capacitor {
                value: scale * c1,
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params: Vec::new(),
                deferred_params: Vec::new(),
            },
            vec![node.to_owned(), ground.clone()],
        ),
    };

    // Sections grow by K from both ends toward the middle node, where the
    // two halves meet.
    let mut low_left = left;
    let mut high_right = right;
    let mut scale = 1.0;
    for index in 1..=lumps {
        let high_left = format!("{tag}#HI{index}");
        let low_right = if index == lumps {
            high_left.clone()
        } else {
            format!("{tag}#LO{index}")
        };
        elements.push(element(
            format!("R{tag}#LO{index}"),
            resistor(scale * r1),
            vec![low_left, low_right.clone()],
        ));
        elements.push(element(
            format!("R{tag}#HI{index}"),
            resistor(scale * r1),
            vec![high_left.clone(), high_right],
        ));
        elements.push(shunt("LO", index, &low_right, scale));
        if index != lumps {
            elements.push(shunt("HI", index, &high_left, scale));
        }
        scale *= k;
        low_left = low_right;
        high_right = high_left;
    }
    Ok(())
}
