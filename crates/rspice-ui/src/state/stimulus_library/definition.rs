//! One reusable stimulus, as the project stores it.
//!
//! A definition is a placeable source held apart from any sheet: the same
//! `(component type, value, params)` triple a placed instance carries, plus the
//! name, purpose and revision that make it a library entry. It is deliberately
//! not a second waveform model — the netlister realizes a definition through
//! exactly the component type it names, so a definition and the instance it was
//! copied onto cannot describe different cards.
//!
//! The family is the component type. RSpice spells a waveform shape as a
//! `ComponentType` (24 of them: 12 shapes × 2 quantities), and a definition
//! that named a shape the schematic cannot place would be a definition nothing
//! could adopt, so [`StimulusDefinition::new`] refuses every other type. That
//! is why [`StimulusFamily`] is exactly the engine's independent-source
//! families and nothing more: each one is placeable, so each one is adoptable.

use serde::{Deserialize, Serialize};

use crate::product::ContentDigest;
use crate::state::{ComponentType, format_params_string, parse_params_string};

/// Why a stimulus definition could not be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StimulusDefinitionError {
    /// The name is not a single SPICE identifier.
    Name(String),
    /// The component type is not one of the 24 independent-source types.
    NotASource(ComponentType),
    /// The library already holds a definition under this name.
    DuplicateName(String),
    /// The named definition is not in the library.
    Unknown(String),
}

impl std::fmt::Display for StimulusDefinitionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name(name) => write!(
                formatter,
                "'{name}' is not a SPICE identifier — a definition is named the way a source \
                 instance is, so the name must be one unquoted word the netlist reader accepts"
            ),
            Self::NotASource(kind) => write!(
                formatter,
                "{} is not an independent source, so it has no waveform to define",
                kind.display_name()
            ),
            Self::DuplicateName(name) => {
                write!(formatter, "this project already defines '{name}'")
            }
            Self::Unknown(name) => write!(formatter, "this project defines no '{name}'"),
        }
    }
}

impl std::error::Error for StimulusDefinitionError {}

/// Which quantity a stimulus drives.
///
/// Kind is circuit topology: a voltage source is a branch the solver adds a
/// current unknown for, a current source is an injection into two nodes. It is
/// never a property of the waveform, which is why adoption across kinds is
/// refused rather than converted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StimulusKind {
    /// `V` — a voltage source.
    Voltage,
    /// `I` — a current source.
    Current,
}

impl StimulusKind {
    /// The kind of an independent-source component type, or `None` for
    /// anything else on the sheet.
    #[must_use]
    pub fn of(kind: ComponentType) -> Option<Self> {
        StimulusFamily::of(kind)?;
        Some(match kind {
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourcePwlFile
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm
            | ComponentType::VoltageSourceAm
            | ComponentType::VoltageSourcePat
            | ComponentType::VoltageSourceNoise
            | ComponentType::VoltageSourceRandom => Self::Voltage,
            _ => Self::Current,
        })
    }

    /// `V` or `I`, the letter a card is named with.
    #[must_use]
    pub const fn letter(self) -> &'static str {
        match self {
            Self::Voltage => "V",
            Self::Current => "I",
        }
    }

    /// `voltage` or `current`, as a sentence names it.
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::Voltage => "voltage",
            Self::Current => "current",
        }
    }
}

/// Which waveform shape a stimulus has.
///
/// One variant per placeable shape. The label is the keyword the card carries,
/// so a surface never has to spell one itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StimulusFamily {
    /// `DC <level>`.
    Dc,
    /// `DC <bias> AC <magnitude> <phase>`.
    Ac,
    /// `PULSE(V1 V2 TD TR TF PW PER NP)`.
    Pulse,
    /// `SIN(VO VA FREQ TD THETA PHASE)`.
    Sin,
    /// `PWL(T1 V1 …) [TD=] [R=]`.
    Pwl,
    /// `PWL FILE="…" [TD=] [R=] [TSCALE=] [VSCALE=] [TOFFSET=] [VOFFSET=]`.
    PwlFile,
    /// `EXP(V1 V2 TD1 TAU1 TD2 TAU2)`.
    Exp,
    /// `SFFM(VO VA FC MDI FM TD PHASEM PHASEC)`.
    Sffm,
    /// `AM(VO VMO VMA FM FC TD PHASEM PHASEC)`.
    Am,
    /// `PAT(VHI VLO TD TR TF TSAMPLE DATA [R=])`.
    Pat,
    /// `DC <offset> TRNOISE(NA NT NALPHA NAMP RTSAM RTSCAPT RTSEMT)`.
    Trnoise,
    /// `TRRANDOM(TYPE TS TD PARAM1 PARAM2)`.
    Trrandom,
}

