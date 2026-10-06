//! Touchstone adaptation uses the same v1/v2 parser as the application importer.
use super::*;

pub(super) fn load(
    path: &Path,
    limits: rspice_core::ResourceLimits,
    section: Option<&str>,
) -> Result<ExportTable, CliError> {
    let text = read_utf8_input_limited(path, limits.max_external_data_bytes)?;
    parse(path, text.as_bytes(), limits, section)
}

pub(super) fn parse(
    path: &Path,
    bytes: &[u8],
    limits: rspice_core::ResourceLimits,
    section: Option<&str>,
) -> Result<ExportTable, CliError> {
    let dataset = rspice_formats::read_touchstone_bytes_with_limit(
        &path.to_string_lossy(),
        bytes,
        limits.max_external_data_values,
    )
    .map_err(|error| match error {
        rspice_formats::TouchstoneError::ValueLimit { requested, limit } => resource_limit_error(
            path,
            rspice_core::ResourceKind::ExternalDataValues,
            requested,
            limit,
        ),
        error => conversion_error(path, error),
    })?;
    let has_noise = dataset.signals.iter().any(|signal| signal.name == "Fmin");
    let names: &[&str] = if has_noise {
        &["network", "noise"]
    } else {
        &["network"]
    };
    let index = select_section(path, names, section)?;
    let reference = dataset
        .metadata
        .get("z0_ports")
        .ok_or_else(|| conversion_error(path, "missing Touchstone port references"))?;
    let references = reference
        .split(',')
        .map(|value| {
            value
                .parse::<f64>()
                .map_err(|error| conversion_error(path, error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut scale = dataset
        .x_signal
        .ok_or_else(|| conversion_error(path, "missing Touchstone frequency axis"))?
        .data;
    let selected: Vec<_> = dataset
        .signals
        .into_iter()
        .filter(|signal| {
            (signal.signal_type == rspice_formats::SignalType::SParameter) == (index == 0)
        })
        .collect();
    if index == 1
        && let Some(axis) = selected.first().and_then(|signal| signal.x_values.as_ref())
    {
        scale = axis.clone();
    }
    let retained_values = selected
        .iter()
        .fold(scale.len(), |count, signal| {
            count.saturating_add(signal.data.len())
        })
        .saturating_add(
            scale
                .len()
                .saturating_mul(references.len() + usize::from(index == 1)),
        );
    enforce_resource_limit(
        path,
        rspice_core::ResourceKind::ExternalDataValues,
        retained_values,
        limits.max_external_data_values,
    )?;
    let mut columns = Vec::new();
    let mut signals = selected.into_iter().peekable();
    while let Some(signal) = signals.next() {
        if let Some(name) = signal.name.strip_suffix("_RE") {
            let imaginary = signals
                .next()
                .ok_or_else(|| conversion_error(path, "missing Touchstone imaginary column"))?;
            if imaginary.name != format!("{name}_IM") {
                return Err(conversion_error(
                    path,
                    "misordered Touchstone complex columns",
                ));
            }
            columns.push(ExportColumn {
                unit: None,
                name: name.to_owned(),
                var_type: "dimensionless".to_owned(),
                data: ColumnData::Complex {
                    real: signal.data,
                    imag: imaginary.data,
                },
            });
        } else {
            columns.push(ExportColumn {
                unit: None,
                var_type: if signal.name == "Rn" {
                    "impedance"
                } else {
                    "dimensionless"
                }
                .to_owned(),
                name: signal.name,
                data: ColumnData::Real(signal.data),
            });
        }
    }
    // Scattering values depend on these references: include them in comparisons
    // and conversions, even when every coefficient happens to be identical.
    for (port, value) in references.into_iter().enumerate() {
        columns.push(ExportColumn {
            unit: None,
            name: format!("Z0({})", port + 1),
            var_type: "impedance".to_owned(),
            data: ColumnData::Real(vec![value; scale.len()]),
        });
    }
    if index == 1 {
        let temperature = dataset
            .metadata
            .get("noise_reference_temperature_kelvin")
            .ok_or_else(|| {
                conversion_error(path, "missing Touchstone noise reference temperature")
            })?
            .parse::<f64>()
            .map_err(|error| conversion_error(path, error))?;
        columns.push(ExportColumn {
            unit: None,
            name: "noise_reference_temperature".to_owned(),
            var_type: "temperature".to_owned(),
            data: ColumnData::Real(vec![temperature; scale.len()]),
        });
    }
    Ok(ExportTable {
        scale_unit: None,
        analysis: names[index].to_owned(),
        plot_name: format!("Touchstone {}", names[index]),
        scale_name: "frequency".to_owned(),
        scale_type: "frequency".to_owned(),
        scale,
        columns,
    })
}
