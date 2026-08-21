//! Model types, and the device each maps onto.

use serde::{Deserialize, Serialize};

/// Type/category of device model
///
/// Polarity is part of the type wherever the `.MODEL` card's own type token
/// carries it — `NJF` and `PJF` are two tokens, not one token and a sign — so
/// the transistor families are split the way SPICE spells them rather than
/// folded into one variant with an attribute beside it.
///
/// Variants are serialized by name and read back by name. New ones may be
/// added, and the order here is presentational, but no existing variant may be
/// renamed: a saved project names its cards' types in these exact words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ModelType {
    /// NMOS transistor
    #[default]
    Nmos,
    /// PMOS transistor
    Pmos,
    /// NPN bipolar
    Npn,
    /// PNP bipolar
    Pnp,
    /// N-channel junction FET (`NJF`)
    Njfet,
    /// P-channel junction FET (`PJF`)
    Pjfet,
    /// N-channel MESFET/HFET (`NMF`, `NHFET`)
    Nmesfet,
    /// P-channel MESFET/HFET (`PMF`, `PHFET`)
    Pmesfet,
    /// N-channel vertical power MOSFET (`NVDMOS`, `VDMOSN`)
    NVdmos,
    /// P-channel vertical power MOSFET (`PVDMOS`, `VDMOSP`)
    PVdmos,
    /// Resistor
    Resistor,
    /// Capacitor
    Capacitor,
    /// Inductor
    Inductor,
    /// Diode
    Diode,
    /// Varactor
    Varactor,
    /// RF device
    Rf,
    /// ESD protection
    Esd,
    /// Custom/other
    Other,
}

impl ModelType {
    /// Display name
    ///
    /// Presentation only. Nothing may classify, key, or round-trip a card by
    /// this string — [`Self::spice_token`] is what a `.MODEL` card is written
    /// with, and [`Self::from_name`] is what one is read back by.
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelType::Nmos => "NMOS",
            ModelType::Pmos => "PMOS",
            ModelType::Npn => "NPN",
            ModelType::Pnp => "PNP",
            ModelType::Njfet => "NJFET",
            ModelType::Pjfet => "PJFET",
            ModelType::Nmesfet => "NMESFET",
            ModelType::Pmesfet => "PMESFET",
            ModelType::NVdmos => "NVDMOS",
            ModelType::PVdmos => "PVDMOS",
            ModelType::Resistor => "Resistor",
            ModelType::Capacitor => "Capacitor",
            ModelType::Inductor => "Inductor",
            ModelType::Diode => "Diode",
            ModelType::Varactor => "Varactor",
            ModelType::Rf => "RF",
            ModelType::Esd => "ESD",
            ModelType::Other => "Other",
        }
    }

    /// The family a `.MODEL` card's declared type token names.
    ///
    /// This is the catalog's one classification vocabulary: the `.lib` reader
    /// classifies a parsed card through it, so an `NMF` card is an n-channel
    /// MESFET wherever it is read, and no surface re-derives a family of its
    /// own from the same token.
    ///
    /// A token this build has no family for answers [`ModelType::Other`],
    /// which is a statement about this vocabulary and not about the card: the
    /// exact token is retained beside the type on every card, and the shelf's
    /// class column reads *that*. Switches, couplings, magnetic cores and
    /// transmission lines are all real families with no variant here, and
    /// `Other` is the honest answer for each of them.
    ///
    /// Bare `VDMOS` is deliberately absent. ngspice spells that card's
    /// polarity as a flag *parameter* — `.model IRFP240 VDMOS nchan` — which a
    /// type-token classifier cannot see, so naming a polarity here would file
    /// every p-channel power MOSFET under N. The engine reads the flag where
    /// it can see it (`resolve_vdmos_type_from_model`), and the polarity-typed
    /// spellings this build's own foundation cards use — `NVDMOS`, `PVDMOS`,
    /// and Xyce's `VDMOSN`/`VDMOSP` — do classify.
    pub fn from_name(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "nmos" | "nch" | "n" => ModelType::Nmos,
            "pmos" | "pch" | "p" => ModelType::Pmos,
            "npn" => ModelType::Npn,
            // A lateral PNP is a PNP whose card declares a different token,
            // which is how the shipped-corpus generator classifies it too.
            "pnp" | "lpnp" => ModelType::Pnp,
            "njf" => ModelType::Njfet,
            "pjf" => ModelType::Pjfet,
            // HFET cards are MESFET cards in this build: one device, one
            // symbol, two accepted spellings — the same pairing the binding
            // gate makes.
            "nmf" | "nhfet" => ModelType::Nmesfet,
            "pmf" | "phfet" => ModelType::Pmesfet,
            "nvdmos" | "vdmosn" => ModelType::NVdmos,
            "pvdmos" | "vdmosp" => ModelType::PVdmos,
            "r" | "res" | "resistor" => ModelType::Resistor,
            "c" | "cap" | "capacitor" => ModelType::Capacitor,
            "l" | "ind" | "inductor" => ModelType::Inductor,
            "d" | "diode" => ModelType::Diode,
            "var" | "varactor" => ModelType::Varactor,
            "rf" => ModelType::Rf,
            "esd" => ModelType::Esd,
            _ => ModelType::Other,
        }
    }

    /// The `.MODEL` type token a card of this family declares.
    ///
    /// The inverse of [`Self::from_name`] for every family SPICE has a token
    /// for, so a card this workspace authors is written with the token the
    /// parser will read back rather than with a rendering of a label. `Rf`,
    /// `Esd` and `Other` are catalog categories rather than card types and
    /// answer `None`: there is no `.MODEL … ESD` card to write, and inventing
    /// one would author a deck nothing can execute.
    pub const fn spice_token(&self) -> Option<&'static str> {
        Some(match self {
            ModelType::Nmos => "NMOS",
            ModelType::Pmos => "PMOS",
            ModelType::Npn => "NPN",
            ModelType::Pnp => "PNP",
            ModelType::Njfet => "NJF",
            ModelType::Pjfet => "PJF",
            ModelType::Nmesfet => "NMF",
            ModelType::Pmesfet => "PMF",
            ModelType::NVdmos => "NVDMOS",
            ModelType::PVdmos => "PVDMOS",
            ModelType::Resistor => "R",
            ModelType::Capacitor => "C",
            ModelType::Inductor => "L",
            // A varactor is a junction diode card with a tuning-oriented
            // parameter set; SPICE has no separate token for one.
            ModelType::Diode | ModelType::Varactor => "D",
            ModelType::Rf | ModelType::Esd | ModelType::Other => return None,
        })
    }
}

