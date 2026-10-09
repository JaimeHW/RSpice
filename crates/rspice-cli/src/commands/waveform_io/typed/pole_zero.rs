//! Preserve transfer gains, root sets, and their numerical qualification.

use super::{ColumnData, PayloadProjection, exact_integer_sample};
use crate::cli::CliError;
use rspice_core::execution::SignalUnit;
use rspice_core::execution::result_document::{
    ComplexSample, PoleZeroPayload, RootSetEvidenceDocument,
};

impl PayloadProjection<'_> {
    fn pole_zero_marker(&mut self, field: &str, value: &str) -> Result<(), CliError> {
        self.constant(
            format!("pz:{field}({value})"),
            SignalUnit::Dimensionless,
            1.0,
        )
    }

    fn pole_zero_count(&mut self, field: &str, value: usize) -> Result<(), CliError> {
        let value = exact_integer_sample(value as i128).ok_or_else(|| {
            super::conversion_error(
                self.path,
                format!("PZ {field} cannot be represented exactly"),
            )
        })?;
        self.constant(format!("pz:{field}"), SignalUnit::Dimensionless, value)
    }

    pub(super) fn pole_zero(&mut self, payload: &PoleZeroPayload) -> Result<(), CliError> {
        let unit = payload
            .root_unit
            .as_ref()
            .unwrap_or(&SignalUnit::Unspecified);
        // Keep the traditional root columns first; append report quantities.
        self.pole_zero_roots("pole", &payload.poles, unit)?;
        self.pole_zero_roots("zero", &payload.zeros, unit)?;
        self.pole_zero_marker("input", &payload.input)?;
        self.pole_zero_marker("output", &payload.output)?;
        let gain_unit = payload
            .gain_unit
            .as_ref()
            .unwrap_or(&SignalUnit::Unspecified);
        for (name, value) in [
            ("dc_gain", payload.dc_gain),
            ("high_frequency_gain", payload.high_frequency_gain),
        ] {
            let points = self.points;
            self.push(name.into(), gain_unit.clone(), 1, || {
                ColumnData::optional_real(vec![value; points])
            })?;
        }
        for (role, roots, evidence) in [
            ("pole", payload.poles.as_slice(), &payload.pole_evidence),
            ("zero", payload.zeros.as_slice(), &payload.zero_evidence),
        ] {
            self.pole_zero_count(&format!("finite_{role}s"), roots.len())?;
            let (state, certificate) = match evidence {
                RootSetEvidenceDocument::NotRequested => ("not_requested", None),
                RootSetEvidenceDocument::LegacyUnknown => ("legacy_unknown", None),
                RootSetEvidenceDocument::QualifiedEmpty { certificate } => {
                    ("qualified_empty", Some(certificate))
                }
                RootSetEvidenceDocument::Qualified { certificate } => {
                    ("qualified", Some(certificate))
                }
                RootSetEvidenceDocument::Approximate { certificate } => {
                    ("approximate", Some(certificate))
                }
            };
            self.pole_zero_marker(&format!("{role}s_evidence"), state)?;
            if let Some(certificate) = certificate {
                self.pole_zero_count(&format!("{role}s_problem_order"), certificate.problem_order)?;
                self.pole_zero_count(&format!("infinite_{role}s"), certificate.infinite_count)?;
                self.constant(
                    format!("pz:{role}s_max_backward_error"),
                    SignalUnit::Dimensionless,
                    certificate.max_backward_error,
                )?;
                self.constant(
                    format!("pz:{role}s_qualification_tolerance"),
                    SignalUnit::Dimensionless,
                    certificate.qualification_tolerance,
                )?;
                self.pole_zero_marker(
                    &format!("{role}s_asymptotically_stable"),
                    match certificate.asymptotically_stable {
                        Some(true) => "true",
                        Some(false) => "false",
                        None => "unknown",
                    },
                )?;
            }
        }
        Ok(())
    }

    fn pole_zero_roots(
        &mut self,
        role: &str,
        roots: &[ComplexSample],
        unit: &SignalUnit,
    ) -> Result<(), CliError> {
        // Root sets have no authored order. Admit their complete expansion
        // before cloning and sorting to give reordered spectra equal identities.
        super::enforce_table_value_limits(
            self.path,
            self.points
                .saturating_mul(self.width.saturating_add(roots.len().saturating_mul(2))),
            self.limits,
        )?;
        let mut roots = roots.to_vec();
        roots.sort_by(|left, right| {
            left.real
                .total_cmp(&right.real)
                .then_with(|| left.imaginary.total_cmp(&right.imaginary))
        });
        for (index, root) in roots.into_iter().enumerate() {
            let points = self.points;
            self.push(format!("{role}({})", index + 1), unit.clone(), 2, || {
                ColumnData::Complex {
                    real: vec![root.real; points],
                    imag: vec![root.imaginary; points],
                }
            })?;
            // Preserve the traditional SPICE root types, which imply rad/s.
            // Legacy documents without units must not acquire that claim.
            if unit == &SignalUnit::RadianPerSecond
                && let Some(column) = self.columns.last_mut()
            {
                column.var_type = role.into();
            }
        }
        Ok(())
    }
}
