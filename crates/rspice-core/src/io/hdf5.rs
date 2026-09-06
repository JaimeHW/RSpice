//! HDF5 publication: named columns standing on one coordinate.
//!
//! This module is the one place an RSpice HDF5 file is written. The CLI's
//! `--format hdf5` and the GUI's HDF5 export both come through here, so the
//! layout below is a single definition rather than an agreement between two
//! encoders, and the GUI's own HDF5 reader can rely on it.
//!
//! # The layout
//!
//! A document is a set of root attributes and a set of groups. Attributes and
//! datasets are written in the order they were added, so two documents holding
//! the same values in the same order serialize to the same bytes.
//!
//! ## Root attributes
//!
//! | Attribute | Type | Meaning |
//! |---|---|---|
//! | `schema_version` | text | [`HDF5_SCHEMA_VERSION`] |
//! | `simulator` | text | [`HDF5_SIMULATOR`] |
//! | `title` | text | the deck title, possibly empty |
//!
//! A producer may add further root attributes after those three — the CLI adds
//! the analysis instance and run coordinate it published under. A reader that
//! does not know an attribute ignores it.
//!
//! ## A section group
//!
//! One group is one analysis section: a coordinate, and the columns standing
//! on it. Its group name is the producer's choice — the three names RSpice's
//! own reader accepts are `transient`, `dc_sweep` and `ac` — and its
//! `section_type` attribute names the analysis family regardless of what the
//! group was called.
//!
//! A **real** section (`transient`, `dc_sweep`, `operating_point`, `noise`)
//! carries:
//!
//! - attributes `section_type`, `independent_name`, `signal_count`, then, for
//!   each column `i` counting from zero, `signal_{i:04}_name`,
//!   `signal_{i:04}_type` and — only when the producer stated one —
//!   `signal_{i:04}_unit`;
//! - datasets `independent`, then `signal_{i:04}`, all `f64`.
//!
//! A **spectral** section (`ac`) carries:
//!
//! - attributes `section_type`, `signal_count`, then, for each column,
//!   `signal_{i:04}_name` and — only when stated — `signal_{i:04}_unit`;
//! - datasets `frequency`, then `signal_{i:04}_real` and `signal_{i:04}_imag`,
//!   all `f64`.
//!
//! A complex value is therefore two real datasets rather than a compound
//! type: every HDF5 reader can open an `f64` array, and few agree on how a
//! complex one is spelled.
//!
//! `signal_count` is what a reader iterates on. It is the number of columns,
//! not the number of datasets, so a spectral section of three signals declares
//! `signal_count = 3` and holds seven datasets.
//!
//! # What is not here
//!
//! Nothing decides *which* analysis a caller publishes, or what a column is
//! called. Callers hand over names and columns of `f64`; the analysis families
//! whose sections carry more than a table — a `.DISTO` series, an `.FFT`
//! result and its metrics — are assembled by the frontend that owns those
//! semantics and handed here as ordinary groups.

use std::io::Write;

use rustyhdf5::{AttrValue, FileBuilder};
use thiserror::Error;

/// The `schema_version` every RSpice HDF5 document declares on its root.
///
/// It versions the layout above, not the simulator: a reader that understands
/// this version can read any document that declares it.
pub const HDF5_SCHEMA_VERSION: &str = "1";

/// The `simulator` root attribute, which says who wrote the file.
pub const HDF5_SIMULATOR: &str = "RSpice";

/// Why a document could not be published.
#[derive(Debug, Error)]
pub enum Hdf5Error {
    /// The HDF5 backend refused to serialize the document.
    #[error("HDF5 serialization failed: {0}")]
    Backend(#[source] rustyhdf5::Error),
    /// The serialized bytes could not be handed to the writer.
    #[error("could not write the HDF5 document: {0}")]
    Write(#[source] std::io::Error),
    /// A column does not stand on the section's coordinate. Padding or
    /// truncating it would publish numbers the run never produced.
    #[error(
        "the '{section}' section's column '{column}' has {actual} values against \
         {expected} coordinate samples"
    )]
    RaggedColumn {
        /// The section group the column belongs to.
        section: String,
        /// The column's name.
        column: String,
        /// How many values the column carries.
        actual: usize,
        /// How many the coordinate has.
        expected: usize,
    },
    /// A column of the wrong kind for the section's coordinate. A real
    /// coordinate spells its columns as one dataset each and a frequency
    /// coordinate spells them as a real/imaginary pair, so the two cannot
    /// share a section.
    #[error(
        "the '{section}' section stands on a {coordinate} coordinate, which cannot \
         carry the {column} column '{name}'"
    )]
    ColumnKind {
        /// The section group the column was offered to.
        section: String,
        /// The coordinate kind the section stands on.
        coordinate: &'static str,
        /// The column kind that was offered.
        column: &'static str,
        /// The column's name.
        name: String,
    },
}

