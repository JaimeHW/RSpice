//! Authored study-measurement selectors shared by execution and result lookup.

/// Explicit zero-based retained spectral bin, with no implicit complex reduction.
pub fn parse_study_bin(key: &str) -> Result<(usize, &str, Option<&str>), String> {
    let mut parts = key.splitn(3, ':');
    let index = parts
        .next()
        .unwrap_or_default()
        .parse::<usize>()
        .map_err(|_| "Spectral bin index must be a nonnegative integer")?;
    let quantity = parts.next().unwrap_or_default();
    if !["real", "imag", "magnitude", "phase"]
        .iter()
        .any(|name| quantity.eq_ignore_ascii_case(name))
    {
        return Err("Spectral quantity must be real, imag, magnitude, or phase (degrees)".into());
    }
    let signal = parts.next();
    if signal.is_some_and(|name| name.trim().is_empty()) {
        return Err("Spectral signal name is empty".into());
    }
    Ok((index, quantity, signal))
}

/// Signed lattice coordinates and a mandatory signal for a quasi-periodic observation.
pub fn parse_study_tuple(key: &str) -> Result<(Vec<i32>, &str, &str), String> {
    let mut parts = key.splitn(3, ':');
    let tuple = parts
        .next()
        .unwrap_or_default()
        .split(',')
        .map(|part| part.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Lattice coordinates must be comma-separated signed integers")?;
    let quantity = parts.next().unwrap_or_default();
    if !["real", "imag", "magnitude", "phase"]
        .iter()
        .any(|name| quantity.eq_ignore_ascii_case(name))
    {
        return Err("Lattice quantity must be real, imag, magnitude, or phase (degrees)".into());
    }
    let signal = parts
        .next()
        .filter(|signal| !signal.trim().is_empty())
        .ok_or("Lattice observations require an explicit signal")?;
    Ok((tuple, quantity, signal))
}

/// Validate the exact measurement selectors retained by a study.
pub fn validate_measurements(names: &[String]) -> Result<(), String> {
    if names.is_empty() {
        return Err("Select at least one study measurement".into());
    }
    let mut seen = std::collections::HashSet::new();
    for name in names {
        let (mode, key) = name.split_once(':').unwrap_or(("meas", name));
        if key.trim().is_empty()
            || name.chars().any(char::is_control)
            || !["meas", "scalar", "last", "bin", "tuple"]
                .iter()
                .any(|value| mode.eq_ignore_ascii_case(value))
        {
            return Err(format!(
                "Invalid study measurement {name:?}; use a .MEAS name, scalar:name, last:signal, bin:index:quantity[:signal], or tuple:k1,k2:quantity:signal"
            ));
        }
        if mode.eq_ignore_ascii_case("bin") {
            parse_study_bin(key)?;
        }
        if mode.eq_ignore_ascii_case("tuple") {
            parse_study_tuple(key)?;
        }
        if !seen.insert(name.to_ascii_lowercase()) {
            return Err(format!("Repeated study measurement {name:?}"));
        }
    }
    Ok(())
}
