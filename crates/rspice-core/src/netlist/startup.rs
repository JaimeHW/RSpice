//! Semantic validation and typed diagnostics for `.IC`/`.NODESET` cards.

use super::{
    ExpressionDialect, InitialCondition, Netlist, NodeSet, ParseDiagnostic, ParseError,
    ParseWithAbortError, StartupDiagnostic, StartupDiagnosticCode, StartupDirectiveConflictError,
    StartupDirectiveDisposition, StartupDirectiveKind, StartupDirectiveRecord,
    StartupDirectiveScope, collect_output_node_namespace_from_elements_with_abort,
    ensure_parse_not_aborted, finish_non_aborting_parse, flatten_netlist_with_models_with_abort,
    poll_parse_abort, poll_parse_text,
};
use crate::abort_signal::{AbortSignal, NoAbort};
use std::collections::{BTreeSet, HashSet};

/// Validate startup cards without cooperative cancellation.
pub fn validate_startup_directives(netlist: &mut Netlist) -> Result<(), ParseError> {
    finish_non_aborting_parse(validate_startup_directives_with_abort(netlist, &NoAbort))
}

/// Validate startup cards transactionally after hierarchy is available.
///
/// The public netlist is replaced only after the complete parse-stage and
/// startup-topology pass succeeds. Cancellation or a typed Xyce mode conflict
/// therefore leaves every record and effective numeric vector unchanged.
pub fn validate_startup_directives_with_abort(
    netlist: &mut Netlist,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let mut candidate = netlist.clone();
    validate_candidate(&mut candidate, abort)?;
    ensure_parse_not_aborted(abort)?;
    *netlist = candidate;
    Ok(())
}

fn validate_candidate(
    netlist: &mut Netlist,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let unowned = extract_unowned_startup_state(netlist);
    remove_projected_diagnostics(netlist);
    reset_records(netlist, abort)?;
    discard_scoped_global_cards(netlist, abort)?;
    rebuild_effective_vectors(netlist, &unowned, abort)?;

    ensure_parse_not_aborted(abort)?;
    match flatten_netlist_with_models_with_abort(netlist, abort) {
        Ok(flattened) => {
            ensure_parse_not_aborted(abort)?;
            merge_qualified_expansions(netlist, &flattened.scoped_startup_directives, abort)?;
            validate_xyce_mode_conflict(netlist)?;
            let namespace = collect_output_node_namespace_from_elements_with_abort(
                netlist,
                &flattened.elements,
                abort,
            )?;
            filter_unknown_entries(netlist, &namespace, abort)?;
            rebuild_effective_vectors(netlist, &unowned, abort)?;
        }
        Err(ParseWithAbortError::Aborted) => return Err(ParseWithAbortError::Aborted),
        Err(ParseWithAbortError::Parse(_)) => {
            // Preserve ordinary hierarchy/elaboration error precedence while
            // still enforcing conflicts between effective top-level cards.
            validate_xyce_mode_conflict(netlist)?;
        }
    }
    project_typed_diagnostics(netlist);
    ensure_parse_not_aborted(abort)
}

impl Netlist {
    /// Physical startup cards retained as immutable semantic provenance.
    pub fn startup_directives(&self) -> &[StartupDirectiveRecord] {
        &self.startup_directives
    }

    /// Deterministic typed warning report for finalized startup cards.
    pub fn startup_diagnostics(&self) -> Vec<StartupDiagnostic> {
        let mut diagnostics = Vec::new();
        let mut undefined = [
            UndefinedAccumulator::default(),
            UndefinedAccumulator::default(),
        ];
        for record in &self.startup_directives {
            if record.entries.is_empty()
                && matches!(
                    record.disposition,
                    StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::EmptyDirective)
                )
            {
                diagnostics.push(StartupDiagnostic {
                    code: StartupDiagnosticCode::EmptyDirective,
                    stage: super::StartupDiagnosticStage::Parse,
                    kind: record.kind,
                    origins: vec![record.origin.clone()],
                    scopes: vec![record.scope.clone()],
                    canonical_nodes: Vec::new(),
                });
            }
            for entry in &record.entries {
                if let StartupDirectiveDisposition::Ignored(code) = entry.disposition {
                    match code {
                        StartupDiagnosticCode::ScopedGlobalNode => {
                            diagnostics.push(StartupDiagnostic {
                                code,
                                stage: code.stage(),
                                kind: record.kind,
                                origins: vec![record.origin.clone()],
                                scopes: vec![record.scope.clone()],
                                canonical_nodes: vec![entry.canonical_node.clone()],
                            });
                        }
                        StartupDiagnosticCode::UndefinedNode => {
                            let index = usize::from(record.kind == StartupDirectiveKind::NodeSet);
                            undefined[index].push(record, &entry.canonical_node);
                        }
                        StartupDiagnosticCode::EmptyDirective => {}
                    }
                }
            }
        }
        for (index, accumulator) in undefined.into_iter().enumerate() {
            if !accumulator.nodes.is_empty() {
                diagnostics.push(StartupDiagnostic {
                    code: StartupDiagnosticCode::UndefinedNode,
                    stage: super::StartupDiagnosticStage::StartupTopology,
                    kind: if index == 0 {
                        StartupDirectiveKind::Ic
                    } else {
                        StartupDirectiveKind::NodeSet
                    },
                    origins: accumulator.origins,
                    scopes: accumulator.scopes,
                    canonical_nodes: accumulator.nodes.into_iter().collect(),
                });
            }
        }
        diagnostics
    }
}

