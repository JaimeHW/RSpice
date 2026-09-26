//! Retained transfer-function evidence and exact scalar validation.

use crate::validation::require_non_empty;

/// Electrical quantity carried by one side of a retained transfer derivative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferFunctionQuantityEvidence {
    Voltage,
    Current,
}

impl TransferFunctionQuantityEvidence {
    const fn canonical_unit(self) -> &'static str {
        match self {
            Self::Voltage => "V",
            Self::Current => "A",
        }
    }
}

/// Explicit JSON-safe scalar evidence.
///
/// Open-circuit transfer resistances are legitimately infinite. Encoding
/// infinity as a classification keeps persisted evidence standards-compliant
/// while preserving the mathematical result exactly. `Finite` is validated
/// separately so deserialization cannot smuggle NaN or infinity through it.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "classification",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum TransferFunctionScalarEvidence {
    Finite(f64),
    PositiveInfinity,
    NegativeInfinity,
}

impl TransferFunctionScalarEvidence {
    #[must_use]
    pub fn from_f64(value: f64) -> Option<Self> {
        if value.is_nan() {
            None
        } else if value == f64::INFINITY {
            Some(Self::PositiveInfinity)
        } else if value == f64::NEG_INFINITY {
            Some(Self::NegativeInfinity)
        } else {
            Some(Self::Finite(value))
        }
    }

    #[must_use]
    pub const fn as_f64(self) -> f64 {
        match self {
            Self::Finite(value) => value,
            Self::PositiveInfinity => f64::INFINITY,
            Self::NegativeInfinity => f64::NEG_INFINITY,
        }
    }

    fn validate(self, label: &str) -> Result<(), String> {
        if let Self::Finite(value) = self
            && !value.is_finite()
        {
            return Err(format!(
                "transfer-function {label} uses a non-finite value in the finite classification"
            ));
        }
        Ok(())
    }
}

/// Gain-normalization policy actually applied to retained TF evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferFunctionNormalizationEvidence {
    None,
    RelativeToNominal,
    PerSourceUnit,
}

/// Numerical policy actually applied to the TF operating-point solves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferFunctionAccuracyEvidence {
    Fast,
    Balanced,
    Accurate,
    Robust,
}

/// Borrowed inputs for validating retained transfer-function evidence.
pub struct TransferFunctionRef<'a> {
    pub input_source: &'a str,
    pub output_expression: &'a str,
    pub input_quantity: &'a TransferFunctionQuantityEvidence,
    pub output_quantity: &'a TransferFunctionQuantityEvidence,
    pub input_unit: &'a str,
    pub output_unit: &'a str,
    pub normalization: &'a TransferFunctionNormalizationEvidence,
    pub gain: &'a Option<TransferFunctionScalarEvidence>,
    pub input_resistance: &'a Option<TransferFunctionScalarEvidence>,
    pub output_resistance: &'a Option<TransferFunctionScalarEvidence>,
    pub nominal_input: &'a Option<f64>,
    pub nominal_output: &'a Option<f64>,
}

impl TransferFunctionRef<'_> {
    pub fn validate(self) -> Result<(), String> {
        let Self {
            input_source,
            output_expression,
            input_quantity,
            output_quantity,
            input_unit,
            output_unit,
            normalization,
            gain,
            input_resistance,
            output_resistance,
            nominal_input,
            nominal_output,
        } = self;
        require_non_empty(input_source, "transfer-function input source")?;
        if input_source.chars().any(char::is_whitespace) {
            return Err("transfer-function input source contains whitespace".to_owned());
        }
        require_non_empty(output_expression, "transfer-function output expression")?;
        validate_transfer_function_output(output_expression, *output_quantity)?;

        if input_unit != input_quantity.canonical_unit() {
            return Err(format!(
                "transfer-function input unit '{input_unit}' does not match {input_quantity:?}"
            ));
        }
        if output_unit != output_quantity.canonical_unit() {
            return Err(format!(
                "transfer-function output unit '{output_unit}' does not match {output_quantity:?}"
            ));
        }
        if gain.is_none() && input_resistance.is_none() && output_resistance.is_none() {
            return Err(
                "transfer-function payload contains no requested scalar evidence".to_owned(),
            );
        }
        for (label, scalar) in [
            ("gain", gain.as_ref()),
            ("input resistance", input_resistance.as_ref()),
            ("output resistance", output_resistance.as_ref()),
        ] {
            if let Some(scalar) = scalar {
                scalar.validate(label)?;
            }
        }

        let relative_gain = *normalization
            == TransferFunctionNormalizationEvidence::RelativeToNominal
            && gain.is_some();
        if relative_gain != nominal_input.is_some() || relative_gain != nominal_output.is_some() {
            return Err(
                "transfer-function nominal values must be present exactly when a relative-normalized gain is retained"
                    .to_owned(),
            );
        }
        for (label, value) in [
            ("nominal input", nominal_input.as_ref()),
            ("nominal output", nominal_output.as_ref()),
        ] {
            if let Some(value) = value
                && (!value.is_finite() || *value == 0.0)
            {
                return Err(format!(
                    "transfer-function {label} must be finite and nonzero"
                ));
            }
        }
        Ok(())
    }
}

fn validate_transfer_function_output(
    expression: &str,
    expected_quantity: TransferFunctionQuantityEvidence,
) -> Result<(), String> {
    let trimmed = expression.trim();
    if expression != trimmed {
        return Err("transfer-function output contains surrounding whitespace".to_owned());
    }
    let expression = trimmed;
    let Some(open) = expression.find('(') else {
        return Err(
            "transfer-function output must use V(node), V(node,ref), or I(element)".to_owned(),
        );
    };
    if !expression.ends_with(')') || expression[open + 1..expression.len() - 1].contains(['(', ')'])
    {
        return Err("transfer-function output has unbalanced parentheses".to_owned());
    }
    let function = &expression[..open];
    let arguments = expression[open + 1..expression.len() - 1]
        .split(',')
        .collect::<Vec<_>>();
    if arguments.iter().any(|argument| {
        argument.is_empty()
            || *argument != argument.trim()
            || argument.chars().any(char::is_whitespace)
    }) {
        return Err("transfer-function output contains an invalid identifier".to_owned());
    }
    let quantity = if function.eq_ignore_ascii_case("V") && matches!(arguments.len(), 1 | 2) {
        TransferFunctionQuantityEvidence::Voltage
    } else if function.eq_ignore_ascii_case("I") && arguments.len() == 1 {
        TransferFunctionQuantityEvidence::Current
    } else {
        return Err(
            "transfer-function output must use V(node), V(node,ref), or I(element)".to_owned(),
        );
    };
    if quantity != expected_quantity {
        return Err("transfer-function output quantity contradicts its expression".to_owned());
    }
    Ok(())
}
