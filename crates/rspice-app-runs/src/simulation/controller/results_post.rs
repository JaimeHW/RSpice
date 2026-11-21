//! Populating the post-run viewers.
//!
//! After a run completes, seeds each analysis viewer with the data it needs:
//! AC magnitude and phase and the viewers behind them. A transient's spectrum
//! and eye are not seeded here; each is prepared when its sheet is opened.

use super::*;
use crate::state::{RunEvent, SpecializedViewerCacheProvenance};

impl SimulationController {
    pub fn in_flight_specialized_viewer_provenance(
        &self,
        model: &Model,
    ) -> Option<SpecializedViewerCacheProvenance> {
        let run_sequence = self.current_run_id?;
        let run = model.simulation.retained.run_by_sequence(run_sequence)?;
        let source_instance_id = self.current_provenance.as_ref()?.source_instance_id();
        Some(SpecializedViewerCacheProvenance::for_prepared_analysis(
            run.dataset_id,
            source_instance_id,
        ))
    }

    pub fn populate_ac_post_views(
        &self,
        state: &mut impl RunHost,
        frequencies: &[f64],
        waveforms: &std::collections::HashMap<String, crate::simulation::SampledWaveform>,
    ) {
        let mut bode_data = rspice_results_ui::bode::BodeData::new();

        let mut names: Vec<_> = waveforms.keys().cloned().collect();
        names.sort();
        for name in names {
            let Some(waveform) = waveforms.get(&name) else {
                continue;
            };
            let Some(imag) = waveform.y_imag.as_ref() else {
                continue;
            };
            // The engine's own samples, held in memory until the run is stored.
            let Some(values) = waveform.y_values.in_memory() else {
                continue;
            };
            if values.len() != frequencies.len() || imag.len() != frequencies.len() {
                continue;
            }

            bode_data.add_response();
        }

        let owner = self.in_flight_specialized_viewer_provenance(state.model());
        state.apply_run_event(RunEvent::FrequencyResponseReplaced {
            owner,
            bode: bode_data,
        });
    }

    pub(super) fn fft_source_waveform_from_state<'a>(
        waveforms: &'a [crate::state::RetainedTrace],
        preferred_name: Option<&str>,
    ) -> Option<(String, &'a crate::state::RetainedTrace)> {
        // In the order the run saved them, never sorted: the fallback is the
        // first voltage the run saved, not whichever name sorts first.
        let candidates: Vec<&crate::state::RetainedTrace> = waveforms
            .iter()
            // Keep malformed or short authored traces selectable so the
            // typed preparation boundary can explain their exact defect.
            // Only a record with no evidence at all is not a candidate.
            .filter(|waveform| !waveform.x.is_empty() || !waveform.y.is_empty())
            .collect();

        if let Some(name) = preferred_name {
            if let Some(waveform) = candidates
                .iter()
                .copied()
                .find(|wf| wf.name == name || wf.name.eq_ignore_ascii_case(name))
            {
                return Some((waveform.name.clone(), waveform));
            }

            let preferred = Self::parse_fft_source_name(name);
            if let Some(waveform) = candidates
                .iter()
                .copied()
                .filter(|waveform| {
                    Self::parse_fft_source_name(&waveform.name).core == preferred.core
                })
                .min_by_key(|waveform| {
                    Self::fft_source_kind_rank(
                        preferred.kind,
                        Self::parse_fft_source_name(&waveform.name).kind,
                    )
                })
            {
                return Some((waveform.name.clone(), waveform));
            }
        }

        candidates
            .iter()
            .copied()
            .find(|waveform| {
                Self::parse_fft_source_name(&waveform.name).kind == FftSourceKind::Voltage
            })
            .or_else(|| candidates.first().copied())
            .map(|waveform| (waveform.name.clone(), waveform))
    }

    fn parse_fft_source_name(name: &str) -> ParsedFftSourceName {
        let trimmed = name.trim().trim_matches('|');
        if let Some(core) = trimmed.strip_prefix("V(").and_then(|s| s.strip_suffix(')')) {
            return ParsedFftSourceName {
                core: core.trim().to_ascii_lowercase(),
                kind: FftSourceKind::Voltage,
            };
        }
        if let Some(core) = trimmed.strip_prefix("I(").and_then(|s| s.strip_suffix(')')) {
            return ParsedFftSourceName {
                core: core.trim().to_ascii_lowercase(),
                kind: FftSourceKind::Current,
            };
        }
        ParsedFftSourceName {
            core: trimmed.trim().to_ascii_lowercase(),
            kind: FftSourceKind::Other,
        }
    }

    fn fft_source_kind_rank(preferred: FftSourceKind, candidate: FftSourceKind) -> i32 {
        match preferred {
            FftSourceKind::Voltage => {
                if candidate == FftSourceKind::Voltage {
                    0
                } else {
                    1
                }
            }
            FftSourceKind::Current => {
                if candidate == FftSourceKind::Current {
                    0
                } else {
                    1
                }
            }
            FftSourceKind::Other => match candidate {
                // For ambiguous untyped labels (e.g. "out"), prefer voltage
                // traces over current traces.
                FftSourceKind::Voltage => 0,
                FftSourceKind::Other => 1,
                FftSourceKind::Current => 2,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FftSourceKind {
    Voltage,
    Current,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedFftSourceName {
    core: String,
    kind: FftSourceKind,
}