#[derive(Default)]
struct UndefinedAccumulator {
    origins: Vec<super::NetlistSourceLocation>,
    scopes: Vec<StartupDirectiveScope>,
    nodes: BTreeSet<String>,
}

impl UndefinedAccumulator {
    fn push(&mut self, record: &StartupDirectiveRecord, node: &str) {
        if !self.origins.contains(&record.origin) {
            self.origins.push(record.origin.clone());
        }
        if !self.scopes.contains(&record.scope) {
            self.scopes.push(record.scope.clone());
        }
        self.nodes.insert(node.to_string());
    }
}

fn reset_records(
    netlist: &mut Netlist,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let ground_policy = netlist.ground_policy();
    for (record_index, record) in netlist.startup_directives.iter_mut().enumerate() {
        poll_parse_abort(abort, record_index)?;
        if let StartupDirectiveScope::Subcircuit {
            qualified_instances,
            ..
        } = &mut record.scope
        {
            qualified_instances.clear();
        }
        for entry in &mut record.entries {
            poll_parse_text(abort, &entry.authored_node)?;
            entry.canonical_node = ground_policy
                .canonical_node(&entry.execution_node)
                .replace(':', ".")
                .to_ascii_uppercase();
            entry.qualified_nodes.clear();
            entry.disposition = StartupDirectiveDisposition::Applied;
            if matches!(record.scope, StartupDirectiveScope::TopLevel) {
                entry.qualified_nodes.push(entry.canonical_node.clone());
            }
        }
        record.disposition = if record.entries.is_empty() {
            StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::EmptyDirective)
        } else {
            StartupDirectiveDisposition::Applied
        };
    }
    Ok(())
}

fn discard_scoped_global_cards(
    netlist: &mut Netlist,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    if netlist.params.expression_dialect() != ExpressionDialect::Xyce {
        return Ok(());
    }
    let explicit_globals = netlist
        .global_nodes
        .iter()
        .map(|node| node.to_ascii_uppercase())
        .collect::<BTreeSet<_>>();
    for (record_index, record) in netlist.startup_directives.iter_mut().enumerate() {
        poll_parse_abort(abort, record_index)?;
        if !matches!(record.scope, StartupDirectiveScope::Subcircuit { .. })
            || record.entries.is_empty()
        {
            continue;
        }
        let offending = record.entries.iter().any(|entry| {
            explicit_globals.contains(&entry.canonical_node)
                || entry.canonical_node.starts_with("$G")
        });
        if offending {
            record.disposition =
                StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::ScopedGlobalNode);
            // Only the actual global target(s) carry the warning code. Valid
            // siblings are discarded by the card disposition without being
            // falsely reported as global nodes.
            for entry in &mut record.entries {
                if explicit_globals.contains(&entry.canonical_node)
                    || entry.canonical_node.starts_with("$G")
                {
                    entry.disposition = StartupDirectiveDisposition::Ignored(
                        StartupDiagnosticCode::ScopedGlobalNode,
                    );
                }
            }
        }
    }
    Ok(())
}