impl StimulusFamily {
    /// Every family a definition may name, in the order the library lists them.
    pub const ALL: [Self; 12] = [
        Self::Dc,
        Self::Ac,
        Self::Pulse,
        Self::Sin,
        Self::Pwl,
        Self::PwlFile,
        Self::Exp,
        Self::Sffm,
        Self::Am,
        Self::Pat,
        Self::Trnoise,
        Self::Trrandom,
    ];

    /// The family of an independent-source component type, or `None` for
    /// anything else on the sheet.
    #[must_use]
    pub const fn of(kind: ComponentType) -> Option<Self> {
        Some(match kind {
            ComponentType::VoltageSource | ComponentType::CurrentSource => Self::Dc,
            ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc => Self::Ac,
            ComponentType::VoltageSourcePulse | ComponentType::CurrentSourcePulse => Self::Pulse,
            ComponentType::VoltageSourceSin | ComponentType::CurrentSourceSin => Self::Sin,
            ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => Self::Pwl,
            ComponentType::VoltageSourcePwlFile | ComponentType::CurrentSourcePwlFile => {
                Self::PwlFile
            }
            ComponentType::VoltageSourceExp | ComponentType::CurrentSourceExp => Self::Exp,
            ComponentType::VoltageSourceSffm | ComponentType::CurrentSourceSffm => Self::Sffm,
            ComponentType::VoltageSourceAm | ComponentType::CurrentSourceAm => Self::Am,
            ComponentType::VoltageSourcePat | ComponentType::CurrentSourcePat => Self::Pat,
            ComponentType::VoltageSourceNoise | ComponentType::CurrentSourceNoise => Self::Trnoise,
            ComponentType::VoltageSourceRandom | ComponentType::CurrentSourceRandom => {
                Self::Trrandom
            }
            _ => return None,
        })
    }

    /// The component type this family takes on the given quantity.
    #[must_use]
    pub const fn component_type(self, kind: StimulusKind) -> ComponentType {
        match (self, kind) {
            (Self::Dc, StimulusKind::Voltage) => ComponentType::VoltageSource,
            (Self::Dc, StimulusKind::Current) => ComponentType::CurrentSource,
            (Self::Ac, StimulusKind::Voltage) => ComponentType::VoltageSourceAc,
            (Self::Ac, StimulusKind::Current) => ComponentType::CurrentSourceAc,
            (Self::Pulse, StimulusKind::Voltage) => ComponentType::VoltageSourcePulse,
            (Self::Pulse, StimulusKind::Current) => ComponentType::CurrentSourcePulse,
            (Self::Sin, StimulusKind::Voltage) => ComponentType::VoltageSourceSin,
            (Self::Sin, StimulusKind::Current) => ComponentType::CurrentSourceSin,
            (Self::Pwl, StimulusKind::Voltage) => ComponentType::VoltageSourcePwl,
            (Self::Pwl, StimulusKind::Current) => ComponentType::CurrentSourcePwl,
            (Self::PwlFile, StimulusKind::Voltage) => ComponentType::VoltageSourcePwlFile,
            (Self::PwlFile, StimulusKind::Current) => ComponentType::CurrentSourcePwlFile,
            (Self::Exp, StimulusKind::Voltage) => ComponentType::VoltageSourceExp,
            (Self::Exp, StimulusKind::Current) => ComponentType::CurrentSourceExp,
            (Self::Sffm, StimulusKind::Voltage) => ComponentType::VoltageSourceSffm,
            (Self::Sffm, StimulusKind::Current) => ComponentType::CurrentSourceSffm,
            (Self::Am, StimulusKind::Voltage) => ComponentType::VoltageSourceAm,
            (Self::Am, StimulusKind::Current) => ComponentType::CurrentSourceAm,
            (Self::Pat, StimulusKind::Voltage) => ComponentType::VoltageSourcePat,
            (Self::Pat, StimulusKind::Current) => ComponentType::CurrentSourcePat,
            (Self::Trnoise, StimulusKind::Voltage) => ComponentType::VoltageSourceNoise,
            (Self::Trnoise, StimulusKind::Current) => ComponentType::CurrentSourceNoise,
            (Self::Trrandom, StimulusKind::Voltage) => ComponentType::VoltageSourceRandom,
            (Self::Trrandom, StimulusKind::Current) => ComponentType::CurrentSourceRandom,
        }
    }

