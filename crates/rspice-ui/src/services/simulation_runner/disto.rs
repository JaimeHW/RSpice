//! Small-signal Volterra distortion analysis.

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, generate_freq_points_with_abort,
    parse_runner_netlist_with_abort,
};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::DistortionProduct;
use rspice_core::engine::Engine;
use std::collections::HashSet;
use std::fmt;
use std::path::Path;

/// Sweep type for DISTO analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistoFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl DistoFrequencySweep {
    fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// Explicit configuration for DISTO execution.
#[derive(Debug, Clone)]
pub struct DistoRunConfig {
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: DistoFrequencySweep,
    /// Optional secondary tone ratio for IMD estimates.
    pub f2_over_f1: Option<Value>,
}

impl DistoRunConfig {
    fn validate(&self) -> Result<(), DistoRunError> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err(DistoRunError::Validation(
                "DISTO start frequency must be positive".to_string(),
            ));
        }
        if !self.stop_freq.is_finite() || self.stop_freq <= self.start_freq {
            return Err(DistoRunError::Validation(
                "DISTO stop frequency must be greater than start frequency".to_string(),
            ));
        }
        if self.points_per_unit == 0 {
            return Err(DistoRunError::Validation(
                "DISTO points per unit must be greater than zero".to_string(),
            ));
        }
        if let Some(ratio) = self.f2_over_f1
            && (!ratio.is_finite() || ratio <= 0.0 || ratio >= 1.0)
        {
            return Err(DistoRunError::Validation(
                "DISTO f2_over_f1 must be finite and strictly between 0 and 1".to_string(),
            ));
        }
        Ok(())
    }
}

/// One exact nonlinear product relative to the F1 response of the same
/// voltage or current quantity.
#[derive(Debug, Clone)]
pub struct DistoProductTrace {
    pub product: DistortionProduct,
    /// Complex product/F1 ratio. Retaining the linear ratio keeps exact zero
    /// representable; the results viewer owns the dBc projection.
    pub ratios: Vec<Complex64>,
}

/// Per-quantity DISTO output.
#[derive(Debug, Clone)]
pub struct DistoTrace {
    pub name: String,
    pub unit: &'static str,
    pub fundamental_f1: Vec<Complex64>,
    pub fundamental_f2: Option<Vec<Complex64>>,
    pub products: Vec<DistoProductTrace>,
    /// Present only for single-tone harmonic distortion.
    pub thd_percent: Option<Vec<Value>>,
}

