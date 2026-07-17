use std::collections::{BTreeMap, HashMap, HashSet};

use super::lexer::{TokenKind, tokenize};
use super::{
    Element, ElementKind, Netlist, NetlistSourceLocation, ParseError, ParseWithAbortError,
    SubcircuitDef, UndefinedMutualInductorReferenceError, ensure_parse_not_aborted,
    finish_non_aborting_parse, poll_parse_abort,
};
use crate::abort_signal::{AbortSignal, NoAbort};

/// One source-ordered semantic record used by the parser's post-parse
/// validation pass.
#[derive(Debug, Clone)]
pub(crate) enum MutualInductorSemanticRecord {
    Inductor {
        scope_name: Option<String>,
        authored_name: String,
    },
    Coupling {
        scope_name: Option<String>,
        authored_name: String,
        referenced_inductors: Vec<String>,
        origin: NetlistSourceLocation,
    },
}

impl MutualInductorSemanticRecord {
    pub(crate) fn append_parsed_elements(
        records: &mut Vec<Self>,
        elements: &[Element],
        scope_name: Option<&str>,
        origin: &NetlistSourceLocation,
        source_line: &str,
    ) {
        let authored_tokens = authored_tokens(source_line);
        for element in elements {
            match &element.kind {
                ElementKind::Inductor { .. } | ElementKind::JilesAthertonInductor { .. } => records
                    .push(Self::Inductor {
                        scope_name: scope_name.map(ToString::to_string),
                        authored_name: authored_tokens
                            .first()
                            .cloned()
                            .unwrap_or_else(|| element.name.clone()),
                    }),
                ElementKind::Coupling { inductors, .. } => records.push(Self::Coupling {
                    scope_name: scope_name.map(ToString::to_string),
                    authored_name: authored_tokens
                        .first()
                        .cloned()
                        .unwrap_or_else(|| element.name.clone()),
                    referenced_inductors: inductors
                        .iter()
                        .enumerate()
                        .map(|(index, parsed_name)| {
                            authored_tokens
                                .get(index + 1)
                                .cloned()
                                .unwrap_or_else(|| parsed_name.clone())
                        })
                        .collect(),
                    origin: origin.clone(),
                }),
                _ => {}
            }
        }
    }

    fn append_ast_elements(
        records: &mut Vec<Self>,
        elements: &[Element],
        scope_name: Option<&str>,
    ) {
        for element in elements {
            match &element.kind {
                ElementKind::Inductor { .. } | ElementKind::JilesAthertonInductor { .. } => records
                    .push(Self::Inductor {
                        scope_name: scope_name.map(ToString::to_string),
                        authored_name: element.name.clone(),
                    }),
                ElementKind::Coupling { inductors, .. } => records.push(Self::Coupling {
                    scope_name: scope_name.map(ToString::to_string),
                    authored_name: element.name.clone(),
                    referenced_inductors: inductors.clone(),
                    origin: NetlistSourceLocation::in_memory(0),
                }),
                _ => {}
            }
        }
    }
}

/// Validate every K-card against the complete set of inductors in its own
/// scope.
///
/// All inductor definitions are collected before references are checked, so
/// forward references are legal. Couplings are checked in source order. Within
/// one K-card, Xyce stores references in a case-insensitive ordered map, so
/// distinct canonical names are validated in lexical order and case-colliding
/// duplicates collapse to their first authored occurrence. The coefficient is
/// intentionally irrelevant: a zero-coupling card still carries real
/// references and must be valid.
pub(crate) fn validate_mutual_inductor_semantic_records(
    records: &[MutualInductorSemanticRecord],
) -> Result<(), ParseError> {
    finish_non_aborting_parse(validate_mutual_inductor_semantic_records_with_abort(
        records, &NoAbort,
    ))
}

