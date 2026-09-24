//! Exact conversion of integer sample values to the waveform `f64` representation.

pub const MAX_EXACT_F64_INTEGER: u64 = 1_u64 << 53;

pub fn exact_signed_integer(identity: &str, value: i64) -> Result<f64, String> {
    if value.unsigned_abs() > MAX_EXACT_F64_INTEGER {
        Err(format!(
            "'{identity}' integer {value} cannot be represented exactly as f64"
        ))
    } else {
        Ok(value as f64)
    }
}

pub fn exact_unsigned_integer(identity: &str, value: u64) -> Result<f64, String> {
    if value > MAX_EXACT_F64_INTEGER {
        Err(format!(
            "'{identity}' integer {value} cannot be represented exactly as f64"
        ))
    } else {
        Ok(value as f64)
    }
}
