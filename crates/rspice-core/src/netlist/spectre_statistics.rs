//! Executable statistical semantics shared by the Spectre frontend and the
//! circuit elaborator.
//!
//! The sampler is deliberately counter based.  Every latent draw is a pure
//! function of the authored seed, run coordinate, variation scope, instance
//! identity, and variable index.  Scheduling and worker count therefore have
//! no opportunity to perturb a statistical result.
//!
//! # The `.RSPICE_SPECTRE_STAT` carrier is a deliberate internal ABI
//!
//! The Spectre adapter is a source-text-in, source-text-out dialect
//! front-end: the include expander runs it on every source it reads, before
//! any card is parsed, and it must not change a source's line count. A
//! `statistics` block therefore cannot hand the parser a typed value — the
//! only thing that survives the round trip through `.include` expansion, and
//! through the include cache, is a line of text.
//!
//! [`SPECTRE_STATISTICS_DIRECTIVE`] is that line. It is not authored syntax
//! and is not documented for users; the payload is
//! [`SpectreStatisticsPlan::encode_internal`]'s versioned, whitespace-free
//! record list, which [`SpectreStatisticsPlan::decode_directive`] is the one
//! place that decodes. The parser's whole-source prescan calls that decoder
//! so declarations apply independent of library statement order, and the
//! ordinary command dispatcher only consumes the line lexically — it never
//! parses the payload a second time.

use super::{ParamContext, expr::eval_expression};
use crate::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Internal directive carrying a lowered Spectre `statistics` block from the
/// adapter to the parser. See the module documentation: this is an internal
/// ABI, not authored netlist syntax.
pub(crate) const SPECTRE_STATISTICS_DIRECTIVE: &str = ".RSPICE_SPECTRE_STAT";

