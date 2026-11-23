//! The names an `.options` card can set: one table, which the parser reads
//! to know a name and an editor reads to offer it.
//!
//! A name is read outside any package, or after a package word such as
//! Xyce's `TIMEINT` or `DEVICE` and until the next one. Each row says where
//! its spellings are read and which of the parser's arms reads them; a
//! name no row admits where it is written is an unknown option.

/// The spellings of the card itself: `.options`, the usual one, first.
pub const OPTIONS_CARD_SPELLINGS: &[&str] = &[".options", ".option", ".opt", ".opts"];

/// Whether `head`, a card's first word, is an `.options` card's.
pub fn is_options_card(head: &str) -> bool {
    OPTIONS_CARD_SPELLINGS
        .iter()
        .any(|spelling| spelling.eq_ignore_ascii_case(head))
}

/// A package an `.options` card can open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptionPackage {
    /// The package's name, upper case, as the parser compares it.
    pub name: &'static str,
    /// Whether a name the package does not list is unknown in it, rather
    /// than read as it would be outside any package.
    pub closed: bool,
    /// Whether the word opens the package on its own; otherwise only when
    /// one of the package's own names follows it (`LINSOL TR_PARTITION`).
    pub opens_alone: bool,
}

const fn package(name: &'static str, closed: bool) -> OptionPackage {
    OptionPackage {
        name,
        closed,
        opens_alone: true,
    }
}

/// Every package an `.options` card can open.
pub static OPTION_PACKAGES: &[OptionPackage] = &[
    package("TOPOLOGY", false),
    package("FFT", true),
    package("MEASURE", true),
    package("DEVICE", false),
    package("XSPICE", false),
    package("TIMEINT", true),
    package("NONLIN", false),
    package("NONLIN-TRAN", true),
    package("NONLIN-TRANSIENT", true),
    package("NONLIN-HB", true),
    package("NONLIN-CONTINUATION", false),
    package("LOCA", false),
    package("OUTPUT", false),
    package("PARSER", false),
    package("RESTART", true),
    package("HBINT", true),
    package("LINSOL-HB", true),
    package("SENSITIVITY", false),
    OptionPackage {
        name: "LINSOL",
        closed: false,
        opens_alone: false,
    },
];

impl OptionPackage {
    /// Whether `key`, upper case, is one of the package's own names: a row
    /// that names the package reads it.
    pub fn lists(&self, key: &str) -> bool {
        OPTION_NAMES.iter().any(|row| {
            matches!(row.scope, OptionScope::In(packages) | OptionScope::GlobalAnd(packages)
                if packages.contains(&self.name))
                && row.spells(key)
        })
    }
}

/// The package named `word`, upper case.
pub fn option_package(word: &str) -> Option<&'static OptionPackage> {
    OPTION_PACKAGES.iter().find(|package| package.name == word)
}

/// Whether a name of no package is read in `package` as it is outside one:
/// outside every package it is, and in a package that is not closed.
pub fn package_reads_open_names(package: Option<&str>) -> bool {
    package.is_none_or(|name| option_package(name).is_none_or(|package| !package.closed))
}

/// Where a row's spellings are read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionScope {
    /// Outside every package only.
    Global,
    /// In these packages only.
    In(&'static [&'static str]),
    /// Outside every package, and in these packages.
    GlobalAnd(&'static [&'static str]),
    /// Outside every package, and in each package that is not closed.
    Open,
}

impl OptionScope {
    /// Whether a name of this scope is read in `package`.
    pub fn admits(self, package: Option<&str>) -> bool {
        match self {
            Self::Global => package.is_none(),
            Self::In(packages) => package.is_some_and(|name| packages.contains(&name)),
            Self::GlobalAnd(packages) => package.is_none_or(|name| packages.contains(&name)),
            Self::Open => package_reads_open_names(package),
        }
    }
}

/// One row of the table: the spellings of a name, upper case and the usual
/// one first, where they are read, and the arm of the parser that reads
/// them. Rows that differ only in where they are read share an arm.
#[derive(Debug, Clone, Copy)]
pub struct OptionName {
    pub(in crate::netlist::parser) id: OptionId,
    pub scope: OptionScope,
    pub spellings: &'static [&'static str],
    /// Whether a spelling may be followed by more, as `NUMFREQ2` gives the
    /// second tone's count.
    pub numbered: bool,
}