pub(crate) fn validate_mutual_inductor_semantic_records_with_abort(
    records: &[MutualInductorSemanticRecord],
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let mut inductors_by_scope = HashMap::<Option<String>, HashSet<String>>::new();
    for (record_index, record) in records.iter().enumerate() {
        poll_parse_abort(abort, record_index)?;
        if let MutualInductorSemanticRecord::Inductor {
            scope_name,
            authored_name,
        } = record
        {
            inductors_by_scope
                .entry(canonical_scope(scope_name.as_deref()))
                .or_default()
                .insert(authored_name.to_ascii_uppercase());
        }
    }

    ensure_parse_not_aborted(abort)?;
    for (record_index, record) in records.iter().enumerate() {
        poll_parse_abort(abort, record_index)?;
        let MutualInductorSemanticRecord::Coupling {
            scope_name,
            authored_name,
            referenced_inductors,
            origin,
        } = record
        else {
            continue;
        };
        let canonical_scope_name = canonical_scope(scope_name.as_deref());
        let local_inductors = inductors_by_scope.get(&canonical_scope_name);
        let mut ordered_references = BTreeMap::<String, (usize, &str)>::new();
        for (index, inductor_name) in referenced_inductors.iter().enumerate() {
            poll_parse_abort(abort, index)?;
            ordered_references
                .entry(inductor_name.to_ascii_uppercase())
                .or_insert((index, inductor_name));
        }
        for (ordered_index, (canonical_inductor_name, (first_index, inductor_name))) in
            ordered_references.into_iter().enumerate()
        {
            poll_parse_abort(abort, ordered_index)?;
            if local_inductors.is_some_and(|names| names.contains(&canonical_inductor_name)) {
                continue;
            }

            return Err(ParseError::UndefinedMutualInductorReference(Box::new(
                UndefinedMutualInductorReferenceError {
                    origin: origin.clone(),
                    authored_coupling_name: authored_name.clone(),
                    canonical_coupling_name: authored_name.to_ascii_uppercase(),
                    qualified_coupling_name: qualify(scope_name.as_deref(), authored_name),
                    authored_inductor_name: inductor_name.to_string(),
                    canonical_inductor_name,
                    qualified_inductor_name: qualify(scope_name.as_deref(), inductor_name),
                    scope_name: scope_name.clone(),
                    reference_position: first_index + 1,
                },
            ))
            .into());
        }
    }

    ensure_parse_not_aborted(abort)?;
    Ok(())
}

/// Revalidate mutual-inductor references on an existing AST.
///
/// Parsed decks are validated automatically with exact physical origins. This
/// entry point is also used defensively before hierarchy flattening and is
/// available to callers that construct or mutate a [`Netlist`] directly.
/// Programmatically constructed ASTs have no physical source location, so an
/// unlocated (`line == 0`) origin is reported.
pub fn validate_mutual_inductor_references(netlist: &Netlist) -> Result<(), ParseError> {
    let mut records = Vec::new();
    MutualInductorSemanticRecord::append_ast_elements(&mut records, &netlist.elements, None);

    let mut visited_scopes = HashSet::new();
    for subcircuit in &netlist.subcircuits {
        collect_subcircuit_records(subcircuit, None, &mut visited_scopes, &mut records);
    }

    validate_mutual_inductor_semantic_records(&records)
}

fn collect_subcircuit_records(
    subcircuit: &SubcircuitDef,
    parent_scope: Option<&str>,
    visited_scopes: &mut HashSet<String>,
    records: &mut Vec<MutualInductorSemanticRecord>,
) {
    let qualified_scope_name = qualify_subcircuit_scope(parent_scope, &subcircuit.name);
    let canonical_scope_name = qualified_scope_name.to_ascii_uppercase();
    if !visited_scopes.insert(canonical_scope_name) {
        return;
    }

    MutualInductorSemanticRecord::append_ast_elements(
        records,
        &subcircuit.elements,
        Some(&qualified_scope_name),
    );
    for nested in &subcircuit.nested_subcircuits {
        collect_subcircuit_records(nested, Some(&qualified_scope_name), visited_scopes, records);
    }
}

fn canonical_scope(scope_name: Option<&str>) -> Option<String> {
    scope_name.map(str::to_ascii_uppercase)
}

fn qualify(scope_name: Option<&str>, local_name: &str) -> String {
    match scope_name {
        Some(scope_name) if !scope_name.is_empty() => format!("{scope_name}.{local_name}"),
        _ => local_name.to_string(),
    }
}

fn qualify_subcircuit_scope(parent_scope: Option<&str>, authored_scope: &str) -> String {
    let Some(parent_scope) = parent_scope.filter(|scope| !scope.is_empty()) else {
        return authored_scope.to_string();
    };
    let already_qualified = authored_scope
        .get(..parent_scope.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(parent_scope))
        && authored_scope.as_bytes().get(parent_scope.len()) == Some(&b'.');
    if already_qualified {
        authored_scope.to_string()
    } else {
        format!("{parent_scope}.{authored_scope}")
    }
}

