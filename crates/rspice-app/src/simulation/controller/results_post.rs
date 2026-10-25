//! Populating the post-run viewers.
//!
//! After a run completes, seeds each analysis viewer with the data it needs
//! — transient traces, AC magnitude and phase, and the specialized viewers
//! behind the analysis workspace.

use super::transient_post::{DerivedViewAvailability, LoadedDerivedView};
use super::*;
use crate::state::{RunEvent, SpecializedViewerCacheProvenance};

use rspice_simulation::results::STB_NYQUIST_CONTOUR_WAVEFORM;

/// What the locus is, named for the reader rather than for the transport.
pub const LOOP_GAIN_LOCUS_LABEL: &str = "L(jω)";

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

    pub fn populate_transient_post_views(
        &mut self,
        state: &mut impl RunHost,
        analysis: &crate::state::AnalysisResult,
    ) {
        self.transient_post.eye_loaded = None;
        self.transient_post.fft_loaded = None;
        if !analysis.success || analysis.analysis_type != crate::state::AnalysisType::Transient {
            state.apply_run_event(RunEvent::TransientResultReplaced);
            return;
        }

        let preferred_fft_source = state.viewer_reading().fft_source;
        let Some((waveform_key, waveform)) = Self::fft_source_waveform_from_state(
            &analysis.waveforms,
            preferred_fft_source.as_deref(),
        ) else {
            // A specialized viewer must never outlive the retained source it
            // claims to represent.  In particular, an analysis with no saved
            // transient outputs cannot keep an FFT derived from the engine's
            // discarded working set.
            state.apply_run_event(RunEvent::TransientResultReplaced);
            return;
        };
        // Eye folding retains its historical paired-prefix behavior. FFT
        // preparation receives the authored arrays whole so its strict shape
        // contract can diagnose a mismatch instead of inheriting truncation.
        let paired_count = waveform.x.len().min(waveform.y.len());
        let eye_time = &waveform.x[..paired_count];
        let eye_values = &waveform.y[..paired_count];
        let time = waveform.x.as_slice();
        let values = waveform.y.as_slice();

        state.apply_run_event(RunEvent::FftSourceChosen {
            source: waveform_key.clone(),
        });
        let input_options = state.fft_input_options(time);

        let provenance = self.in_flight_specialized_viewer_provenance(state.model());
        // Same builder as the lazy path in `transient_post`, at the same
        // reader-chosen bit period. Seeding the viewer eagerly must not mean
        // seeding it from a second, differently-behaved estimator.
        let timebase = provenance
            .map(|owner| state.eye_timebase_for(owner))
            .unwrap_or_default();
        let (eye_data, eye_provenance) = build_eye_from_waveform(eye_time, eye_values, timebase);
        state.apply_run_event(RunEvent::EyeFolded {
            owner: provenance,
            eye: eye_data,
            timebase: eye_provenance,
        });

        let input = rspice_results_ui::fft::prepare_fft_input_with_options(
            &waveform_key,
            time,
            values,
            input_options,
        );
        state.apply_run_event(RunEvent::FftInputPrepared {
            owner: provenance,
            input,
        });
        // The viewer holds a spectrum exactly when it accepted the input.
        let availability = if state.viewer_reading().fft_has_data {
            DerivedViewAvailability::Ready
        } else {
            DerivedViewAvailability::Unavailable
        };
        if let Some(analysis) = provenance {
            self.transient_post.fft_loaded = Some(LoadedDerivedView {
                analysis,
                availability,
            });
        }
    }

    /// The waveform a stability run retains its loop-gain contour under, when
    /// the analysis that just finished is one.
    ///
    /// The shared frequency-view population receives a projection of the typed
    /// stability result. Resolve its analysis kind from the prepared task so
    /// other frequency responses are not interpreted as loop gain.
    fn loop_gain_contour_name(&self) -> Option<&'static str> {
        let analysis_type = self
            .current_spec
            .as_ref()
            .map(|spec| self.spec_to_analysis_type(spec))
            .or_else(|| {
                self.current_config
                    .as_ref()
                    .map(|config| self.config_to_analysis_type(config))
            })?;
        (analysis_type == AnalysisType::Stb).then_some(STB_NYQUIST_CONTOUR_WAVEFORM)
    }

    pub fn populate_ac_post_views(
        &self,
        state: &mut impl RunHost,
        frequencies: &[f64],
        waveforms: &std::collections::HashMap<String, crate::simulation::SampledWaveform>,
    ) {
        let mut bode_data = rspice_results_ui::bode::BodeData::new();
        let mut loop_gain_locus = None;

        let mut names: Vec<_> = waveforms.keys().cloned().collect();
        names.sort();
        // The Nyquist sheet is a loop-gain instrument: its encirclements and
        // margins are the stability criterion applied to a return ratio. Only
        // a stability run measures one, and it retains exactly one contour for
        // it, so that contour is the only thing the sheet is given. Every
        // other complex response here — an AC node response, a harmonic
        // balance spectrum — is a perfectly good curve that is not a loop
        // gain, and drawing it there would attach a stability verdict to a
        // quantity that has none.
        let loop_gain = self
            .loop_gain_contour_name()
            .filter(|name| waveforms.contains_key(*name));
        for name in names {
            let Some(waveform) = waveforms.get(&name) else {
                continue;
            };
            let Some(imag) = waveform.y_imag.as_ref() else {
                continue;
            };
            if waveform.y_values.len() != frequencies.len() || imag.len() != frequencies.len() {
                continue;
            }

            bode_data.add_response();

            if loop_gain == Some(name.as_str()) {
                loop_gain_locus = Some(rspice_results_ui::nyquist::NyquistData::from_arrays(
                    LOOP_GAIN_LOCUS_LABEL,
                    frequencies,
                    &waveform.y_values,
                    imag,
                ));
            }
        }

        let owner = self.in_flight_specialized_viewer_provenance(state.model());
        state.apply_run_event(RunEvent::FrequencyResponseReplaced {
            owner,
            bode: bode_data,
            loop_gain: loop_gain_locus,
        });
    }

    pub(super) fn fft_source_waveform_from_state<'a>(
        waveforms: &'a [crate::state::RetainedTrace],
        preferred_name: Option<&str>,
    ) -> Option<(String, &'a crate::state::RetainedTrace)> {
        let mut candidates: Vec<&crate::state::RetainedTrace> = waveforms
            .iter()
            // Keep malformed or short authored traces selectable so the
            // typed preparation boundary can explain their exact defect.
            // Only a record with no evidence at all is not a candidate.
            .filter(|waveform| !waveform.x.is_empty() || !waveform.y.is_empty())
            .collect();
        candidates.sort_by(|a, b| a.name.cmp(&b.name));

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
            .into_iter()
            .next()
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
