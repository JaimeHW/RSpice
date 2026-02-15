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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl AnalysisType {
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
            AnalysisType::Ac
            | AnalysisType::Disto
            | AnalysisType::Noise
            | AnalysisType::Tf
            | AnalysisType::Pac
            | AnalysisType::Pxf
            | AnalysisType::Pstb
            | AnalysisType::Stb
            | AnalysisType::SParameter
            | AnalysisType::HarmonicBalance
            | AnalysisType::Fourier => ("Frequency", "Hz", "Magnitude", "V"),
            AnalysisType::Pnoise => ("Frequency", "Hz", "Noise", "V^2/Hz"),
            AnalysisType::DcSweep => ("Voltage", "V", "Voltage", "V"),
            AnalysisType::DcOp => ("", "", "Voltage", "V"),
            AnalysisType::PoleZero => ("Real", "", "Imaginary", ""),
            AnalysisType::Sensitivity => ("Parameter", "", "Sensitivity", ""),
            AnalysisType::MonteCarlo => ("Value", "", "Count", "count"),
            AnalysisType::Parametric => ("Sweep", "", "Voltage", "V"),
            AnalysisType::Corner => ("Temperature", "C", "Voltage", "V"),
            AnalysisType::Reliability => ("Lifetime", "year", "Shift", ""),
            AnalysisType::Optimization => ("Iteration", "iter", "Cost", "cost"),
        }
    }
}

impl std::fmt::Display for AnalysisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