fn validate_xyce_mode_conflict(netlist: &Netlist) -> Result<(), ParseWithAbortError> {
    if netlist.params.expression_dialect() != ExpressionDialect::Xyce {
        return Ok(());
    }
    let mut first_effective: Option<&StartupDirectiveRecord> = None;
    for record in &netlist.startup_directives {
        if !matches!(
            record.disposition,
            StartupDirectiveDisposition::Applied | StartupDirectiveDisposition::PartiallyApplied
        ) || record.entries.is_empty()
            || matches!(
                &record.scope,
                StartupDirectiveScope::Subcircuit {
                    qualified_instances,
                    ..
                } if qualified_instances.is_empty()
            )
        {
            continue;
        }
        let Some(first) = first_effective else {
            first_effective = Some(record);
            continue;
        };
        if first.kind != record.kind {
            return Err(ParseError::StartupDirectiveConflict(Box::new(
                StartupDirectiveConflictError {
                    first_kind: first.kind,
                    first: first.origin.clone(),
                    conflicting_kind: record.kind,
                    conflicting: record.origin.clone(),
                },
            ))
            .into());
        }
    }
    Ok(())
}

fn merge_qualified_expansions(
    netlist: &mut Netlist,
    elaborated: &[StartupDirectiveRecord],
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    for (record_index, source) in netlist.startup_directives.iter_mut().enumerate() {
        poll_parse_abort(abort, record_index)?;
        let StartupDirectiveScope::Subcircuit {
            qualified_definition,
            qualified_instances,
        } = &mut source.scope
        else {
            continue;
        };
        for expansion in elaborated.iter().filter(|candidate| {
            candidate.kind == source.kind
                && candidate.origin == source.origin
                && matches!(
                    &candidate.scope,
                    StartupDirectiveScope::Subcircuit {
                        qualified_definition: candidate_definition,
                        ..
                    } if candidate_definition.eq_ignore_ascii_case(qualified_definition)
                )
        }) {
            if let StartupDirectiveScope::Subcircuit {
                qualified_instances: instances,
                ..
            } = &expansion.scope
            {
                qualified_instances.extend(instances.iter().cloned());
            }
            for (entry, expanded_entry) in source.entries.iter_mut().zip(&expansion.entries) {
                entry
                    .qualified_nodes
                    .extend(expanded_entry.qualified_nodes.iter().cloned());
            }
        }
        sort_dedup_case_insensitive(qualified_instances);
        for entry in &mut source.entries {
            sort_dedup_case_insensitive(&mut entry.qualified_nodes);
        }
    }
    Ok(())
}

fn filter_unknown_entries(
    netlist: &mut Netlist,
    namespace: &HashSet<String>,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    for (record_index, record) in netlist.startup_directives.iter_mut().enumerate() {
        poll_parse_abort(abort, record_index)?;
        if matches!(record.disposition, StartupDirectiveDisposition::Ignored(_)) {
            continue;
        }
        let mut applied = 0usize;
        let mut ignored = 0usize;
        for entry in &mut record.entries {
            let exists = !entry.qualified_nodes.is_empty()
                && entry
                    .qualified_nodes
                    .iter()
                    .any(|node| namespace.contains(&canonical_symbol(node)));
            if exists
                || entry.qualified_nodes.is_empty()
                    && matches!(record.scope, StartupDirectiveScope::Subcircuit { .. })
            {
                // A valid definition with no concrete instances has no
                // startup-topology target to reject.
                applied += 1;
            } else {
                entry.disposition =
                    StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::UndefinedNode);
                ignored += 1;
            }
        }
        record.disposition = match (applied, ignored) {
            (0, _) => StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::UndefinedNode),
            (_, 0) => StartupDirectiveDisposition::Applied,
            _ => StartupDirectiveDisposition::PartiallyApplied,
        };
    }
    Ok(())
}

#[derive(Default)]
struct UnownedStartupState {
    top_initial_conditions: Vec<InitialCondition>,
    top_node_sets: Vec<NodeSet>,
    scoped: Vec<(String, Vec<InitialCondition>, Vec<NodeSet>)>,
}

