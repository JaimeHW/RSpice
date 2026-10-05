//! Admit declared counts and decoded dataset sizes before allocating results.

use super::{Hdf5Error, Result, non_negative_count};
use rspice_core::{ResourceKind, ResourceLimitError, ResourceLimits};
use rustyhdf5::{AttrValue, DType, File};

pub(super) fn admit(resource: ResourceKind, requested: usize, limit: usize) -> Result<()> {
    if requested > limit {
        return Err(ResourceLimitError {
            resource,
            requested,
            limit,
        }
        .into());
    }
    Ok(())
}

pub(super) fn validate(file: &File, limits: ResourceLimits) -> Result<()> {
    let mut values = 0usize;
    for group_name in file.root().groups()? {
        let group = file.group(&group_name)?;
        let attrs = group.attrs()?;
        let datasets = group.datasets()?;
        for (name, value) in &attrs {
            if !name.ends_with("_count") {
                continue;
            }
            let AttrValue::I64(value) = value else {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "count attribute '{name}' must be an integer"
                )));
            };
            let count = non_negative_count(*value, name)?;
            admit(
                ResourceKind::ExternalDataValues,
                count,
                limits.max_external_data_values,
            )?;
            // Every retained signal/series/result/measurement requires at
            // least one named attribute or dataset. Reject invented counts
            // before reserving vectors for records that do not exist.
            if (name.ends_with("signal_count")
                || name == "series_count"
                || name == "result_count"
                || name == "measurement_count")
                && count > attrs.len().saturating_add(datasets.len())
            {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "'{name}' declares {count} records without enough metadata in '{group_name}'"
                )));
            }
        }
        for name in datasets {
            let dataset = group.dataset(&name)?;
            if !matches!(
                dataset.dtype()?,
                DType::F32
                    | DType::F64
                    | DType::I8
                    | DType::I16
                    | DType::I32
                    | DType::I64
                    | DType::U8
                    | DType::U16
                    | DType::U32
                    | DType::U64
            ) {
                return Err(Hdf5Error::InvalidSchema(format!(
                    "dataset '{group_name}/{name}' must have a numeric scalar datatype"
                )));
            }
            let count = dataset
                .shape()?
                .into_iter()
                .try_fold(1usize, |size, dim| {
                    usize::try_from(dim)
                        .ok()
                        .and_then(|dim| size.checked_mul(dim))
                })
                .unwrap_or(usize::MAX);
            values = values.saturating_add(count);
            admit(
                ResourceKind::ExternalDataValues,
                values,
                limits.max_external_data_values,
            )?;
            admit(
                ResourceKind::ExternalDataBytes,
                values.saturating_mul(size_of::<f64>()),
                limits.max_external_data_bytes,
            )?;
        }
    }
    Ok(())
}
