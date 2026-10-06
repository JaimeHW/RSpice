//! Signal identity and indexed probe selection for waveform comparison.
use super::{CompareArgs, CompareResult, WaveformData, strip_outer_call};
use std::collections::{HashMap, HashSet};

struct VariableIndex<'a> {
    names: &'a [ParsedVariableName],
    exact: HashMap<&'a VariableKey, usize>,
    aliases: HashMap<&'a str, Vec<usize>>,
}

impl<'a> VariableIndex<'a> {
    fn new(names: &'a [ParsedVariableName], include_aliases: bool) -> Self {
        let mut exact = HashMap::with_capacity(names.len());
        let mut aliases = HashMap::<&str, Vec<usize>>::new();
        for (index, name) in names.iter().enumerate() {
            exact.insert(&name.key, index);
            if include_aliases {
                for alias in &name.aliases {
                    aliases.entry(alias).or_default().push(index);
                }
            }
        }
        Self {
            names,
            exact,
            aliases,
        }
    }

    fn selected(&self, request: &ParsedVariableName) -> Vec<usize> {
        self.aliases
            .get(request.key.base.as_str())
            .into_iter()
            .flatten()
            .copied()
            .filter(|&index| requested_variable_matches(request, &self.names[index]))
            .collect()
    }
}

