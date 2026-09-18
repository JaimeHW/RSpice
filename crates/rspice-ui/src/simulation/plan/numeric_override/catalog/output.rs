//! The options that ride on an `OUTPUT` card of their own.
//!
//! The output schedule: which of the solved time points a time-domain run
//! reports, and how much of each one it keeps. Spectre calls the first of them
//! `strobeperiod`, and the reason it is a control rather than a preference is
//! that a run reporting every accepted step produces a result whose row count
//! is a property of the solver's step choices instead of a property the reader
//! asked for.
//!
//! These are the catalog's first entries admitted under the fourth route. None
//! of the three reaches `SimulationConfig`: `resolve_simulation_config` has no
//! arm for any of them, and the engine takes each one straight off
//! `netlist.options` at the site the entry cites. The harness proves them by
//! parsing the emitted deck and asserting the parsed `SimulationOptions`
//! moved, which is the strongest statement available about a value the
//! resolver never touches.
//!
//! `INITIAL_INTERVAL` and `OUTPUTTIMEPOINTS` are mutually exclusive in the
//! engine's own parser — a deck carrying both is a syntax error — so the
//! record refuses the second of them rather than emitting a card that cannot
//! be read back.

use super::{
    NumericOverrideOption, OptionPackage, OptionReach, OptionSpec, OverrideSection,
    OverrideValueKind,
};

pub(super) const OUTPUT: [OptionSpec; 3] = [
    OptionSpec {
        option: NumericOverrideOption::StrobeInterval,
        key: "INITIAL_INTERVAL",
        package: OptionPackage::Output,
        section: OverrideSection::Output,
        label: "Strobe interval",
        value_kind: OverrideValueKind::PositiveReal,
        value_hint: "time, SI suffixes accepted",
        // Nothing: the resolver has no arm for the schedule, and the engine
        // reads the parsed record. Named rather than left empty so the catalog
        // test's "names its field" check stays meaningful for the routes that
        // do have one.
        config_field: "netlist.options.output_interval_schedule",
        consumer: "engine/transient.rs:11728 · TransientCompressionOutputSchedule::from_netlist",
        reach: OptionReach::TimeStepped,
    },
    OptionSpec {
        option: NumericOverrideOption::OutputTimePoints,
        key: "OUTPUTTIMEPOINTS",
        package: OptionPackage::Output,
        section: OverrideSection::Output,
        label: "Report at exactly these times",
        value_kind: OverrideValueKind::TimeList,
        value_hint: "increasing times, space or comma separated",
        config_field: "netlist.options.output_time_points",
        consumer: "engine/transient/breakpoints.rs:1072 · add_user_transient_breakpoints",
        reach: OptionReach::TimeStepped,
    },
    OptionSpec {
        option: NumericOverrideOption::RetainEverySignal,
        // Xyce writes snapshot files for this key. RSpice's engine reads it as
        // what a snapshot needs: every analog operand retained rather than the
        // requested outputs projected out of the run.
        key: "SNAPSHOTS",
        package: OptionPackage::Output,
        section: OverrideSection::Output,
        label: "Retain every signal · OUTPUT SNAPSHOTS",
        value_kind: OverrideValueKind::Flag,
        value_hint: "on or off",
        config_field: "netlist.options.output_snapshots",
        consumer: "engine/transient.rs:1292 · retain_all",
        reach: OptionReach::TimeStepped,
    },
];
