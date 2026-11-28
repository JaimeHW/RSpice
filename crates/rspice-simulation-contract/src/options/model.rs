//! Simulator options.
//!
//! The tolerances, limits, and integration settings a run is executed under.

use super::{
    DampingStrategy, IntegrationMethod, MatrixSolver, SimulationCompatibility, SourceSimulator,
    ValidationError,
};

/// A named numerical policy: its label and how to build it.
pub type NamedPreset = (&'static str, fn() -> SimulationOptions);

/// The SPICE default transient truncation-error tolerance.
const fn default_trtol() -> f64 {
    7.0
}

/// Solver tolerances, limits, and numerical policies authored by a simulation plan.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationOptions {
    /// Numerical and available device-model variants, not the source syntax.
    #[serde(default, skip_serializing_if = "SimulationCompatibility::is_inherited")]
    pub compatibility: SimulationCompatibility,
    /// The simulator the project's deck was written for: its random functions
    /// draw as that program draws them, and LTspice's seven-value `EXP`
    /// repeats as LTspice repeats it.
    #[serde(default, skip_serializing_if = "SourceSimulator::is_native")]
    pub source_simulator: SourceSimulator,
    pub reltol: f64,
    pub residual_reltol: f64,
    pub vntol: f64,
    /// The one current tolerance, stated on the deck as `IABSTOL`: the
    /// spelling the resolver prefers over `ABSTOL`.
    pub iabstol: f64,
    pub chgtol: f64,
    pub pivrel: f64,
    pub pivtol: f64,
    pub itl1: usize,
    pub itl4: usize,
    /// Transient truncation-error tolerance. Scales the local truncation
    /// error the timestep controller will accept, so it is the one knob that
    /// trades transient run time against waveform fidelity directly.
    #[serde(default = "default_trtol")]
    pub trtol: f64,
    /// Relative bound on the accepted local truncation error. `None` leaves
    /// the engine's own bound in force rather than asserting one.
    #[serde(default)]
    pub transient_lte_reltol: Option<f64>,
    /// Absolute bound on the accepted local truncation error.
    #[serde(default)]
    pub transient_lte_abstol: Option<f64>,
    /// Seed for the deck's statistical expression functions — `agauss`,
    /// `gauss`, `unif`, `aunif`, and two-argument `limit`.
    ///
    /// `None` leaves the engine's default stream. Setting it is what makes a
    /// deck containing statistical parameters reproducible: without a stated
    /// seed the same project can be re-run and disagree with itself, and a
    /// result nobody can reproduce is not evidence.
    #[serde(default)]
    pub statistical_seed: Option<u64>,
    pub gmin_stepping: bool,
    pub source_stepping: bool,
    pub pseudo_transient: bool,
    pub arc_length: bool,
    pub gmin: f64,
    /// `.OPTIONS RSHUNT`: a resistance, in ohms, from every node to ground.
    /// `None` places none.
    ///
    /// Unlike GMIN it is part of the circuit, not of the numerical policy: it
    /// conducts in every analysis, is a noise source, and gives a node that a
    /// source forces current into a DC path the engine accepts. A preset
    /// therefore neither sets nor clears it. Not serialized while unset, so a
    /// plan that never states one keeps the bytes it had before it existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rshunt: Option<f64>,
    /// `.OPTIONS CONNECTRULES`: the Verilog-AMS `connectrules` block that
    /// converts at every analog/digital boundary the design makes. `None`
    /// states none, and the engine takes the design's only block; a design
    /// whose libraries declare two or more refuses every boundary until one
    /// is named.
    ///
    /// Like RSHUNT it decides the circuit rather than the numerics — which
    /// converters are elaborated into it — so a preset neither sets nor
    /// clears it. Not serialized while unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connect_rules: Option<String>,
    /// The run's own connect-rule block, written into the deck as a
    /// `.CONNECTRULES` card while [`Self::connect_rules`] names it. Kept when
    /// another block is selected, so choosing it again restores its values.
    /// Like the selection it decides the circuit, so a preset neither sets
    /// nor clears it. Not serialized while unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connect_rule_block: Option<super::RunConnectRules>,
    /// Nets held at a voltage while the operating point a transient starts
    /// from is solved, written as `.IC` cards. Like the connect rules they
    /// decide where the circuit starts rather than the numerics, so a preset
    /// neither sets nor clears them. Not serialized while empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_conditions: Vec<super::NodeVoltage>,
    /// Nets the operating point's search starts from, written as
    /// `.NODESET` cards; kept across presets as the initial conditions are.
    /// Not serialized while empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodesets: Vec<super::NodeVoltage>,
    pub damping: DampingStrategy,
    pub method: IntegrationMethod,
    pub solver: MatrixSolver,
    pub bypass_enabled: bool,
    pub bypass_reltol: f64,
    pub bypass_abstol: f64,
    pub min_timestep: f64,
    /// The run's step ceiling. `None` states none, so each transient steps at
    /// up to its own step time.
    pub max_timestep: Option<f64>,
    pub tnom: f64,
}

