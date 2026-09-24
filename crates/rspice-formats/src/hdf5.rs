//! HDF5 and MATLAB 7.3 waveform byte decoding.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Hdf5Limits<'a> {
    pub max_columns: usize,
    pub max_values: usize,
    pub coordinate_names: &'a [&'a str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hdf5SectionFamily {
    Transient,
    DcSweep,
    Ac,
}

#[derive(Debug)]
pub struct Hdf5Signal {
    pub name: String,
    pub real: Vec<f64>,
    pub imag: Option<Vec<f64>>,
    pub unit: Option<String>,
}

#[derive(Debug)]
pub enum DecodedHdf5 {
    Section {
        family: Hdf5SectionFamily,
        coordinate_name: String,
        coordinate: Vec<f64>,
        signals: Vec<Hdf5Signal>,
    },
    Root {
        coordinate_name: String,
        coordinate: Vec<f64>,
        columns: Vec<(String, Vec<f64>)>,
    },
}

fn adapter_error(format: &str, detail: impl std::fmt::Display) -> String {
    format!("{format} import: {detail}")
}

pub fn decode_hdf5(
    bytes: &[u8],
    limits: Hdf5Limits<'_>,
    format: &str,
) -> Result<DecodedHdf5, String> {
    let file = rustyhdf5::File::from_bytes(bytes.to_vec())
        .map_err(|error| adapter_error(format, format_args!("invalid HDF5 container: {error}")))?;
    let root = file.root();
    let groups = root.groups().map_err(|error| {
        adapter_error(format, format_args!("could not enumerate groups: {error}"))
    })?;
    let supported = groups
        .iter()
        .filter_map(|name| hdf5_section_family(&file, name).map(|family| (name.clone(), family)))
        .collect::<Vec<_>>();
    if supported.len() > 1 {
        return Err(adapter_error(
            format,
            format_args!(
                "the file contains multiple waveform sections ({}); import one analysis per file",
                supported
                    .iter()
                    .map(|(name, _)| name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ));
    }
    if let Some((section, family)) = supported.first() {
        return parse_rspice_hdf5_section(&file, section, *family, format, limits);
    }
    parse_generic_hdf5_root(&file, format, limits)
}

/// The analysis family one root group holds, or `None` when this reader has no
/// section shape for it.
///
/// The layout contract in `rspice_core::io::hdf5` is explicit that a section
/// group's *name* is the producer's choice and its `section_type` attribute is
/// what names the family. The command line names its one section group after
/// the analysis instance that produced it — `tran1`, not `transient` — so a
/// reader keyed on the name alone read every identity-named file as an
/// anonymous root, fell through to [`parse_generic_hdf5_root`], and refused a
/// file this product had just written. The name is still consulted, because a
/// file written before the attribute existed carries nothing else.
///
/// `operating_point`, `noise`, `distortion` and `fft` are families the layout
/// defines and this reader has no domain for; they return `None` and are
/// refused by name at the root, rather than imported under a heading that
/// would make the result something it is not.
fn hdf5_section_family(file: &rustyhdf5::File, group: &str) -> Option<Hdf5SectionFamily> {
    let declared = file
        .group(group)
        .ok()
        .and_then(|opened| opened.attrs().ok())
        .and_then(|attrs| hdf_optional_string_attr(&attrs, "section_type"));
    match declared.as_deref().unwrap_or(group) {
        "transient" => Some(Hdf5SectionFamily::Transient),
        "dc_sweep" => Some(Hdf5SectionFamily::DcSweep),
        "ac" => Some(Hdf5SectionFamily::Ac),
        _ => None,
    }
}

pub fn decode_matlab_v73(
    bytes: &[u8],
    limits: Hdf5Limits<'_>,
    format: &str,
) -> Result<DecodedHdf5, String> {
    let file = rustyhdf5::File::from_bytes(bytes.to_vec()).map_err(|error| {
        adapter_error(
            format,
            format_args!("invalid MATLAB 7.3/HDF5 container: {error}"),
        )
    })?;
    parse_generic_hdf5_root(&file, format, limits)
}

fn parse_rspice_hdf5_section(
    file: &rustyhdf5::File,
    section: &str,
    family: Hdf5SectionFamily,
    format: &str,
    limits: Hdf5Limits<'_>,
) -> Result<DecodedHdf5, String> {
    let group = file.group(section).map_err(|error| {
        adapter_error(format, format_args!("could not open /{section}: {error}"))
    })?;
    let attrs = group.attrs().map_err(|error| {
        adapter_error(
            format,
            format_args!("could not read /{section} attributes: {error}"),
        )
    })?;
    let signal_count = hdf_i64_attr(&attrs, "signal_count", format)?;
    let signal_count = usize::try_from(signal_count).map_err(|_| {
        adapter_error(
            format,
            format_args!("/{section} signal_count is negative or too large"),
        )
    })?;
    if signal_count == 0 || signal_count.saturating_add(1) > limits.max_columns {
        return Err(adapter_error(
            format,
            format_args!("/{section} declares invalid signal_count {signal_count}"),
        ));
    }
    if family == Hdf5SectionFamily::Ac {
        let coordinate = hdf_f64_dataset(&group, "frequency", format, limits)?;
        ensure_table_value_limit(
            format,
            coordinate.len(),
            1 + signal_count.saturating_mul(2),
            limits,
        )?;
        let mut signals = Vec::with_capacity(signal_count);
        for index in 0..signal_count {
            let prefix = format!("signal_{index:04}");
            let name = hdf_string_attr(&attrs, &format!("{prefix}_name"), format)?;
            signals.push(Hdf5Signal {
                name,
                real: hdf_f64_dataset(&group, &format!("{prefix}_real"), format, limits)?,
                imag: Some(hdf_f64_dataset(
                    &group,
                    &format!("{prefix}_imag"),
                    format,
                    limits,
                )?),
                unit: hdf_stated_unit(&attrs, &format!("{prefix}_unit")),
            });
        }
        return Ok(DecodedHdf5::Section {
            family,
            coordinate_name: "frequency".to_owned(),
            coordinate,
            signals,
        });
    }

    let coordinate_name = hdf_string_attr(&attrs, "independent_name", format)?;
    let coordinate = hdf_f64_dataset(&group, "independent", format, limits)?;
    ensure_table_value_limit(format, coordinate.len(), 1 + signal_count, limits)?;
    let mut signals = Vec::with_capacity(signal_count);
    for index in 0..signal_count {
        let prefix = format!("signal_{index:04}");
        signals.push(Hdf5Signal {
            name: hdf_string_attr(&attrs, &format!("{prefix}_name"), format)?,
            real: hdf_f64_dataset(&group, &prefix, format, limits)?,
            imag: None,
            unit: hdf_stated_unit(&attrs, &format!("{prefix}_unit")),
        });
    }
    Ok(DecodedHdf5::Section {
        family,
        coordinate_name,
        coordinate,
        signals,
    })
}

fn parse_generic_hdf5_root(
    file: &rustyhdf5::File,
    format: &str,
    limits: Hdf5Limits<'_>,
) -> Result<DecodedHdf5, String> {
    let root = file.root();
    let names = root.datasets().map_err(|error| {
        adapter_error(
            format,
            format_args!("could not enumerate datasets: {error}"),
        )
    })?;
    if names.len() > limits.max_columns.saturating_mul(2) {
        return Err(adapter_error(format, "too many root datasets"));
    }
    let coordinate_name =
        select_coordinate_name(names.iter().map(String::as_str), limits.coordinate_names)
            .ok_or_else(|| {
                adapter_error(
                    format,
                    format_args!(
                        "no unambiguous root coordinate dataset named {}",
                        stated_coordinate_names(limits.coordinate_names)
                    ),
                )
            })?;
    let coordinate = hdf_f64_dataset(&root, &coordinate_name, format, limits)?;
    ensure_table_value_limit(format, coordinate.len(), names.len(), limits)?;
    let mut columns = Vec::new();
    for name in names {
        if name.eq_ignore_ascii_case(&coordinate_name) || name.starts_with('#') {
            continue;
        }
        let dataset = root.dataset(&name).map_err(|error| {
            adapter_error(
                format,
                format_args!("could not open dataset '{name}': {error}"),
            )
        })?;
        let shape = dataset.shape().map_err(|error| {
            adapter_error(
                format,
                format_args!("could not inspect dataset '{name}': {error}"),
            )
        })?;
        let count = shape
            .iter()
            .try_fold(1_u64, |count, dim| count.checked_mul(*dim))
            .ok_or_else(|| {
                adapter_error(format, format_args!("dataset '{name}' shape overflows"))
            })?;
        if count != coordinate.len() as u64 {
            return Err(adapter_error(
                format,
                format_args!(
                    "dataset '{name}' has {count} values; coordinate '{coordinate_name}' has {}",
                    coordinate.len()
                ),
            ));
        }
        let values = hdf_dataset_values(&dataset, &name, format, limits)?;
        columns.push((name, values));
    }
    Ok(DecodedHdf5::Root {
        coordinate_name,
        coordinate,
        columns,
    })
}

fn hdf_f64_dataset(
    group: &rustyhdf5::Group<'_>,
    name: &str,
    format: &str,
    limits: Hdf5Limits<'_>,
) -> Result<Vec<f64>, String> {
    let dataset = group.dataset(name).map_err(|error| {
        adapter_error(
            format,
            format_args!("could not open numeric dataset '{name}': {error}"),
        )
    })?;
    hdf_dataset_values(&dataset, name, format, limits)
}

fn hdf_dataset_values(
    dataset: &rustyhdf5::Dataset<'_>,
    name: &str,
    format: &str,
    limits: Hdf5Limits<'_>,
) -> Result<Vec<f64>, String> {
    let count = dataset
        .shape()
        .map_err(|error| {
            adapter_error(
                format,
                format_args!("could not inspect dataset '{name}' shape: {error}"),
            )
        })?
        .into_iter()
        .try_fold(1_u64, |count, dimension| count.checked_mul(dimension))
        .ok_or_else(|| adapter_error(format, format_args!("dataset '{name}' shape overflows")))?;
    if count > limits.max_values as u64 {
        return Err(adapter_error(
            format,
            format_args!(
                "dataset '{name}' contains {count} values; the limit is {}",
                limits.max_values
            ),
        ));
    }
    let dtype = dataset.dtype().map_err(|error| {
        adapter_error(
            format,
            format_args!("could not inspect dataset '{name}': {error}"),
        )
    })?;
    match dtype {
        rustyhdf5::DType::I64 => dataset
            .read_i64()
            .map_err(|error| {
                adapter_error(format, format_args!("could not read '{name}': {error}"))
            })?
            .into_iter()
            .map(|value| {
                crate::numeric::exact_signed_integer(name, value)
                    .map_err(|detail| adapter_error(format, detail))
            })
            .collect(),
        rustyhdf5::DType::U64 => dataset
            .read_u64()
            .map_err(|error| {
                adapter_error(format, format_args!("could not read '{name}': {error}"))
            })?
            .into_iter()
            .map(|value| {
                crate::numeric::exact_unsigned_integer(name, value)
                    .map_err(|detail| adapter_error(format, detail))
            })
            .collect(),
        _ => dataset.read_f64().map_err(|error| {
            adapter_error(
                format,
                format_args!("could not read numeric dataset '{name}': {error}"),
            )
        }),
    }
}

fn hdf_string_attr(
    attrs: &HashMap<String, rustyhdf5::AttrValue>,
    name: &str,
    format: &str,
) -> Result<String, String> {
    match attrs.get(name) {
        Some(rustyhdf5::AttrValue::String(value)) if !value.trim().is_empty() => Ok(value.clone()),
        Some(other) => Err(adapter_error(
            format,
            format_args!("attribute '{name}' must be a non-empty string, found {other:?}"),
        )),
        None => Err(adapter_error(
            format,
            format_args!("missing attribute '{name}'"),
        )),
    }
}

/// The unit a section states for one column, if it states one.
///
/// `rspice_core::io::hdf5` writes `signal_NNNN_unit` only when the producer
/// had a unit to state, so an absent attribute means unstated rather than
/// dimensionless, and it is never an import error. An empty or non-string
/// value is read the same way: a waveform must not come back claiming "" as
/// its unit.
fn hdf_stated_unit(attrs: &HashMap<String, rustyhdf5::AttrValue>, name: &str) -> Option<String> {
    hdf_optional_string_attr(attrs, name)
}

/// A text attribute the producer may or may not have written.
///
/// An absent attribute, a non-string one and an empty one are all "unstated":
/// the reader is asking whether the file says something, and "" is not a
/// statement.
fn hdf_optional_string_attr(
    attrs: &HashMap<String, rustyhdf5::AttrValue>,
    name: &str,
) -> Option<String> {
    match attrs.get(name) {
        Some(rustyhdf5::AttrValue::String(value)) if !value.trim().is_empty() => {
            Some(value.clone())
        }
        _ => None,
    }
}

fn hdf_i64_attr(
    attrs: &HashMap<String, rustyhdf5::AttrValue>,
    name: &str,
    format: &str,
) -> Result<i64, String> {
    match attrs.get(name) {
        Some(rustyhdf5::AttrValue::I64(value)) => Ok(*value),
        Some(other) => Err(adapter_error(
            format,
            format_args!("attribute '{name}' must be an integer, found {other:?}"),
        )),
        None => Err(adapter_error(
            format,
            format_args!("missing attribute '{name}'"),
        )),
    }
}

fn select_coordinate_name<'a>(
    names: impl Iterator<Item = &'a str>,
    candidates: &[&str],
) -> Option<String> {
    let names = names.collect::<Vec<_>>();
    for candidate in candidates {
        if let Some(name) = names
            .iter()
            .find(|name| name.eq_ignore_ascii_case(candidate))
        {
            return Some((*name).to_owned());
        }
    }
    None
}

pub fn stated_coordinate_names(names: &[&str]) -> String {
    let (last, rest) = names
        .split_last()
        .expect("the coordinate-name list is never empty");
    format!("{}, or {last}", rest.join(", "))
}

fn ensure_table_value_limit(
    format: &str,
    rows: usize,
    columns: usize,
    limits: Hdf5Limits<'_>,
) -> Result<(), String> {
    let values = rows
        .checked_mul(columns)
        .ok_or_else(|| adapter_error(format, "table value count overflow"))?;
    if values > limits.max_values {
        Err(adapter_error(
            format,
            format_args!(
                "the table contains {values} values; the limit is {}",
                limits.max_values
            ),
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_hdf5, DecodedHdf5, Hdf5Limits};

    #[test]
    fn root_reader_preserves_coordinate_and_numeric_bounds() {
        let mut builder = rustyhdf5::FileBuilder::new();
        builder.create_dataset("time").with_f64_data(&[0.0, 1.0]);
        builder.create_dataset("V(out)").with_f64_data(&[2.0, 3.0]);
        let bytes = builder.finish().expect("HDF5 fixture");
        let limits = Hdf5Limits {
            max_columns: 2,
            max_values: 4,
            coordinate_names: &["time", "x"],
        };
        let decoded = decode_hdf5(&bytes, limits, "hdf5").expect("root table");
        let DecodedHdf5::Root {
            coordinate_name,
            coordinate,
            columns,
        } = decoded
        else {
            panic!("root dataset should not be a named section");
        };
        assert_eq!(coordinate_name, "time");
        assert_eq!(coordinate, [0.0, 1.0]);
        assert_eq!(columns, [("V(out)".to_owned(), vec![2.0, 3.0])]);

        let small_limit = Hdf5Limits {
            max_values: 1,
            ..limits
        };
        let error = decode_hdf5(&bytes, small_limit, "hdf5").expect_err("value limit");
        assert!(error.contains("contains 2 values; the limit is 1"));
    }
}
