//! The one writer of a `.fft` card, and the request it writes.
//!
//! A recorded FFT is evaluated by the engine inside the transient that carries
//! its card, so the card text is the whole request: nothing here computes a
//! window, a transform or a normalisation. Two ends have to agree on which
//! returned spectrum belongs to which analysis, and they agree by going
//! through core's parser and this one writer — never by position and never by
//! authored spelling.

use rspice_core::netlist::{FftAnalysis, FftOutput, FftWindow};
use rspice_core::numerics::rustfft_qualification::MAX_QUALIFIED_RUSTFFT_LENGTH;
use rspice_results::fft::fft_format_from_core;
use serde::{Deserialize, Serialize};

/// The `.FFT FORMAT=` selection, or `None` for the mode's own default.
///
/// Spelled with the retained evidence's own enum rather than a second
/// two-variant copy: the keyword a card is written with and the keyword a
/// result reports have to be the same table.
pub use rspice_results::fft::FftSpectrumFormatEvidence as FftFormatChoice;

/// Every window the engine implements, in the order the form offers them.
///
/// The array and [`window_keyword`] are the only two places a window is named
/// in this crate. `window_keyword` has no wildcard arm, so a window added to
/// `rspice_core::netlist::FftWindow` fails this crate's build instead of
/// quietly becoming unofferable.
pub const FFT_WINDOWS: [FftWindow; 16] = [
    FftWindow::Rectangular,
    FftWindow::Bartlett,
    FftWindow::BartlettHann,
    FftWindow::Hamming,
    FftWindow::Hann,
    FftWindow::Blackman67Db,
    FftWindow::Blackman,
    FftWindow::BlackmanHarris,
    FftWindow::Nuttall,
    FftWindow::HalfCycleSine,
    FftWindow::HalfCycleSine3,
    FftWindow::HalfCycleSine6,
    FftWindow::Cosine2,
    FftWindow::Cosine4,
    FftWindow::Gaussian,
    FftWindow::Kaiser,
];

/// The canonical `.FFT WINDOW=` keyword for one engine window.
#[must_use]
pub const fn window_keyword(window: FftWindow) -> &'static str {
    match window {
        FftWindow::Rectangular => "RECT",
        FftWindow::Bartlett => "BART",
        FftWindow::BartlettHann => "BARTLETTHANN",
        FftWindow::Hamming => "HAMM",
        FftWindow::Hann => "HANN",
        FftWindow::Blackman67Db => "BLACK",
        FftWindow::Blackman => "BLACKMAN",
        FftWindow::BlackmanHarris => "HARRIS",
        FftWindow::Nuttall => "NUTTALL",
        FftWindow::HalfCycleSine => "HALFCYCLESINE",
        FftWindow::HalfCycleSine3 => "HALFCYCLESINE3",
        FftWindow::HalfCycleSine6 => "HALFCYCLESINE6",
        FftWindow::Cosine2 => "COSINE2",
        FftWindow::Cosine4 => "COSINE4",
        FftWindow::Gaussian => "GAUSS",
        FftWindow::Kaiser => "KAISER",
    }
}

/// Every transform length the engine accepts, as the card spells them.
///
/// The engine's rule is a power of two in `4 ..= MAX_QUALIFIED_RUSTFFT_LENGTH`;
/// offering exactly those makes an unrepresentable length unauthorable.
#[must_use]
pub fn fft_point_counts() -> Vec<usize> {
    let mut counts = Vec::new();
    let mut points = 4usize;
    while points <= MAX_QUALIFIED_RUSTFFT_LENGTH {
        counts.push(points);
        points *= 2;
    }
    counts
}

/// The default transform length, which is also the engine's own.
pub const FFT_DEFAULT_POINTS: usize = FftAnalysis::DEFAULT_POINTS;

/// One authored `.FFT` request, in the engine's own field set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FftRequest {
    /// Probe such as `V(out)`, or a braced expression including its braces.
    pub output: String,
    #[serde(default)]
    pub start: Option<f64>,
    #[serde(default)]
    pub stop: Option<f64>,
    pub points: usize,
    #[serde(default)]
    pub format: Option<FftFormatChoice>,
    /// The canonical window keyword, as [`window_keyword`] spells it.
    pub window: String,
    /// HSPICE `ALFA`, used by the Gaussian and Kaiser windows.
    #[serde(default)]
    pub alfa: Option<f64>,
    #[serde(default)]
    pub fundamental: Option<f64>,
    #[serde(default)]
    pub fmin: Option<f64>,
    #[serde(default)]
    pub fmax: Option<f64>,
}