    /// The keyword a card of this family carries.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Dc => "DC",
            Self::Ac => "AC",
            Self::Pulse => "PULSE",
            Self::Sin => "SIN",
            Self::Pwl => "PWL",
            Self::PwlFile => "PWL FILE",
            Self::Exp => "EXP",
            Self::Sffm => "SFFM",
            Self::Am => "AM",
            Self::Pat => "PAT",
            Self::Trnoise => "TRNOISE",
            Self::Trrandom => "TRRANDOM",
        }
    }

    /// The label as a name fragment, for generating a fresh definition name.
    #[must_use]
    pub(super) const fn name_stem(self) -> &'static str {
        match self {
            Self::Dc => "dc",
            Self::Ac => "ac",
            Self::Pulse => "pulse",
            Self::Sin => "sin",
            Self::Pwl => "pwl",
            Self::PwlFile => "pwl_file",
            Self::Exp => "exp",
            Self::Sffm => "sffm",
            Self::Am => "am",
            Self::Pat => "pat",
            Self::Trnoise => "trnoise",
            Self::Trrandom => "trrandom",
        }
    }
}

/// The bytes of a `PWL FILE=` table, kept inside the project document.
///
/// The card names a file; the project keeps the file. That is the same bargain
/// `ProjectModelLibrary::source_contents` strikes, and for the same two
/// reasons: a browser session has no path authority at all, and a desktop
/// project that has been zipped and mailed still has to open. The digest is the
/// identity the reader compares when the named file is present on this host and
/// may or may not still be the one that was imported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetainedPwlFile {
    /// The file's own name, as the card spells it.
    pub file_name: String,
    /// Exact retained text of the table the engine's `PWL FILE` loader reads.
    pub contents: String,
    /// SHA-256 of `contents`.
    pub digest: ContentDigest,
    /// When the import happened, in unix milliseconds.
    pub imported_at_unix_ms: u64,
}

impl RetainedPwlFile {
    /// Retain one imported table, taking its digest from the bytes retained.
    #[must_use]
    pub fn new(
        file_name: impl Into<String>,
        contents: impl Into<String>,
        imported_at_unix_ms: u64,
    ) -> Self {
        let contents = contents.into();
        let digest = crate::state::content_digest(&contents);
        Self {
            file_name: file_name.into(),
            contents,
            digest,
            imported_at_unix_ms,
        }
    }
}

/// One reusable stimulus this project owns.
///
/// `value` and `params` are exactly the two fields a placed source carries:
/// the first positional argument of the waveform, and the rest as a SPICE
/// `key=value` string. Storing them in the instance's own spelling is what lets
/// adoption be a copy and realization be the netlister's ordinary path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StimulusDefinition {
    name: String,
    component_type: ComponentType,
    /// The primary positional field, as `Component::value` holds it.
    #[serde(default)]
    pub value: String,
    /// The remaining waveform fields, as `Component::params` holds them.
    #[serde(default)]
    pub params: String,
    /// What this stimulus is for, in the author's own words.
    #[serde(default)]
    pub purpose: String,
    revision: u32,
    #[serde(default)]
    modified_unix_ms: u64,
    /// The retained table a `PWL FILE=` definition reads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pwl_file: Option<RetainedPwlFile>,
}

