//! Xyce-compatible OneStep DAE residual construction.
//!
//! This module deliberately operates on already-aggregated global `Q`, `F`,
//! and `B` vectors. Device-level differencing changes floating-point behavior
//! and is not equivalent to Xyce's `OneStep::obtainResidual` sequence.

use crate::Value;
use crate::circuit::dae::XyceDaeVectors;
use thiserror::Error;

/// The Xyce OneStep integration order used to form the nonlinear residual.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum XyceOneStepOrder {
    /// First-order backward Euler.
    First,
    /// Second-order trapezoidal integration.
    Second,
}

/// Identifies an input vector in a residual-construction error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum XyceDaeVectorKind {
    Q,
    F,
    B,
    PreviousQ,
    PreviousStatic,
    CorrectionRhs,
}

impl std::fmt::Display for XyceDaeVectorKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Q => "Q",
            Self::F => "F",
            Self::B => "B",
            Self::PreviousQ => "previous Q",
            Self::PreviousStatic => "previous F-B",
            Self::CorrectionRhs => "correction RHS",
        };
        formatter.write_str(name)
    }
}

/// A validated failure while constructing a Xyce OneStep correction RHS.
#[derive(Clone, Debug, Error, PartialEq)]
pub(crate) enum XyceDaeResidualError {
    #[error("{vector} has dimension {actual}, expected {expected}")]
    DimensionMismatch {
        vector: XyceDaeVectorKind,
        expected: usize,
        actual: usize,
    },

    #[error("transient time step must be finite and positive, got {time_step}")]
    InvalidTimeStep { time_step: Value },

    #[error("second-order OneStep residual requires the previously accepted F-B vector")]
    MissingPreviousStatic,

    #[error("{vector}[{index}] is not finite: {value}")]
    NonFiniteValue {
        vector: XyceDaeVectorKind,
        index: usize,
        value: Value,
    },
}

/// Allocation-retaining workspace for Xyce OneStep residual construction.
#[derive(Clone, Debug, Default)]
pub(crate) struct XyceOneStepWorkspace {
    correction_rhs: Vec<Value>,
}

impl XyceOneStepWorkspace {
    /// Preallocates the correction vector for `dimension` circuit equations.
    pub(crate) fn new(dimension: usize) -> Self {
        Self {
            correction_rhs: vec![0.0; dimension],
        }
    }

