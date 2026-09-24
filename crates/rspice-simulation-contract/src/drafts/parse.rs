//! Shared parsing of authored numeric draft fields.

pub fn parse_positive(text: &str, field: &str) -> Result<f64, String> {
    let value = crate::options::parse_si_value(text)
        .map_err(|error| format!("invalid {field}: {error}"))?;
    if value <= 0.0 {
        Err(format!("{field} must be greater than zero"))
    } else {
        Ok(value)
    }
}

pub fn parse_nonnegative(text: &str, field: &str) -> Result<f64, String> {
    let value = crate::options::parse_si_value(text)
        .map_err(|error| format!("invalid {field}: {error}"))?;
    if value < 0.0 {
        Err(format!("{field} must not be negative"))
    } else {
        Ok(value)
    }
}

pub fn parse_positive_usize(text: &str, field: &str) -> Result<usize, String> {
    let value = text
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("{field} must be a positive integer"))?;
    if value == 0 {
        Err(format!("{field} must be greater than zero"))
    } else {
        Ok(value)
    }
}

pub fn parse_i32_tuple(text: &str, field: &str) -> Result<Vec<i32>, String> {
    let values = text
        .split(',')
        .map(|part| part.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| format!("{field} must contain comma-separated integers"))?;
    if values.len() < 2 {
        return Err(format!("{field} must contain at least two integers"));
    }
    Ok(values)
}
