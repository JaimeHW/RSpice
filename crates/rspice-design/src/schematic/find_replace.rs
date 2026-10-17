//! Find and replace over one schematic document's instance names, net-label
//! names and property values, as Virtuoso's Edit > Search/Replace does.
//!
//! The query is compiled once; [`find_in_document`] lists what it would change
//! and [`replace_in_document`] makes exactly those changes, after
//! [`refusal`] has proved every new name legal and every instance name still
//! unique. The caller owns the scope (which objects, which documents) and
//! the undo step.

use super::component::Component;
use super::document::SchematicDocument;
use super::net_label::NetLabel;
use regex::{NoExpand, Regex};

/// How the find text is read.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FindOptions {
    /// Only where the text stands as a whole word.
    pub whole_word: bool,
    /// Upper and lower case differ.
    pub match_case: bool,
    /// The find text is a regular expression, and the replacement may name
    /// its groups as `$1` or `${name}`.
    pub regex: bool,
}

/// What the query looks at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindTargets {
    pub instance_names: bool,
    pub net_labels: bool,
    pub values: bool,
}

impl Default for FindTargets {
    fn default() -> Self {
        Self {
            instance_names: true,
            net_labels: true,
            values: true,
        }
    }
}

/// A compiled find and replace.
#[derive(Debug, Clone)]
pub struct FindQuery {
    pattern: Regex,
    replacement: String,
    expand_groups: bool,
    targets: FindTargets,
}

/// The text a match lives in.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FindField {
    InstanceName,
    NetLabel,
    Value,
    /// The value of the named instance parameter.
    Parameter(String),
}

impl FindField {
    /// How a row names the field: what a reader would call it.
    pub fn label(&self) -> String {
        match self {
            Self::InstanceName => "name".to_owned(),
            Self::NetLabel => "label".to_owned(),
            Self::Value => "value".to_owned(),
            Self::Parameter(name) => name.clone(),
        }
    }
}

/// One text the replacement changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindMatch {
    /// The instance or net label that owns the text.
    pub object_id: u64,
    pub field: FindField,
    /// The instance's or the label's name, for the reader.
    pub owner: String,
    pub before: String,
    pub after: String,
}

impl FindQuery {
    /// Compile `find` under `options`. An empty find text, or a pattern that
    /// does not compile, is refused with the reason.
    pub fn new(
        find: &str,
        replacement: &str,
        options: FindOptions,
        targets: FindTargets,
    ) -> Result<Self, String> {
        if find.is_empty() {
            return Err("Enter the text to find.".to_owned());
        }
        let body = if options.regex {
            find.to_owned()
        } else {
            regex::escape(find)
        };
        let body = if options.whole_word {
            format!(r"\b(?:{body})\b")
        } else {
            body
        };
        let body = if options.match_case {
            body
        } else {
            format!("(?i:{body})")
        };
        let pattern = Regex::new(&body).map_err(|error| match error {
            regex::Error::Syntax(detail) => format!(
                "The pattern does not read as a regular expression: {}",
                detail
                    .lines()
                    .last()
                    .unwrap_or_default()
                    .trim()
                    .trim_start_matches("error: ")
            ),
            other => format!("The pattern cannot be used: {other}"),
        })?;
        Ok(Self {
            pattern,
            replacement: replacement.to_owned(),
            expand_groups: options.regex,
            targets,
        })
    }

    /// `text` with every match replaced, if anything matched.
    pub fn replace(&self, text: &str) -> Option<String> {
        if !self.pattern.is_match(text) {
            return None;
        }
        let replaced = if self.expand_groups {
            self.pattern.replace_all(text, self.replacement.as_str())
        } else {
            self.pattern
                .replace_all(text, NoExpand(self.replacement.as_str()))
        };
        Some(replaced.into_owned())
    }

    pub fn targets(&self) -> FindTargets {
        self.targets
    }
}