/// One scalar attribute value.
///
/// HDF5 carries more kinds than these; an RSpice document carries these.
#[derive(Debug, Clone, PartialEq)]
pub enum Hdf5Attribute {
    /// A string attribute.
    Text(String),
    /// A signed integer attribute.
    Integer(i64),
    /// A double-precision attribute.
    Real(f64),
}

/// The numbers under one dataset name.
#[derive(Debug, Clone, PartialEq)]
pub enum Hdf5Values {
    /// A `f64` array.
    Real(Vec<f64>),
    /// An `i64` array.
    Integer(Vec<i64>),
}

/// One group of a document: its attributes and its datasets, each in the order
/// they will be written.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hdf5Group {
    /// The group's name inside the file's root.
    pub name: String,
    /// Attributes, in publication order.
    pub attributes: Vec<(String, Hdf5Attribute)>,
    /// Datasets, in publication order.
    pub datasets: Vec<(String, Hdf5Values)>,
}

impl Hdf5Group {
    /// An empty group under `name`.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: Vec::new(),
            datasets: Vec::new(),
        }
    }

    /// Append one attribute.
    pub fn set_attr(&mut self, name: &str, value: Hdf5Attribute) {
        self.attributes.push((name.to_owned(), value));
    }

    /// Append one `f64` dataset.
    pub fn set_real_dataset(&mut self, name: &str, values: &[f64]) {
        self.datasets
            .push((name.to_owned(), Hdf5Values::Real(values.to_vec())));
    }

    /// Append one `i64` dataset.
    pub fn set_integer_dataset(&mut self, name: &str, values: &[i64]) {
        self.datasets
            .push((name.to_owned(), Hdf5Values::Integer(values.to_vec())));
    }
}

/// A whole HDF5 document: root attributes, then groups.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hdf5Document {
    /// Root attributes, in publication order.
    pub attributes: Vec<(String, Hdf5Attribute)>,
    /// Groups, in publication order.
    pub groups: Vec<Hdf5Group>,
}

impl Hdf5Document {
    /// A document carrying the three root attributes every RSpice reader
    /// expects: the schema version, the simulator, and this title.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            attributes: vec![
                (
                    "schema_version".to_owned(),
                    Hdf5Attribute::Text(HDF5_SCHEMA_VERSION.to_owned()),
                ),
                (
                    "simulator".to_owned(),
                    Hdf5Attribute::Text(HDF5_SIMULATOR.to_owned()),
                ),
                ("title".to_owned(), Hdf5Attribute::Text(title.into())),
            ],
            groups: Vec::new(),
        }
    }

    /// Append one root attribute.
    pub fn set_attr(&mut self, name: &str, value: Hdf5Attribute) {
        self.attributes.push((name.to_owned(), value));
    }

    /// Append one analysis section, in the layout this module documents.
    ///
    /// # Errors
    ///
    /// Refuses a column that does not stand on the section's coordinate, and a
    /// column whose kind the coordinate cannot carry.
    pub fn add_table(&mut self, table: &Hdf5Table) -> Result<(), Hdf5Error> {
        self.groups.push(table.to_group()?);
        Ok(())
    }
}

