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
//! A general **table** section uses that real-section layout and adds group
//! attributes `analysis` and `coordinate_type`. The latter preserves arbitrary
//! coordinates such as a report's `index`, instead of inferring time from a
//! column name. A complex column occupies two adjacent real columns named
//! `Re(name)` and `Im(name)`, with types `complex_real:<quantity>` and
//! `complex_imag:<quantity>`. Both halves must agree on name and quantity.
//! Waveform readers may admit only the coordinate domains they can represent;
//! the CLI also reads report/index tables.
//!
//! Nullable real table quantities use two adjacent real columns: `name`
//! of type `nullable_real:<quantity>` and `Valid(name)` of type
//! `nullable_validity:<quantity>` (unit `1`). The validity label may have a
//! suffix to avoid collisions; pairing is positional. Validity is exactly 0 or 1;
//! absent values are padded with zero and must not be interpreted without the
//! validity column. This same typed pair is used in tabular RAW exports.
//!
//! A **spectral** section (`ac`) carries:
//!
//! - attributes `section_type`, `signal_count`, then, for each column,
//!   `signal_{i:04}_name` and — only when stated — `signal_{i:04}_unit`;
//! - datasets `frequency`, then `signal_{i:04}_real` and `signal_{i:04}_imag`,
//!   all `f64`.
//!
//! All section families may state `coordinate_unit` when the producer retains
//! a unit; absence means unstated. A spectral section may also state
//! `independent_name` to preserve an authored coordinate label. Its physical
//! dataset remains `frequency`, which is also the default label for older
//! files without that attribute.
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
    /// A streamed dataset's values did not arrive as the document declares them: the writer
    /// was given no source, the source wrote another count, or the encoder lays the file out
    /// otherwise than the streamed writer does.
    #[error("the HDF5 dataset '{dataset}' could not be streamed: {reason}")]
    Streamed {
        /// The dataset's name, or the file's part the layout disagrees on.
        dataset: String,
        /// Why.
        reason: &'static str,
    },
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
    /// This many `f64` values, which [`write_hdf5_streamed`]'s source writes while the file is
    /// written instead of the document holding them.
    Streamed(usize),
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

impl Hdf5Values {
    /// How many values the dataset holds.
    fn len(&self) -> usize {
        match self {
            Self::Real(values) => values.len(),
            Self::Integer(values) => values.len(),
            Self::Streamed(count) => *count,
        }
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
    /// `frequency`. A caller may add `independent_name` on the resulting group
    /// to preserve an authored label; otherwise the label is `frequency`.
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
    let bytes = encode(document, |name, _| {
        Err(Hdf5Error::Streamed {
            dataset: name.to_owned(),
            reason: "the document holds no values for it",
        })
    })?;
    writer.write_all(&bytes).map_err(Hdf5Error::Write)
}

/// The encoder's bytes for `document`, with the values `streamed` gives in place of each
/// [`Hdf5Values::Streamed`] dataset.
fn encode(
    document: &Hdf5Document,
    mut streamed: impl FnMut(&str, usize) -> Result<Vec<f64>, Hdf5Error>,
) -> Result<Vec<u8>, Hdf5Error> {
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
                Hdf5Values::Streamed(count) => {
                    built
                        .create_dataset(name)
                        .with_f64_data(&streamed(name, *count)?);
                }
            }
        }
        builder.add_group(built.finish());
    }
    builder.finish().map_err(Hdf5Error::Backend)
}

