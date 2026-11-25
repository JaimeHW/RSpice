//! Source attribution for external datasets, independent of prepared solver receipts.

pub mod waveforms;

/// Every import identifier declared by the neutral result-data contract.
///
/// Being present here means the format can be identified and governed. It
/// does not mean an adapter is available; `parse_result_dataset` fails closed
/// for identified formats without a lossless implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ResultImportFormat {
    #[serde(rename = "rspice-result-bundle")]
    RSpiceResultBundle,
    #[serde(rename = "rspice-dataset-bundle")]
    RSpiceDatasetBundle,
    #[serde(rename = "csv-rfc4180")]
    CsvRfc4180,
    #[serde(rename = "tsv")]
    Tsv,
    #[serde(rename = "touchstone-v1")]
    TouchstoneV1,
    #[serde(rename = "touchstone-v2")]
    TouchstoneV2,
    #[serde(rename = "hdf5")]
    Hdf5,
    #[serde(rename = "arrow-ipc")]
    ArrowIpc,
    #[serde(rename = "parquet")]
    Parquet,
    #[serde(rename = "numpy-npy")]
    NumpyNpy,
    #[serde(rename = "numpy-npz")]
    NumpyNpz,
    #[serde(rename = "matlab-v4")]
    MatlabV4,
    #[serde(rename = "matlab-v5")]
    MatlabV5,
    #[serde(rename = "matlab-v7.3")]
    MatlabV73,
    #[serde(rename = "spice-raw")]
    SpiceRaw,
    #[serde(rename = "qspice-raw")]
    QspiceRaw,
    #[serde(rename = "hspice")]
    Hspice,
    #[serde(rename = "hspice-measure")]
    HspiceMeasure,
    #[serde(rename = "psf-ascii")]
    PsfAscii,
    #[serde(rename = "vcd")]
    Vcd,
    #[serde(rename = "fst")]
    Fst,
}

impl ResultImportFormat {
    pub const ALL: [Self; 21] = [
        Self::RSpiceResultBundle,
        Self::RSpiceDatasetBundle,
        Self::CsvRfc4180,
        Self::Tsv,
        Self::TouchstoneV1,
        Self::TouchstoneV2,
        Self::Hdf5,
        Self::ArrowIpc,
        Self::Parquet,
        Self::NumpyNpy,
        Self::NumpyNpz,
        Self::MatlabV4,
        Self::MatlabV5,
        Self::MatlabV73,
        Self::SpiceRaw,
        Self::QspiceRaw,
        Self::Hspice,
        Self::HspiceMeasure,
        Self::PsfAscii,
        Self::Vcd,
        Self::Fst,
    ];

    pub const fn canonical_id(self) -> &'static str {
        match self {
            Self::RSpiceResultBundle => "rspice-result-bundle",
            Self::RSpiceDatasetBundle => "rspice-dataset-bundle",
            Self::CsvRfc4180 => "csv-rfc4180",
            Self::Tsv => "tsv",
            Self::TouchstoneV1 => "touchstone-v1",
            Self::TouchstoneV2 => "touchstone-v2",
            Self::Hdf5 => "hdf5",
            Self::ArrowIpc => "arrow-ipc",
            Self::Parquet => "parquet",
            Self::NumpyNpy => "numpy-npy",
            Self::NumpyNpz => "numpy-npz",
            Self::MatlabV4 => "matlab-v4",
            Self::MatlabV5 => "matlab-v5",
            Self::MatlabV73 => "matlab-v7.3",
            Self::SpiceRaw => "spice-raw",
            Self::QspiceRaw => "qspice-raw",
            Self::Hspice => "hspice",
            Self::HspiceMeasure => "hspice-measure",
            Self::PsfAscii => "psf-ascii",
            Self::Vcd => "vcd",
            Self::Fst => "fst",
        }
    }

    /// The format as a reader names it, for messages about one of its files.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RSpiceResultBundle => "RSpice result bundle",
            Self::RSpiceDatasetBundle => "RSpice dataset bundle",
            Self::CsvRfc4180 => "CSV",
            Self::Tsv => "TSV",
            Self::TouchstoneV1 => "Touchstone v1",
            Self::TouchstoneV2 => "Touchstone v2",
            Self::Hdf5 => "HDF5",
            Self::ArrowIpc => "Arrow IPC",
            Self::Parquet => "Parquet",
            Self::NumpyNpy => "NumPy .npy",
            Self::NumpyNpz => "NumPy .npz",
            Self::MatlabV4 => "MATLAB v4",
            Self::MatlabV5 => "MATLAB v5",
            Self::MatlabV73 => "MATLAB v7.3",
            Self::SpiceRaw => "SPICE rawfile",
            Self::QspiceRaw => "QSPICE .qraw",
            Self::Hspice => "HSPICE waveform",
            Self::HspiceMeasure => "HSPICE measure",
            Self::PsfAscii => "Spectre PSF",
            Self::Vcd => "VCD",
            Self::Fst => "FST",
        }
    }

    pub fn from_canonical_id(id: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|format| format.canonical_id() == id)
    }
}