/// Every text `query` matches in `document`, among the objects `include`
/// admits, in document order.
pub fn find_in_document(
    document: &SchematicDocument,
    query: &FindQuery,
    include: impl Fn(u64) -> bool,
) -> Vec<FindMatch> {
    let mut matches = Vec::new();
    let targets = query.targets;
    for component in document
        .components
        .iter()
        .filter(|component| include(component.id))
    {
        let mut push = |field: FindField, before: &str| {
            if let Some(after) = query.replace(before) {
                matches.push(FindMatch {
                    object_id: component.id,
                    field,
                    owner: component.name.clone(),
                    before: before.to_owned(),
                    after,
                });
            }
        };
        if targets.instance_names {
            push(FindField::InstanceName, &component.name);
        }
        if targets.values {
            push(FindField::Value, &component.value);
            for (name, value) in parameters(&component.params) {
                push(FindField::Parameter(name.to_owned()), value);
            }
        }
    }
    if targets.net_labels {
        for label in document.net_labels.iter().filter(|label| include(label.id)) {
            if let Some(after) = query.replace(&label.name) {
                matches.push(FindMatch {
                    object_id: label.id,
                    field: FindField::NetLabel,
                    owner: label.name.clone(),
                    before: label.name.clone(),
                    after,
                });
            }
        }
    }
    matches
}

/// Why `matches` cannot be applied to `document`, if they cannot: a new
/// name that is not a legal designator or net name, or two instances that
/// would share a name.
pub fn refusal(document: &SchematicDocument, matches: &[FindMatch]) -> Option<String> {
    let policy = document.document_policy.net_naming;
    for found in matches {
        match &found.field {
            FindField::InstanceName => {
                let component = document
                    .components
                    .iter()
                    .find(|component| component.id == found.object_id)?;
                if let Err(error) = component.validate_reference_designator(&found.after) {
                    return Some(format!(
                        "{} cannot be renamed {}: {error}",
                        found.before, found.after
                    ));
                }
            }
            FindField::NetLabel => {
                if let Err(reason) = NetLabel::validate_name(&found.after, policy) {
                    return Some(format!("Net label {}: {reason}.", found.after));
                }
            }
            FindField::Value | FindField::Parameter(_) => {
                if found.after.chars().any(char::is_whitespace) && found.field != FindField::Value {
                    return Some(format!(
                        "{} {}: a parameter value cannot contain a space.",
                        found.owner,
                        found.field.label()
                    ));
                }
            }
        }
    }
    let mut names: Vec<String> = document
        .components
        .iter()
        .map(|component| {
            matches
                .iter()
                .find(|found| {
                    found.object_id == component.id && found.field == FindField::InstanceName
                })
                .map_or_else(|| component.name.clone(), |found| found.after.clone())
                .to_ascii_lowercase()
        })
        .collect();
    names.sort_unstable();
    names
        .windows(2)
        .find(|pair| pair[0] == pair[1] && !pair[0].is_empty())
        .map(|pair| {
            format!(
                "Two instances would both be named {}.",
                pair[0].to_ascii_uppercase()
            )
        })
}

/// Make the changes `matches` lists. Returns how many texts changed. A
/// match whose text has changed since it was found is left alone.
pub fn replace_in_document(document: &mut SchematicDocument, matches: &[FindMatch]) -> usize {
    let mut changed = 0;
    for found in matches {
        let applied = match &found.field {
            FindField::NetLabel => document
                .net_labels
                .iter_mut()
                .find(|label| label.id == found.object_id && label.name == found.before)
                .map(|label| label.name.clone_from(&found.after))
                .is_some(),
            field => document
                .components
                .iter_mut()
                .find(|component| component.id == found.object_id)
                .is_some_and(|component| replace_component_text(component, field, found)),
        };
        changed += usize::from(applied);
    }
    changed
}

fn replace_component_text(component: &mut Component, field: &FindField, found: &FindMatch) -> bool {
    match field {
        FindField::InstanceName if component.name == found.before => {
            component.name.clone_from(&found.after);
            true
        }
        FindField::Value if component.value == found.before => {
            component.value.clone_from(&found.after);
            true
        }
        FindField::Parameter(name) => {
            let mut applied = false;
            let rewritten: Vec<String> = component
                .params
                .split_whitespace()
                .map(|token| match token.split_once('=') {
                    Some((key, value)) if key == name && value == found.before && !applied => {
                        applied = true;
                        format!("{key}={}", found.after)
                    }
                    _ => token.to_owned(),
                })
                .collect();
            if applied {
                component.params = rewritten.join(" ");
            }
            applied
        }
        _ => false,
    }
}