/// The coordinate a section's columns stand on.
#[derive(Debug, Clone, PartialEq)]
pub enum Hdf5Coordinate {
    /// A real coordinate. It is written as the dataset `independent`, and its
    /// name travels in the `independent_name` attribute because a dataset name
    /// has to be path-safe and a SPICE sweep parameter is not.
    Independent {
        /// What the coordinate is called: `time`, `v-sweep`, a parameter name.
        name: String,
        /// The samples.
        values: Vec<f64>,
    },
    /// The frequency axis of a spectral section, written as the dataset
    /// `frequency`. It states no `independent_name`: the dataset's own name is
    /// the name, and every RSpice-written spectral section uses it.
    Frequency(Vec<f64>),
}

impl Hdf5Coordinate {
    fn values(&self) -> &[f64] {
        match self {
            Self::Independent { values, .. } => values,
            Self::Frequency(values) => values,
        }
    }

    const fn kind(&self) -> &'static str {
        match self {
            Self::Independent { .. } => "real",
            Self::Frequency(_) => "frequency",
        }
    }
}

/// One column of a section.
#[derive(Debug, Clone, PartialEq)]
pub enum Hdf5Column {
    /// A real column: one dataset of `f64`.
    Real {
        /// The signal's name, as the producer spells it.
        name: String,
        /// What kind of quantity it is — `voltage`, `current`, `value`. It is
        /// published as `signal_{i:04}_type` and is not a unit.
        quantity: String,
        /// The engineering unit, when the producer states one. `None` means
        /// unstated, not dimensionless, and no attribute is written.
        unit: Option<String>,
        /// The samples.
        values: Vec<f64>,
    },
    /// A complex column: a real dataset and an imaginary dataset, each `f64`.
    Complex {
        /// The signal's name.
        name: String,
        /// The engineering unit, when stated.
        unit: Option<String>,
        /// The real components.
        real: Vec<f64>,
        /// The imaginary components.
        imag: Vec<f64>,
    },
}

impl Hdf5Column {
    fn name(&self) -> &str {
        match self {
            Self::Real { name, .. } | Self::Complex { name, .. } => name,
        }
    }

    const fn kind(&self) -> &'static str {
        match self {
            Self::Real { .. } => "real",
            Self::Complex { .. } => "complex",
        }
    }
}

/// One analysis section: a coordinate and the columns that stand on it.
#[derive(Debug, Clone, PartialEq)]
pub struct Hdf5Table {
    /// The group name the section is published under. RSpice's own reader
    /// accepts `transient`, `dc_sweep` and `ac`.
    pub group: String,
    /// The `section_type` attribute: which analysis family this group holds.
    /// It is stated separately from the group name so a document that names
    /// its group after an analysis instance still says what the group is.
    pub section_type: String,
    /// The coordinate every column stands on.
    pub coordinate: Hdf5Coordinate,
    /// The columns, in publication order.
    pub columns: Vec<Hdf5Column>,
}

impl Hdf5Table {
    fn to_group(&self) -> Result<Hdf5Group, Hdf5Error> {
        let coordinate = self.coordinate.values();
        let mut group = Hdf5Group::new(&self.group);
        group.set_attr(
            "section_type",
            Hdf5Attribute::Text(self.section_type.clone()),
        );
        if let Hdf5Coordinate::Independent { name, .. } = &self.coordinate {
            group.set_attr("independent_name", Hdf5Attribute::Text(name.clone()));
        }
        group.set_attr(
            "signal_count",
            Hdf5Attribute::Integer(self.columns.len() as i64),
        );
        match &self.coordinate {
            Hdf5Coordinate::Independent { .. } => group.set_real_dataset("independent", coordinate),
            Hdf5Coordinate::Frequency(_) => group.set_real_dataset("frequency", coordinate),
        }

        // Attributes and datasets are two ordered lists, so naming a column
        // and holding its numbers can be done in one pass over the columns
        // without the file's layout depending on the interleaving.
        for (index, column) in self.columns.iter().enumerate() {
            let prefix = format!("signal_{index:04}");
            self.check(column, coordinate.len())?;
            match column {
                Hdf5Column::Real {
                    name,
                    quantity,
                    unit,
                    values,
                } => {
                    group.set_attr(&format!("{prefix}_name"), Hdf5Attribute::Text(name.clone()));
                    group.set_attr(
                        &format!("{prefix}_type"),
                        Hdf5Attribute::Text(quantity.clone()),
                    );
                    if let Some(unit) = unit {
                        group
                            .set_attr(&format!("{prefix}_unit"), Hdf5Attribute::Text(unit.clone()));
                    }
                    group.set_real_dataset(&prefix, values);
                }
                Hdf5Column::Complex {
                    name,
                    unit,
                    real,
                    imag,
                } => {
                    group.set_attr(&format!("{prefix}_name"), Hdf5Attribute::Text(name.clone()));
                    if let Some(unit) = unit {
                        group
                            .set_attr(&format!("{prefix}_unit"), Hdf5Attribute::Text(unit.clone()));
                    }
                    group.set_real_dataset(&format!("{prefix}_real"), real);
                    group.set_real_dataset(&format!("{prefix}_imag"), imag);
                }
            }
        }
        Ok(group)
    }

