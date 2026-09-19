//! The library every stimulus surface is reviewed and gated against.
//!
//! The mockup's fourteen definitions, in the application's own spelling: the
//! same component type, the same primary value and the same parameter string a
//! placed source of that shape would carry. They are a fixture and nothing
//! else — no surface reads them and no project ships them — but they are the
//! only set that covers every family a stimulus surface has a distinct layout
//! for, so the rasters and the fit gates seed from here rather than from
//! whatever definition a test happened to write.
//!
//! They sit beside the model rather than beside one surface because three
//! surfaces are gated against them — the instrument, the library browser and
//! the inspector — and they name nothing above `state`. A copy per surface
//! would be three libraries drifting apart under three sets of renders.

use crate::state::ComponentType;

use super::definition::{RetainedPwlFile, StimulusDefinition};
use super::library::StimulusLibrary;

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
/// to describe — and the preview a waveform to draw — without reaching a lab
/// share that is not there.
///
/// Plain numbers, as an instrument exports them: the engine's table loader
/// reads decimal text and no SPICE suffixes, so `200u` here would be a file
/// the engine refuses at its second line.
const MEASURED_TABLE: &str = "0 0\n200e-6 0\n210e-6 8.2e-3\n230e-6 11.6e-3\n260e-6 12.4e-3\n\
                              300e-6 10.4e-3\n360e-6 9.2e-3\n430e-6 9.7e-3\n520e-6 10.15e-3\n\
                              650e-6 9.94e-3\n800e-6 10.02e-3\n1e-3 9.99e-3\n1.4e-3 10e-3\n\
                              2e-3 10e-3\n";

/// One definition, by the name the mockup gives it.
pub(crate) fn definition(name: &str) -> StimulusDefinition {
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
pub(crate) fn library() -> StimulusLibrary {
    let mut library = StimulusLibrary::default();
    for (name, ..) in ROWS {
        library
            .insert(definition(name))
            .expect("unique fixture name");
    }
    library
}

/// The name of every fixture definition.
pub(crate) fn names() -> Vec<&'static str> {
    ROWS.into_iter().map(|(name, ..)| name).collect()
}