/// Why [`write_hdf5_streamed`] stopped.
#[derive(Debug, Error)]
pub enum Hdf5StreamError<E> {
    /// The document or the writer refused.
    #[error(transparent)]
    Document(#[from] Hdf5Error),
    /// The source of a streamed dataset failed.
    #[error("{0}")]
    Source(E),
}

/// [`write_hdf5`] of a document whose [`Hdf5Values::Streamed`] datasets `source` writes, the
/// `index`th of them (counting in document order) as the little-endian bytes of its `f64`
/// values, while the file is written: memory holds the file's metadata and what `source` holds,
/// never a dataset. The bytes are the ones [`write_hdf5`] writes for the same values.
///
/// Every dataset is stored contiguously after all of the file's metadata, at an address its
/// length alone fixes, so the metadata is the encoder's own for the same document with one
/// value in each streamed dataset, with each dataset's header restated for its real length.
/// The encoder's layout is checked against that reading before a byte is written.
///
/// # Errors
///
/// [`Hdf5StreamError::Source`] when `source` fails, and [`Hdf5Error::Streamed`] when it writes
/// another count of bytes than its dataset declares or the encoder lays the file out otherwise.
pub fn write_hdf5_streamed<E>(
    writer: impl Write,
    document: &Hdf5Document,
    mut source: impl FnMut(usize, &mut dyn Write) -> Result<(), E>,
) -> Result<(), Hdf5StreamError<E>> {
    let mut writer = writer;
    let skeleton = encode(document, |_, _| Ok(vec![0.0]))?;
    // Each dataset in the order the encoder lays them out: its type, its length in the file
    // and in the skeleton.
    let datasets = document
        .groups
        .iter()
        .flat_map(|group| &group.datasets)
        .map(|(name, values)| {
            let real = !matches!(values, Hdf5Values::Integer(_));
            let skeleton = if matches!(values, Hdf5Values::Streamed(_)) {
                1
            } else {
                values.len()
            };
            (name.as_str(), real, values.len() as u64, skeleton as u64)
        })
        .collect::<Vec<_>>();
    let headers = |lengths: &mut dyn Iterator<Item = (bool, u64)>, data: u64| {
        let mut address = data;
        lengths
            .map(|(real, length)| {
                let header = dataset_header(real, length, address);
                address += length * 8;
                header
            })
            .collect::<Vec<_>>()
    };
    let layout_differs = |part: &str| Hdf5Error::Streamed {
        dataset: part.to_owned(),
        reason: "the HDF5 encoder lays the file out otherwise than this writer streams it",
    };
    let header_bytes = headers(&mut datasets.iter().map(|d| (d.1, 0)), 0)
        .iter()
        .map(Vec::len)
        .sum::<usize>();
    let skeleton_data = datasets.iter().map(|d| d.3 as usize * 8).sum::<usize>();
    let metadata = skeleton
        .len()
        .checked_sub(header_bytes + skeleton_data)
        .filter(|&metadata| metadata >= SUPERBLOCK_BYTES)
        .ok_or_else(|| layout_differs("file"))?;
    let data = (metadata + header_bytes) as u64;
    let expected = headers(&mut datasets.iter().map(|d| (d.1, d.3)), data).concat();
    if skeleton[metadata..metadata + header_bytes] != expected[..]
        || skeleton[..SUPERBLOCK_BYTES] != superblock(skeleton.len() as u64)[..]
    {
        return Err(layout_differs("dataset headers").into());
    }
    let end = data + datasets.iter().map(|d| d.2 * 8).sum::<u64>();
    let write = |writer: &mut dyn Write, bytes: &[u8]| -> Result<(), Hdf5StreamError<E>> {
        writer
            .write_all(bytes)
            .map_err(|error| Hdf5Error::Write(error).into())
    };
    write(&mut writer, &superblock(end))?;
    write(&mut writer, &skeleton[SUPERBLOCK_BYTES..metadata])?;
    for header in headers(&mut datasets.iter().map(|d| (d.1, d.2)), data) {
        write(&mut writer, &header)?;
    }
    drop(skeleton);
    let mut streamed = 0;
    let mut bytes = Vec::new();
    for (name, values) in document.groups.iter().flat_map(|group| &group.datasets) {
        match values {
            Hdf5Values::Real(values) => {
                for chunk in values.chunks(8192) {
                    bytes.clear();
                    bytes.extend(chunk.iter().flat_map(|value| value.to_le_bytes()));
                    write(&mut writer, &bytes)?;
                }
            }
            Hdf5Values::Integer(values) => {
                for chunk in values.chunks(8192) {
                    bytes.clear();
                    bytes.extend(chunk.iter().flat_map(|value| value.to_le_bytes()));
                    write(&mut writer, &bytes)?;
                }
            }
            Hdf5Values::Streamed(count) => {
                let mut counted = Counted {
                    inner: &mut writer,
                    bytes: 0,
                };
                source(streamed, &mut counted).map_err(Hdf5StreamError::Source)?;
                if counted.bytes != *count as u64 * 8 {
                    return Err(Hdf5Error::Streamed {
                        dataset: name.clone(),
                        reason: "its source wrote another count of values than it declares",
                    }
                    .into());
                }
                streamed += 1;
            }
        }
    }
    Ok(())
}

/// The bytes of a v3 superblock before the root group, as the encoder writes them.
const SUPERBLOCK_BYTES: usize = 48;

/// The encoder's superblock for a file of `end` bytes whose root group follows it.
fn superblock(end: u64) -> Vec<u8> {
    rustyhdf5_format::superblock::Superblock {
        version: 3,
        offset_size: 8,
        length_size: 8,
        base_address: 0,
        eof_address: end,
        root_group_address: SUPERBLOCK_BYTES as u64,
        group_leaf_node_k: None,
        group_internal_node_k: None,
        indexed_storage_internal_node_k: None,
        free_space_address: None,
        driver_info_address: None,
        consistency_flags: 0,
        superblock_extension_address: Some(u64::MAX),
        checksum: None,
    }
    .serialize()
}

/// The encoder's object header of a contiguous one-dimensional dataset of `length` values
/// (`f64` when `real`, else `i64`) stored at `address`, with no attributes.
fn dataset_header(real: bool, length: u64, address: u64) -> Vec<u8> {
    use rustyhdf5_format::dataspace::{Dataspace, DataspaceType};
    use rustyhdf5_format::message_type::MessageType;
    use rustyhdf5_format::object_header_writer::ObjectHeaderWriter;
    use rustyhdf5_format::type_builders::{make_f64_type, make_i64_type};
    let datatype = if real {
        make_f64_type()
    } else {
        make_i64_type()
    };
    let dataspace = Dataspace {
        space_type: DataspaceType::Simple,
        rank: 1,
        dimensions: vec![length],
        max_dimensions: None,
    };
    let mut layout = vec![4, 1];
    layout.extend_from_slice(&address.to_le_bytes());
    layout.extend_from_slice(&(length * 8).to_le_bytes());
    let mut header = ObjectHeaderWriter::new();
    header.add_message_with_flags(MessageType::Datatype, datatype.serialize(), 0x01);
    header.add_message(MessageType::Dataspace, dataspace.serialize(8));
    header.add_message_with_flags(MessageType::FillValue, vec![3, 0x0a], 0x01);
    header.add_message(MessageType::DataLayout, layout);
    header.serialize()
}

/// A writer that counts the bytes it passes on.
struct Counted<'w> {
    inner: &'w mut dyn Write,
    bytes: u64,
}

