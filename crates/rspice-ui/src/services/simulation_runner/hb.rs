use super::{build_engine_config, build_multi_tone_hb_layout};
use rspice_core::Value;
use rspice_core::engine::Engine;
/// Harmonic Balance analysis data
#[derive(Debug, Clone)]
pub struct HbData {
    /// Fundamental frequencies (one per tone)
    pub fundamentals: Vec<Value>,
    /// Number of harmonics per tone
    pub harmonics_per_tone: Vec<usize>,
    /// DC operating point voltages
    pub dc_voltages: Vec<(String, Value)>,
    /// Harmonic spectra: (node_name, [(freq, magnitude, phase_deg)])
    pub spectra: Vec<(String, Vec<(Value, Value, Value)>)>,
    /// Total number of frequency components
    pub num_components: usize,
    /// Whether solution converged
    pub converged: bool,
}

/// Harmonic Balance run configuration passed from the simulation pipeline.
#[derive(Debug, Clone)]
pub struct HbToneRunConfig {
    pub frequency: Value,
    pub harmonics: usize,
    pub source: Option<String>,
    pub name: Option<String>,
}

impl HbToneRunConfig {
    pub fn new(frequency: Value, harmonics: usize) -> Self {
        Self {
            frequency,
            harmonics,
            source: None,
            name: None,
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        let source = source.into();
        self.source = if source.trim().is_empty() {
            None
        } else {
            Some(source)
        };
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.name = if name.trim().is_empty() {
            None
        } else {
            Some(name)
        };
        self
    }
}

/// Harmonic Balance run configuration passed from the simulation pipeline.
#[derive(Debug, Clone)]
pub struct HbRunConfig {
    pub tones: Vec<HbToneRunConfig>,
    pub reltol: Value,
    pub abstol: Value,
    pub max_iterations: usize,
    pub damping: Value,
    pub oversample: usize,
    pub max_mixing_order: usize,
    pub use_krylov: bool,
    pub gmres_restart: usize,
    pub source_stepping: bool,
    pub verbose: bool,
}

impl Default for HbRunConfig {
    fn default() -> Self {
        Self {
            tones: vec![HbToneRunConfig::new(1e9, 9)],
            reltol: 1e-6,
            abstol: 1e-12,
            max_iterations: 100,
            damping: 1.0,
            oversample: 2,
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
            verbose: false,
        }
    }
}

impl HbRunConfig {
    fn validate(&self) -> Result<(), String> {
        if self.tones.is_empty() {
            return Err("HB requires at least one tone".to_string());
        }
        for (idx, tone) in self.tones.iter().enumerate() {
            if !tone.frequency.is_finite() || tone.frequency <= 0.0 {
                return Err(format!("HB tone {} frequency must be positive", idx + 1));
            }
            if tone.harmonics == 0 {
                return Err(format!("HB tone {} harmonics must be > 0", idx + 1));
            }
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err("HB reltol must be > 0".to_string());
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err("HB abstol must be > 0".to_string());
        }
        if self.max_iterations == 0 {
            return Err("HB max_iterations must be > 0".to_string());
        }
        if !self.damping.is_finite() || self.damping <= 0.0 || self.damping > 1.0 {
            return Err("HB damping must be in (0, 1]".to_string());
        }
        if self.oversample == 0 {
            return Err("HB oversample must be > 0".to_string());
        }
        if self.max_mixing_order == 0 {
            return Err("HB max_mixing_order must be > 0".to_string());
        }
        if self.gmres_restart == 0 {
            return Err("HB gmres_restart must be > 0".to_string());
        }
        Ok(())
    }
}

/// Run Harmonic Balance analysis
///
/// Solves for the steady-state response in the frequency domain,
/// suitable for RF circuits with multiple tones.
pub fn run_hb_analysis(netlist_text: &str, config: &HbRunConfig) -> Result<HbData, String> {
    use rspice_core::analysis::{HbConfig, HbTone};
    config.validate()?;

    // Parse the netlist
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let engine = Engine::new(build_engine_config(&netlist, None));

    let layout = build_multi_tone_hb_layout(&config.tones, config.max_mixing_order)?;
    let mut hb_config = HbConfig::new(layout.base_frequency).with_harmonics(layout.max_harmonic);
    hb_config.tones = config
        .tones
        .iter()
        .enumerate()
        .map(|(idx, tone)| {
            let mut hb_tone = HbTone::new(tone.frequency, tone.harmonics.max(1));
            if let Some(name) = tone
                .name
                .as_deref()
                .map(str::trim)
                .filter(|name| !name.is_empty())
            {
                hb_tone = hb_tone.with_name(name.to_string());
            } else {
                hb_tone = hb_tone.with_name(format!("tone{}", idx + 1));
            }
            if let Some(source) = tone
                .source
                .as_deref()
                .map(str::trim)
                .filter(|source| !source.is_empty())
            {
                hb_tone = hb_tone.with_source(source.to_string());
            }
            hb_tone
        })
        .collect();
    hb_config = hb_config
        .with_tolerance(config.reltol)
        .with_max_iterations(config.max_iterations)
        .with_damping(config.damping)
        .with_oversample(config.oversample);
    hb_config.abstol = config.abstol;
    hb_config.max_mixing_order = config.max_mixing_order;
    hb_config.use_krylov = config.use_krylov;
    hb_config.gmres_restart = config.gmres_restart;
    hb_config.source_stepping = config.source_stepping;
    hb_config.verbose = config.verbose;

    // Run actual HB analysis
    let hb_result = engine
        .run_hb(&netlist, hb_config)
        .map_err(|e| format!("HB error: {}", e))?;

    // Build fundamentals list
    let fundamentals = config.tones.iter().map(|tone| tone.frequency).collect();
    let harmonics_per_tone = config.tones.iter().map(|tone| tone.harmonics).collect();

    // Extract DC operating point from spectral data
    let dc_voltages: Vec<(String, Value)> = hb_result
        .result
        .spectral_voltages
        .iter()
        .map(|sv| {
            // DC is the zeroth harmonic (real part only for DC)
            let dc_val = sv.coefficients.first().map(|c| c.re).unwrap_or(0.0);
            (sv.node_name.clone(), dc_val)
        })
        .collect();

    // Build spectra from HB result's spectral voltages
    let mut spectra = Vec::new();
    for sv in &hb_result.result.spectral_voltages {
        let mut spectrum = Vec::new();

        // For each harmonic coefficient
        for (h, coeff) in sv.coefficients.iter().enumerate() {
            let freq = hb_result.fundamental_freq * h as Value;
            let magnitude = coeff.norm();
            let phase_deg = coeff.arg().to_degrees();
            spectrum.push((freq, magnitude, phase_deg));
        }

        spectra.push((format!("V({})", sv.node_name), spectrum));
    }

    // Number of frequency components
    let num_components = hb_result.num_harmonics + 1;

    Ok(HbData {
        fundamentals,
        harmonics_per_tone,
        dc_voltages,
        spectra,
        num_components,
        converged: hb_result.converged,
    })
}
