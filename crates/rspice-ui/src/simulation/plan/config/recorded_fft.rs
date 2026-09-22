//! The draft of a recorded `.FFT` spectrum.
//!
//! Every field here is a field of the engine's own `.FFT` card, because the
//! card is the whole request: the engine evaluates it inside the transient
//! that carries it. There is no window list, no transform length table and no
//! normalisation policy of the Studio's own — one `const fn` over
//! `rspice_core::netlist::FftWindow` gives the keywords, so a window added to
//! the engine fails this crate's build rather than quietly becoming
//! unofferable.
//!
use serde::{Deserialize, Serialize};

use crate::simulation::config::{FFT_DEFAULT_POINTS, FftFormatChoice, FftRequest, window_keyword};

/// Raw form state for one `.fft` card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FftDraft {
    /// Probe or braced expression, exactly as typed.
    pub output: String,
    /// `START=`, empty for the engine's own default of the record's beginning.
    pub start: String,
    /// `STOP=`, empty for the engine's own default of the transient stop time.
    pub stop: String,
    /// `NP=`. Stored as the number, never as a chooser index.
    pub points: usize,
    /// `WINDOW=`, stored as the engine's canonical keyword.
    pub window: String,
    /// `ALFA=`, used by the Gaussian and Kaiser windows. Empty keeps the
    /// engine default of 3.
    pub alfa: String,
    /// `FORMAT=`: empty writes no keyword and keeps the mode's own default.
    pub format: String,
    /// `FREQ=`, the bin the metrics call the fundamental.
    pub fundamental: String,
    /// `FMIN=`.
    pub fmin: String,
    /// `FMAX=`.
    pub fmax: String,
}

impl Default for FftDraft {
    fn default() -> Self {
        Self {
            output: "V(out)".to_owned(),
            start: String::new(),
            stop: String::new(),
            points: FFT_DEFAULT_POINTS,
            window: window_keyword(rspice_core::netlist::FftWindow::Rectangular).to_owned(),
            alfa: String::new(),
            format: String::new(),
            fundamental: String::new(),
            fmin: String::new(),
            fmax: String::new(),
        }
    }
}

/// Read one optional authored quantity, or nothing when the field is empty.
fn optional_quantity(field: &str, label: &str) -> Result<Option<f64>, String> {
    let trimmed = field.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    crate::simulation::spice_value::parse_spice_value_checked(trimmed)
        .map(Some)
        .map_err(|error| format!("{label}: {error}"))
}

impl FftDraft {
    /// Project this draft onto the request the engine reads.
    pub fn to_request(&self) -> Result<FftRequest, String> {
        let format = match self.format.trim() {
            "" => None,
            "NORM" => Some(FftFormatChoice::Normalized),
            "UNORM" => Some(FftFormatChoice::Unnormalized),
            other => return Err(format!("Invalid FORMAT type {other} on .FFT line")),
        };
        let request = FftRequest {
            output: self.output.trim().to_owned(),
            start: optional_quantity(&self.start, "FFT START")?,
            stop: optional_quantity(&self.stop, "FFT STOP")?,
            points: self.points,
            format,
            window: self.window.trim().to_ascii_uppercase(),
            alfa: optional_quantity(&self.alfa, "FFT ALFA")?,
            fundamental: optional_quantity(&self.fundamental, "FFT FREQ")?,
            fmin: optional_quantity(&self.fmin, "FFT FMIN")?,
            fmax: optional_quantity(&self.fmax, "FFT FMAX")?,
        };
        request.validate()?;
        Ok(request)
    }

    /// The one-line summary the plan manager shows for this instance.
    #[must_use]
    pub fn summary(&self) -> String {
        match self.to_request() {
            Ok(request) => format!(
                "{} · {} points · {}",
                request.output, request.points, request.window
            ),
            Err(error) => error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::multi_run::AnalysisSpec;

    #[test]
    fn an_fft_draft_saved_before_a_field_existed_restores_with_engine_defaults() {
        // Every field carries a serde default, so a draft saved before a
        // control existed restores as the engine's own default rather than as
        // a zero somebody has to notice.
        let restored: FftDraft = serde_json::from_str("{}").expect("an empty draft restores");
        let defaults = FftDraft::default();
        assert_eq!(restored.output, defaults.output);
        assert_eq!(restored.points, FFT_DEFAULT_POINTS);
        assert_eq!(restored.window, "RECT");
        assert!(restored.alfa.is_empty());
        assert!(restored.format.is_empty());
        assert!(restored.start.is_empty() && restored.stop.is_empty());
        assert!(
            restored.fundamental.is_empty() && restored.fmin.is_empty() && restored.fmax.is_empty()
        );
        let request = restored.to_request().expect("the default draft is valid");
        assert_eq!(request.to_card(), ".fft V(out) NP=1024 WINDOW=RECT");

        let partial: FftDraft =
            serde_json::from_str(r#"{"points":256,"window":"HANN"}"#).expect("a partial draft");
        assert_eq!(partial.output, defaults.output);
        assert_eq!(partial.points, 256);
        assert_eq!(partial.window, "HANN");
    }

    #[test]
    fn an_fft_specification_restores_from_the_wire_with_engine_defaults() {
        // The wire twin: an `AnalysisSpec::Fft` sealed before an optional
        // qualifier was authorable restores with that qualifier unauthored,
        // which is what the engine reads as its own default.
        let wire = r#"{"Fft":{"request":{"output":"V(OUT)","points":256,"window":"RECT"}}}"#;
        let spec: AnalysisSpec = serde_json::from_str(wire).expect("the sealed spec restores");
        let AnalysisSpec::Fft { request } = &spec else {
            panic!("an FFT specification");
        };
        assert_eq!(request.start, None);
        assert_eq!(request.stop, None);
        assert_eq!(request.format, None);
        assert_eq!(request.alfa, None);
        assert_eq!(request.to_card(), ".fft V(OUT) NP=256 WINDOW=RECT");
        assert!(spec.validate().is_ok());
    }

    #[test]
    fn the_form_refuses_an_unreadable_quantity_where_it_is_typed() {
        let draft = FftDraft {
            stop: "eight".to_owned(),
            ..FftDraft::default()
        };
        let error = draft
            .to_request()
            .expect_err("an unreadable STOP is refused");
        assert!(error.starts_with("FFT STOP: "), "{error}");
        assert_eq!(draft.summary(), error);
    }

    #[test]
    fn the_plan_wires_gaussian_alfa_into_the_engine_request() {
        let draft = FftDraft {
            window: "GAUSS".to_owned(),
            alfa: "6".to_owned(),
            ..FftDraft::default()
        };
        let request = draft.to_request().expect("Gaussian ALFA is valid");
        assert_eq!(request.window, "GAUSS");
        assert_eq!(request.alfa, Some(6.0));
        assert_eq!(request.to_card(), ".fft V(out) NP=1024 WINDOW=GAUSS ALFA=6");
    }
}