    fn check(&self, column: &Hdf5Column, expected: usize) -> Result<(), Hdf5Error> {
        let ragged = |actual: usize| Hdf5Error::RaggedColumn {
            section: self.group.clone(),
            column: column.name().to_owned(),
            actual,
            expected,
        };
        match (&self.coordinate, column) {
            (Hdf5Coordinate::Independent { .. }, Hdf5Column::Real { values, .. }) => {
                if values.len() != expected {
                    return Err(ragged(values.len()));
                }
            }
            (Hdf5Coordinate::Frequency(_), Hdf5Column::Complex { real, imag, .. }) => {
                if real.len() != expected {
                    return Err(ragged(real.len()));
                }
                if imag.len() != expected {
                    return Err(ragged(imag.len()));
                }
            }
            _ => {
                return Err(Hdf5Error::ColumnKind {
                    section: self.group.clone(),
                    coordinate: self.coordinate.kind(),
                    column: column.kind(),
                    name: column.name().to_owned(),
                });
            }
        }
        Ok(())
    }
}

/// Serialize a document and hand the bytes to `writer`.
///
/// The whole file is built in memory before a byte reaches the writer, so a
/// caller staging an artifact never commits a partial one: a serialization
/// failure happens before the first write.
///
/// # Errors
///
/// Returns [`Hdf5Error::Backend`] when the HDF5 encoder refuses the document
/// and [`Hdf5Error::Write`] when the writer does.
pub fn write_hdf5(writer: impl Write, document: &Hdf5Document) -> Result<(), Hdf5Error> {
    let mut writer = writer;
    let mut builder = FileBuilder::new();
    for (name, value) in &document.attributes {
        builder.set_attr(name, attribute(value));
    }
    for group in &document.groups {
        let mut built = builder.create_group(&group.name);
        for (name, value) in &group.attributes {
            built.set_attr(name, attribute(value));
        }
        for (name, values) in &group.datasets {
            match values {
                Hdf5Values::Real(values) => {
                    built.create_dataset(name).with_f64_data(values);
                }
                Hdf5Values::Integer(values) => {
                    built.create_dataset(name).with_i64_data(values);
                }
            }
        }
        builder.add_group(built.finish());
    }
    let bytes = builder.finish().map_err(Hdf5Error::Backend)?;
    writer.write_all(&bytes).map_err(Hdf5Error::Write)
}

