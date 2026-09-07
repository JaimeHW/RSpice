//! The analysis kinds, and their SPICE spellings.

/// Analysis type identifier for viewer selection and result organization.
///
/// Each analysis type maps to a specialized viewer:
/// - DcOp → Operating Point Table
/// - DcSweep → DC Sweep Plot
/// - Ac → Bode Plot (magnitude/phase)
/// - Transient → Time-domain Waveform Viewer
/// - Noise → Noise Spectrum Plot
/// - PoleZero → S-plane Pole-Zero Diagram
/// - Sensitivity → Parameter Sensitivity Bar Chart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AnalysisType {
    /// DC operating point analysis - node voltages and branch currents
    DcOp,
    /// DC sweep analysis - parameter sweep with DC solution at each point
    DcSweep,
    /// AC small-signal frequency response analysis
    Ac,
    /// Distortion analysis
    Disto,
    /// Time-domain transient analysis
    Transient,
    /// Noise analysis - noise spectral density vs frequency
    Noise,
    /// Pole-zero analysis - transfer function poles and zeros
    PoleZero,
    /// Transfer function analysis
    Tf,
    /// Sensitivity analysis - output sensitivity to parameters
    Sensitivity,
    /// Periodic AC analysis
    Pac,
    /// Periodic noise analysis
    Pnoise,
    /// Periodic transfer function analysis
    Pxf,
    /// Periodic stability analysis
    Pstb,
    /// Stability analysis
    Stb,
    /// Monte Carlo statistical analysis
    MonteCarlo,
    /// Parametric sweep analysis
    Parametric,
    /// Corner sweep analysis
    Corner,
    /// Reliability aging analysis
    Reliability,
    /// Optimization analysis
    Optimization,
    /// Safety/SOA analysis
    Soa,
    /// S-parameter analysis
    SParameter,
    /// Envelope analysis
    Envelope,
    /// Fourier analysis
    Fourier,
    /// Harmonic Balance analysis for RF circuits
    HarmonicBalance,
    /// Periodic Steady State analysis
    Pss,
    Qpss,
    Hbsp,
    Hbnoise,
    Psp,
    Qpac,
    Qpnoise,
    Qpxf,
    TransientNoise,
    DcMismatch,
}

impl AnalysisType {
    /// Whether retained samples use time as their physical abscissa and can
    /// therefore share the exact waveform workspace.
    pub const fn is_time_domain(self) -> bool {
        matches!(
            self,
            Self::Transient | Self::TransientNoise | Self::Pss | Self::Envelope | Self::Soa
        )
    }

    /// Whether retained samples from this analysis describe a swept complex
    /// frequency response that the shared magnitude/phase workspace can
    /// present. PSTB is deliberately absent: its current retained abscissa is
    /// a Floquet mode index, not frequency.
    pub const fn is_bode_response(self) -> bool {
        matches!(
            self,
            Self::Ac | Self::Pac | Self::Pxf | Self::Stb | Self::Qpac | Self::Qpxf
        )
    }

    /// Whether the retained response uses the standard complex-to-
    /// magnitude/phase projection. DISTO retains exact linear Volterra
    /// response and product-ratio phasors, so its dB/dBc curves are also a
    /// presentation projection. STB already retains loop gain in dB and phase
    /// in degrees, so applying the projection again would double-scale it.
    pub const fn uses_complex_bode_projection(self) -> bool {
        matches!(self, Self::Disto) || (self.is_bode_response() && !matches!(self, Self::Stb))
    }

    /// Whether this is a frequency-curve family that uses the Bode sheet's
    /// logarithmic X axis without participating in stability-margin logic.
    pub const fn is_raw_frequency_curve(self) -> bool {
        matches!(self, Self::Disto)
    }

