//! Simulator options.
//!
//! The tolerances, limits, and integration settings a run is executed under.

use super::{DampingStrategy, IntegrationMethod, MatrixSolver, ValidationError};

/// A named numerical policy: its label and how to build it.
pub type NamedPreset = (&'static str, fn() -> SimulationOptions);

/// The SPICE default transient truncation-error tolerance.
const fn default_trtol() -> f64 {
    7.0
}

/// Complete simulation options matching Cadence Spectre.
///
/// These options control all aspects of simulation accuracy, convergence,
/// and performance. Default values match industry-standard SPICE defaults.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(from = "PersistedSimulationOptions")]
pub struct SimulationOptions {
    pub reltol: f64,
    pub residual_reltol: f64,
    pub vntol: f64,
    pub abstol: f64,
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
    pub damping: DampingStrategy,
    pub method: IntegrationMethod,
    pub solver: MatrixSolver,
    pub bypass_enabled: bool,
    pub bypass_reltol: f64,
    pub bypass_abstol: f64,
    pub min_timestep: f64,
    pub max_timestep: f64,
    pub temp: f64,
    pub tnom: f64,
}

/// Persisted options. New fields serialize; retired fields only decode.
///
/// Every field below that decodes into nothing named a control the engine
/// never read, so a project written before it was retired still opens and
/// simply stops carrying it:
///
/// - `itl2` was a DC-transfer-curve iteration budget, but the sweep and the
///   operating point share one Newton budget.
/// - `timestep_factor` claimed to set the transient step growth ratio, which
///   is the compile-time `constants::TIMESTEP_GROWTH_MAX`, not a setting.
/// - `verbose` fed `ConvergenceConfig::verbose`, which no solver path reads.
/// - `save_internals` had no engine field at all; internal device nodes are
///   requested per signal on a `.SAVE` card.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedSimulationOptions {
    #[serde(default)]
    #[allow(dead_code)]
    itl2: serde::de::IgnoredAny,
    #[serde(default)]
    #[allow(dead_code)]
    timestep_factor: serde::de::IgnoredAny,
    #[serde(default)]
    #[allow(dead_code)]
    verbose: serde::de::IgnoredAny,
    #[serde(default)]
    #[allow(dead_code)]
    save_internals: serde::de::IgnoredAny,
    reltol: f64,
    residual_reltol: f64,
    vntol: f64,
    abstol: f64,
    iabstol: f64,
    chgtol: f64,
    pivrel: f64,
    pivtol: f64,
    itl1: usize,
    itl4: usize,
    #[serde(default = "default_trtol")]
    trtol: f64,
    #[serde(default)]
    transient_lte_reltol: Option<f64>,
    #[serde(default)]
    transient_lte_abstol: Option<f64>,
    #[serde(default)]
    statistical_seed: Option<u64>,
    gmin_stepping: bool,
    source_stepping: bool,
    pseudo_transient: bool,
    arc_length: bool,
    gmin: f64,
    damping: DampingStrategy,
    method: IntegrationMethod,
    solver: MatrixSolver,
    bypass_enabled: bool,
    bypass_reltol: f64,
    bypass_abstol: f64,
    min_timestep: f64,
    max_timestep: f64,
    temp: f64,
    tnom: f64,
}

impl From<PersistedSimulationOptions> for SimulationOptions {
    fn from(fields: PersistedSimulationOptions) -> Self {
        Self {
            reltol: fields.reltol,
            residual_reltol: fields.residual_reltol,
            vntol: fields.vntol,
            abstol: fields.abstol,
            iabstol: fields.iabstol,
            chgtol: fields.chgtol,
            pivrel: fields.pivrel,
            pivtol: fields.pivtol,
            itl1: fields.itl1,
            itl4: fields.itl4,
            trtol: fields.trtol,
            transient_lte_reltol: fields.transient_lte_reltol,
            transient_lte_abstol: fields.transient_lte_abstol,
            statistical_seed: fields.statistical_seed,
            gmin_stepping: fields.gmin_stepping,
            source_stepping: fields.source_stepping,
            pseudo_transient: fields.pseudo_transient,
            arc_length: fields.arc_length,
            gmin: fields.gmin,
            damping: fields.damping,
            method: fields.method,
            solver: fields.solver,
            bypass_enabled: fields.bypass_enabled,
            bypass_reltol: fields.bypass_reltol,
            bypass_abstol: fields.bypass_abstol,
            min_timestep: fields.min_timestep,
            max_timestep: fields.max_timestep,
            temp: fields.temp,
            tnom: fields.tnom,
        }
    }
}