/// SPICE model level/type.
///
/// This classifies what a model card *claims to be* for browsing and
/// filtering — it is not a statement of native engine support. Cards run
/// natively only for the levels the core implements; bundled model metadata
/// is not a promise that a third-party Verilog-A source is shipped or
/// simulation-ready.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ModelLevel {
    /// BSIM3 v3.3
    Bsim3v3,
    /// BSIM4
    Bsim4,
    /// BSIM-SOI family (PD/FD/DD and the CMC Verilog-A releases)
    BsimSoi,
    /// BSIM CMG (FinFET)
    BsimCmg,
    /// BSIM-BULK
    BsimBulk,
    /// BSIM-IMG
    BsimImg,
    /// PSP (including thermal and NQS releases)
    Psp,
    /// EKV (2.6 / 3.x)
    Ekv,
    /// HiSIM family (HV / SOI / SOTB)
    HiSim,
    /// L-UTSOI
    LUtsoi,
    /// MOSVAR varactor
    Mosvar,
    /// MVSG-CMC (GaN)
    Mvsg,
    /// VDMOS power MOSFET
    Vdmos,
    /// VBIC 1.3
    Vbic,
    /// MEXTRAM 504/505
    Mextram,
    /// HICUM L0/L2
    Hicum,
    /// GaAs/GaN HEMT compact models (ASM-HEMT, Angelov, EPFL HEMT)
    Hemt,
    /// JUNCAP200 junction model
    Juncap,
    /// DIODE_CMC
    DiodeCmc,
    /// CMC resistor models (r2_cmc / r3_cmc)
    RCmc,
    /// Verilog-A compact model
    VerilogA,
    /// SPICE Level 1
    SpiceLevel1,
    /// SPICE Level 3
    SpiceLevel3,
    /// Unknown/custom
    #[default]
    Unknown,
}

