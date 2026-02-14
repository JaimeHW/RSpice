#[cfg(test)]
pub(super) fn parse_spice_value(s: &str) -> f64 {
    let s = s.trim();
    if s.is_empty() {
        return 0.0;
    }

    // Try direct parse first
    if let Ok(v) = s.parse::<f64>() {
        return v;
    }

    // Find where the number ends
    let mut num_end = 0;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' || c == 'e' || c == 'E' {
            num_end = i + c.len_utf8();
        } else {
            break;
        }
    }

    let (num_str, suffix) = s.split_at(num_end);
    let base: f64 = num_str.parse().unwrap_or(0.0);

    let multiplier = match suffix.to_lowercase().as_str() {
        "t" | "tera" => 1e12,
        "g" | "gig" => 1e9,
        "meg" | "m" if suffix.len() >= 3 => 1e6, // MEG is megHH, not milli
        "k" | "kilo" => 1e3,
        "m" | "milli" => 1e-3,
        "u" | "µ" | "micro" => 1e-6,
        "n" | "nano" => 1e-9,
        "p" | "pico" => 1e-12,
        "f" | "femto" => 1e-15,
        "a" | "atto" => 1e-18,
        "" => 1.0,
        _ => 1.0,
    };

    base * multiplier
}

pub(super) fn parse_spice_value_checked(s: &str) -> Result<f64, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("value is empty".to_string());
    }

    if let Ok(v) = s.parse::<f64>() {
        if v.is_finite() {
            return Ok(v);
        }
        return Err("value is not finite".to_string());
    }

    let mut num_end = 0;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' || c == 'e' || c == 'E' {
            num_end = i + c.len_utf8();
        } else {
            break;
        }
    }

    if num_end == 0 || num_end == s.len() && s.parse::<f64>().is_err() {
        return Err(format!("invalid numeric value '{}'", s));
    }

    let (num_str, suffix) = s.split_at(num_end);
    let base: f64 = num_str
        .parse()
        .map_err(|_| format!("invalid numeric value '{}'", s))?;

    let multiplier = match suffix.to_ascii_lowercase().as_str() {
        "t" | "tera" => 1e12,
        "g" | "gig" => 1e9,
        "meg" => 1e6,
        "k" | "kilo" => 1e3,
        "m" | "milli" => 1e-3,
        "u" | "micro" => 1e-6,
        "n" | "nano" => 1e-9,
        "p" | "pico" => 1e-12,
        "f" | "femto" => 1e-15,
        "a" | "atto" => 1e-18,
        "" => 1.0,
        _ => return Err(format!("unsupported SPICE suffix '{}'", suffix)),
    };

    let value = base * multiplier;
    if value.is_finite() {
        Ok(value)
    } else {
        Err("value is not finite".to_string())
    }
}

//=============================================================================
// Tests
//=============================================================================