fn authored_tokens(source_line: &str) -> Vec<String> {
    tokenize(source_line)
        .map(|tokens| {
            tokens
                .into_iter()
                .filter(|token| {
                    !matches!(
                        token.kind,
                        TokenKind::Comma | TokenKind::Newline | TokenKind::Eof
                    )
                })
                .map(|token| token.lexeme)
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct AbortAfterChecks {
        checks_before_abort: usize,
        checks: AtomicUsize,
    }

    impl AbortSignal for AbortAfterChecks {
        fn is_aborted(&self) -> bool {
            self.checks.fetch_add(1, Ordering::Relaxed) >= self.checks_before_abort
        }
    }

    fn undefined_error(source: &str) -> Box<UndefinedMutualInductorReferenceError> {
        match Netlist::parse(source).expect_err("undefined mutual reference must fail") {
            ParseError::UndefinedMutualInductorReference(error) => error,
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn forward_and_case_insensitive_local_references_are_valid() {
        Netlist::parse(
            "forward references\n\
             Kmix lLEFT Lright 0\n\
             LRIGHT b 0 2u\n\
             lleft a 0 1u\n\
             .end\n",
        )
        .expect("forward, case-insensitive references are valid");
    }

    #[test]
    fn bug75_reports_exact_typed_identity_and_origin() {
        let error = undefined_error(
            "Test error message capability when a mutual inductor references an undefined\n\
             *inductor\n\
             \n\
             R1 1 2 1\n\
             L1 2 0 1\n\
             *L2 2 3 1\n\
             L3 2 0 1\n\
             V1 1 0 DC 1\n\
             \n\
             K1 L1 L3 0.1\n\
             K2 L1 L3 0\n\
             K3 L1 L2 0\n\
             \n\
             .DC V1 1 1 0.1\n\
             .end\n",
        );

        assert_eq!(
            error.to_string(),
            "Undefined inductor L2 in mutual inductor K3 definition."
        );
        assert_eq!(error.origin, NetlistSourceLocation::in_memory(12));
        assert_eq!(error.authored_coupling_name, "K3");
        assert_eq!(error.canonical_coupling_name, "K3");
        assert_eq!(error.qualified_coupling_name, "K3");
        assert_eq!(error.authored_inductor_name, "L2");
        assert_eq!(error.canonical_inductor_name, "L2");
        assert_eq!(error.qualified_inductor_name, "L2");
        assert_eq!(error.scope_name, None);
        assert_eq!(error.reference_position, 2);
    }

    #[test]
    fn zero_coupling_still_validates_references() {
        let error = undefined_error(
            "zero coupling\n\
             L1 1 0 1u\n\
             K1 L1 Lmissing 0\n\
             .end\n",
        );
        assert_eq!(error.authored_inductor_name, "Lmissing");
        assert_eq!(error.reference_position, 2);
    }

    #[test]
    fn display_canonicalizes_lowercase_authored_names() {
        let error = undefined_error(
            "canonical display\n\
             l1 1 0 1u\n\
             kLower l1 lMissing 0\n\
             .end\n",
        );
        assert_eq!(error.authored_coupling_name, "kLower");
        assert_eq!(error.authored_inductor_name, "lMissing");
        assert_eq!(error.canonical_coupling_name, "KLOWER");
        assert_eq!(error.canonical_inductor_name, "LMISSING");
        assert_eq!(
            error.to_string(),
            "Undefined inductor LMISSING in mutual inductor KLOWER definition."
        );
    }

    #[test]
    fn unused_subcircuit_is_validated_and_identity_is_qualified() {
        let error = undefined_error(
            "unused subcircuit\n\
             .subckt Cell p n\n\
             Klocal L1 lMissing 0.5\n\
             L1 p n 1u\n\
             .ends\n\
             .end\n",
        );
        assert_eq!(error.scope_name.as_deref(), Some("Cell"));
        assert_eq!(error.qualified_coupling_name, "Cell.Klocal");
        assert_eq!(error.qualified_inductor_name, "Cell.lMissing");
        assert_eq!(error.origin.line, 3);
    }

    #[test]
    fn an_inductor_in_another_scope_does_not_satisfy_reference() {
        let error = undefined_error(
            "scope isolation\n\
             Lshared 1 0 1u\n\
             .subckt cell p n\n\
             K1 Llocal Lshared 0.5\n\
             Llocal p n 1u\n\
             .ends\n\
             .end\n",
        );
        assert_eq!(error.scope_name.as_deref(), Some("cell"));
        assert_eq!(error.authored_inductor_name, "Lshared");
        assert_eq!(error.reference_position, 2);
    }

    #[test]
    fn deterministic_order_is_card_then_reference_position() {
        let error = undefined_error(
            "deterministic order\n\
             Kfirst Lmissing1 Lmissing2 0.5\n\
             Ksecond Lmissing3 Lmissing4 0.5\n\
             .end\n",
        );
        assert_eq!(error.authored_coupling_name, "Kfirst");
        assert_eq!(error.authored_inductor_name, "Lmissing1");
        assert_eq!(error.reference_position, 1);
    }

    #[test]
    fn references_within_one_card_use_canonical_lexical_order_and_deduplicate() {
        let error = undefined_error(
            "canonical K reference map\n\
             K1 lz LA la LZ 0.5\n\
             .end\n",
        );
        assert_eq!(error.authored_inductor_name, "LA");
        assert_eq!(error.canonical_inductor_name, "LA");
        assert_eq!(error.reference_position, 2);
    }

    #[test]
    fn semantic_scan_observes_abort_during_large_two_pass_validation() {
        let mut records = Vec::new();
        for index in 0..130 {
            records.push(MutualInductorSemanticRecord::Inductor {
                scope_name: None,
                authored_name: format!("L{index}"),
            });
        }
        records.push(MutualInductorSemanticRecord::Coupling {
            scope_name: None,
            authored_name: "K1".into(),
            referenced_inductors: vec!["L0".into(), "L1".into()],
            origin: NetlistSourceLocation::in_memory(132),
        });
        let abort = AbortAfterChecks {
            checks_before_abort: 5,
            checks: AtomicUsize::new(0),
        };

        assert!(matches!(
            validate_mutual_inductor_semantic_records_with_abort(&records, &abort),
            Err(ParseWithAbortError::Aborted)
        ));
    }

    #[test]
    fn explicit_ast_validation_catches_post_parse_mutation() {
        let mut netlist = Netlist::parse(
            "valid before mutation\n\
             L1 1 0 1u\n\
             L2 2 0 1u\n\
             K1 L1 L2 0.5\n\
             .end\n",
        )
        .expect("initial deck is valid");
        let ElementKind::Coupling { inductors, .. } = &mut netlist.elements[2].kind else {
            panic!("K1 must remain a coupling");
        };
        inductors[1] = "Lmissing".to_string();

        let error = validate_mutual_inductor_references(&netlist)
            .expect_err("mutated AST must fail validation");
        let ParseError::UndefinedMutualInductorReference(error) = error else {
            panic!("unexpected error: {error}");
        };
        assert_eq!(error.origin.line, 0);
        assert_eq!(error.authored_inductor_name, "Lmissing");
    }

    #[test]
    fn file_backed_parse_retains_physical_coupling_origin() {
        let unique = format!(
            "rspice-bug75-origin-{}-{}.cir",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock after Unix epoch")
                .as_nanos()
        );
        let path = std::env::temp_dir().join(unique);
        std::fs::write(
            &path,
            "file origin\n\
             L1 1 0 1u\n\
             K3 L1 L2 0\n\
             .end\n",
        )
        .expect("temporary deck writes");

        let result = Netlist::parse_file(&path);
        let _ = std::fs::remove_file(&path);
        let ParseError::UndefinedMutualInductorReference(error) =
            result.expect_err("undefined file-backed reference must fail")
        else {
            panic!("unexpected error");
        };
        assert_eq!(error.origin.path.as_deref(), Some(path.as_path()));
        assert_eq!(error.origin.line, 3);
    }

    #[test]
    fn included_coupling_failure_retains_child_file_provenance() {
        let unique = format!(
            "rspice-bug75-include-origin-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock after Unix epoch")
                .as_nanos()
        );
        let dir = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&dir).expect("temporary fixture directory creates");
        let deck = dir.join("deck.cir");
        let child = dir.join("child.inc");
        std::fs::write(
            &deck,
            "included mutual reference\n.include child.inc\n.end\n",
        )
        .expect("owner deck writes");
        std::fs::write(&child, "L1 1 0 1u\nK3 L1 L2 0\n").expect("included deck writes");

        let result = Netlist::parse_file(&deck);
        let canonical_child = child.canonicalize().expect("child canonicalizes");
        let _ = std::fs::remove_dir_all(&dir);
        let ParseError::UndefinedMutualInductorReference(error) =
            result.expect_err("included undefined reference must fail")
        else {
            panic!("unexpected error");
        };
        assert_eq!(
            error.origin.path.as_deref(),
            Some(canonical_child.as_path())
        );
        assert_eq!(error.origin.line, 2);
        assert_eq!(error.canonical_coupling_name, "K3");
        assert_eq!(error.canonical_inductor_name, "L2");
    }

    #[test]
    fn public_flattener_revalidates_a_mutated_ast_before_expansion() {
        let mut netlist = Netlist::parse(
            "valid before mutation\n\
             L1 1 0 1u\n\
             L2 2 0 1u\n\
             K1 L1 L2 0.5\n\
             .end\n",
        )
        .expect("initial deck is valid");
        let ElementKind::Coupling { inductors, .. } = &mut netlist.elements[2].kind else {
            panic!("K1 must remain a coupling");
        };
        inductors[0] = "missing".to_string();

        let mut flattener = super::super::Flattener::new(&netlist.subcircuits);
        let error = flattener
            .flatten(&netlist)
            .expect_err("flatten must validate before expansion");
        assert!(matches!(
            error,
            ParseError::UndefinedMutualInductorReference(_)
        ));
    }

    #[test]
    fn programmatic_nested_same_named_subcircuits_keep_distinct_qualified_scopes() {
        fn inductor(name: &str) -> Element {
            Element {
                name: name.to_string(),
                kind: ElementKind::Inductor {
                    value: 1.0,
                    value_expr: None,
                    model: None,
                    instance_params: Vec::new(),
                    deferred_params: Vec::new(),
                    initial_current: None,
                },
                nodes: vec!["1".into(), "0".into()],
                provenance: crate::netlist::ElementProvenance::Authored,
            }
        }
        fn coupling(name: &str, missing: &str) -> Element {
            Element {
                name: name.to_string(),
                kind: ElementKind::Coupling {
                    inductors: vec!["L1".into(), missing.into()],
                    coefficient: 0.5,
                },
                nodes: Vec::new(),
                provenance: crate::netlist::ElementProvenance::Authored,
            }
        }
        fn subcircuit(
            name: &str,
            nested: Vec<SubcircuitDef>,
            elements: Vec<Element>,
        ) -> SubcircuitDef {
            SubcircuitDef {
                name: name.into(),
                ports: Vec::new(),
                elements,
                initial_conditions: Vec::new(),
                node_sets: Vec::new(),
                params: Vec::new(),
                expr_params: Vec::new(),
                string_params: Vec::new(),
                body_params: Vec::new(),
                body_expr_params: Vec::new(),
                body_string_params: Vec::new(),
                body_functions: Vec::new(),
                local_options: HashMap::new(),
                library_ref: None,
                nested_subcircuits: nested,
            }
        }

        let leaf_a = subcircuit("leaf", Vec::new(), vec![inductor("L1")]);
        let leaf_b = subcircuit(
            "leaf",
            Vec::new(),
            vec![inductor("L1"), coupling("Kbad", "Lmissing")],
        );
        let mut netlist = Netlist::default();
        netlist.subcircuits = vec![
            subcircuit("parent_a", vec![leaf_a], Vec::new()),
            subcircuit("parent_b", vec![leaf_b], Vec::new()),
        ];

        let ParseError::UndefinedMutualInductorReference(error) =
            validate_mutual_inductor_references(&netlist)
                .expect_err("parent_b.leaf must not be skipped as a duplicate of parent_a.leaf")
        else {
            panic!("unexpected error");
        };
        assert_eq!(error.scope_name.as_deref(), Some("parent_b.leaf"));
        assert_eq!(error.qualified_coupling_name, "parent_b.leaf.Kbad");
        assert_eq!(error.qualified_inductor_name, "parent_b.leaf.Lmissing");
    }
}