#[cfg(test)]
// The round-trip contract is intentionally adjacent to the persistence
// translation it authenticates; the remaining impls are serialization code.
#[allow(clippy::items_after_test_module)]
mod tests {
    use super::*;

    /// Resolve `options` exactly the way a run does.
    ///
    /// The deck is the only channel these options have: the block is spliced
    /// into the prepared source, the engine re-parses that source, and every
    /// shipping runner resolves the parsed `.OPTIONS` against the core
    /// defaults with no override layer. Asserting on the emitted string
    /// instead would pass for a key the parser does not know, which is how
    /// fields drifted out of the engine in the first place.
    fn parse_through_the_deck(options: &SimulationOptions) -> rspice_core::Netlist {
        let deck = crate::simulation::SimulationController::apply_simulation_options_to_netlist(
            "round trip\nV1 1 0 1\nR1 1 0 1k\n.op\n.end\n",
            options,
        );
        rspice_core::netlist::parse_netlist(&deck)
            .unwrap_or_else(|error| panic!("the emitted deck must parse: {error}\n{deck}"))
    }

    fn resolve_through_the_deck(
        options: &SimulationOptions,
    ) -> rspice_core::engine::SimulationConfig {
        let netlist = parse_through_the_deck(options);
        rspice_core::resolve_simulation_config(
            &rspice_core::engine::SimulationConfig::default(),
            Some(&netlist.options),
            &rspice_core::SimulationConfigOverrides::default(),
        )
    }

    #[test]
    fn the_newly_connected_controls_survive_the_deck_round_trip() {
        let options = SimulationOptions {
            iabstol: 7.0e-13,
            chgtol: 3.0e-16,
            transient_lte_reltol: Some(2.5e-4),
            transient_lte_abstol: Some(3.5e-9),
            gmin_stepping: false,
            source_stepping: false,
            pseudo_transient: false,
            arc_length: true,
            damping: DampingStrategy::Combined,
            solver: MatrixSolver::Klu,
            min_timestep: 2.0e-18,
            ..SimulationOptions::default()
        };

        let resolved = resolve_through_the_deck(&options);

        assert_eq!(resolved.convergence_config.current_abstol, 7.0e-13);
        assert_eq!(resolved.convergence_config.charge_abstol, 3.0e-16);
        assert_eq!(resolved.transient_lte_reltol, Some(2.5e-4));
        assert_eq!(resolved.transient_lte_abstol, Some(3.5e-9));
        assert!(!resolved.convergence_config.gmin_stepping);
        assert!(!resolved.convergence_config.source_stepping);
        assert!(!resolved.convergence_config.pseudo_transient);
        assert!(resolved.convergence_config.arc_length);
        assert_eq!(
            resolved.convergence_config.damping_strategy,
            rspice_core::engine::DampingStrategy::Combined
        );
        assert_eq!(
            resolved.matrix_solver,
            Some(rspice_core::solver::RealSolverBackend::Klu)
        );
        assert_eq!(resolved.min_timestep, 2.0e-18);
    }

    #[test]
    fn the_timeint_card_leaves_the_global_tolerances_alone() {
        // The two LTE bounds are spelled RELTOL and ABSTOL inside the TIMEINT
        // package, which are also global key names. They travel on their own
        // card so the package selector cannot re-scope the keys after them,
        // and they must not land on the global tolerances either.
        let options = SimulationOptions {
            reltol: 4.0e-4,
            abstol: 6.0e-13,
            transient_lte_reltol: Some(2.5e-7),
            transient_lte_abstol: Some(3.5e-11),
            temp: 85.0,
            ..SimulationOptions::default()
        };

        let resolved = resolve_through_the_deck(&options);

        assert_eq!(resolved.transient_lte_reltol, Some(2.5e-7));
        assert_eq!(resolved.transient_lte_abstol, Some(3.5e-11));
        assert_eq!(resolved.convergence_config.voltage_reltol, 4.0e-4);
        assert_eq!(resolved.convergence_config.current_abstol, 6.0e-13);
        // TEMP is emitted after the global tolerances and before the TIMEINT
        // card; a scoped key emitted inline would have swallowed it.
        assert_eq!(resolved.temperature, 85.0 + 273.15);
    }

