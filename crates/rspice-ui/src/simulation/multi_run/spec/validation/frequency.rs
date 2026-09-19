//! Validation for the frequency-domain analyses.
//!
//! The same family `runner/spec/frequency.rs` dispatches. Two of the three
//! here refuse on a probe rather than on a number: a scattering roster and
//! a transfer's two ends both name design objects, and a name the design
//! does not carry is a run that fails in the solver instead of at the
//! form.

use crate::simulation::multi_run::AnalysisSpec;

/// Validate one frequency-domain specification.
pub(super) fn validate(spec: &AnalysisSpec) -> Result<(), String> {
    match spec {
        AnalysisSpec::Stb {
            probe_node,
            start_freq,
            stop_freq,
            points_per_decade,
            ..
        } => {
            if probe_node.trim().is_empty() {
                return Err("STB probe_node is required".to_string());
            }
            if *start_freq <= 0.0 {
                return Err("STB start_freq must be > 0".to_string());
            }
            if *stop_freq <= 0.0 {
                return Err("STB stop_freq must be > 0".to_string());
            }
            if *stop_freq <= *start_freq {
                return Err("STB stop_freq must be > start_freq".to_string());
            }
            if *points_per_decade == 0 {
                return Err("STB points_per_decade must be > 0".to_string());
            }
            Ok(())
        }
        AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            points_per_unit,
            z0,
            ports,
            sweep,
            do_noise,
            ..
        } => {
            if !start_freq.is_finite()
                || *start_freq < 0.0
                || (*start_freq == 0.0
                    && (*sweep != crate::simulation::multi_run::FrequencySweep::Linear
                        || *do_noise))
            {
                return Err("SP start frequency must be nonnegative for LIN, and positive for logarithmic sweeps or noise".into());
            }
            if !stop_freq.is_finite() || *stop_freq < *start_freq {
                return Err(
                    "SP stop frequency must be finite and at least the start frequency".into(),
                );
            }
            if *points_per_unit == 0 {
                return Err("S-parameter points_per_unit must be > 0".to_string());
            }
            if !z0.is_finite() || *z0 <= 0.0 {
                return Err("S-parameter z0 must be > 0".to_string());
            }
            // An empty list uses the deck's authored ports; the engine
            // validates the final count after hierarchy and parameters resolve.
            for (idx, port) in ports.iter().enumerate() {
                if port.node_pos.trim().is_empty() {
                    return Err(format!(
                        "S-parameter port{} positive node is required",
                        idx + 1
                    ));
                }
                if port.node_neg.trim().is_empty() {
                    return Err(format!(
                        "S-parameter port{} negative node is required",
                        idx + 1
                    ));
                }
                if let Some(port_z0) = port.z0
                    && (!port_z0.is_finite() || port_z0 <= 0.0)
                {
                    return Err(format!("S-parameter port{} z0 must be > 0", idx + 1));
                }
            }
            Ok(())
        }
        AnalysisSpec::Tf {
            input_source,
            output_expression,
            transfer_gain,
            input_resistance,
            output_resistance,
            ..
        } => {
            if input_source.trim().is_empty() {
                return Err("TF input_source is required".to_owned());
            }
            if input_source != input_source.trim()
                || input_source.trim().chars().any(char::is_whitespace)
            {
                return Err("TF input_source must be one independent-source name".to_owned());
            }
            validate_tf_output_expression(output_expression)?;
            if !transfer_gain && !input_resistance && !output_resistance {
                return Err(
                    "TF requires transfer gain, input resistance, or output resistance".to_owned(),
                );
            }
            Ok(())
        }
        other => Err(super::misrouted_specification("frequency-domain", other)),
    }
}

pub(super) fn validate_tf_output_expression(expression: &str) -> Result<(), String> {
    let trimmed = expression.trim();
    if expression != trimmed {
        return Err("TF output_expression must not contain surrounding whitespace".to_owned());
    }
    let expression = trimmed;
    let Some(open) = expression.find('(') else {
        return Err("TF output_expression must use V(node), V(node,ref), or I(element)".to_owned());
    };
    if !expression.ends_with(')') || expression[open + 1..expression.len() - 1].contains(['(', ')'])
    {
        return Err("TF output_expression must contain one balanced probe call".to_owned());
    }
    let function = &expression[..open];
    let arguments = expression[open + 1..expression.len() - 1]
        .split(',')
        .collect::<Vec<_>>();
    let valid = if function.eq_ignore_ascii_case("V") {
        matches!(arguments.as_slice(), [node] if !node.is_empty())
            || matches!(arguments.as_slice(), [node, reference] if !node.is_empty() && !reference.is_empty())
    } else if function.eq_ignore_ascii_case("I") {
        matches!(arguments.as_slice(), [element] if !element.is_empty())
    } else {
        false
    };
    if !valid
        || arguments.iter().any(|argument| {
            *argument != argument.trim() || argument.chars().any(char::is_whitespace)
        })
    {
        return Err("TF output_expression must use V(node), V(node,ref), or I(element)".to_owned());
    }
    Ok(())
}
