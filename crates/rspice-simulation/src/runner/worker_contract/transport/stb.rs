//! Transfer complete STB samples with explicit presence bits in a bounded numeric buffer.
use super::*;
use rspice_core::analysis::stb::{BodePoint, NyquistPoint, StabilityMargins, StbResult};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerStbResultTransport {
    margins: StabilityMargins,
    warnings: Vec<String>,
    nyquist: bool,
    points: WorkerF64Series,
}
impl WorkerStbResultTransport {
    #[cfg(any(feature = "browser-worker", test))]
    pub(super) fn from_response(
        response: StbResult,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Result<Self, String> {
        response
            .validate_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|e| e.to_string())?;
        let len = response
            .bode_points
            .len()
            .checked_mul(9)
            .filter(|n| *n <= MAX_WORKER_F64_VALUES)
            .ok_or("STB evidence exceeds worker limit")?;
        let mut values = Vec::new();
        values.try_reserve_exact(len).map_err(|e| e.to_string())?;
        for p in response.bode_points {
            values.extend([p.frequency, p.loop_gain.re, p.loop_gain.im]);
            for v in [p.magnitude, p.magnitude_db, p.phase_deg] {
                values.extend([f64::from(v.is_some()), v.unwrap_or(0.0)]);
            }
        }
        Ok(Self {
            margins: response.margins,
            warnings: response.warnings,
            nyquist: !response.nyquist_points.is_empty(),
            points: WorkerF64Series::from_vec(values, buffers),
        })
    }
    pub(super) fn into_response(self, buffers: &[Vec<f64>]) -> Result<StbResult, String> {
        let n = self.points.len();
        let limits = rspice_core::ResourceLimits::default();
        if !matches!(self.points, WorkerF64Series::Buffer { .. })
            || n == 0
            || !n.is_multiple_of(9)
            || n > MAX_WORKER_F64_VALUES
            || n > limits.max_result_values
            || n / 9 > limits.max_analysis_points
        {
            return Err("STB requires a bounded dedicated sample buffer".into());
        }
        let values = self.points.into_vec(buffers)?;
        let optional = |flag: f64, value: f64| -> Result<Option<f64>, String> {
            match flag {
                0.0 if value == 0.0 => Ok(None),
                1.0 if value.is_finite() => Ok(Some(value)),
                _ => Err("invalid STB sample availability".into()),
            }
        };
        let mut response = StbResult::new();
        response
            .bode_points
            .try_reserve_exact(n / 9)
            .map_err(|e| e.to_string())?;
        if self.nyquist {
            response
                .nyquist_points
                .try_reserve_exact(n / 9)
                .map_err(|e| e.to_string())?;
        }
        for p in values.chunks_exact(9) {
            let loop_gain = rspice_core::Complex64::new(p[1], p[2]);
            response.bode_points.push(BodePoint {
                frequency: p[0],
                loop_gain,
                magnitude: optional(p[3], p[4])?,
                magnitude_db: optional(p[5], p[6])?,
                phase_deg: optional(p[7], p[8])?,
            });
            if self.nyquist {
                response
                    .nyquist_points
                    .push(NyquistPoint::from_loop_gain(loop_gain, p[0]));
            }
        }
        response.margins = self.margins;
        response.warnings = self.warnings;
        response
            .validate_with_abort(&limits, &rspice_core::NoAbort)
            .map_err(|e| e.to_string())?;
        Ok(response)
    }
}
