//! Mapping on-screen objects to what the printer will actually put on paper.
//!
//! A mapping entry is per object kind, so a change to how nets print cannot
//! silently change how components do. The redundancy policy exists because
//! colour alone is not a legible distinction in monochrome — an object that
//! relies on colour must also carry a pattern or weight, and the table refuses
//! an entry that would print as indistinguishable.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColorMapping {
    PrintSafeEngineeringPalette,
    ScreenColors,
    GrayscaleWithDashMarkerRedundancy,
    Monochrome,
}

/// Semantic category of an engineering object participating in the exact
/// layer/trace/marker print-mapping table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PrintObjectKind {
    Trace,
    Layer,
    Net,
    Marker,
    DrcMarker,
    ReviewAnnotation,
    Other,
}

/// Stable semantic identity and the source-owned screen style shown beside a
/// print mapping. The screen style is descriptive input, never the authority
/// for the resolved print style.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintObjectIdentity {
    kind: PrintObjectKind,
    stable_id: String,
    display_name: String,
    screen_style: String,
}

impl PrintObjectIdentity {
    pub fn try_new(
        kind: PrintObjectKind,
        stable_id: impl Into<String>,
        display_name: impl Into<String>,
        screen_style: impl Into<String>,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            kind,
            stable_id: stable_id.into(),
            display_name: display_name.into(),
            screen_style: screen_style.into(),
        };
        value.validate()?;
        Ok(value)
    }

    pub(super) fn validate(&self) -> Result<(), HardcopyError> {
        validate_text(
            "print object stable identity",
            &self.stable_id,
            MAX_TEXT_BYTES,
        )?;
        validate_text(
            "print object display name",
            &self.display_name,
            MAX_TEXT_BYTES,
        )?;
        validate_text(
            "print object screen style",
            &self.screen_style,
            MAX_TEXT_BYTES,
        )
    }

    #[must_use]
    pub const fn kind(&self) -> PrintObjectKind {
        self.kind
    }

    #[must_use]
    pub fn stable_id(&self) -> &str {
        &self.stable_id
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[must_use]
    pub fn screen_style(&self) -> &str {
        &self.screen_style
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum PrintColor {
    Black,
    /// Percentage of black ink coverage. For example, `70` renders as a dark
    /// gray with 30% reflected sRGB intensity.
    GrayPercent(u8),
    Rgb {
        red: u8,
        green: u8,
        blue: u8,
    },
}

impl PrintColor {
    pub(super) fn validate(self) -> Result<(), HardcopyError> {
        if let Self::GrayPercent(percent) = self
            && !(1..=99).contains(&percent)
        {
            return Err(HardcopyError::InvalidPrintGrayPercent(percent));
        }
        Ok(())
    }
}

/// Non-color print encoding. These physical dimensions make redundancy exact
/// in PDF, raster export, and printer backends rather than a display-pixel
/// approximation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum PrintRedundancy {
    SolidLine {
        width: Length,
    },
    DashedLine {
        width: Length,
        dash: Length,
        gap: Length,
    },
    DottedLeader {
        width: Length,
        spacing: Length,
    },
    SolidFill,
    CrossHatch {
        line_width: Length,
        spacing: Length,
    },
    TriangleWithId {
        size: Length,
    },
    SourceStyle,
}

impl PrintRedundancy {
    pub(super) fn validate(self) -> Result<(), HardcopyError> {
        match self {
            Self::SolidLine { width } => validate_print_feature("line width", width),
            Self::DashedLine { width, dash, gap } => {
                validate_print_feature("line width", width)?;
                validate_print_feature("dash length", dash)?;
                validate_print_feature("dash gap", gap)
            }
            Self::DottedLeader { width, spacing } => {
                validate_print_feature("leader width", width)?;
                validate_print_feature("dot spacing", spacing)
            }
            Self::SolidFill | Self::SourceStyle => Ok(()),
            Self::CrossHatch {
                line_width,
                spacing,
            } => {
                validate_print_feature("hatch line width", line_width)?;
                validate_print_feature("hatch spacing", spacing)
            }
            Self::TriangleWithId { size } => validate_print_feature("marker size", size),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintMappingEntry {
    object: PrintObjectIdentity,
    print_color: PrintColor,
    redundancy: PrintRedundancy,
    include_in_legend: bool,
}

impl PrintMappingEntry {
    pub fn try_new(
        object: PrintObjectIdentity,
        print_color: PrintColor,
        redundancy: PrintRedundancy,
        include_in_legend: bool,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            object,
            print_color,
            redundancy,
            include_in_legend,
        };
        value.validate()?;
        Ok(value)
    }

    pub(super) fn validate(&self) -> Result<(), HardcopyError> {
        self.object.validate()?;
        self.print_color.validate()?;
        self.redundancy.validate()
    }

    #[must_use]
    pub const fn object(&self) -> &PrintObjectIdentity {
        &self.object
    }

    #[must_use]
    pub const fn print_color(&self) -> PrintColor {
        self.print_color
    }

    #[must_use]
    pub const fn redundancy(&self) -> PrintRedundancy {
        self.redundancy
    }

    #[must_use]
    pub const fn include_in_legend(&self) -> bool {
        self.include_in_legend
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum PrintMappingSaveScope {
    Document,
    ProjectPrintSet(String),
    PortablePersonalPreset(String),
}

impl PrintMappingSaveScope {
    pub(super) fn validate(&self) -> Result<(), HardcopyError> {
        match self {
            Self::Document => Ok(()),
            Self::ProjectPrintSet(name) => {
                validate_text("project print set mapping name", name, MAX_TEXT_BYTES)
            }
            Self::PortablePersonalPreset(name) => {
                validate_text("personal print mapping preset name", name, MAX_TEXT_BYTES)
            }
        }
    }
}

/// Exact, ordered mapping table for layer, trace, net, marker, and annotation
/// output. Entries remain in source display order while duplicate semantic
/// identities are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintMappingTable {
    save_scope: PrintMappingSaveScope,
    entries: Vec<PrintMappingEntry>,
}

impl PrintMappingTable {
    pub fn try_new(
        save_scope: PrintMappingSaveScope,
        entries: Vec<PrintMappingEntry>,
    ) -> Result<Self, HardcopyError> {
        let value = Self {
            save_scope,
            entries,
        };
        value.validate()?;
        Ok(value)
    }

    pub(super) fn validate(&self) -> Result<(), HardcopyError> {
        self.save_scope.validate()?;
        if self.entries.len() > 4_096 {
            return Err(HardcopyError::TooManyPrintMappings(self.entries.len()));
        }
        let mut identities = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            let identity = (entry.object.kind, entry.object.stable_id.clone());
            if !identities.insert(identity) {
                return Err(HardcopyError::DuplicatePrintObjectIdentity {
                    kind: entry.object.kind,
                    stable_id: entry.object.stable_id.clone(),
                });
            }
        }
        Ok(())
    }

    #[must_use]
    pub const fn save_scope(&self) -> &PrintMappingSaveScope {
        &self.save_scope
    }

    #[must_use]
    pub fn entries(&self) -> &[PrintMappingEntry] {
        &self.entries
    }
}

impl Default for PrintMappingTable {
    fn default() -> Self {
        Self {
            save_scope: PrintMappingSaveScope::Document,
            entries: Vec::new(),
        }
    }
}