impl Default for SimulationOptions {
    fn default() -> Self {
        Self {
            compatibility: SimulationCompatibility::Inherit,
            source_simulator: SourceSimulator::Native,
            reltol: 1e-3,
            residual_reltol: 1e-3,
            vntol: 1e-6,
            iabstol: 1e-12,
            chgtol: 1e-14,
            pivrel: 1e-3,
            pivtol: 1e-13,
            itl1: 50,
            itl4: 6,
            trtol: default_trtol(),
            transient_lte_reltol: None,
            transient_lte_abstol: None,
            statistical_seed: None,
            gmin_stepping: true,
            source_stepping: true,
            pseudo_transient: true,
            arc_length: false,
            gmin: 1e-12,
            rshunt: None,
            connect_rules: None,
            connect_rule_block: None,
            initial_conditions: Vec::new(),
            nodesets: Vec::new(),
            damping: DampingStrategy::VoltageLimiting,
            method: IntegrationMethod::Trap,
            solver: MatrixSolver::Lu,
            bypass_enabled: false,
            bypass_reltol: 1e-3,
            bypass_abstol: 1e-6,
            min_timestep: 1e-15,
            max_timestep: None,
            tnom: 27.0,
        }
    }
}

impl SimulationOptions {
    /// Create options optimized for fast simulation (loose tolerances).
    pub fn fast() -> Self {
        Self {
            reltol: 1e-2,
            residual_reltol: 1e-2,
            iabstol: 1e-9,
            itl1: 30,
            itl4: 4,
            gmin_stepping: false,
            source_stepping: false,
            pseudo_transient: false,
            bypass_enabled: true,
            ..Default::default()
        }
    }

    /// Create options optimized for accuracy (tight tolerances).
    pub fn accurate() -> Self {
        Self {
            reltol: 1e-4,
            residual_reltol: 1e-4,
            vntol: 1e-7,
            iabstol: 1e-14,
            chgtol: 1e-16,
            itl1: 100,
            itl4: 10,
            damping: DampingStrategy::Combined,
            ..Default::default()
        }
    }

    /// Create options optimized for difficult/stiff circuits.
    pub fn robust() -> Self {
        Self {
            reltol: 1e-3,
            residual_reltol: 1e-3,
            itl1: 200,
            itl4: 20,
            gmin_stepping: true,
            source_stepping: true,
            pseudo_transient: true,
            arc_length: true,
            gmin: 1e-10,
            method: IntegrationMethod::Gear2,
            damping: DampingStrategy::Combined,
            ..Default::default()
        }
    }

    /// The named presets, in the order a chooser should offer them: loosest
    /// first, then the shipping default, then the two that trade time for
    /// convergence.
    pub const PRESETS: [NamedPreset; 4] = [
        ("Fast", Self::fast),
        ("Balanced", Self::default),
        ("Accurate", Self::accurate),
        ("Robust", Self::robust),
    ];

    /// How the Solver page labels [`Self::connect_rules`], so a finding that
    /// names the setting names the field the reader will find.
    pub const CONNECT_RULES_LABEL: &'static str = "Connect rules \u{b7} CONNECTRULES";