    /// Get SPICE directive keyword for this analysis type.
    pub fn spice_command(&self) -> &'static str {
        match self {
            AnalysisType::DcOp => ".op",
            AnalysisType::DcSweep => ".dc",
            AnalysisType::Ac => ".ac",
            AnalysisType::Disto => ".disto",
            AnalysisType::Transient => ".tran",
            AnalysisType::Noise => ".noise",
            AnalysisType::PoleZero => ".pz",
            AnalysisType::Tf => ".tf",
            AnalysisType::Sensitivity => ".sens",
            AnalysisType::Pac => ".pac",
            AnalysisType::Pnoise => ".pnoise",
            AnalysisType::Pxf => ".pxf",
            AnalysisType::Pstb => ".pstb",
            AnalysisType::Stb => ".stb",
            AnalysisType::MonteCarlo => ".mc",
            AnalysisType::Parametric => ".step",
            AnalysisType::Corner => ".step",
            AnalysisType::Reliability => ".reliability",
            AnalysisType::Optimization => ".opt",
            AnalysisType::Soa => ".soa",
            AnalysisType::SParameter => ".sp",
            AnalysisType::Envelope => ".envlp",
            AnalysisType::Fourier => ".four",
            AnalysisType::HarmonicBalance => ".hb",
            AnalysisType::Pss => ".pss",
            AnalysisType::Qpss => ".qpss",
            AnalysisType::Hbsp => ".hbsp",
            AnalysisType::Hbnoise => ".hbnoise",
            AnalysisType::Psp => ".psp",
            AnalysisType::Qpac => ".qpac",
            AnalysisType::Qpnoise => ".qpnoise",
            AnalysisType::Qpxf => ".qpxf",
            AnalysisType::TransientNoise => ".tnoise",
            AnalysisType::DcMismatch => ".dcmatch",
        }
    }

    /// Get human-readable display name for this analysis type
    pub fn display_name(&self) -> &'static str {
        match self {
            AnalysisType::DcOp => "DC Operating Point",
            AnalysisType::DcSweep => "DC Sweep",
            AnalysisType::Ac => "AC Analysis",
            AnalysisType::Disto => "DISTO",
            AnalysisType::Transient => "Transient",
            AnalysisType::Noise => "Noise",
            AnalysisType::PoleZero => "Pole-Zero",
            AnalysisType::Tf => "Transfer Function",
            AnalysisType::Sensitivity => "Sensitivity",
            AnalysisType::Pac => "PAC",
            AnalysisType::Pnoise => "PNoise",
            AnalysisType::Pxf => "PXF",
            AnalysisType::Pstb => "PSTB",
            AnalysisType::Stb => "STB",
            AnalysisType::MonteCarlo => "Monte Carlo",
            AnalysisType::Parametric => "Parametric Sweep",
            AnalysisType::Corner => "Corner Sweep",
            AnalysisType::Reliability => "Reliability",
            AnalysisType::Optimization => "Optimization",
            AnalysisType::Soa => "Safety (SOA)",
            AnalysisType::SParameter => "S-Parameter",
            AnalysisType::Envelope => "Envelope",
            AnalysisType::Fourier => "Fourier",
            AnalysisType::HarmonicBalance => "Harmonic Balance",
            AnalysisType::Pss => "PSS",
            AnalysisType::Qpss => "QPSS",
            AnalysisType::Hbsp => "Large-Signal S-Parameters",
            AnalysisType::Hbnoise => "Harmonic-Balance Noise",
            AnalysisType::Psp => "Periodic S-Parameters",
            AnalysisType::Qpac => "QPAC",
            AnalysisType::Qpnoise => "Quasi-Periodic Noise",
            AnalysisType::Qpxf => "Quasi-Periodic Transfer",
            AnalysisType::TransientNoise => "Transient Noise",
            AnalysisType::DcMismatch => "DC Mismatch Contribution",
        }
    }

    /// Get short icon-friendly label
    pub fn short_label(&self) -> &'static str {
        match self {
            AnalysisType::DcOp => "DC",
            AnalysisType::DcSweep => "DCS",
            AnalysisType::Ac => "AC",
            AnalysisType::Disto => "DIST",
            AnalysisType::Transient => "TR",
            AnalysisType::Noise => "NS",
            AnalysisType::PoleZero => "PZ",
            AnalysisType::Tf => "TF",
            AnalysisType::Sensitivity => "SN",
            AnalysisType::Pac => "PAC",
            AnalysisType::Pnoise => "PN",
            AnalysisType::Pxf => "PXF",
            AnalysisType::Pstb => "PSTB",
            AnalysisType::Stb => "STB",
            AnalysisType::MonteCarlo => "MC",
            AnalysisType::Parametric => "PAR",
            AnalysisType::Corner => "CRN",
            AnalysisType::Reliability => "REL",
            AnalysisType::Optimization => "OPT",
            AnalysisType::Soa => "SOA",
            AnalysisType::SParameter => "SP",
            AnalysisType::Envelope => "ENV",
            AnalysisType::Fourier => "FOU",
            AnalysisType::HarmonicBalance => "HB",
            AnalysisType::Pss => "PSS",
            AnalysisType::Qpss => "QPSS",
            AnalysisType::Hbsp => "HSP",
            AnalysisType::Hbnoise => "HN",
            AnalysisType::Psp => "PSP",
            AnalysisType::Qpac => "QPAC",
            AnalysisType::Qpnoise => "QPN",
            AnalysisType::Qpxf => "QPXF",
            AnalysisType::TransientNoise => "TN",
            AnalysisType::DcMismatch => "DM",
        }
    }

    /// Get axis labels and units for this analysis type
    ///
    /// Returns (x_axis_label, x_axis_unit, y_axis_label, y_axis_unit)
    pub fn axis_info(&self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            AnalysisType::Transient
            | AnalysisType::Pss
            | AnalysisType::Envelope
            | AnalysisType::Soa => ("Time", "s", "Voltage", "V"),
            AnalysisType::TransientNoise => ("Time", "s", "Voltage", "V"),
            AnalysisType::Ac
            | AnalysisType::Disto
            | AnalysisType::Tf
            | AnalysisType::Stb
            | AnalysisType::SParameter
            | AnalysisType::HarmonicBalance
            | AnalysisType::Fourier => ("Frequency", "Hz", "Magnitude", "V"),
            // A periodic small-signal sweep is not a frequency the circuit is
            // driven at. Its abscissa is the baseband offset the conversion
            // matrix is indexed by; the drive sits at
            // `offset + INPUTSIDEBAND * f0` and the response at
            // `OUTSIDEBAND * f0 + offset`, which the run publishes as its own
            // curve. Calling the axis "Frequency" invited a reader to take it
            // for the first of those three.
            //
            // "Offset frequency" is what core calls the quantity
            // (`PacSidebandData::frequency_offset`, with
            // `absolute_frequency = sideband * f0 + frequency_offset`), and it
            // is true of every card in the family. "Translated frequency" is
            // not: the translated frequency is `offset + n*f0`, one of the
            // other two numbers.
            AnalysisType::Pac | AnalysisType::Pxf | AnalysisType::Qpac | AnalysisType::Qpxf => {
                ("Offset Frequency", "Hz", "Magnitude", "V")
            }
            AnalysisType::Qpss | AnalysisType::Hbsp | AnalysisType::Psp => {
                ("Frequency", "Hz", "Magnitude", "V")
            }
            AnalysisType::Pstb => ("Mode", "index", "Stability metric", ""),
            // Plain `.NOISE` sweeps a real absolute frequency about a DC
            // operating point; it is the periodic members of the family whose
            // abscissa is an offset from a carrier.
            AnalysisType::Noise => ("Frequency", "Hz", "Noise", "V^2/Hz"),
            AnalysisType::Pnoise | AnalysisType::Hbnoise | AnalysisType::Qpnoise => {
                ("Offset Frequency", "Hz", "Noise", "V^2/Hz")
            }
            AnalysisType::DcSweep => ("Voltage", "V", "Voltage", "V"),
            AnalysisType::DcOp => ("", "", "Voltage", "V"),
            AnalysisType::PoleZero => ("Real", "", "Imaginary", ""),
            AnalysisType::Sensitivity => ("Parameter", "", "Sensitivity", ""),
            AnalysisType::MonteCarlo => ("Value", "", "Count", "count"),
            AnalysisType::Parametric => ("Sweep", "", "Voltage", "V"),
            AnalysisType::Corner => ("Temperature", "C", "Voltage", "V"),
            AnalysisType::Reliability => ("Lifetime", "year", "Shift", ""),
            AnalysisType::Optimization => ("Iteration", "iter", "Cost", "cost"),
            AnalysisType::DcMismatch => ("Parameter", "", "Contribution", "%"),
        }
    }
}

