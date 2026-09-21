//! Harmonic Balance Configuration
//!
//! Defines configuration parameters for HB analysis, including multi-tone
//! setup, convergence tolerances, and FFT sizing.

use crate::Value;

/// Maximum collocation-grid size accepted by the standalone HB numerical
/// kernel.
///
/// Engine clients can impose a smaller resource limit.  This hard ceiling is
/// also enforced by the public solver constructor so an unauthenticated
/// configuration cannot ask `rustfft` to allocate an effectively unbounded
/// plan before the caller has a chance to receive an error.
pub(crate) const MAX_HB_COLLOCATION_POINTS: usize = 2_000_000;

/// Maximum requested Newton iterations accepted by one HB analysis.
pub(crate) const MAX_HB_ITERATIONS: usize = 1_000_000;

/// Maximum authored intermodulation order.
pub(crate) const MAX_HB_MIXING_ORDER: usize = 4096;

/// Maximum useful GMRES restart.  The shared Krylov implementation retains at
/// most this many Arnoldi vectors.
pub(crate) const MAX_HB_GMRES_RESTART: usize = 64;

/// A malformed harmonic-balance configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbConfigError {
    field: &'static str,
    detail: String,
}

impl HbConfigError {
    pub(crate) fn new(field: &'static str, detail: impl Into<String>) -> Self {
        Self {
            field,
            detail: detail.into(),
        }
    }

    /// Configuration field that violated the numerical contract.
    pub fn field(&self) -> &'static str {
        self.field
    }

    /// Human-readable description of the violated invariant.
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl std::fmt::Display for HbConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.field, self.detail)
    }
}

impl std::error::Error for HbConfigError {}

/// Configuration for a single tone in Harmonic Balance analysis
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct HbTone {
    /// Tone frequency in Hz
    pub frequency: Value,
    /// Number of harmonics to include for this tone
    pub num_harmonics: usize,
    /// Tone name (for identification in results)
    pub name: String,
    /// Optional source name filter.
    ///
    /// When set, this tone only drives independent sources with a matching name.
    /// When omitted, the tone is broadcast to all AC-capable independent sources.
    pub source_name: Option<String>,
}

impl HbTone {
    /// Create a new tone configuration
    pub fn new(frequency: Value, num_harmonics: usize) -> Self {
        Self {
            frequency,
            num_harmonics,
            name: format!("f{:.3e}", frequency),
            source_name: None,
        }
    }

    /// Set the tone name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the independent source name this tone should drive.
    pub fn with_source(mut self, source_name: impl Into<String>) -> Self {
        let source_name = source_name.into();
        self.source_name = if source_name.trim().is_empty() {
            None
        } else {
            Some(source_name)
        };
        self
    }
}

/// Configuration for Harmonic Balance analysis
///
/// HB analysis finds the periodic steady-state solution by solving for
/// Fourier coefficients directly in the frequency domain.
///
/// # Examples
///
/// ## Single-tone analysis
/// ```
/// use rspice_core::analysis::harmonic_balance::HbConfig;
///
/// let config = HbConfig::new(1e9)  // 1 GHz fundamental
///     .with_harmonics(9)           // DC through 9th harmonic
///     .with_tolerance(1e-6);
/// ```
///
/// ## Multi-tone analysis (mixer)
/// ```
/// use rspice_core::analysis::harmonic_balance::{HbConfig, HbTone};
///
/// let config = HbConfig::multi_tone(vec![
///     HbTone::new(900e6, 5).with_name("RF"),   // 900 MHz RF
///     HbTone::new(800e6, 5).with_name("LO"),   // 800 MHz LO
/// ])
/// .with_tolerance(1e-6);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct HbConfig {
    /// Primary fundamental frequency (Hz)
    /// For single-tone analysis, this is the only frequency.
    pub fundamental_freq: Value,

    /// Number of harmonics for primary tone (including DC)
    /// Total spectral components = num_harmonics + 1 for single-tone
    pub num_harmonics: usize,

    /// Additional tones for multi-tone analysis
    /// Empty for single-tone analysis
    pub tones: Vec<HbTone>,

    /// Relative Newton residual tolerance, applied separately to every row
    /// against the sum of that row's contribution magnitudes.
    pub tolerance: Value,

    /// Absolute residual tolerance for node equations: amperes for electrical
    /// and delay nodes, watts for thermal nodes. Explicit integral-rate
    /// equations use their integrand's units. The engine supplies VNTOL
    /// separately for voltage-valued branch equations.
    pub abstol: Value,

    /// Maximum Newton iterations
    pub max_iterations: usize,

    /// Newton damping factor (0 < damping <= 1)
    ///
    /// The scale the Armijo line search gives its first trial step, so values
    /// below one provide more conservative updates. [`Self::with_damping`]
    /// bounds an authored factor to `[0.1, 1]`, which is the same domain the
    /// Python entry points enforce.
    pub damping: Value,

    /// Smallest step scale the line search will take before it settles for the
    /// best trial it saw.
    ///
    /// The backtracking halves [`Self::damping`] until the Armijo test passes
    /// or the scale would fall below this floor. The default is the floor the
    /// search used while it was a literal, so honouring the field left every
    /// converged HB result unchanged.
    pub min_damping: Value,

    /// Oversampling factor for FFT (anti-aliasing)
    /// 2 = 2x oversampling, 4 = 4x, etc.
    /// Higher values reduce aliasing in nonlinear evaluation
    pub oversample_factor: usize,

    /// Optional exact number of time-domain collocation points.
    ///
    /// When unset, the solver chooses an oversampled power-of-two FFT grid.
    /// Set this for simulator-compatible minimal odd grids such as Xyce's
    /// `2 * NUMFREQ + 1` HB discretization. The engine validates that the
    /// grid is odd and can represent every configured harmonic.
    pub collocation_points: Option<usize>,

    /// Maximum intermodulation order for multi-tone
    /// Limits the number of mixing products considered
    pub max_mixing_order: usize,

    /// Force the Krylov solver.
    ///
    /// `false` is automatic: systems with ≥ 256 unknowns (nodes × spectral
    /// components) use Krylov, smaller systems use exact dense elimination.
    /// Shared Arnoldi storage is bounded independently of system dimension.
    /// Each consuming analysis owns its preconditioner, numerical qualification,
    /// and recovery policy.
    pub use_krylov: bool,

    /// Requested GMRES restart parameter for HB, PAC, and PNoise Krylov solves.
    ///
    /// The canonical configuration contract accepts 1 through 64. The shared
    /// solver further bounds this to the system dimension; requests below
    /// eight retain the historical minimum restart of eight whenever the
    /// dimension permits it. The default is 30.
    pub gmres_restart: usize,

    /// Enable source stepping for difficult convergence
    pub source_stepping: bool,

    /// Solve Newton steps with the exact real-split Jacobian (Toeplitz plus
    /// conjugate/Hankel coupling). `false` selects the legacy Toeplitz-only
    /// complex Jacobian, kept for A/B comparison and the large-system Krylov
    /// fast path; both converge to identical spectra, the exact path just
    /// gets there in fewer iterations.
    pub use_exact_jacobian: bool,

    /// Verbose logging
    pub verbose: bool,
}

