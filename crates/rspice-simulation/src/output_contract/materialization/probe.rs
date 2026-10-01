//! Physical voltage/current projection from retained solved waveforms.
//! Canonical ground is an exact reference, not a solver unknown. Differential
//! AC values are formed in rectangular coordinates before magnitude projection.

use super::*;

pub fn resolve_raw_probe<W: OutputWaveform>(
    expression: &str,
    waveforms: &[W],
    output_name: &str,
    complex_domain: bool,
) -> Result<W, String> {
    resolve_bound_raw_probe(
        expression,
        output_name,
        waveforms.first(),
        complex_domain,
        |signal| {
            if signal.eq_ignore_ascii_case("V(0)") {
                Ok(Source::Ground)
            } else {
                find_waveform(waveforms, signal)
                    .map(Source::Waveform)
                    .ok_or_else(|| format!("source probe '{signal}' is absent"))
            }
        },
    )
}

pub(super) enum Source<'a, W> {
    Ground,
    Waveform(&'a W),
}

pub(super) fn resolve_bound_raw_probe<'a, W: OutputWaveform + 'a>(
    expression: &str,
    output_name: &str,
    axis: Option<&W>,
    complex_domain: bool,
    resolve: impl Fn(&str) -> Result<Source<'a, W>, String>,
) -> Result<W, String> {
    if rspice_results::saved_output::device_current_probe(expression).is_some() {
        return match resolve(expression.trim())? {
            Source::Waveform(source) => Ok(clone_with_name(source, output_name)),
            Source::Ground => Err("device current cannot bind to voltage ground".to_owned()),
        };
    }
    let (function, arguments) = parse_probe(expression)?;
    let voltage = function.eq_ignore_ascii_case("V");
    if !(voltage && matches!(arguments.len(), 1 | 2)
        || function.eq_ignore_ascii_case("I") && arguments.len() == 1)
    {
        return Err("probe must use V(node), V(node+, node-), or I(source)".to_owned());
    }
    let positive = resolve(&format!(
        "{}({})",
        if voltage { "V" } else { "I" },
        arguments[0]
    ))?;
    let negative = if arguments.len() == 2 {
        resolve(&format!("V({})", arguments[1]))?
    } else {
        Source::Ground
    };
    match (positive, negative) {
        (Source::Ground, Source::Ground) => ground_waveform(axis, output_name, complex_domain),
        (Source::Ground, Source::Waveform(negative)) => {
            negate_waveform(negative, output_name, complex_domain)
        }
        (Source::Waveform(positive), Source::Ground) => Ok(clone_with_name(positive, output_name)),
        (Source::Waveform(positive), Source::Waveform(negative)) => {
            subtract_waveforms(positive, negative, output_name, complex_domain)
        }
    }
}

fn ground_waveform<W: OutputWaveform>(
    axis: Option<&W>,
    name: &str,
    complex_domain: bool,
) -> Result<W, String> {
    let axis = axis
        .filter(|wave| !wave.as_ref().x.is_empty())
        .ok_or_else(|| "ground probe has no retained analysis axis".to_owned())?;
    let zeros = Arc::new(vec![0.0; axis.as_ref().x.len()]);
    let mut result = RetainedWaveform::new(name, Arc::clone(&axis.as_ref().x), Arc::clone(&zeros))
        .with_unit("V");
    if complex_domain || axis.as_ref().complex.is_some() {
        result = result.with_complex_components(name, Arc::clone(&zeros), zeros);
    }
    Ok(W::from_retained(result))
}

fn complex_samples<W: OutputWaveform>(waveform: &W) -> Result<(&[f64], &[f64]), String> {
    let complex = waveform.as_ref().complex.as_ref().ok_or_else(|| {
        "differential AC probe requires retained rectangular source samples".to_owned()
    })?;
    if complex.real.len() != waveform.as_ref().x.len()
        || complex.imag.len() != waveform.as_ref().x.len()
    {
        return Err("differential AC probe has misaligned rectangular samples".to_owned());
    }
    Ok((&complex.real, &complex.imag))
}

fn negate_waveform<W: OutputWaveform>(
    source: &W,
    name: &str,
    complex_domain: bool,
) -> Result<W, String> {
    if complex_domain || source.as_ref().complex.is_some() {
        let (real, imag) = complex_samples(source)?;
        Ok(complex_result(
            source,
            name,
            real.iter().map(|value| -value).collect(),
            imag.iter().map(|value| -value).collect(),
        ))
    } else {
        let mut result = clone_with_name(source, name);
        result.as_mut().y = Arc::new(source.as_ref().y.iter().map(|value| -value).collect());
        Ok(result)
    }
}

fn complex_result<W: OutputWaveform>(source: &W, name: &str, real: Vec<f64>, imag: Vec<f64>) -> W {
    let magnitude = real
        .iter()
        .zip(&imag)
        .map(|(real, imag)| real.hypot(*imag))
        .collect::<Vec<_>>();
    let mut result = RetainedWaveform::new(name, Arc::clone(&source.as_ref().x), magnitude)
        .with_complex_components(name, real, imag);
    result.unit = source.as_ref().unit.clone();
    W::from_retained(result)
}

fn subtract_waveforms<W: OutputWaveform>(
    positive: &W,
    negative: &W,
    name: &str,
    complex_domain: bool,
) -> Result<W, String> {
    if positive.as_ref().x != negative.as_ref().x
        || positive.as_ref().y.len() != negative.as_ref().y.len()
    {
        return Err("differential probe sources do not share an exact axis".to_owned());
    }
    if positive.as_ref().unit != negative.as_ref().unit {
        return Err("differential probe sources do not share a unit".to_owned());
    }
    if complex_domain || positive.as_ref().complex.is_some() || negative.as_ref().complex.is_some()
    {
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
            .as_ref()
            .y
            .iter()
            .zip(negative.as_ref().y.iter())
            .map(|(p, n)| p - n)
            .collect::<Vec<_>>();
        let mut result = RetainedWaveform::new(name, Arc::clone(&positive.as_ref().x), y);
        result.unit = positive.as_ref().unit.clone();
        Ok(W::from_retained(result))
    }
}
