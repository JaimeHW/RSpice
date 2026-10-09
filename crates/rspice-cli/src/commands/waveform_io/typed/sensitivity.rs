//! Retain every derivative, its parameter identity, and per-sample availability.

use super::{ColumnData, PayloadProjection};
use crate::cli::CliError;
use rspice_core::analysis::{SensitivityUnavailability as Reason, SensitivityValue};
use rspice_core::execution::SignalUnit;
use rspice_core::execution::result_document::{
    ComplexSample, SensitivityElementTag, SensitivityPayload,
};

impl PayloadProjection<'_> {
    pub(super) fn sensitivity(&mut self, payload: &SensitivityPayload) -> Result<(), CliError> {
        self.constant(
            format!("sens:output({})", payload.output),
            SignalUnit::Dimensionless,
            1.0,
        )?;
        for entry in &payload.entries {
            if self.points != 1 {
                return Err(super::conversion_error(
                    self.path,
                    "DC sensitivity requires one report row",
                ));
            }
            self.sensitivity_identity(
                &entry.vector_name,
                &entry.element,
                entry.element_kind,
                &entry.parameter,
                entry.nominal_value,
            )?;
            let name = format!("d{}/d({})", payload.output, entry.vector_name);
            // The payload names the native parameter but does not declare its
            // unit. A derivative is not just a voltage or current quantity.
            self.constant(name.clone(), SignalUnit::Unspecified, entry.absolute)?;
            self.sensitivity_real(
                format!("normalized({name})"),
                SignalUnit::Dimensionless,
                std::slice::from_ref(&entry.normalized),
            )?;
        }
        for entry in &payload.ac_entries {
            self.sensitivity_identity(
                &entry.vector_name,
                &entry.element,
                entry.element_kind,
                &entry.parameter,
                entry.nominal_value,
            )?;
            let name = format!("d{}/d({})", payload.output, entry.vector_name);
            if entry.absolute.len() != self.points {
                return Err(super::conversion_error(
                    self.path,
                    "AC sensitivity does not cover its frequency grid",
                ));
            }
            self.push(name.clone(), SignalUnit::Unspecified, 2, || {
                ColumnData::Complex {
                    real: entry.absolute.iter().map(|value| value.real).collect(),
                    imag: entry.absolute.iter().map(|value| value.imaginary).collect(),
                }
            })?;
            self.sensitivity_complex(format!("normalized({name})"), &entry.normalized)?;
            self.sensitivity_real(
                format!("d|{}|/d({})", payload.output, entry.vector_name),
                SignalUnit::Unspecified,
                &entry.magnitude,
            )?;
            self.sensitivity_real(
                format!("darg({})/d({})", payload.output, entry.vector_name),
                SignalUnit::Unspecified,
                &entry.phase,
            )?;
        }
        Ok(())
    }

    fn sensitivity_identity(
        &mut self,
        vector: &str,
        element: &str,
        kind: SensitivityElementTag,
        parameter: &str,
        nominal: f64,
    ) -> Result<(), CliError> {
        let serde_json::Value::String(kind) = serde_json::to_value(kind)
            .map_err(|error| super::conversion_error(self.path, error.to_string()))?
        else {
            return Err(super::conversion_error(
                self.path,
                "sensitivity element kind is not a tag",
            ));
        };
        let fields =
            [vector, &kind, element, parameter].map(crate::commands::report_identity::encode_part);
        self.constant(
            format!("sens:parameter({})", fields.join(",")),
            SignalUnit::Dimensionless,
            1.0,
        )?;
        self.constant(
            format!("nominal({vector})"),
            SignalUnit::Unspecified,
            nominal,
        )
    }

    fn sensitivity_real(
        &mut self,
        name: String,
        unit: SignalUnit,
        values: &[SensitivityValue<f64>],
    ) -> Result<(), CliError> {
        if values.len() != self.points {
            return Err(super::conversion_error(
                self.path,
                "sensitivity values do not cover the report grid",
            ));
        }
        self.push(name.clone(), unit, 1, || {
            ColumnData::optional_real(values.iter().map(|value| value.value()).collect())
        })?;
        self.sensitivity_status(&name, values.iter().map(SensitivityValue::reason))
    }

    fn sensitivity_complex(
        &mut self,
        name: String,
        values: &[SensitivityValue<ComplexSample>],
    ) -> Result<(), CliError> {
        if values.len() != self.points {
            return Err(super::conversion_error(
                self.path,
                "sensitivity values do not cover the report grid",
            ));
        }
        self.push(name.clone(), SignalUnit::Dimensionless, 2, || {
            ColumnData::optional_complex(
                values
                    .iter()
                    .map(|value| value.value().map(Into::into))
                    .collect(),
            )
        })?;
        self.sensitivity_status(&name, values.iter().map(SensitivityValue::reason))
    }

    fn sensitivity_status(
        &mut self,
        name: &str,
        reasons: impl Iterator<Item = Option<Reason>> + Clone,
    ) -> Result<(), CliError> {
        if reasons
            .clone()
            .any(|reason| reason == Some(Reason::InvalidInput))
        {
            return Err(super::conversion_error(
                self.path,
                "invalid sensitivity input cannot be exported",
            ));
        }
        if reasons.clone().any(|reason| reason.is_some()) {
            self.push(
                format!("{name}:sensitivity_status"),
                SignalUnit::Dimensionless,
                1,
                || {
                    ColumnData::Real(
                        reasons
                            .map(|reason| match reason {
                                None => 0.0,
                                Some(Reason::ZeroOutput) => 1.0,
                                Some(Reason::NondifferentiableMagnitude) => 2.0,
                                Some(Reason::OutOfRange) => 3.0,
                                Some(Reason::InvalidInput) => {
                                    unreachable!("invalid input rejected before allocation")
                                }
                            })
                            .collect(),
                    )
                },
            )?;
        }
        Ok(())
    }
}
