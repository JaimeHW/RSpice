//! Shared checks for exact retained result evidence.

pub fn require_non_empty(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{label} is empty"))
    } else {
        Ok(())
    }
}

pub fn require_finite_values(values: &[f64], label: &str) -> Result<(), String> {
    if values.iter().any(|value| !value.is_finite()) {
        Err(format!("{label} contain a non-finite value"))
    } else {
        Ok(())
    }
}

pub fn strictly_increasing(values: &[f64]) -> bool {
    values
        .windows(2)
        .all(|pair| normalized_f64(pair[0]) < normalized_f64(pair[1]))
}

pub fn normalized_f64(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}