impl OptionName {
    /// Whether `key`, upper case, spells this row's name.
    pub fn spells(&self, key: &str) -> bool {
        self.spellings.iter().any(|spelling| {
            if self.numbered {
                key.starts_with(spelling)
            } else {
                key == *spelling
            }
        })
    }
}

/// The row that reads `key`, upper case, written in `package`, if any does.
pub fn option_name(package: Option<&str>, key: &str) -> Option<&'static OptionName> {
    OPTION_NAMES
        .iter()
        .find(|row| row.scope.admits(package) && row.spells(key))
}

/// The arm of the options parser that reads a name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::netlist::parser) enum OptionId {
    RspiceStatistics,
    RspiceExpRepeat,
    RspiceSource,
    RspiceDialect,
    Seed,
    FftMode,
    FftAccurate,
    FftOut,
    MeasureFail,
    MeasureDefaultValue,
    MeasureUseContFiles,
    MeasureUseLttm,
    NonlinContinuation,
    LocaConparam,
    HbNumFreq,
    HbSaveIcData,
    HbTahb,
    LinsolHbPrecType,
    TopologySupernode,
    DeviceZeroResistanceTol,
    DeviceMinRes,
    DeviceMinCap,
    Pnjmaxi,
    DeviceB3soiGminScaling,
    DeviceVoltLim,
    DeviceDebugLevel,
    DeviceSeparateLoad,
    TimeintDebugLevel,
    DeviceTryToCompact,
    GlobalTopologySupernode,
    GlobalZeroResistanceTol,
    GlobalMinRes,
    GlobalMinCap,
    GlobalB3soiGminScaling,
    GlobalTryToCompact,
    LinsolTrPartition,
    ConnectRules,
    HdlSeed,
    HdlPlusargs,
    HdlRundir,
    HdlDelaySelect,
    HdlPathDelays,
    HdlTimingChecks,
    HdlLibmap,
    ConnectRulesSource,
    WrealResolution,
    AutoBridge,
    NonlinTranReltol,
    NonlinTranAbstol,
    NonlinTranDeltaxtol,
    NonlinTranRhstol,
    NonlinTranMaxstep,
    NonlinHbMaxstep,
    NonlinTranEnforceDeviceConv,
    NonlinTranNox,
    TimeintReltol,
    TimeintAbstol,
    TimeintDelmax,
    TimeintMinTimestep,
    TimeintErroption,
    TimeintMinTimestepsBp,
    TimeintNlmin,
    TimeintNlmax,
    TimeintTimestepsReversal,
    TimeintNlNearConv,
    TimeintNlSmallUpdate,
    SensitivityColumn,
    TimeintMinord,
    TimeintMaxord,
    TimeintBreakpoints,
    MaxTimestep,
    TimeintUseDeviceMax,
    TimeintNewLte,
    TimeintNewBpStepping,
    TimeintMethod,
    RestartPack,
    RestartPrintTimeintOptions,
    RestartJob,
    RestartStartTime,
    RestartFile,
    RestartInitialInterval,
    Bypass,
    BypassReltol,
    BypassAbstol,
    Reltol,
    Abstol,
    Vntol,
    Iabstol,
    ResidualReltol,
    Gmin,
    Rshunt,
    Gshunt,
    Cshunt,
    Xmu,
    Trtol,
    Ramptime,
    MaxEvtIter,
    DigitalDelayType,
    EventSave,
    Chgtol,
    EventFluxTol,
    Pivtol,
    Pivrel,
    Temp,
    Tnom,
    Scale,
    Itl1,
    Itl2,
    Itl4,
    Itl6,
    Method,
    GminStepping,
    SourceStepping,
    PseudoTransient,
    ArcLength,
    Damping,
    Solver,
    OutputInitialInterval,
    OutputTimePoints,
    OutputSnapshots,
    OutputPrintHeader,
    OutputPrintFooter,
    OutputPhaseRadians,
    Interp,
    Noacct,
    AllowSimplifiedMos,
}

const fn row(id: OptionId, scope: OptionScope, spellings: &'static [&'static str]) -> OptionName {
    OptionName {
        id,
        scope,
        spellings,
        numbered: false,
    }
}

const NONLIN_TRAN: &[&str] = &["NONLIN-TRAN", "NONLIN-TRANSIENT"];

