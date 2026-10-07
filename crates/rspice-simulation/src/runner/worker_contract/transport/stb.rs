//! Transfer complete STB samples with explicit presence bits in a bounded numeric buffer.
use super::*;
use rspice_core::analysis::pole_zero::{PoleSpectrum, RootSetEvidence};
use rspice_core::analysis::stb::{
    BodePoint, CircuitPoleEvidence, CircuitPoleFailure, NyquistPoint, StabilityMargins, StbResult,
};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
enum WorkerCircuitPoles {
    #[default]
    NotComputed,
    Available {
        roots: WorkerF64Series,
        roots_digest: rspice_app_types::product::ContentDigest,
        evidence: RootSetEvidence,
    },
    Unavailable {
        cause: CircuitPoleFailure,
    },
}

impl WorkerCircuitPoles {
    #[cfg(any(feature = "browser-worker", test))]
    fn from_response(
        value: CircuitPoleEvidence,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Result<Self, String> {
        Ok(match value {
            CircuitPoleEvidence::NotComputed => Self::NotComputed,
            CircuitPoleEvidence::Unavailable { cause } => Self::Unavailable { cause },
            CircuitPoleEvidence::Available { spectrum } => {
                let mut values = Vec::new();
                values
                    .try_reserve_exact(
                        spectrum
                            .poles
                            .len()
                            .checked_mul(2)
                            .ok_or("circuit-pole buffer overflow")?,
                    )
                    .map_err(|e| e.to_string())?;
                for pole in spectrum.poles {
                    values.extend([pole.re, pole.im]);
                }
                Self::Available {
                    roots_digest: crate::execution_identity::f64_sequence_digest(
                        "rspice.worker-stb-circuit-poles/v1",
                        &values,
                    ),
                    roots: WorkerF64Series::from_vec(values, buffers),
                    evidence: spectrum.evidence,
                }
            }
        })
    }

    fn into_response(
        self,
        buffers: &[Vec<f64>],
        remaining: usize,
    ) -> Result<CircuitPoleEvidence, String> {
        Ok(match self {
            Self::NotComputed => CircuitPoleEvidence::NotComputed,
            Self::Unavailable { cause } => CircuitPoleEvidence::Unavailable { cause },
            Self::Available {
                roots,
                roots_digest,
                evidence,
            } => {
                if !matches!(roots, WorkerF64Series::Buffer { .. })
                    || !roots.len().is_multiple_of(2)
                    || roots.len() > remaining
                {
                    return Err("circuit poles require a bounded dedicated buffer".into());
                }
                let values = roots.into_vec(buffers)?;
                if crate::execution_identity::f64_sequence_digest(
                    "rspice.worker-stb-circuit-poles/v1",
                    &values,
                ) != roots_digest
                {
                    return Err("circuit-pole buffer digest mismatch".into());
                }
                let mut poles = Vec::new();
                poles
                    .try_reserve_exact(values.len() / 2)
                    .map_err(|e| e.to_string())?;
                for value in values.chunks_exact(2) {
                    poles.push(rspice_core::Complex64::new(value[0], value[1]));
                }
                CircuitPoleEvidence::Available {
                    spectrum: PoleSpectrum { poles, evidence },
                }
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerStbResultTransport {
    #[serde(default)]
    circuit_poles: WorkerCircuitPoles,
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
            .filter(|n| {
                n.saturating_add(response.circuit_poles.retained_value_count())
                    <= MAX_WORKER_F64_VALUES
            })
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
            circuit_poles: WorkerCircuitPoles::from_response(response.circuit_poles, buffers)?,
            margins: response.margins,
            warnings: response.warnings,
            nyquist: !response.nyquist_points.is_empty(),
            points: WorkerF64Series::from_vec(values, buffers),
        })
    }
    pub(super) fn into_response(self, buffers: &[Vec<f64>]) -> Result<StbResult, String> {
        if let (
            WorkerCircuitPoles::Available {
                roots: WorkerF64Series::Buffer { buffer: roots, .. },
                ..
            },
            WorkerF64Series::Buffer { buffer: points, .. },
        ) = (&self.circuit_poles, &self.points)
            && roots == points
        {
            return Err("circuit poles and STB samples must use distinct buffers".into());
        }
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
        response.circuit_poles = self
            .circuit_poles
            .into_response(buffers, MAX_WORKER_F64_VALUES.saturating_sub(n))?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stb_worker_refuses_aliasing_circuit_poles_with_loop_samples() {
        let mut response = rspice_core::analysis::stb::StbAnalyzer::new(Default::default())
            .analyze(&[1.0, 2.0], &[rspice_core::Complex64::new(0.5, 0.0); 2])
            .unwrap();
        response.circuit_poles = CircuitPoleEvidence::Available {
            spectrum: PoleSpectrum {
                poles: vec![rspice_core::Complex64::new(-1.0, 0.0)],
                evidence: RootSetEvidence::Qualified {
                    certificate: rspice_core::analysis::SpectrumCertificate::exact(1, 0).unwrap(),
                },
            },
        };
        let mut buffers = Vec::new();
        let mut transport =
            WorkerStbResultTransport::from_response(response, &mut buffers).unwrap();
        let WorkerCircuitPoles::Available { roots, .. } = &mut transport.circuit_poles else {
            panic!()
        };
        *roots = transport.points.clone();
        assert!(
            transport
                .into_response(&buffers)
                .unwrap_err()
                .contains("distinct buffers")
        );
    }
}
