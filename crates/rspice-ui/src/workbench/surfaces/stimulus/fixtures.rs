//! The library the instrument is reviewed and gated against.
//!
//! The mockup's fourteen definitions, in the application's own spelling: the
//! same component type, the same primary value and the same parameter string a
//! placed source of that shape would carry. They are a fixture and nothing
//! else — no surface reads them and no project ships them — but they are the
//! only set that covers every family the stage has a distinct layout for, so
//! the rasters and the fit gates seed from here rather than from whatever
//! definition a test happened to write.

use crate::state::ComponentType;
use crate::state::stimulus_library::definition::{RetainedPwlFile, StimulusDefinition};
use crate::state::stimulus_library::library::StimulusLibrary;

/// One fixture row: name, type, primary value, parameter string.
type Row = (&'static str, ComponentType, &'static str, &'static str);

/// The mockup's `STW_LIBRARY`, in app spelling.
const ROWS: [Row; 14] = [
    (
        "sensor_diff_1k",
        ComponentType::VoltageSourceSin,
        "0",
        "va=2m freq=1k ac=1m acphase=0",
    ),
    (
        "vdd_operate",
        ComponentType::VoltageSource,
        "{VSUP}",
        "ac=1",
    ),
    ("vref_mid", ComponentType::VoltageSource, "{VCM_REF}", ""),
    (
        "vdd_ramp_1ms",
        ComponentType::VoltageSourcePwl,
        "0 0 1m 5 6m 5",
        "",
    ),
    (
        "vdd_brownout_2v8",
        ComponentType::VoltageSourcePwl,
        "0 5 2m 5 2.05m 2.8 4m 2.8 4.05m 5 8m 5",
        "",
    ),
    (
        "bridge_cal_step",
        ComponentType::VoltageSourcePulse,
        "-10m",
        "v2=10m td=100u tr=1u tf=1u pw=500u per=1m",
    ),
    (
        "load_step_50u",
        ComponentType::CurrentSourcePulse,
        "0",
        "i2=50u td=1m tr=100n tf=100n pw=2m per=5m",
    ),
    (
        "overload_recovery_exp",
        ComponentType::VoltageSourceExp,
        "0",
        "v2=250m td1=100u tau1=10u td2=600u tau2=50u",
    ),
    (
        "emi_am_150k",
        ComponentType::VoltageSourceAm,
        "0",
        "vmo=100m vma=80m fm=1k fc=150k",
    ),
    (
        "fm_interferer_100k",
        ComponentType::VoltageSourceSffm,
        "0",
        "va=50m fc=100k mdi=4 fm=10k",
    ),
    (
        "supply_trnoise",
        ComponentType::VoltageSourceNoise,
        "20u",
        "nt=1u",
    ),
    (
        "en_boot_pat",
        ComponentType::VoltageSourcePat,
        "5",
        "vlo=0 td=0 tr=100n tf=100n tsample=200u data=B00111",
    ),
    (
        "bridge_meas_step",
        ComponentType::VoltageSourcePwlFile,
        "meas/bridge_step_c07.csv",
        "",
    ),
    (
        "bridge_drift_walk",
        ComponentType::VoltageSourceRandom,
        "500u",
        "type=gaussian param1=2m param2=0",
    ),
];

/// The retained table `bridge_meas_step` carries, so the file editor has bytes
/// to describe without reaching a lab share that is not there.
const MEASURED_TABLE: &str = "0 0\n200u 0\n210u 8.2m\n230u 11.6m\n260u 12.4m\n300u 10.4m\n\
                              360u 9.2m\n430u 9.7m\n520u 10.15m\n650u 9.94m\n800u 10.02m\n\
                              1m 9.99m\n1.4m 10m\n2m 10m\n";

/// One definition, by the name the mockup gives it.
pub(super) fn definition(name: &str) -> StimulusDefinition {
    let (_, kind, value, params) = ROWS
        .into_iter()
        .find(|(row, ..)| *row == name)
        .unwrap_or_else(|| panic!("no stimulus fixture named {name}"));
    let mut definition = StimulusDefinition::new(name, kind).expect("a placeable source type");
    definition.value = value.to_owned();
    definition.params = params.to_owned();
    if kind == ComponentType::VoltageSourcePwlFile {
        definition.pwl_file = Some(RetainedPwlFile::new(
            "bridge_step_c07.csv",
            MEASURED_TABLE,
            1_755_000_000_000,
        ));
    }
    definition
}

/// Every fixture definition, in the order the mockup lists them.
pub(super) fn library() -> StimulusLibrary {
    let mut library = StimulusLibrary::default();
    for (name, ..) in ROWS {
        library
            .insert(definition(name))
            .expect("unique fixture name");
    }
    library
}

/// The name of every fixture definition.
pub(super) fn names() -> Vec<&'static str> {
    ROWS.into_iter().map(|(name, ..)| name).collect()
}