impl std::fmt::Display for AnalysisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod axis_tests {
    use super::AnalysisType;

    /// A periodic small-signal abscissa is the swept baseband offset, and the
    /// navigator's sweep caption reads it from here. An absolute frequency is
    /// a different number: the drive sits at `offset + INPUTSIDEBAND * f0`,
    /// and the converted response, which the run publishes as its own curve,
    /// at `OUTSIDEBAND * f0 + offset`.
    ///
    /// Everything here is display text. A waveform export's first column is an
    /// identifier a reader keys on and it is spelled independently, in
    /// `waveform_export::axis_signal_for_analysis_type`, because retitling an
    /// axis once moved that identifier and lost the coordinate's unit.
    #[test]
    fn every_periodic_small_signal_axis_is_named_the_offset_it_holds() {
        for periodic in [
            AnalysisType::Pac,
            AnalysisType::Pxf,
            AnalysisType::Qpac,
            AnalysisType::Qpxf,
        ] {
            assert_eq!(
                periodic.axis_info(),
                ("Offset Frequency", "Hz", "Magnitude", "V"),
                "{periodic:?}"
            );
        }
        assert_eq!(
            AnalysisType::Ac.axis_info(),
            ("Frequency", "Hz", "Magnitude", "V"),
            "an .AC sweep really is the frequency the circuit is driven at"
        );
        for absolute in [
            AnalysisType::SParameter,
            AnalysisType::Psp,
            AnalysisType::Hbsp,
        ] {
            assert_eq!(
                absolute.axis_info(),
                ("Frequency", "Hz", "Magnitude", "V"),
                "a scattering sweep states absolute port frequencies: {absolute:?}"
            );
        }
    }