impl StimulusDefinition {
    /// A fresh `r1` definition of one placeable family.
    ///
    /// Refuses a name that is not a single SPICE identifier and a component
    /// type that is not one of the 24 independent sources; uniqueness is the
    /// library's to enforce, because only the library knows what it holds.
    pub fn new(
        name: impl Into<String>,
        component_type: ComponentType,
    ) -> Result<Self, StimulusDefinitionError> {
        let name = name.into();
        if !is_spice_identifier(&name) {
            return Err(StimulusDefinitionError::Name(name));
        }
        if StimulusFamily::of(component_type).is_none() {
            return Err(StimulusDefinitionError::NotASource(component_type));
        }
        Ok(Self {
            name,
            component_type,
            value: String::new(),
            params: String::new(),
            purpose: String::new(),
            revision: 1,
            modified_unix_ms: super::now_unix_ms(),
            pwl_file: None,
        })
    }

    /// The definition's name, unique in its library.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The schematic type a placed adopter of this definition has.
    #[must_use]
    pub const fn component_type(&self) -> ComponentType {
        self.component_type
    }

    /// The waveform shape.
    ///
    /// Total, because the constructor already refused every type that has none.
    #[must_use]
    pub fn family(&self) -> StimulusFamily {
        StimulusFamily::of(self.component_type).unwrap_or(StimulusFamily::Dc)
    }

    /// The quantity driven.
    ///
    /// Total, for the same reason [`Self::family`] is.
    #[must_use]
    pub fn kind(&self) -> StimulusKind {
        StimulusKind::of(self.component_type).unwrap_or(StimulusKind::Voltage)
    }

    /// Which saved revision this is. Apply publishes the next one.
    #[must_use]
    pub const fn revision(&self) -> u32 {
        self.revision
    }

    /// When this revision was published, in unix milliseconds.
    #[must_use]
    pub const fn modified_unix_ms(&self) -> u64 {
        self.modified_unix_ms
    }

    /// The same definition under a different family, on the same quantity.
    ///
    /// A family switch resets the shape parameters, and it has to: `PULSE`'s
    /// `pw=` has no counterpart on a `SIN`, and carrying it across would invent
    /// a value nobody authored. Cleared `value`/`params` are the sheet's own
    /// defaults — every field resolves to its `PropertyDefinition::default_value`
    /// when the instance carries nothing, which is exactly what a freshly
    /// placed source of the new type does — so the switched definition realizes
    /// to the new family's default card. Undo restores the parameters the old
    /// family had; the draft's history is what makes that true.
    #[must_use]
    pub fn with_family(&self, family: StimulusFamily) -> Self {
        let component_type = family.component_type(self.kind());
        Self {
            name: self.name.clone(),
            component_type,
            value: String::new(),
            params: String::new(),
            purpose: self.purpose.clone(),
            revision: self.revision,
            modified_unix_ms: self.modified_unix_ms,
            pwl_file: match family {
                StimulusFamily::PwlFile => self.pwl_file.clone(),
                _ => None,
            },
        }
    }

    /// The same definition on the other quantity, keeping its shape.
    ///
    /// Every waveform field but the first positional keeps its spelling across
    /// the two kinds — the netlister's `SFFM`, `AM`, `PAT`, `TRNOISE` and
    /// `TRRANDOM` lists are shared outright — so a kind switch is a retype
    /// rather than a reset.
    /// The primary value moves with the card because it is the same number in a
    /// different unit, and the author is the one who decides whether 5 V should
    /// have become 5 A.
    #[must_use]
    pub fn with_kind(&self, kind: StimulusKind) -> Self {
        let mut switched = self.clone();
        switched.component_type = self.family().component_type(kind);
        switched
    }

    /// Rename the definition, refusing anything that is not an identifier.
    pub fn rename(&mut self, name: impl Into<String>) -> Result<(), StimulusDefinitionError> {
        let name = name.into();
        if !is_spice_identifier(&name) {
            return Err(StimulusDefinitionError::Name(name));
        }
        self.name = name;
        Ok(())
    }

