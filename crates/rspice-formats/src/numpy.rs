//! NumPy NPY byte encoding for real and complex arrays.

use npyz::WriterBuilder as _;
use num_complex::Complex64;

fn npy_error(error: std::io::Error) -> String {
    format!("The NumPy array could not be written: {error}")
}

/// Encode real samples with the specified NumPy array shape.
pub fn encode_real_array(shape: &[u64], values: &[f64]) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    let mut writer = npyz::WriteOptions::<f64>::new()
        .default_dtype()
        .shape(shape)
        .writer(&mut bytes)
        .begin_nd()
        .map_err(npy_error)?;
    writer.extend(values.iter().copied()).map_err(npy_error)?;
    writer.finish().map_err(npy_error)?;
    Ok(bytes)
}

/// Encode rectangular complex samples with the specified NumPy array shape.
pub fn encode_complex_array(shape: &[u64], values: &[Complex64]) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    let mut writer = npyz::WriteOptions::<Complex64>::new()
        .default_dtype()
        .shape(shape)
        .writer(&mut bytes)
        .begin_nd()
        .map_err(npy_error)?;
    writer.extend(values.iter().copied()).map_err(npy_error)?;
    writer.finish().map_err(npy_error)?;
    Ok(bytes)
}