    /// Only the periodic members of the noise family sweep an offset.
    ///
    /// A plain `.NOISE` run sweeps a real absolute frequency about a DC
    /// operating point: there is no carrier to be offset from, and it shared
    /// an arm with the three that have one.
    #[test]
    fn periodic_noise_sweeps_an_offset_and_plain_noise_sweeps_an_absolute_frequency() {
        for periodic in [
            AnalysisType::Pnoise,
            AnalysisType::Qpnoise,
            AnalysisType::Hbnoise,
        ] {
            assert_eq!(
                periodic.axis_info(),
                ("Offset Frequency", "Hz", "Noise", "V^2/Hz"),
                "{periodic:?}"
            );
        }
        assert_eq!(
            AnalysisType::Noise.axis_info(),
            ("Frequency", "Hz", "Noise", "V^2/Hz"),
            "a .NOISE sweep is about a DC operating point, with no carrier to offset from"
        );
    }

    /// A periodic run whose abscissa is not a frequency at all keeps its own
    /// name: `.PSS` and `.QPSS` publish a periodic phase, and `.PSTB` a
    /// Floquet mode index. The rename covers the small-signal family, not
    /// everything with a carrier.
    #[test]
    fn a_periodic_axis_that_is_not_a_frequency_is_not_renamed_to_one() {
        assert_eq!(AnalysisType::Pss.axis_info().0, "Time");
        assert_eq!(
            AnalysisType::Pstb.axis_info(),
            ("Mode", "index", "Stability metric", "")
        );
    }
}