/// DISTO analysis output.
#[derive(Debug, Clone)]
pub struct DistoData {
    pub frequencies: Vec<Value>,
    pub traces: Vec<DistoTrace>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DistoRunError {
    Aborted,
    ResourceLimit(rspice_core::ResourceLimitError),
    Validation(String),
    Parse(String),
    Execution(String),
    Data(String),
}

impl fmt::Display for DistoRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Aborted => f.write_str("Simulation aborted"),
            Self::ResourceLimit(error) => fmt::Display::fmt(error, f),
            Self::Validation(message)
            | Self::Parse(message)
            | Self::Execution(message)
            | Self::Data(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for DistoRunError {}

impl DistoRunError {
    fn from_service(error: ServiceRunError, classify: fn(String) -> Self) -> Self {
        match error {
            ServiceRunError::Aborted => Self::Aborted,
            ServiceRunError::ResourceLimit(error) => Self::ResourceLimit(error),
            ServiceRunError::Failure(message) => classify(message),
        }
    }

    fn from_core(context: &str, error: rspice_core::SimulationError) -> Self {
        match ServiceRunError::from_core(context, error) {
            ServiceRunError::Aborted => Self::Aborted,
            ServiceRunError::ResourceLimit(error) => Self::ResourceLimit(error),
            ServiceRunError::Failure(message) => Self::Execution(message),
        }
    }

    fn into_service(self) -> ServiceRunError {
        match self {
            Self::Aborted => ServiceRunError::Aborted,
            Self::ResourceLimit(error) => ServiceRunError::ResourceLimit(error),
            other => ServiceRunError::Failure(other.to_string()),
        }
    }
}

#[inline]
fn ensure_disto_not_aborted(abort: &dyn AbortSignal) -> Result<(), DistoRunError> {
    ensure_not_aborted(abort).map_err(|_| DistoRunError::Aborted)
}

#[inline]
fn poll_disto_periodically(abort: &dyn AbortSignal, index: usize) -> Result<(), DistoRunError> {
    poll_periodically(abort, index).map_err(|_| DistoRunError::Aborted)
}

/// Run DISTO analysis with cooperative cancellation.
///
/// Test-only. The shipping path is
/// [`run_disto_analysis_with_source_path_and_abort`], reached from
/// `simulation::runner::spec::frequency`. Primary execution solves HB per
/// sweep point and retains the dedicated circuit-wide Volterra solver's
/// second- and third-order products. Excitation comes only from authored
/// `DISTOF1`/`DISTOF2` source annotations.
#[cfg(test)]
pub fn run_disto_analysis_with_abort(
    netlist_text: &str,
    config: &DistoRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<DistoData> {
    run_disto_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run DISTO analysis with source-path resolution and cooperative
/// cancellation through every nonlinear solve.
pub fn run_disto_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &DistoRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<DistoData> {
    run_disto_analysis_typed(netlist_text, config, source_path, abort)
        .map_err(DistoRunError::into_service)
}

fn run_disto_analysis_typed(
    netlist_text: &str,
    config: &DistoRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<DistoData, DistoRunError> {
    ensure_disto_not_aborted(abort)?;
    let validation = config.validate();
    ensure_disto_not_aborted(abort)?;
    validation?;

    run_disto_analysis_volterra(netlist_text, config, source_path, abort)
}

fn run_disto_analysis_volterra(
    netlist_text: &str,
    config: &DistoRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<DistoData, DistoRunError> {
    ensure_disto_not_aborted(abort)?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)
        .map_err(|error| DistoRunError::from_service(error, DistoRunError::Parse))?;
    let engine = Engine::new(build_engine_config(&netlist, None));

    let frequencies = generate_freq_points_with_abort(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
        abort,
    )
    .map_err(|error| DistoRunError::from_service(error, DistoRunError::Data))?;
    ensure_disto_not_aborted(abort)?;
    let result = engine
        .run_distortion_with_abort(&netlist, &frequencies, config.f2_over_f1, abort)
        .map_err(|error| DistoRunError::from_core("Volterra DISTO solve failed", error))?;
    let traces = convert_distortion_result(&frequencies, config.f2_over_f1, result, abort)?;

    ensure_disto_not_aborted(abort)?;
    Ok(DistoData {
        frequencies,
        traces,
    })
}

struct DistoAccumulator {
    name: String,
    unit: &'static str,
    is_voltage: bool,
    value_index: usize,
    fundamental_f1: Vec<Complex64>,
    fundamental_f2: Option<Vec<Complex64>>,
    products: Vec<DistoProductTrace>,
    thd_percent: Option<Vec<Value>>,
}

fn convert_distortion_result(
    frequencies: &[Value],
    requested_ratio: Option<Value>,
    result: rspice_core::analysis::DistortionAnalysisResult,
    abort: &dyn AbortSignal,
) -> Result<Vec<DistoTrace>, DistoRunError> {
    ensure_disto_not_aborted(abort)?;
    if result.f2_over_f1.map(Value::to_bits) != requested_ratio.map(Value::to_bits)
        || result.points.len() != frequencies.len()
        || result.points.is_empty()
    {
        return Err(DistoRunError::Data(
            "DISTO engine returned an inconsistent sweep basis".to_owned(),
        ));
    }
    let expected_products: &[DistortionProduct] = if requested_ratio.is_some() {
        &[
            DistortionProduct::Sum,
            DistortionProduct::Difference,
            DistortionProduct::ThirdOrderDifference,
        ]
    } else {
        &[
            DistortionProduct::SecondHarmonic,
            DistortionProduct::ThirdHarmonic,
        ]
    };

    let first = &result.points[0].fundamental_f1;
    validate_ac_response(first, frequencies[0], None, "DISTO F1 response")?;
    let mut accumulators = Vec::with_capacity(first.voltages.len() + first.currents.len());
    for (index, name) in first.node_names.iter().enumerate() {
        accumulators.push(new_accumulator(
            format!("V({name})"),
            "V",
            true,
            index,
            frequencies.len(),
            requested_ratio.is_some(),
            expected_products,
        ));
    }
    for (index, name) in first.branch_names.iter().enumerate() {
        accumulators.push(new_accumulator(
            format!("I({name})"),
            "A",
            false,
            index,
            frequencies.len(),
            requested_ratio.is_some(),
            expected_products,
        ));
    }
    if accumulators.is_empty() {
        return Err(DistoRunError::Data(
            "DISTO produced no voltage or current quantities".to_owned(),
        ));
    }

    let fixed_f2 = requested_ratio.map(|ratio| ratio * frequencies[0]);
    for (point_index, (point, &f1)) in result.points.iter().zip(frequencies).enumerate() {
        poll_disto_periodically(abort, point_index)?;
        validate_ac_response(&point.fundamental_f1, f1, Some(first), "DISTO F1 response")?;
        match (&point.fundamental_f2, fixed_f2) {
            (Some(response), Some(f2)) => {
                validate_ac_response(response, f2, Some(first), "DISTO F2 response")?;
            }
            (None, None) => {}
            _ => {
                return Err(DistoRunError::Data(format!(
                    "DISTO point {} returned inconsistent F2 evidence",
                    point_index + 1
                )));
            }
        }

        let mut seen = HashSet::with_capacity(point.products.len());
        for product in &point.products {
            if !seen.insert(product.product) || !expected_products.contains(&product.product) {
                return Err(DistoRunError::Data(format!(
                    "DISTO point {} returned an unexpected or duplicate {} product",
                    point_index + 1,
                    product.product.label()
                )));
            }
            let expected_frequency = product_frequency(product.product, f1, fixed_f2)?;
            validate_ac_response(
                &product.response,
                expected_frequency,
                Some(first),
                "DISTO nonlinear response",
            )?;
        }
        if seen.len() != expected_products.len()
            || expected_products
                .iter()
                .any(|product| !seen.contains(product))
        {
            return Err(DistoRunError::Data(format!(
                "DISTO point {} returned an incomplete nonlinear product set",
                point_index + 1
            )));
        }

        for accumulator in &mut accumulators {
            ensure_disto_not_aborted(abort)?;
            let fundamental = response_value(&point.fundamental_f1, accumulator)?;
            accumulator.fundamental_f1.push(fundamental);
            if let Some(f2) = point.fundamental_f2.as_ref() {
                let value = response_value(f2, accumulator)?;
                accumulator
                    .fundamental_f2
                    .as_mut()
                    .ok_or_else(|| {
                        DistoRunError::Data("DISTO F2 accumulator is unavailable".to_owned())
                    })?
                    .push(value);
            }

            let mut harmonic_ratios = [0.0, 0.0];
            for (product_index, expected_product) in expected_products.iter().enumerate() {
                let product = point.product(*expected_product).ok_or_else(|| {
                    DistoRunError::Data(format!(
                        "DISTO point {} is missing {}",
                        point_index + 1,
                        expected_product.label()
                    ))
                })?;
                let numerator = response_value(&product.response, accumulator)?;
                let ratio = exact_complex_ratio(
                    numerator,
                    fundamental,
                    &accumulator.name,
                    *expected_product,
                )?;
                accumulator.products[product_index].ratios.push(ratio);
                if *expected_product == DistortionProduct::SecondHarmonic {
                    harmonic_ratios[0] = ratio.norm();
                } else if *expected_product == DistortionProduct::ThirdHarmonic {
                    harmonic_ratios[1] = ratio.norm();
                }
            }
            if let Some(thd) = accumulator.thd_percent.as_mut() {
                let value = harmonic_ratios[0].hypot(harmonic_ratios[1]) * 100.0;
                if !value.is_finite() {
                    return Err(DistoRunError::Data(format!(
                        "DISTO THD for '{}' is non-finite",
                        accumulator.name
                    )));
                }
                thd.push(value);
            }
        }
    }

    ensure_disto_not_aborted(abort)?;
    Ok(accumulators
        .into_iter()
        .map(|accumulator| DistoTrace {
            name: accumulator.name,
            unit: accumulator.unit,
            fundamental_f1: accumulator.fundamental_f1,
            fundamental_f2: accumulator.fundamental_f2,
            products: accumulator.products,
            thd_percent: accumulator.thd_percent,
        })
        .collect())
}

fn new_accumulator(
    name: String,
    unit: &'static str,
    is_voltage: bool,
    value_index: usize,
    point_count: usize,
    two_tone: bool,
    products: &[DistortionProduct],
) -> DistoAccumulator {
    DistoAccumulator {
        name,
        unit,
        is_voltage,
        value_index,
        fundamental_f1: Vec::with_capacity(point_count),
        fundamental_f2: two_tone.then(|| Vec::with_capacity(point_count)),
        products: products
            .iter()
            .copied()
            .map(|product| DistoProductTrace {
                product,
                ratios: Vec::with_capacity(point_count),
            })
            .collect(),
        thd_percent: (!two_tone).then(|| Vec::with_capacity(point_count)),
    }
}

fn validate_ac_response(
    response: &rspice_core::analysis::AcResult,
    expected_frequency: Value,
    expected_basis: Option<&rspice_core::analysis::AcResult>,
    context: &str,
) -> Result<(), DistoRunError> {
    if response.frequency.to_bits() != expected_frequency.to_bits()
        || !response.frequency.is_finite()
        || response.frequency <= 0.0
        || response.node_names.len() != response.voltages.len()
        || response.branch_names.len() != response.currents.len()
        || response
            .voltages
            .iter()
            .chain(&response.currents)
            .any(|value| !value.re.is_finite() || !value.im.is_finite())
    {
        return Err(DistoRunError::Data(format!(
            "{context} returned an invalid frequency, shape, or value"
        )));
    }
    if let Some(expected) = expected_basis {
        if response.node_names != expected.node_names
            || response.branch_names != expected.branch_names
        {
            return Err(DistoRunError::Data(format!(
                "{context} changed the solved quantity basis"
            )));
        }
    } else {
        let mut names =
            HashSet::with_capacity(response.node_names.len() + response.branch_names.len());
        for (kind, name) in response
            .node_names
            .iter()
            .map(|name| ("voltage", name))
            .chain(response.branch_names.iter().map(|name| ("current", name)))
        {
            let identity = format!("{kind}:{}", name.trim().to_ascii_lowercase());
            if name.trim().is_empty() || !names.insert(identity) {
                return Err(DistoRunError::Data(format!(
                    "{context} returned an empty or duplicate quantity identity"
                )));
            }
        }
    }
    Ok(())
}

fn product_frequency(
    product: DistortionProduct,
    f1: Value,
    f2: Option<Value>,
) -> Result<Value, DistoRunError> {
    let frequency = match (product, f2) {
        (DistortionProduct::SecondHarmonic, None) => 2.0 * f1,
        (DistortionProduct::ThirdHarmonic, None) => 3.0 * f1,
        (DistortionProduct::Sum, Some(f2)) => f1 + f2,
        (DistortionProduct::Difference, Some(f2)) => f1 - f2,
        (DistortionProduct::ThirdOrderDifference, Some(f2)) => 2.0 * f1 - f2,
        _ => {
            return Err(DistoRunError::Data(format!(
                "DISTO product {} is inconsistent with the selected tone mode",
                product.label()
            )));
        }
    };
    if !frequency.is_finite() || frequency <= 0.0 {
        return Err(DistoRunError::Data(format!(
            "DISTO product {} has invalid frequency {frequency}",
            product.label()
        )));
    }
    Ok(frequency)
}

fn response_value(
    response: &rspice_core::analysis::AcResult,
    accumulator: &DistoAccumulator,
) -> Result<Complex64, DistoRunError> {
    let value = if accumulator.is_voltage {
        response.voltages.get(accumulator.value_index)
    } else {
        response.currents.get(accumulator.value_index)
    }
    .copied()
    .ok_or_else(|| {
        DistoRunError::Data(format!("DISTO response is missing '{}'", accumulator.name))
    })?;
    Ok(value)
}

fn exact_complex_ratio(
    numerator: Complex64,
    denominator: Complex64,
    quantity: &str,
    product: DistortionProduct,
) -> Result<Complex64, DistoRunError> {
    let ratio = if denominator == Complex64::new(0.0, 0.0) {
        if numerator == Complex64::new(0.0, 0.0) {
            Complex64::new(0.0, 0.0)
        } else {
            return Err(DistoRunError::Data(format!(
                "DISTO {} ratio for '{}' is infinite because the F1 response is zero",
                product.label(),
                quantity
            )));
        }
    } else {
        numerator / denominator
    };
    if !ratio.re.is_finite() || !ratio.im.is_finite() {
        return Err(DistoRunError::Data(format!(
            "DISTO {} ratio for '{}' is non-finite",
            product.label(),
            quantity
        )));
    }
    Ok(ratio)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use rspice_core::abort_signal::NoAbort;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn cancellation_precedes_invalid_disto_configuration() {
        let config = DistoRunConfig {
            start_freq: -1.0,
            stop_freq: -2.0,
            points_per_unit: 0,
            sweep: DistoFrequencySweep::Decade,
            f2_over_f1: None,
        };
        let abort = AbortOnPoll {
            abort_on: 2,
            polls: AtomicUsize::new(0),
        };

        let result = run_disto_analysis_with_abort("invalid", &config, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn two_tone_ratio_follows_the_spice_f2_below_f1_contract() {
        let valid = DistoRunConfig {
            start_freq: 1.0e3,
            stop_freq: 2.0e3,
            points_per_unit: 3,
            sweep: DistoFrequencySweep::Linear,
            f2_over_f1: Some(0.9),
        };
        assert!(valid.validate().is_ok());

        for ratio in [0.0, 1.0, 1.1, f64::INFINITY, f64::NAN] {
            let mut invalid = valid.clone();
            invalid.f2_over_f1 = Some(ratio);
            assert!(invalid.validate().is_err(), "ratio {ratio} must fail");
        }
    }

    #[test]
    fn linear_zero_distortion_is_retained_as_exact_zero_ratio() {
        let config = DistoRunConfig {
            start_freq: 1.0e3,
            stop_freq: 2.0e3,
            points_per_unit: 3,
            sweep: DistoFrequencySweep::Linear,
            f2_over_f1: None,
        };
        let data = run_disto_analysis_with_abort(
            "linear distortion\n\
             V1 in 0 DISTOF1 1 0\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .end\n",
            &config,
            &NoAbort,
        )
        .expect("dedicated Volterra path solves");

        assert!(!data.frequencies.is_empty());
        let output = data
            .traces
            .iter()
            .find(|trace| trace.name.eq_ignore_ascii_case("V(out)"))
            .expect("output voltage trace");
        assert_eq!(output.products.len(), 2);
        assert!(output.products.iter().all(|product| {
            product.ratios.len() == data.frequencies.len()
                && product
                    .ratios
                    .iter()
                    .all(|ratio| *ratio == Complex64::new(0.0, 0.0))
        }));
        assert!(
            output
                .thd_percent
                .as_ref()
                .is_some_and(|values| values.iter().all(|value| *value == 0.0))
        );
    }
}