const STATISTICS_ENCODING_VERSION: &str = "S1";
const PSD_TOLERANCE: Value = 1.0e-10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpectreVariationScope {
    Process,
    Mismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectreDistribution {
    Gaussian,
    Uniform,
    Lognormal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpectreSpread {
    StandardDeviation(String),
    HalfRange(String),
}

impl SpectreSpread {
    fn expression(&self) -> &str {
        match self {
            Self::StandardDeviation(expression) | Self::HalfRange(expression) => expression,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpectreVariation {
    pub line: usize,
    pub scope: SpectreVariationScope,
    pub parameter: String,
    pub distribution: SpectreDistribution,
    pub spread: SpectreSpread,
    pub percent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpectreCorrelation {
    pub line: usize,
    pub scope: SpectreVariationScope,
    pub parameters: Vec<String>,
    pub coefficient: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpectreStatisticsPlan {
    pub variations: Vec<SpectreVariation>,
    pub correlations: Vec<SpectreCorrelation>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpectreStatisticalCoordinate {
    pub seed: u64,
    /// Zero-based Monte Carlo point.  Keeping this explicit makes one point
    /// replayable without consuming the preceding points.
    pub monte_carlo_run: u64,
    pub temperature_celsius: Value,
    /// Authored sweep coordinates.  Names are canonicalized and sorted before
    /// hashing, so loop nesting and materialization order are immaterial.
    pub axes: Vec<(String, Value)>,
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum SpectreStatisticsError {
    #[error("line {line}: {message}")]
    InvalidDeclaration { line: usize, message: String },
    #[error("invalid internal Spectre statistics encoding: {0}")]
    InvalidEncoding(String),
    #[error("Spectre statistics expression '{expression}' could not be evaluated: {reason}")]
    Expression { expression: String, reason: String },
    #[error("Spectre correlation matrix is invalid: {0}")]
    CorrelationMatrix(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpectreCorrelationMatrix {
    values: Vec<Vec<Value>>,
    lower: Vec<Vec<Value>>,
}

impl SpectreCorrelationMatrix {
    pub fn new(values: Vec<Vec<Value>>) -> Result<Self, SpectreStatisticsError> {
        let size = values.len();
        if values.iter().any(|row| row.len() != size) {
            return Err(SpectreStatisticsError::CorrelationMatrix(
                "matrix must be square".to_owned(),
            ));
        }
        // The symmetry check reads `[column][row]` as well, which is in a
        // different row of the matrix.
        #[allow(clippy::needless_range_loop)]
        for row in 0..size {
            for column in 0..size {
                let value = values[row][column];
                if !value.is_finite() || !(-1.0..=1.0).contains(&value) {
                    return Err(SpectreStatisticsError::CorrelationMatrix(format!(
                        "entry ({},{}) must be finite and in [-1, 1], got {value}",
                        row + 1,
                        column + 1
                    )));
                }
                if row == column && (value - 1.0).abs() > PSD_TOLERANCE {
                    return Err(SpectreStatisticsError::CorrelationMatrix(format!(
                        "diagonal entry ({},{}) must equal 1, got {value}",
                        row + 1,
                        column + 1
                    )));
                }
                if (value - values[column][row]).abs() > PSD_TOLERANCE {
                    return Err(SpectreStatisticsError::CorrelationMatrix(format!(
                        "entries ({},{}) and ({},{}) are not symmetric",
                        row + 1,
                        column + 1,
                        column + 1,
                        row + 1
                    )));
                }
            }
        }

        let mut lower = vec![vec![0.0; size]; size];
        for row in 0..size {
            for column in 0..=row {
                let product_sum = (0..column)
                    .map(|index| lower[row][index] * lower[column][index])
                    .sum::<Value>();
                let residual = values[row][column] - product_sum;
                if row == column {
                    if residual < -PSD_TOLERANCE {
                        return Err(SpectreStatisticsError::CorrelationMatrix(format!(
                            "matrix is not positive semidefinite (negative pivot {residual} at row {})",
                            row + 1
                        )));
                    }
                    lower[row][column] = libm::sqrt(residual.max(0.0));
                } else if lower[column][column] > PSD_TOLERANCE {
                    lower[row][column] = residual / lower[column][column];
                } else if residual.abs() > PSD_TOLERANCE {
                    return Err(SpectreStatisticsError::CorrelationMatrix(format!(
                        "matrix is not positive semidefinite (inconsistent zero pivot at ({},{}))",
                        row + 1,
                        column + 1
                    )));
                }
            }
        }
        Ok(Self { values, lower })
    }

    pub fn values(&self) -> &[Vec<Value>] {
        &self.values
    }

    fn correlate(&self, independent: &[Value]) -> Vec<Value> {
        self.lower
            .iter()
            .map(|row| {
                row.iter()
                    .zip(independent)
                    .map(|(coefficient, draw)| coefficient * draw)
                    .sum()
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct ResolvedVariation<'a> {
    source: &'a SpectreVariation,
    nominal: Value,
    spread: Value,
}

impl SpectreStatisticsPlan {
    pub(crate) fn validate_structure(&self) -> Result<(), SpectreStatisticsError> {
        let mut declarations = BTreeSet::new();
        for variation in &self.variations {
            let parameter = canonical_parameter(&variation.parameter, variation.line)?;
            if !declarations.insert((variation.scope, parameter.clone())) {
                return Err(invalid(
                    variation.line,
                    format!(
                        "Spectre {:?} variation '{}' is declared more than once",
                        variation.scope, variation.parameter
                    ),
                ));
            }
            match (&variation.distribution, &variation.spread) {
                (SpectreDistribution::Uniform, SpectreSpread::HalfRange(_))
                | (
                    SpectreDistribution::Gaussian | SpectreDistribution::Lognormal,
                    SpectreSpread::StandardDeviation(_),
                ) => {}
                (SpectreDistribution::Uniform, _) => {
                    return Err(invalid(
                        variation.line,
                        format!(
                            "uniform variation '{}' requires N= (half range), not std=",
                            variation.parameter
                        ),
                    ));
                }
                (_, _) => {
                    return Err(invalid(
                        variation.line,
                        format!(
                            "Gaussian/lognormal variation '{}' requires std=, not N=",
                            variation.parameter
                        ),
                    ));
                }
            }
        }

        for correlation in &self.correlations {
            if correlation.parameters.len() < 2 {
                return Err(invalid(
                    correlation.line,
                    "Spectre correlation requires at least two parameters".to_owned(),
                ));
            }
            let mut local = BTreeSet::new();
            for parameter in &correlation.parameters {
                let parameter = canonical_parameter(parameter, correlation.line)?;
                if !local.insert(parameter.clone()) {
                    return Err(invalid(
                        correlation.line,
                        format!("correlation repeats parameter '{parameter}'"),
                    ));
                }
                if !declarations.contains(&(correlation.scope, parameter.clone())) {
                    return Err(invalid(
                        correlation.line,
                        format!(
                            "correlation references undeclared {:?} variation '{parameter}'",
                            correlation.scope
                        ),
                    ));
                }
            }
        }
        Ok(())
    }

    pub(crate) fn references_parameter(&self, expression: &str) -> bool {
        expression_identifiers(expression).any(|identifier| {
            self.variations
                .iter()
                .any(|variation| variation.parameter.eq_ignore_ascii_case(identifier))
        })
    }

    pub(crate) fn sample_process(
        &self,
        params: &ParamContext,
        coordinate: &SpectreStatisticalCoordinate,
    ) -> Result<BTreeMap<String, Value>, SpectreStatisticsError> {
        self.sample_scope(
            SpectreVariationScope::Process,
            params,
            &BTreeMap::new(),
            None,
            coordinate,
        )
    }

    pub(crate) fn sample_mismatch(
        &self,
        params: &ParamContext,
        process: &BTreeMap<String, Value>,
        instance: &str,
        coordinate: &SpectreStatisticalCoordinate,
    ) -> Result<BTreeMap<String, Value>, SpectreStatisticsError> {
        if instance.trim().is_empty() {
            return Err(SpectreStatisticsError::InvalidDeclaration {
                line: 0,
                message: "mismatch sampling requires a non-empty instance identity".to_owned(),
            });
        }
        self.sample_scope(
            SpectreVariationScope::Mismatch,
            params,
            process,
            Some(instance),
            coordinate,
        )
    }

    fn sample_scope(
        &self,
        scope: SpectreVariationScope,
        params: &ParamContext,
        process: &BTreeMap<String, Value>,
        instance: Option<&str>,
        coordinate: &SpectreStatisticalCoordinate,
    ) -> Result<BTreeMap<String, Value>, SpectreStatisticsError> {
        self.validate_structure()?;
        let mut variations = self
            .variations
            .iter()
            .filter(|variation| variation.scope == scope)
            .map(|variation| {
                let key = variation.parameter.to_ascii_uppercase();
                let nominal = process
                    .get(&key)
                    .copied()
                    .or_else(|| params.get(&variation.parameter))
                    .ok_or_else(|| {
                        invalid(
                            variation.line,
                            format!(
                                "Spectre variation '{}' has no nominal parameter value",
                                variation.parameter
                            ),
                        )
                    })?;
                if !nominal.is_finite() {
                    return Err(invalid(
                        variation.line,
                        format!(
                            "Spectre variation '{}' nominal value is not finite",
                            variation.parameter
                        ),
                    ));
                }
                let authored = evaluate_finite(variation.spread.expression(), params)?;
                if authored < 0.0 {
                    return Err(invalid(
                        variation.line,
                        format!(
                            "Spectre variation '{}' spread must be non-negative, got {authored}",
                            variation.parameter
                        ),
                    ));
                }
                let spread = if variation.percent {
                    nominal.abs() * authored / 100.0
                } else {
                    authored
                };
                if variation.distribution == SpectreDistribution::Lognormal && nominal <= 0.0 {
                    return Err(invalid(
                        variation.line,
                        format!(
                            "lognormal variation '{}' requires a positive nominal value, got {nominal}",
                            variation.parameter
                        ),
                    ));
                }
                Ok(ResolvedVariation {
                    source: variation,
                    nominal,
                    spread,
                })
            })
            .collect::<Result<Vec<_>, SpectreStatisticsError>>()?;
        variations.sort_by(|left, right| {
            left.source
                .parameter
                .to_ascii_uppercase()
                .cmp(&right.source.parameter.to_ascii_uppercase())
        });
        if variations.is_empty() {
            return Ok(BTreeMap::new());
        }

        let stream = coordinate_stream_key(coordinate, scope, instance);
        if !self
            .correlations
            .iter()
            .any(|correlation| correlation.scope == scope)
        {
            // Independent variations are the overwhelmingly common case.
            // Keep this path linear in the number of variables and keyed by
            // canonical identity; constructing an identity matrix and
            // factorizing it would add quadratic storage and cubic work
            // without changing a single draw.
            let mut samples = BTreeMap::new();
            for variation in &variations {
                let draw = keyed_standard_normal(
                    stream,
                    variation_identity(variation.source.parameter.as_str()),
                );
                samples.insert(
                    variation.source.parameter.to_ascii_uppercase(),
                    sample_resolved_variation(variation, draw)?,
                );
            }
            return Ok(samples);
        }

        let target = self.target_correlation_matrix(scope, &variations, params)?;
        // Validate the user-authored matrix before translating it into the
        // Gaussian copula's latent correlation space.
        SpectreCorrelationMatrix::new(target.clone())?;
        let latent = latent_correlation_matrix(&variations, &target)?;
        let factor = SpectreCorrelationMatrix::new(latent)?;

        let independent = variations
            .iter()
            .map(|variation| {
                keyed_standard_normal(
                    stream,
                    variation_identity(variation.source.parameter.as_str()),
                )
            })
            .collect::<Vec<_>>();
        let latent = factor.correlate(&independent);
        let mut samples = BTreeMap::new();
        for (variation, draw) in variations.iter().zip(latent) {
            samples.insert(
                variation.source.parameter.to_ascii_uppercase(),
                sample_resolved_variation(variation, draw)?,
            );
        }
        Ok(samples)
    }

    fn target_correlation_matrix(
        &self,
        scope: SpectreVariationScope,
        variations: &[ResolvedVariation<'_>],
        params: &ParamContext,
    ) -> Result<Vec<Vec<Value>>, SpectreStatisticsError> {
        let size = variations.len();
        let indices = variations
            .iter()
            .enumerate()
            .map(|(index, variation)| (variation.source.parameter.to_ascii_uppercase(), index))
            .collect::<BTreeMap<_, _>>();
        let mut matrix = vec![vec![0.0; size]; size];
        for (index, row) in matrix.iter_mut().enumerate() {
            row[index] = 1.0;
        }
        let mut assigned = BTreeMap::<(usize, usize), (Value, usize)>::new();
        for correlation in self
            .correlations
            .iter()
            .filter(|correlation| correlation.scope == scope)
        {
            let coefficient = evaluate_finite(&correlation.coefficient, params)?;
            if !(-1.0..=1.0).contains(&coefficient) {
                return Err(invalid(
                    correlation.line,
                    format!(
                        "Spectre correlation coefficient must be in [-1, 1], got {coefficient}"
                    ),
                ));
            }
            let correlation_indices = correlation
                .parameters
                .iter()
                .map(|parameter| {
                    indices
                        .get(&parameter.to_ascii_uppercase())
                        .copied()
                        .ok_or_else(|| {
                            invalid(
                                correlation.line,
                                format!("correlation parameter '{parameter}' is unavailable"),
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            for left in 0..correlation_indices.len() {
                for right in left + 1..correlation_indices.len() {
                    let pair = ordered_pair(correlation_indices[left], correlation_indices[right]);
                    if let Some((existing, line)) = assigned.get(&pair)
                        && (*existing - coefficient).abs() > PSD_TOLERANCE
                    {
                        return Err(invalid(
                            correlation.line,
                            format!(
                                "correlation for one parameter pair conflicts with coefficient {existing} declared at line {line}"
                            ),
                        ));
                    }
                    assigned.insert(pair, (coefficient, correlation.line));
                    matrix[pair.0][pair.1] = coefficient;
                    matrix[pair.1][pair.0] = coefficient;
                }
            }
        }
        Ok(matrix)
    }

    /// Compact, whitespace-free representation used only between the Spectre
    /// adapter and the canonical parser.  It is versioned and decoded
    /// strictly so malformed or future payloads cannot become inert metadata.
    pub(crate) fn encode_internal(&self) -> String {
        let mut records = vec![STATISTICS_ENCODING_VERSION.to_owned()];
        for variation in &self.variations {
            let (spread_kind, spread) = match &variation.spread {
                SpectreSpread::StandardDeviation(value) => ("S", value),
                SpectreSpread::HalfRange(value) => ("N", value),
            };
            records.push(format!(
                "V,{},{},{},{},{},{},{}",
                variation.line,
                scope_code(variation.scope),
                distribution_code(variation.distribution),
                u8::from(variation.percent),
                hex_encode(&variation.parameter),
                spread_kind,
                hex_encode(spread)
            ));
        }
        for correlation in &self.correlations {
            records.push(format!(
                "C,{},{},{},{},{}",
                correlation.line,
                scope_code(correlation.scope),
                correlation.parameters.len(),
                correlation
                    .parameters
                    .iter()
                    .map(|parameter| hex_encode(parameter))
                    .collect::<Vec<_>>()
                    .join("."),
                hex_encode(&correlation.coefficient)
            ));
        }
        records.join("~")
    }

    /// Decode one `.RSPICE_SPECTRE_STAT` line, or report that the line is not
    /// one.
    ///
    /// This is the only place the internal carrier documented at the top of
    /// this module is interpreted. `rest` is everything after the directive
    /// token; it must be exactly one versioned payload.
    pub(crate) fn decode_directive(line: &str) -> Option<Result<Self, SpectreStatisticsError>> {
        let mut fields = line.trim().splitn(2, char::is_whitespace);
        if !fields
            .next()
            .is_some_and(|command| command.eq_ignore_ascii_case(SPECTRE_STATISTICS_DIRECTIVE))
        {
            return None;
        }
        let mut payload = fields.next().unwrap_or_default().split_whitespace();
        let Some(encoded) = payload.next() else {
            return Some(Err(SpectreStatisticsError::InvalidEncoding(format!(
                "{SPECTRE_STATISTICS_DIRECTIVE} requires one versioned payload"
            ))));
        };
        if payload.next().is_some() {
            return Some(Err(SpectreStatisticsError::InvalidEncoding(format!(
                "{SPECTRE_STATISTICS_DIRECTIVE} accepts exactly one versioned payload"
            ))));
        }
        Some(Self::decode_internal(encoded))
    }

    fn decode_internal(payload: &str) -> Result<Self, SpectreStatisticsError> {
        let mut records = payload.split('~');
        if records.next() != Some(STATISTICS_ENCODING_VERSION) {
            return Err(SpectreStatisticsError::InvalidEncoding(
                "unsupported or missing version".to_owned(),
            ));
        }
        let mut plan = Self::default();
        for record in records {
            let fields = record.split(',').collect::<Vec<_>>();
            match fields.first().copied() {
                Some("V") if fields.len() == 8 => {
                    let line = parse_usize(fields[1], "variation line")?;
                    let scope = parse_scope(fields[2])?;
                    let distribution = parse_distribution(fields[3])?;
                    let percent = match fields[4] {
                        "0" => false,
                        "1" => true,
                        _ => {
                            return Err(SpectreStatisticsError::InvalidEncoding(
                                "variation percent flag must be 0 or 1".to_owned(),
                            ));
                        }
                    };
                    let parameter = hex_decode(fields[5])?;
                    let spread_value = hex_decode(fields[7])?;
                    let spread = match fields[6] {
                        "S" => SpectreSpread::StandardDeviation(spread_value),
                        "N" => SpectreSpread::HalfRange(spread_value),
                        _ => {
                            return Err(SpectreStatisticsError::InvalidEncoding(
                                "unknown spread kind".to_owned(),
                            ));
                        }
                    };
                    plan.variations.push(SpectreVariation {
                        line,
                        scope,
                        parameter,
                        distribution,
                        spread,
                        percent,
                    });
                }
                Some("C") if fields.len() == 6 => {
                    let line = parse_usize(fields[1], "correlation line")?;
                    let scope = parse_scope(fields[2])?;
                    let count = parse_usize(fields[3], "correlation parameter count")?;
                    let parameters = if fields[4].is_empty() {
                        Vec::new()
                    } else {
                        fields[4]
                            .split('.')
                            .map(hex_decode)
                            .collect::<Result<Vec<_>, _>>()?
                    };
                    if parameters.len() != count {
                        return Err(SpectreStatisticsError::InvalidEncoding(format!(
                            "correlation declares {count} parameters but encodes {}",
                            parameters.len()
                        )));
                    }
                    plan.correlations.push(SpectreCorrelation {
                        line,
                        scope,
                        parameters,
                        coefficient: hex_decode(fields[5])?,
                    });
                }
                _ => {
                    return Err(SpectreStatisticsError::InvalidEncoding(format!(
                        "malformed record '{record}'"
                    )));
                }
            }
        }
        plan.validate_structure()?;
        Ok(plan)
    }
}

fn variation_identity(parameter: &str) -> u64 {
    hash_bytes(
        0x5354_4154_5F56_4152,
        parameter.to_ascii_uppercase().as_bytes(),
    )
}

fn sample_resolved_variation(
    variation: &ResolvedVariation<'_>,
    draw: Value,
) -> Result<Value, SpectreStatisticsError> {
    let value = match variation.source.distribution {
        SpectreDistribution::Gaussian => variation.nominal + variation.spread * draw,
        SpectreDistribution::Uniform => {
            variation.nominal + variation.spread * (2.0 * standard_normal_cdf(draw) - 1.0)
        }
        SpectreDistribution::Lognormal => {
            // Spectre defines `std` in log space: log(x) is normal with mean
            // log(nominal) and standard deviation `std`. The authored nominal
            // is therefore the median, not the arithmetic mean.
            if variation.spread == 0.0 {
                variation.nominal
            } else {
                libm::exp(libm::log(variation.nominal) + variation.spread * draw)
            }
        }
    };
    if !value.is_finite() {
        return Err(invalid(
            variation.source.line,
            format!(
                "Spectre variation '{}' produced a non-finite value",
                variation.source.parameter
            ),
        ));
    }
    Ok(value)
}

fn latent_correlation_matrix(
    variations: &[ResolvedVariation<'_>],
    target: &[Vec<Value>],
) -> Result<Vec<Vec<Value>>, SpectreStatisticsError> {
    let size = variations.len();
    let mut latent = vec![vec![0.0; size]; size];
    for (index, row) in latent.iter_mut().enumerate().take(size) {
        row[index] = 1.0;
    }
    for left in 0..size {
        for right in left + 1..size {
            let value = latent_pair_correlation(
                &variations[left],
                &variations[right],
                target[left][right],
            )?;
            if !value.is_finite() || !(-1.0 - PSD_TOLERANCE..=1.0 + PSD_TOLERANCE).contains(&value)
            {
                return Err(SpectreStatisticsError::CorrelationMatrix(format!(
                    "requested correlation {} between '{}' and '{}' is infeasible for their distributions",
                    target[left][right],
                    variations[left].source.parameter,
                    variations[right].source.parameter
                )));
            }
            latent[left][right] = value.clamp(-1.0, 1.0);
            latent[right][left] = latent[left][right];
        }
    }
    Ok(latent)
}

fn latent_pair_correlation(
    left: &ResolvedVariation<'_>,
    right: &ResolvedVariation<'_>,
    target: Value,
) -> Result<Value, SpectreStatisticsError> {
    use SpectreDistribution::{Gaussian, Lognormal, Uniform};
    if target == 0.0 {
        return Ok(0.0);
    }
    if left.spread == 0.0 || right.spread == 0.0 {
        let constant = if left.spread == 0.0 { left } else { right };
        return Err(invalid(
            constant.source.line,
            format!(
                "nonzero correlation involving zero-spread variation '{}' is undefined",
                constant.source.parameter
            ),
        ));
    }
    let left_distribution = left.source.distribution;
    let right_distribution = right.source.distribution;
    let latent = match (left_distribution, right_distribution) {
        (Gaussian, Gaussian) => target,
        (Uniform, Uniform) => 2.0 * libm::sin(std::f64::consts::PI * target / 6.0),
        (Lognormal, Lognormal) => {
            let left_sigma = lognormal_sigma(left)?;
            let right_sigma = lognormal_sigma(right)?;
            let left_variance = left_sigma * left_sigma;
            let right_variance = right_sigma * right_sigma;
            let scale = libm::sqrt(libm::expm1(left_variance) * libm::expm1(right_variance));
            libm::log(target * scale + 1.0) / (left_sigma * right_sigma)
        }
        (Gaussian, Uniform) | (Uniform, Gaussian) => {
            target * libm::sqrt(std::f64::consts::PI / 3.0)
        }
        (Gaussian, Lognormal) => gaussian_lognormal_latent(target, right)?,
        (Lognormal, Gaussian) => gaussian_lognormal_latent(target, left)?,
        (Uniform, Lognormal) => uniform_lognormal_latent(target, right)?,
        (Lognormal, Uniform) => uniform_lognormal_latent(target, left)?,
    };
    Ok(latent)
}

fn lognormal_sigma(variation: &ResolvedVariation<'_>) -> Result<Value, SpectreStatisticsError> {
    if variation.spread <= 0.0 {
        return Err(invalid(
            variation.source.line,
            format!(
                "correlated lognormal variation '{}' requires std>0",
                variation.source.parameter
            ),
        ));
    }
    Ok(variation.spread)
}

fn gaussian_lognormal_latent(
    target: Value,
    lognormal: &ResolvedVariation<'_>,
) -> Result<Value, SpectreStatisticsError> {
    let sigma = lognormal_sigma(lognormal)?;
    Ok(target * libm::sqrt(libm::expm1(sigma * sigma)) / sigma)
}

fn uniform_lognormal_latent(
    target: Value,
    lognormal: &ResolvedVariation<'_>,
) -> Result<Value, SpectreStatisticsError> {
    let sigma = lognormal_sigma(lognormal)?;
    let probability = 0.5 + target * libm::sqrt(libm::expm1(sigma * sigma)) / libm::sqrt(12.0);
    if !(0.0..1.0).contains(&probability) {
        return Ok(Value::NAN);
    }
    Ok(libm::sqrt(2.0) * inverse_standard_normal_cdf(probability) / sigma)
}

fn evaluate_finite(
    expression: &str,
    params: &ParamContext,
) -> Result<Value, SpectreStatisticsError> {
    let value = eval_expression(expression, params).map_err(|error| {
        SpectreStatisticsError::Expression {
            expression: expression.to_owned(),
            reason: error.to_string(),
        }
    })?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SpectreStatisticsError::Expression {
            expression: expression.to_owned(),
            reason: "result is not finite".to_owned(),
        })
    }
}

fn coordinate_stream_key(
    coordinate: &SpectreStatisticalCoordinate,
    scope: SpectreVariationScope,
    instance: Option<&str>,
) -> u64 {
    let mut hash = splitmix64(coordinate.seed ^ 0xA076_1D64_78BD_642F);
    hash = hash_combine(hash, coordinate.monte_carlo_run);
    hash = hash_combine(hash, coordinate.temperature_celsius.to_bits());
    let mut axes = coordinate.axes.clone();
    axes.sort_by(|left, right| {
        left.0
            .to_ascii_uppercase()
            .cmp(&right.0.to_ascii_uppercase())
            .then_with(|| left.1.total_cmp(&right.1))
    });
    for (name, value) in axes {
        hash = hash_bytes(hash, name.to_ascii_uppercase().as_bytes());
        hash = hash_combine(hash, value.to_bits());
    }
    hash = hash_combine(
        hash,
        match scope {
            SpectreVariationScope::Process => 0x5052_4F43_4553_5301,
            SpectreVariationScope::Mismatch => 0x4D49_534D_4154_4302,
        },
    );
    if let Some(instance) = instance {
        hash = hash_bytes(hash, instance.to_ascii_uppercase().as_bytes());
    }
    hash
}

fn keyed_standard_normal(stream: u64, identity: u64) -> Value {
    let first = splitmix64(stream ^ identity.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let second = splitmix64(stream ^ identity.wrapping_mul(0xD1B5_4A32_D192_ED03) ^ 1);
    let u1 = 1.0 - uniform_from_bits(first);
    let u2 = uniform_from_bits(second);
    libm::sqrt(-2.0 * libm::log(u1)) * libm::cos(std::f64::consts::TAU * u2)
}

fn uniform_from_bits(bits: u64) -> Value {
    (bits >> 11) as Value * (1.0 / 9_007_199_254_740_992.0)
}

fn splitmix64(mut value: u64) -> u64 {
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

fn hash_combine(state: u64, value: u64) -> u64 {
    splitmix64(state ^ value.wrapping_add(0x9E37_79B9_7F4A_7C15))
}

fn hash_bytes(mut state: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        state = hash_combine(state, u64::from(*byte));
    }
    state
}

fn standard_normal_cdf(value: Value) -> Value {
    // Abramowitz and Stegun 7.1.26.  The maximum absolute error is below
    // 7.5e-8, ample for transforming a 53-bit latent draw without distorting
    // statistical-property tolerances.
    let x = value.abs();
    let t = 1.0 / (1.0 + 0.231_641_9 * x);
    let density = libm::exp(-0.5 * x * x) / libm::sqrt(2.0 * std::f64::consts::PI);
    let tail = density
        * t
        * (0.319_381_530
            + t * (-0.356_563_782
                + t * (1.781_477_937 + t * (-1.821_255_978 + t * 1.330_274_429))));
    if value >= 0.0 { 1.0 - tail } else { tail }
}

fn inverse_standard_normal_cdf(probability: Value) -> Value {
    // Peter J. Acklam's rational approximation.  One Halley refinement gives
    // full double precision for the interior probabilities used here.
    const A: [Value; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];
    const B: [Value; 5] = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];
    const C: [Value; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];
    const D: [Value; 4] = [
        7.784_695_709_041_462e-3,
        3.224_671_290_700_398e-1,
        2.445_134_137_142_996,
        3.754_408_661_907_416,
    ];
    let mut value = if probability < 0.024_25 {
        let q = libm::sqrt(-2.0 * libm::log(probability));
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if probability > 1.0 - 0.024_25 {
        let q = libm::sqrt(-2.0 * libm::log(1.0 - probability));
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else {
        let q = probability - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    };
    let error = standard_normal_cdf(value) - probability;
    let density = libm::exp(-0.5 * value * value) / libm::sqrt(2.0 * std::f64::consts::PI);
    value -= error / (density + 0.5 * value * error);
    value
}

fn expression_identifiers(expression: &str) -> impl Iterator<Item = &str> {
    expression
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|token| {
            token
                .chars()
                .next()
                .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        })
}

fn canonical_parameter(parameter: &str, line: usize) -> Result<String, SpectreStatisticsError> {
    let mut chars = parameter.chars();
    if !chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        || !chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return Err(invalid(
            line,
            format!("'{parameter}' is not a portable statistical parameter name"),
        ));
    }
    Ok(parameter.to_ascii_uppercase())
}

fn invalid(line: usize, message: String) -> SpectreStatisticsError {
    SpectreStatisticsError::InvalidDeclaration { line, message }
}

fn ordered_pair(left: usize, right: usize) -> (usize, usize) {
    if left <= right {
        (left, right)
    } else {
        (right, left)
    }
}

fn scope_code(scope: SpectreVariationScope) -> &'static str {
    match scope {
        SpectreVariationScope::Process => "P",
        SpectreVariationScope::Mismatch => "M",
    }
}

fn parse_scope(value: &str) -> Result<SpectreVariationScope, SpectreStatisticsError> {
    match value {
        "P" => Ok(SpectreVariationScope::Process),
        "M" => Ok(SpectreVariationScope::Mismatch),
        _ => Err(SpectreStatisticsError::InvalidEncoding(
            "unknown variation scope".to_owned(),
        )),
    }
}

fn distribution_code(distribution: SpectreDistribution) -> &'static str {
    match distribution {
        SpectreDistribution::Gaussian => "G",
        SpectreDistribution::Uniform => "U",
        SpectreDistribution::Lognormal => "L",
    }
}

fn parse_distribution(value: &str) -> Result<SpectreDistribution, SpectreStatisticsError> {
    match value {
        "G" => Ok(SpectreDistribution::Gaussian),
        "U" => Ok(SpectreDistribution::Uniform),
        "L" => Ok(SpectreDistribution::Lognormal),
        _ => Err(SpectreStatisticsError::InvalidEncoding(
            "unknown distribution".to_owned(),
        )),
    }
}

fn parse_usize(value: &str, label: &str) -> Result<usize, SpectreStatisticsError> {
    value
        .parse()
        .map_err(|_| SpectreStatisticsError::InvalidEncoding(format!("invalid {label} '{value}'")))
}

fn hex_encode(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn hex_decode(value: &str) -> Result<String, SpectreStatisticsError> {
    if !value.len().is_multiple_of(2) {
        return Err(SpectreStatisticsError::InvalidEncoding(
            "hex field has odd length".to_owned(),
        ));
    }
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks_exact(2) {
        let high = hex_digit(pair[0])?;
        let low = hex_digit(pair[1])?;
        decoded.push((high << 4) | low);
    }
    String::from_utf8(decoded)
        .map_err(|_| SpectreStatisticsError::InvalidEncoding("field is not UTF-8".to_owned()))
}

fn hex_digit(value: u8) -> Result<u8, SpectreStatisticsError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(SpectreStatisticsError::InvalidEncoding(
            "field contains non-hexadecimal data".to_owned(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn variation(
        parameter: &str,
        distribution: SpectreDistribution,
        spread: Value,
    ) -> SpectreVariation {
        SpectreVariation {
            line: 1,
            scope: SpectreVariationScope::Process,
            parameter: parameter.to_owned(),
            distribution,
            spread: match distribution {
                SpectreDistribution::Uniform => SpectreSpread::HalfRange(spread.to_string()),
                _ => SpectreSpread::StandardDeviation(spread.to_string()),
            },
            percent: false,
        }
    }

    #[test]
    fn internal_encoding_round_trips_without_source_escaping() {
        let plan = SpectreStatisticsPlan {
            variations: vec![SpectreVariation {
                spread: SpectreSpread::StandardDeviation("sigma * 2".to_owned()),
                ..variation("dvth", SpectreDistribution::Gaussian, 1.0)
            }],
            correlations: vec![],
        };
        assert_eq!(
            SpectreStatisticsPlan::decode_internal(&plan.encode_internal()).unwrap(),
            plan
        );
    }

    #[test]
    fn mixed_case_internal_directive_with_malformed_payload_fails_closed() {
        let error = crate::Netlist::parse(
            "malformed internal statistics\n.RsPiCe_SpEcTrE_StAt future-version\n.end\n",
        )
        .expect_err("mixed-case internal directives must be decoded, not consumed as metadata");
        assert!(
            error
                .to_string()
                .contains("internal Spectre statistics encoding")
        );
    }

    #[test]
    fn correlation_matrix_rejects_asymmetry_and_indefinite_inputs() {
        assert!(SpectreCorrelationMatrix::new(vec![vec![1.0, 0.4], vec![0.2, 1.0]]).is_err());
        let indefinite = vec![
            vec![1.0, 0.9, 0.9],
            vec![0.9, 1.0, -0.9],
            vec![0.9, -0.9, 1.0],
        ];
        assert!(SpectreCorrelationMatrix::new(indefinite).is_err());
        assert!(
            SpectreCorrelationMatrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]).is_ok(),
            "positive semidefinite singular matrices are valid"
        );
    }

    #[test]
    fn nonzero_correlation_with_a_zero_spread_variable_fails_closed() {
        let plan = SpectreStatisticsPlan {
            variations: vec![
                variation("constant", SpectreDistribution::Gaussian, 0.0),
                variation("random", SpectreDistribution::Uniform, 1.0),
            ],
            correlations: vec![SpectreCorrelation {
                line: 3,
                scope: SpectreVariationScope::Process,
                parameters: vec!["constant".into(), "random".into()],
                coefficient: "0.5".into(),
            }],
        };
        let mut params = ParamContext::new();
        params.set("constant", 1.0);
        params.set("random", 2.0);
        let error = plan
            .sample_process(&params, &SpectreStatisticalCoordinate::default())
            .expect_err("a constant variable has no Pearson correlation");
        assert!(
            error
                .to_string()
                .contains("zero-spread variation 'constant'")
        );
    }

    #[test]
    fn seeded_sequence_is_exact_and_coordinate_order_independent() {
        let plan = SpectreStatisticsPlan {
            variations: vec![
                variation("a", SpectreDistribution::Gaussian, 2.0),
                variation("b", SpectreDistribution::Uniform, 3.0),
                variation("c", SpectreDistribution::Lognormal, 0.2),
            ],
            correlations: vec![],
        };
        let mut params = ParamContext::new();
        params.set("a", 10.0);
        params.set("b", 20.0);
        params.set("c", 5.0);
        let first = SpectreStatisticalCoordinate {
            seed: 42,
            monte_carlo_run: 7,
            temperature_celsius: 85.0,
            axes: vec![("W".into(), 2.0), ("L".into(), 1.0)],
        };
        let reordered = SpectreStatisticalCoordinate {
            axes: vec![("l".into(), 1.0), ("w".into(), 2.0)],
            ..first.clone()
        };
        let samples = plan.sample_process(&params, &first).unwrap();
        assert_eq!(samples, plan.sample_process(&params, &reordered).unwrap());
        // This pins the cross-platform counter/substream contract.
        assert_eq!(samples["A"].to_bits(), 4_622_626_497_987_086_581);
        assert_eq!(samples["B"].to_bits(), 4_626_002_768_258_348_166);
        assert_eq!(samples["C"].to_bits(), 4_617_388_684_598_768_948);
        let next_run = SpectreStatisticalCoordinate {
            monte_carlo_run: first.monte_carlo_run + 1,
            ..first.clone()
        };
        let next_temperature = SpectreStatisticalCoordinate {
            temperature_celsius: first.temperature_celsius + 1.0,
            ..first.clone()
        };
        let next_axis = SpectreStatisticalCoordinate {
            axes: vec![("L".into(), 1.0), ("W".into(), 3.0)],
            ..first.clone()
        };
        assert_ne!(samples, plan.sample_process(&params, &next_run).unwrap());
        assert_ne!(
            samples,
            plan.sample_process(&params, &next_temperature).unwrap()
        );
        assert_ne!(samples, plan.sample_process(&params, &next_axis).unwrap());
    }

    #[test]
    fn unrelated_variation_does_not_shift_existing_variable_substreams() {
        let base = SpectreStatisticsPlan {
            variations: vec![
                variation("b", SpectreDistribution::Gaussian, 1.0),
                variation("c", SpectreDistribution::Uniform, 2.0),
            ],
            correlations: vec![],
        };
        let mut extended = base.clone();
        extended
            .variations
            .push(variation("a", SpectreDistribution::Lognormal, 0.5));
        let mut params = ParamContext::new();
        params.set("a", 5.0);
        params.set("b", 10.0);
        params.set("c", 20.0);
        let coordinate = SpectreStatisticalCoordinate {
            seed: 1234,
            monte_carlo_run: 8,
            temperature_celsius: 27.0,
            axes: vec![],
        };
        let base_sample = base.sample_process(&params, &coordinate).unwrap();
        let extended_sample = extended.sample_process(&params, &coordinate).unwrap();
        assert_eq!(base_sample["B"].to_bits(), extended_sample["B"].to_bits());
        assert_eq!(base_sample["C"].to_bits(), extended_sample["C"].to_bits());
    }

    #[test]
    fn many_independent_variations_sample_linearly_and_replay_exactly() {
        const COUNT: usize = 1_024;
        let mut params = ParamContext::new();
        let mut variations = Vec::with_capacity(COUNT);
        for index in 0..COUNT {
            let name = format!("v{index:04}");
            params.set(&name, 10.0 + index as Value);
            variations.push(variation(&name, SpectreDistribution::Gaussian, 0.5));
        }
        let plan = SpectreStatisticsPlan {
            variations,
            correlations: vec![],
        };
        let coordinate = SpectreStatisticalCoordinate {
            seed: 0x1020_3040,
            monte_carlo_run: 55,
            temperature_celsius: 27.0,
            axes: vec![("outer".into(), 2.0)],
        };

        let first = plan.sample_process(&params, &coordinate).unwrap();
        let replay = plan.sample_process(&params, &coordinate).unwrap();
        assert_eq!(first.len(), COUNT);
        assert_eq!(first, replay);
        assert!(first.values().all(|value| value.is_finite()));
    }

    #[test]
    fn distribution_moments_and_mixed_correlations_match_the_contract() {
        let mut plan = SpectreStatisticsPlan {
            variations: vec![
                variation("g", SpectreDistribution::Gaussian, 2.0),
                variation("u", SpectreDistribution::Uniform, 3.0),
                variation("l", SpectreDistribution::Lognormal, 0.25),
            ],
            correlations: vec![SpectreCorrelation {
                line: 4,
                scope: SpectreVariationScope::Process,
                parameters: vec!["g".into(), "u".into(), "l".into()],
                coefficient: "0.35".into(),
            }],
        };
        let mut params = ParamContext::new();
        params.set("g", 10.0);
        params.set("u", 20.0);
        params.set("l", 5.0);
        let count = 30_000_u64;
        let mut columns = (0..3)
            .map(|_| Vec::with_capacity(count as usize))
            .collect::<Vec<_>>();
        for run in 0..count {
            let values = plan
                .sample_process(
                    &params,
                    &SpectreStatisticalCoordinate {
                        seed: 123,
                        monte_carlo_run: run,
                        temperature_celsius: 27.0,
                        axes: vec![],
                    },
                )
                .unwrap();
            for (column, name) in columns.iter_mut().zip(["G", "U", "L"]) {
                column.push(values[name]);
            }
        }
        let (g_mean, g_std) = moments(&columns[0]);
        let (u_mean, u_std) = moments(&columns[1]);
        let (l_mean, l_std) = moments(&columns[2]);
        assert!((g_mean - 10.0).abs() < 0.05);
        assert!((g_std - 2.0).abs() < 0.05);
        assert!((u_mean - 20.0).abs() < 0.05);
        assert!((u_std - 3.0 / 3.0_f64.sqrt()).abs() < 0.05);
        let expected_l_mean = 5.0 * (0.5_f64 * 0.25 * 0.25).exp();
        let expected_l_std = expected_l_mean * (0.25_f64 * 0.25).exp_m1().sqrt();
        assert!((l_mean - expected_l_mean).abs() < 0.05);
        assert!((l_std - expected_l_std).abs() < 0.05);
        for left in 0..3 {
            for right in left + 1..3 {
                let actual = correlation(&columns[left], &columns[right]);
                assert!((actual - 0.35).abs() < 0.035, "correlation={actual}");
            }
        }
        // Keep ownership explicit so a future test mutation cannot make the
        // plan immutable accidentally and hide a stale-sampler regression.
        plan.correlations.clear();
        assert!(plan.validate_structure().is_ok());
    }

    #[test]
    fn process_is_shared_and_mismatch_is_instance_specific_and_reproducible() {
        let plan = SpectreStatisticsPlan {
            variations: vec![
                variation("x", SpectreDistribution::Gaussian, 1.0),
                SpectreVariation {
                    scope: SpectreVariationScope::Mismatch,
                    ..variation("x", SpectreDistribution::Gaussian, 0.1)
                },
            ],
            correlations: vec![],
        };
        let mut params = ParamContext::new();
        params.set("x", 10.0);
        let coordinate = SpectreStatisticalCoordinate {
            seed: 9,
            monte_carlo_run: 3,
            temperature_celsius: 27.0,
            axes: vec![],
        };
        let process = plan.sample_process(&params, &coordinate).unwrap();
        let first = plan
            .sample_mismatch(&params, &process, "XTOP.M1", &coordinate)
            .unwrap();
        let replay = plan
            .sample_mismatch(&params, &process, "xtop.m1", &coordinate)
            .unwrap();
        let second = plan
            .sample_mismatch(&params, &process, "XTOP.M2", &coordinate)
            .unwrap();
        assert_eq!(first, replay);
        assert_ne!(first, second);
        assert_ne!(first["X"], process["X"]);
    }

    #[test]
    fn correlated_mismatch_matches_pearson_contract_and_replays_by_instance() {
        let plan = SpectreStatisticsPlan {
            variations: vec![
                SpectreVariation {
                    scope: SpectreVariationScope::Mismatch,
                    ..variation("x", SpectreDistribution::Gaussian, 2.0)
                },
                SpectreVariation {
                    scope: SpectreVariationScope::Mismatch,
                    ..variation("y", SpectreDistribution::Gaussian, 3.0)
                },
            ],
            correlations: vec![SpectreCorrelation {
                line: 3,
                scope: SpectreVariationScope::Mismatch,
                parameters: vec!["x".into(), "y".into()],
                coefficient: "0.6".into(),
            }],
        };
        let mut params = ParamContext::new();
        params.set("x", 10.0);
        params.set("y", 20.0);
        let coordinate = SpectreStatisticalCoordinate {
            seed: 55,
            monte_carlo_run: 7,
            temperature_celsius: 27.0,
            axes: vec![],
        };
        let process = plan.sample_process(&params, &coordinate).unwrap();
        let mut x = Vec::with_capacity(20_000);
        let mut y = Vec::with_capacity(20_000);
        for instance in 0..20_000 {
            let identity = format!("X{instance}");
            let sample = plan
                .sample_mismatch(&params, &process, &identity, &coordinate)
                .unwrap();
            if instance == 0 {
                assert_eq!(
                    sample,
                    plan.sample_mismatch(&params, &process, &identity, &coordinate)
                        .unwrap()
                );
            }
            x.push(sample["X"]);
            y.push(sample["Y"]);
        }
        assert!((correlation(&x, &y) - 0.6).abs() < 0.03);
        let (_, x_std) = moments(&x);
        let (_, y_std) = moments(&y);
        assert!((x_std - 2.0).abs() < 0.05);
        assert!((y_std - 3.0).abs() < 0.08);
    }

    fn moments(values: &[Value]) -> (Value, Value) {
        let mean = values.iter().sum::<Value>() / values.len() as Value;
        let variance = values
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<Value>()
            / values.len() as Value;
        (mean, variance.sqrt())
    }

    fn correlation(left: &[Value], right: &[Value]) -> Value {
        let (left_mean, left_std) = moments(left);
        let (right_mean, right_std) = moments(right);
        left.iter()
            .zip(right)
            .map(|(left, right)| (left - left_mean) * (right - right_mean))
            .sum::<Value>()
            / left.len() as Value
            / (left_std * right_std)
    }
}