    #[test]
    fn the_bypass_controls_reach_the_resolved_bypass_config() {
        let options = SimulationOptions {
            bypass_enabled: true,
            bypass_reltol: 5.0e-4,
            bypass_abstol: 2.0e-7,
            ..SimulationOptions::default()
        };

        let resolved = resolve_through_the_deck(&options);

        assert!(resolved.bypass_config.enabled);
        assert_eq!(resolved.bypass_config.reltol, 5.0e-4);
        assert_eq!(resolved.bypass_config.abstol, 2.0e-7);
    }

    /// The Solver page's bypass voltage floor is the only editor of
    /// `bypass_abstol`, so the field is pinned separately from the relative
    /// bound: a regression that dropped `BYPASSABSTOL` from the emitter would
    /// otherwise still pass while the floor silently reverted to the core
    /// default the two share nothing with.
    #[test]
    fn the_bypass_voltage_floor_reaches_the_engine_on_its_own() {
        let default_floor = resolve_through_the_deck(&SimulationOptions {
            bypass_enabled: true,
            ..SimulationOptions::default()
        })
        .bypass_config
        .abstol;
        let options = SimulationOptions {
            bypass_enabled: true,
            bypass_abstol: 4.0e-9,
            ..SimulationOptions::default()
        };

        let resolved = resolve_through_the_deck(&options);

        assert_ne!(
            4.0e-9, default_floor,
            "the fixture value has to differ from the untouched one to prove anything"
        );
        assert_eq!(resolved.bypass_config.abstol, 4.0e-9);
        assert_eq!(
            resolved.bypass_config.reltol,
            SimulationOptions::default().bypass_reltol,
            "editing the floor must not disturb the relative bound beside it"
        );
    }

    /// TNOM is the model reference temperature, and it does not travel on
    /// `SimulationConfig` the way TEMP does: the builder reads
    /// `netlist.options.tnom` when it resolves each model card. So the round
    /// trip is asserted where the value actually lands, and against TEMP, to
    /// pin that the two temperatures stay separate keys.
    #[test]
    fn the_model_reference_temperature_reaches_the_parsed_deck_separately_from_temp() {
        let options = SimulationOptions {
            temp: 85.0,
            tnom: 40.0,
            ..SimulationOptions::default()
        };

        let netlist = parse_through_the_deck(&options);

        assert_eq!(netlist.options.tnom, Some(40.0));
        assert_eq!(
            resolve_through_the_deck(&options).temperature,
            85.0 + 273.15,
            "TNOM must not be read as the simulation temperature"
        );
    }

    #[test]
    fn an_untouched_model_reference_temperature_states_nothing() {
        assert!(
            !SimulationOptions::default()
                .to_spice_options()
                .contains("TNOM"),
            "the shipping TNOM matches the engine's own, so the deck states no opinion"
        );
        assert_eq!(
            parse_through_the_deck(&SimulationOptions::default())
                .options
                .tnom,
            None
        );
    }

    #[test]
    fn bypass_stays_off_and_states_no_bounds_when_the_page_leaves_it_alone() {
        let resolved = resolve_through_the_deck(&SimulationOptions::default());
        assert!(!resolved.bypass_config.enabled);
        assert!(
            !SimulationOptions::default()
                .to_spice_options()
                .contains("BYPASS")
        );
    }

    #[test]
    fn shipping_solver_policy_reaches_the_engine_even_where_core_defaults_differ() {
        let options = SimulationOptions::default();
        let resolved = resolve_through_the_deck(&options);

        assert_eq!(resolved.transient_max_iterations, options.itl4);
        assert_eq!(
            resolved.matrix_absolute_pivot_tolerance.to_bits(),
            options.pivtol.to_bits()
        );
        assert_eq!(
            resolved.min_timestep.to_bits(),
            options.min_timestep.to_bits()
        );
        assert_eq!(
            resolved.max_timestep.to_bits(),
            options.max_timestep.to_bits()
        );
    }