fn extract_unowned_startup_state(netlist: &Netlist) -> UnownedStartupState {
    let mut state = UnownedStartupState {
        top_initial_conditions: netlist.initial_conditions.clone(),
        top_node_sets: netlist.node_sets.clone(),
        scoped: netlist
            .subcircuits
            .iter()
            .map(|subcircuit| {
                (
                    subcircuit.name.clone(),
                    subcircuit.initial_conditions.clone(),
                    subcircuit.node_sets.clone(),
                )
            })
            .collect(),
    };
    for record in &netlist.startup_directives {
        if matches!(record.disposition, StartupDirectiveDisposition::Ignored(_)) {
            continue;
        }
        let (initial_conditions, node_sets) = match &record.scope {
            StartupDirectiveScope::TopLevel => {
                (&mut state.top_initial_conditions, &mut state.top_node_sets)
            }
            StartupDirectiveScope::Subcircuit {
                qualified_definition,
                ..
            } => {
                let Some((_, initial_conditions, node_sets)) = state
                    .scoped
                    .iter_mut()
                    .find(|(name, _, _)| name.eq_ignore_ascii_case(qualified_definition))
                else {
                    continue;
                };
                (initial_conditions, node_sets)
            }
        };
        for entry in &record.entries {
            if matches!(entry.disposition, StartupDirectiveDisposition::Ignored(_)) {
                continue;
            }
            match record.kind {
                StartupDirectiveKind::Ic => {
                    remove_matching_initial_condition(initial_conditions, entry)
                }
                StartupDirectiveKind::NodeSet => remove_matching_node_set(node_sets, entry),
            }
        }
    }
    state
}

fn remove_matching_initial_condition(
    entries: &mut Vec<InitialCondition>,
    owned: &super::StartupDirectiveEntry,
) {
    if let Some(index) = entries
        .iter()
        .position(|entry| entry.node.eq_ignore_ascii_case(&owned.execution_node))
    {
        entries.remove(index);
    }
}

fn remove_matching_node_set(entries: &mut Vec<NodeSet>, owned: &super::StartupDirectiveEntry) {
    if let Some(index) = entries
        .iter()
        .position(|entry| entry.node.eq_ignore_ascii_case(&owned.execution_node))
    {
        entries.remove(index);
    }
}

fn rebuild_effective_vectors(
    netlist: &mut Netlist,
    unowned: &UnownedStartupState,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    netlist.initial_conditions.clear();
    netlist.node_sets.clear();
    for subcircuit in &mut netlist.subcircuits {
        subcircuit.initial_conditions.clear();
        subcircuit.node_sets.clear();
    }

    for (record_index, record) in netlist.startup_directives.iter().enumerate() {
        poll_parse_abort(abort, record_index)?;
        if matches!(record.disposition, StartupDirectiveDisposition::Ignored(_)) {
            continue;
        }
        match &record.scope {
            StartupDirectiveScope::TopLevel => {
                append_applied_entries(
                    record,
                    &mut netlist.initial_conditions,
                    &mut netlist.node_sets,
                );
            }
            StartupDirectiveScope::Subcircuit {
                qualified_definition,
                ..
            } => {
                if let Some(subcircuit) = netlist
                    .subcircuits
                    .iter_mut()
                    .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case(qualified_definition))
                {
                    append_applied_entries(
                        record,
                        &mut subcircuit.initial_conditions,
                        &mut subcircuit.node_sets,
                    );
                }
            }
        }
    }
    netlist
        .initial_conditions
        .extend(unowned.top_initial_conditions.iter().cloned());
    netlist
        .node_sets
        .extend(unowned.top_node_sets.iter().cloned());
    for (name, initial_conditions, node_sets) in &unowned.scoped {
        if let Some(subcircuit) = netlist
            .subcircuits
            .iter_mut()
            .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case(name))
        {
            subcircuit
                .initial_conditions
                .extend(initial_conditions.iter().cloned());
            subcircuit.node_sets.extend(node_sets.iter().cloned());
        }
    }
    Ok(())
}

fn append_applied_entries(
    record: &StartupDirectiveRecord,
    initial_conditions: &mut Vec<InitialCondition>,
    node_sets: &mut Vec<NodeSet>,
) {
    for entry in &record.entries {
        if matches!(entry.disposition, StartupDirectiveDisposition::Ignored(_)) {
            continue;
        }
        match record.kind {
            StartupDirectiveKind::Ic => initial_conditions.push(InitialCondition {
                node: entry.execution_node.clone(),
                voltage: entry.voltage,
                voltage_expr: entry.voltage_expr.clone(),
            }),
            StartupDirectiveKind::NodeSet => node_sets.push(NodeSet {
                node: entry.execution_node.clone(),
                voltage: entry.voltage,
                voltage_expr: entry.voltage_expr.clone(),
            }),
        }
    }
}

fn canonical_symbol(symbol: &str) -> String {
    symbol.trim().replace(':', ".").to_ascii_uppercase()
}

