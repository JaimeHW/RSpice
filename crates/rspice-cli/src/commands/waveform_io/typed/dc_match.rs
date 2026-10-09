//! Preserve the owners of mismatch variance, including trimmed report context.

use super::PayloadProjection;
use crate::cli::CliError;
use crate::commands::report_identity::encode_part;
use rspice_core::execution::{SignalUnit, result_document::DcMatchPayload};

impl PayloadProjection<'_> {
    pub(super) fn dc_match(
        &mut self,
        payload: &DcMatchPayload,
        unit: SignalUnit,
    ) -> Result<(), CliError> {
        if self.points != 1 {
            return Err(super::conversion_error(
                self.path,
                "DC mismatch requires one report row",
            ));
        }
        self.constant(
            format!("dcmatch:output({})", payload.output),
            SignalUnit::Dimensionless,
            1.0,
        )?;
        self.constant(
            "sigma_multiplier".into(),
            SignalUnit::Dimensionless,
            payload.sigma_multiplier,
        )?;
        for (name, value) in [
            ("retained_contributors", payload.contributors.len()),
            ("evaluated_contributors", payload.evaluated_contributors),
            (
                "applied_correlations_mismatch",
                payload.applied_correlations_mismatch,
            ),
            (
                "applied_correlations_process",
                payload.applied_correlations_process,
            ),
        ] {
            let value = super::exact_integer_sample(value as u64).ok_or_else(|| {
                super::conversion_error(
                    self.path,
                    format!(
                        "DC mismatch '{name}' cannot be represented exactly by a numeric flat table"
                    ),
                )
            })?;
            self.constant(name.into(), SignalUnit::Dimensionless, value)?;
        }
        for contributor in &payload.contributors {
            let scope = contributor.scope.tag();
            let instance = encode_part(&contributor.instance);
            let parameter = encode_part(&contributor.parameter);
            let owner = format!("{scope}:{instance}/{parameter}");
            self.constant(
                format!("dcmatch:contributor({scope},{instance},{parameter})"),
                SignalUnit::Dimensionless,
                1.0,
            )?;
            // The payload declares the output unit, but not the native unit of
            // each statistical parameter. Do not infer it from a device name.
            self.constant(
                format!("sigma_parameter({owner})"),
                SignalUnit::Unspecified,
                contributor.sigma_parameter,
            )?;
            self.constant(
                format!("sensitivity({owner})"),
                SignalUnit::Unspecified,
                contributor.sensitivity,
            )?;
            self.constant(
                format!("contribution({owner})"),
                unit.clone(),
                contributor.contribution,
            )?;
            self.constant(
                format!("share({owner})"),
                SignalUnit::Dimensionless,
                contributor.share,
            )?;
        }
        Ok(())
    }
}
