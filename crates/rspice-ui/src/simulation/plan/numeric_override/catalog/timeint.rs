//! The options that ride on a `TIMEINT` card of their own.
//!
//! The parser's package selector stays in force for the rest of the command
//! it appears on, so one of these placed among the global keys would re-scope
//! every key after it. That is why the package is a table rather than a field
//! read off each entry, and why the emitter writes it as a separate card.
//!
//! All four bound the accepted truncation error of one time step, so they
//! report in the integration section beside the global keys that choose the
//! method: the split here is by owner, not by where the reader meets them.

use super::{NumericOverrideOption, OptionPackage, OptionSpec, OverrideSection, OverrideValueKind};

pub(super) const TIMEINT: [OptionSpec; 4] = [
    OptionSpec {
        option: NumericOverrideOption::LteReltol,
        key: "RELTOL",
        package: OptionPackage::Timeint,
        section: OverrideSection::Integration,
        label: "Truncation relative bound · TIMEINT RELTOL",
        value_kind: OverrideValueKind::PositiveReal,
        value_hint: "positive real",
        config_field: "transient_lte_reltol",
        consumer: "engine/transient.rs:2412 · accepted local truncation error",
        time_stepped_only: true,
    },
    OptionSpec {
        option: NumericOverrideOption::LteAbstol,
        key: "ABSTOL",
        package: OptionPackage::Timeint,
        section: OverrideSection::Integration,
        label: "Truncation absolute bound · TIMEINT ABSTOL",
        value_kind: OverrideValueKind::PositiveReal,
        value_hint: "positive real",
        config_field: "transient_lte_abstol",
        consumer: "engine/transient.rs:2413 · accepted local truncation error",
        time_stepped_only: true,
    },
    OptionSpec {
        option: NumericOverrideOption::MinTimestep,
        key: "MINTIMESTEP",
        package: OptionPackage::Timeint,
        section: OverrideSection::Integration,
        label: "Step floor",
        value_kind: OverrideValueKind::PositiveReal,
        value_hint: "time, SI suffixes accepted",
        config_field: "min_timestep",
        consumer: "engine/transient.rs:2391 · preferred_min_dt",
        time_stepped_only: true,
    },
    OptionSpec {
        option: NumericOverrideOption::MaximumTimestep,
        key: "DELMAX",
        package: OptionPackage::Timeint,
        section: OverrideSection::Integration,
        label: "Step ceiling",
        value_kind: OverrideValueKind::PositiveReal,
        value_hint: "time, SI suffixes accepted",
        config_field: "transient_timeint_max_timestep",
        consumer: "engine/transient.rs:1545 · hinted_max_step clamp",
        time_stepped_only: true,
    },
];