fn sort_dedup_case_insensitive(values: &mut Vec<String>) {
    values.sort_unstable_by_key(|value| value.to_ascii_uppercase());
    values.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
}

fn remove_projected_diagnostics(netlist: &mut Netlist) {
    netlist.diagnostics.retain(|diagnostic| {
        !matches!(
            diagnostic.code.as_str(),
            "startup-empty-directive" | "startup-undefined-node" | "startup-scoped-global-node"
        )
    });
}

fn project_typed_diagnostics(netlist: &mut Netlist) {
    for diagnostic in netlist.startup_diagnostics() {
        let directive = diagnostic.kind.as_spice_directive();
        let message = match diagnostic.code {
            StartupDiagnosticCode::EmptyDirective => {
                format!("{directive} statement has no assignments and was ignored")
            }
            StartupDiagnosticCode::UndefinedNode => format!(
                "{directive} ignored undefined node(s): {}",
                diagnostic.canonical_nodes.join(", ")
            ),
            StartupDiagnosticCode::ScopedGlobalNode => format!(
                "{directive} card in a subcircuit targets a global node and was ignored in full: {}",
                diagnostic.canonical_nodes.join(", ")
            ),
        };
        netlist.diagnostics.push(ParseDiagnostic::warning(
            diagnostic.origins.first().map_or(0, |origin| origin.line),
            diagnostic.code.as_str(),
            message,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::CountingAbort;
    use crate::netlist::{NetlistParseOptions, ParameterRedefinitionPolicy, StatisticalParamMode};

    fn xyce_options() -> NetlistParseOptions {
        NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Sample,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
        }
    }

    fn parse_xyce(source: &str) -> Result<Netlist, ParseError> {
        crate::netlist::parse_netlist_with_options(source, xyce_options())
    }

    #[test]
    fn empty_cards_are_retained_as_typed_parse_warnings() {
        for (card, kind) in [
            (".IC", StartupDirectiveKind::Ic),
            (".NODESET", StartupDirectiveKind::NodeSet),
        ] {
            let source = format!("empty startup\nV1 1 0 1\n{card}\n.OP\n.END\n");
            let netlist = parse_xyce(&source).expect("empty startup card is warning-only");
            assert!(netlist.initial_conditions.is_empty());
            assert!(netlist.node_sets.is_empty());
            assert_eq!(netlist.startup_directives.len(), 1);
            let record = &netlist.startup_directives[0];
            assert_eq!(record.kind, kind);
            assert_eq!(record.origin.line, 3);
            assert!(record.entries.is_empty());
            assert_eq!(
                record.disposition,
                StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::EmptyDirective)
            );
            assert_eq!(
                netlist.startup_diagnostics()[0].stage,
                super::super::StartupDiagnosticStage::Parse
            );
        }
    }

    #[test]
    fn unknown_top_level_entries_are_filtered_individually_and_reported_stably() {
        let netlist = parse_xyce(
            "mixed top-level IC\n\
             V1 Known 0 1\n\
             .IC V(zeta)=1 V(Known)=2 V(alpha)=3 V(ZETA)=4\n\
             .OP\n\
             .END\n",
        )
        .expect("unknown startup nodes are warnings");
        assert_eq!(netlist.initial_conditions.len(), 1);
        assert_eq!(netlist.initial_conditions[0].node, "KNOWN");
        assert_eq!(
            netlist.startup_directives[0].entries[1].authored_node,
            "Known"
        );
        assert_eq!(netlist.initial_conditions[0].voltage, 2.0);
        let record = &netlist.startup_directives[0];
        assert_eq!(record.entries.len(), 4);
        assert_eq!(
            record.disposition,
            StartupDirectiveDisposition::PartiallyApplied
        );
        let diagnostic = &netlist.startup_diagnostics()[0];
        assert_eq!(diagnostic.code, StartupDiagnosticCode::UndefinedNode);
        assert_eq!(diagnostic.canonical_nodes, ["ALPHA", "ZETA"]);
    }

    #[test]
    fn scoped_global_discards_the_whole_card_but_reports_only_global_targets() {
        let netlist = parse_xyce(
            "scoped global\n\
             .GLOBAL VCC\n\
             X1 in out CELL\n\
             V1 in 0 1\n\
             R1 out 0 1\n\
             .SUBCKT CELL a b\n\
             R2 a mid 1\n\
             R3 mid b 1\n\
             .IC V(mid)=0.5 V(VCC)=1\n\
             .ENDS\n\
             .OP\n\
             .END\n",
        )
        .expect("scoped global startup is warning-only");
        let subcircuit = netlist
            .subcircuits
            .iter()
            .find(|definition| definition.name.eq_ignore_ascii_case("CELL"))
            .expect("CELL definition");
        assert!(subcircuit.initial_conditions.is_empty());
        let record = &netlist.startup_directives[0];
        assert_eq!(
            record.disposition,
            StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::ScopedGlobalNode)
        );
        assert_eq!(
            record.entries[0].disposition,
            StartupDirectiveDisposition::Applied,
            "valid sibling is discarded by the card, not mislabeled global"
        );
        assert_eq!(netlist.startup_diagnostics()[0].canonical_nodes, ["VCC"]);
    }

    #[test]
    fn every_xyce_dollar_g_prefix_is_global() {
        let netlist = parse_xyce(
            "implicit global\n\
             X1 1 0 CELL\n\
             V1 1 0 1\n\
             .SUBCKT CELL a b\n\
             R1 a b 1\n\
             .NODESET V($Gfoo)=1\n\
             .ENDS\n\
             .OP\n\
             .END\n",
        )
        .expect("implicit global warning");
        assert!(matches!(
            netlist.startup_directives[0].disposition,
            StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::ScopedGlobalNode)
        ));
    }

    #[test]
    fn scoped_records_aggregate_all_instance_paths_and_validate_missing_locals() {
        let netlist = parse_xyce(
            "qualified startup\n\
             V1 in 0 1\n\
             Xb in out2 CELL\n\
             Xa in out1 CELL\n\
             R1 out1 0 1\n\
             R2 out2 0 1\n\
             .SUBCKT CELL a b\n\
             R3 a mid 1\n\
             R4 mid b 1\n\
             .NODESET V(mid)=0.5 V(missing)=2\n\
             .ENDS\n\
             .OP\n\
             .END\n",
        )
        .expect("scoped missing local is warning-only");
        let record = &netlist.startup_directives[0];
        let StartupDirectiveScope::Subcircuit {
            qualified_instances,
            ..
        } = &record.scope
        else {
            panic!("expected subcircuit scope");
        };
        assert_eq!(qualified_instances, &["Xa", "Xb"]);
        assert_eq!(record.entries[0].qualified_nodes, ["Xa.MID", "Xb.MID"]);
        assert_eq!(
            record.entries[1].qualified_nodes,
            ["Xa.MISSING", "Xb.MISSING"]
        );
        assert!(matches!(
            record.entries[1].disposition,
            StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::UndefinedNode)
        ));
        let flattened =
            crate::netlist::flatten_netlist_with_models(&netlist).expect("validated deck flattens");
        assert_eq!(flattened.scoped_node_sets.len(), 2);
        assert!(
            flattened
                .scoped_node_sets
                .iter()
                .all(|entry| entry.node.ends_with(".MID"))
        );
    }

    #[test]
    fn control_nodes_share_the_authoritative_output_namespace() {
        let netlist = parse_xyce(
            "control node startup\n\
             V1 in 0 1\n\
             E1 out 0 ctrl 0 2\n\
             R1 out 0 1\n\
             .IC V(ctrl)=0.25\n\
             .OP\n\
             .END\n",
        )
        .expect("VCVS control node is a real startup target");
        assert_eq!(netlist.initial_conditions.len(), 1);
        assert_eq!(
            netlist.startup_directives[0].entries[0].disposition,
            StartupDirectiveDisposition::Applied
        );
    }

    #[test]
    fn xyce_conflict_uses_first_effective_opposite_modes_and_ngspice_remains_permissive() {
        for (first, second, first_kind, second_kind) in [
            (
                ".IC V(1)=1",
                ".NODESET V(1)=2",
                StartupDirectiveKind::Ic,
                StartupDirectiveKind::NodeSet,
            ),
            (
                ".NODESET V(1)=2",
                ".IC V(1)=1",
                StartupDirectiveKind::NodeSet,
                StartupDirectiveKind::Ic,
            ),
        ] {
            let source = format!("conflict\nV1 1 0 1\n{first}\n{second}\n.OP\n.END\n");
            let error = parse_xyce(&source).expect_err("Xyce startup modes conflict");
            let ParseError::StartupDirectiveConflict(error) = error else {
                panic!("unexpected error: {error}");
            };
            assert_eq!(error.first_kind, first_kind);
            assert_eq!(error.conflicting_kind, second_kind);
            assert_eq!(error.first.line, 3);
            assert_eq!(error.conflicting.line, 4);
        }

        Netlist::parse("ngspice coexistence\nV1 1 0 1\n.IC V(1)=1\n.NODESET V(1)=2\n.OP\n.END\n")
            .expect("default/ngspice compatibility remains permissive");
    }

    #[test]
    fn empty_and_scoped_global_cards_do_not_establish_xyce_mode() {
        parse_xyce("empty does not conflict\nV1 1 0 1\n.IC\n.NODESET V(1)=2\n.OP\n.END\n")
            .expect("empty IC does not establish mode");
        parse_xyce(
            "discarded does not conflict\n\
             V1 1 0 1\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL a b\n\
             R1 a b 1\n\
             .IC V($Gfoo)=1\n\
             .ENDS\n\
             .NODESET V(1)=2\n\
             .OP\n\
             .END\n",
        )
        .expect("scoped-global IC is discarded before mode conflict");
    }

    #[test]
    fn uninstantiated_scoped_cards_do_not_establish_xyce_mode() {
        parse_xyce(
            "unused scoped mode\n\
             V1 1 0 1\n\
             .IC V(1)=0.25\n\
             .SUBCKT UNUSED a b\n\
             R1 a b 1\n\
             .NODESET V(a)=0.5\n\
             .ENDS\n\
             .OP\n\
             .END\n",
        )
        .expect("an unused definition contributes no effective startup mode");

        let error = parse_xyce(
            "instantiated nested mode\n\
             V1 1 0 1\n\
             X1 1 0 PARENT\n\
             .IC V(1)=0.25\n\
             .SUBCKT PARENT a b\n\
             XCHILD a b CHILD\n\
             .SUBCKT CHILD c d\n\
             R1 c d 1\n\
             .NODESET V(c)=0.5\n\
             .ENDS CHILD\n\
             .ENDS PARENT\n\
             .OP\n\
             .END\n",
        )
        .expect_err("an instantiated nested definition contributes its startup mode");
        assert!(matches!(error, ParseError::StartupDirectiveConflict(_)));
    }

    #[test]
    fn scoped_global_cards_remain_effective_outside_xyce_mode() {
        let netlist = Netlist::parse(
            "ngspice scoped global\n\
             .GLOBAL VCC\n\
             V1 VCC 0 1\n\
             X1 VCC 0 CELL\n\
             .SUBCKT CELL a b\n\
             R1 a b 1\n\
             .IC V(VCC)=0.75\n\
             .ENDS\n\
             .OP\n\
             .END\n",
        )
        .expect("default/ngspice mode preserves scoped global startup cards");
        let record = &netlist.startup_directives[0];
        assert_eq!(record.disposition, StartupDirectiveDisposition::Applied);
        assert!(
            netlist
                .startup_diagnostics()
                .iter()
                .all(|diagnostic| diagnostic.code != StartupDiagnosticCode::ScopedGlobalNode)
        );
        assert_eq!(netlist.subcircuits[0].initial_conditions[0].voltage, 0.75);
    }

    #[test]
    fn repeated_validation_is_idempotent_and_abort_rolls_back() {
        let mut netlist = parse_xyce(
            "revalidation\n\
             V1 1 0 1\n\
             .IC V(missing)=1 V(1)=2\n\
             .OP\n\
             .END\n",
        )
        .expect("initial validation");
        let records = netlist.startup_directives.clone();
        let effective = netlist
            .initial_conditions
            .iter()
            .map(|entry| (entry.node.clone(), entry.voltage.to_bits()))
            .collect::<Vec<_>>();
        validate_startup_directives(&mut netlist).expect("second validation succeeds");
        assert_eq!(netlist.startup_directives, records);
        assert_eq!(
            netlist
                .initial_conditions
                .iter()
                .map(|entry| (entry.node.clone(), entry.voltage.to_bits()))
                .collect::<Vec<_>>(),
            effective
        );

        let before_records = netlist.startup_directives.clone();
        let before_diagnostics = netlist.diagnostics.clone();
        let abort = CountingAbort::new(3);
        assert!(validate_startup_directives_with_abort(&mut netlist, &abort).is_err());
        assert_eq!(netlist.startup_directives, before_records);
        assert_eq!(netlist.diagnostics, before_diagnostics);
        assert_eq!(
            netlist
                .initial_conditions
                .iter()
                .map(|entry| (entry.node.clone(), entry.voltage.to_bits()))
                .collect::<Vec<_>>(),
            effective
        );
    }

    #[test]
    fn programmatic_numeric_startup_without_sidecar_survives_validation() {
        let mut netlist =
            Netlist::parse("programmatic\nV1 1 0 1\n.OP\n.END\n").expect("base deck parses");
        netlist.initial_conditions.push(InitialCondition {
            node: "1".to_string(),
            voltage: 0.75,
            voltage_expr: None,
        });
        netlist.node_sets.push(NodeSet {
            node: "1".to_string(),
            voltage: 0.25,
            voltage_expr: None,
        });
        validate_startup_directives(&mut netlist).expect("programmatic state validates");
        assert_eq!(netlist.initial_conditions.len(), 1);
        assert_eq!(netlist.initial_conditions[0].voltage, 0.75);
        assert_eq!(netlist.node_sets.len(), 1);
        assert_eq!(netlist.node_sets[0].voltage, 0.25);
    }

    #[test]
    fn parsed_sidecar_ownership_is_stable_across_numeric_ast_edits() {
        let mut netlist =
            Netlist::parse("mixed startup ownership\nV1 1 0 1\n.IC V(1)=0.25\n.OP\n.END\n")
                .expect("parsed startup deck");
        netlist.initial_conditions[0].voltage = 9.0;
        validate_startup_directives(&mut netlist).expect("revalidation restores authored payload");
        assert_eq!(netlist.initial_conditions.len(), 1);
        assert_eq!(netlist.initial_conditions[0].voltage, 0.25);

        netlist.initial_conditions.push(InitialCondition {
            node: "1".to_string(),
            voltage: 0.75,
            voltage_expr: None,
        });
        validate_startup_directives(&mut netlist)
            .expect("an appended programmatic override remains unowned");
        assert_eq!(netlist.initial_conditions.len(), 2);
        assert_eq!(netlist.initial_conditions[0].voltage, 0.25);
        assert_eq!(netlist.initial_conditions[1].voltage, 0.75);
    }

    #[test]
    fn undefined_nodes_aggregate_across_cards_while_global_occurrences_do_not_dedup() {
        let netlist = parse_xyce(
            "aggregate unknowns\n\
             V1 1 0 1\n\
             .IC V(zeta)=1\n\
             .IC V(alpha)=2 V(ZETA)=3\n\
             .OP\n\
             .END\n",
        )
        .expect("unknowns are warning-only");
        let undefined = netlist
            .startup_diagnostics()
            .into_iter()
            .filter(|diagnostic| diagnostic.code == StartupDiagnosticCode::UndefinedNode)
            .collect::<Vec<_>>();
        assert_eq!(undefined.len(), 1);
        assert_eq!(undefined[0].canonical_nodes, ["ALPHA", "ZETA"]);
        assert_eq!(undefined[0].origins.len(), 2);

        let scoped = parse_xyce(
            "global occurrences\n\
             X1 1 0 CELL\n\
             V1 1 0 1\n\
             .SUBCKT CELL a b\n\
             R1 a b 1\n\
             .NODESET V($Gfoo)=1 V($GFOO)=2\n\
             .ENDS\n\
             .OP\n\
             .END\n",
        )
        .expect("global occurrences warn");
        let globals = scoped
            .startup_diagnostics()
            .into_iter()
            .filter(|diagnostic| diagnostic.code == StartupDiagnosticCode::ScopedGlobalNode)
            .collect::<Vec<_>>();
        assert_eq!(globals.len(), 2, "authored occurrences remain distinct");
        assert_eq!(globals[0].canonical_nodes, ["$GFOO"]);
        assert_eq!(globals[1].canonical_nodes, ["$GFOO"]);
    }

    #[test]
    fn circuit_build_revalidates_programmatically_changed_sidecar_state() {
        let mut netlist = Netlist::parse(
            "build validation\n\
             V1 1 0 1\n\
             .IC V(1)=1\n\
             .NODESET V(1)=2\n\
             .OP\n\
             .END\n",
        )
        .expect("ngspice parse permits both modes");
        netlist
            .params
            .set_expression_dialect(ExpressionDialect::Xyce);
        let error = crate::Engine::default()
            .build_circuit(&netlist)
            .expect_err("builder enforces Xyce startup conflict");
        assert!(
            error
                .to_string()
                .contains("Cannot set both .IC and .NODESET simultaneously")
        );
    }
}
