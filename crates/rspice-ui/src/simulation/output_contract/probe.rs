//! Physical voltage/current projection from retained solved waveforms.
//! Canonical ground is an exact reference, not a solver unknown. Differential
//! AC values are formed in rectangular coordinates before magnitude projection.

use super::*;

pub(super) fn resolve_raw_probe(
    expression: &str,
    waveforms: &[WaveformData],
    output_name: &str,
    complex_domain: bool,
) -> Result<WaveformData, String> {
    resolve_raw_probe_with(
        expression,
        output_name,
        waveforms.first(),
        complex_domain,
        |name| find_waveform(waveforms, name),
    )
}

pub(super) fn resolve_raw_probe_with<'a>(
    expression: &str,
    output_name: &str,
    axis: Option<&WaveformData>,
    complex_domain: bool,
    find: impl Fn(&str) -> Option<&'a WaveformData>,
) -> Result<WaveformData, String> {
    let (function, arguments) = parse_probe(expression)?;
    let voltage = function.eq_ignore_ascii_case("V");
    if !(voltage && matches!(arguments.len(), 1 | 2)
        || function.eq_ignore_ascii_case("I") && arguments.len() == 1)
    {
        return Err("probe must use V(node), V(node+, node-), or I(source)".to_owned());
    }
    let source = |node: &str| {
        find(&format!("{}({node})", if voltage { "V" } else { "I" }))
            .ok_or_else(|| format!("source probe '{function}({node})' is absent"))
    };
    match arguments.as_slice() {
        [positive, negative] if positive == "0" && negative == "0" => {
            ground_waveform(axis, output_name, complex_domain)
        }
        [positive, negative] if positive == "0" => {
            negate_waveform(source(negative)?, output_name, complex_domain)
        }
        [positive, negative] if negative != "0" => subtract_waveforms(
            source(positive)?,
            source(negative)?,
            output_name,
            complex_domain,
        ),
        [node] if voltage && node == "0" => ground_waveform(axis, output_name, complex_domain),
        [node] | [node, _] => Ok(clone_with_name(source(node)?, output_name)),
        _ => unreachable!("probe arity was validated"),
    }
}

fn ground_waveform(
    axis: Option<&WaveformData>,
    name: &str,
    complex_domain: bool,
) -> Result<WaveformData, String> {
    let axis = axis
        .filter(|wave| !wave.x.is_empty())
        .ok_or_else(|| "ground probe has no retained analysis axis".to_owned())?;
    let zeros = Arc::new(vec![0.0; axis.x.len()]);
    let mut result =
        WaveformData::new(name, Arc::clone(&axis.x), Arc::clone(&zeros), "#f5b700").with_unit("V");
    if complex_domain || axis.complex.is_some() {
        result = result.with_complex_components(name, Arc::clone(&zeros), zeros);
    }
    Ok(result)
}

fn complex_samples(waveform: &WaveformData) -> Result<(&[f64], &[f64]), String> {
    let complex = waveform.complex.as_ref().ok_or_else(|| {
        "differential AC probe requires retained rectangular source samples".to_owned()
    })?;
    if complex.real.len() != waveform.x.len() || complex.imag.len() != waveform.x.len() {
        return Err("differential AC probe has misaligned rectangular samples".to_owned());
    }
    Ok((&complex.real, &complex.imag))
}

fn negate_waveform(
    source: &WaveformData,
    name: &str,
    complex_domain: bool,
) -> Result<WaveformData, String> {
    if complex_domain || source.complex.is_some() {
        let (real, imag) = complex_samples(source)?;
        Ok(complex_result(
            source,
            name,
            real.iter().map(|value| -value).collect(),
            imag.iter().map(|value| -value).collect(),
        ))
    } else {
        let mut result = clone_with_name(source, name);
        result.y = Arc::new(source.y.iter().map(|value| -value).collect());
        Ok(result)
    }
}

fn complex_result(
    source: &WaveformData,
    name: &str,
    real: Vec<f64>,
    imag: Vec<f64>,
) -> WaveformData {
    let magnitude = real
        .iter()
        .zip(&imag)
        .map(|(real, imag)| real.hypot(*imag))
        .collect::<Vec<_>>();
    let mut result = WaveformData::new(name, Arc::clone(&source.x), magnitude, "#f5b700")
        .with_complex_components(name, real, imag);
    result.unit = source.unit.clone();
    result
}

fn subtract_waveforms(
    positive: &WaveformData,
    negative: &WaveformData,
    name: &str,
    complex_domain: bool,
) -> Result<WaveformData, String> {
    if positive.x != negative.x || positive.y.len() != negative.y.len() {
        return Err("differential probe sources do not share an exact axis".to_owned());
    }
    if positive.unit != negative.unit {
        return Err("differential probe sources do not share a unit".to_owned());
    }
    if complex_domain || positive.complex.is_some() || negative.complex.is_some() {
        let (positive_real, positive_imag) = complex_samples(positive)?;
        let (negative_real, negative_imag) = complex_samples(negative)?;
        Ok(complex_result(
            positive,
            name,
            positive_real
                .iter()
                .zip(negative_real)
                .map(|(p, n)| p - n)
                .collect(),
            positive_imag
                .iter()
                .zip(negative_imag)
                .map(|(p, n)| p - n)
                .collect(),
        ))
    } else {
        let y = positive
            .y
            .iter()
            .zip(negative.y.iter())
            .map(|(p, n)| p - n)
            .collect::<Vec<_>>();
        let mut result = WaveformData::new(name, Arc::clone(&positive.x), y, "#f5b700");
        result.unit = positive.unit.clone();
        Ok(result)
    }
}
