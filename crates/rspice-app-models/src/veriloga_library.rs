//! The built-in Verilog-A model library: which models ship, by device class
//! and revision, and the source text each is generated from.
//!
//! The Library page shows this as a read-only library. The sources themselves
//! are embedded by this crate's build script from `models/veriloga`.

use std::collections::BTreeMap;

use rspice_app_state::state::workspace::{ImportClosure, resolve_veriloga_import};

include!(concat!(env!("OUT_DIR"), "/veriloga_sources.rs"));

/// The name the Library page gives the built-in library.
pub const BUILTIN_VERILOGA_LIBRARY: &str = "Verilog-A models";

/// The classes the library groups its models by, in the order it shows them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BuiltinVerilogAClass {
    Mosfet,
    MultiGate,
    Soi,
    HighVoltage,
    Bipolar,
    Hemt,
    DiodeEsd,
    Passive,
}

impl BuiltinVerilogAClass {
    /// The heading the class stands under.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Mosfet => "MOSFET",
            Self::MultiGate => "Multi-gate FET",
            Self::Soi => "SOI",
            Self::HighVoltage => "High-voltage MOSFET",
            Self::Bipolar => "BJT and HBT",
            Self::Hemt => "HEMT",
            Self::DiodeEsd => "Diode and ESD",
            Self::Passive => "Passive",
        }
    }
}

/// One model the library holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinVerilogAModel {
    /// The engine's registry name, the model's identity.
    pub registry_name: &'static str,
    /// What the library calls it.
    pub name: &'static str,
    pub class: BuiltinVerilogAClass,
    /// The released revision of the model code.
    pub version: &'static str,
    pub summary: &'static str,
    /// The module the source declares.
    pub module: &'static str,
    /// The root file, relative to the model tree.
    pub root: &'static str,
    /// Whether its source branches on the simulator being ngspice, and is
    /// built with that branch taken.
    pub ngspice_branch: bool,
}

#[allow(clippy::too_many_arguments)]
const fn entry(
    registry_name: &'static str,
    name: &'static str,
    class: BuiltinVerilogAClass,
    version: &'static str,
    summary: &'static str,
    module: &'static str,
    root: &'static str,
    ngspice_branch: bool,
) -> BuiltinVerilogAModel {
    BuiltinVerilogAModel {
        registry_name,
        name,
        class,
        version,
        summary,
        module,
        root,
        ngspice_branch,
    }
}