pub(super) fn pairs(
    result: &WaveformData,
    golden: &WaveformData,
    args: &CompareArgs,
    comparison: &mut CompareResult,
) -> Vec<(usize, usize)> {
    let result_names: Vec<_> = result
        .variables
        .iter()
        .map(|name| parse_variable_name(name))
        .collect();
    let golden_names: Vec<_> = golden
        .variables
        .iter()
        .map(|name| parse_variable_name(name))
        .collect();
    let selected = !args.variables.is_empty();
    let result_index = VariableIndex::new(&result_names, selected);
    let golden_index = VariableIndex::new(&golden_names, selected);
    let mut pairs = if selected {
        explicit_variable_pairs(
            result,
            golden,
            args,
            comparison,
            &result_index,
            &golden_index,
        )
    } else {
        if !args.ignore_missing {
            for (name, parsed) in golden.variables.iter().zip(&golden_names) {
                if !result_index.exact.contains_key(&parsed.key) {
                    comparison
                        .problems
                        .push(format!("variable '{name}' is missing from the result"));
                }
            }
        }
        // Preserve result order for deterministic mismatch previews.
        result_names
            .iter()
            .enumerate()
            .filter_map(|(index, name)| {
                golden_index
                    .exact
                    .get(&name.key)
                    .map(|&golden| (index, golden))
            })
            .collect()
    };

    // Signal selection never discards the independent coordinate contract.
    if result_names[0].key != golden_names[0].key {
        comparison.problems.push(format!(
            "independent coordinates differ: '{}' versus '{}'",
            result.variables[0], golden.variables[0]
        ));
    } else if !pairs.contains(&(0, 0)) {
        pairs.insert(0, (0, 0));
    }
    pairs
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ComplexPart {
    Real,
    Imag,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct VariableKey {
    part: Option<ComplexPart>,
    base: String,
}

#[derive(Clone, Debug)]
pub(super) struct ParsedVariableName {
    pub(super) key: VariableKey,
    aliases: Vec<String>,
}

fn parsed_variable_names_match(left: &ParsedVariableName, right: &ParsedVariableName) -> bool {
    left.key == right.key
}

pub(super) fn variable_name_matches(left: &str, right: &str) -> bool {
    parsed_variable_names_match(&parse_variable_name(left), &parse_variable_name(right))
}

pub(super) fn parse_variable_name(name: &str) -> ParsedVariableName {
    let trimmed = name.trim();
    let (part, base) = if let Some(inner) = strip_outer_call(trimmed, "Re") {
        (Some(ComplexPart::Real), inner)
    } else if let Some(inner) = strip_outer_call(trimmed, "Im") {
        (Some(ComplexPart::Imag), inner)
    } else {
        (None, trimmed)
    };
    let base = normalize_variable_name(base);
    let mut aliases = vec![base.clone()];
    if let Some(inner) = signal_inner_name(base.as_str()) {
        push_alias(&mut aliases, normalize_variable_name(inner));
    }
    ParsedVariableName {
        key: VariableKey { part, base },
        aliases,
    }
}

fn normalize_variable_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

fn signal_inner_name(name: &str) -> Option<&str> {
    let (prefix, rest) = name.split_once('(')?;
    if prefix.eq_ignore_ascii_case("v") || prefix.eq_ignore_ascii_case("i") {
        return rest.strip_suffix(')').map(str::trim);
    }
    None
}

fn push_alias(aliases: &mut Vec<String>, alias: String) {
    if !aliases.iter().any(|existing| existing == &alias) {
        aliases.push(alias);
    }
}

fn requested_variable_matches(
    request: &ParsedVariableName,
    candidate: &ParsedVariableName,
) -> bool {
    if request.key.part.is_some() && request.key.part != candidate.key.part {
        return false;
    }
    // Quantity-qualified requests retain their meaning. A bare selector may
    // use a signal's alias, subject to the ambiguity check below.
    if signal_inner_name(&request.key.base).is_some() {
        request.key.base == candidate.key.base
    } else {
        candidate.aliases.contains(&request.key.base)
    }
}

fn explicit_variable_pairs(
    result: &WaveformData,
    golden: &WaveformData,
    args: &CompareArgs,
    cmp_result: &mut CompareResult,
    result_index: &VariableIndex<'_>,
    golden_index: &VariableIndex<'_>,
) -> Vec<(usize, usize)> {
    let result_names = result_index.names;
    let golden_names = golden_index.names;

    let mut pairs = Vec::new();
    let mut seen = HashSet::new();

    for requested in &args.variables {
        let request = parse_variable_name(requested);
        let result_indices = result_index.selected(&request);
        let golden_indices = golden_index.selected(&request);

        if signal_inner_name(&request.key.base).is_none() {
            let meanings: HashSet<_> = result_indices
                .iter()
                .map(|&index| &result_names[index].key.base)
                .chain(
                    golden_indices
                        .iter()
                        .map(|&index| &golden_names[index].key.base),
                )
                .collect();
            if meanings.len() > 1 {
                cmp_result.problems.push(format!(
                    "variable selector '{requested}' is ambiguous; use a quantity-qualified name such as V({requested}) or I({requested})"
                ));
                continue;
            }
        }

        if result_indices.is_empty() || golden_indices.is_empty() {
            if !args.ignore_missing {
                if result_indices.is_empty() {
                    cmp_result
                        .problems
                        .push(format!("variable '{requested}' is missing from the result"));
                }
                if golden_indices.is_empty() {
                    cmp_result.problems.push(format!(
                        "variable '{requested}' is missing from the golden file"
                    ));
                }
            }
            continue;
        }

        let mut matched_golden = HashSet::new();

        for result_index in result_indices {
            let mut matched = false;
            for &golden_index in &golden_indices {
                if parsed_variable_names_match(
                    &result_names[result_index],
                    &golden_names[golden_index],
                ) {
                    matched = true;
                    matched_golden.insert(golden_index);
                    if seen.insert((result_index, golden_index)) {
                        pairs.push((result_index, golden_index));
                    }
                }
            }
            if !matched && !args.ignore_missing {
                cmp_result.problems.push(format!(
                    "variable '{}' is missing from the golden file",
                    result.variables[result_index]
                ));
            }
        }

        if !args.ignore_missing {
            for golden_index in golden_indices {
                if !matched_golden.contains(&golden_index) {
                    cmp_result.problems.push(format!(
                        "variable '{}' is missing from the result",
                        golden.variables[golden_index]
                    ));
                }
            }
        }
    }

    pairs
}
