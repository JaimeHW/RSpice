//! Keep circuit stability evidence beside the independently measured loop gain.

use super::{ColumnData, ExportTable, PayloadProjection, exact_integer_sample};
use crate::cli::CliError;
use rspice_core::analysis::pole_zero::StabilityVerdict;
use rspice_core::analysis::stb::{CircuitPoleEvidence, CircuitPoleFailure};
use rspice_core::execution::SignalUnit;
use std::path::Path;

pub(in crate::commands) fn append_stability_columns(
    path: &Path,
    table: &mut ExportTable,
    completed: bool,
    poles: &CircuitPoleEvidence,
    limits: rspice_core::ResourceLimits,
) -> Result<(), CliError> {
    let width = table.columns.iter().fold(1usize, |width, column| {
        width.saturating_add(match column.data {
            ColumnData::Complex { .. } | ColumnData::NullableComplex(_) => 2,
            _ => 1,
        })
    });
    PayloadProjection {
        path,
        points: table.scale.len(),
        columns: &mut table.columns,
        limits,
        width,
    }
    .stability(completed, poles)
}

impl PayloadProjection<'_> {
    fn stability_marker(&mut self, field: &str, value: &str) -> Result<(), CliError> {
        // Categorical meaning is part of the identity, so loose numeric
        // tolerances cannot turn unstable or unknown evidence into stable.
        self.constant(
            format!("stb:{field}({value})"),
            SignalUnit::Dimensionless,
            1.0,
        )
    }

    fn stability_count(&mut self, field: &str, value: usize) -> Result<(), CliError> {
        let value = exact_integer_sample(value as i128).ok_or_else(|| {
            super::conversion_error(
                self.path,
                format!("STB {field} cannot be represented exactly"),
            )
        })?;
        self.constant(format!("stb:{field}"), SignalUnit::Dimensionless, value)
    }

    pub(super) fn stability(
        &mut self,
        completed: bool,
        evidence: &CircuitPoleEvidence,
    ) -> Result<(), CliError> {
        self.stability_marker("completed", if completed { "true" } else { "false" })?;
        self.stability_marker(
            "circuit_stability",
            match evidence.stability_verdict() {
                StabilityVerdict::Stable => "stable",
                StabilityVerdict::Unstable => "unstable",
                StabilityVerdict::Indeterminate => "indeterminate",
            },
        )?;
        let spectrum = match evidence {
            CircuitPoleEvidence::NotComputed => {
                return self.stability_marker("circuit_poles", "not_computed");
            }
            CircuitPoleEvidence::Unavailable { cause } => {
                return self.stability_marker(
                    "circuit_poles",
                    match cause {
                        CircuitPoleFailure::Unsupported { .. } => "unsupported",
                        CircuitPoleFailure::Numerical { .. } => "numerical_failure",
                        CircuitPoleFailure::ResourceLimit { .. } => "resource_limit",
                    },
                );
            }
            CircuitPoleEvidence::Available { spectrum } => spectrum,
        };
        self.stability_marker(
            "circuit_poles",
            if spectrum.evidence.is_qualified() {
                "qualified"
            } else {
                "approximate"
            },
        )?;
        self.stability_count("finite_poles", spectrum.poles.len())?;
        let certificate = spectrum.evidence.certificate().ok_or_else(|| {
            super::conversion_error(
                self.path,
                "STB circuit spectrum has no numerical certificate",
            )
        })?;
        self.stability_count("problem_order", certificate.problem_order)?;
        self.stability_count("infinite_poles", certificate.infinite_count)?;
        self.constant(
            "stb:max_backward_error".into(),
            SignalUnit::Dimensionless,
            certificate.max_backward_error,
        )?;
        self.constant(
            "stb:qualification_tolerance".into(),
            SignalUnit::Dimensionless,
            certificate.qualification_tolerance,
        )?;

        // An eigenvalue set has no authored order. Canonicalize it before
        // assigning column indices so reordered, identical spectra still match.
        super::enforce_table_value_limits(
            self.path,
            self.points.saturating_mul(
                self.width
                    .saturating_add(spectrum.poles.len().saturating_mul(2)),
            ),
            self.limits,
        )?;
        let mut poles = spectrum.poles.clone();
        poles.sort_by(|left, right| {
            left.re
                .total_cmp(&right.re)
                .then_with(|| left.im.total_cmp(&right.im))
        });
        for (index, pole) in poles.into_iter().enumerate() {
            let points = self.points;
            self.push(
                format!("stb:pole({})", index + 1),
                SignalUnit::Custom("rad/s".into()),
                2,
                || ColumnData::Complex {
                    real: vec![pole.re; points],
                    imag: vec![pole.im; points],
                },
            )?;
        }
        Ok(())
    }
}