    #[test]
    fn the_bypass_bounds_do_not_leak_into_the_timeint_package() {
        // BYPASS rides the global card, and the LTE bounds ride TIMEINT. If the
        // bypass keys ever moved behind a package selector, one of these two
        // groups would swallow the other.
        let options = SimulationOptions {
            bypass_enabled: true,
            bypass_reltol: 5.0e-4,
            transient_lte_reltol: Some(2.5e-7),
            transient_lte_abstol: Some(3.5e-11),
            ..SimulationOptions::default()
        };

        let resolved = resolve_through_the_deck(&options);

        assert_eq!(resolved.bypass_config.reltol, 5.0e-4);
        assert_eq!(resolved.transient_lte_reltol, Some(2.5e-7));
        assert_eq!(resolved.transient_lte_abstol, Some(3.5e-11));
        assert_eq!(
            resolved.convergence_config.voltage_reltol,
            rspice_core::engine::SimulationConfig::default()
                .convergence_config
                .voltage_reltol
        );
    }

    #[test]
    fn every_offered_solver_reaches_the_backend_it_names() {
        for solver in MatrixSolver::all() {
            let options = SimulationOptions {
                solver: *solver,
                ..SimulationOptions::default()
            };
            assert_eq!(
                resolve_through_the_deck(&options).matrix_solver,
                solver.core_backend_override(),
                "{} must resolve to the backend the page names",
                solver.display_name()
            );
        }
    }

    #[test]
    fn a_step_floor_alone_still_reaches_the_deck() {
        // The shipping policy always states values that differ from core
        // defaults. The edited floor must remain on its own TIMEINT card and
        // must not disturb those authoritative global values.
        let options = SimulationOptions {
            min_timestep: 2.0e-18,
            ..SimulationOptions::default()
        };

        assert_eq!(
            options.to_spice_options(),
            ".OPTIONS\n+ PIVTOL=1.00e-13\n+ ITL4=6\n+ MAXTIMESTEP=1.00e-3\n.OPTIONS TIMEINT\n+ MINTIMESTEP=2.00e-18"
        );
        assert_eq!(resolve_through_the_deck(&options).min_timestep, 2.0e-18);
    }

    #[test]
    fn every_offered_integration_method_names_one_the_engine_knows() {
        for method in IntegrationMethod::all() {
            let options = SimulationOptions {
                method: *method,
                ..SimulationOptions::default()
            };
            assert_eq!(
                resolve_through_the_deck(&options).integration_method,
                options.core_integration_method(),
                "{} must resolve to the method the page names",
                method.display_name()
            );
        }
    }

    #[test]
    fn the_run_step_ceiling_reaches_the_engine_through_the_deck() {
        let options = SimulationOptions {
            max_timestep: 4.0e-9,
            ..SimulationOptions::default()
        };

        assert_eq!(
            resolve_through_the_deck(&options).max_timestep,
            4.0e-9,
            "the transient clamps its step against this field, so the deck must carry it"
        );
    }

    #[test]
    fn the_plan_ceiling_and_an_analysis_step_ceiling_bound_the_step_separately() {
        // A run splices the plan's options block and then the analysis's own
        // override block into the same deck. The two ceilings are different
        // engine fields and the transient applies both, so each must arrive
        // whole: a key that overwrote the other would silently drop a bound
        // the page and the ledger both still report.
        let options = SimulationOptions {
            max_timestep: 4.0e-9,
            ..SimulationOptions::default()
        };
        let mut record = crate::simulation::plan::AnalysisNumericOverride::default();
        record
            .set(
                crate::simulation::plan::AnalysisKind::Fourier,
                crate::simulation::plan::NumericOverrideOption::MaximumTimestep,
                "700p",
            )
            .expect("a Fourier measurement runs a transient");

        let deck = format!(
            "two ceilings\nV1 1 0 1\nR1 1 0 1k\n{}\n{}\n.op\n.end\n",
            options.to_spice_options(),
            record.to_spice_options()
        );
        let netlist = rspice_core::netlist::parse_netlist(&deck)
            .unwrap_or_else(|error| panic!("the spliced deck must parse: {error}\n{deck}"));
        let resolved = rspice_core::resolve_simulation_config(
            &rspice_core::engine::SimulationConfig::default(),
            Some(&netlist.options),
            &rspice_core::SimulationConfigOverrides::default(),
        );

        assert_eq!(resolved.max_timestep, 4.0e-9);
        assert_eq!(resolved.transient_timeint_max_timestep, Some(7.0e-10));
    }