impl HbConfig {
    /// Create a new single-tone HB configuration
    ///
    /// # Arguments
    /// * `fundamental_freq` - Fundamental frequency in Hz
    pub fn new(fundamental_freq: Value) -> Self {
        Self {
            fundamental_freq,
            num_harmonics: 9,
            tones: Vec::new(),
            tolerance: 1e-6,
            abstol: 1e-12,
            max_iterations: 100,
            damping: 1.0,
            min_damping: 0.01,
            oversample_factor: 2,
            collocation_points: None,
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
            use_exact_jacobian: true,
            verbose: false,
        }
    }

    /// Create a multi-tone HB configuration
    ///
    /// The spectral basis is the common fundamental of all tones (their
    /// approximate greatest common divisor), so every tone lands on an
    /// integer harmonic: 900 MHz + 800 MHz resolves to a 100 MHz basis with
    /// the tones at harmonics 9 and 8. The harmonic count covers each tone's
    /// requested order against that basis. Taking the first tone's frequency
    /// as the basis (the previous behaviour) rejected every genuinely
    /// multi-tone configuration at run time.
    ///
    /// # Arguments
    /// * `tones` - Vector of tone configurations
    pub fn multi_tone(tones: Vec<HbTone>) -> Self {
        let basis = Self::common_basis(&tones);
        let num_harmonics = tones
            .iter()
            .map(|t| {
                let tone_harmonic = if basis > 0.0 {
                    (t.frequency / basis).round().max(1.0) as usize
                } else {
                    1
                };
                tone_harmonic.saturating_mul(t.num_harmonics.max(1))
            })
            .max()
            .unwrap_or(9)
            .min(4096);

        Self {
            fundamental_freq: basis,
            num_harmonics,
            tones,
            ..Self::new(basis)
        }
    }

    /// Approximate greatest common divisor of the tone frequencies.
    fn common_basis(tones: &[HbTone]) -> Value {
        fn float_gcd(a: Value, b: Value) -> Value {
            let (mut a, mut b) = (a.abs(), b.abs());
            let tol = 1e-9 * a.max(b).max(f64::MIN_POSITIVE);
            while b > tol {
                let r = a % b;
                a = b;
                b = r;
            }
            a
        }

        let mut basis = tones.first().map(|t| t.frequency).unwrap_or(1e9);
        for tone in tones.iter().skip(1) {
            basis = float_gcd(basis, tone.frequency);
        }
        basis
    }

