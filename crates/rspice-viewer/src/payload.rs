//! Figure payload loading: byte integrity, contract validation, and binding
//! resolution into a plot-ready model.
//!
//! The page's figure manifest names each payload with its exact size and
//! SHA-256; the loader re-verifies both before parsing, so a tampered or
//! truncated asset is rejected even though the transport already promised
//! integrity. Everything here is fail-closed: any mismatch rejects the
//! figure rather than presenting an approximation of sealed results.

use rspice_publication_contract::{
    AxisScale, ContractError, FigurePayload, PayloadRef, Scene, TraceTransform,
};
use sha2::{Digest as _, Sha256};

use crate::transform::{self, IncompatibleTransform};

/// Why a fetched figure payload was rejected.
#[derive(Debug, thiserror::Error)]
pub enum PayloadError {
    #[error("payload is {found} bytes but the manifest sealed {expected}")]
    Size { expected: u64, found: u64 },
    #[error("payload digest does not match the manifest seal")]
    Digest,
    #[error("payload belongs to figure {found}, not figure {expected}")]
    FigureId { expected: u64, found: u64 },
    #[error(transparent)]
    Contract(#[from] ContractError),
    #[error(transparent)]
    Transform(#[from] IncompatibleTransform),
    #[error("binding {binding} resolves {y_len} ordinates against {x_len} sweep points")]
    Misaligned {
        binding: usize,
        x_len: usize,
        y_len: usize,
    },
}

/// One plotted series, fully resolved: abscissa, transformed ordinates, and
/// the palette slot the page's static rendering used for the same curve.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedSeries {
    pub label: String,
    pub unit: String,
    /// Trace-palette slot. Bindings are colored in declaration order, which
    /// is the same order the renderer assigned `TraceSeries` roles in the
    /// sealed scene, so the hydrated plot matches the static page.
    pub series_index: u8,
    pub transform: TraceTransform,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

/// A hydratable plot resolved from one payload.
#[derive(Debug, Clone, PartialEq)]
pub struct HydratedPlot {
    pub x_scale: AxisScale,
    pub y_scale: AxisScale,
    pub x_label: String,
    pub y_label: String,
    pub series: Vec<ResolvedSeries>,
}

/// One loaded figure: the sealed scene, plus the live plot when the payload
/// carries hydration bindings.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadedFigure {
    pub figure_id: u64,
    pub scene: Scene,
    pub plot: Option<HydratedPlot>,
}

fn digest_matches(bytes: &[u8], expected_hex: &str) -> bool {
    let digest = Sha256::digest(bytes);
    if expected_hex.len() != digest.len() * 2 {
        return false;
    }
    digest
        .iter()
        .zip(expected_hex.as_bytes().chunks(2))
        .all(|(byte, pair)| {
            let hex = [
                char::from_digit(u32::from(byte >> 4), 16),
                char::from_digit(u32::from(byte & 0x0f), 16),
            ];
            hex[0] == Some(char::from(pair[0])) && hex[1] == Some(char::from(pair[1]))
        })
}

/// Verify, parse, validate, and resolve one fetched figure payload.
pub fn load_figure(
    bytes: &[u8],
    sealed: &PayloadRef,
    figure_id: u64,
) -> Result<LoadedFigure, PayloadError> {
    let found = bytes.len() as u64;
    if found != sealed.byte_len {
        return Err(PayloadError::Size {
            expected: sealed.byte_len,
            found,
        });
    }
    if !digest_matches(bytes, &sealed.sha256_hex) {
        return Err(PayloadError::Digest);
    }
    let payload = FigurePayload::from_canonical_bytes(bytes)?;
    if payload.figure_id != figure_id {
        return Err(PayloadError::FigureId {
            expected: figure_id,
            found: payload.figure_id,
        });
    }

    let plot = match &payload.hydration {
        None => None,
        Some(hydration) => {
            let mut series = Vec::with_capacity(hydration.bindings.len());
            for (index, binding) in hydration.bindings.iter().enumerate() {
                let dataset = payload
                    .datasets
                    .iter()
                    .find(|dataset| dataset.id == binding.dataset_id)
                    .ok_or(ContractError::DanglingReference {
                        figure_id,
                        kind: "dataset",
                        reference: binding.dataset_id,
                    })?;
                let trace = dataset.traces.get(binding.trace_index as usize).ok_or(
                    ContractError::DanglingReference {
                        figure_id,
                        kind: "trace",
                        reference: u64::from(binding.trace_index),
                    },
                )?;
                let y = transform::apply(binding.transform, &trace.values)?;
                let x: Vec<f64> = dataset
                    .sweep
                    .values_bits
                    .iter()
                    .copied()
                    .map(f64::from_bits)
                    .collect();
                if x.len() != y.len() {
                    return Err(PayloadError::Misaligned {
                        binding: index,
                        x_len: x.len(),
                        y_len: y.len(),
                    });
                }
                series.push(ResolvedSeries {
                    label: trace.label.clone(),
                    unit: trace.unit.clone(),
                    series_index: u8::try_from(index % crate::theme::TRACE_PALETTE_SIZE)
                        .expect("palette size fits u8"),
                    transform: binding.transform,
                    x,
                    y,
                });
            }
            Some(HydratedPlot {
                x_scale: hydration.x_scale,
                y_scale: hydration.y_scale,
                x_label: hydration.x_label.clone(),
                y_label: hydration.y_label.clone(),
                series,
            })
        }
    };

    Ok(LoadedFigure {
        figure_id: payload.figure_id,
        scene: payload.scene,
        plot,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_publication_contract::{
        Dataset, PlotHydration, PlotTraceBinding, SweepAxis, Trace, TraceValues,
    };

    fn scene() -> Scene {
        Scene {
            width_um: 100_000,
            height_um: 50_000,
            groups: Vec::new(),
        }
    }

    fn payload() -> FigurePayload {
        FigurePayload {
            schema_version: 1,
            figure_id: 7,
            scene: scene(),
            datasets: vec![Dataset {
                id: 3,
                analysis_id: 1,
                name: "tran1".to_owned(),
                variant: None,
                sweep: SweepAxis {
                    label: "time".to_owned(),
                    unit: "s".to_owned(),
                    values_bits: [0.0f64, 1e-6, 2e-6]
                        .iter()
                        .map(|value| value.to_bits())
                        .collect(),
                },
                traces: vec![Trace {
                    label: "V(out)".to_owned(),
                    unit: "V".to_owned(),
                    values: TraceValues::Real {
                        bits: [0.0f64, 2.5, 5.0]
                            .iter()
                            .map(|value| value.to_bits())
                            .collect(),
                    },
                }],
            }],
            hydration: Some(PlotHydration {
                x_scale: AxisScale::Linear,
                y_scale: AxisScale::Linear,
                x_label: "time".to_owned(),
                y_label: "V(out)".to_owned(),
                bindings: vec![PlotTraceBinding {
                    dataset_id: 3,
                    trace_index: 0,
                    transform: TraceTransform::Identity,
                }],
            }),
        }
    }

    fn sealed_bytes(payload: &FigurePayload) -> (Vec<u8>, PayloadRef) {
        let bytes = payload.canonical_bytes().expect("canonical payload bytes");
        let digest = Sha256::digest(&bytes);
        let mut sha256_hex = String::with_capacity(64);
        for byte in digest {
            use core::fmt::Write as _;
            let _ = write!(sha256_hex, "{byte:02x}");
        }
        let sealed = PayloadRef {
            path: "figures/7.json".to_owned(),
            sha256_hex,
            byte_len: bytes.len() as u64,
        };
        (bytes, sealed)
    }

    #[test]
    fn sealed_payload_round_trips_into_a_resolved_plot() {
        let payload = payload();
        let (bytes, sealed) = sealed_bytes(&payload);
        let figure = load_figure(&bytes, &sealed, 7).expect("load sealed figure");
        assert_eq!(figure.figure_id, 7);
        let plot = figure.plot.expect("hydrated plot");
        assert_eq!(plot.series.len(), 1);
        assert_eq!(plot.series[0].label, "V(out)");
        assert_eq!(plot.series[0].series_index, 0);
        assert_eq!(plot.series[0].x, vec![0.0, 1e-6, 2e-6]);
        assert_eq!(plot.series[0].y, vec![0.0, 2.5, 5.0]);
    }

    #[test]
    fn tampered_bytes_are_rejected_by_the_manifest_seal() {
        let payload = payload();
        let (mut bytes, sealed) = sealed_bytes(&payload);
        let last = bytes.len() - 1;
        bytes[last] ^= 0x01;
        assert!(matches!(
            load_figure(&bytes, &sealed, 7),
            Err(PayloadError::Digest)
        ));
    }

    #[test]
    fn truncated_bytes_are_rejected_by_length_before_hashing() {
        let payload = payload();
        let (bytes, sealed) = sealed_bytes(&payload);
        assert!(matches!(
            load_figure(&bytes[..bytes.len() - 1], &sealed, 7),
            Err(PayloadError::Size { .. })
        ));
    }

    #[test]
    fn a_payload_for_another_figure_is_rejected() {
        let payload = payload();
        let (bytes, sealed) = sealed_bytes(&payload);
        assert!(matches!(
            load_figure(&bytes, &sealed, 8),
            Err(PayloadError::FigureId {
                expected: 8,
                found: 7
            })
        ));
    }

    #[test]
    fn an_incompatible_transform_rejects_the_figure() {
        let mut payload = payload();
        payload.hydration.as_mut().expect("hydration").bindings[0].transform =
            TraceTransform::ImaginaryPart;
        let (bytes, sealed) = sealed_bytes(&payload);
        assert!(matches!(
            load_figure(&bytes, &sealed, 7),
            Err(PayloadError::Transform(_))
        ));
    }

    #[test]
    fn schematic_payloads_load_without_a_plot() {
        let mut payload = payload();
        payload.hydration = None;
        payload.datasets.clear();
        let (bytes, sealed) = sealed_bytes(&payload);
        let figure = load_figure(&bytes, &sealed, 7).expect("schematic figure");
        assert!(figure.plot.is_none());
    }
}
