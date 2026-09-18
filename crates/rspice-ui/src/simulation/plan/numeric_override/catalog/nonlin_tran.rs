//! The options that ride on a `NONLIN-TRAN` card of their own.
//!
//! Xyce's transient nonlinear-solver package: the seven values that bound the
//! Newton solve *inside* one accepted time step, as distinct from the global
//! keys that bound the operating point's Newton solve and the `TIMEINT` keys
//! that bound the step itself. A transient that converges at the operating
//! point and then stalls, or one that accepts steps a reader does not believe,
//! is tuned here and nowhere else.
//!
//! All seven satisfy the catalog's first three conditions: the parser scopes
//! them (`netlist/parser/commands.rs:2053-2130`), `resolve_simulation_config`
//! maps each onto its own `SimulationConfig` field
//! (`engine/config_resolver.rs:242-262`), and the engine reads that field at
//! the site each entry cites.
//!
//! One caveat a reader should have from the table rather than from a surprise:
//! the four tolerances and the NOX switch are read through the Xyce transient
//! solver contract, so what they change is the Xyce dialect's weighted update
//! and residual tests. Under the ngspice and best-available dialects the same
//! getters answer with the dialect's own tolerances unless the deck states
//! these, which is exactly what stating them does — the field is read in every
//! dialect; which *test* consumes it is the dialect's choice. The step budget
//! and the device-convergence policy are read the same way.

use super::{
    NumericOverrideOption, OptionPackage, OptionReach, OptionSpec, OverrideSection,
    OverrideValueKind,
};

pub(super) const NONLIN_TRAN: [OptionSpec; 7] = [
    OptionSpec {
        option: NumericOverrideOption::TransientNewtonReltol,
        key: "RELTOL",
        package: OptionPackage::NonlinTran,
        section: OverrideSection::TransientNewton,
        label: "Step update bound · NONLIN-TRAN RELTOL",
        value_kind: OverrideValueKind::PositiveReal,
        value_hint: "positive real",
        config_field: "transient_nonlinear_reltol",
        consumer: "engine/convergence/tolerances.rs:190 · transient_newton_update_weights",
        reach: OptionReach::TimeStepped,
    },
    OptionSpec {
        option: NumericOverrideOption::TransientNewtonAbstol,
        key: "ABSTOL",
        package: OptionPackage::NonlinTran,
        section: OverrideSection::TransientNewton,
        label: "Step update floor · NONLIN-TRAN ABSTOL",
        value_kind: OverrideValueKind::PositiveReal,
        value_hint: "positive real",
        config_field: "transient_nonlinear_abstol",
        consumer: "engine/convergence/tolerances.rs:191 · transient_newton_update_weights",
        reach: OptionReach::TimeStepped,
    },
    OptionSpec {
        option: NumericOverrideOption::TransientNewtonUpdateBound,
        // The weighted max norm of one Newton correction has to fall below
        // this before the step is accepted. Xyce's default is 0.33, which is
        // loose by SPICE standards because the weights already carry RELTOL.
        key: "DELTAXTOL",
        package: OptionPackage::NonlinTran,
        section: OverrideSection::TransientNewton,
        label: "Accepted correction norm · NONLIN-TRAN DELTAXTOL",
        value_kind: OverrideValueKind::PositiveReal,
        value_hint: "positive real",
        config_field: "transient_nonlinear_deltaxtol",
        consumer: "engine/convergence/tolerances.rs:224 · transient_newton_update_convergence_met",
        reach: OptionReach::TimeStepped,
    },
    OptionSpec {
        option: NumericOverrideOption::TransientNewtonResidualBound,
        key: "RHSTOL",
        package: OptionPackage::NonlinTran,
        section: OverrideSection::TransientNewton,
        label: "Accepted residual norm · NONLIN-TRAN RHSTOL",
        value_kind: OverrideValueKind::PositiveReal,
        value_hint: "positive real",
        config_field: "transient_nonlinear_rhstol",
        consumer: "engine/transient/residual.rs:1357 · transient_nonlinear_rhstol",
        reach: OptionReach::TimeStepped,
    },
    OptionSpec {
        option: NumericOverrideOption::TransientNewtonBudget,
        // Xyce's NOX transient parameter set owns an iteration budget
        // independent of ITL4, and the engine reads this one under the Xyce
        // dialect rather than raising it to ngspice's NIiter floor.
        key: "MAXSTEP",
        package: OptionPackage::NonlinTran,
        section: OverrideSection::TransientNewton,
        label: "Newton budget per step · NONLIN-TRAN MAXSTEP",
        value_kind: OverrideValueKind::IterationCount,
        value_hint: "whole iteration count",
        config_field: "transient_nonlinear_max_iterations",
        consumer: "engine/transient/step_control.rs:173 · transient_newton_iteration_budget",
        reach: OptionReach::TimeStepped,
    },
    OptionSpec {
        option: NumericOverrideOption::TransientDeviceConvergence,
        key: "ENFORCEDEVICECONV",
        package: OptionPackage::NonlinTran,
        section: OverrideSection::TransientNewton,
        label: "Require device convergence",
        value_kind: OverrideValueKind::Flag,
        value_hint: "on or off",
        config_field: "transient_enforce_device_convergence",
        consumer: "engine/transient.rs:4621 · transient_enforce_device_convergence",
        reach: OptionReach::TimeStepped,
    },
    OptionSpec {
        option: NumericOverrideOption::TransientNoxSolver,
        // Turning NOX on selects Xyce's undamped transient Newton path, so the
        // switch reads as what it selects rather than as the acronym.
        key: "NOX",
        package: OptionPackage::NonlinTran,
        section: OverrideSection::TransientNewton,
        label: "Undamped transient Newton · NONLIN-TRAN NOX",
        value_kind: OverrideValueKind::Flag,
        value_hint: "on or off",
        config_field: "transient_nonlinear_nox",
        consumer: "engine/transient.rs:1475 · uses_xyce_damped_transient_solver",
        reach: OptionReach::TimeStepped,
    },
];