/// The `name=value` pairs of an instance's parameter text.
fn parameters(params: &str) -> impl Iterator<Item = (&str, &str)> {
    params
        .split_whitespace()
        .filter_map(|token| token.split_once('='))
        .filter(|(name, _)| !name.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::component_type::ComponentType;
    use rspice_design_model::Point;

    fn resistor(id: u64, name: &str, value: &str, params: &str) -> Component {
        let mut component = Component::new(id, ComponentType::Resistor, Point::origin());
        component.name = name.to_owned();
        component.value = value.to_owned();
        component.params = params.to_owned();
        component
    }

    fn document() -> SchematicDocument {
        SchematicDocument {
            components: vec![
                resistor(1, "R1", "10k", "tc1=1m"),
                resistor(2, "R12", "rload", ""),
            ],
            net_labels: vec![NetLabel::new(3, Point::origin(), "vout_r1")],
            ..SchematicDocument::default()
        }
    }

    fn query(find: &str, replace: &str, options: FindOptions) -> FindQuery {
        FindQuery::new(find, replace, options, FindTargets::default()).expect("query")
    }

    #[test]
    fn plain_text_finds_names_labels_values_and_parameters_without_case() {
        let found = find_in_document(
            &document(),
            &query("r1", "RX", FindOptions::default()),
            |_| true,
        );
        let fields: Vec<_> = found
            .iter()
            .map(|found| (found.object_id, found.after.as_str()))
            .collect();
        assert_eq!(fields, [(1, "RX"), (2, "RX2"), (3, "vout_RX")]);
    }

    #[test]
    fn whole_word_case_and_regex_narrow_the_match() {
        let whole = FindOptions {
            whole_word: true,
            ..FindOptions::default()
        };
        let found = find_in_document(&document(), &query("r1", "RX", whole), |_| true);
        assert_eq!(
            found
                .iter()
                .map(|found| found.object_id)
                .collect::<Vec<_>>(),
            [1]
        );

        let cased = FindOptions {
            match_case: true,
            ..FindOptions::default()
        };
        assert_eq!(
            find_in_document(&document(), &query("r1", "RX", cased), |_| true)
                .iter()
                .map(|found| found.object_id)
                .collect::<Vec<_>>(),
            [3]
        );

        let pattern = FindOptions {
            regex: true,
            ..FindOptions::default()
        };
        let found = find_in_document(&document(), &query(r"^R(\d+)$", "RB$1", pattern), |_| true);
        assert_eq!(
            found
                .iter()
                .map(|found| found.after.as_str())
                .collect::<Vec<_>>(),
            ["RB1", "RB12"]
        );
        let literal = find_in_document(
            &document(),
            &query("10k", "$1", FindOptions::default()),
            |_| true,
        );
        assert_eq!(
            literal[0].after, "$1",
            "a plain replacement is taken literally"
        );
        assert!(FindQuery::new("(", "", pattern, FindTargets::default()).is_err());
        assert!(FindQuery::new("", "x", FindOptions::default(), FindTargets::default()).is_err());
    }

    #[test]
    fn parameter_values_are_replaced_and_names_are_left_alone() {
        let mut document = document();
        let found = find_in_document(
            &document,
            &query("1m", "2m", FindOptions::default()),
            |_| true,
        );
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].field, FindField::Parameter("tc1".to_owned()));
        assert_eq!(replace_in_document(&mut document, &found), 1);
        assert_eq!(document.components[0].params, "tc1=2m");
    }

    #[test]
    fn a_replacement_that_would_duplicate_or_break_a_name_is_refused() {
        let document = document();
        let clash = find_in_document(
            &document,
            &query("R12", "R1", FindOptions::default()),
            |id| id == 2,
        );
        assert_eq!(
            refusal(&document, &clash).as_deref(),
            Some("Two instances would both be named R1.")
        );
        let broken = find_in_document(
            &document,
            &query("vout_r1", "a b", FindOptions::default()),
            |_| true,
        );
        assert!(refusal(&document, &broken).is_some());
        let fine = find_in_document(
            &document,
            &query("R12", "R2", FindOptions::default()),
            |_| true,
        );
        assert_eq!(refusal(&document, &fine), None);
    }

    #[test]
    fn scope_and_targets_are_the_callers() {
        let only_labels = FindQuery::new(
            "r1",
            "x",
            FindOptions::default(),
            FindTargets {
                instance_names: false,
                net_labels: true,
                values: false,
            },
        )
        .expect("query");
        assert_eq!(
            find_in_document(&document(), &only_labels, |_| true).len(),
            1
        );
        assert!(
            find_in_document(
                &document(),
                &query("r1", "x", FindOptions::default()),
                |_| false
            )
            .is_empty()
        );
    }
}