impl ModelLevel {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelLevel::Bsim3v3 => "BSIM3v3",
            ModelLevel::Bsim4 => "BSIM4",
            ModelLevel::BsimSoi => "BSIM-SOI",
            ModelLevel::BsimCmg => "BSIM-CMG",
            ModelLevel::BsimBulk => "BSIM-BULK",
            ModelLevel::BsimImg => "BSIM-IMG",
            ModelLevel::Psp => "PSP",
            ModelLevel::Ekv => "EKV",
            ModelLevel::HiSim => "HiSIM",
            ModelLevel::LUtsoi => "L-UTSOI",
            ModelLevel::Mosvar => "MOSVAR",
            ModelLevel::Mvsg => "MVSG",
            ModelLevel::Vdmos => "VDMOS",
            ModelLevel::Vbic => "VBIC",
            ModelLevel::Mextram => "MEXTRAM",
            ModelLevel::Hicum => "HICUM",
            ModelLevel::Hemt => "HEMT",
            ModelLevel::Juncap => "JUNCAP",
            ModelLevel::DiodeCmc => "DIODE_CMC",
            ModelLevel::RCmc => "R-CMC",
            ModelLevel::VerilogA => "Verilog-A",
            ModelLevel::SpiceLevel1 => "SPICE L1",
            ModelLevel::SpiceLevel3 => "SPICE L3",
            ModelLevel::Unknown => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every family with a card token reads back as itself.
    ///
    /// Written as a round trip rather than as a table of pairs because the
    /// failure this guards is the two halves drifting apart: a token written
    /// one way and read another puts a card in a family it will not be found
    /// in again.
    #[test]
    fn a_written_card_token_reads_back_as_the_family_that_wrote_it() {
        for model_type in [
            ModelType::Nmos,
            ModelType::Pmos,
            ModelType::Npn,
            ModelType::Pnp,
            ModelType::Njfet,
            ModelType::Pjfet,
            ModelType::Nmesfet,
            ModelType::Pmesfet,
            ModelType::NVdmos,
            ModelType::PVdmos,
            ModelType::Resistor,
            ModelType::Capacitor,
            ModelType::Inductor,
            ModelType::Diode,
        ] {
            let token = model_type
                .spice_token()
                .unwrap_or_else(|| panic!("{model_type:?} declares a card token"));
            assert_eq!(
                ModelType::from_name(token),
                model_type,
                "'{token}' does not read back as {model_type:?}"
            );
        }

        // A varactor writes the diode card it is, and reads back as the diode
        // family — the one place the round trip is deliberately not an
        // identity, because SPICE has no varactor card to write.
        assert_eq!(ModelType::Varactor.spice_token(), Some("D"));
        for category in [ModelType::Rf, ModelType::Esd, ModelType::Other] {
            assert_eq!(
                category.spice_token(),
                None,
                "{category:?} is a catalog category, not a card type"
            );
        }
    }

    /// The families whose cards used to fall through to `Other`.
    #[test]
    fn the_polarity_typed_transistor_tokens_classify() {
        for (token, expected) in [
            ("NJF", ModelType::Njfet),
            ("pjf", ModelType::Pjfet),
            ("NMF", ModelType::Nmesfet),
            ("NHFET", ModelType::Nmesfet),
            ("PMF", ModelType::Pmesfet),
            ("PHFET", ModelType::Pmesfet),
            ("NVDMOS", ModelType::NVdmos),
            ("VDMOSN", ModelType::NVdmos),
            ("PVDMOS", ModelType::PVdmos),
            ("VDMOSP", ModelType::PVdmos),
            ("LPNP", ModelType::Pnp),
        ] {
            assert_eq!(ModelType::from_name(token), expected, "token '{token}'");
        }
    }

    /// A token whose polarity lives somewhere the classifier cannot see is not
    /// given one, and neither is a family this build has no variant for.
    #[test]
    fn undeclared_polarity_and_absent_families_stay_other() {
        assert_eq!(
            ModelType::from_name("VDMOS"),
            ModelType::Other,
            "bare VDMOS carries its polarity in a flag parameter, not the token"
        );
        for token in ["SW", "CSW", "CORE", "LTRA", "CPL", "PSP103", ""] {
            assert_eq!(
                ModelType::from_name(token),
                ModelType::Other,
                "token '{token}'"
            );
        }
    }
}