    /// Forms `-R` in Xyce's OneStep vector-update order.
    ///
    /// The first-order residual is
    /// `R = (Q - Q_previous) / h + F - B`. The second-order residual is
    /// `R = (Q - Q_previous) / h + 0.5 F - 0.5 B + 0.5 G_previous`, where
    /// `G_previous` is the previously accepted `F - B` vector.
    ///
    /// `previous_static` is required and dimension-checked only at second
    /// order. Inputs are fully validated before the output buffer is changed.
    pub(crate) fn form_correction_rhs<'workspace>(
        &'workspace mut self,
        dae: &XyceDaeVectors,
        previous_q: &[Value],
        previous_static: Option<&[Value]>,
        time_step: Value,
        order: XyceOneStepOrder,
    ) -> Result<&'workspace [Value], XyceDaeResidualError> {
        if !time_step.is_finite() || time_step <= 0.0 {
            return Err(XyceDaeResidualError::InvalidTimeStep { time_step });
        }

        let dimension = dae.dimension();
        validate_dimension(XyceDaeVectorKind::PreviousQ, previous_q, dimension)?;
        let previous_static = match order {
            XyceOneStepOrder::First => None,
            XyceOneStepOrder::Second => {
                let previous_static =
                    previous_static.ok_or(XyceDaeResidualError::MissingPreviousStatic)?;
                validate_dimension(
                    XyceDaeVectorKind::PreviousStatic,
                    previous_static,
                    dimension,
                )?;
                Some(previous_static)
            }
        };

        validate_finite(XyceDaeVectorKind::Q, dae.q())?;
        validate_finite(XyceDaeVectorKind::F, dae.f())?;
        validate_finite(XyceDaeVectorKind::B, dae.b())?;
        validate_finite(XyceDaeVectorKind::PreviousQ, previous_q)?;
        if let Some(previous_static) = previous_static {
            validate_finite(XyceDaeVectorKind::PreviousStatic, previous_static)?;
        }

        resize_and_zero(&mut self.correction_rhs, dimension);

        // Equivalent to Epetra Update(1.0, Q, -1.0, Q_previous, 0.0).
        // Keeping this as a distinct pass is important: all device Q loads are
        // aggregated before the accepted Q vector is differenced.
        for (rhs, (&q, &q_previous)) in self
            .correction_rhs
            .iter_mut()
            .zip(dae.q().iter().zip(previous_q))
        {
            *rhs = q - q_previous;
        }

        let inverse_time_step = 1.0 / time_step;
        match order {
            XyceOneStepOrder::First => {
                // Epetra Update(1.0, F, -1.0, B, 1.0 / h).
                for ((rhs, &f), &b) in self.correction_rhs.iter_mut().zip(dae.f()).zip(dae.b()) {
                    let scaled_charge = inverse_time_step * *rhs;
                    let with_static = scaled_charge + f;
                    *rhs = with_static - b;
                }
            }
            XyceOneStepOrder::Second => {
                // Epetra Update(0.5, F, -0.5, B, 1.0 / h), followed by
                // Update(0.5, G_previous). The two passes intentionally are
                // not fused because their rounding points are observable.
                for ((rhs, &f), &b) in self.correction_rhs.iter_mut().zip(dae.f()).zip(dae.b()) {
                    let scaled_charge = inverse_time_step * *rhs;
                    let with_static = scaled_charge + (0.5 * f);
                    *rhs = with_static + (-0.5 * b);
                }
                for (rhs, &previous_static) in self
                    .correction_rhs
                    .iter_mut()
                    .zip(previous_static.expect("validated for second order"))
                {
                    *rhs += 0.5 * previous_static;
                }
            }
        }

        // Xyce's nonlinear solver expects the negative residual.
        for rhs in &mut self.correction_rhs {
            *rhs *= -1.0;
        }

        validate_finite(XyceDaeVectorKind::CorrectionRhs, &self.correction_rhs)?;
        Ok(&self.correction_rhs)
    }
}

fn validate_dimension(
    vector: XyceDaeVectorKind,
    values: &[Value],
    expected: usize,
) -> Result<(), XyceDaeResidualError> {
    if values.len() == expected {
        Ok(())
    } else {
        Err(XyceDaeResidualError::DimensionMismatch {
            vector,
            expected,
            actual: values.len(),
        })
    }
}