impl Default for FftRequest {
    fn default() -> Self {
        Self {
            output: "V(out)".to_owned(),
            start: None,
            stop: None,
            points: FFT_DEFAULT_POINTS,
            format: None,
            window: window_keyword(FftWindow::Rectangular).to_owned(),
            alfa: None,
            fundamental: None,
            fmin: None,
            fmax: None,
        }
    }
}

impl FftRequest {
    /// Read a request back out of the card core parsed.
    ///
    /// The window is taken from the parsed variant rather than the authored
    /// spelling, so `WINDOW=RECTANGULAR` and `WINDOW=RECT` are one request.
    #[must_use]
    pub fn from_core(analysis: &FftAnalysis) -> Self {
        Self {
            output: match &analysis.output {
                FftOutput::Probe(probe) => probe.clone(),
                FftOutput::Expression(expression) => format!("{{{expression}}}"),
            },
            start: analysis.start,
            stop: analysis.stop,
            points: analysis.points,
            format: analysis.format.map(fft_format_from_core),
            window: window_keyword(analysis.window).to_owned(),
            alfa: (analysis.alpha != FftAnalysis::DEFAULT_ALPHA).then_some(analysis.alpha),
            fundamental: analysis.fundamental_frequency,
            fmin: analysis.minimum_frequency,
            fmax: analysis.maximum_frequency,
        }
    }

    /// The card this request executes as. The only writer of `.fft`.
    ///
    /// Length and window are always written, because they decide the transform
    /// and the identity of the spectrum. Every other qualifier is written only
    /// when it was authored, so an unauthored `STOP` keeps the engine's own
    /// default of the transient's stop time rather than freezing a number the
    /// author never stated.
    #[must_use]
    pub fn to_card(&self) -> String {
        let mut card = format!(
            ".fft {} NP={} WINDOW={}",
            self.output.trim(),
            self.points,
            self.window
        );
        if let Some(start) = self.start {
            card.push_str(&format!(" START={start}"));
        }
        if let Some(stop) = self.stop {
            card.push_str(&format!(" STOP={stop}"));
        }
        if let Some(format) = self.format {
            card.push_str(&format!(" FORMAT={}", format.keyword()));
        }
        if let Some(alfa) = self.alfa {
            card.push_str(&format!(" ALFA={alfa}"));
        }
        if let Some(fundamental) = self.fundamental {
            card.push_str(&format!(" FREQ={fundamental}"));
        }
        if let Some(fmin) = self.fmin {
            card.push_str(&format!(" FMIN={fmin}"));
        }
        if let Some(fmax) = self.fmax {
            card.push_str(&format!(" FMAX={fmax}"));
        }
        card
    }

    /// The key that pairs this request with the spectrum the engine returned.
    ///
    /// Both ends of the pairing derive it the same way — parse the card with
    /// core, then write the parsed request back out — so the parser's own
    /// normalisation of the probe spelling and the transform length is applied
    /// to both. Position is never the key: PVT expansion reorders tasks, and
    /// two instances with identical requests select equal spectra.
    pub fn engine_key(&self) -> Result<String, String> {
        let deck = format!(
            "rspice recorded fft request key\nR1 nkey 0 1k\n.tran 1u 1m\n{}\n.end\n",
            self.to_card()
        );
        let netlist = rspice_core::Netlist::parse(&deck).map_err(|error| {
            format!("the FFT card this analysis writes is not readable: {error}")
        })?;
        let [analysis] = netlist.fft_analyses.as_slice() else {
            return Err(
                "the FFT card this analysis writes did not read back as one request".to_owned(),
            );
        };
        Ok(Self::from_core(analysis).to_card())
    }