use OptionId as Id;
use OptionScope::{Global, GlobalAnd, In, Open};

/// Every name an `.options` card can set.
pub static OPTION_NAMES: &[OptionName] = &[
    row(Id::RspiceStatistics, Global, &["RSPICE_STATISTICS"]),
    row(Id::RspiceExpRepeat, Global, &["RSPICE_EXP_REPEAT"]),
    row(Id::RspiceSource, Global, &["RSPICE_SOURCE"]),
    row(Id::RspiceDialect, Global, &["RSPICE_DIALECT"]),
    row(Id::Seed, Open, &["SEED", "RNDSEED"]),
    row(Id::FftMode, In(&["FFT"]), &["FFT_MODE"]),
    row(Id::FftAccurate, In(&["FFT"]), &["FFT_ACCURATE"]),
    row(Id::FftOut, In(&["FFT"]), &["FFTOUT"]),
    row(Id::MeasureFail, In(&["MEASURE"]), &["MEASFAIL"]),
    row(Id::MeasureDefaultValue, In(&["MEASURE"]), &["DEFAULT_VAL"]),
    row(
        Id::MeasureUseContFiles,
        In(&["MEASURE"]),
        &["USE_CONT_FILES"],
    ),
    row(Id::MeasureUseLttm, In(&["MEASURE"]), &["USE_LTTM"]),
    row(
        Id::NonlinContinuation,
        In(&["NONLIN", "NONLIN-CONTINUATION"]),
        &["CONTINUATION"],
    ),
    row(Id::LocaConparam, In(&["LOCA"]), &["CONPARAM"]),
    // Xyce lists one count per tone, `numfreq=11,7`, or spells them
    // `numfreq=11 numfreq2=7`.
    OptionName {
        id: Id::HbNumFreq,
        scope: In(&["HBINT"]),
        spellings: &["NUMFREQ"],
        numbered: true,
    },
    row(Id::HbSaveIcData, In(&["HBINT"]), &["SAVEICDATA"]),
    row(Id::HbTahb, In(&["HBINT"]), &["TAHB"]),
    row(Id::LinsolHbPrecType, In(&["LINSOL-HB"]), &["PREC_TYPE"]),
    row(Id::TopologySupernode, In(&["TOPOLOGY"]), &["SUPERNODE"]),
    row(
        Id::DeviceZeroResistanceTol,
        In(&["DEVICE"]),
        &["ZERORESISTANCETOL", "ZERO_RESISTANCE_TOL"],
    ),
    row(Id::DeviceMinRes, In(&["DEVICE"]), &["MINRES", "MIN_RES"]),
    row(Id::DeviceMinCap, In(&["DEVICE"]), &["MINCAP", "MIN_CAP"]),
    row(Id::Pnjmaxi, GlobalAnd(&["DEVICE"]), &["PNJMAXI"]),
    row(
        Id::DeviceB3soiGminScaling,
        In(&["DEVICE"]),
        &["B3SOIGMINSCALING", "B3SOI_GMIN_SCALING"],
    ),
    row(Id::DeviceVoltLim, In(&["DEVICE"]), &["VOLTLIM", "VOLT_LIM"]),
    row(
        Id::DeviceDebugLevel,
        In(&["DEVICE"]),
        &["DEBUGLEVEL", "DEBUG_LEVEL"],
    ),
    row(
        Id::DeviceSeparateLoad,
        In(&["DEVICE"]),
        &["SEPARATELOAD", "SEPARATE_LOAD"],
    ),
    row(
        Id::TimeintDebugLevel,
        In(&["TIMEINT"]),
        &["DEBUGLEVEL", "DEBUG_LEVEL"],
    ),
    row(
        Id::DeviceTryToCompact,
        In(&["DEVICE"]),
        &["TRYTOCOMPACT", "TRY_TO_COMPACT"],
    ),
    row(
        Id::GlobalTopologySupernode,
        Global,
        &["TOPOLOGY_SUPERNODE", "TOPOLOGYSUPERNODE"],
    ),
    row(
        Id::GlobalZeroResistanceTol,
        Global,
        &[
            "ZERORESISTANCETOL",
            "ZERO_RESISTANCE_TOL",
            "DEVICE_ZERORESISTANCETOL",
            "DEVICEZERORESISTANCETOL",
        ],
    ),
    row(
        Id::GlobalMinRes,
        Global,
        &["MINRES", "MIN_RES", "DEVICE_MINRES", "DEVICEMINRES"],
    ),
    row(
        Id::GlobalMinCap,
        Global,
        &["MINCAP", "MIN_CAP", "DEVICE_MINCAP", "DEVICE_MIN_CAP"],
    ),
    row(
        Id::GlobalB3soiGminScaling,
        Global,
        &[
            "B3SOIGMINSCALING",
            "B3SOI_GMIN_SCALING",
            "DEVICE_B3SOIGMINSCALING",
        ],
    ),
    row(
        Id::GlobalTryToCompact,
        Global,
        &["TRYTOCOMPACT", "TRY_TO_COMPACT", "DEVICE_TRYTOCOMPACT"],
    ),
    row(
        Id::LinsolTrPartition,
        In(&["LINSOL"]),
        &["TR_PARTITION", "TRPARTITION"],
    ),
    row(Id::ConnectRules, Global, &["CONNECTRULES"]),
    row(Id::HdlSeed, Global, &["HDLSEED", "HDL_SEED"]),
    row(Id::HdlPlusargs, Global, &["HDL_PLUSARGS"]),
    row(Id::HdlRundir, Global, &["HDL_RUNDIR"]),
    row(Id::HdlDelaySelect, Global, &["HDL_DELAY_SELECT"]),
    row(Id::HdlPathDelays, Global, &["HDL_PATH_DELAYS"]),
    row(Id::HdlTimingChecks, Global, &["HDL_TIMING_CHECKS"]),
    row(Id::HdlLibmap, Global, &["HDL_LIBMAP"]),
    row(Id::ConnectRulesSource, Global, &["CONNECTRULES_SOURCE"]),
    row(Id::WrealResolution, Global, &["WREAL_RESOLUTION"]),
    row(
        Id::AutoBridge,
        In(&["XSPICE"]),
        &["AUTO_BRIDGE", "AUTOBRIDGE"],
    ),
    row(
        Id::AutoBridge,
        Global,
        &["AUTO_BRIDGE", "AUTOBRIDGE", "XSPICE_AUTO_BRIDGE"],
    ),
    row(Id::NonlinTranReltol, In(NONLIN_TRAN), &["RELTOL"]),
    row(Id::NonlinTranAbstol, In(NONLIN_TRAN), &["ABSTOL"]),
    row(Id::NonlinTranDeltaxtol, In(NONLIN_TRAN), &["DELTAXTOL"]),
    row(Id::NonlinTranRhstol, In(NONLIN_TRAN), &["RHSTOL"]),
    row(Id::NonlinTranMaxstep, In(NONLIN_TRAN), &["MAXSTEP"]),
    row(Id::NonlinHbMaxstep, In(&["NONLIN-HB"]), &["MAXSTEP"]),
    row(
        Id::NonlinTranEnforceDeviceConv,
        In(NONLIN_TRAN),
        &["ENFORCEDEVICECONV", "ENFORCE_DEVICE_CONV"],
    ),
    row(Id::NonlinTranNox, In(NONLIN_TRAN), &["NOX"]),
    row(Id::TimeintReltol, In(&["TIMEINT"]), &["RELTOL"]),
    row(Id::TimeintAbstol, In(&["TIMEINT"]), &["ABSTOL"]),
    row(Id::TimeintDelmax, In(&["TIMEINT"]), &["DELMAX"]),
    row(
        Id::TimeintMinTimestep,
        In(&["TIMEINT"]),
        &["MINTIMESTEP", "MIN_TIMESTEP"],
    ),
    row(Id::TimeintErroption, In(&["TIMEINT"]), &["ERROPTION"]),
    row(
        Id::TimeintMinTimestepsBp,
        In(&["TIMEINT"]),
        &["MINTIMESTEPSBP"],
    ),
    row(Id::TimeintNlmin, In(&["TIMEINT"]), &["NLMIN"]),
    row(Id::TimeintNlmax, In(&["TIMEINT"]), &["NLMAX"]),
    row(
        Id::TimeintTimestepsReversal,
        In(&["TIMEINT"]),
        &["TIMESTEPSREVERSAL"],
    ),
    row(Id::TimeintNlNearConv, In(&["TIMEINT"]), &["NLNEARCONV"]),
    row(
        Id::TimeintNlSmallUpdate,
        In(&["TIMEINT"]),
        &["NLSMALLUPDATE"],
    ),
    // Which derivative columns a `.SENS` table prints; one arm reads all four.
    row(Id::SensitivityColumn, In(&["SENSITIVITY"]), &["DIRECT"]),
    row(Id::SensitivityColumn, In(&["SENSITIVITY"]), &["ADJOINT"]),
    row(
        Id::SensitivityColumn,
        In(&["SENSITIVITY"]),
        &["OUTPUTSCALED"],
    ),
    row(
        Id::SensitivityColumn,
        In(&["SENSITIVITY"]),
        &["OUTPUTUNSCALED"],
    ),
    row(Id::TimeintMinord, In(&["TIMEINT"]), &["MINORD"]),
    row(Id::TimeintMaxord, In(&["TIMEINT"]), &["MAXORD"]),
    row(Id::TimeintBreakpoints, In(&["TIMEINT"]), &["BREAKPOINTS"]),
    // The run's step ceiling is deliberately outside `TIMEINT`: that
    // package's own ceiling is `DELMAX`.
    row(Id::MaxTimestep, Global, &["MAXTIMESTEP", "MAX_TIMESTEP"]),
    row(
        Id::TimeintUseDeviceMax,
        In(&["TIMEINT"]),
        &["USEDEVICEMAX", "USE_DEVICE_MAX"],
    ),
    row(Id::TimeintNewLte, In(&["TIMEINT"]), &["NEWLTE"]),
    row(
        Id::TimeintNewBpStepping,
        In(&["TIMEINT"]),
        &["NEWBPSTEPPING"],
    ),
    row(Id::TimeintMethod, In(&["TIMEINT"]), &["METHOD"]),
    row(Id::RestartPack, In(&["RESTART"]), &["PACK"]),
    row(
        Id::RestartPrintTimeintOptions,
        In(&["RESTART"]),
        &["PRINT_TIMEINT_OPTIONS", "PRINTTIMEINTOPTIONS"],
    ),
    row(Id::RestartJob, In(&["RESTART"]), &["JOB"]),
    row(
        Id::RestartStartTime,
        In(&["RESTART"]),
        &["START_TIME", "STARTTIME"],
    ),
    row(Id::RestartFile, In(&["RESTART"]), &["FILE"]),
    row(
        Id::RestartInitialInterval,
        In(&["RESTART"]),
        &["INITIAL_INTERVAL", "INITIALINTERVAL"],
    ),
    // Spelled unscoped, as ngspice spells `bypass`.
    row(Id::Bypass, Open, &["BYPASS"]),
    row(Id::BypassReltol, Open, &["BYPASSRELTOL", "BYPASS_RELTOL"]),
    row(Id::BypassAbstol, Open, &["BYPASSABSTOL", "BYPASS_ABSTOL"]),
    row(Id::Reltol, Open, &["RELTOL"]),
    row(Id::Abstol, Open, &["ABSTOL"]),
    row(Id::Vntol, Open, &["VNTOL"]),
    row(Id::Iabstol, Open, &["IABSTOL"]),
    row(Id::ResidualReltol, Open, &["RESIDUAL_RELTOL", "RESRELTOL"]),
    row(Id::Gmin, Open, &["GMIN"]),
    row(Id::Rshunt, Open, &["RSHUNT"]),
    row(Id::Gshunt, Open, &["GSHUNT"]),
    row(Id::Cshunt, Open, &["CSHUNT"]),
    row(Id::Xmu, Global, &["XMU"]),
    row(Id::Trtol, Open, &["TRTOL"]),
    row(Id::Ramptime, Open, &["RAMPTIME"]),
    row(Id::MaxEvtIter, Open, &["MAXEVTITER"]),
    row(
        Id::DigitalDelayType,
        In(&["XSPICE"]),
        &["DIGITAL_DELAY_TYPE", "DIGITALDELAYTYPE", "DIGITAL_DELAY"],
    ),
    row(
        Id::DigitalDelayType,
        Global,
        &[
            "DIGITAL_DELAY_TYPE",
            "DIGITALDELAYTYPE",
            "XSPICE_DIGITAL_DELAY_TYPE",
        ],
    ),
    row(
        Id::EventSave,
        In(&["XSPICE"]),
        &["ESAVE", "EVENT_SAVE", "EVENTSAVE"],
    ),
    row(
        Id::EventSave,
        Global,
        &["XSPICE_ESAVE", "XSPICE_EVENT_SAVE", "XSPICE_EVENTSAVE"],
    ),
    row(Id::Chgtol, Open, &["CHGTOL"]),
    row(Id::EventFluxTol, Global, &["EVENTFLUXTOL"]),
    row(Id::Pivtol, Open, &["PIVTOL"]),
    row(Id::Pivrel, Open, &["PIVREL"]),
    row(Id::Temp, GlobalAnd(&["DEVICE"]), &["TEMP"]),
    row(Id::Tnom, GlobalAnd(&["DEVICE"]), &["TNOM"]),
    // Element geometry scale factor (`.options scale`, Xyce's
    // `.options parser scale`).
    row(Id::Scale, Open, &["SCALE"]),
    row(Id::Itl1, Open, &["ITL1"]),
    row(Id::Itl2, Open, &["ITL2"]),
    row(Id::Itl4, Open, &["ITL4"]),
    row(Id::Itl6, Open, &["ITL6"]),
    row(Id::Method, Open, &["METHOD"]),
    row(Id::GminStepping, Open, &["GMINSTEPPING", "GMIN_STEPPING"]),
    row(
        Id::SourceStepping,
        Open,
        &["SOURCESTEPPING", "SOURCE_STEPPING", "SRCSTEPPING"],
    ),
    row(
        Id::PseudoTransient,
        Open,
        &["PSEUDOTRANSIENT", "PSEUDO_TRANSIENT"],
    ),
    row(Id::ArcLength, Open, &["ARCLENGTH", "ARC_LENGTH"]),
    row(Id::Damping, Open, &["DAMPING"]),
    row(Id::Solver, Open, &["SOLVER"]),
    row(
        Id::OutputInitialInterval,
        In(&["OUTPUT"]),
        &["INITIAL_INTERVAL", "INITIALINTERVAL"],
    ),
    row(Id::OutputTimePoints, In(&["OUTPUT"]), &["OUTPUTTIMEPOINTS"]),
    row(Id::OutputSnapshots, In(&["OUTPUT"]), &["SNAPSHOTS"]),
    row(Id::OutputPrintHeader, In(&["OUTPUT"]), &["PRINTHEADER"]),
    row(Id::OutputPrintFooter, In(&["OUTPUT"]), &["PRINTFOOTER"]),
    row(
        Id::OutputPhaseRadians,
        In(&["OUTPUT"]),
        &["PHASE_OUTPUT_RADIANS"],
    ),
    row(Id::Interp, Open, &["INTERP"]),
    row(Id::Noacct, Open, &["NOACCT"]),
    row(
        Id::AllowSimplifiedMos,
        Open,
        &["ALLOW_SIMPLIFIED_MOS", "ALLOWSIMPLIFIEDMOS"],
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Every package a row names, and every package word written anywhere.
    fn written_packages() -> Vec<Option<&'static str>> {
        std::iter::once(None)
            .chain(OPTION_PACKAGES.iter().map(|package| Some(package.name)))
            .collect()
    }

    #[test]
    fn no_two_rows_read_one_spelling_in_one_package() {
        for package in written_packages() {
            for (at, row) in OPTION_NAMES.iter().enumerate() {
                if !row.scope.admits(package) {
                    continue;
                }
                for spelling in row.spellings {
                    let reader = option_name(package, spelling).expect("a row reads its own name");
                    assert!(
                        std::ptr::eq(reader, row),
                        "{spelling} in {package:?} is row {at}'s, but an earlier row reads it"
                    );
                }
            }
        }
    }

    #[test]
    fn spellings_are_upper_case_and_packages_are_known() {
        for row in OPTION_NAMES {
            assert!(!row.spellings.is_empty());
            for spelling in row.spellings {
                assert_eq!(*spelling, spelling.to_ascii_uppercase());
            }
            if let OptionScope::In(packages) | OptionScope::GlobalAnd(packages) = row.scope {
                for name in packages {
                    assert!(option_package(name).is_some(), "{name} is no package");
                }
            }
        }
    }
}