fn validate_finite(
    vector: XyceDaeVectorKind,
    values: &[Value],
) -> Result<(), XyceDaeResidualError> {
    if let Some((index, &value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        Err(XyceDaeResidualError::NonFiniteValue {
            vector,
            index,
            value,
        })
    } else {
        Ok(())
    }
}

fn resize_and_zero(vector: &mut Vec<Value>, dimension: usize) {
    vector.resize(dimension, 0.0);
    vector.fill(0.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dae(q: &[Value], f: &[Value], b: &[Value]) -> XyceDaeVectors {
        assert_eq!(q.len(), f.len());
        assert_eq!(q.len(), b.len());
        let mut vectors = XyceDaeVectors::new(q.len());
        let (loaded_q, loaded_f, loaded_b) = vectors.q_f_b_mut();
        loaded_q.copy_from_slice(q);
        loaded_f.copy_from_slice(f);
        loaded_b.copy_from_slice(b);
        vectors
    }

    #[test]
    fn first_order_forms_negative_dae_residual() {
        let vectors = dae(&[5.0, -2.0], &[7.0, 3.0], &[2.0, -1.0]);
        let mut workspace = XyceOneStepWorkspace::new(2);

        let rhs = workspace
            .form_correction_rhs(&vectors, &[1.0, -4.0], None, 0.5, XyceOneStepOrder::First)
            .unwrap();

        assert_eq!(rhs, &[-13.0, -8.0]);
    }

    #[test]
    fn second_order_forms_negative_trapezoidal_residual() {
        let vectors = dae(&[7.0, -1.0], &[8.0, 6.0], &[2.0, -4.0]);
        let mut workspace = XyceOneStepWorkspace::new(2);

        let rhs = workspace
            .form_correction_rhs(
                &vectors,
                &[3.0, -3.0],
                Some(&[10.0, -2.0]),
                2.0,
                XyceOneStepOrder::Second,
            )
            .unwrap();

        assert_eq!(rhs, &[-10.0, -5.0]);
    }

    #[test]
    fn charge_is_aggregated_before_history_differencing() {
        let mut vectors = XyceDaeVectors::new(1);
        let (q, _, _) = vectors.q_f_b_mut();
        q[0] += 1.0e16;
        q[0] += 1.0; // Rounded away in the global Q aggregation, as in Xyce.
        let mut workspace = XyceOneStepWorkspace::new(1);

        let rhs = workspace
            .form_correction_rhs(&vectors, &[1.0e16], None, 1.0, XyceOneStepOrder::First)
            .unwrap();

        assert_eq!(rhs[0].to_bits(), (-0.0_f64).to_bits());
    }

    #[test]
    fn f_and_b_remain_separate_until_the_xyce_vector_update() {
        // Xyce evaluates ((Q/h + F) - B), not Q/h + (F - B). These values
        // make that rounding boundary observable: precombining F-B loses the
        // unit contribution, whereas Xyce's update sequence retains it.
        let vectors = dae(&[1.0e16], &[-1.0e16], &[-1.0]);
        let mut workspace = XyceOneStepWorkspace::new(1);

        let rhs = workspace
            .form_correction_rhs(&vectors, &[0.0], None, 1.0, XyceOneStepOrder::First)
            .unwrap();

        assert_eq!(rhs, &[-1.0]);
        assert_eq!(vectors.f(), &[-1.0e16]);
        assert_eq!(vectors.b(), &[-1.0]);
    }

    #[test]
    fn rejects_history_dimension_mismatches() {
        let vectors = dae(&[0.0, 0.0], &[0.0, 0.0], &[0.0, 0.0]);
        let mut workspace = XyceOneStepWorkspace::default();

        let error = workspace
            .form_correction_rhs(&vectors, &[0.0], None, 1.0, XyceOneStepOrder::First)
            .unwrap_err();
        assert_eq!(
            error,
            XyceDaeResidualError::DimensionMismatch {
                vector: XyceDaeVectorKind::PreviousQ,
                expected: 2,
                actual: 1,
            }
        );

        let error = workspace
            .form_correction_rhs(
                &vectors,
                &[0.0, 0.0],
                Some(&[0.0]),
                1.0,
                XyceOneStepOrder::Second,
            )
            .unwrap_err();
        assert_eq!(
            error,
            XyceDaeResidualError::DimensionMismatch {
                vector: XyceDaeVectorKind::PreviousStatic,
                expected: 2,
                actual: 1,
            }
        );
    }

    #[test]
    fn second_order_requires_previous_static_vector() {
        let vectors = dae(&[0.0], &[0.0], &[0.0]);
        let mut workspace = XyceOneStepWorkspace::default();

        let error = workspace
            .form_correction_rhs(&vectors, &[0.0], None, 1.0, XyceOneStepOrder::Second)
            .unwrap_err();

        assert_eq!(error, XyceDaeResidualError::MissingPreviousStatic);
    }

    #[test]
    fn first_order_does_not_require_unused_previous_static_vector() {
        let vectors = dae(&[1.0], &[0.0], &[0.0]);
        let mut workspace = XyceOneStepWorkspace::default();

        let rhs = workspace
            .form_correction_rhs(&vectors, &[0.0], Some(&[]), 1.0, XyceOneStepOrder::First)
            .unwrap();

        assert_eq!(rhs, &[-1.0]);
    }

    #[test]
    fn rejects_invalid_time_steps_before_changing_output() {
        let vectors = dae(&[1.0], &[2.0], &[3.0]);
        let mut workspace = XyceOneStepWorkspace::new(1);
        workspace.correction_rhs[0] = 42.0;

        for time_step in [0.0, -1.0, Value::NAN, Value::INFINITY] {
            let error = workspace
                .form_correction_rhs(&vectors, &[0.0], None, time_step, XyceOneStepOrder::First)
                .unwrap_err();
            assert!(matches!(
                error,
                XyceDaeResidualError::InvalidTimeStep { time_step: value }
                    if value.to_bits() == time_step.to_bits()
            ));
            assert_eq!(workspace.correction_rhs, &[42.0]);
        }
    }

    #[test]
    fn rejects_each_nonfinite_input_vector() {
        let cases = [
            (
                dae(&[Value::NAN], &[0.0], &[0.0]),
                &[0.0][..],
                None,
                XyceOneStepOrder::First,
                XyceDaeVectorKind::Q,
            ),
            (
                dae(&[0.0], &[Value::INFINITY], &[0.0]),
                &[0.0][..],
                None,
                XyceOneStepOrder::First,
                XyceDaeVectorKind::F,
            ),
            (
                dae(&[0.0], &[0.0], &[Value::NEG_INFINITY]),
                &[0.0][..],
                None,
                XyceOneStepOrder::First,
                XyceDaeVectorKind::B,
            ),
            (
                dae(&[0.0], &[0.0], &[0.0]),
                &[Value::NAN][..],
                None,
                XyceOneStepOrder::First,
                XyceDaeVectorKind::PreviousQ,
            ),
            (
                dae(&[0.0], &[0.0], &[0.0]),
                &[0.0][..],
                Some(&[Value::INFINITY][..]),
                XyceOneStepOrder::Second,
                XyceDaeVectorKind::PreviousStatic,
            ),
        ];

        for (vectors, previous_q, previous_static, order, expected_vector) in cases {
            let mut workspace = XyceOneStepWorkspace::default();
            let error = workspace
                .form_correction_rhs(&vectors, previous_q, previous_static, 1.0, order)
                .unwrap_err();
            assert!(matches!(
                error,
                XyceDaeResidualError::NonFiniteValue {
                    vector,
                    index: 0,
                    ..
                } if vector == expected_vector
            ));
        }
    }

    #[test]
    fn reports_nonfinite_output_from_finite_inputs() {
        let vectors = dae(&[0.0], &[Value::MAX], &[-Value::MAX]);
        let mut workspace = XyceOneStepWorkspace::default();

        let error = workspace
            .form_correction_rhs(&vectors, &[0.0], None, 1.0, XyceOneStepOrder::First)
            .unwrap_err();

        assert!(matches!(
            error,
            XyceDaeResidualError::NonFiniteValue {
                vector: XyceDaeVectorKind::CorrectionRhs,
                index: 0,
                value
            } if value.is_infinite()
        ));
    }

    #[test]
    fn workspace_reuses_correction_buffer() {
        let vectors = dae(&[1.0, 2.0, 3.0], &[0.0; 3], &[0.0; 3]);
        let mut workspace = XyceOneStepWorkspace::new(3);
        let original_pointer = workspace.correction_rhs.as_ptr();
        let original_capacity = workspace.correction_rhs.capacity();

        workspace
            .form_correction_rhs(&vectors, &[0.0; 3], None, 1.0, XyceOneStepOrder::First)
            .unwrap();
        workspace
            .form_correction_rhs(
                &vectors,
                &[1.0; 3],
                Some(&[0.0; 3]),
                2.0,
                XyceOneStepOrder::Second,
            )
            .unwrap();

        assert_eq!(workspace.correction_rhs.as_ptr(), original_pointer);
        assert_eq!(workspace.correction_rhs.capacity(), original_capacity);
    }
}