    /// These options with a named preset applied.
    ///
    /// A preset decides the tolerances, the iteration budgets and the
    /// convergence aids, and nothing else: the temperatures, the seed, the step
    /// limits, the compatibility, the node shunt, the connect rules and where
    /// the circuit starts are kept as they are.
    #[must_use]
    pub fn with_preset(&self, build: fn() -> Self) -> Self {
        let preset = build();
        Self {
            reltol: preset.reltol,
            residual_reltol: preset.residual_reltol,
            vntol: preset.vntol,
            iabstol: preset.iabstol,
            chgtol: preset.chgtol,
            trtol: preset.trtol,
            itl1: preset.itl1,
            itl4: preset.itl4,
            gmin: preset.gmin,
            gmin_stepping: preset.gmin_stepping,
            source_stepping: preset.source_stepping,
            pseudo_transient: preset.pseudo_transient,
            arc_length: preset.arc_length,
            damping: preset.damping,
            method: preset.method,
            bypass_enabled: preset.bypass_enabled,
            ..self.clone()
        }
    }

    /// Which named preset these options are exactly, if any.
    ///
    /// Compared by value over the fields a preset decides rather than by a
    /// remembered "last preset pressed": editing a tolerance leaves the
    /// preset, and a field no preset decides never does. Returns `None` for
    /// options that match no preset.
    #[must_use]
    pub fn preset_name(&self) -> Option<&'static str> {
        let current = serde_json::to_vec(self).ok()?;
        Self::PRESETS.iter().find_map(|(label, build)| {
            serde_json::to_vec(&self.with_preset(*build))
                .ok()
                .filter(|preset| *preset == current)
                .map(|_| *label)
        })
    }

    /// The run's own connect-rule block while the selection names it, which
    /// is when the deck writes it and the engine converts with it.
    #[must_use]
    pub fn written_connect_rule_block(&self) -> Option<&super::RunConnectRules> {
        self.connect_rule_block
            .as_ref()
            .filter(|block| self.connect_rules.as_deref() == Some(block.name.as_str()))
    }

    /// The active preset's name, or `Custom` when the options match none.
    #[must_use]
    pub fn preset_label(&self) -> String {
        self.preset_name()
            .map_or_else(|| "Custom".to_owned(), str::to_owned)
    }

    /// Validate the options before they become an effective solver policy.
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        for (name, value) in [
            ("reltol", self.reltol),
            ("residual_reltol", self.residual_reltol),
            ("vntol", self.vntol),
            ("iabstol", self.iabstol),
            ("chgtol", self.chgtol),
            ("pivtol", self.pivtol),
            ("trtol", self.trtol),
        ] {
            if !value.is_finite() || value <= 0.0 {
                errors.push(ValidationError::InvalidTolerance(name, value));
            }
        }
        if !self.pivrel.is_finite() || self.pivrel <= 0.0 || self.pivrel > 1.0 {
            errors.push(ValidationError::InvalidPivotRelative(self.pivrel));
        }
        for (name, value) in [
            ("gmin", self.gmin),
            ("bypass_reltol", self.bypass_reltol),
            ("bypass_abstol", self.bypass_abstol),
        ] {
            if !value.is_finite() || value < 0.0 {
                errors.push(ValidationError::InvalidNonNegative(name, value));
            }
        }
        for (name, value) in [
            ("transient_lte_reltol", self.transient_lte_reltol),
            ("transient_lte_abstol", self.transient_lte_abstol),
            // The parser refuses `RSHUNT=0` and below: a zero-ohm shunt would
            // short every node to ground.
            ("rshunt", self.rshunt),
        ] {
            if let Some(value) = value
                && (!value.is_finite() || value <= 0.0)
            {
                errors.push(ValidationError::InvalidTolerance(name, value));
            }
        }
        for (name, value) in [("itl1", self.itl1), ("itl4", self.itl4)] {
            if value == 0 {
                errors.push(ValidationError::InvalidIteration(name, value));
            }
        }
        let stated_steps = [
            ("min_timestep", Some(self.min_timestep)),
            ("max_timestep", self.max_timestep),
        ];
        for (name, value) in stated_steps {
            if let Some(value) = value.filter(|value| !value.is_finite() || *value <= 0.0) {
                errors.push(ValidationError::InvalidTimestep(name, value));
            }
        }
        if let Some(max) = self.max_timestep.filter(|max| self.min_timestep >= *max) {
            errors.push(ValidationError::TimestepOrder(self.min_timestep, max));
        }
        if !self.tnom.is_finite() || self.tnom <= -273.15 {
            errors.push(ValidationError::InvalidTemperature("tnom", self.tnom));
        }
        // A block is named by a simple Verilog-AMS identifier. Anything else
        // names no block a library can declare, and could break the card.
        if let Some(block) = &self.connect_rules
            && !is_simple_identifier(block)
        {
            errors.push(ValidationError::InvalidConnectRules(block.clone()));
        }
        if let Some(block) = &self.connect_rule_block {
            errors.extend(
                block
                    .problems()
                    .into_iter()
                    .map(ValidationError::InvalidConnectRuleBlock),
            );
        }
        for (kind, rows) in [
            (
                super::StartupKind::InitialCondition,
                &self.initial_conditions,
            ),
            (super::StartupKind::NodeSet, &self.nodesets),
        ] {
            errors.extend(
                super::startup_problems(kind, rows)
                    .into_iter()
                    .map(ValidationError::InvalidStartupRow),
            );
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Export as SPICE .options string without rounding authored values.
    /// Exact default comparisons also preserve changes smaller than a
    /// display tolerance; these bytes are solver input.
    ///
    /// The temperature is the plan's reference temperature, which the run
    /// set stores; the card states it as `TEMP` unless it is the engine's own.
    pub fn to_spice_options(&self, temperature_celsius: f64) -> String {
        let mut lines = vec![".OPTIONS".to_string()];
        let default = Self::default();

        if let Some(policy) = self.compatibility.option_value() {
            lines.push(format!("+ RSPICE_DIALECT={policy}"));
        }
        if !self.source_simulator.is_native() {
            lines.push("+ RSPICE_STATISTICS=SAMPLE".to_owned());
        }
        if self.source_simulator == SourceSimulator::Ltspice {
            lines.push("+ RSPICE_EXP_REPEAT=LTSPICE".to_owned());
            lines.push("+ RSPICE_SOURCE=LTSPICE".to_owned());
        }
        if self.source_simulator == SourceSimulator::Qspice {
            lines.push("+ RSPICE_SOURCE=QSPICE".to_owned());
        }
        if self.reltol != default.reltol {
            lines.push(format!("+ RELTOL={:e}", self.reltol));
        }
        if self.residual_reltol != default.residual_reltol {
            lines.push(format!("+ RESIDUAL_RELTOL={:e}", self.residual_reltol));
        }
        if self.vntol != default.vntol {
            lines.push(format!("+ VNTOL={:e}", self.vntol));
        }
        if self.iabstol != default.iabstol {
            lines.push(format!("+ IABSTOL={:e}", self.iabstol));
        }
        if self.chgtol != default.chgtol {
            lines.push(format!("+ CHGTOL={:e}", self.chgtol));
        }
        if self.pivrel.to_bits() != default.pivrel.to_bits() {
            lines.push(format!("+ PIVREL={:e}", self.pivrel));
        }
        // The product policy's default is deliberately non-zero while the
        // core fallback is zero. Always state it so the ledger and solve
        // cannot disagree when the user leaves this field untouched.
        lines.push(format!("+ PIVTOL={:e}", self.pivtol));
        if self.itl1 != default.itl1 {
            lines.push(format!("+ ITL1={}", self.itl1));
        }
        // The product's transient Newton budget differs from the core
        // fallback. Omission would execute ten iterations while Studio shows
        // six, so this value is an explicit part of every prepared deck.
        lines.push(format!("+ ITL4={}", self.itl4));
        if self.trtol.to_bits() != default.trtol.to_bits() {
            lines.push(format!("+ TRTOL={}", self.trtol));
        }
        // Emitted rather than applied through the override path: the parser
        // seeds the statistical stream before any parameter is evaluated, so
        // the seed has to be in the deck to reach the draws it governs.
        if let Some(seed) = self.statistical_seed {
            lines.push(format!("+ SEED={seed}"));
        }
        if self.method != default.method {
            lines.push(format!("+ METHOD={}", self.method.spice_name()));
        }
        if temperature_celsius != crate::run_set::ReferencePoint::default().temperature_celsius {
            lines.push(format!("+ TEMP={temperature_celsius}"));
        }
        if self.tnom != default.tnom {
            lines.push(format!("+ TNOM={}", self.tnom));
        }
        if self.gmin != default.gmin {
            lines.push(format!("+ GMIN={:e}", self.gmin));
        }
        // Stated only when asked for: with none, the deck holds no resistor
        // nobody drew.
        if let Some(rshunt) = self.rshunt {
            lines.push(format!("+ RSHUNT={rshunt:e}"));
        }
        // Quoted, so the parser keeps the case-sensitive block name as it was
        // written. With none, the engine takes the design's only block.
        if let Some(block) = &self.connect_rules {
            lines.push(format!("+ CONNECTRULES=\"{block}\""));
        }
        if self.gmin_stepping != default.gmin_stepping {
            lines.push(format!("+ GMINSTEPPING={}", u8::from(self.gmin_stepping)));
        }
        if self.source_stepping != default.source_stepping {
            lines.push(format!(
                "+ SOURCESTEPPING={}",
                u8::from(self.source_stepping)
            ));
        }
        if self.pseudo_transient != default.pseudo_transient {
            lines.push(format!(
                "+ PSEUDOTRANSIENT={}",
                u8::from(self.pseudo_transient)
            ));
        }
        if self.arc_length != default.arc_length {
            lines.push(format!("+ ARCLENGTH={}", u8::from(self.arc_length)));
        }
        // Unscoped, so the bounds ride the global card and re-scope nothing
        // after them. They are stated only alongside an enabled bypass: with
        // the feature off they select nothing, and emitting them would put a
        // key in the deck that changes no result.
        if self.bypass_enabled != default.bypass_enabled {
            lines.push(format!("+ BYPASS={}", u8::from(self.bypass_enabled)));
        }
        if self.bypass_enabled {
            if self.bypass_reltol != default.bypass_reltol {
                lines.push(format!("+ BYPASSRELTOL={:e}", self.bypass_reltol));
            }
            if self.bypass_abstol != default.bypass_abstol {
                lines.push(format!("+ BYPASSABSTOL={:e}", self.bypass_abstol));
            }
        }
        if self.damping != default.damping {
            lines.push(format!("+ DAMPING={}", self.damping.spice_name()));
        }
        if let Some(backend) = self.solver.spice_name() {
            lines.push(format!("+ SOLVER={backend}"));
        }
        // The run's step ceiling is unscoped. `TIMEINT DELMAX` is the time
        // integrator's own ceiling and belongs to the per-analysis override
        // record; stating the plan's ceiling under that key would make one of
        // the two bounds unstatable. A plan that states none leaves each
        // transient's own step time as its ceiling.
        if let Some(max_timestep) = self.max_timestep {
            lines.push(format!("+ MAXTIMESTEP={max_timestep:e}"));
        }

        // The parser's package selector stays in force for the rest of the
        // `.OPTIONS` command it appears on, so a scoped key placed among the
        // global ones would re-scope every key after it. The timestep
        // integrator's settings therefore get their own card.
        let mut timeint = Vec::new();
        if let Some(reltol) = self.transient_lte_reltol {
            timeint.push(format!("+ RELTOL={reltol:e}"));
        }
        if let Some(abstol) = self.transient_lte_abstol {
            timeint.push(format!("+ ABSTOL={abstol:e}"));
        }
        // The product floor is lower than the core fallback and therefore
        // must be stated even for the untouched shipping policy.
        timeint.push(format!("+ MINTIMESTEP={:e}", self.min_timestep));
        if !timeint.is_empty() {
            // A card whose only content is the global header states nothing,
            // and would re-scope nothing either; drop it rather than emit it.
            if lines.len() == 1 {
                lines.clear();
            }
            lines.push(".OPTIONS TIMEINT".to_string());
            lines.append(&mut timeint);
        }

        // The run's own block, while the selection names it: the deck then
        // carries every value it converts with.
        if let Some(block) = self.written_connect_rule_block() {
            if lines.len() == 1 && lines[0] == ".OPTIONS" {
                lines.clear();
            }
            lines.push(block.card());
        }

        // Where the run starts: one card per net.
        let startup = super::startup_cards(
            super::StartupKind::InitialCondition,
            &self.initial_conditions,
        )
        .into_iter()
        .chain(super::startup_cards(
            super::StartupKind::NodeSet,
            &self.nodesets,
        ))
        .collect::<Vec<_>>();
        if !startup.is_empty() {
            if lines.len() == 1 && lines[0] == ".OPTIONS" {
                lines.clear();
            }
            lines.extend(startup);
        }

        lines.join("\n")
    }
}

/// A Verilog-AMS simple identifier: a letter or underscore, then letters,
/// digits, underscores and dollar signs.
pub(super) fn is_simple_identifier(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters.all(|rest| rest.is_ascii_alphanumeric() || rest == '_' || rest == '$')
}

#[cfg(test)]
mod tests;