/// Every model, grouped by class and in the order each class shows them.
pub const BUILTIN_VERILOGA_MODELS: &[BuiltinVerilogAModel] = &[
    entry(
        "bsimbulk",
        "BSIM-BULK",
        BuiltinVerilogAClass::Mosfet,
        "107.2.1",
        "Bulk MOSFET",
        "bsimbulk",
        "cmc/BSIM-BULK107.2.1_02112025/code/bsimbulk.va",
        false,
    ),
    entry(
        "PSP104VA",
        "PSP",
        BuiltinVerilogAClass::Mosfet,
        "104.1.0",
        "Surface-potential bulk MOSFET",
        "PSP104VA",
        "cmc/PSP104.1.0_vacode/vacode/psp104.va",
        false,
    ),
    entry(
        "PSPNQS104VA",
        "PSP NQS",
        BuiltinVerilogAClass::Mosfet,
        "104.1.0",
        "PSP with non-quasi-static effects",
        "PSPNQS104VA",
        "cmc/PSP104.1.0_vacode/vacode/psp104_nqs.va",
        false,
    ),
    entry(
        "PSP104TVA",
        "PSP self-heating",
        BuiltinVerilogAClass::Mosfet,
        "104.1.0",
        "PSP with a thermal node",
        "PSP104TVA",
        "cmc/PSP104.1.0_vacode/vacode/psp104t.va",
        false,
    ),
    entry(
        "ekv_va",
        "EKV 2.6",
        BuiltinVerilogAClass::Mosfet,
        "2.6.15",
        "Charge-based compact MOSFET",
        "ekv_va",
        "ekv26_2.6/ekv26_SDext_Verilog-A.va",
        false,
    ),
    entry(
        "ekv3_rf",
        "EKV3 RF",
        BuiltinVerilogAClass::Mosfet,
        "302.00",
        "EKV3 with RF extensions",
        "ekv3_rf",
        "ekv3_302.00/vacode/ekv3.va",
        false,
    ),
    entry(
        "bsimcmg_va",
        "BSIM-CMG",
        BuiltinVerilogAClass::MultiGate,
        "112.1.0",
        "FinFET and gate-all-around",
        "bsimcmg_va",
        "cmc/BSIM-CMG_112.1.0_04282026/code/bsimcmg.va",
        false,
    ),
    entry(
        "bsimimg",
        "BSIM-IMG",
        BuiltinVerilogAClass::MultiGate,
        "103.0.0",
        "Independent-gate double-gate FET",
        "bsimimg",
        "cmc/BSIM-IMG_103.0.0_20200102/code/bsimimg.va",
        false,
    ),
    entry(
        "bsimsoi_va",
        "BSIM-SOI 4.6",
        BuiltinVerilogAClass::Soi,
        "4.6.1",
        "Partially depleted SOI with floating body",
        "bsimsoi_va",
        "bsimsoi_4.6.1/vacode/bsimsoi.va",
        false,
    ),
    entry(
        "bsimsoi__18c250bc",
        "BSIM-SOI 4.7",
        BuiltinVerilogAClass::Soi,
        "4.7.0",
        "Partially depleted SOI with floating body",
        "bsimsoi",
        "cmc/BSIM-SOI_4.7.0_05192025/code/bsimsoi.va",
        false,
    ),
    entry(
        "bsimsoi__e2aff994",
        "BSIM-SOI 100",
        BuiltinVerilogAClass::Soi,
        "100.1.1",
        "Partially depleted SOI with a body tie",
        "bsimsoi",
        "cmc/BSIM_SOI_100.1.1_09152025/code/bsimsoi.va",
        false,
    ),
    entry(
        "hisimsoi_va__5be18005",
        "HiSIM-SOI",
        BuiltinVerilogAClass::Soi,
        "1.5.0",
        "Partially depleted SOI",
        "hisimsoi_va",
        "cmc/HiSIM_SOI_1.5.0_Release_20211008/HiSIM_SOI_1.5.0_VA-Code/hisimsoi_va/hisimsoi.va",
        false,
    ),
    entry(
        "hisimsoi_va__242bc21d",
        "HiSIM-SOI n4",
        BuiltinVerilogAClass::Soi,
        "1.5.0",
        "HiSIM-SOI in its four-node form",
        "hisimsoi_va",
        "cmc/HiSIM_SOI_1.5.0_Release_20211008/HiSIM_SOI_1.5.0_VA-Code/hisimsoi_va/hisimsoi_n4.va",
        false,
    ),
    entry(
        "hisimsoi_va__38074d06",
        "HiSIM-SOI n5",
        BuiltinVerilogAClass::Soi,
        "1.5.0",
        "HiSIM-SOI in its five-node form",
        "hisimsoi_va",
        "cmc/HiSIM_SOI_1.5.0_Release_20211008/HiSIM_SOI_1.5.0_VA-Code/hisimsoi_va/hisimsoi_n5.va",
        false,
    ),
    entry(
        "hisimsotb_va",
        "HiSIM-SOTB",
        BuiltinVerilogAClass::Soi,
        "1.3.0",
        "Fully depleted thin-box SOI",
        "hisimsotb_va",
        "cmc/HiSIM_SOTB_1.3.0_Release_20211116/HiSIM_SOTB_1.3.0_VA-Code/hisimsotb_va/hisimsotb.va",
        false,
    ),
    entry(
        "l_utsoi__832ce87d",
        "L-UTSOI",
        BuiltinVerilogAClass::Soi,
        "102.9.0",
        "Fully depleted SOI",
        "l_utsoi",
        "cmc/L_UTSOI_102.9.0_code_package/vacode/L_UTSOI_102.va",
        false,
    ),
    entry(
        "l_utsoi__485e0ac9",
        "L-UTSOI NQS",
        BuiltinVerilogAClass::Soi,
        "102.9.0",
        "L-UTSOI with non-quasi-static effects",
        "l_utsoi",
        "cmc/L_UTSOI_102.9.0_code_package/vacode/L_UTSOI_102_nqs.va",
        false,
    ),
    entry(
        "hisimhv_va",
        "HiSIM-HV",
        BuiltinVerilogAClass::HighVoltage,
        "2.5.1",
        "High-voltage MOSFET with six terminals",
        "hisimhv_va",
        "cmc/HiSIM_HV_2.5.1_Release_20230209/HiSIM_HV_2.5.1_VA-Code/hisimhv_va/hisimhv.va",
        false,
    ),
    entry(
        "hisimhv_n4_va",
        "HiSIM-HV n4",
        BuiltinVerilogAClass::HighVoltage,
        "2.5.1",
        "High-voltage MOSFET with four terminals",
        "hisimhv_n4_va",
        "cmc/HiSIM_HV_2.5.1_Release_20230209/HiSIM_HV_2.5.1_VA-Code/hisimhv_va/hisimhv_n4.va",
        false,
    ),
    entry(
        "hisimhv_n5_va",
        "HiSIM-HV n5",
        BuiltinVerilogAClass::HighVoltage,
        "2.5.1",
        "High-voltage MOSFET with five terminals",
        "hisimhv_n5_va",
        "cmc/HiSIM_HV_2.5.1_Release_20230209/HiSIM_HV_2.5.1_VA-Code/hisimhv_va/hisimhv_n5.va",
        false,
    ),
    entry(
        "bjt505_va",
        "Mextram 505",
        BuiltinVerilogAClass::Bipolar,
        "505.5.0",
        "Bipolar with a substrate terminal",
        "bjt505_va",
        "cmc/505p5p0_va/bjt505.va",
        false,
    ),
    entry(
        "bjt505t_va",
        "Mextram 505 self-heating",
        BuiltinVerilogAClass::Bipolar,
        "505.5.0",
        "Mextram 505 with a thermal node",
        "bjt505t_va",
        "cmc/505p5p0_va/bjt505t.va",
        false,
    ),
    entry(
        "bjtd505_va",
        "Mextram 505 discrete",
        BuiltinVerilogAClass::Bipolar,
        "505.5.0",
        "Bipolar without a substrate terminal",
        "bjtd505_va",
        "cmc/505p5p0_va/bjtd505.va",
        false,
    ),
    entry(
        "bjtd505t_va",
        "Mextram 505 discrete self-heating",
        BuiltinVerilogAClass::Bipolar,
        "505.5.0",
        "Discrete Mextram 505 with a thermal node",
        "bjtd505t_va",
        "cmc/505p5p0_va/bjtd505t.va",
        false,
    ),
    entry(
        "hicumL0va",
        "HICUM L0",
        BuiltinVerilogAClass::Bipolar,
        "2.1.0",
        "Compact SiGe HBT",
        "hicumL0va",
        "cmc/hicumL0_v2p1p0_files/hicumL0_v2p1p0.va",
        false,
    ),
    entry(
        "hicumL2va",
        "HICUM L2",
        BuiltinVerilogAClass::Bipolar,
        "3.2.0",
        "Full SiGe HBT",
        "hicumL2va",
        "cmc/hicumL2_v320_files/hicumL2_v320.va",
        false,
    ),
    entry(
        "vbic13",
        "VBIC 3-terminal",
        BuiltinVerilogAClass::Bipolar,
        "1.3",
        "Bipolar with three terminals",
        "vbic13",
        "vbic_1.3/vacode/vbic_3T.va",
        true,
    ),
    entry(
        "vbic13_3t_et",
        "VBIC 3-terminal self-heating",
        BuiltinVerilogAClass::Bipolar,
        "1.3",
        "VBIC with a thermal node",
        "vbic13_3t_et",
        "vbic_1.3/vacode/vbic_3T_et.va",
        true,
    ),
    entry(
        "vbic13_4t",
        "VBIC 4-terminal",
        BuiltinVerilogAClass::Bipolar,
        "1.3",
        "Bipolar with four terminals",
        "vbic13_4t",
        "vbic_1.3/vacode/vbic_1p3.va",
        true,
    ),
    entry(
        "vbic_4T_et_cf",
        "VBIC 4-terminal self-heating",
        BuiltinVerilogAClass::Bipolar,
        "1.3",
        "VBIC with a thermal node",
        "vbic_4T_et_cf",
        "vbic_1.3/vacode/vbic_4T_et_cf.va",
        true,
    ),
    entry(
        "asmhemt",
        "ASM-HEMT",
        BuiltinVerilogAClass::Hemt,
        "101.6.0",
        "GaN HEMT, surface potential",
        "asmhemt",
        "cmc/ASM-HEMT101.6.0_05132026/vacode/asmhemt.va",
        false,
    ),
    entry(
        "mvsg_cmc",
        "MVSG",
        BuiltinVerilogAClass::Hemt,
        "4.0.0",
        "GaN HEMT, virtual source",
        "mvsg_cmc",
        "cmc/mvsg_cmc_v4.0.0_official/vacode/mvsg_cmc_4.0.0.va",
        false,
    ),
    entry(
        "EPFL_HEMT_10a",
        "EPFL-HEMT",
        BuiltinVerilogAClass::Hemt,
        "3.0.0",
        "GaN HEMT, charge based",
        "EPFL_HEMT_10a",
        "epfl_hemt_3.0.0/vacode/epfl_hemt.va",
        false,
    ),
    entry(
        "angelov",
        "Angelov",
        BuiltinVerilogAClass::Hemt,
        "2.0",
        "Empirical large-signal FET",
        "angelov",
        "angelov_2.0/vacode/angelov.va",
        false,
    ),
    entry(
        "angelov_gan",
        "Angelov GaN",
        BuiltinVerilogAClass::Hemt,
        "2.0",
        "Empirical GaN FET with a thermal node",
        "angelov_gan",
        "angelov_2.0/vacode/angelov_gan.va",
        false,
    ),
    entry(
        "DIODE_CMC",
        "CMC diode",
        BuiltinVerilogAClass::DiodeEsd,
        "3.0",
        "Junction diode",
        "DIODE_CMC",
        "cmc/diode_cmc_3.0_20250714/vacode/diode_cmc.va",
        false,
    ),
    entry(
        "JUNCAP200",
        "JUNCAP 200",
        BuiltinVerilogAClass::DiodeEsd,
        "200.7.0",
        "Junction capacitance and leakage",
        "JUNCAP200",
        "cmc/PSP104.1.0_vacode/vacode/juncap200.va",
        false,
    ),
    entry(
        "asmesd",
        "ASM-ESD",
        BuiltinVerilogAClass::DiodeEsd,
        "101.1.0",
        "ESD bipolar clamp",
        "asmesd",
        "cmc/ASM-ESD101.1.0_04042025/asmesd.va",
        false,
    ),
    entry(
        "asmesd_dio",
        "ASM-ESD diode",
        BuiltinVerilogAClass::DiodeEsd,
        "101.1.0",
        "ESD diode",
        "asmesd_dio",
        "cmc/ASM-ESD101.1.0_04042025/asmesd_dio.va",
        false,
    ),
    entry(
        "r2_cmc",
        "R2",
        BuiltinVerilogAClass::Passive,
        "1.0.2",
        "Two-terminal semiconductor resistor",
        "r2_cmc",
        "cmc/r2_cmc_v1.0.2/r2_cmc.va",
        false,
    ),
    entry(
        "r2_et_cmc",
        "R2 self-heating",
        BuiltinVerilogAClass::Passive,
        "1.0.2",
        "R2 with a thermal node",
        "r2_et_cmc",
        "cmc/r2_cmc_v1.0.2/r2_et_cmc.va",
        false,
    ),
    entry(
        "r3_cmc",
        "R3",
        BuiltinVerilogAClass::Passive,
        "1.1.2",
        "Three-terminal semiconductor resistor",
        "r3_cmc",
        "cmc/r3_cmc_release1.1.2_2023Jun16/r3_cmc.va",
        false,
    ),
    entry(
        "mosvar",
        "MOSVAR",
        BuiltinVerilogAClass::Passive,
        "1.4.0",
        "MOS varactor",
        "mosvar",
        "cmc/MOSVAR140/mosvar.va",
        false,
    ),
];