    /// Publish the next revision, stamping the moment it was published.
    pub(super) fn publish_next_revision(&mut self) {
        self.revision = self.revision.saturating_add(1);
        self.modified_unix_ms = super::now_unix_ms();
    }

    /// Start the revision count over, for a definition that is new rather than
    /// edited — a duplicate, or one extracted from a placed source.
    pub(super) fn restart_revisions(&mut self) {
        self.revision = 1;
        self.modified_unix_ms = super::now_unix_ms();
    }

    /// This definition with its parameter text canonicalized and its revision
    /// stamps dropped, so two records compare on what they say rather than on
    /// when they were written or in which order their keys happen to sit.
    #[must_use]
    pub fn normalized(&self) -> Self {
        Self {
            name: self.name.clone(),
            component_type: self.component_type,
            value: self.value.trim().to_owned(),
            params: normalize_params(&self.params),
            purpose: self.purpose.clone(),
            revision: 0,
            modified_unix_ms: 0,
            pwl_file: self.pwl_file.clone(),
        }
    }
}

/// A parameter string in one canonical spelling.
///
/// Round-tripping through the grammar sorts the keys and drops empty ones, so
/// `va=1 vo=0` and `vo=0 va=1` are the same text and neither reads as an edit.
#[must_use]
pub fn normalize_params(params: &str) -> String {
    format_params_string(&parse_params_string(params))
}

