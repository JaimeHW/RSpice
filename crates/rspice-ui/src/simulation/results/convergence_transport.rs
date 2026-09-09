//! Compact convergence metadata shared by result and dependency transport.
//!
//! Indices use two exact 32-bit limbs per entry in Float64 transfer buffers.
//! Counts remain decimal strings in metadata; no 64-bit integer is rounded by JS.

use crate::state::{PeriodicConvergenceEvidence, TransientConvergenceEvidence};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConvergenceTransport<S> {
    metadata: TransientConvergenceEvidence,
    transient_indices: S,
    transient_times: S,
    initialization_indices: Option<S>,
}

impl<S> ConvergenceTransport<S> {
    pub(in crate::simulation) fn from_evidence<'a>(
        evidence: &'a TransientConvergenceEvidence,
        mut append: impl FnMut(Cow<'a, [f64]>) -> S,
    ) -> Self {
        let report = &evidence.transient;
        let transient_indices = append(Cow::Owned(pack_indices(&report.force_accepted_indices)));
        let transient_times = append(Cow::Borrowed(
            report
                .time_basis
                .as_ref()
                .map_or(&[], |basis| basis.force_accepted_times_s.as_slice()),
        ));
        let initialization_indices = evidence.initialization.as_ref().map(|initialization| {
            append(Cow::Owned(pack_indices(
                &initialization.report.force_accepted_indices,
            )))
        });
        Self {
            metadata: TransientConvergenceEvidence {
                transient: report.metadata_only(),
                initialization: evidence.initialization.as_ref().map(|initialization| {
                    PeriodicConvergenceEvidence {
                        method: initialization.method,
                        solver_iterations: initialization.solver_iterations,
                        final_residual: initialization.final_residual,
                        report: initialization.report.metadata_only(),
                    }
                }),
            },
            transient_indices,
            transient_times,
            initialization_indices,
        }
    }

    pub(in crate::simulation) fn into_evidence(
        self,
        declared_len: impl Fn(&S) -> usize,
        mut read: impl FnMut(S) -> Result<Vec<f64>, String>,
    ) -> Result<TransientConvergenceEvidence, String> {
        let Self {
            mut metadata,
            transient_indices,
            transient_times,
            initialization_indices,
        } = self;
        let count = metadata.transient.force_accepted_points;
        if count.checked_mul(2) != Some(declared_len(&transient_indices) as u64)
            || count != declared_len(&transient_times) as u64
        {
            return Err(
                "Convergence transfer lengths do not match the reported point count".to_owned(),
            );
        }
        match (&metadata.initialization, &initialization_indices) {
            (Some(initialization), Some(indices))
                if initialization.report.force_accepted_points.checked_mul(2)
                    == Some(declared_len(indices) as u64) => {}
            (None, None) => {}
            _ => {
                return Err(
                    "Periodic initialization convergence transfer length is missing or invalid"
                        .to_owned(),
                );
            }
        }
        if !metadata.transient.force_accepted_indices.is_empty()
            || metadata
                .transient
                .time_basis
                .as_ref()
                .is_some_and(|basis| !basis.force_accepted_times_s.is_empty())
            || metadata
                .initialization
                .as_ref()
                .is_some_and(|initialization| {
                    !initialization.report.force_accepted_indices.is_empty()
                        || initialization.report.time_basis.is_some()
                })
        {
            return Err("Convergence arrays must use their dedicated transfer buffers".to_owned());
        }
        metadata.transient.force_accepted_indices = unpack_indices(
            read(transient_indices)?,
            metadata.transient.force_accepted_points,
        )?;
        let times = read(transient_times)?;
        if times.len() as u64 != metadata.transient.force_accepted_points {
            return Err(
                "Convergence time buffer does not match the reported point count".to_owned(),
            );
        }
        metadata
            .transient
            .time_basis
            .as_mut()
            .ok_or_else(|| "Transient convergence is missing its source time basis".to_owned())?
            .force_accepted_times_s = times;
        match (&mut metadata.initialization, initialization_indices) {
            (Some(initialization), Some(indices)) => {
                initialization.report.force_accepted_indices =
                    unpack_indices(read(indices)?, initialization.report.force_accepted_points)?;
            }
            (None, None) => {}
            _ => {
                return Err(
                    "Periodic initialization convergence buffer is missing or unexpected"
                        .to_owned(),
                );
            }
        }
        metadata.validate()?;
        Ok(metadata)
    }
}

fn pack_indices(indices: &[u64]) -> Vec<f64> {
    indices
        .iter()
        .flat_map(|&index| [f64::from(index as u32), f64::from((index >> 32) as u32)])
        .collect()
}

fn unpack_indices(values: Vec<f64>, count: u64) -> Result<Vec<u64>, String> {
    if count.checked_mul(2) != Some(values.len() as u64) {
        return Err("Convergence index buffer does not match the reported point count".to_owned());
    }
    values
        .chunks_exact(2)
        .map(|pair| {
            if pair.iter().any(|&value| {
                !value.is_finite()
                    || value < 0.0
                    || value > f64::from(u32::MAX)
                    || value.fract() != 0.0
            }) {
                return Err("Convergence index buffer contains an invalid integer limb".to_owned());
            }
            Ok((pair[0] as u64) | ((pair[1] as u64) << 32))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::NoAbort;

    #[test]
    fn convergence_index_buffers_preserve_the_entire_u64_domain() {
        let indices = [0, (1 << 53) + 1, u64::MAX];
        assert_eq!(unpack_indices(pack_indices(&indices), 3).unwrap(), indices);
        for invalid in [
            vec![0.0],
            vec![f64::NAN, 0.0],
            vec![0.5, 0.0],
            vec![4294967296.0, 0.0],
            vec![-1.0, 0.0],
        ] {
            assert!(unpack_indices(invalid, 1).is_err());
        }
    }

    #[test]
    fn convergence_transfer_rejects_bad_lengths_before_reading_buffers() {
        let mut metrics = rspice_core::diagnostics::ConvergenceQuality::default();
        metrics.record_force_accept(1);
        let evidence =
            TransientConvergenceEvidence::capture(metrics, &[0.0, 1.0], &NoAbort).unwrap();
        let mut transfer = ConvergenceTransport::from_evidence(&evidence, |values| values.len());
        transfer.transient_times = 2;
        let error = transfer
            .into_evidence(
                |len| *len,
                |_| panic!("invalid descriptors must fail before buffer allocation"),
            )
            .unwrap_err();
        assert!(error.contains("lengths"));
    }

    #[test]
    fn convergence_transport_keeps_large_quality_vectors_out_of_metadata() {
        let time = (0..100_000).map(f64::from).collect::<Vec<_>>();
        let metrics = rspice_core::diagnostics::ConvergenceQuality {
            force_accepted_points: time.len(),
            force_accepted_indices: (0..time.len()).collect(),
            ..Default::default()
        };
        let evidence = TransientConvergenceEvidence::capture(metrics, &time, &NoAbort).unwrap();
        let mut buffers = Vec::new();
        let transport = ConvergenceTransport::from_evidence(&evidence, |values| {
            let index = buffers.len();
            buffers.push(values.into_owned());
            index
        });
        let json = serde_json::to_string(&transport).unwrap();
        assert!(
            json.len() < 1024,
            "quality metadata must not grow with point count"
        );
        let restored: ConvergenceTransport<usize> = serde_json::from_str(&json).unwrap();
        let restored = restored
            .into_evidence(
                |index| buffers[*index].len(),
                |index| Ok(buffers[index].clone()),
            )
            .unwrap();
        assert_eq!(restored, evidence);
    }
}