impl Write for Counted<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let written = self.inner.write(buf)?;
        self.bytes += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

fn attribute(value: &Hdf5Attribute) -> AttrValue {
    match value {
        // The backend encodes String("") as a zero-width datatype, which its
        // own reader returns as an empty array. A one-byte NUL-padded scalar
        // is an empty string and is readable by ordinary HDF5 consumers.
        Hdf5Attribute::Text(value) if value.is_empty() => AttrValue::String("\0".to_owned()),
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

    #[test]
    fn a_streamed_document_is_the_whole_document_byte_for_byte() {
        // Wide enough that the group's attributes go dense, with columns of a length that
        // takes more than one buffer, an integer dataset and an empty section beside them.
        let rows = 20_000;
        let column = |seed: f64| {
            (0..rows)
                .map(|row| (row as f64 * 1.0e-3 + seed).sin() * seed)
                .collect::<Vec<_>>()
        };
        let columns = (0..12)
            .map(|index| Hdf5Column::Real {
                name: format!("V(n{index})"),
                quantity: "voltage".to_owned(),
                unit: (index % 2 == 0).then(|| "V".to_owned()),
                values: column(index as f64 + 0.5),
            })
            .collect::<Vec<_>>();
        let time = (0..rows).map(|row| row as f64 * 1.0e-9).collect::<Vec<_>>();
        let mut whole = Hdf5Document::new("streamed");
        whole.set_attr("analysis_id", Hdf5Attribute::Text("tran".to_owned()));
        whole
            .add_table(&Hdf5Table {
                group: "tran".to_owned(),
                section_type: "transient".to_owned(),
                coordinate: Hdf5Coordinate::Independent {
                    name: "time".to_owned(),
                    values: time.clone(),
                },
                columns: columns.clone(),
            })
            .expect("a shared axis");
        let mut measures = Hdf5Group::new("measurements");
        measures.set_attr("count", Hdf5Attribute::Integer(2));
        measures.set_integer_dataset("flags", &[1, 0]);
        measures.set_real_dataset("values", &[]);
        whole.groups.push(measures);
        whole.add_table(&spectral()).expect("a shared axis");
        let mut expected = Vec::new();
        write_hdf5(&mut expected, &whole).expect("serializes");

        let mut streamed = whole.clone();
        let mut sources = Vec::new();
        for (_, values) in &mut streamed.groups[0].datasets {
            let Hdf5Values::Real(held) = std::mem::replace(values, Hdf5Values::Streamed(rows))
            else {
                unreachable!("the section's datasets are real");
            };
            sources.push(held);
        }
        assert_eq!(sources.len(), 13);
        let mut actual = Vec::new();
        write_hdf5_streamed(&mut actual, &streamed, |index, out| {
            for chunk in sources[index].chunks(1_000) {
                let bytes = chunk
                    .iter()
                    .flat_map(|value| value.to_le_bytes())
                    .collect::<Vec<_>>();
                out.write_all(&bytes)?;
            }
            Ok::<(), std::io::Error>(())
        })
        .expect("streams");
        assert!(actual == expected, "the streamed file differs");

        // A source that writes another count is refused rather than published.
        let short =
            write_hdf5_streamed(&mut Vec::new(), &streamed, |_, out| out.write_all(&[0; 8]));
        assert!(matches!(
            short,
            Err(Hdf5StreamError::Document(Hdf5Error::Streamed { .. }))
        ));
        // And the whole-document writer refuses a dataset it holds no values for.
        assert!(matches!(
            write_hdf5(&mut Vec::new(), &streamed),
            Err(Hdf5Error::Streamed { .. })
        ));
    }

    #[test]
    fn empty_text_attributes_are_readable_scalar_strings() {
        let mut document = Hdf5Document::new("");
        document.add_table(&transient()).unwrap();
        document.groups[0].set_attr("optional_text", Hdf5Attribute::Text(String::new()));
        let mut bytes = Vec::new();
        write_hdf5(&mut bytes, &document).unwrap();
        let file = rustyhdf5::File::from_bytes(bytes).unwrap();
        let root = file.root().attrs().unwrap();
        let group = file.group("transient").unwrap().attrs().unwrap();
        for (attrs, name) in [(root, "title"), (group, "optional_text")] {
            assert!(
                matches!(attrs.get(name), Some(AttrValue::String(value)) if value.is_empty()),
                "{attrs:?}"
            );
        }
    }
}