    /// Refuse, in the engine's own words, what the engine refuses.
    ///
    /// The transient-relative rule — the engine's `STOP {stop} exceeds
    /// transient stop time {tstop}` — is not here: it is a fact about the pair
    /// and is checked by the dependency contract, before the run rather than
    /// inside it.
    pub fn validate(&self) -> Result<(), String> {
        if self.output.trim().is_empty() {
            return Err(
                ".FFT requires one output probe or braced expression as its first field".to_owned(),
            );
        }
        if self.points < 4
            || !self.points.is_power_of_two()
            || self.points > MAX_QUALIFIED_RUSTFFT_LENGTH
        {
            return Err(format!(
                "NP must be a power of two no smaller than 4, found {}",
                self.points
            ));
        }
        if FFT_WINDOWS
            .into_iter()
            .all(|window| window_keyword(window) != self.window)
        {
            return Err(format!("Invalid WINDOW type {} on .FFT line", self.window));
        }
        let start = self.start.unwrap_or(0.0);
        if !start.is_finite() || start < 0.0 {
            return Err(format!(
                "START must be finite and non-negative, found {start}"
            ));
        }
        if let Some(stop) = self.stop
            && (!stop.is_finite() || stop <= start)
        {
            return Err(format!(
                "STOP must be finite and greater than START ({start}), found {stop}"
            ));
        }
        for (name, value, allow_zero) in [
            ("FREQ", self.fundamental, false),
            ("FMIN", self.fmin, true),
            ("FMAX", self.fmax, false),
        ] {
            if let Some(value) = value
                && (!value.is_finite() || value < 0.0 || (!allow_zero && value == 0.0))
            {
                return Err(format!(
                    "{name} must be a {}finite frequency, found {value}",
                    if allow_zero {
                        "non-negative "
                    } else {
                        "positive "
                    }
                ));
            }
        }
        if let (Some(minimum), Some(maximum)) = (self.fmin, self.fmax)
            && minimum > maximum
        {
            return Err(format!("FMIN {minimum} exceeds FMAX {maximum}"));
        }
        if let Some(alfa) = self.alfa
            && (!alfa.is_finite() || !(1.0..=20.0).contains(&alfa))
        {
            return Err(format!(
                ".FFT ALFA must be finite and between 1 and 20, found {alfa}"
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_one(card: &str) -> FftAnalysis {
        let deck = format!("recorded fft card\nR1 a b 1k\n.tran 1u 10m\n{card}\n.end\n");
        let netlist = rspice_core::Netlist::parse(&deck).expect("the written card parses in core");
        netlist
            .fft_analyses
            .into_iter()
            .next()
            .expect("one .fft card")
    }

    #[test]
    fn the_fft_card_carries_its_window_and_point_count() {
        let mut request = FftRequest {
            output: "V(out)".to_owned(),
            points: 256,
            window: "HANN".to_owned(),
            ..FftRequest::default()
        };
        assert_eq!(request.to_card(), ".fft V(out) NP=256 WINDOW=HANN");
        let parsed = parse_one(&request.to_card());
        assert_eq!(parsed.points, 256);
        assert_eq!(parsed.window, FftWindow::Hann);
        assert_eq!(parsed.window_name, "HANN");
        // An unauthored field writes no keyword, so the engine keeps its own
        // default rather than a number the author never stated.
        assert_eq!(parsed.start, None);
        assert_eq!(parsed.stop, None);
        assert_eq!(parsed.format, None);
        assert_eq!(parsed.alpha, FftAnalysis::DEFAULT_ALPHA);
        assert_eq!(parsed.fundamental_frequency, None);
        assert_eq!(parsed.minimum_frequency, None);
        assert_eq!(parsed.maximum_frequency, None);

        request.start = Some(0.001);
        request.stop = Some(0.008);
        request.format = Some(FftFormatChoice::Unnormalized);
        request.alfa = Some(4.0);
        request.fundamental = Some(1000.0);
        request.fmin = Some(500.0);
        request.fmax = Some(9000.0);
        assert_eq!(
            request.to_card(),
            ".fft V(out) NP=256 WINDOW=HANN START=0.001 STOP=0.008 FORMAT=UNORM ALFA=4 \
             FREQ=1000 FMIN=500 FMAX=9000"
        );
        let parsed = parse_one(&request.to_card());
        assert_eq!(parsed.start, Some(0.001));
        assert_eq!(parsed.stop, Some(0.008));
        assert_eq!(
            parsed.format,
            Some(rspice_core::netlist::FftFormat::Unnormalized)
        );
        assert_eq!(parsed.alpha, 4.0);
        assert_eq!(parsed.fundamental_frequency, Some(1000.0));
        assert_eq!(parsed.minimum_frequency, Some(500.0));
        assert_eq!(parsed.maximum_frequency, Some(9000.0));
        // The lexer canonicalizes identifiers, so the probe reads back
        // uppercased; every other field is the one that was authored.
        assert_eq!(
            FftRequest::from_core(&parsed),
            FftRequest {
                output: "V(OUT)".to_owned(),
                ..request
            }
        );
    }

    #[test]
    fn every_window_the_form_offers_parses_to_the_engine_window_it_names() {
        let mut seen = std::collections::BTreeSet::new();
        for window in FFT_WINDOWS {
            let keyword = window_keyword(window);
            assert!(seen.insert(keyword), "{keyword} is offered twice");
            let request = FftRequest {
                window: keyword.to_owned(),
                ..FftRequest::default()
            };
            let parsed = parse_one(&request.to_card());
            assert_eq!(parsed.window, window, "{keyword}");
            assert_eq!(parsed.window_name, keyword);
            assert_eq!(FftRequest::from_core(&parsed).window, keyword);
        }
        // The offered set is the whole enum: `window_keyword` has no wildcard,
        // so a new engine window fails to compile, and this count is what stops
        // one from being added to the enum and left out of the array.
        assert_eq!(seen.len(), 16);
    }

    #[test]
    fn gaussian_and_kaiser_requests_round_trip_their_alfa() {
        for (window, alfa) in [("GAUSS", 4.0), ("KAISER", 12.0)] {
            let request = FftRequest {
                output: "V(OUT)".to_owned(),
                window: window.to_owned(),
                alfa: Some(alfa),
                ..FftRequest::default()
            };
            let parsed = parse_one(&request.to_card());
            let expected_window = if window == "GAUSS" {
                FftWindow::Gaussian
            } else {
                FftWindow::Kaiser
            };
            assert_eq!(parsed.window, expected_window);
            assert_eq!(parsed.alpha, alfa);
            assert_eq!(FftRequest::from_core(&parsed), request);
        }
    }

    #[test]
    fn the_point_count_choices_end_at_the_engine_s_qualified_transform_length() {
        let counts = fft_point_counts();
        assert_eq!(counts.first().copied(), Some(4));
        assert_eq!(counts.last().copied(), Some(MAX_QUALIFIED_RUSTFFT_LENGTH));
        assert!(counts.contains(&FFT_DEFAULT_POINTS));
        assert!(
            counts
                .iter()
                .all(|points| points.is_power_of_two() && *points >= 4)
        );
        assert_eq!(counts.len(), 19);
        for points in [3usize, 100, MAX_QUALIFIED_RUSTFFT_LENGTH * 2] {
            let request = FftRequest {
                points,
                ..FftRequest::default()
            };
            assert!(request.validate().is_err(), "{points}");
        }
    }

    #[test]
    fn the_form_refuses_what_the_engine_refuses_in_its_words() {
        let base = FftRequest {
            output: "V(out)".to_owned(),
            points: 256,
            ..FftRequest::default()
        };
        let cases: [(FftRequest, &str); 5] = [
            (
                FftRequest {
                    start: Some(-1.0),
                    ..base.clone()
                },
                "START must be finite and non-negative, found -1",
            ),
            (
                FftRequest {
                    start: Some(0.002),
                    stop: Some(0.001),
                    ..base.clone()
                },
                "STOP must be finite and greater than START (0.002), found 0.001",
            ),
            (
                FftRequest {
                    fundamental: Some(0.0),
                    ..base.clone()
                },
                "FREQ must be a positive finite frequency, found 0",
            ),
            (
                FftRequest {
                    fmax: Some(0.0),
                    ..base.clone()
                },
                "FMAX must be a positive finite frequency, found 0",
            ),
            (
                FftRequest {
                    fmin: Some(900.0),
                    fmax: Some(100.0),
                    ..base.clone()
                },
                "FMIN 900 exceeds FMAX 100",
            ),
        ];
        for (request, sentence) in cases {
            assert_eq!(request.validate().err().as_deref(), Some(sentence));
        }

        // The same sentences, from the engine itself, for the same requests.
        // The engine prefixes the request ordinal; the rest must be identical.
        for request in [
            FftRequest {
                start: Some(0.002),
                stop: Some(0.001),
                ..base.clone()
            },
            FftRequest {
                fundamental: Some(0.0),
                ..base.clone()
            },
            FftRequest {
                fmin: Some(900.0),
                fmax: Some(100.0),
                ..base.clone()
            },
        ] {
            let expected = request.validate().expect_err("the form refuses it");
            let deck = format!(
                "recorded fft refusal\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.tran 1u 10m\n{}\n.end\n",
                request.to_card()
            );
            let netlist = rspice_core::Netlist::parse(&deck).expect("the card parses");
            let engine =
                rspice_core::engine::Engine::new(rspice_core::engine::SimulationConfig::default());
            let error = engine
                .run_tran(&netlist, 0.01, 1.0e-5)
                .expect_err("the engine refuses the same request");
            let message = error.to_string();
            assert!(
                message.ends_with(&expected),
                "engine said {message:?}, form says {expected:?}"
            );
        }
    }

    #[test]
    fn the_pairing_key_is_the_parsers_own_spelling_of_the_request() {
        let lower = FftRequest {
            output: "v(out)".to_owned(),
            points: 256,
            window: "RECTANGULAR".to_owned(),
            ..FftRequest::default()
        };
        // An unofferable spelling still reads back as the canonical keyword.
        let parsed = parse_one(&lower.to_card());
        assert_eq!(FftRequest::from_core(&parsed).window, "RECT");

        let upper = FftRequest {
            output: "V(OUT)".to_owned(),
            points: 256,
            window: "RECT".to_owned(),
            ..FftRequest::default()
        };
        assert_eq!(
            lower.engine_key().expect("lower-case probe keys"),
            upper.engine_key().expect("upper-case probe keys")
        );
        assert_eq!(
            upper.engine_key().expect("the key round-trips"),
            FftRequest::from_core(&parse_one(&upper.to_card())).to_card()
        );
    }
}