    #[test]
    fn a_plan_ceiling_alone_leaves_the_integrators_ceiling_unstated() {
        let options = SimulationOptions {
            max_timestep: 4.0e-9,
            ..SimulationOptions::default()
        };

        assert_eq!(
            resolve_through_the_deck(&options).transient_timeint_max_timestep,
            None
        );
    }

    #[test]
    fn a_project_saved_with_the_retired_gear_only_method_still_opens() {
        let mut persisted =
            serde_json::to_value(SimulationOptions::default()).expect("options encode");
        persisted
            .as_object_mut()
            .expect("options are an object")
            .insert("method".to_owned(), serde_json::json!("Gear2Only"));

        let restored: SimulationOptions =
            serde_json::from_value(persisted).expect("a project naming the retired method decodes");

        assert_eq!(restored.method, IntegrationMethod::Gear2);
    }

    #[test]
    fn a_project_saved_with_the_retired_plain_gear_method_still_opens() {
        // `Gear` and `Gear2` both resolved to the engine's one second-order
        // Gear integrator, so a project that chose either was always running
        // the survivor.
        let mut persisted =
            serde_json::to_value(SimulationOptions::default()).expect("options encode");
        persisted
            .as_object_mut()
            .expect("options are an object")
            .insert("method".to_owned(), serde_json::json!("Gear"));

        let restored: SimulationOptions =
            serde_json::from_value(persisted).expect("a project naming the retired method decodes");

        assert_eq!(restored.method, IntegrationMethod::Gear2);
    }

    #[test]
    fn a_project_saved_with_the_retired_controls_still_opens() {
        // The shape refuses unknown fields so a typo in a project file is
        // caught rather than dropped, which means every retired key has to be
        // named explicitly for older projects to keep opening.
        let mut persisted =
            serde_json::to_value(SimulationOptions::default()).expect("options encode");
        let object = persisted.as_object_mut().expect("options are an object");
        object.insert("itl2".to_owned(), serde_json::json!(50));
        object.insert("timestep_factor".to_owned(), serde_json::json!(16.0));
        object.insert("verbose".to_owned(), serde_json::json!(true));
        object.insert("save_internals".to_owned(), serde_json::json!(true));

        let restored: SimulationOptions = serde_json::from_value(persisted)
            .expect("a project written before these controls were retired decodes");

        assert_eq!(
            serde_json::to_value(&restored).expect("restored options re-encode"),
            serde_json::to_value(SimulationOptions::default()).expect("options encode"),
            "a retired key must decode away, not resurface on the next save"
        );
    }

    #[test]
    fn matrix_selection_and_pivrel_reach_core_configuration() {
        for (solver, expected) in [
            (MatrixSolver::Lu, None),
            (
                MatrixSolver::SparseLu,
                Some(rspice_core::solver::RealSolverBackend::Faer),
            ),
            (
                MatrixSolver::Klu,
                Some(rspice_core::solver::RealSolverBackend::Klu),
            ),
            (MatrixSolver::Gmres, None),
        ] {
            let options = SimulationOptions {
                solver,
                pivrel: 0.125,
                pivtol: 2.5e-14,
                ..SimulationOptions::default()
            };
            let config = options.resolve_simulation_config(None);
            assert_eq!(config.matrix_solver, expected);
            assert_eq!(config.matrix_pivot_tolerance, 0.125);
            assert_eq!(config.matrix_absolute_pivot_tolerance, 2.5e-14);
        }
    }

    #[test]
    fn truncation_error_controls_reach_the_core_configuration() {
        let options = SimulationOptions {
            trtol: 1.5,
            transient_lte_reltol: Some(2.5e-4),
            transient_lte_abstol: Some(3.5e-9),
            ..SimulationOptions::default()
        };

        let overrides = options.simulation_config_overrides();

        assert_eq!(overrides.transient_trtol, Some(1.5));
        assert_eq!(overrides.transient_lte_reltol, Some(2.5e-4));
        assert_eq!(overrides.transient_lte_abstol, Some(3.5e-9));
    }

    #[test]
    fn an_unset_truncation_bound_leaves_the_engines_own_in_force() {
        let overrides = SimulationOptions::default().simulation_config_overrides();

        assert_eq!(
            overrides.transient_trtol,
            Some(7.0),
            "TRTOL always has a value, and it is the SPICE default"
        );
        assert_eq!(overrides.transient_lte_reltol, None);
        assert_eq!(overrides.transient_lte_abstol, None);
    }