/// The model the engine knows by `registry_name`.
#[must_use]
pub fn builtin_veriloga_model(registry_name: &str) -> Option<&'static BuiltinVerilogAModel> {
    BUILTIN_VERILOGA_MODELS
        .iter()
        .find(|model| model.registry_name == registry_name)
}

/// The embedded file at `path` relative to the model tree.
#[must_use]
pub fn builtin_veriloga_source(path: &str) -> Option<&'static str> {
    VERILOGA_SOURCES
        .binary_search_by_key(&path, |(file, _)| *file)
        .ok()
        .map(|index| VERILOGA_SOURCES[index].1)
}

/// A model's source as the files of an import: the root by its own name, and
/// what its includes reach, read from the embedded tree.
#[derive(Debug, Clone)]
pub struct BuiltinVerilogASource {
    /// The root file's name, the key of its text in `files`.
    pub root: String,
    pub files: BTreeMap<String, String>,
    pub closure: ImportClosure,
}

/// Walk `model`'s includes through the embedded tree, the way an import does.
#[must_use]
pub fn read_builtin_veriloga_model(model: &BuiltinVerilogAModel) -> BuiltinVerilogASource {
    let (folder, root) = model.root.rsplit_once('/').unwrap_or(("", model.root));
    let text = builtin_veriloga_source(model.root).unwrap_or_default();
    let mut files = BTreeMap::from([(root.to_owned(), text.to_owned())]);
    let read = |logical: &str| {
        let path = if folder.is_empty() {
            logical.to_owned()
        } else {
            format!("{folder}/{logical}")
        };
        builtin_veriloga_source(&normalized_path(&path)).map(str::to_owned)
    };
    let closure = resolve_veriloga_import(root, &mut files, &read);
    BuiltinVerilogASource {
        root: root.to_owned(),
        files,
        closure,
    }
}