/// Whether a name is one unquoted word the netlist reader accepts.
///
/// The engine's own lexer decides, rather than a character class written here:
/// a definition's name becomes a source instance's name the moment someone
/// places it, and the only authority on what that may be is the reader that
/// will parse the card.
#[must_use]
pub fn is_spice_identifier(name: &str) -> bool {
    use rspice_core::netlist::lexer::TokenKind;

    let name = name.trim();
    if name.is_empty()
        || name != name.trim()
        || name
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
    {
        return false;
    }
    let Ok(tokens) = rspice_core::netlist::lexer::tokenize(name) else {
        return false;
    };
    tokens.len() == 2 && matches!(tokens[0].kind, TokenKind::Ident(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_placeable_source_type_round_trips_through_family_and_kind() {
        let mut seen = Vec::new();
        for family in StimulusFamily::ALL {
            for kind in [StimulusKind::Voltage, StimulusKind::Current] {
                let component_type = family.component_type(kind);
                assert_eq!(StimulusFamily::of(component_type), Some(family));
                assert_eq!(StimulusKind::of(component_type), Some(kind));
                seen.push(component_type);
            }
        }
        seen.sort_by_key(|kind| format!("{kind:?}"));
        seen.dedup();
        assert_eq!(
            seen.len(),
            24,
            "the definition's family set is exactly the placeable source types"
        );
    }

    #[test]
    fn a_definition_refuses_a_type_that_is_not_an_independent_source() {
        assert_eq!(
            StimulusDefinition::new("vin", ComponentType::Resistor),
            Err(StimulusDefinitionError::NotASource(ComponentType::Resistor))
        );
        assert_eq!(
            StimulusDefinition::new("vin", ComponentType::BehavioralSource),
            Err(StimulusDefinitionError::NotASource(
                ComponentType::BehavioralSource
            ))
        );
    }

    #[test]
    fn a_definition_refuses_a_name_the_netlist_reader_would_not_accept() {
        for name in ["", " ", "two words", "va=1", "\"quoted\""] {
            assert!(
                StimulusDefinition::new(name, ComponentType::VoltageSourceSin).is_err(),
                "'{name}' should not be a definition name"
            );
        }
        for name in ["vin", "sensor_diff_1k", "VDD_OPERATE", "step2"] {
            assert!(
                StimulusDefinition::new(name, ComponentType::VoltageSourceSin).is_ok(),
                "'{name}' should be a definition name"
            );
        }
    }

    #[test]
    fn a_family_switch_keeps_the_kind_and_resets_the_shape_parameters() {
        let mut pulse =
            StimulusDefinition::new("clk", ComponentType::CurrentSourcePulse).expect("definition");
        pulse.value = "0".to_owned();
        pulse.params = "i2=1m pw=1u per=2u".to_owned();

        let sin = pulse.with_family(StimulusFamily::Sin);
        assert_eq!(sin.component_type(), ComponentType::CurrentSourceSin);
        assert_eq!(sin.kind(), StimulusKind::Current);
        assert_eq!(sin.value, "");
        assert_eq!(sin.params, "");
        assert_eq!(sin.name(), "clk");
        assert_eq!(sin.revision(), pulse.revision());
    }

    /// TRRANDOM is a library family like any other: a definition can be
    /// switched into it and back out, and the round trip leaves the kind and
    /// the name where they were. It used to be the one engine source shape the
    /// library could not name at all.
    #[test]
    fn a_definition_switches_into_trrandom_and_back_out_again() {
        let mut noise =
            StimulusDefinition::new("dither", ComponentType::CurrentSourceNoise).expect("ok");
        noise.params = "na=1n nt=1u".to_owned();

        let random = noise.with_family(StimulusFamily::Trrandom);
        assert_eq!(random.component_type(), ComponentType::CurrentSourceRandom);
        assert_eq!(random.family(), StimulusFamily::Trrandom);
        assert_eq!(random.kind(), StimulusKind::Current);
        assert_eq!(random.family().label(), "TRRANDOM");
        assert_eq!(random.params, "");
        assert_eq!(random.name(), "dither");

        let voltage = random.with_kind(StimulusKind::Voltage);
        assert_eq!(voltage.component_type(), ComponentType::VoltageSourceRandom);
        assert_eq!(voltage.family(), StimulusFamily::Trrandom);

        let back = random.with_family(StimulusFamily::Trnoise);
        assert_eq!(back.component_type(), ComponentType::CurrentSourceNoise);
        assert_eq!(back.kind(), StimulusKind::Current);
    }

    #[test]
    fn a_kind_switch_keeps_the_family_and_the_card() {
        let mut sffm =
            StimulusDefinition::new("carrier", ComponentType::VoltageSourceSffm).expect("ok");
        sffm.value = "0".to_owned();
        sffm.params = "fc=1Meg fm=1k".to_owned();

        let current = sffm.with_kind(StimulusKind::Current);
        assert_eq!(current.component_type(), ComponentType::CurrentSourceSffm);
        assert_eq!(current.family(), StimulusFamily::Sffm);
        assert_eq!(current.params, "fc=1Meg fm=1k");
    }

    #[test]
    fn a_family_switch_away_from_pwl_file_drops_the_retained_table() {
        let mut definition =
            StimulusDefinition::new("bridge", ComponentType::VoltageSourcePwlFile).expect("ok");
        definition.pwl_file = Some(RetainedPwlFile::new("step.csv", "0 0\n1e-9 1\n", 17));

        assert!(
            definition
                .with_family(StimulusFamily::PwlFile)
                .pwl_file
                .is_some()
        );
        assert!(
            definition
                .with_family(StimulusFamily::Pwl)
                .pwl_file
                .is_none()
        );
    }

    #[test]
    fn normalization_ignores_parameter_order_and_revision_stamps() {
        let mut first = StimulusDefinition::new("s", ComponentType::VoltageSourceSin).expect("ok");
        first.params = "va=1 vo=0 freq=1k".to_owned();
        let mut second = first.clone();
        second.params = "freq=1k  vo=0  va=1".to_owned();
        second.publish_next_revision();

        assert_ne!(first, second);
        assert_eq!(first.normalized(), second.normalized());
    }

    #[test]
    fn a_retained_table_takes_its_digest_from_the_bytes_it_retains() {
        let retained = RetainedPwlFile::new("step.csv", "0 0\n1e-9 1\n", 42);
        assert_eq!(
            retained.digest,
            crate::state::content_digest("0 0\n1e-9 1\n")
        );
        assert_eq!(retained.imported_at_unix_ms, 42);
    }
}