    /// Resolve one authored [`HbCard`](crate::netlist::HbCard) against the
    /// deck's `.OPTIONS`.
    ///
    /// Taking the whole card rather than its tone list is what makes this the
    /// only channel: a control the card gains reaches every surface at once,
    /// and a surface that would have ignored it fails to compile instead.
    ///
    /// The harmonic-order contract, whether the orders come from the card's
    /// `HARMS=` or from the deck's `NUMFREQ`, is Xyce's:
    ///
    /// - neither authored: every tone keeps this configuration's own default
    ///   order, and the collocation grid stays the solver's default;
    /// - one order: it is broadcast across every tone;
    /// - one order per tone: they pair positionally;
    /// - anything else is an authored-input defect.
    ///
    /// A single tone whose order the deck stated explicitly also pins the
    /// minimal bilateral `2N+1` collocation grid, which is what an explicit
    /// `NUMFREQ` asks for. Every other shape uses the configuration's own
    /// default grid, unless `POINTS=` names an exact one.
    ///
    /// This is the one implementation of that rule. The CLI, the Python
    /// bindings, the browser API, the engine adapter and the Studio all
    /// translate the same authored card, and independent translations of a
    /// harmonic order list are independent chances to disagree about how many
    /// harmonics a deck asked for.
    pub fn from_hb_card(
        card: &crate::netlist::HbCard,
        options: &crate::netlist::SimulationOptions,
    ) -> Result<Self, HbConfigError> {
        let frequencies = card.frequencies.as_slice();
        if frequencies.is_empty() {
            return Err(HbConfigError::new(
                "tones",
                ".HB requires at least one positive tone frequency",
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        for (index, frequency) in frequencies.iter().enumerate() {
            if !frequency.is_finite() || *frequency <= 0.0 {
                return Err(HbConfigError::new(
                    "tones",
                    format!(
                        ".HB tone {} must be a positive finite frequency, not {frequency}",
                        index + 1
                    ),
                ));
            }
            if !seen.insert(frequency.to_bits()) {
                return Err(HbConfigError::new(
                    "tones",
                    format!(".HB lists the tone frequency {frequency} more than once"),
                ));
            }
        }

        // One quantity, one home. `HARMS=` and `MAXITER=` are the per-card
        // statements of what `.OPTIONS HBINT NUMFREQ` and
        // `.OPTIONS NONLIN-HB MAXSTEP` say deck-wide; a deck that states one
        // of them in both places is answered by name instead of being given a
        // precedence rule it cannot see. The parser cannot judge this — an
        // `.OPTIONS` line may follow the card — so the resolution does.
        if !card.harmonics.is_empty() && !options.hb_num_frequencies.is_empty() {
            return Err(HbConfigError::new(
                "num_harmonics",
                ".HB states the harmonic count on the card (HARMS=) and in \
                 .OPTIONS HBINT NUMFREQ; keep one",
            ));
        }
        if card.max_iterations.is_some() && options.nonlin_hb_maxstep.is_some() {
            return Err(HbConfigError::new(
                "max_iterations",
                ".HB states the Newton iteration budget on the card (MAXITER=) and in \
                 .OPTIONS NONLIN-HB MAXSTEP; keep one",
            ));
        }

        let (harmonic_orders, spelling): (&[usize], &str) = if card.harmonics.is_empty() {
            (&options.hb_num_frequencies, ".OPTIONS HBINT NUMFREQ")
        } else {
            (&card.harmonics, "the .HB card's HARMS=")
        };

        let default_harmonics = Self::new(frequencies[0]).num_harmonics;
        let orders: Vec<usize> = if harmonic_orders.is_empty() {
            vec![default_harmonics; frequencies.len()]
        } else if harmonic_orders.contains(&0) {
            return Err(HbConfigError::new(
                "num_harmonics",
                format!("{spelling} harmonic orders must all be at least 1"),
            ));
        } else if harmonic_orders.len() == 1 {
            vec![harmonic_orders[0]; frequencies.len()]
        } else if harmonic_orders.len() == frequencies.len() {
            harmonic_orders.to_vec()
        } else {
            return Err(HbConfigError::new(
                "num_harmonics",
                format!(
                    ".HB has {} tones but {spelling} lists {} harmonic orders; \
                     provide one order to broadcast or one per tone",
                    frequencies.len(),
                    harmonic_orders.len()
                ),
            ));
        };

        let mut tones = Vec::new();
        tones
            .try_reserve_exact(frequencies.len())
            .map_err(|_| HbConfigError::new("tones", "could not allocate the .HB tone list"))?;
        for (index, (frequency, order)) in frequencies.iter().zip(&orders).enumerate() {
            let tone = HbTone::new(*frequency, *order).with_name(format!("tone{}", index + 1));
            tones.push(match card.sources.get(index) {
                Some(Some(source)) => tone.with_source(source.clone()),
                _ => tone,
            });
        }

        let mut config = Self::resolve_basis(tones, harmonic_orders, spelling)?;
        Self::apply_authored_controls(card, &mut config);
        Self::widen_basis_to_mixing_order(card, &mut config)?;
        config.validate()?;
        Ok(config)
    }

    /// Widen a multi-tone basis to the intermodulation order its card states.
    ///
    /// Without `MAXMIXING=` the basis covers each tone's own order and
    /// nothing else, which is what every deck written before the keyword
    /// existed resolves to. With it, the basis must also reach the highest
    /// mixing product the card asked to see: `MAXMIXING` copies of the
    /// highest-placed tone. 900 MHz and 800 MHz at three harmonics each sit
    /// at harmonics 9 and 8 of a 100 MHz basis and need 27 of them; asking
    /// for fifth-order mixing needs 45.
    ///
    /// One tone has nothing to mix with, so the keyword only reaches the
    /// basis on a multi-tone card — it still reaches the Xyce APFT lattice
    /// check through `max_mixing_order` either way.
    fn widen_basis_to_mixing_order(
        card: &crate::netlist::HbCard,
        config: &mut Self,
    ) -> Result<(), HbConfigError> {
        let Some(mixing) = card.max_mixing_order else {
            return Ok(());
        };
        if config.tones.len() < 2 {
            return Ok(());
        }
        let basis = config.fundamental_freq;
        if !basis.is_finite() || basis <= 0.0 {
            return Err(HbConfigError::new(
                "fundamental_freq",
                "must be finite and positive",
            ));
        }
        let mut highest_index = 1usize;
        for tone in &config.tones {
            let index = (tone.frequency / basis).round();
            if !index.is_finite() || index < 1.0 || index > usize::MAX as Value {
                return Err(HbConfigError::new(
                    "tones",
                    format!(
                        "tone '{}' does not land on a positive integer harmonic of the \
                         {basis} Hz basis",
                        tone.name
                    ),
                ));
            }
            highest_index = highest_index.max(index as usize);
        }
        let widened = mixing.checked_mul(highest_index).ok_or_else(|| {
            HbConfigError::new(
                "max_mixing_order",
                "and the tone placement overflow the addressable collocation grid",
            )
        })?;
        config.num_harmonics = config.num_harmonics.max(widened);
        Ok(())
    }

    /// Place the card's tones on the basis the engine will solve them over.
    ///
    /// A lone tone that does not name its source needs no tone list at all:
    /// the drive is broadcast, the basis IS the tone, and leaving `tones`
    /// empty is what every `.HB` deck written before the card had keywords
    /// resolves to. A lone tone that DOES name a source has to be carried as
    /// a tone, because the source filter lives on the tone; it still sits at
    /// harmonic one of its own frequency, so the engine drives exactly the
    /// same single harmonic.
    fn resolve_basis(
        tones: Vec<HbTone>,
        harmonic_orders: &[usize],
        spelling: &str,
    ) -> Result<Self, HbConfigError> {
        let [tone] = tones.as_slice() else {
            return Ok(Self::multi_tone(tones));
        };
        let (frequency, order, names_a_source) = (
            tone.frequency,
            tone.num_harmonics,
            tone.source_name.is_some(),
        );
        let mut config = Self::new(frequency).with_harmonics(order);
        if !harmonic_orders.is_empty() {
            let points = config.minimum_collocation_points().ok_or_else(|| {
                HbConfigError::new(
                    "collocation_points",
                    format!(
                        "{spelling} harmonic count {order} exceeds the addressable \
                         collocation grid"
                    ),
                )
            })?;
            config = config.with_collocation_points(points);
        }
        if names_a_source {
            config.tones = tones;
        }
        Ok(config)
    }

    /// Move every authored keyword onto the field it names.
    ///
    /// An authored value is assigned, never passed through a builder: the
    /// builders clamp, and a clamp here would silently run a different
    /// configuration from the one the card states. Out-of-range values are
    /// refused — by the card's own ranges when a deck wrote them, and by
    /// [`Self::validate`] for every other caller — rather than adjusted.
    ///
    /// `POINTS=` is applied after the harmonic-order rule, so an authored
    /// grid replaces the `2N+1` one an explicit order implies.
    fn apply_authored_controls(card: &crate::netlist::HbCard, config: &mut Self) {
        if let Some(value) = card.oversample {
            config.oversample_factor = value;
        }
        if card.automatic_collocation {
            config.collocation_points = None;
        } else if let Some(value) = card.collocation_points {
            config.collocation_points = Some(value);
        }
        if let Some(value) = card.max_mixing_order {
            config.max_mixing_order = value;
        }
        if let Some(value) = card.reltol {
            config.tolerance = value;
        }
        if let Some(value) = card.abstol {
            config.abstol = value;
        }
        if let Some(value) = card.max_iterations {
            config.max_iterations = value;
        }
        if let Some(value) = card.damping {
            config.damping = value;
        }
        if let Some(value) = card.min_damping {
            config.min_damping = value;
        }
        if let Some(value) = card.use_krylov {
            config.use_krylov = value;
        }
        if let Some(value) = card.gmres_restart {
            config.gmres_restart = value;
        }
        if let Some(value) = card.source_stepping {
            config.source_stepping = value;
        }
        if let Some(value) = card.use_exact_jacobian {
            config.use_exact_jacobian = value;
        }
        if let Some(value) = card.verbose {
            config.verbose = value;
        }
    }

    /// Set number of harmonics
    pub fn with_harmonics(mut self, n: usize) -> Self {
        self.num_harmonics = n.max(1);
        self
    }

    /// Set convergence tolerance
    pub fn with_tolerance(mut self, tol: Value) -> Self {
        self.tolerance = tol;
        self
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    /// Set Newton damping factor
    ///
    /// Bounded to `[0.1, 1]`: a scale above one is not a damped step, and the
    /// lower bound is the authored domain the Python entry points already
    /// refuse outside of. A more conservative first trial than `0.1` is asked
    /// for through [`Self::min_damping`], which the line search honours down
    /// to whatever positive floor [`Self::validate`] accepts.
    pub fn with_damping(mut self, damping: Value) -> Self {
        self.damping = damping.clamp(0.1, 1.0);
        self
    }

    /// Set oversampling factor for FFT
    pub fn with_oversample(mut self, factor: usize) -> Self {
        self.oversample_factor = factor.max(2);
        self
    }

    /// Use an exact odd time-domain collocation grid.
    pub fn with_collocation_points(mut self, points: usize) -> Self {
        self.collocation_points = Some(points);
        self
    }

    /// Smallest odd collocation grid that can represent DC and every
    /// configured positive and negative harmonic.
    ///
    /// Returns `None` when the configured harmonic count cannot be
    /// represented by `usize`; callers should reject that configuration.
    pub fn minimum_collocation_points(&self) -> Option<usize> {
        self.num_harmonics.checked_mul(2)?.checked_add(1)
    }

    /// Validate every numerical invariant consumed by the HB engine and
    /// standalone solver.
    ///
    /// Callers may construct `HbConfig` with struct literals or mutate its
    /// public fields, so builder-method clamping is not an authentication
    /// boundary.  This method is the canonical contract used before FFT
    /// planning, solver construction, and retained-state reconstruction.
    pub(crate) fn validate(&self) -> Result<(), HbConfigError> {
        if !self.fundamental_freq.is_finite() || self.fundamental_freq <= 0.0 {
            return Err(HbConfigError::new(
                "fundamental_freq",
                "must be finite and positive",
            ));
        }
        if !self.fundamental_freq.recip().is_finite() {
            return Err(HbConfigError::new(
                "fundamental_freq",
                "must have a finite representable period",
            ));
        }
        if self.num_harmonics == 0 {
            return Err(HbConfigError::new("num_harmonics", "must be at least one"));
        }
        let minimum_points = self.minimum_collocation_points().ok_or_else(|| {
            HbConfigError::new(
                "num_harmonics",
                "overflows the addressable collocation grid",
            )
        })?;
        if minimum_points > MAX_HB_COLLOCATION_POINTS {
            return Err(HbConfigError::new(
                "num_harmonics",
                format!(
                    "requires {minimum_points} collocation points, above the supported limit {MAX_HB_COLLOCATION_POINTS}"
                ),
            ));
        }
        let highest_frequency = self.fundamental_freq * self.num_harmonics as Value;
        if !highest_frequency.is_finite()
            || !(std::f64::consts::TAU * highest_frequency).is_finite()
        {
            return Err(HbConfigError::new(
                "fundamental_freq",
                "and num_harmonics produce a non-finite angular frequency",
            ));
        }

        for (field, value) in [("tolerance", self.tolerance), ("abstol", self.abstol)] {
            if !value.is_finite() || value <= 0.0 {
                return Err(HbConfigError::new(
                    field,
                    "must be finite and greater than zero",
                ));
            }
        }
        if self.max_iterations == 0 || self.max_iterations > MAX_HB_ITERATIONS {
            return Err(HbConfigError::new(
                "max_iterations",
                format!("must be in 1..={MAX_HB_ITERATIONS}"),
            ));
        }
        if !self.damping.is_finite() || self.damping <= 0.0 || self.damping > 1.0 {
            return Err(HbConfigError::new(
                "damping",
                "must be finite and in (0, 1]",
            ));
        }
        if !self.min_damping.is_finite()
            || self.min_damping <= 0.0
            || self.min_damping > self.damping
        {
            return Err(HbConfigError::new(
                "min_damping",
                "must be finite, greater than zero, and no greater than damping",
            ));
        }
        if self.oversample_factor < 2 {
            return Err(HbConfigError::new(
                "oversample_factor",
                "must be at least two",
            ));
        }
        if self.max_mixing_order == 0 || self.max_mixing_order > MAX_HB_MIXING_ORDER {
            return Err(HbConfigError::new(
                "max_mixing_order",
                format!("must be in 1..={MAX_HB_MIXING_ORDER}"),
            ));
        }
        if self.gmres_restart == 0 || self.gmres_restart > MAX_HB_GMRES_RESTART {
            return Err(HbConfigError::new(
                "gmres_restart",
                format!("must be in 1..={MAX_HB_GMRES_RESTART}"),
            ));
        }

        for (index, tone) in self.tones.iter().enumerate() {
            if !tone.frequency.is_finite() || tone.frequency <= 0.0 {
                return Err(HbConfigError::new(
                    "tones",
                    format!("tone {index} frequency must be finite and greater than zero"),
                ));
            }
            if tone.num_harmonics == 0 {
                return Err(HbConfigError::new(
                    "tones",
                    format!("tone {index} must retain at least one harmonic"),
                ));
            }
            let ratio = tone.frequency / self.fundamental_freq;
            let harmonic = ratio.round();
            let relative_error = (ratio - harmonic).abs() / harmonic.abs().max(1.0);
            if !ratio.is_finite()
                || !harmonic.is_finite()
                || harmonic < 1.0
                || relative_error > 1.0e-9
                || harmonic > usize::MAX as Value
            {
                return Err(HbConfigError::new(
                    "tones",
                    format!(
                        "tone {index} frequency must be a positive integer harmonic of fundamental_freq"
                    ),
                ));
            }
            let harmonic = harmonic as usize;
            let required = harmonic.checked_mul(tone.num_harmonics).ok_or_else(|| {
                HbConfigError::new("tones", format!("tone {index} harmonic order overflows"))
            })?;
            if required > self.num_harmonics {
                return Err(HbConfigError::new(
                    "tones",
                    format!(
                        "tone {index} requires common-basis harmonic {required}, beyond num_harmonics {}",
                        self.num_harmonics
                    ),
                ));
            }
        }

        self.checked_fft_size().map(|_| ())
    }

    /// Enable verbose logging
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Check if this is a multi-tone analysis
    pub fn is_multi_tone(&self) -> bool {
        !self.tones.is_empty()
    }

    /// Get total number of spectral components per node
    pub fn num_spectral_components(&self) -> usize {
        self.checked_num_spectral_components().unwrap_or(usize::MAX)
    }

    fn checked_num_spectral_components(&self) -> Result<usize, HbConfigError> {
        if self.is_multi_tone() {
            self.tones.iter().try_fold(1usize, |count, tone| {
                tone.num_harmonics
                    .checked_mul(2)
                    .and_then(|components| count.checked_add(components))
                    .ok_or_else(|| {
                        HbConfigError::new(
                            "tones",
                            "spectral-component count exceeds this platform",
                        )
                    })
            })
        } else {
            self.num_harmonics.checked_add(1).ok_or_else(|| {
                HbConfigError::new(
                    "num_harmonics",
                    "spectral-component count exceeds this platform",
                )
            })
        }
    }

    /// Get FFT size for time-domain evaluation
    pub fn fft_size(&self) -> usize {
        self.checked_fft_size().unwrap_or(usize::MAX)
    }

    /// Return the exact FFT/collocation size without saturating arithmetic.
    pub(crate) fn checked_fft_size(&self) -> Result<usize, HbConfigError> {
        let minimum_points = self.minimum_collocation_points().ok_or_else(|| {
            HbConfigError::new(
                "num_harmonics",
                "overflows the addressable collocation grid",
            )
        })?;
        let fft_size = if let Some(points) = self.collocation_points {
            if points % 2 == 0 {
                return Err(HbConfigError::new(
                    "collocation_points",
                    "collocation grid must be odd",
                ));
            }
            if points < minimum_points {
                return Err(HbConfigError::new(
                    "collocation_points",
                    format!("collocation grid must contain at least {minimum_points} points"),
                ));
            }
            points
        } else {
            let spectral_components = self.checked_num_spectral_components()?;
            let oversampled = spectral_components
                .checked_mul(self.oversample_factor)
                .ok_or_else(|| {
                    HbConfigError::new(
                        "oversample_factor",
                        "overflows the addressable collocation grid",
                    )
                })?;
            oversampled
                .max(minimum_points)
                .checked_next_power_of_two()
                .ok_or_else(|| {
                    HbConfigError::new(
                        "oversample_factor",
                        "requires a collocation grid too large for this platform",
                    )
                })?
        };
        if fft_size > MAX_HB_COLLOCATION_POINTS {
            return Err(HbConfigError::new(
                if self.collocation_points.is_some() {
                    "collocation_points"
                } else {
                    "oversample_factor"
                },
                format!(
                    "requires {fft_size} points, above the supported limit {MAX_HB_COLLOCATION_POINTS}"
                ),
            ));
        }
        Ok(fft_size)
    }

    /// Get the fundamental period
    pub fn period(&self) -> Value {
        if self.fundamental_freq > 0.0 {
            1.0 / self.fundamental_freq
        } else {
            1.0
        }
    }

    /// Get all harmonic frequencies for single-tone
    pub fn harmonic_frequencies(&self) -> Vec<Value> {
        (0..=self.num_harmonics)
            .map(|k| k as Value * self.fundamental_freq)
            .collect()
    }
}

impl Default for HbConfig {
    fn default() -> Self {
        Self::new(1e9) // 1 GHz default
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::{HbCard, SimulationOptions};

    /// A `.HB` card that authors nothing but its tones.
    fn tones_only(frequencies: &[Value]) -> HbCard {
        HbCard {
            frequencies: frequencies.to_vec(),
            ..HbCard::default()
        }
    }

    /// A deck whose only harmonic-balance option is `HBINT NUMFREQ`.
    fn numfreq(orders: &[usize]) -> SimulationOptions {
        SimulationOptions {
            hb_num_frequencies: orders.to_vec(),
            ..SimulationOptions::default()
        }
    }

    /// Resolve the `.HB` card of a deck written in full, so the card under
    /// test is the one the parser produced rather than one built by hand.
    fn resolve(cards: &str) -> Result<HbConfig, HbConfigError> {
        let deck = crate::netlist::Netlist::parse(&format!(
            "hb resolution\nV1 in 0 SIN(0 1 900MEG)\nV2 lo 0 SIN(0 1 800MEG)\n\
             R1 in out 1k\nR2 lo out 1k\nC1 out 0 1p\n{cards}\n.end\n"
        ))
        .unwrap_or_else(|error| panic!("the deck parses: {error}"));
        let card = deck
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                crate::netlist::AnalysisCommand::Hb(card) => Some((**card).clone()),
                _ => None,
            })
            .expect("the deck authors one .HB card");
        HbConfig::from_hb_card(&card, &deck.options)
    }

    /// Every float of two configurations agrees to the bit, not merely to
    /// `PartialEq` on a value that could have been produced by a different
    /// arithmetic path.
    fn assert_bit_identical(resolved: &HbConfig, expected: &HbConfig) {
        assert_eq!(resolved, expected, "resolved configuration differs");
        assert_eq!(
            resolved.fundamental_freq.to_bits(),
            expected.fundamental_freq.to_bits(),
            "fundamental frequency differs in its bits"
        );
        for (field, left, right) in [
            ("tolerance", resolved.tolerance, expected.tolerance),
            ("abstol", resolved.abstol, expected.abstol),
            ("damping", resolved.damping, expected.damping),
            ("min_damping", resolved.min_damping, expected.min_damping),
        ] {
            assert_eq!(
                left.to_bits(),
                right.to_bits(),
                "{field} differs in its bits"
            );
        }
        for (resolved_tone, expected_tone) in resolved.tones.iter().zip(&expected.tones) {
            assert_eq!(
                resolved_tone.frequency.to_bits(),
                expected_tone.frequency.to_bits(),
                "tone {} frequency differs in its bits",
                resolved_tone.name
            );
        }
    }

    /// The four shapes a `.HB` card with no keywords can take, each against
    /// the configuration the documented rule builds by hand. This is the
    /// guarantee every deck written before the card had keywords depends on:
    /// the typed card changed the plumbing and nothing else.
    #[test]
    fn an_hb_card_without_keywords_resolves_as_it_always_has() {
        let default_order = HbConfig::new(1.0e9).num_harmonics;

        // One tone, no option: the solver's own order and its own grid.
        assert_bit_identical(
            &HbConfig::from_hb_card(&tones_only(&[1.0e9]), &SimulationOptions::default())
                .expect("a one-tone .HB resolves"),
            &HbConfig::new(1.0e9).with_harmonics(default_order),
        );

        // One tone with NUMFREQ: the stated order and the minimal 2N+1 grid.
        assert_bit_identical(
            &HbConfig::from_hb_card(&tones_only(&[1.0e9]), &numfreq(&[5]))
                .expect("an explicit order resolves"),
            &HbConfig::new(1.0e9)
                .with_harmonics(5)
                .with_collocation_points(11),
        );

        // Two tones, no option: the common basis at the default order.
        let two_tone_default = HbConfig::multi_tone(vec![
            HbTone::new(9.0e8, HbConfig::new(9.0e8).num_harmonics).with_name("tone1"),
            HbTone::new(8.0e8, HbConfig::new(8.0e8).num_harmonics).with_name("tone2"),
        ]);
        assert_bit_identical(
            &HbConfig::from_hb_card(&tones_only(&[9.0e8, 8.0e8]), &SimulationOptions::default())
                .expect("a two-tone .HB resolves"),
            &two_tone_default,
        );

        // Two tones with a broadcast order, and the same orders paired.
        let two_tone_paired = HbConfig::multi_tone(vec![
            HbTone::new(9.0e8, 4).with_name("tone1"),
            HbTone::new(8.0e8, 4).with_name("tone2"),
        ]);
        assert_bit_identical(
            &HbConfig::from_hb_card(&tones_only(&[9.0e8, 8.0e8]), &numfreq(&[4]))
                .expect("broadcasting resolves"),
            &two_tone_paired,
        );
        assert_bit_identical(
            &HbConfig::from_hb_card(&tones_only(&[9.0e8, 8.0e8]), &numfreq(&[4, 4]))
                .expect("pairing resolves"),
            &two_tone_paired,
        );
    }

    /// One row of the keyword sweep: the clause a card carries, what the
    /// resolution must then hold, and how to put that one field back so the
    /// rest of the configuration can be compared against the keyword-less
    /// resolution.
    struct KeywordRow {
        clause: &'static str,
        holds: fn(&HbConfig) -> bool,
        restore: fn(&mut HbConfig, &HbConfig),
    }

    /// Every keyword, one at a time, at a value that is not its default:
    /// the field it names moves, and nothing else does.
    #[test]
    fn every_hb_keyword_moves_exactly_the_field_it_names() {
        let baseline = resolve(".HB 900MEG").expect("a keyword-less one-tone card resolves");
        let rows = [
            // HARMS names the harmonic count; the minimal bilateral grid an
            // explicit count asks for comes with it, as it does for NUMFREQ.
            KeywordRow {
                clause: "HARMS=5",
                holds: |config| config.num_harmonics == 5 && config.collocation_points == Some(11),
                restore: |authored, baseline| {
                    authored.num_harmonics = baseline.num_harmonics;
                    authored.collocation_points = baseline.collocation_points;
                },
            },
            // A named source is carried as the tone that holds the filter.
            KeywordRow {
                clause: "SOURCE1=V1",
                holds: |config| {
                    config.tones.len() == 1
                        && config.tones[0].source_name.as_deref() == Some("V1")
                        && config.tones[0].frequency == config.fundamental_freq
                },
                restore: |authored, baseline| authored.tones = baseline.tones.clone(),
            },
            KeywordRow {
                clause: "OVERSAMPLE=4",
                holds: |config| config.oversample_factor == 4,
                restore: |authored, baseline| {
                    authored.oversample_factor = baseline.oversample_factor
                },
            },
            KeywordRow {
                clause: "POINTS=101",
                holds: |config| config.collocation_points == Some(101),
                restore: |authored, baseline| {
                    authored.collocation_points = baseline.collocation_points
                },
            },
            KeywordRow {
                clause: "MAXMIXING=7",
                holds: |config| config.max_mixing_order == 7,
                restore: |authored, baseline| authored.max_mixing_order = baseline.max_mixing_order,
            },
            KeywordRow {
                clause: "RELTOL=1e-8",
                holds: |config| config.tolerance == 1.0e-8,
                restore: |authored, baseline| authored.tolerance = baseline.tolerance,
            },
            KeywordRow {
                clause: "ABSTOL=1e-14",
                holds: |config| config.abstol == 1.0e-14,
                restore: |authored, baseline| authored.abstol = baseline.abstol,
            },
            KeywordRow {
                clause: "MAXITER=42",
                holds: |config| config.max_iterations == 42,
                restore: |authored, baseline| authored.max_iterations = baseline.max_iterations,
            },
            KeywordRow {
                clause: "DAMPING=0.5",
                holds: |config| config.damping == 0.5,
                restore: |authored, baseline| authored.damping = baseline.damping,
            },
            KeywordRow {
                clause: "MINDAMPING=0.02",
                holds: |config| config.min_damping == 0.02,
                restore: |authored, baseline| authored.min_damping = baseline.min_damping,
            },
            KeywordRow {
                clause: "SOLVER=KRYLOV",
                holds: |config| config.use_krylov,
                restore: |authored, baseline| authored.use_krylov = baseline.use_krylov,
            },
            KeywordRow {
                clause: "GMRESRESTART=16",
                holds: |config| config.gmres_restart == 16,
                restore: |authored, baseline| authored.gmres_restart = baseline.gmres_restart,
            },
            KeywordRow {
                clause: "SOURCESTEPPING=yes",
                holds: |config| config.source_stepping,
                restore: |authored, baseline| authored.source_stepping = baseline.source_stepping,
            },
            KeywordRow {
                clause: "EXACTJACOBIAN=no",
                holds: |config| !config.use_exact_jacobian,
                restore: |authored, baseline| {
                    authored.use_exact_jacobian = baseline.use_exact_jacobian
                },
            },
            KeywordRow {
                clause: "VERBOSE=yes",
                holds: |config| config.verbose,
                restore: |authored, baseline| authored.verbose = baseline.verbose,
            },
        ];

        // Every field the vocabulary can reach is exercised: a keyword added
        // to the card without a row here would leave its field unproven.
        assert_eq!(rows.len(), 15, "one row per .HB keyword");

        for row in rows {
            let mut authored = resolve(&format!(".HB 900MEG {}", row.clause))
                .unwrap_or_else(|error| panic!("{} resolves: {error}", row.clause));
            assert!(
                (row.holds)(&authored),
                "{} did not reach its field: {authored:?}",
                row.clause
            );
            assert_ne!(
                authored, baseline,
                "{} is written at its own default and proves nothing",
                row.clause
            );
            (row.restore)(&mut authored, &baseline);
            assert_eq!(
                authored, baseline,
                "{} moved a field it does not name",
                row.clause
            );
        }
    }

    /// The harmonic count has one home. A deck that states it on the card and
    /// in `.OPTIONS` is refused by name, not given a precedence rule.
    #[test]
    fn a_harmonic_count_stated_on_the_card_and_in_hbint_is_refused() {
        let error = resolve(".options hbint numfreq=5\n.HB 900MEG HARMS=3")
            .expect_err("a harmonic count stated twice is refused");
        assert_eq!(error.field(), "num_harmonics");
        assert_eq!(
            error.to_string(),
            "num_harmonics .HB states the harmonic count on the card (HARMS=) and in \
             .OPTIONS HBINT NUMFREQ; keep one"
        );
        // Each spelling on its own still resolves, and to the same order.
        assert_eq!(
            resolve(".options hbint numfreq=3\n.HB 900MEG")
                .expect("the option alone resolves")
                .num_harmonics,
            3
        );
        assert_eq!(
            resolve(".HB 900MEG HARMS=3")
                .expect("the card alone resolves")
                .num_harmonics,
            3
        );
    }

    /// So does the Newton budget.
    #[test]
    fn a_newton_budget_stated_on_the_card_and_in_nonlin_hb_is_refused() {
        let error = resolve(".options nonlin-hb maxstep=7\n.HB 900MEG MAXITER=42")
            .expect_err("a Newton budget stated twice is refused");
        assert_eq!(error.field(), "max_iterations");
        assert_eq!(
            error.to_string(),
            "max_iterations .HB states the Newton iteration budget on the card (MAXITER=) and \
             in .OPTIONS NONLIN-HB MAXSTEP; keep one"
        );
        assert_eq!(
            resolve(".options nonlin-hb maxstep=7\n.HB 900MEG")
                .expect("the option alone resolves")
                .max_iterations,
            HbConfig::new(9.0e8).max_iterations,
            "the option is applied by the engine, not by this resolution"
        );
        assert_eq!(
            resolve(".HB 900MEG MAXITER=42")
                .expect("the card alone resolves")
                .max_iterations,
            42
        );
    }

    /// The Studio's basis rule, now core's: 900 MHz and 800 MHz at three
    /// harmonics each sit at harmonics 9 and 8 of a 100 MHz basis, which
    /// needs 27 common-basis harmonics; a card that asks to see fifth-order
    /// mixing needs 45. A card that does not ask still gets 27, so no deck
    /// written before the keyword existed moves.
    #[test]
    fn an_authored_mixing_order_widens_a_two_tone_basis() {
        let plain = resolve(".HB 900MEG 800MEG HARMS=3").expect("a two-tone card resolves");
        assert_eq!(plain.fundamental_freq, 1.0e8);
        assert_eq!(plain.num_harmonics, 27);
        assert_eq!(plain.max_mixing_order, 5, "the default mixing order");

        let widened =
            resolve(".HB 900MEG 800MEG HARMS=3 MAXMIXING=5").expect("an authored order resolves");
        assert_eq!(widened.fundamental_freq, 1.0e8);
        assert_eq!(widened.num_harmonics, 45);

        // The widening is the maximum of the two rules, so an order that asks
        // for less than the tones already need cannot shrink the basis.
        assert_eq!(
            resolve(".HB 900MEG 800MEG HARMS=3 MAXMIXING=2")
                .expect("a low mixing order resolves")
                .num_harmonics,
            27
        );
        // One tone has nothing to mix with, so its basis is untouched.
        assert_eq!(
            resolve(".HB 900MEG HARMS=3 MAXMIXING=5")
                .expect("a one-tone card resolves")
                .num_harmonics,
            3
        );
    }

    /// Two tones that share no low-order basis are refused by name rather
    /// than placed on a basis the spectrum cannot reach.
    #[test]
    fn incommensurate_tones_are_refused_by_name() {
        let error = HbConfig::from_hb_card(
            &tones_only(&[1.0e9, 1.0001e9]),
            &SimulationOptions::default(),
        )
        .expect_err("an incommensurate tone pair cannot be placed");
        assert_eq!(error.field(), "tones");
        assert_eq!(
            error.to_string(),
            "tones tone 0 requires common-basis harmonic 90000, beyond num_harmonics 4096"
        );
    }

    #[test]
    fn multi_tone_derives_the_common_basis() {
        // The mixer case from the type documentation: 900 + 800 MHz.
        let config = HbConfig::multi_tone(vec![
            HbTone::new(900e6, 5).with_name("RF"),
            HbTone::new(800e6, 5).with_name("LO"),
        ]);
        assert!(
            (config.fundamental_freq - 100e6).abs() < 1.0,
            "basis must be the 100 MHz common fundamental, got {}",
            config.fundamental_freq
        );
        // Both tones map to exact integer harmonics of the basis.
        for tone in &config.tones {
            let ratio = tone.frequency / config.fundamental_freq;
            assert!(
                (ratio - ratio.round()).abs() < 1e-9,
                "tone {} must land on an integer harmonic, got ratio {}",
                tone.name,
                ratio
            );
            let harmonic = ratio.round() as usize;
            assert!(
                harmonic * tone.num_harmonics <= config.num_harmonics,
                "harmonic budget must cover tone {} order {} at harmonic {}",
                tone.name,
                tone.num_harmonics,
                harmonic
            );
        }
    }

    #[test]
    fn multi_tone_with_harmonically_related_tones_keeps_the_lower_tone() {
        let config = HbConfig::multi_tone(vec![HbTone::new(1e9, 4), HbTone::new(2e9, 3)]);
        assert!(
            (config.fundamental_freq - 1e9).abs() < 1.0,
            "basis of harmonically related tones is the lower tone, got {}",
            config.fundamental_freq
        );
        assert!(config.num_harmonics >= 6);
    }

    #[test]
    fn multi_tone_fft_grid_represents_the_highest_common_basis_harmonic() {
        let config = HbConfig::multi_tone(vec![HbTone::new(900e6, 5), HbTone::new(800e6, 5)]);
        assert!(
            config.fft_size() > 2 * config.num_harmonics,
            "FFT grid {} cannot represent +/- harmonic {}",
            config.fft_size(),
            config.num_harmonics
        );
    }

    #[test]
    fn a_card_with_no_numfreq_keeps_the_core_default_order_and_grid() {
        let config = HbConfig::from_hb_card(&tones_only(&[1.0e9]), &SimulationOptions::default())
            .expect("a one-tone .HB resolves");
        assert_eq!(config.fundamental_freq, 1.0e9);
        assert_eq!(config.num_harmonics, HbConfig::new(1.0e9).num_harmonics);
        assert_eq!(
            config.collocation_points, None,
            "without NUMFREQ the solver's own grid is used"
        );
    }

    #[test]
    fn an_explicit_single_tone_order_pins_the_minimal_bilateral_grid() {
        let config = HbConfig::from_hb_card(&tones_only(&[1.0e9]), &numfreq(&[5]))
            .expect("an explicit order resolves");
        assert_eq!(config.num_harmonics, 5);
        assert_eq!(
            config.collocation_points,
            Some(11),
            "an explicit NUMFREQ asks for the minimal 2N+1 grid"
        );
    }

    #[test]
    fn one_order_broadcasts_across_every_tone() {
        let broadcast = HbConfig::from_hb_card(&tones_only(&[9.0e8, 8.0e8]), &numfreq(&[4]))
            .expect("broadcasting resolves");
        let paired = HbConfig::from_hb_card(&tones_only(&[9.0e8, 8.0e8]), &numfreq(&[4, 4]))
            .expect("pairing resolves");
        assert_eq!(broadcast.fundamental_freq, paired.fundamental_freq);
        assert_eq!(broadcast.num_harmonics, paired.num_harmonics);
        assert_eq!(broadcast.tones.len(), 2);
        assert_eq!(
            broadcast
                .tones
                .iter()
                .map(|tone| tone.num_harmonics)
                .collect::<Vec<_>>(),
            vec![4, 4]
        );
    }

    #[test]
    fn a_multi_tone_card_uses_the_core_common_basis_rule() {
        let config =
            HbConfig::from_hb_card(&tones_only(&[9.0e8, 8.0e8]), &SimulationOptions::default())
                .expect("a two-tone .HB resolves");
        let default_order = HbConfig::new(9.0e8).num_harmonics;
        let expected = HbConfig::multi_tone(vec![
            HbTone::new(9.0e8, default_order).with_name("tone1"),
            HbTone::new(8.0e8, default_order).with_name("tone2"),
        ]);
        assert_eq!(config.fundamental_freq, expected.fundamental_freq);
        assert_eq!(config.num_harmonics, expected.num_harmonics);
    }

    #[test]
    fn an_impossible_card_or_order_list_fails_closed() {
        for (frequencies, orders) in [
            (vec![], vec![]),
            (vec![0.0], vec![]),
            (vec![f64::NAN], vec![]),
            (vec![1.0e9, 1.0e9], vec![]),
            (vec![1.0e9], vec![0]),
            (vec![1.0e9, 2.0e9], vec![3, 4, 5]),
        ] {
            HbConfig::from_hb_card(&tones_only(&frequencies), &numfreq(&orders))
                .expect_err("an impossible .HB card must fail before any solve");
        }
    }
}
