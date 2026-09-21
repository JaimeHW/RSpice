//! Lift rates and outputs after SDT occurrences have independent identities.
//!
//! Waveform lowering may duplicate an argument expression, for example the
//! high level of a pulse. Duplicating a scalar state reference is safe;
//! duplicating an SDT call would invent additional memory coordinates.

use super::*;

impl IntegralEquations {
    pub(in crate::device::behavioral) fn lifted_phases(
        &self,
        phase_names: &[String],
        lift: impl Fn(&Expr) -> Result<Expr, String>,
    ) -> Result<Self, String> {
        if self.phase_dimensions.is_some() {
            return Err("behavioral integral equations have already been phase-lifted".into());
        }
        let equation = |equation: &Equation| {
            let mut inputs = equation
                .program
                .node_map
                .iter()
                .map(|(name, &index)| (name.clone(), equation.inputs[index]))
                .collect::<HashMap<_, _>>();
            for (dimension, name) in phase_names.iter().enumerate() {
                if inputs
                    .insert(name.clone(), Input::Phase(dimension))
                    .is_some()
                {
                    return Err("behavioral phase identity collides with an integral input".into());
                }
            }
            Ok(Equation::new(lift(&equation.ast)?, &inputs))
        };
        Ok(Self {
            rates: self
                .rates
                .iter()
                .map(equation)
                .collect::<Result<_, String>>()?,
            output: equation(&self.output)?,
            phase_dimensions: Some(phase_names.len()),
        })
    }

    pub(in crate::device::behavioral) fn has_phase_basis(&self, dimensions: usize) -> bool {
        self.phase_dimensions == Some(dimensions)
    }
}