    #[test]
    fn projects_written_before_trtol_existed_open_on_the_spice_default() {
        let mut persisted =
            serde_json::to_value(SimulationOptions::default()).expect("options encode");
        let object = persisted.as_object_mut().expect("options are an object");
        object.remove("trtol");
        object.remove("transient_lte_reltol");
        object.remove("transient_lte_abstol");

        let restored: SimulationOptions =
            serde_json::from_value(persisted).expect("legacy options decode");

        assert_eq!(restored.trtol, 7.0);
        assert_eq!(restored.transient_lte_reltol, None);
    }

    #[test]
    fn a_stated_statistical_seed_reaches_the_deck() {
        let options = SimulationOptions {
            statistical_seed: Some(20260811),
            ..SimulationOptions::default()
        };

        assert!(
            options.to_spice_options().contains("SEED=20260811"),
            "the parser seeds the statistical stream from the deck, so the seed must be in it"
        );
    }

    #[test]
    fn no_seed_emits_no_seed_line() {
        assert!(
            !SimulationOptions::default()
                .to_spice_options()
                .contains("SEED")
        );
    }

    #[test]
    fn spice_export_preserves_nondefault_pivot_controls() {
        let options = SimulationOptions {
            pivrel: 0.25,
            pivtol: 2.0e-14,
            ..SimulationOptions::default()
        };
        let text = options.to_spice_options();
        assert!(text.contains("PIVREL=2.50e-1"));
        assert!(text.contains("PIVTOL=2.00e-14"));
    }
}