/// `path` with its `.` and `..` segments folded away.
fn normalized_path(path: &str) -> String {
    let mut kept: Vec<&str> = Vec::new();
    for segment in path.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                kept.pop();
            }
            other => kept.push(other),
        }
    }
    kept.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository_file(relative: &str) -> String {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join(relative),
        )
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
    }

    /// The generated catalog is the census of what the engine ships: every
    /// model it lists has a row here, and no row names anything else.
    #[test]
    fn the_library_holds_exactly_the_models_the_engine_ships() {
        let readme = repository_file("crates/rspice-veriloga-models/README.md");
        let shipped = readme
            .lines()
            .filter_map(|line| line.strip_prefix("| `")?.split('`').next())
            .filter(|name| !name.starts_with("veriloga-"))
            .collect::<std::collections::BTreeSet<_>>();
        let listed = BUILTIN_VERILOGA_MODELS
            .iter()
            .map(|model| model.registry_name)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(shipped, listed);
        assert_eq!(listed.len(), BUILTIN_VERILOGA_MODELS.len());
    }

    /// Every row names a root the profile pairs with its module.
    #[test]
    fn every_model_names_the_root_its_generated_code_came_from() {
        let profile = repository_file("models/veriloga/.rspice-veriloga-profile");
        for model in BUILTIN_VERILOGA_MODELS {
            let pair = format!("namespace {}::{}=", model.root, model.module);
            assert!(
                profile.lines().any(|line| line.starts_with(&pair)),
                "{} is not {pair}",
                model.registry_name
            );
        }
    }

    /// Each class stays together, so a heading is drawn once for it.
    #[test]
    fn the_models_of_a_class_are_adjacent_and_classes_keep_their_order() {
        let classes = BUILTIN_VERILOGA_MODELS
            .iter()
            .map(|model| model.class)
            .collect::<Vec<_>>();
        assert!(classes.windows(2).all(|pair| pair[0] <= pair[1]));
    }

    /// A copy needs the whole closure and the module the row says it declares.
    #[test]
    fn every_model_reaches_all_its_includes_and_declares_its_module() {
        for model in BUILTIN_VERILOGA_MODELS {
            let source = read_builtin_veriloga_model(model);
            let missing = source
                .closure
                .missing()
                .map(|file| file.path.clone())
                .collect::<Vec<_>>();
            assert!(missing.is_empty(), "{}: {missing:?}", model.registry_name);
            let modules = source
                .closure
                .modules
                .as_ref()
                .unwrap_or_else(|error| panic!("{}: {error}", model.registry_name));
            assert!(
                modules.iter().any(|declared| declared.name == model.module),
                "{}: {modules:?}",
                model.registry_name
            );
        }
    }
}