/// The file name and format actually accepted by the import adapter.
/// This does not authenticate a foreign solver, execution time or source deck.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultImportSource {
    pub source_name: String,
    pub format: ResultImportFormat,
    /// The admitted coordinate, including explicit absence of a unit. Older
    /// projects omitted this field and keep their historical interpretation.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_coordinate"
    )]
    pub coordinate: Option<ResultImportCoordinate>,
    /// The `.step` run the source says this analysis is, when one file held
    /// every run of a sweep (an LTspice `stepped` rawfile). The runs of one
    /// import then draw as one family, as a deck's `.step` members do.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_member: Option<crate::provenance::AnalysisResultFamilyMember>,
    /// Where and in what order an imported dump declared its variables, so a dump written from
    /// the import declares them as the source did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_dump: Option<ImportedEventDump>,
}

/// The scope a Value Change Dump declared its variables under and the order it declared them in.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportedEventDump {
    /// The scope path every variable shared, outermost first; empty when the source named none.
    pub scope: Vec<String>,
    /// Each variable in declaration order, by the bus or node name its event history has.
    pub order: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultImportCoordinate {
    pub name: String,
    pub unit: Option<String>,
}

fn deserialize_coordinate<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<ResultImportCoordinate>, D::Error> {
    // Present null must not masquerade as absence in an older schema.
    <Option<ResultImportCoordinate> as serde::Deserialize>::deserialize(deserializer)?
        .map(Some)
        .ok_or_else(|| serde::de::Error::custom("import coordinate cannot be null"))
}

impl ResultImportCoordinate {
    pub fn validate(&self) -> Result<(), String> {
        for (field, value) in [
            ("name", Some(self.name.as_str())),
            ("unit", self.unit.as_deref()),
        ] {
            if value
                .is_some_and(|value| value.trim().is_empty() || value.chars().any(char::is_control))
            {
                return Err(format!(
                    "import coordinate {field} must be non-empty and control-free"
                ));
            }
        }
        Ok(())
    }

    pub fn validate_domain(
        &self,
        analysis: crate::analysis_type::AnalysisType,
    ) -> Result<(), String> {
        self.validate()?;
        let required = if analysis.is_time_domain() {
            Some("s")
        } else if matches!(
            analysis,
            crate::analysis_type::AnalysisType::Ac | crate::analysis_type::AnalysisType::SParameter
        ) {
            Some("Hz")
        } else {
            None
        };
        if let (Some(required), Some(unit)) = (required, self.unit.as_deref())
            && unit != required
        {
            return Err(format!(
                "import coordinate unit '{unit}' is incompatible with {analysis}; expected '{required}'"
            ));
        }
        Ok(())
    }
}

impl ResultImportSource {
    pub fn validate(&self) -> Result<(), String> {
        if self.source_name.trim().is_empty() || self.source_name.chars().any(char::is_control) {
            return Err("import source requires a non-empty control-free file name".to_owned());
        }
        if let Some(coordinate) = &self.coordinate {
            coordinate.validate()?;
        }
        if let Some(member) = &self.family_member {
            member.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_source_format_wire_names_match_the_import_contract() {
        for format in ResultImportFormat::ALL {
            let text = serde_json::to_string(&format).unwrap();
            assert_eq!(text, format!("\"{}\"", format.canonical_id()));
            assert_eq!(
                serde_json::from_str::<ResultImportFormat>(&text).unwrap(),
                format
            );
        }
    }
}
