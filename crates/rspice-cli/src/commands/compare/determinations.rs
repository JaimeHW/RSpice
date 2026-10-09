//! Explicit scalar determinations are evidence; uncomputed gaps are not.

use super::{WaveformData, parse_variable_name};
use rspice_core::execution::result_document::ScalarUnavailability;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Evidence {
    Unstated,
    Invalid,
    Known(ScalarUnavailability),
}

pub(super) struct Determinations(Vec<Evidence>);

impl Determinations {
    pub(super) fn invalid_names<'a>(
        &'a self,
        data: &'a WaveformData,
    ) -> impl Iterator<Item = &'a str> {
        self.0
            .iter()
            .zip(&data.variables)
            .filter_map(|(evidence, name)| {
                matches!(evidence, Evidence::Invalid).then_some(name.as_str())
            })
    }

    pub(super) fn new(data: &WaveformData) -> Self {
        if !data.variables.iter().any(|name| {
            name.as_bytes()
                .windows(b":unavailable(".len())
                .any(|part| part.eq_ignore_ascii_case(b":unavailable("))
        }) {
            return Self(Vec::new());
        }
        let indices: HashMap<_, _> = data
            .variables
            .iter()
            .enumerate()
            .map(|(index, name)| (parse_variable_name(name).key, index))
            .collect();
        let mut evidence = vec![Evidence::Unstated; data.variables.len()];
        for (column, name) in data.variables.iter().enumerate() {
            let normalized = name.trim().to_ascii_lowercase();
            let Some((quantity, suffix)) = normalized.rsplit_once(":unavailable(") else {
                continue;
            };
            let Some(reason) = suffix.strip_suffix(')') else {
                continue;
            };
            let Some(&metric) = indices.get(&parse_variable_name(quantity).key) else {
                continue;
            };
            let reason = match reason {
                "positive_infinity" => Some(ScalarUnavailability::PositiveInfinity),
                "negative_infinity" => Some(ScalarUnavailability::NegativeInfinity),
                "no_crossover" => Some(ScalarUnavailability::NoCrossover),
                "empty_domain" => Some(ScalarUnavailability::EmptyDomain),
                _ => None,
            };
            // One true indicator, a wholly missing metric, and compatible
            // indicator units are required. Contradictory or duplicate claims
            // cannot turn unknown samples into a successful verification.
            let known = reason.filter(|_| {
                !data.values[column].is_empty()
                    && data.units[column].as_deref().is_none_or(|unit| unit == "1")
                    && super::quantity_type(&data.variable_types[column])
                        .as_deref()
                        .is_none_or(|unit| unit == "1")
                    && (0..data.values[column].len())
                        .all(|row| data.sample(column, row) == Some(1.0))
                    && data.validity[metric]
                        .as_ref()
                        .is_some_and(|mask| mask.iter().all(|&valid| !valid))
            });
            evidence[metric] = match evidence[metric] {
                Evidence::Unstated => known.map_or(Evidence::Invalid, Evidence::Known),
                _ => Evidence::Invalid,
            };
        }
        Self(evidence)
    }

    pub(super) fn agrees(&self, index: usize, other: &Self, other_index: usize) -> bool {
        matches!((self.0.get(index), other.0.get(other_index)), (Some(Evidence::Known(left)), Some(Evidence::Known(right))) if left == right)
    }
}