fn attribute(value: &Hdf5Attribute) -> AttrValue {
    match value {
        Hdf5Attribute::Text(value) => AttrValue::String(value.clone()),
        Hdf5Attribute::Integer(value) => AttrValue::I64(*value),
        Hdf5Attribute::Real(value) => AttrValue::F64(*value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transient() -> Hdf5Table {
        Hdf5Table {
            group: "transient".to_owned(),
            section_type: "transient".to_owned(),
            coordinate: Hdf5Coordinate::Independent {
                name: "time".to_owned(),
                values: vec![0.0, 1.0, 2.0],
            },
            columns: vec![Hdf5Column::Real {
                name: "V(out)".to_owned(),
                quantity: "voltage".to_owned(),
                unit: Some("V".to_owned()),
                values: vec![10.0, 11.0, 12.0],
            }],
        }
    }

    fn spectral() -> Hdf5Table {
        Hdf5Table {
            group: "ac".to_owned(),
            section_type: "ac".to_owned(),
            coordinate: Hdf5Coordinate::Frequency(vec![1.0, 10.0]),
            columns: vec![Hdf5Column::Complex {
                name: "V(out)".to_owned(),
                unit: None,
                real: vec![1.0, 0.5],
                imag: vec![0.0, -0.5],
            }],
        }
    }

    fn names(pairs: &[(String, impl Sized)]) -> Vec<&str> {
        pairs.iter().map(|(name, _)| name.as_str()).collect()
    }

    #[test]
    fn a_real_section_spells_the_layout_this_module_documents() {
        let mut document = Hdf5Document::new("a title");
        document.add_table(&transient()).expect("a shared axis");
        assert_eq!(
            names(&document.attributes),
            ["schema_version", "simulator", "title"]
        );
        let group = &document.groups[0];
        assert_eq!(group.name, "transient");
        assert_eq!(
            names(&group.attributes),
            [
                "section_type",
                "independent_name",
                "signal_count",
                "signal_0000_name",
                "signal_0000_type",
                "signal_0000_unit",
            ]
        );
        assert_eq!(names(&group.datasets), ["independent", "signal_0000"]);
        assert_eq!(
            group.attributes[2].1,
            Hdf5Attribute::Integer(1),
            "signal_count is the column count"
        );
    }

    #[test]
    fn a_spectral_section_names_no_independent_and_pairs_its_datasets() {
        let mut document = Hdf5Document::new(String::new());
        document.add_table(&spectral()).expect("a shared axis");
        let group = &document.groups[0];
        assert_eq!(
            names(&group.attributes),
            ["section_type", "signal_count", "signal_0000_name"],
            "no independent_name, and no unit was stated"
        );
        assert_eq!(
            names(&group.datasets),
            ["frequency", "signal_0000_real", "signal_0000_imag"]
        );
    }

    #[test]
    fn an_unstated_unit_writes_no_attribute_at_all() {
        let mut table = transient();
        let Hdf5Column::Real { unit, .. } = &mut table.columns[0] else {
            unreachable!("the fixture's column is real")
        };
        *unit = None;
        let mut document = Hdf5Document::new(String::new());
        document.add_table(&table).expect("a shared axis");
        assert_eq!(
            names(&document.groups[0].attributes),
            [
                "section_type",
                "independent_name",
                "signal_count",
                "signal_0000_name",
                "signal_0000_type",
            ]
        );
    }

    #[test]
    fn a_column_off_the_coordinate_is_refused_rather_than_padded() {
        let mut table = transient();
        let Hdf5Column::Real { values, .. } = &mut table.columns[0] else {
            unreachable!("the fixture's column is real")
        };
        values.pop();
        let error = Hdf5Document::new(String::new())
            .add_table(&table)
            .expect_err("two samples do not stand on three");
        assert!(
            matches!(
                error,
                Hdf5Error::RaggedColumn {
                    actual: 2,
                    expected: 3,
                    ..
                }
            ),
            "{error}"
        );
    }

    #[test]
    fn a_coordinate_cannot_carry_the_other_kind_of_column() {
        let table = Hdf5Table {
            coordinate: Hdf5Coordinate::Frequency(vec![1.0, 10.0, 100.0]),
            ..transient()
        };
        let error = Hdf5Document::new(String::new())
            .add_table(&table)
            .expect_err("a frequency section spells its columns as pairs");
        assert!(
            matches!(
                error,
                Hdf5Error::ColumnKind {
                    coordinate: "frequency",
                    column: "real",
                    ..
                }
            ),
            "{error}"
        );
    }

    #[test]
    fn the_same_document_serializes_to_the_same_bytes_twice() {
        let mut document = Hdf5Document::new("stable");
        document.add_table(&transient()).expect("a shared axis");
        document.add_table(&spectral()).expect("a shared axis");
        let mut first = Vec::new();
        let mut second = Vec::new();
        write_hdf5(&mut first, &document).expect("serializes");
        write_hdf5(&mut second, &document).expect("serializes");
        assert_eq!(first, second);
        assert_eq!(&first[..8], b"\x89HDF\r\n\x1a\n", "the HDF5 signature");
    }
}