impl Default for SimulationOptions {
    fn default() -> Self {
        Self {
            reltol: 1e-3,
            residual_reltol: 1e-3,
            vntol: 1e-6,
            abstol: 1e-12,
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
            damping: DampingStrategy::VoltageLimiting,
            method: IntegrationMethod::TrapGear,
            solver: MatrixSolver::Lu,
            bypass_enabled: false,
            bypass_reltol: 1e-3,
            bypass_abstol: 1e-6,
            min_timestep: 1e-15,
            max_timestep: 1e-3,
            temp: 27.0,
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
            abstol: 1e-9,
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
            abstol: 1e-14,
            iabstol: 1e-14,
            chgtol: 1e-16,
            itl1: 100,
            itl4: 10,
            min_timestep: 1e-18,
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

    /// Which named preset these options are exactly, if any.
    ///
    /// Compared by serialized value rather than by a remembered "last preset
    /// pressed": editing any field leaves the preset, and reporting otherwise
    /// would misstate what the next run will use. Returns `None` for options
    /// that match no preset, which is a real state rather than an error.
    #[must_use]
    pub fn preset_name(&self) -> Option<&'static str> {
        let current = serde_json::to_vec(self).ok()?;
        Self::PRESETS.iter().find_map(|(label, build)| {
            serde_json::to_vec(&build())
                .ok()
                .filter(|preset| *preset == current)
                .map(|_| *label)
        })
    }

    /// The active preset's name, or `Custom` when the options match none.
    #[must_use]
    pub fn preset_label(&self) -> String {
        self.preset_name()
            .map_or_else(|| "Custom".to_owned(), str::to_owned)
    }

    pub fn temp_kelvin(&self) -> f64 {
        self.temp + 273.15
    }

    fn core_damping_strategy(&self) -> rspice_core::engine::DampingStrategy {
        match self.damping {
            DampingStrategy::None => rspice_core::engine::DampingStrategy::None,
            DampingStrategy::LineSearch => rspice_core::engine::DampingStrategy::LineSearch,
            DampingStrategy::VoltageLimiting => {
                rspice_core::engine::DampingStrategy::VoltageLimiting
            }
            DampingStrategy::BankRose => rspice_core::engine::DampingStrategy::BankRose,
            DampingStrategy::Combined => rspice_core::engine::DampingStrategy::Combined,
        }
    }

    fn core_integration_method(&self) -> rspice_core::numerics::integration::IntegrationMethod {
        match self.method {
            IntegrationMethod::Trap => {
                rspice_core::numerics::integration::IntegrationMethod::Trapezoidal
            }
            IntegrationMethod::Euler => {
                rspice_core::numerics::integration::IntegrationMethod::BackwardEuler
            }
            IntegrationMethod::Gear2 => {
                rspice_core::numerics::integration::IntegrationMethod::Gear2
            }
            IntegrationMethod::TrapGear => {
                rspice_core::numerics::integration::IntegrationMethod::TrapGear
            }
        }
    }

    fn simulation_config_overrides(&self) -> rspice_core::SimulationConfigOverrides {
        rspice_core::SimulationConfigOverrides {
            temperature_kelvin: Some(self.temp_kelvin()),
            max_iterations: Some(self.itl1),
            min_timestep: Some(self.min_timestep),
            max_timestep: Some(self.max_timestep),
            integration_method: Some(self.core_integration_method()),
            transient_trtol: Some(self.trtol),
            transient_lte_reltol: self.transient_lte_reltol,
            transient_lte_abstol: self.transient_lte_abstol,
            transient_timeint_max_timestep: None,
            transient_use_device_max_timestep: None,
            transient_nonlinear_reltol: None,
            transient_nonlinear_abstol: None,
            transient_nonlinear_deltaxtol: None,
            transient_nonlinear_rhstol: None,
            transient_nonlinear_max_iterations: None,
            transient_enforce_device_convergence: None,
            transient_nonlinear_nox: None,
            transient_lte_reference: None,
            transient_new_bp_stepping: None,
            convergence_preset: None,
            reltol: Some(self.reltol),
            abstol: Some(self.abstol),
            voltage_abstol: Some(self.vntol),
            current_abstol: Some(self.iabstol),
            charge_abstol: Some(self.chgtol),
            residual_reltol: Some(self.residual_reltol),
            gmin_initial: Some(self.gmin),
            device_voltage_limiting: None,
            spice_dialect: None,
            jfet_level2_model: None,
            ramptime: None,
            digital_delay_type: None,
        }
    }

    /// Resolve to core simulation config using layered precedence:
    /// core defaults < netlist `.OPTIONS` < UI options.
    pub fn resolve_simulation_config(
        &self,
        netlist_options: Option<&rspice_core::netlist::SimulationOptions>,
    ) -> rspice_core::engine::SimulationConfig {
        use rspice_core::engine::{BypassConfig, SimulationConfig};

        let overrides = self.simulation_config_overrides();
        let mut sim_config = rspice_core::resolve_simulation_config(
            &SimulationConfig::default(),
            netlist_options,
            &overrides,
        );

        sim_config.bypass_config = BypassConfig {
            enabled: self.bypass_enabled,
            reltol: self.bypass_reltol,
            abstol: self.bypass_abstol,
        };

        sim_config.convergence_config.gmin_stepping = self.gmin_stepping;
        sim_config.convergence_config.source_stepping = self.source_stepping;
        sim_config.convergence_config.pseudo_transient = self.pseudo_transient;
        sim_config.convergence_config.arc_length = self.arc_length;
        sim_config.convergence_config.damping_strategy = self.core_damping_strategy();
        sim_config.convergence_config.gmin_target = sim_config
            .convergence_config
            .gmin_target
            .min(sim_config.convergence_config.gmin_initial);
        sim_config.matrix_solver = self.solver.core_backend_override();
        sim_config.matrix_pivot_tolerance = self.pivrel;
        sim_config.matrix_absolute_pivot_tolerance = self.pivtol;

        sim_config
    }

    /// Validate all options.
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        if self.reltol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("reltol", self.reltol));
        }
        if self.residual_reltol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance(
                "residual_reltol",
                self.residual_reltol,
            ));
        }
        if self.vntol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("vntol", self.vntol));
        }
        if self.abstol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("abstol", self.abstol));
        }
        if self.iabstol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("iabstol", self.iabstol));
        }
        if self.chgtol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("chgtol", self.chgtol));
        }
        if !self.pivrel.is_finite() || self.pivrel <= 0.0 || self.pivrel > 1.0 {
            errors.push(ValidationError::InvalidTolerance("pivrel", self.pivrel));
        }
        if !self.pivtol.is_finite() || self.pivtol <= 0.0 {
            errors.push(ValidationError::InvalidTolerance("pivtol", self.pivtol));
        }

        if self.itl1 == 0 {
            errors.push(ValidationError::InvalidIteration("itl1", self.itl1));
        }
        if self.itl4 == 0 {
            errors.push(ValidationError::InvalidIteration("itl4", self.itl4));
        }

        if self.min_timestep <= 0.0 {
            errors.push(ValidationError::InvalidTimestep(
                "min_timestep",
                self.min_timestep,
            ));
        }
        if self.max_timestep <= 0.0 {
            errors.push(ValidationError::InvalidTimestep(
                "max_timestep",
                self.max_timestep,
            ));
        }
        if self.min_timestep >= self.max_timestep {
            errors.push(ValidationError::TimestepOrder(
                self.min_timestep,
                self.max_timestep,
            ));
        }

        if self.temp <= -273.15 {
            errors.push(ValidationError::InvalidTemperature("temp", self.temp));
        }
        if self.tnom < -273.15 {
            errors.push(ValidationError::InvalidTemperature("tnom", self.tnom));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Export as SPICE .options string.
    pub fn to_spice_options(&self) -> String {
        let mut lines = vec![".OPTIONS".to_string()];
        let default = Self::default();

        if (self.reltol - default.reltol).abs() > 1e-15 {
            lines.push(format!("+ RELTOL={:.2e}", self.reltol));
        }
        if (self.residual_reltol - default.residual_reltol).abs() > 1e-15 {
            lines.push(format!("+ RESIDUAL_RELTOL={:.2e}", self.residual_reltol));
        }
        if (self.abstol - default.abstol).abs() > 1e-20 {
            lines.push(format!("+ ABSTOL={:.2e}", self.abstol));
        }
        if (self.vntol - default.vntol).abs() > 1e-12 {
            lines.push(format!("+ VNTOL={:.2e}", self.vntol));
        }
        if (self.iabstol - default.iabstol).abs() > 1e-20 {
            lines.push(format!("+ IABSTOL={:.2e}", self.iabstol));
        }
        if (self.chgtol - default.chgtol).abs() > 1e-22 {
            lines.push(format!("+ CHGTOL={:.2e}", self.chgtol));
        }
        if self.pivrel.to_bits() != default.pivrel.to_bits() {
            lines.push(format!("+ PIVREL={:.2e}", self.pivrel));
        }
        // The product policy's default is deliberately non-zero while the
        // core fallback is zero. Always state it so the ledger and solve
        // cannot disagree when the user leaves this field untouched.
        lines.push(format!("+ PIVTOL={:.2e}", self.pivtol));
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
        if (self.temp - default.temp).abs() > 0.01 {
            lines.push(format!("+ TEMP={:.2}", self.temp));
        }
        if (self.tnom - default.tnom).abs() > 0.01 {
            lines.push(format!("+ TNOM={:.2}", self.tnom));
        }
        if (self.gmin - default.gmin).abs() > 1e-20 {
            lines.push(format!("+ GMIN={:.2e}", self.gmin));
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
            if (self.bypass_reltol - default.bypass_reltol).abs() > 1e-15 {
                lines.push(format!("+ BYPASSRELTOL={:.2e}", self.bypass_reltol));
            }
            if (self.bypass_abstol - default.bypass_abstol).abs() > 1e-12 {
                lines.push(format!("+ BYPASSABSTOL={:.2e}", self.bypass_abstol));
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
        // the two bounds unstatable.
        // The core fallback is unbounded; Studio's product policy is not.
        lines.push(format!("+ MAXTIMESTEP={:.2e}", self.max_timestep));

        // The parser's package selector stays in force for the rest of the
        // `.OPTIONS` command it appears on, so a scoped key placed among the
        // global ones would re-scope every key after it. The timestep
        // integrator's settings therefore get their own card.
        let mut timeint = Vec::new();
        if let Some(reltol) = self.transient_lte_reltol {
            timeint.push(format!("+ RELTOL={reltol:.2e}"));
        }
        if let Some(abstol) = self.transient_lte_abstol {
            timeint.push(format!("+ ABSTOL={abstol:.2e}"));
        }
        // The product floor is lower than the core fallback and therefore
        // must be stated even for the untouched shipping policy.
        timeint.push(format!("+ MINTIMESTEP={:.2e}", self.min_timestep));
        if !timeint.is_empty() {
            // A card whose only content is the global header states nothing,
            // and would re-scope nothing either; drop it rather than emit it.
            if lines.len() == 1 {
                lines.clear();
            }
            lines.push(".OPTIONS TIMEINT".to_string());
            lines.append(&mut timeint);
        }

        lines.join("\n")
    }
}
