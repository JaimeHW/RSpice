use std::{
    borrow::Cow,
    collections::{BTreeSet, HashMap, HashSet},
};

use crate::canonical_ir::{
    CanonicalIrArtifact, CanonicalValueType, EquationId, ExprId, HirAnalogOperator, HirExprKind,
    HirStatement, MirBranchRef, MirEquation, MirEquationKind, MirParameterSlot,
};

use super::expr::{
    BranchCurrentSlot, DdtSlots, LoweredExpr, LoweredVariable, branch_pair_key,
    comparison_operator, is_analysis_name, is_intrinsic_name as expr_is_intrinsic_name,
    lower_assignment_expr_with_branch_currents, lower_equation_expr_with_branch_currents,
    lower_reactive_assignment_expr_with_branch_currents, lower_reactive_expr_with_branch_currents,
    lower_value_assignment_expr_with_branch_currents, normalize_analysis_query,
    parameter_field_names, unique_identifiers,
};
use super::{
    GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames, RustTranspileOptions,
};

const MAX_STAMP_HELPER_LINES: usize = 512;
const MAX_STAMP_HELPERS_PER_MODULE: usize = 16;
const DENSE_STAMP_DERIVATIVE_THRESHOLD: usize = 4;
const COMPACT_EQUATION_EXPR_NODE_THRESHOLD: usize = 32;

#[derive(Debug, Clone, Copy, Default)]
struct StampCommonUsage {
    param_given: bool,
    time: bool,
}

impl StampCommonUsage {
    fn from_artifact(artifact: &CanonicalIrArtifact) -> Self {
        let mut usage = Self::default();
        for expression in &artifact.mir.expressions {
            match &expression.kind {
                HirExprKind::SystemFunction { name, .. } | HirExprKind::Call { name, .. } => {
                    if name.eq_ignore_ascii_case("$param_given") {
                        usage.param_given = true;
                    } else if name.eq_ignore_ascii_case("$abstime")
                        || name.eq_ignore_ascii_case("$realtime")
                    {
                        usage.time = true;
                    }
                }
                _ => {}
            }
        }
        usage
    }

    fn for_helper_block(self, block: &str) -> StampHelperCommonUsage {
        StampHelperCommonUsage {
            ctx: block.contains("ctx"),
            stamper: block.contains("stamper."),
            scratch: block.contains("scratch.") || block.contains("s."),
            params: block.contains("params.") || block.contains("p."),
            nodes: block.contains("self.nodes")
                || block.contains("nodes[")
                || block.contains("nodes,"),
            branches: block.contains("self.branches")
                || block.contains("branches[")
                || block.contains("branches,"),
            param_given: self.param_given
                && (block.contains("self.param_given[") || block.contains("param_given[")),
            multiplicity: block.contains("self.multiplicity") || block.contains("multiplicity"),
            time: self.time && (block.contains("self.time") || block.contains("time")),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct StampHelperCommonUsage {
    ctx: bool,
    stamper: bool,
    scratch: bool,
    params: bool,
    nodes: bool,
    branches: bool,
    param_given: bool,
    multiplicity: bool,
    time: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct StampOperatorUsage {
    ddt: bool,
    idt: bool,
}

impl StampOperatorUsage {
    fn transient(ddt_slots: &DdtSlots) -> Self {
        Self {
            ddt: ddt_slots.len() > 0,
            idt: ddt_slots.idt_len() > 0,
        }
    }

    fn has_any(self) -> bool {
        self.ddt || self.idt
    }

    fn for_helper_block(self, block: &str) -> Self {
        Self {
            ddt: self.ddt && block.contains("eval_ddt("),
            idt: self.idt && block.contains("eval_idt("),
        }
    }
}

fn stamp_helper_call_args(
    common_usage: StampHelperCommonUsage,
    operator_usage: StampOperatorUsage,
) -> String {
    let mut args = Vec::new();
    if common_usage.ctx {
        args.push("ctx");
    }
    if common_usage.stamper {
        args.push("stamper");
    }
    if common_usage.scratch {
        args.push("s");
    }
    if common_usage.params {
        args.push("p");
    }
    if common_usage.nodes {
        args.push("nodes");
    }
    if common_usage.branches {
        args.push("branches");
    }
    if common_usage.param_given {
        args.push("param_given");
    }
    if common_usage.multiplicity {
        args.push("multiplicity");
    }
    if common_usage.time {
        args.push("time");
    }
    if operator_usage.has_any() {
        args.push("ddt_active");
    }
    if operator_usage.ddt {
        args.push("ddt_scale");
    }
    if operator_usage.idt {
        args.push("idt_scale");
    }
    if operator_usage.ddt {
        args.push("ddt_state_current");
        args.push("ddt_state_previous");
        args.push("ddt_state_initialized");
    }
    if operator_usage.idt {
        args.push("idt_state_current");
        args.push("idt_state_previous");
        args.push("idt_state_initialized");
    }
    args.join(", ")
}

fn stamp_helper_common_params(
    common_usage: StampHelperCommonUsage,
    stamper_type: &str,
    scratch_type: &str,
) -> String {
    let mut params = String::new();
    if common_usage.ctx {
        params.push_str("        ctx: &GeneratedEvalContext<'_>,\n");
    }
    if common_usage.stamper {
        params.push_str(&format!("        stamper: &mut {stamper_type}<'_>,\n"));
    }
    if common_usage.scratch {
        params.push_str(&format!("        scratch: &mut {scratch_type},\n"));
    }
    if common_usage.params {
        params.push_str("        p: &Parameters,\n");
    }
    if common_usage.nodes {
        params.push_str("        nodes: &[usize; Instance::NODE_COUNT],\n");
    }
    if common_usage.branches {
        params.push_str("        branches: &[usize; Instance::BRANCH_COUNT],\n");
    }
    if common_usage.param_given {
        params.push_str("        param_given: &[bool; Instance::PARAMETER_COUNT],\n");
    }
    if common_usage.multiplicity {
        params.push_str("        multiplicity: f64,\n");
    }
    if common_usage.time {
        params.push_str("        time: f64,\n");
    }
    params
}

fn stamp_helper_operator_params(operator_usage: StampOperatorUsage) -> String {
    let mut params = String::new();
    if operator_usage.has_any() {
        params.push_str("        ddt_active: bool,\n");
    }
    if operator_usage.ddt {
        params.push_str("        ddt_scale: f64,\n");
    }
    if operator_usage.idt {
        params.push_str("        idt_scale: f64,\n");
    }
    if operator_usage.ddt {
        params.push_str(
            "        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],\n        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],\n        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],\n",
        );
    }
    if operator_usage.idt {
        params.push_str(
            "        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],\n        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],\n        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],\n",
        );
    }
    params
}

pub fn generate_device(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
) -> Result<GeneratedRustDevice, RustBackendError> {
    reject_unsupported_model_shape(artifact)?;

    let source_file_name = artifact.metadata.source_package.as_str();
    let names = RustDeviceNames::new(
        source_file_name,
        artifact.mir.module_name.as_str(),
        artifact.metadata.source_digest.as_str(),
    );
    let parameter_fields = parameter_field_names(artifact);
    let variable_fields = variable_local_names(artifact);
    let ddt_slots = collect_ddt_slots(artifact)?;
    let reactive_liveness = if ddt_slots.len() == 0 {
        ReactiveLiveness::default()
    } else {
        collect_reactive_liveness(artifact)?
    };
    let transient_liveness = collect_transient_liveness(artifact)?;
    let potential_branch_slots = collect_potential_branch_slots(artifact)?;

    let stamp_files = compact_stamp_files(generate_stamp_file(
        artifact,
        options,
        &parameter_fields,
        &variable_fields,
        &ddt_slots,
        &transient_liveness,
        &reactive_liveness,
        &potential_branch_slots,
    )?);
    let mut files = vec![
        GeneratedRustFile {
            relative_path: "mod.rs".to_string(),
            contents: generate_mod_file(),
        },
        GeneratedRustFile {
            relative_path: "state.rs".to_string(),
            contents: generate_state_file(
                artifact,
                options,
                &parameter_fields,
                ddt_slots.len(),
                ddt_slots.idt_len(),
                potential_branch_slots.len(),
            )?,
        },
        GeneratedRustFile {
            relative_path: "stamp.rs".to_string(),
            contents: stamp_files.stamp,
        },
    ];
    files.extend(stamp_files.helpers);

    Ok(GeneratedRustDevice {
        module_name: artifact.mir.module_name.to_string(),
        public_model_name: names.public_model_name,
        folder_name: names.folder,
        source_digest: artifact.metadata.source_digest.to_string(),
        files,
    })
}

fn compact_stamp_files(mut files: StampFiles) -> StampFiles {
    files.stamp = compact_generated_stamp_surface(files.stamp);
    for helper in &mut files.helpers {
        helper.contents = compact_generated_stamp_surface(std::mem::take(&mut helper.contents));
        helper.contents =
            compact_generated_stamp_helper_surface(std::mem::take(&mut helper.contents));
    }
    files
}

fn compact_generated_stamp_helper_surface(mut source: String) -> String {
    for (from, to) in [("&nodes", "nodes"), ("&branches", "branches")] {
        source = source.replace(from, to);
    }
    source
}

fn compact_generated_stamp_surface(mut source: String) -> String {
    for (from, to) in [
        ("scratch.reactive_node_derivatives", "scratch.rdn"),
        ("scratch.reactive_branch_derivatives", "scratch.rdb"),
        ("scratch.reactive_values", "scratch.rv"),
        ("scratch.bool_values", "scratch.b"),
        ("scratch.node_derivatives", "scratch.dn"),
        ("scratch.branch_derivatives", "scratch.db"),
        ("scratch.values", "scratch.v"),
        (".node_derivatives", ".dn"),
        (".branch_derivatives", ".db"),
        ("let mut scratch = ", "let mut s = "),
        ("&mut scratch", "&mut s"),
        ("scratch: &mut ReactiveScratch", "s: &mut ReactiveScratch"),
        ("scratch: &mut Scratch", "s: &mut Scratch"),
        ("scratch.", "s."),
        ("type AdValue = GenericAdValue", "type A = GenericAdValue"),
        ("AdValue::", "A::"),
        (": AdValue", ": A"),
        ("params.", "p."),
        ("&self.nodes", "nodes"),
        ("self.nodes[", "nodes["),
        ("&self.branches", "branches"),
        ("self.branches[", "branches["),
        ("self.multiplicity", "multiplicity"),
        ("self.param_given[", "param_given["),
        ("self.time", "time"),
    ] {
        source = source.replace(from, to);
    }
    merge_adjacent_simple_if_blocks(cache_context_reads(source))
}

#[derive(Debug)]
struct SimpleIfBlock<'a> {
    indent: &'a str,
    condition: &'a str,
    body: Vec<&'a str>,
    next_index: usize,
}

fn merge_adjacent_simple_if_blocks(source: String) -> String {
    let had_trailing_newline = source.ends_with('\n');
    let lines: Vec<_> = source.lines().collect();
    let mut merged = Vec::with_capacity(lines.len());
    let mut index = 0usize;

    while index < lines.len() {
        let Some(first) = parse_simple_if_block(&lines, index) else {
            merged.push(lines[index].to_string());
            index += 1;
            continue;
        };
        if !is_merge_safe_if_body(&first.body) {
            merged.extend(
                lines[index..first.next_index]
                    .iter()
                    .map(|line| line.to_string()),
            );
            index = first.next_index;
            continue;
        }

        let mut body = first.body.clone();
        let mut next_index = first.next_index;
        let mut found_match = false;
        loop {
            let mut candidate_index = next_index;
            while candidate_index < lines.len() && lines[candidate_index].trim().is_empty() {
                candidate_index += 1;
            }
            let Some(next) = parse_simple_if_block(&lines, candidate_index) else {
                break;
            };
            if next.indent != first.indent
                || next.condition != first.condition
                || !is_merge_safe_if_body(&next.body)
            {
                break;
            }

            body.extend(next.body);
            next_index = next.next_index;
            found_match = true;
        }

        if found_match {
            merged.push(format!("{}if {} {{", first.indent, first.condition));
            merged.extend(body.iter().map(|line| (*line).to_string()));
            merged.push(format!("{}}}", first.indent));
            index = next_index;
        } else {
            merged.extend(
                lines[index..first.next_index]
                    .iter()
                    .map(|line| line.to_string()),
            );
            index = first.next_index;
        }
    }

    let mut out = merged.join("\n");
    if had_trailing_newline {
        out.push('\n');
    }
    out
}

fn parse_simple_if_block<'a>(lines: &'a [&'a str], index: usize) -> Option<SimpleIfBlock<'a>> {
    let line = *lines.get(index)?;
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    let condition = trimmed.strip_prefix("if ")?.strip_suffix(" {")?;
    if condition.is_empty() || condition.contains(" else ") {
        return None;
    }

    let close = format!("{indent}}}");
    let mut body = Vec::new();
    let mut next_index = index + 1;
    while next_index < lines.len() {
        if lines[next_index] == close {
            return Some(SimpleIfBlock {
                indent,
                condition,
                body,
                next_index: next_index + 1,
            });
        }
        body.push(lines[next_index]);
        next_index += 1;
    }

    None
}

fn is_merge_safe_if_body(body: &[&str]) -> bool {
    !body.is_empty()
        && body.iter().all(|line| {
            let trimmed = line.trim_start();
            !(trimmed.starts_with("let ")
                || trimmed.starts_with("if ")
                || trimmed.starts_with("else")
                || trimmed.starts_with('}')
                || trimmed.starts_with("for ")
                || trimmed.starts_with("while ")
                || trimmed.starts_with("match ")
                || trimmed.starts_with("return")
                || trimmed.contains(" else ")
                || trimmed.contains("__rspice_"))
        })
}

fn cache_context_reads(source: String) -> String {
    let source = cache_context_reads_after_anchor(
        source,
        "        let branches = self.branches;\n",
        "        let branches = self.branches;\n",
    );
    let source = cache_context_reads_after_anchor(
        source,
        "        let branches = &(*self).branches;\n",
        "        let branches = &(*self).branches;\n",
    );
    cache_context_reads_after_anchor(source, "    ) {\n", "\n    pub(super) fn ")
}

fn cache_context_reads_after_anchor(source: String, anchor: &str, next_marker: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut remaining = source.as_str();
    while let Some(anchor_start) = remaining.find(anchor) {
        let anchor_end = anchor_start + anchor.len();
        out.push_str(&remaining[..anchor_end]);
        remaining = &remaining[anchor_end..];

        let segment_end = remaining.find(next_marker).unwrap_or(remaining.len());
        out.push_str(&cache_context_reads_in_segment(&remaining[..segment_end]));
        remaining = &remaining[segment_end..];
    }
    out.push_str(remaining);
    out
}

fn cache_context_reads_in_segment(segment: &str) -> String {
    let node_indices = collect_indexed_calls(segment, "ctx.node_voltage(nodes[", "])");
    let branch_indices = collect_indexed_calls(segment, "ctx.branch_current(branches[", "])");
    let uses_temperature = segment.contains("ctx.temperature()");
    let uses_thermal_voltage = segment.contains("ctx.thermal_voltage()");
    if node_indices.is_empty()
        && branch_indices.is_empty()
        && !uses_temperature
        && !uses_thermal_voltage
    {
        return segment.to_string();
    }

    let mut body = segment.to_string();
    if uses_temperature {
        body = body.replace("ctx.temperature()", "ctx_temp");
    }
    if uses_thermal_voltage {
        body = body.replace("ctx.thermal_voltage()", "ctx_thermal_vt");
    }
    for index in &node_indices {
        body = body.replace(
            &format!("ctx.node_voltage(nodes[{index}])"),
            &format!("nv{index}"),
        );
    }
    for index in &branch_indices {
        body = body.replace(
            &format!("ctx.branch_current(branches[{index}])"),
            &format!("bi{index}"),
        );
    }

    let mut cached = String::new();
    if uses_temperature {
        cached.push_str("        let ctx_temp = ctx.temperature();\n");
    }
    if uses_thermal_voltage {
        cached.push_str("        let ctx_thermal_vt = ctx.thermal_voltage();\n");
    }
    for index in node_indices {
        cached.push_str(&format!(
            "        let nv{index} = ctx.node_voltage(nodes[{index}]);\n"
        ));
    }
    for index in branch_indices {
        cached.push_str(&format!(
            "        let bi{index} = ctx.branch_current(branches[{index}]);\n"
        ));
    }
    cached.push_str(&body);
    cached
}

fn collect_indexed_calls(segment: &str, prefix: &str, suffix: &str) -> BTreeSet<usize> {
    let mut indices = BTreeSet::new();
    let mut remaining = segment;
    while let Some(start) = remaining.find(prefix) {
        let after_prefix = &remaining[start + prefix.len()..];
        let digit_len = after_prefix
            .as_bytes()
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        if digit_len > 0 && after_prefix[digit_len..].starts_with(suffix) {
            if let Ok(index) = after_prefix[..digit_len].parse() {
                indices.insert(index);
            }
        }
        let advance = digit_len.max(1);
        if advance >= after_prefix.len() {
            break;
        }
        remaining = &after_prefix[advance..];
    }
    indices
}

#[derive(Debug, Clone, Default)]
struct PotentialBranchSlots {
    equation_slots: HashMap<EquationId, usize>,
    current_slots: HashMap<String, BranchCurrentSlot>,
    branches: Vec<MirBranchRef>,
}

impl PotentialBranchSlots {
    fn len(&self) -> usize {
        self.branches.len()
    }

    fn slot_for(&self, equation: EquationId) -> Option<usize> {
        self.equation_slots.get(&equation).copied()
    }

    fn branches(&self) -> &[MirBranchRef] {
        &self.branches
    }

    fn current_slots(&self) -> &HashMap<String, BranchCurrentSlot> {
        &self.current_slots
    }
}

fn reject_unsupported_model_shape(artifact: &CanonicalIrArtifact) -> Result<(), RustBackendError> {
    if !artifact.hir.arrays.is_empty() {
        return Err(unsupported(artifact, "arrays"));
    }
    for variable in &artifact.hir.variables {
        if variable.is_state {
            return Err(unsupported(
                artifact,
                format!("state variable '{}'", variable.name),
            ));
        }
        if !is_supported_scalar_value_type(variable.value_type) {
            return Err(unsupported(
                artifact,
                format!(
                    "non-numeric scalar variable '{}' with type {:?}",
                    variable.name, variable.value_type
                ),
            ));
        }
    }
    if !artifact.mir.state_slots.is_empty() {
        return Err(unsupported(artifact, "state slots"));
    }
    reject_unsupported_statements(artifact, &artifact.hir.statements)?;

    for equation in &artifact.mir.equations {
        if equation.kind == MirEquationKind::Indirect {
            return Err(unsupported(artifact, "indirect contributions"));
        }
    }
    for expression in &artifact.mir.expressions {
        match &expression.kind {
            HirExprKind::AnalogOperator { op } => match op {
                HirAnalogOperator::Ddt { abstol, .. } => {
                    if abstol.is_some() {
                        return Err(unsupported(artifact, "ddt abstol argument"));
                    }
                }
                HirAnalogOperator::Idt { assert, abstol, .. } => {
                    if assert.is_some() || abstol.is_some() {
                        return Err(unsupported(artifact, "idt assert/abstol argument"));
                    }
                }
                HirAnalogOperator::Ddx { .. } => {}
                HirAnalogOperator::Limexp { .. } => {}
                _ => {
                    return Err(unsupported(
                        artifact,
                        format!("stateful or effectful analog operator {op:?}"),
                    ));
                }
            },
            HirExprKind::Laplace { .. } | HirExprKind::Zi { .. } => {
                return Err(unsupported(
                    artifact,
                    format!(
                        "stateful or effectful expression kind {:?}",
                        expression.kind
                    ),
                ));
            }
            HirExprKind::NoiseSource { .. } => {}
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                if args.len() != 1 {
                    return Err(unsupported(
                        artifact,
                        format!("ddt expects one operand, found {}", args.len()),
                    ));
                }
            }
            HirExprKind::Call { name, args } if is_idt_name(name.as_str()) => {
                if !(1..=2).contains(&args.len()) {
                    return Err(unsupported(
                        artifact,
                        format!("idt expects one or two operands, found {}", args.len()),
                    ));
                }
            }
            HirExprKind::Call { name, args } if is_ddx_name(name.as_str()) => {
                if args.len() != 2 {
                    return Err(unsupported(
                        artifact,
                        format!("ddx expects two operands, found {}", args.len()),
                    ));
                }
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {}
            HirExprKind::Call { name, .. } if is_stateful_or_effectful_call(name.as_str()) => {
                return Err(unsupported(
                    artifact,
                    format!("stateful or effectful analog operator call {name}"),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn is_supported_scalar_value_type(value_type: CanonicalValueType) -> bool {
    matches!(
        value_type,
        CanonicalValueType::Real | CanonicalValueType::Integer | CanonicalValueType::Boolean
    )
}

fn reject_unsupported_statements(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
) -> Result<(), RustBackendError> {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                if assignment.index.is_some() {
                    return Err(unsupported(
                        artifact,
                        format!("indexed assignment to '{}'", assignment.target_name),
                    ));
                }
                if usize::from(assignment.target) >= artifact.hir.variables.len() {
                    return Err(RustBackendError::internal(
                        artifact.metadata.source_package.as_str(),
                        artifact.mir.module_name.as_str(),
                        format!(
                            "assignment target {} is outside HIR variable arena",
                            assignment.target
                        ),
                    ));
                }
            }
            HirStatement::Loop(loop_statement) => {
                reject_unsupported_statements(artifact, &loop_statement.body)?;
            }
        }
    }
    Ok(())
}

fn collect_ddt_slots(artifact: &CanonicalIrArtifact) -> Result<DdtSlots, RustBackendError> {
    let mut collector = DdtSlotCollector {
        artifact,
        slots: HashMap::new(),
        idt_slots: HashMap::new(),
        visited: HashSet::new(),
    };

    for statement in &artifact.hir.statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                collector.collect(assignment.expr.id)?;
                if let Some(index) = &assignment.index {
                    collector.collect(index.id)?;
                }
            }
            HirStatement::Loop(loop_statement) => {
                collector.collect(loop_statement.condition.id)?;
                collector.collect_statements(&loop_statement.body)?;
            }
        }
    }
    for equation in &artifact.mir.equations {
        collector.collect(equation.expression.id)?;
    }

    Ok(DdtSlots::with_idt_slots(
        collector.slots,
        collector.idt_slots,
    ))
}

fn collect_potential_branch_slots(
    artifact: &CanonicalIrArtifact,
) -> Result<PotentialBranchSlots, RustBackendError> {
    let mut slots = PotentialBranchSlots::default();
    for unknown in &artifact.mir.branch_unknowns {
        let slot = usize::from(unknown.id);
        let equation = artifact
            .mir
            .equations
            .get(usize::from(unknown.equation))
            .ok_or_else(|| {
                RustBackendError::internal(
                    artifact.metadata.source_package.as_str(),
                    artifact.mir.module_name.as_str(),
                    format!(
                        "branch unknown {} references missing equation {}",
                        unknown.id, unknown.equation
                    ),
                )
            })?;
        slots.equation_slots.insert(unknown.equation, slot);
        if let Some(name) = unknown.declared_name.as_deref() {
            slots
                .current_slots
                .entry(name.to_string())
                .or_insert(BranchCurrentSlot::forward(slot));
        }
        let pos = unknown.pos_node.map(usize::from);
        let neg = unknown.neg_node.map(usize::from);
        slots
            .current_slots
            .entry(branch_pair_key(pos, neg))
            .or_insert(BranchCurrentSlot::forward(slot));
        slots
            .current_slots
            .entry(branch_pair_key(neg, pos))
            .or_insert(BranchCurrentSlot::reverse(slot));
        slots.branches.push(MirBranchRef {
            label: equation.branch.label.clone(),
            declared_name: unknown.declared_name.clone(),
            pos_node: unknown.pos_node,
            neg_node: unknown.neg_node,
        });
    }

    Ok(slots)
}

#[derive(Debug, Clone, Default)]
struct TransientLiveness {
    values: HashSet<String>,
    derivatives: HashSet<String>,
}

impl TransientLiveness {
    fn is_value_live(&self, name: &str) -> bool {
        self.values.contains(name)
    }

    fn is_derivative_live(&self, name: &str) -> bool {
        self.derivatives.contains(name)
    }

    fn loop_has_live_value_assignment(
        &self,
        loop_statement: &crate::canonical_ir::HirLoop,
    ) -> bool {
        loop_statement
            .body
            .iter()
            .any(|statement| self.statement_has_live_value_assignment(statement))
    }

    fn statement_has_live_value_assignment(&self, statement: &HirStatement) -> bool {
        match statement {
            HirStatement::Assignment(assignment) => {
                self.is_value_live(assignment.target_name.as_str())
            }
            HirStatement::Loop(loop_statement) => {
                self.loop_has_live_value_assignment(loop_statement)
            }
        }
    }
}

fn collect_transient_liveness(
    artifact: &CanonicalIrArtifact,
) -> Result<TransientLiveness, RustBackendError> {
    let mut values = HashSet::new();
    let mut derivatives = HashSet::new();
    for equation in &artifact.mir.equations {
        collect_expression_identifiers(
            artifact,
            equation.expression.id,
            &mut values,
            &mut HashSet::new(),
        )?;
        collect_expression_identifiers(
            artifact,
            equation.expression.id,
            &mut derivatives,
            &mut HashSet::new(),
        )?;
    }

    loop {
        let value_changed =
            collect_live_statement_dependencies(artifact, &artifact.hir.statements, &mut values)?;
        let derivative_changed = collect_live_statement_dependencies(
            artifact,
            &artifact.hir.statements,
            &mut derivatives,
        )?;
        if !value_changed && !derivative_changed {
            break;
        }
    }

    Ok(TransientLiveness {
        values,
        derivatives,
    })
}

#[derive(Debug, Clone, Default)]
struct ReactiveLiveness {
    variables: HashSet<String>,
    equations: HashSet<EquationId>,
}

impl ReactiveLiveness {
    fn is_variable_live(&self, name: &str) -> bool {
        self.variables.contains(name)
    }

    fn is_equation_reactive(&self, equation: EquationId) -> bool {
        self.equations.contains(&equation)
    }

    fn loop_has_live_assignment(&self, loop_statement: &crate::canonical_ir::HirLoop) -> bool {
        loop_statement
            .body
            .iter()
            .any(|statement| self.statement_has_live_assignment(statement))
    }

    fn statement_has_live_assignment(&self, statement: &HirStatement) -> bool {
        match statement {
            HirStatement::Assignment(assignment) => {
                self.is_variable_live(assignment.target_name.as_str())
            }
            HirStatement::Loop(loop_statement) => self.loop_has_live_assignment(loop_statement),
        }
    }
}

fn collect_reactive_liveness(
    artifact: &CanonicalIrArtifact,
) -> Result<ReactiveLiveness, RustBackendError> {
    let dynamic_variables = collect_dynamic_variables(artifact)?;
    let mut live_variables = HashSet::new();
    let mut reactive_equations = HashSet::new();

    for equation in &artifact.mir.equations {
        if expr_depends_on_ddt_or_dynamic(
            artifact,
            equation.expression.id,
            &dynamic_variables,
            &mut HashSet::new(),
        )? {
            reactive_equations.insert(equation.id);
            collect_expression_identifiers(
                artifact,
                equation.expression.id,
                &mut live_variables,
                &mut HashSet::new(),
            )?;
        }
    }

    loop {
        if !collect_live_statement_dependencies(
            artifact,
            &artifact.hir.statements,
            &mut live_variables,
        )? {
            break;
        }
    }

    Ok(ReactiveLiveness {
        variables: live_variables,
        equations: reactive_equations,
    })
}

fn collect_dynamic_variables(
    artifact: &CanonicalIrArtifact,
) -> Result<HashSet<String>, RustBackendError> {
    let mut dynamic_variables = HashSet::new();
    loop {
        let mut next = dynamic_variables.clone();
        let mut changed = false;
        collect_dynamic_statement_targets(
            artifact,
            &artifact.hir.statements,
            &dynamic_variables,
            &mut changed,
            &mut next,
        )?;
        if !changed {
            break;
        }
        dynamic_variables = next;
    }
    Ok(dynamic_variables)
}

fn collect_dynamic_statement_targets(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
    current: &HashSet<String>,
    changed: &mut bool,
    next: &mut HashSet<String>,
) -> Result<(), RustBackendError> {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                if expr_depends_on_ddt_or_dynamic(
                    artifact,
                    assignment.expr.id,
                    current,
                    &mut HashSet::new(),
                )? && next.insert(assignment.target_name.to_string())
                {
                    *changed = true;
                }
            }
            HirStatement::Loop(loop_statement) => {
                collect_dynamic_statement_targets(
                    artifact,
                    &loop_statement.body,
                    current,
                    changed,
                    next,
                )?;
            }
        }
    }
    Ok(())
}

fn collect_live_statement_dependencies(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
    live_variables: &mut HashSet<String>,
) -> Result<bool, RustBackendError> {
    let before = live_variables.len();
    for statement in statements.iter().rev() {
        match statement {
            HirStatement::Assignment(assignment) => {
                if live_variables.contains(assignment.target_name.as_str()) {
                    collect_expression_identifiers(
                        artifact,
                        assignment.expr.id,
                        live_variables,
                        &mut HashSet::new(),
                    )?;
                    if let Some(index) = &assignment.index {
                        collect_expression_identifiers(
                            artifact,
                            index.id,
                            live_variables,
                            &mut HashSet::new(),
                        )?;
                    }
                }
            }
            HirStatement::Loop(loop_statement) => {
                collect_live_statement_dependencies(
                    artifact,
                    &loop_statement.body,
                    live_variables,
                )?;
                if loop_contains_live_assignment(loop_statement, live_variables) {
                    collect_expression_identifiers(
                        artifact,
                        loop_statement.condition.id,
                        live_variables,
                        &mut HashSet::new(),
                    )?;
                }
            }
        }
    }
    Ok(live_variables.len() != before)
}

fn loop_contains_live_assignment(
    loop_statement: &crate::canonical_ir::HirLoop,
    live_variables: &HashSet<String>,
) -> bool {
    loop_statement
        .body
        .iter()
        .any(|statement| statement_contains_live_assignment(statement, live_variables))
}

fn statement_contains_live_assignment(
    statement: &HirStatement,
    live_variables: &HashSet<String>,
) -> bool {
    match statement {
        HirStatement::Assignment(assignment) => {
            live_variables.contains(assignment.target_name.as_str())
        }
        HirStatement::Loop(loop_statement) => {
            loop_contains_live_assignment(loop_statement, live_variables)
        }
    }
}

fn expr_depends_on_ddt_or_dynamic(
    artifact: &CanonicalIrArtifact,
    id: ExprId,
    dynamic_variables: &HashSet<String>,
    visited: &mut HashSet<ExprId>,
) -> Result<bool, RustBackendError> {
    if !visited.insert(id) {
        return Ok(false);
    }
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(id))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("expression {id} is outside MIR arena"),
            )
        })?;

    match &expression.kind {
        HirExprKind::Identifier { name } => Ok(dynamic_variables.contains(name.as_str())),
        HirExprKind::Call { name, .. }
            if is_ddt_name(name.as_str()) || is_idt_name(name.as_str()) =>
        {
            Ok(true)
        }
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Ddt { .. },
        } => Ok(true),
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Idt { .. },
        } => Ok(true),
        other => {
            for child in expression_children(other) {
                if expr_depends_on_ddt_or_dynamic(artifact, child, dynamic_variables, visited)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    }
}

fn collect_expression_identifiers(
    artifact: &CanonicalIrArtifact,
    id: ExprId,
    identifiers: &mut HashSet<String>,
    visited: &mut HashSet<ExprId>,
) -> Result<(), RustBackendError> {
    if !visited.insert(id) {
        return Ok(());
    }
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(id))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("expression {id} is outside MIR arena"),
            )
        })?;

    if let HirExprKind::Identifier { name } = &expression.kind {
        identifiers.insert(name.to_string());
    }
    for child in expression_children(&expression.kind) {
        collect_expression_identifiers(artifact, child, identifiers, visited)?;
    }
    Ok(())
}

fn collect_named_current_accesses(
    artifact: &CanonicalIrArtifact,
    id: ExprId,
    currents: &mut HashSet<String>,
    visited: &mut HashSet<ExprId>,
) -> Result<(), RustBackendError> {
    if !visited.insert(id) {
        return Ok(());
    }
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(id))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("expression {id} is outside MIR arena"),
            )
        })?;

    if let HirExprKind::NamedBranchAccess { access, name } = &expression.kind
        && access == "I"
    {
        currents.insert(name.to_string());
    }
    for child in expression_children(&expression.kind) {
        collect_named_current_accesses(artifact, child, currents, visited)?;
    }
    Ok(())
}

fn equation_inline_plan(artifact: &CanonicalIrArtifact) -> Result<Vec<bool>, RustBackendError> {
    let mut reads_by_index = Vec::with_capacity(artifact.mir.equations.len());
    for equation in &artifact.mir.equations {
        let mut reads = HashSet::new();
        collect_named_current_accesses(
            artifact,
            equation.expression.id,
            &mut reads,
            &mut HashSet::new(),
        )?;
        reads_by_index.push(reads);
    }

    let mut future_reads = vec![HashSet::<String>::new(); artifact.mir.equations.len() + 1];
    for index in (0..artifact.mir.equations.len()).rev() {
        future_reads[index] = future_reads[index + 1].clone();
        future_reads[index].extend(reads_by_index[index].iter().cloned());
    }

    let mut inline = vec![false; artifact.mir.equations.len()];
    for (index, equation) in artifact.mir.equations.iter().enumerate() {
        if !reads_by_index[index].is_empty() {
            inline[index] = true;
            continue;
        }
        if equation.kind == MirEquationKind::Current
            && let Some(branch_name) = declared_contribution_branch_name(artifact, equation)
            && future_reads[index + 1].contains(branch_name.as_str())
        {
            inline[index] = true;
        }
    }
    Ok(inline)
}

fn expression_node_count(
    artifact: &CanonicalIrArtifact,
    id: ExprId,
) -> Result<usize, RustBackendError> {
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(id))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("expression {id} is outside MIR arena"),
            )
        })?;

    let mut count = 1;
    for child in expression_children(&expression.kind) {
        count += expression_node_count(artifact, child)?;
    }
    Ok(count)
}

fn expression_children(kind: &HirExprKind) -> Vec<ExprId> {
    let mut children = Vec::new();
    match kind {
        HirExprKind::Number { .. }
        | HirExprKind::StringLiteral { .. }
        | HirExprKind::Identifier { .. }
        | HirExprKind::BranchAccess { .. }
        | HirExprKind::NamedBranchAccess { .. } => {}
        HirExprKind::SystemFunction { args, .. } | HirExprKind::Call { args, .. } => {
            children.extend(args.iter().copied());
        }
        HirExprKind::Binary { left, right, .. } => {
            children.push(*left);
            children.push(*right);
        }
        HirExprKind::Unary { operand, .. } => {
            children.push(*operand);
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => {
            children.push(*condition);
            children.push(*then_expr);
            children.push(*else_expr);
        }
        HirExprKind::ArrayAccess { index, .. } => {
            children.push(*index);
        }
        HirExprKind::ArrayLiteral { elements } => {
            children.extend(elements.iter().copied());
        }
        HirExprKind::AnalogOperator { op } => push_analog_operator_children(op, &mut children),
        HirExprKind::Laplace { expr, kind } => {
            children.push(*expr);
            push_laplace_children(kind, &mut children);
        }
        HirExprKind::Zi { expr, kind } => {
            children.push(*expr);
            push_zi_children(kind, &mut children);
        }
        HirExprKind::NoiseSource { operands, .. } => {
            children.extend(operands.iter().copied());
        }
    }
    children
}

fn push_analog_operator_children(op: &HirAnalogOperator, children: &mut Vec<ExprId>) {
    match op {
        HirAnalogOperator::Ddt { expr, abstol } => {
            children.push(*expr);
            children.extend(abstol.iter().copied());
        }
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => {
            children.push(*expr);
            children.extend([*ic, *assert, *abstol].into_iter().flatten());
        }
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => {
            children.push(*expr);
            children.extend([*ic, *modulus, *offset, *abstol].into_iter().flatten());
        }
        HirAnalogOperator::Ddx { expr, probe } => {
            children.push(*expr);
            children.push(*probe);
        }
        HirAnalogOperator::Limexp { expr } => {
            children.push(*expr);
        }
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => {
            children.push(*expr);
            children.push(*delay);
            children.extend(max_delay.iter().copied());
        }
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
        } => {
            children.push(*expr);
            children.extend([*delay, *rise, *fall, *tolerance].into_iter().flatten());
        }
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => {
            children.push(*expr);
            children.extend([*max_rise, *max_fall].into_iter().flatten());
        }
        HirAnalogOperator::LastCrossing { expr, .. } => {
            children.push(*expr);
        }
    }
}

fn push_laplace_children(kind: &crate::canonical_ir::HirLaplaceKind, children: &mut Vec<ExprId>) {
    match kind {
        crate::canonical_ir::HirLaplaceKind::ZeroPole { zeros, poles }
        | crate::canonical_ir::HirLaplaceKind::NumeratorPole {
            numerator: zeros,
            poles,
        } => {
            children.extend(zeros.iter().copied());
            children.extend(poles.iter().copied());
        }
        crate::canonical_ir::HirLaplaceKind::ZeroDenominator { zeros, denominator }
        | crate::canonical_ir::HirLaplaceKind::NumeratorDenominator {
            numerator: zeros,
            denominator,
        } => {
            children.extend(zeros.iter().copied());
            children.extend(denominator.iter().copied());
        }
    }
}

fn push_zi_children(kind: &crate::canonical_ir::HirZiKind, children: &mut Vec<ExprId>) {
    match kind {
        crate::canonical_ir::HirZiKind::ZeroPole { zeros, poles }
        | crate::canonical_ir::HirZiKind::NumeratorPole {
            numerator: zeros,
            poles,
        } => {
            children.extend(zeros.iter().copied());
            children.extend(poles.iter().copied());
        }
        crate::canonical_ir::HirZiKind::ZeroDenominator { zeros, denominator }
        | crate::canonical_ir::HirZiKind::NumeratorDenominator {
            numerator: zeros,
            denominator,
        } => {
            children.extend(zeros.iter().copied());
            children.extend(denominator.iter().copied());
        }
    }
}

struct DdtSlotCollector<'a> {
    artifact: &'a CanonicalIrArtifact,
    slots: HashMap<ExprId, usize>,
    idt_slots: HashMap<ExprId, usize>,
    visited: HashSet<ExprId>,
}

impl DdtSlotCollector<'_> {
    fn collect_statements(&mut self, statements: &[HirStatement]) -> Result<(), RustBackendError> {
        for statement in statements {
            match statement {
                HirStatement::Assignment(assignment) => {
                    self.collect(assignment.expr.id)?;
                    if let Some(index) = &assignment.index {
                        self.collect(index.id)?;
                    }
                }
                HirStatement::Loop(loop_statement) => {
                    self.collect(loop_statement.condition.id)?;
                    self.collect_statements(&loop_statement.body)?;
                }
            }
        }
        Ok(())
    }

    fn collect(&mut self, id: ExprId) -> Result<(), RustBackendError> {
        if !self.visited.insert(id) {
            return Ok(());
        }

        let expression = self
            .artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| {
                RustBackendError::internal(
                    self.artifact.metadata.source_package.as_str(),
                    self.artifact.mir.module_name.as_str(),
                    format!("expression {id} is outside MIR arena"),
                )
            })?;

        match &expression.kind {
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::Identifier { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {}
            HirExprKind::SystemFunction { args, .. } | HirExprKind::Call { args, .. } => {
                if let HirExprKind::Call { name, args } = &expression.kind
                    && is_ddt_name(name.as_str())
                {
                    if args.len() != 1 {
                        return Err(unsupported(
                            self.artifact,
                            format!("ddt expects one operand, found {}", args.len()),
                        ));
                    }
                    let next_slot = self.slots.len();
                    self.slots.entry(id).or_insert(next_slot);
                }
                if let HirExprKind::Call { name, args } = &expression.kind
                    && is_idt_name(name.as_str())
                {
                    if !(1..=2).contains(&args.len()) {
                        return Err(unsupported(
                            self.artifact,
                            format!("idt expects one or two operands, found {}", args.len()),
                        ));
                    }
                    let next_slot = self.idt_slots.len();
                    self.idt_slots.entry(id).or_insert(next_slot);
                }
                for arg in args {
                    self.collect(*arg)?;
                }
            }
            HirExprKind::Binary { left, right, .. } => {
                self.collect(*left)?;
                self.collect(*right)?;
            }
            HirExprKind::Unary { operand, .. } => {
                self.collect(*operand)?;
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                self.collect(*condition)?;
                self.collect(*then_expr)?;
                self.collect(*else_expr)?;
            }
            HirExprKind::ArrayAccess { index, .. } => {
                self.collect(*index)?;
            }
            HirExprKind::ArrayLiteral { elements } => {
                for element in elements {
                    self.collect(*element)?;
                }
            }
            HirExprKind::AnalogOperator { op } => {
                self.collect_analog_operator(id, op)?;
            }
            HirExprKind::Laplace { expr, kind } => {
                self.collect(*expr)?;
                match kind {
                    crate::canonical_ir::HirLaplaceKind::ZeroPole { zeros, poles }
                    | crate::canonical_ir::HirLaplaceKind::ZeroDenominator {
                        zeros,
                        denominator: poles,
                    }
                    | crate::canonical_ir::HirLaplaceKind::NumeratorPole {
                        numerator: zeros,
                        poles,
                    }
                    | crate::canonical_ir::HirLaplaceKind::NumeratorDenominator {
                        numerator: zeros,
                        denominator: poles,
                    } => {
                        for child in zeros.iter().chain(poles) {
                            self.collect(*child)?;
                        }
                    }
                }
            }
            HirExprKind::Zi { expr, kind } => {
                self.collect(*expr)?;
                match kind {
                    crate::canonical_ir::HirZiKind::ZeroPole { zeros, poles }
                    | crate::canonical_ir::HirZiKind::ZeroDenominator {
                        zeros,
                        denominator: poles,
                    }
                    | crate::canonical_ir::HirZiKind::NumeratorPole {
                        numerator: zeros,
                        poles,
                    }
                    | crate::canonical_ir::HirZiKind::NumeratorDenominator {
                        numerator: zeros,
                        denominator: poles,
                    } => {
                        for child in zeros.iter().chain(poles) {
                            self.collect(*child)?;
                        }
                    }
                }
            }
            HirExprKind::NoiseSource { operands, .. } => {
                for operand in operands {
                    self.collect(*operand)?;
                }
            }
        }

        Ok(())
    }

    fn collect_analog_operator(
        &mut self,
        id: ExprId,
        op: &HirAnalogOperator,
    ) -> Result<(), RustBackendError> {
        match op {
            HirAnalogOperator::Ddt { expr, abstol } => {
                if abstol.is_some() {
                    return Err(unsupported(self.artifact, "ddt abstol argument"));
                }
                let next_slot = self.slots.len();
                self.slots.entry(id).or_insert(next_slot);
                self.collect(*expr)?;
            }
            HirAnalogOperator::Idt {
                expr,
                ic,
                assert,
                abstol,
            } => {
                if assert.is_some() || abstol.is_some() {
                    return Err(unsupported(self.artifact, "idt assert/abstol argument"));
                }
                let next_slot = self.idt_slots.len();
                self.idt_slots.entry(id).or_insert(next_slot);
                self.collect(*expr)?;
                for child in [*ic].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
            } => {
                self.collect(*expr)?;
                for child in [*ic, *modulus, *offset, *abstol].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::Ddx { expr, probe } => {
                self.collect(*expr)?;
                self.collect(*probe)?;
            }
            HirAnalogOperator::Limexp { expr } => {
                self.collect(*expr)?;
            }
            HirAnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
            } => {
                self.collect(*expr)?;
                self.collect(*delay)?;
                if let Some(max_delay) = max_delay {
                    self.collect(*max_delay)?;
                }
            }
            HirAnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
            } => {
                self.collect(*expr)?;
                for child in [*delay, *rise, *fall, *tolerance].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
            } => {
                self.collect(*expr)?;
                for child in [*max_rise, *max_fall].into_iter().flatten() {
                    self.collect(child)?;
                }
            }
            HirAnalogOperator::LastCrossing { expr, .. } => {
                self.collect(*expr)?;
            }
        }
        Ok(())
    }
}

fn is_stateful_or_effectful_call(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "idt"
            | "idtmod"
            | "absdelay"
            | "transition"
            | "slew"
            | "last_crossing"
            | "laplace_zp"
            | "laplace_zd"
            | "laplace_np"
            | "laplace_nd"
            | "zi_zp"
            | "zi_zd"
            | "zi_np"
            | "zi_nd"
            | "noise_table"
            | "noise_table_log"
    )
}

fn is_ddt_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("ddt")
}

fn is_idt_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("idt")
}

fn is_ddx_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("ddx")
}

fn is_noise_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "white_noise" | "$white_noise" | "flicker_noise" | "$flicker_noise"
    )
}

fn generate_mod_file() -> String {
    [
        "pub mod state;",
        "mod stamp;",
        "",
        "pub use state::{Instance, Parameters};",
        "",
    ]
    .join("\n")
}

fn generate_state_file(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
    parameter_fields: &HashMap<String, String>,
    ddt_state_count: usize,
    idt_state_count: usize,
    branch_count: usize,
) -> Result<String, RustBackendError> {
    let mut out = String::new();
    out.push_str("#![allow(dead_code, unused_parens, unused_variables)]\n\n");
    out.push_str(&format!(
        "use {}::support::{{ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch}};\n\n",
        options.runtime_path
    ));
    out.push_str("pub struct Parameters {\n");
    for parameter in &artifact.mir.parameters {
        let field = &parameter_fields[parameter.name.as_str()];
        out.push_str(&format!("    pub {field}: f64,\n"));
    }
    out.push_str("}\n\n");
    out.push_str("impl Copy for Parameters {}\n\n");
    out.push_str("impl Clone for Parameters {\n");
    out.push_str("    #[inline]\n");
    out.push_str("    fn clone(&self) -> Self { *self }\n");
    out.push_str("}\n\n");

    out.push_str("impl Parameters {\n");
    out.push_str("    fn new_box() -> Box<Self> {\n");
    if artifact.mir.parameters.is_empty() {
        out.push_str("        Box::new(Self {\n");
        out.push_str("        })\n");
    } else {
        out.push_str("        // SAFETY: every generated Parameters field is f64; all-zero bytes are a valid 0.0 value for f64.\n");
        out.push_str("        let mut boxed = Box::<Self>::new_uninit();\n");
        out.push_str("        unsafe {\n");
        out.push_str("            let ptr = boxed.as_mut_ptr();\n");
        out.push_str("            std::ptr::write_bytes(ptr, 0, 1);\n");
        out.push_str("            let params = &mut *ptr;\n");
        for parameter in &artifact.mir.parameters {
            let field = &parameter_fields[parameter.name.as_str()];
            let default = parameter_default_rust_expr(artifact, parameter, parameter_fields)?;
            out.push_str(&format!("            params.{field} = {default};\n"));
            if parameter_default_requires_runtime_validation(parameter) {
                let validation = parameter_validation_call(
                    parameter.name.as_str(),
                    &format!("params.{field}"),
                    parameter.range.as_ref(),
                )?;
                out.push_str(&format!(
                    "            {validation}.expect(\"generated Verilog-A parameter default must satisfy declared range\");\n"
                ));
            }
        }
        out.push_str("            boxed.assume_init()\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("impl Default for Parameters {\n");
    out.push_str("    fn default() -> Self {\n");
    out.push_str("        *Self::new_box()\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    if !artifact.mir.parameters.is_empty() {
        out.push_str(&generate_shared_parameter_validator());
        out.push('\n');
    }

    out.push_str("fn boxed_zero_f64_array<const N: usize>() -> Box<[f64; N]> {\n");
    out.push_str("    let mut boxed = Box::<[f64; N]>::new_uninit();\n");
    out.push_str("    unsafe {\n");
    out.push_str("        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);\n");
    out.push_str("        boxed.assume_init()\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("fn boxed_zero_bool_array<const N: usize>() -> Box<[bool; N]> {\n");
    out.push_str("    let mut boxed = Box::<[bool; N]>::new_uninit();\n");
    out.push_str("    unsafe {\n");
    out.push_str("        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);\n");
    out.push_str("        boxed.assume_init()\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    let node_count = artifact.mir.nodes.len();
    let terminal_count = artifact
        .mir
        .nodes
        .iter()
        .filter(|node| node.is_external)
        .count();
    let internal_node_names: Vec<_> = artifact
        .mir
        .nodes
        .iter()
        .filter(|node| !node.is_external)
        .map(|node| node.name.as_str())
        .collect();
    let internal_node_count = internal_node_names.len();
    let parameter_count = artifact.mir.parameters.len();
    let variable_count = artifact.hir.variables.len();
    out.push_str("pub struct Instance {\n");
    out.push_str(&format!("    pub nodes: [usize; {node_count}],\n"));
    out.push_str(&format!("    pub branches: [usize; {branch_count}],\n"));
    out.push_str("    pub params: Box<Parameters>,\n");
    out.push_str(&format!(
        "    pub(crate) param_given: Box<[bool; {parameter_count}]>,\n"
    ));
    out.push_str("    pub(crate) multiplicity: f64,\n");
    out.push_str(&format!(
        "    pub(crate) ddt_state_current: Box<[f64; {ddt_state_count}]>,\n"
    ));
    out.push_str(&format!(
        "    pub(crate) ddt_state_previous: Box<[f64; {ddt_state_count}]>,\n"
    ));
    out.push_str(&format!(
        "    pub(crate) ddt_state_initialized: Box<[bool; {ddt_state_count}]>,\n"
    ));
    out.push_str(&format!(
        "    pub(crate) idt_state_current: Box<[f64; {idt_state_count}]>,\n"
    ));
    out.push_str(&format!(
        "    pub(crate) idt_state_previous: Box<[f64; {idt_state_count}]>,\n"
    ));
    out.push_str(&format!(
        "    pub(crate) idt_state_initialized: Box<[bool; {idt_state_count}]>,\n"
    ));
    out.push_str("    pub(crate) time: f64,\n");
    out.push_str("    pub(crate) timestep: f64,\n");
    out.push_str(&format!(
        "    pub(crate) scratch: Option<Box<GenericScratch<{variable_count}, {node_count}, {branch_count}>>>,\n"
    ));
    out.push_str(&format!(
        "    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<{variable_count}, {node_count}, {branch_count}>>>,\n"
    ));
    out.push_str("}\n\n");
    out.push_str("impl Clone for Instance {\n");
    out.push_str("    #[inline]\n");
    out.push_str("    fn clone(&self) -> Self {\n");
    out.push_str("        Self {\n");
    out.push_str("            nodes: self.nodes,\n");
    out.push_str("            branches: self.branches,\n");
    out.push_str("            params: self.params.clone(),\n");
    out.push_str("            param_given: self.param_given.clone(),\n");
    out.push_str("            multiplicity: self.multiplicity,\n");
    out.push_str("            ddt_state_current: self.ddt_state_current.clone(),\n");
    out.push_str("            ddt_state_previous: self.ddt_state_previous.clone(),\n");
    out.push_str("            ddt_state_initialized: self.ddt_state_initialized.clone(),\n");
    out.push_str("            idt_state_current: self.idt_state_current.clone(),\n");
    out.push_str("            idt_state_previous: self.idt_state_previous.clone(),\n");
    out.push_str("            idt_state_initialized: self.idt_state_initialized.clone(),\n");
    out.push_str("            time: self.time,\n");
    out.push_str("            timestep: self.timestep,\n");
    out.push_str("            scratch: None,\n");
    out.push_str("            reactive_scratch: None,\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("impl Instance {\n");
    out.push_str(&format!(
        "    pub const TERMINAL_COUNT: usize = {terminal_count};\n"
    ));
    out.push_str(&format!(
        "    pub const INTERNAL_NODE_COUNT: usize = {internal_node_count};\n"
    ));
    out.push_str(&format!(
        "    pub const NODE_COUNT: usize = {node_count};\n"
    ));
    out.push_str(&format!(
        "    pub const INTERNAL_NODE_NAMES: [&str; {internal_node_count}] = [{}];\n\n",
        internal_node_names
            .iter()
            .map(|name| format!("{name:?}"))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    out.push_str(&format!(
        "    pub const BRANCH_COUNT: usize = {branch_count};\n"
    ));
    out.push_str(&format!(
        "    pub const PARAMETER_COUNT: usize = {parameter_count};\n"
    ));
    out.push_str(&format!(
        "    pub const VARIABLE_COUNT: usize = {};\n",
        variable_count
    ));
    out.push_str(&format!(
        "    pub const DDT_STATE_COUNT: usize = {ddt_state_count};\n"
    ));
    out.push_str(&format!(
        "    pub const IDT_STATE_COUNT: usize = {idt_state_count};\n"
    ));
    out.push_str("    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;\n");
    out.push_str("    pub const DDT_EPSILON: f64 = 1.0e-20;\n\n");
    out.push_str("    pub fn new(nodes: &[usize]) -> Self {\n");
    out.push_str("        assert_eq!(nodes.len(), Self::NODE_COUNT, \"generated Verilog-A node count mismatch\");\n");
    out.push_str("        let mut mapped = [0usize; Self::NODE_COUNT];\n");
    out.push_str("        mapped.copy_from_slice(nodes);\n");
    out.push_str("        Self {\n");
    out.push_str("            nodes: mapped,\n");
    out.push_str("            branches: [0usize; Self::BRANCH_COUNT],\n");
    out.push_str("            params: Parameters::new_box(),\n");
    out.push_str(
        "            param_given: boxed_zero_bool_array::<{ Self::PARAMETER_COUNT }>(),\n",
    );
    out.push_str("            multiplicity: 1.0,\n");
    out.push_str(
        "            ddt_state_current: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),\n",
    );
    out.push_str(
        "            ddt_state_previous: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),\n",
    );
    out.push_str("            ddt_state_initialized: boxed_zero_bool_array::<{ Self::DDT_STATE_COUNT }>(),\n");
    out.push_str(
        "            idt_state_current: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),\n",
    );
    out.push_str(
        "            idt_state_previous: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),\n",
    );
    out.push_str("            idt_state_initialized: boxed_zero_bool_array::<{ Self::IDT_STATE_COUNT }>(),\n");
    out.push_str("            time: 0.0,\n");
    out.push_str("            timestep: 0.0,\n");
    out.push_str("            scratch: Some(GenericScratch::new_box()),\n");
    out.push_str("            reactive_scratch: Some(GenericReactiveScratch::new_box()),\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn restore_from_snapshot(&mut self, snapshot: Self) {\n");
    out.push_str("        let scratch = self.scratch.take();\n");
    out.push_str("        let reactive_scratch = self.reactive_scratch.take();\n");
    out.push_str("        let Self {\n");
    out.push_str("            nodes,\n");
    out.push_str("            branches,\n");
    out.push_str("            params,\n");
    out.push_str("            param_given,\n");
    out.push_str("            multiplicity,\n");
    out.push_str("            ddt_state_current,\n");
    out.push_str("            ddt_state_previous,\n");
    out.push_str("            ddt_state_initialized,\n");
    out.push_str("            idt_state_current,\n");
    out.push_str("            idt_state_previous,\n");
    out.push_str("            idt_state_initialized,\n");
    out.push_str("            time,\n");
    out.push_str("            timestep,\n");
    out.push_str("            scratch: _,\n");
    out.push_str("            reactive_scratch: _,\n");
    out.push_str("        } = snapshot;\n");
    out.push_str("        *self = Self {\n");
    out.push_str("            nodes,\n");
    out.push_str("            branches,\n");
    out.push_str("            params,\n");
    out.push_str("            param_given,\n");
    out.push_str("            multiplicity,\n");
    out.push_str("            ddt_state_current,\n");
    out.push_str("            ddt_state_previous,\n");
    out.push_str("            ddt_state_initialized,\n");
    out.push_str("            idt_state_current,\n");
    out.push_str("            idt_state_previous,\n");
    out.push_str("            idt_state_initialized,\n");
    out.push_str("            time,\n");
    out.push_str("            timestep,\n");
    out.push_str("            scratch,\n");
    out.push_str("            reactive_scratch,\n");
    out.push_str("        };\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn set_branch_indices(&mut self, branches: &[usize]) {\n");
    out.push_str("        assert_eq!(branches.len(), Self::BRANCH_COUNT, \"generated Verilog-A branch count mismatch\");\n");
    out.push_str("        self.branches.copy_from_slice(branches);\n");
    out.push_str("    }\n\n");
    out.push_str(
        "    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {\n",
    );
    out.push_str("        match name.to_ascii_lowercase().as_str() {\n");
    for parameter in &artifact.mir.parameters {
        let parameter_index = usize::from(parameter.id);
        let field = &parameter_fields[parameter.name.as_str()];
        let validation =
            parameter_validation_call(parameter.name.as_str(), "value", parameter.range.as_ref())?;
        out.push_str(&format!(
            "            \"{}\" => {{ {validation}?; self.params.{field} = value; self.mark_param_given({parameter_index}); Ok(()) }}\n",
            parameter.name.to_ascii_lowercase()
        ));
        for alias in &parameter.aliases {
            out.push_str(&format!(
                "            \"{}\" => {{ {validation}?; self.params.{field} = value; self.mark_param_given({parameter_index}); Ok(()) }}\n",
                alias.to_ascii_lowercase()
            ));
        }
    }
    out.push_str(&format!(
        "            _ => Err(format!(\"unknown parameter '{{}}' for generated Verilog-A model '{}'\", name)),\n",
        artifact.mir.module_name
    ));
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push('\n');
    out.push_str("    #[inline]\n");
    out.push_str("    fn mark_param_given(&mut self, index: usize) {\n");
    out.push_str("        debug_assert!(index < Self::PARAMETER_COUNT, \"generated parameter index out of range\");\n");
    out.push_str("        self.param_given[index] = true;\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn set_multiplicity(&mut self, multiplicity: f64) {\n");
    out.push_str("        if multiplicity.is_finite() && multiplicity > 0.0 {\n");
    out.push_str("            self.multiplicity = multiplicity;\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn set_timepoint(&mut self, time: f64, timestep: f64) {\n");
    out.push_str("        self.time = time;\n");
    out.push_str("        self.timestep = timestep;\n");
    out.push_str("    }\n\n");
    out.push_str("    #[inline]\n");
    out.push_str("    pub fn accept_timestep(&mut self) {\n");
    out.push_str("        let mut index = 0usize;\n");
    out.push_str("        while index < Self::DDT_STATE_COUNT {\n");
    out.push_str("            self.ddt_state_previous[index] = self.ddt_state_current[index];\n");
    out.push_str("            self.ddt_state_initialized[index] = true;\n");
    out.push_str("            index += 1;\n");
    out.push_str("        }\n");
    out.push_str("        let mut index = 0usize;\n");
    out.push_str("        while index < Self::IDT_STATE_COUNT {\n");
    out.push_str("            self.idt_state_previous[index] = self.idt_state_current[index];\n");
    out.push_str("            self.idt_state_initialized[index] = true;\n");
    out.push_str("            index += 1;\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    if ddt_state_count > 0 {
        out.push_str("    #[inline]\n");
        out.push_str("    pub(crate) fn eval_ddt(&mut self, slot: usize, value: f64) -> f64 {\n");
        out.push_str("        debug_assert!(slot < Self::DDT_STATE_COUNT, \"generated ddt state slot out of range\");\n");
        out.push_str("        let previous = if self.ddt_state_initialized[slot] {\n");
        out.push_str("            self.ddt_state_previous[slot]\n");
        out.push_str("        } else {\n");
        out.push_str("            value\n");
        out.push_str("        };\n");
        out.push_str("        self.ddt_state_current[slot] = value;\n");
        out.push_str("        if self.timestep.abs() > Self::DDT_EPSILON {\n");
        out.push_str("            (value - previous) / self.timestep\n");
        out.push_str("        } else {\n");
        out.push_str("            self.ddt_state_previous[slot] = value;\n");
        out.push_str("            self.ddt_state_initialized[slot] = true;\n");
        out.push_str("            0.0\n");
        out.push_str("        }\n");
        out.push_str("    }\n\n");
        out.push_str("    #[inline]\n");
        out.push_str("    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {\n");
        out.push_str("        if self.timestep.abs() > Self::DDT_EPSILON {\n");
        out.push_str("            derivative / self.timestep\n");
        out.push_str("        } else {\n");
        out.push_str("            0.0\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
    }
    if idt_state_count > 0 {
        out.push_str("    #[inline]\n");
        out.push_str(
            "    pub(crate) fn eval_idt(&mut self, slot: usize, value: f64, ic: f64) -> f64 {\n",
        );
        out.push_str("        debug_assert!(slot < Self::IDT_STATE_COUNT, \"generated idt state slot out of range\");\n");
        out.push_str("        let previous = if self.idt_state_initialized[slot] {\n");
        out.push_str("            self.idt_state_previous[slot]\n");
        out.push_str("        } else {\n");
        out.push_str("            ic\n");
        out.push_str("        };\n");
        out.push_str("        let current = if self.timestep.abs() > Self::DDT_EPSILON {\n");
        out.push_str("            previous + value * self.timestep\n");
        out.push_str("        } else {\n");
        out.push_str("            ic\n");
        out.push_str("        };\n");
        out.push_str("        self.idt_state_current[slot] = current;\n");
        out.push_str("        if self.timestep.abs() <= Self::DDT_EPSILON {\n");
        out.push_str("            self.idt_state_previous[slot] = current;\n");
        out.push_str("            self.idt_state_initialized[slot] = true;\n");
        out.push_str("        }\n");
        out.push_str("        current\n");
        out.push_str("    }\n\n");
        out.push_str("    #[inline]\n");
        out.push_str("    pub(crate) fn idt_jacobian(&self, derivative: f64) -> f64 {\n");
        out.push_str("        if self.timestep.abs() > Self::DDT_EPSILON {\n");
        out.push_str("            derivative * self.timestep\n");
        out.push_str("        } else {\n");
        out.push_str("            0.0\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
    }
    out.push_str("}\n");
    Ok(out)
}

fn parameter_default_requires_runtime_validation(parameter: &MirParameterSlot) -> bool {
    parameter.default.is_none()
}

fn parameter_default_rust_expr(
    artifact: &CanonicalIrArtifact,
    parameter: &MirParameterSlot,
    parameter_fields: &HashMap<String, String>,
) -> Result<String, RustBackendError> {
    if let Some(default) = parameter.default {
        validate_parameter_value_for_codegen(
            artifact,
            parameter.name.as_str(),
            default,
            parameter.range.as_ref(),
        )?;
        return Ok(format_f64(default));
    }

    if let Some(default_expr) = &parameter.default_expr {
        return lower_parameter_default_expr(artifact, default_expr.id, parameter_fields);
    }

    Err(unsupported(
        artifact,
        format!(
            "parameter '{}' default that does not fold to a constant",
            parameter.name
        ),
    ))
}

fn lower_parameter_default_expr(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    parameter_fields: &HashMap<String, String>,
) -> Result<String, RustBackendError> {
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(expr))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("parameter default expression {expr} is outside MIR arena"),
            )
        })?;

    match &expression.kind {
        HirExprKind::Number { value, .. } => Ok(format_f64(*value)),
        HirExprKind::Identifier { name } => parameter_fields
            .get(name.as_str())
            .map(|field| format!("params.{field}"))
            .ok_or_else(|| {
                unsupported(
                    artifact,
                    format!("parameter default references non-parameter identifier '{name}'"),
                )
            }),
        HirExprKind::Unary { op, operand } => {
            let operand = lower_parameter_default_expr(artifact, *operand, parameter_fields)?;
            match op.as_str() {
                "Neg" => Ok(format!("(-{operand})")),
                "Pos" => Ok(format!("({operand})")),
                "Not" => Ok(format!(
                    "if {} {{ 1.0 }} else {{ 0.0 }}",
                    negate_condition(&format!("({operand} != 0.0)"))
                )),
                _ => Err(unsupported(
                    artifact,
                    format!("parameter default unary operator {op}"),
                )),
            }
        }
        HirExprKind::Binary { op, left, right } => {
            if let Some(operator) = comparison_operator(op.as_str()) {
                let left = lower_parameter_default_expr(artifact, *left, parameter_fields)?;
                let right = lower_parameter_default_expr(artifact, *right, parameter_fields)?;
                return Ok(format!(
                    "if ({left} {operator} {right}) {{ 1.0 }} else {{ 0.0 }}"
                ));
            }
            if op.as_str() == "And" || op.as_str() == "Or" {
                let left_condition =
                    lower_parameter_default_condition(artifact, *left, parameter_fields)?;
                let right_condition =
                    lower_parameter_default_condition(artifact, *right, parameter_fields)?;
                let operator = if op.as_str() == "And" { "&&" } else { "||" };
                return Ok(format!(
                    "if ({left_condition} {operator} {right_condition}) {{ 1.0 }} else {{ 0.0 }}"
                ));
            }
            let left = lower_parameter_default_expr(artifact, *left, parameter_fields)?;
            let right = lower_parameter_default_expr(artifact, *right, parameter_fields)?;
            let operator = match op.as_str() {
                "Add" => "+",
                "Sub" => "-",
                "Mul" => "*",
                "Div" => "/",
                "Mod" => "%",
                _ => {
                    return Err(unsupported(
                        artifact,
                        format!("parameter default binary operator {op}"),
                    ));
                }
            };
            Ok(format!("({left} {operator} {right})"))
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => {
            let condition =
                lower_parameter_default_condition(artifact, *condition, parameter_fields)?;
            let then_expr = lower_parameter_default_expr(artifact, *then_expr, parameter_fields)?;
            let else_expr = lower_parameter_default_expr(artifact, *else_expr, parameter_fields)?;
            Ok(format!(
                "if {condition} {{ {then_expr} }} else {{ {else_expr} }}"
            ))
        }
        HirExprKind::SystemFunction { name, args }
            if name.as_str().eq_ignore_ascii_case("$simparam")
                || name.as_str().eq_ignore_ascii_case("simparam") =>
        {
            let Some(fallback) = args.get(1) else {
                return Err(unsupported(
                    artifact,
                    "parameter default $simparam without explicit fallback",
                ));
            };
            lower_parameter_default_expr(artifact, *fallback, parameter_fields)
        }
        other => Err(unsupported(
            artifact,
            format!("parameter default expression kind {other:?}"),
        )),
    }
}

fn lower_parameter_default_condition(
    artifact: &CanonicalIrArtifact,
    expr: ExprId,
    parameter_fields: &HashMap<String, String>,
) -> Result<String, RustBackendError> {
    let expression = artifact
        .mir
        .expressions
        .get(usize::from(expr))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("parameter default condition {expr} is outside MIR arena"),
            )
        })?;

    match &expression.kind {
        HirExprKind::Binary { op, left, right } if comparison_operator(op.as_str()).is_some() => {
            let operator = comparison_operator(op.as_str()).expect("checked above");
            let left = lower_parameter_default_expr(artifact, *left, parameter_fields)?;
            let right = lower_parameter_default_expr(artifact, *right, parameter_fields)?;
            Ok(format!("({left} {operator} {right})"))
        }
        HirExprKind::Binary { op, left, right } if op.as_str() == "And" || op.as_str() == "Or" => {
            let left = lower_parameter_default_condition(artifact, *left, parameter_fields)?;
            let right = lower_parameter_default_condition(artifact, *right, parameter_fields)?;
            let operator = if op.as_str() == "And" { "&&" } else { "||" };
            Ok(format!("({left} {operator} {right})"))
        }
        HirExprKind::Unary { op, operand } if op.as_str() == "Not" => {
            let operand = lower_parameter_default_condition(artifact, *operand, parameter_fields)?;
            Ok(negate_condition(&operand))
        }
        _ => {
            let value = lower_parameter_default_expr(artifact, expr, parameter_fields)?;
            Ok(format!("({value} != 0.0)"))
        }
    }
}

fn validate_parameter_value_for_codegen(
    artifact: &CanonicalIrArtifact,
    parameter_name: &str,
    value: f64,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<(), RustBackendError> {
    if !value.is_finite() {
        return Err(unsupported(
            artifact,
            format!("non-finite default for parameter '{parameter_name}'"),
        ));
    }
    if let Some(range) = range {
        if !range_contains(range, value) {
            return Err(unsupported(
                artifact,
                format!("default for parameter '{parameter_name}' violates declared range"),
            ));
        }
        if range.exclude.iter().any(|excluded| !excluded.is_finite()) {
            return Err(unsupported(
                artifact,
                format!("non-finite exclude constraint for parameter '{parameter_name}'"),
            ));
        }
    }
    Ok(())
}

fn range_contains(range: &crate::canonical_ir::HirParamRange, value: f64) -> bool {
    if let Some(min) = range.min {
        if range.min_exclusive {
            if value <= min {
                return false;
            }
        } else if value < min {
            return false;
        }
    }
    if let Some(max) = range.max {
        if range.max_exclusive {
            if value >= max {
                return false;
            }
        } else if value > max {
            return false;
        }
    }
    !range.exclude.contains(&value)
}

fn parameter_validation_call(
    parameter_name: &str,
    value_expr: &str,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<String, RustBackendError> {
    if !parameter_range_has_runtime_constraints(parameter_name, range)? {
        return Ok(format!(
            "validate_finite_parameter({parameter_name:?}, {value_expr})"
        ));
    }

    let min = range
        .and_then(|range| range.min.filter(|value| value.is_finite()))
        .map(range_bound_arg);
    let max = range
        .and_then(|range| range.max.filter(|value| value.is_finite()))
        .map(range_bound_arg);
    let min_exclusive = range.is_some_and(|range| range.min_exclusive);
    let max_exclusive = range.is_some_and(|range| range.max_exclusive);
    let exclude = if let Some(range) = range {
        let mut excluded = Vec::with_capacity(range.exclude.len());
        for value in &range.exclude {
            if !value.is_finite() {
                return Err(RustBackendError::unsupported(
                    "<generated>",
                    parameter_name,
                    "non-finite parameter exclude constraint",
                ));
            }
            excluded.push(range_excluded_arg(*value));
        }
        format!("&[{}]", excluded.join(", "))
    } else {
        "&[]".to_string()
    };

    Ok(format!(
        "validate_parameter({parameter_name:?}, {value_expr}, {}, {min_exclusive}, {}, {max_exclusive}, {exclude})",
        min.unwrap_or_else(|| "None".to_string()),
        max.unwrap_or_else(|| "None".to_string())
    ))
}

fn parameter_range_has_runtime_constraints(
    parameter_name: &str,
    range: Option<&crate::canonical_ir::HirParamRange>,
) -> Result<bool, RustBackendError> {
    let Some(range) = range else {
        return Ok(false);
    };
    if range.exclude.iter().any(|value| !value.is_finite()) {
        return Err(RustBackendError::unsupported(
            "<generated>",
            parameter_name,
            "non-finite parameter exclude constraint",
        ));
    }
    Ok(range.min.is_some_and(|value| value.is_finite())
        || range.max.is_some_and(|value| value.is_finite())
        || !range.exclude.is_empty())
}

fn range_bound_arg(value: f64) -> String {
    let label = format_f64(value);
    format!("Some(({}, {:?}))", label, label)
}

fn range_excluded_arg(value: f64) -> String {
    let label = format_f64(value);
    format!("({}, {:?})", label, label)
}

fn generate_shared_parameter_validator() -> String {
    [
        "fn validate_finite_parameter(name: &str, value: f64) -> Result<(), String> {",
        "    if !value.is_finite() {",
        "        return Err(format!(\"parameter '{}' must be finite, got {}\", name, value));",
        "    }",
        "    Ok(())",
        "}",
        "",
        "fn validate_parameter(",
        "    name: &str,",
        "    value: f64,",
        "    min: Option<(f64, &str)>,",
        "    min_exclusive: bool,",
        "    max: Option<(f64, &str)>,",
        "    max_exclusive: bool,",
        "    excluded: &[(f64, &str)],",
        ") -> Result<(), String> {",
        "    validate_finite_parameter(name, value)?;",
        "    if let Some((min, label)) = min {",
        "        if min_exclusive {",
        "            if value <= min {",
        "                return Err(format!(\"parameter '{}' must be > {}, got {}\", name, label, value));",
        "            }",
        "        } else if value < min {",
        "            return Err(format!(\"parameter '{}' must be >= {}, got {}\", name, label, value));",
        "        }",
        "    }",
        "    if let Some((max, label)) = max {",
        "        if max_exclusive {",
        "            if value >= max {",
        "                return Err(format!(\"parameter '{}' must be < {}, got {}\", name, label, value));",
        "            }",
        "        } else if value > max {",
        "            return Err(format!(\"parameter '{}' must be <= {}, got {}\", name, label, value));",
        "        }",
        "    }",
        "    for (excluded, label) in excluded {",
        "        if value == *excluded {",
        "            return Err(format!(\"parameter '{}' must not equal {}, got {}\", name, label, value));",
        "        }",
        "    }",
        "    Ok(())",
        "}",
    ]
    .join("\n")
}

fn emit_stamp_common_bindings(
    out: &mut String,
    common_usage: StampCommonUsage,
    needs_timestep: bool,
) {
    out.push_str("        let p = Box::as_ref(&self.params);\n");
    out.push_str("        let nodes = &(*self).nodes;\n");
    out.push_str("        let branches = &(*self).branches;\n");
    if common_usage.param_given {
        out.push_str("        let param_given = self.param_given.as_ref();\n");
    }
    out.push_str("        let multiplicity = (*self).multiplicity;\n");
    if common_usage.time {
        out.push_str("        let time = (*self).time;\n");
    }
    if needs_timestep {
        out.push_str("        let timestep = (*self).timestep;\n");
    }
}

fn generate_stamp_file(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    ddt_slots: &DdtSlots,
    transient_liveness: &TransientLiveness,
    reactive_liveness: &ReactiveLiveness,
    potential_branch_slots: &PotentialBranchSlots,
) -> Result<StampFiles, RustBackendError> {
    let mut out = String::new();
    out.push_str("#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]\n\n");
    out.push_str("use super::state::Instance;\n");
    out.push_str(&format!(
        "use {}::{{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper}};\n\n",
        options.runtime_path
    ));
    out.push_str(&format!(
        "use {}::support::{{AdValue as GenericAdValue, ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch}};\n\n",
        options.runtime_path
    ));
    out.push_str(
        "type AdValue = GenericAdValue<{ Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;\n",
    );
    out.push_str(
        "type Scratch = GenericScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;\n",
    );
    out.push_str(
        "type ReactiveScratch = GenericReactiveScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;\n\n",
    );
    out.push_str("const LIMEXP_MAX: f64 = 5.54062238439351e34;\n");
    out.push_str("const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;\n\n");
    out.push_str("#[inline]\n");
    out.push_str("fn eval_ddt<const STATE_COUNT: usize>(\n");
    out.push_str("    current: &mut [f64; STATE_COUNT],\n");
    out.push_str("    previous: &mut [f64; STATE_COUNT],\n");
    out.push_str("    initialized: &mut [bool; STATE_COUNT],\n");
    out.push_str("    ddt_active: bool,\n");
    out.push_str("    ddt_scale: f64,\n");
    out.push_str("    slot: usize,\n");
    out.push_str("    value: f64,\n");
    out.push_str(") -> f64 {\n");
    out.push_str(
        "    debug_assert!(slot < STATE_COUNT, \"generated ddt state slot out of range\");\n",
    );
    out.push_str(
        "    let previous_value = if initialized[slot] { previous[slot] } else { value };\n",
    );
    out.push_str("    current[slot] = value;\n");
    out.push_str("    if ddt_active {\n");
    out.push_str("        (value - previous_value) * ddt_scale\n");
    out.push_str("    } else {\n");
    out.push_str("        previous[slot] = value;\n");
    out.push_str("        initialized[slot] = true;\n");
    out.push_str("        0.0\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("#[inline]\n");
    out.push_str("fn ddt_jacobian(timestep: f64, derivative: f64) -> f64 {\n");
    out.push_str("    if timestep.abs() > Instance::DDT_EPSILON {\n");
    out.push_str("        derivative / timestep\n");
    out.push_str("    } else {\n");
    out.push_str("        0.0\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("#[inline]\n");
    out.push_str("fn eval_idt<const STATE_COUNT: usize>(\n");
    out.push_str("    current: &mut [f64; STATE_COUNT],\n");
    out.push_str("    previous: &mut [f64; STATE_COUNT],\n");
    out.push_str("    initialized: &mut [bool; STATE_COUNT],\n");
    out.push_str("    ddt_active: bool,\n");
    out.push_str("    idt_scale: f64,\n");
    out.push_str("    slot: usize,\n");
    out.push_str("    value: f64,\n");
    out.push_str("    ic: f64,\n");
    out.push_str(") -> f64 {\n");
    out.push_str(
        "    debug_assert!(slot < STATE_COUNT, \"generated idt state slot out of range\");\n",
    );
    out.push_str("    let previous_value = if initialized[slot] { previous[slot] } else { ic };\n");
    out.push_str("    let current_value = if ddt_active {\n");
    out.push_str("        previous_value + value * idt_scale\n");
    out.push_str("    } else {\n");
    out.push_str("        ic\n");
    out.push_str("    };\n");
    out.push_str("    current[slot] = current_value;\n");
    out.push_str("    if !ddt_active {\n");
    out.push_str("        previous[slot] = current_value;\n");
    out.push_str("        initialized[slot] = true;\n");
    out.push_str("    }\n");
    out.push_str("    current_value\n");
    out.push_str("}\n\n");
    out.push_str("#[inline]\n");
    out.push_str("fn idt_jacobian(timestep: f64, derivative: f64) -> f64 {\n");
    out.push_str("    if timestep.abs() > Instance::DDT_EPSILON {\n");
    out.push_str("        derivative * timestep\n");
    out.push_str("    } else {\n");
    out.push_str("        0.0\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    let mut helper_modules = StampHelperModules::default();
    let common_usage = StampCommonUsage::from_artifact(artifact);
    let transient_operator_usage = StampOperatorUsage::transient(ddt_slots);
    out.push_str("impl Instance {\n");
    out.push_str(
        "    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n",
    );
    emit_stamp_common_bindings(&mut out, common_usage, transient_operator_usage.has_any());
    emit_stamp_body(
        artifact,
        parameter_fields,
        variable_fields,
        ddt_slots,
        transient_liveness,
        reactive_liveness,
        potential_branch_slots,
        false,
        "stamp_transient",
        common_usage,
        &mut helper_modules,
        &mut out,
    )?;
    out.push_str("    }\n\n");
    out.push_str(
        "    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {\n",
    );
    if ddt_slots.len() > 0 {
        emit_stamp_common_bindings(&mut out, common_usage, false);
        emit_stamp_body(
            artifact,
            parameter_fields,
            variable_fields,
            ddt_slots,
            transient_liveness,
            reactive_liveness,
            potential_branch_slots,
            true,
            "stamp_reactive",
            common_usage,
            &mut helper_modules,
            &mut out,
        )?;
    }
    out.push_str("    }\n");
    out.push_str("}\n");
    split_marked_equation_chunks(
        &mut out,
        &mut helper_modules,
        common_usage,
        transient_operator_usage,
    );
    let helpers = helper_modules.finish();
    if !helpers.is_empty() {
        let declarations = helpers
            .iter()
            .map(|file| {
                let module_name = file
                    .relative_path
                    .strip_suffix(".rs")
                    .expect("helper module path must end in .rs")
                    .replace('\\', "/");
                format!(
                    "#[path = \"{}\"]\nmod {};",
                    file.relative_path.replace('\\', "/"),
                    module_name.replace('/', "::")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        out = out.replacen(
            "const THERMAL_VOLTAGE_PER_K",
            &format!("{declarations}\n\nconst THERMAL_VOLTAGE_PER_K"),
            1,
        );
    }
    if helpers.is_empty() && !out.contains("GeneratedDerivative::") {
        out = out.replace(
            &format!(
                "use {}::{{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper}};",
                options.runtime_path
            ),
            &format!(
                "use {}::{{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper}};",
                options.runtime_path
            ),
        );
    }
    Ok(StampFiles {
        stamp: out,
        helpers,
    })
}

struct StampFiles {
    stamp: String,
    helpers: Vec<GeneratedRustFile>,
}

fn generate_scratch_struct() -> String {
    let mut out = [
        "struct Scratch {",
        "    values: [f64; Instance::VARIABLE_COUNT],",
        "    bool_values: [bool; Instance::VARIABLE_COUNT],",
        "    node_derivatives: [[f64; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "    branch_derivatives: [[f64; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "}",
        "",
        "impl Scratch {",
        "    fn new() -> Self {",
        "        *Self::new_box()",
        "    }",
        "",
        "    fn new_box() -> Box<Self> {",
        "        let mut boxed = Box::<Self>::new_uninit();",
        "        unsafe {",
        "            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);",
        "            boxed.assume_init()",
        "        }",
        "    }",
        "",
        "    fn new_value() -> Self {",
        "        Self {",
        "            values: [0.0; Instance::VARIABLE_COUNT],",
        "            bool_values: [false; Instance::VARIABLE_COUNT],",
        "            node_derivatives: [[0.0; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "            branch_derivatives: [[0.0; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn ad_value(&self, index: usize) -> AdValue {",
        "        AdValue { value: self.values[index], node_derivatives: self.node_derivatives[index], branch_derivatives: self.branch_derivatives[index] }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ad(&mut self, index: usize, value: &AdValue) {",
        "        self.values[index] = value.value;",
        "        self.node_derivatives[index] = value.node_derivatives;",
        "        self.branch_derivatives[index] = value.branch_derivatives;",
        "    }",
        "",
        "    #[inline]",
        "    fn copy_ad(&mut self, target: usize, source: usize) {",
        "        self.values[target] = self.values[source];",
        "        self.node_derivatives[target] = self.node_derivatives[source];",
        "        self.branch_derivatives[target] = self.branch_derivatives[source];",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scalar(&mut self, index: usize, value: f64) {",
        "        self.values[index] = value;",
        "        self.node_derivatives[index] = [0.0; Instance::NODE_COUNT];",
        "        self.branch_derivatives[index] = [0.0; Instance::BRANCH_COUNT];",
        "    }",
    ]
    .join("\n");
    out.push('\n');
    out.push_str(&generate_scratch_operation_helpers());
    out.push_str("\n}\n\n");
    out.push_str(&[
        "",
        "struct ReactiveScratch {",
        "    values: [f64; Instance::VARIABLE_COUNT],",
        "    bool_values: [bool; Instance::VARIABLE_COUNT],",
        "    node_derivatives: [[f64; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "    branch_derivatives: [[f64; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "    reactive_values: [f64; Instance::VARIABLE_COUNT],",
        "    reactive_node_derivatives: [[f64; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "    reactive_branch_derivatives: [[f64; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "}",
        "",
        "impl ReactiveScratch {",
        "    fn new() -> Self {",
        "        *Self::new_box()",
        "    }",
        "",
        "    fn new_box() -> Box<Self> {",
        "        let mut boxed = Box::<Self>::new_uninit();",
        "        unsafe {",
        "            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);",
        "            boxed.assume_init()",
        "        }",
        "    }",
        "",
        "    fn new_value() -> Self {",
        "        Self {",
        "            values: [0.0; Instance::VARIABLE_COUNT],",
        "            bool_values: [false; Instance::VARIABLE_COUNT],",
        "            node_derivatives: [[0.0; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "            branch_derivatives: [[0.0; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "            reactive_values: [0.0; Instance::VARIABLE_COUNT],",
        "            reactive_node_derivatives: [[0.0; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],",
        "            reactive_branch_derivatives: [[0.0; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn ad_value(&self, index: usize) -> AdValue {",
        "        AdValue { value: self.values[index], node_derivatives: self.node_derivatives[index], branch_derivatives: self.branch_derivatives[index] }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ad(&mut self, index: usize, value: &AdValue) {",
        "        self.values[index] = value.value;",
        "        self.node_derivatives[index] = value.node_derivatives;",
        "        self.branch_derivatives[index] = value.branch_derivatives;",
        "    }",
        "",
        "    #[inline]",
        "    fn copy_ad(&mut self, target: usize, source: usize) {",
        "        self.values[target] = self.values[source];",
        "        self.node_derivatives[target] = self.node_derivatives[source];",
        "        self.branch_derivatives[target] = self.branch_derivatives[source];",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scalar(&mut self, index: usize, value: f64) {",
        "        self.values[index] = value;",
        "        self.node_derivatives[index] = [0.0; Instance::NODE_COUNT];",
        "        self.branch_derivatives[index] = [0.0; Instance::BRANCH_COUNT];",
        "    }",
    ]
    .join("\n"));
    out.push('\n');
    out.push_str(&generate_scratch_operation_helpers());
    out.push_str("\n}\n\n");
    out
}

fn generate_scratch_operation_helpers() -> String {
    [
        "",
        "    #[inline]",
        "    fn store_ad_value(&mut self, index: usize, value: AdValue) {",
        "        self.values[index] = value.value;",
        "        self.node_derivatives[index] = value.node_derivatives;",
        "        self.branch_derivatives[index] = value.branch_derivatives;",
        "    }",
        "",
        "    #[inline]",
        "    fn store_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; Instance::NODE_COUNT], pos: Option<usize>, neg: Option<usize>) {",
        "        self.store_scaled_voltage(index, ctx, nodes, pos, neg, 1.0);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; Instance::NODE_COUNT], pos: Option<usize>, neg: Option<usize>, scale: f64) {",
        "        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        self.values[index] = (pos_value - neg_value) * scale;",
        "        self.node_derivatives[index] = [0.0; Instance::NODE_COUNT];",
        "        self.branch_derivatives[index] = [0.0; Instance::BRANCH_COUNT];",
        "        if let Some(node) = pos { self.node_derivatives[index][node] += scale; }",
        "        if let Some(node) = neg { self.node_derivatives[index][node] -= scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; Instance::NODE_COUNT], pos: Option<usize>, neg: Option<usize>, offset: f64) {",
        "        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        self.values[index] = pos_value - neg_value + offset;",
        "        self.node_derivatives[index] = [0.0; Instance::NODE_COUNT];",
        "        self.branch_derivatives[index] = [0.0; Instance::BRANCH_COUNT];",
        "        if let Some(node) = pos { self.node_derivatives[index][node] += 1.0; }",
        "        if let Some(node) = neg { self.node_derivatives[index][node] -= 1.0; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_abs_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; Instance::NODE_COUNT], pos: Option<usize>, neg: Option<usize>) {",
        "        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let voltage = pos_value - neg_value;",
        "        let derivative_scale = if voltage >= 0.0 { 1.0 } else { -1.0 };",
        "        self.values[index] = voltage.abs();",
        "        self.node_derivatives[index] = [0.0; Instance::NODE_COUNT];",
        "        self.branch_derivatives[index] = [0.0; Instance::BRANCH_COUNT];",
        "        if let Some(node) = pos { self.node_derivatives[index][node] += derivative_scale; }",
        "        if let Some(node) = neg { self.node_derivatives[index][node] -= derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sub_voltage_abs_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; Instance::NODE_COUNT], left_pos: Option<usize>, left_neg: Option<usize>, abs_pos: Option<usize>, abs_neg: Option<usize>) {",
        "        let left_pos_value = left_pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let left_neg_value = left_neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let abs_pos_value = abs_pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let abs_neg_value = abs_neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let abs_voltage = abs_pos_value - abs_neg_value;",
        "        let abs_derivative_scale = if abs_voltage >= 0.0 { 1.0 } else { -1.0 };",
        "        self.values[index] = left_pos_value - left_neg_value - abs_voltage.abs();",
        "        self.node_derivatives[index] = [0.0; Instance::NODE_COUNT];",
        "        self.branch_derivatives[index] = [0.0; Instance::BRANCH_COUNT];",
        "        if let Some(node) = left_pos { self.node_derivatives[index][node] += 1.0; }",
        "        if let Some(node) = left_neg { self.node_derivatives[index][node] -= 1.0; }",
        "        if let Some(node) = abs_pos { self.node_derivatives[index][node] -= abs_derivative_scale; }",
        "        if let Some(node) = abs_neg { self.node_derivatives[index][node] += abs_derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_mul_voltage_ad(&mut self, index: usize, value: AdValue, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; Instance::NODE_COUNT], pos: Option<usize>, neg: Option<usize>) {",
        "        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let voltage = pos_value - neg_value;",
        "        self.values[index] = value.value * voltage;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * voltage; }",
        "        if let Some(node) = pos { self.node_derivatives[index][node] += value.value; }",
        "        if let Some(node) = neg { self.node_derivatives[index][node] -= value.value; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * voltage; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_voltage_by_ad(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; Instance::NODE_COUNT], pos: Option<usize>, neg: Option<usize>, right: AdValue) {",
        "        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);",
        "        let reciprocal = 1.0 / right.value;",
        "        let quotient = (pos_value - neg_value) * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = right.node_derivatives[axis] * right_scale; }",
        "        if let Some(node) = pos { self.node_derivatives[index][node] += reciprocal; }",
        "        if let Some(node) = neg { self.node_derivatives[index][node] -= reciprocal; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = right.branch_derivatives[axis] * right_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_add_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        self.values[index] = left.value + right.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] + right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] + right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sub_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        self.values[index] = left.value - right.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] - right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] - right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_mul_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        self.values[index] = left.value * right.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] * right.value + left.value * right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] * right.value + left.value * right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        let reciprocal = 1.0 / right.value;",
        "        let quotient = left.value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] * reciprocal + right.node_derivatives[axis] * right_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] * reciprocal + right.branch_derivatives[axis] * right_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_rem_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        self.store_ad_value(index, AdValue::rem(left, right));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_pow_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        let base = left.value;",
        "        let exponent = right.value;",
        "        let output = base.powf(exponent);",
        "        self.values[index] = output;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = AdValue::pow_derivative(output, base, exponent, left.node_derivatives[axis], right.node_derivatives[axis]); }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = AdValue::pow_derivative(output, base, exponent, left.branch_derivatives[axis], right.branch_derivatives[axis]); }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_min_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        let selected = if left.value <= right.value { left } else { right };",
        "        self.values[index] = selected.value;",
        "        self.node_derivatives[index] = selected.node_derivatives;",
        "        self.branch_derivatives[index] = selected.branch_derivatives;",
        "    }",
        "",
        "    #[inline]",
        "    fn store_max_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        let selected = if left.value >= right.value { left } else { right };",
        "        self.values[index] = selected.value;",
        "        self.node_derivatives[index] = selected.node_derivatives;",
        "        self.branch_derivatives[index] = selected.branch_derivatives;",
        "    }",
        "",
        "    #[inline]",
        "    fn store_hypot_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        self.store_ad_value(index, AdValue::hypot(left, right));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_atan2_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        self.store_ad_value(index, AdValue::atan2(left, right));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scale_ad(&mut self, index: usize, value: AdValue, scale: f64) {",
        "        self.values[index] = value.value * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, value, offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_ad_value(&mut self, index: usize, mut value: AdValue, offset: f64) {",
        "        value.value += offset;",
        "        self.store_ad_value(index, value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_scaled_ad(&mut self, index: usize, value: AdValue, scale: f64, offset: f64) {",
        "        self.values[index] = value.value * scale + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_add_ad(&mut self, index: usize, left: AdValue, right: AdValue, offset: f64) {",
        "        self.values[index] = left.value + right.value + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] + right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] + right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_sub_ad(&mut self, index: usize, left: AdValue, right: AdValue, offset: f64) {",
        "        self.values[index] = left.value - right.value + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] - right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] - right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_mul_ad(&mut self, index: usize, left: AdValue, right: AdValue, offset: f64) {",
        "        self.values[index] = left.value * right.value + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] * right.value + left.value * right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] * right.value + left.value * right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_div_ad(&mut self, index: usize, left: AdValue, right: AdValue, offset: f64) {",
        "        let reciprocal = 1.0 / right.value;",
        "        let quotient = left.value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] * reciprocal + right.node_derivatives[axis] * right_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] * reciprocal + right.branch_derivatives[axis] * right_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_exp_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        let output = value.value.exp();",
        "        self.values[index] = output + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * output; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * output; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_sqrt_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        let root = value.value.sqrt();",
        "        self.values[index] = root + offset;",
        "        let derivative_scale = 1.0 / (2.0 * root);",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_ln_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.values[index] = value.value.ln() + offset;",
        "        let derivative_scale = 1.0 / value.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_limited_exp_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        if value.value > 80.0 {",
        "            self.values[index] = LIMEXP_MAX * (1.0 + value.value - 80.0) + offset;",
        "            for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * LIMEXP_MAX; }",
        "            for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * LIMEXP_MAX; }",
        "        } else if value.value < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35 + offset);",
        "        } else {",
        "            let output = value.value.exp();",
        "            self.values[index] = output + offset;",
        "            for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * output; }",
        "            for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * output; }",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_powf_ad(&mut self, index: usize, value: AdValue, exponent: f64, offset: f64) {",
        "        let output = value.value.powf(exponent);",
        "        self.values[index] = output + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = AdValue::pow_derivative(output, value.value, exponent, value.node_derivatives[axis], 0.0); }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = AdValue::pow_derivative(output, value.value, exponent, value.branch_derivatives[axis], 0.0); }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue, offset: f64) {",
        "        self.values[index] = scalar - value.value + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = -value.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = -value.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue, offset: f64) {",
        "        let reciprocal = 1.0 / value.value;",
        "        let quotient = scalar * reciprocal;",
        "        let derivative_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_pow_ad(&mut self, index: usize, left: AdValue, right: AdValue, offset: f64) {",
        "        let base = left.value;",
        "        let exponent = right.value;",
        "        let output = base.powf(exponent);",
        "        self.values[index] = output + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = AdValue::pow_derivative(output, base, exponent, left.node_derivatives[axis], right.node_derivatives[axis]); }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = AdValue::pow_derivative(output, base, exponent, left.branch_derivatives[axis], right.branch_derivatives[axis]); }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_min_ad(&mut self, index: usize, left: AdValue, right: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::min(left, right), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_max_ad(&mut self, index: usize, left: AdValue, right: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::max(left, right), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_rem_ad(&mut self, index: usize, left: AdValue, right: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::rem(left, right), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_hypot_ad(&mut self, index: usize, left: AdValue, right: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::hypot(left, right), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_atan2_ad(&mut self, index: usize, left: AdValue, right: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::atan2(left, right), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_rem_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::rem_from_scalar(scalar, value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_pow_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue, offset: f64) {",
        "        let exponent = value.value;",
        "        let output = scalar.powf(exponent);",
        "        self.values[index] = output + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = AdValue::pow_derivative(output, scalar, exponent, 0.0, value.node_derivatives[axis]); }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = AdValue::pow_derivative(output, scalar, exponent, 0.0, value.branch_derivatives[axis]); }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_min_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::min_from_scalar(scalar, value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_max_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::max_from_scalar(scalar, value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_rem_with_scalar_ad(&mut self, index: usize, value: AdValue, scalar: f64, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::rem_with_scalar(value, scalar), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_min_with_scalar_ad(&mut self, index: usize, value: AdValue, scalar: f64, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::min_with_scalar(value, scalar), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_max_with_scalar_ad(&mut self, index: usize, value: AdValue, scalar: f64, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::max_with_scalar(value, scalar), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_abs_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::abs(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_square_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::square(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_limexp_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::limexp(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_log10_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::log10(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_sin_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::sin(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_cos_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::cos(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_tan_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::tan(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_atan_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::atan(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_sinh_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::sinh(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_cosh_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::cosh(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_tanh_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::tanh(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_asinh_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::asinh(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_acosh_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::acosh(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_atanh_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::atanh(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_floor_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::floor(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_ceil_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        self.store_offset_ad_value(index, AdValue::ceil(value), offset);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_offset_ad(&mut self, index: usize, value: AdValue, offset: f64, scale: f64) {",
        "        self.values[index] = (value.value + offset) * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_add_ad(&mut self, index: usize, left: AdValue, right: AdValue, scale: f64) {",
        "        self.values[index] = (left.value + right.value) * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] + right.node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] + right.branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_sub_ad(&mut self, index: usize, left: AdValue, right: AdValue, scale: f64) {",
        "        self.values[index] = (left.value - right.value) * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] - right.node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] - right.branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_mul_ad(&mut self, index: usize, left: AdValue, right: AdValue, scale: f64) {",
        "        self.values[index] = left.value * right.value * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] * right.value + left.value * right.node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] * right.value + left.value * right.branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_div_ad(&mut self, index: usize, left: AdValue, right: AdValue, scale: f64) {",
        "        let reciprocal = 1.0 / right.value;",
        "        let quotient = left.value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] * reciprocal + right.node_derivatives[axis] * right_scale) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] * reciprocal + right.branch_derivatives[axis] * right_scale) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_exp_ad(&mut self, index: usize, value: AdValue, scale: f64) {",
        "        let output = value.value.exp() * scale;",
        "        self.values[index] = output;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * output; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * output; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_ln_ad(&mut self, index: usize, value: AdValue, scale: f64) {",
        "        self.values[index] = value.value.ln() * scale;",
        "        let derivative_scale = scale / value.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_sqrt_ad(&mut self, index: usize, value: AdValue, scale: f64) {",
        "        let root = value.value.sqrt();",
        "        self.values[index] = root * scale;",
        "        let derivative_scale = scale / (2.0 * root);",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_limexp_ad(&mut self, index: usize, value: AdValue, scale: f64) {",
        "        if value.value < 80.0 {",
        "            let output = value.value.exp() * scale;",
        "            self.values[index] = output;",
        "            for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * output; }",
        "            for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * output; }",
        "        } else {",
        "            self.values[index] = LIMEXP_MAX * (1.0 + (value.value - 80.0)) * scale;",
        "            let derivative_scale = LIMEXP_MAX * scale;",
        "            for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "            for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_limited_exp_ad(&mut self, index: usize, value: AdValue, scale: f64) {",
        "        if value.value > 80.0 {",
        "            self.values[index] = LIMEXP_MAX * (1.0 + value.value - 80.0) * scale;",
        "            let derivative_scale = LIMEXP_MAX * scale;",
        "            for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "            for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "        } else if value.value < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35 * scale);",
        "        } else {",
        "            let output = value.value.exp() * scale;",
        "            self.values[index] = output;",
        "            for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * output; }",
        "            for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * output; }",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_abs_ad(&mut self, index: usize, value: AdValue, scale: f64) {",
        "        self.values[index] = value.value.abs() * scale;",
        "        let derivative_scale = if value.value >= 0.0 { scale } else { -scale };",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_powf_ad(&mut self, index: usize, value: AdValue, exponent: f64, scale: f64) {",
        "        let output = value.value.powf(exponent);",
        "        self.values[index] = output * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = AdValue::pow_derivative(output, value.value, exponent, value.node_derivatives[axis], 0.0) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = AdValue::pow_derivative(output, value.value, exponent, value.branch_derivatives[axis], 0.0) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue, scale: f64) {",
        "        self.values[index] = (scalar - value.value) * scale;",
        "        let derivative_scale = -scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue, scale: f64) {",
        "        let reciprocal = 1.0 / value.value;",
        "        let quotient = scalar * reciprocal;",
        "        let derivative_scale = -quotient * reciprocal * scale;",
        "        self.values[index] = quotient * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_neg_ad(&mut self, index: usize, value: AdValue) {",
        "        self.values[index] = -value.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = -value.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = -value.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_ad(&mut self, index: usize, value: AdValue) {",
        "        let root = value.value.sqrt();",
        "        self.values[index] = root;",
        "        let derivative_scale = 1.0 / (2.0 * root);",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_offset_ad(&mut self, index: usize, value: AdValue, offset: f64) {",
        "        let root = (value.value + offset).sqrt();",
        "        let derivative_scale = 1.0 / (2.0 * root);",
        "        self.values[index] = root;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_scaled_ad(&mut self, index: usize, value: AdValue, scale: f64) {",
        "        let root = (value.value * scale).sqrt();",
        "        let derivative_scale = scale / (2.0 * root);",
        "        self.values[index] = root;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_add_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        let root = (left.value + right.value).sqrt();",
        "        let derivative_scale = 1.0 / (2.0 * root);",
        "        self.values[index] = root;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] + right.node_derivatives[axis]) * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] + right.branch_derivatives[axis]) * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_sub_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        let root = (left.value - right.value).sqrt();",
        "        let derivative_scale = 1.0 / (2.0 * root);",
        "        self.values[index] = root;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] - right.node_derivatives[axis]) * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] - right.branch_derivatives[axis]) * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_mul_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        let raw = left.value * right.value;",
        "        let root = raw.sqrt();",
        "        let derivative_scale = 1.0 / (2.0 * root);",
        "        self.values[index] = root;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] * right.value + left.value * right.node_derivatives[axis]) * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] * right.value + left.value * right.branch_derivatives[axis]) * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_div_ad(&mut self, index: usize, left: AdValue, right: AdValue) {",
        "        let reciprocal = 1.0 / right.value;",
        "        let raw = left.value * reciprocal;",
        "        let root = raw.sqrt();",
        "        let derivative_scale = 1.0 / (2.0 * root);",
        "        let right_scale = -raw * reciprocal;",
        "        self.values[index] = root;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] * reciprocal + right.node_derivatives[axis] * right_scale) * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] * reciprocal + right.branch_derivatives[axis] * right_scale) * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_abs_ad(&mut self, index: usize, value: AdValue) {",
        "        let raw = value.value.abs();",
        "        let root = raw.sqrt();",
        "        let derivative_scale = if value.value >= 0.0 { 1.0 / (2.0 * root) } else { -1.0 / (2.0 * root) };",
        "        self.values[index] = root;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        let root = (scalar - value.value).sqrt();",
        "        let derivative_scale = -1.0 / (2.0 * root);",
        "        self.values[index] = root;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        let raw = scalar / value.value;",
        "        let root = raw.sqrt();",
        "        let derivative_scale = -raw / (value.value * 2.0 * root);",
        "        self.values[index] = root;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_exp_ad(&mut self, index: usize, value: AdValue) {",
        "        let output = value.value.exp();",
        "        self.values[index] = output;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * output; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * output; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_ad(&mut self, index: usize, value: AdValue) {",
        "        self.values[index] = value.value.ln();",
        "        let derivative_scale = 1.0 / value.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_abs_ad(&mut self, index: usize, value: AdValue) {",
        "        self.values[index] = value.value.abs();",
        "        let derivative_scale = if value.value >= 0.0 { 1.0 } else { -1.0 };",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_square_ad(&mut self, index: usize, value: AdValue) {",
        "        self.values[index] = value.value * value.value;",
        "        let derivative_scale = 2.0 * value.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limexp_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::limexp(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limited_exp_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::limited_exp(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_log10_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::log10(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sin_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::sin(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_cos_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::cos(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_tan_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::tan(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_atan_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::atan(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sinh_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::sinh(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_cosh_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::cosh(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_tanh_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::tanh(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_asinh_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::asinh(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_acosh_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::acosh(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_atanh_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::atanh(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_floor_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::floor(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ceil_ad(&mut self, index: usize, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::ceil(value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        self.values[index] = scalar - value.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = -value.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = -value.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        let reciprocal = 1.0 / value.value;",
        "        let quotient = scalar * reciprocal;",
        "        let derivative_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_offset_ad(&mut self, index: usize, scalar: f64, value: AdValue, offset: f64) {",
        "        let denominator = value.value + offset;",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let derivative_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_scaled_ad(&mut self, index: usize, scalar: f64, value: AdValue, scale: f64) {",
        "        let denominator = value.value * scale;",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let derivative_scale = -quotient * reciprocal * scale;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_add_ad(&mut self, index: usize, scalar: f64, left: AdValue, right: AdValue) {",
        "        let denominator = left.value + right.value;",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let denominator_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] + right.node_derivatives[axis]) * denominator_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] + right.branch_derivatives[axis]) * denominator_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_sub_ad(&mut self, index: usize, scalar: f64, left: AdValue, right: AdValue) {",
        "        let denominator = left.value - right.value;",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let denominator_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] - right.node_derivatives[axis]) * denominator_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] - right.branch_derivatives[axis]) * denominator_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_mul_ad(&mut self, index: usize, scalar: f64, left: AdValue, right: AdValue) {",
        "        let denominator = left.value * right.value;",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let denominator_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] * right.value + left.value * right.node_derivatives[axis]) * denominator_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] * right.value + left.value * right.branch_derivatives[axis]) * denominator_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_div_ad(&mut self, index: usize, scalar: f64, left: AdValue, right: AdValue) {",
        "        let right_reciprocal = 1.0 / right.value;",
        "        let denominator = left.value * right_reciprocal;",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let denominator_scale = -quotient * reciprocal;",
        "        let right_scale = -denominator * right_reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] * right_reciprocal + right.node_derivatives[axis] * right_scale) * denominator_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] * right_reciprocal + right.branch_derivatives[axis] * right_scale) * denominator_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_sqrt_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        let root = value.value.sqrt();",
        "        let reciprocal = 1.0 / root;",
        "        let quotient = scalar * reciprocal;",
        "        let derivative_scale = -quotient / (2.0 * value.value);",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_square_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        let denominator = value.value * value.value;",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let derivative_scale = -2.0 * quotient / value.value;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, denominator_scalar: f64, value: AdValue) {",
        "        let denominator = denominator_scalar - value.value;",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let derivative_scale = quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_div_from_scalar_ad(&mut self, index: usize, scalar: f64, denominator_scalar: f64, value: AdValue) {",
        "        let quotient_scale = scalar / denominator_scalar;",
        "        self.values[index] = value.value * quotient_scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * quotient_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * quotient_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_pow_ad(&mut self, index: usize, scalar: f64, left: AdValue, right: AdValue) {",
        "        let denominator = left.value.powf(right.value);",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let denominator_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = AdValue::pow_derivative(denominator, left.value, right.value, left.node_derivatives[axis], right.node_derivatives[axis]) * denominator_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = AdValue::pow_derivative(denominator, left.value, right.value, left.branch_derivatives[axis], right.branch_derivatives[axis]) * denominator_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_powf_ad(&mut self, index: usize, scalar: f64, value: AdValue, exponent: f64) {",
        "        let denominator = value.value.powf(exponent);",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let denominator_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = AdValue::pow_derivative(denominator, value.value, exponent, value.node_derivatives[axis], 0.0) * denominator_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = AdValue::pow_derivative(denominator, value.value, exponent, value.branch_derivatives[axis], 0.0) * denominator_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_exp_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        let denominator = value.value.exp();",
        "        let quotient = scalar / denominator;",
        "        let derivative_scale = -quotient;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_sin_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        let raw = value.value;",
        "        let denominator = raw.sin();",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let derivative_scale = -quotient * reciprocal * raw.cos();",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_from_scalar_sinh_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        let raw = value.value;",
        "        let denominator = raw.sinh();",
        "        let reciprocal = 1.0 / denominator;",
        "        let quotient = scalar * reciprocal;",
        "        let derivative_scale = -quotient * reciprocal * raw.cosh();",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_rem_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::rem_from_scalar(scalar, value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_pow_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        let exponent = value.value;",
        "        let output = scalar.powf(exponent);",
        "        self.values[index] = output;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = AdValue::pow_derivative(output, scalar, exponent, 0.0, value.node_derivatives[axis]); }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = AdValue::pow_derivative(output, scalar, exponent, 0.0, value.branch_derivatives[axis]); }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_min_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::min_from_scalar(scalar, value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_max_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue) {",
        "        self.store_ad_value(index, AdValue::max_from_scalar(scalar, value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_rem_with_scalar_ad(&mut self, index: usize, value: AdValue, scalar: f64) {",
        "        self.store_ad_value(index, AdValue::rem_with_scalar(value, scalar));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_min_with_scalar_ad(&mut self, index: usize, value: AdValue, scalar: f64) {",
        "        self.store_ad_value(index, AdValue::min_with_scalar(value, scalar));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_max_with_scalar_ad(&mut self, index: usize, value: AdValue, scalar: f64) {",
        "        self.store_ad_value(index, AdValue::max_with_scalar(value, scalar));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_powf_ad(&mut self, index: usize, value: AdValue, exponent: f64) {",
        "        let output = value.value.powf(exponent);",
        "        self.values[index] = output;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = AdValue::pow_derivative(output, value.value, exponent, value.node_derivatives[axis], 0.0); }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = AdValue::pow_derivative(output, value.value, exponent, value.branch_derivatives[axis], 0.0); }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_add(&mut self, index: usize, left: usize, right: usize) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left_value + right_value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] + right_node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] + right_branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sub(&mut self, index: usize, left: usize, right: usize) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left_value - right_value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] - right_node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] - right_branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_mul(&mut self, index: usize, left: usize, right: usize) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left_value * right_value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * right_value + left_value * right_node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * right_value + left_value * right_branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_square(&mut self, index: usize, source: usize) {",
        "        let source_value = self.values[source];",
        "        self.store_unary_scaled(index, source, source_value * source_value, 2.0 * source_value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div(&mut self, index: usize, left: usize, right: usize) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        let reciprocal = 1.0 / right_value;",
        "        let quotient = left_value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * reciprocal + right_node_derivatives[axis] * right_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * reciprocal + right_branch_derivatives[axis] * right_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_add_ad_rhs(&mut self, index: usize, left: usize, right: AdValue) {",
        "        let left_value = self.values[left];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        self.values[index] = left_value + right.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] + right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] + right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_add_ad_lhs(&mut self, index: usize, left: AdValue, right: usize) {",
        "        let right_value = self.values[right];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left.value + right_value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] + right_node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] + right_branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sub_ad_rhs(&mut self, index: usize, left: usize, right: AdValue) {",
        "        let left_value = self.values[left];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        self.values[index] = left_value - right.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] - right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] - right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sub_ad_lhs(&mut self, index: usize, left: AdValue, right: usize) {",
        "        let right_value = self.values[right];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left.value - right_value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] - right_node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] - right_branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_add_ad_rhs(&mut self, index: usize, left: usize, right: AdValue, scale: f64) {",
        "        let left_value = self.values[left];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        self.values[index] = (left_value + right.value) * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] + right.node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] + right.branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_add_ad_lhs(&mut self, index: usize, left: AdValue, right: usize, scale: f64) {",
        "        let right_value = self.values[right];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = (left.value + right_value) * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] + right_node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] + right_branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_sub_ad_rhs(&mut self, index: usize, left: usize, right: AdValue, scale: f64) {",
        "        let left_value = self.values[left];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        self.values[index] = (left_value - right.value) * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] - right.node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] - right.branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_sub_ad_lhs(&mut self, index: usize, left: AdValue, right: usize, scale: f64) {",
        "        let right_value = self.values[right];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = (left.value - right_value) * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] - right_node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] - right_branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_add_scaled_ad_rhs(&mut self, index: usize, left: usize, scale: f64, right: AdValue) {",
        "        let left_value = self.values[left] * scale;",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        self.values[index] = left_value + right.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * scale + right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * scale + right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_add_scaled_ad_lhs(&mut self, index: usize, left: AdValue, right: usize, scale: f64) {",
        "        let right_value = self.values[right] * scale;",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left.value + right_value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] + right_node_derivatives[axis] * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] + right_branch_derivatives[axis] * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sub_scaled_ad_rhs(&mut self, index: usize, left: usize, scale: f64, right: AdValue) {",
        "        let left_value = self.values[left] * scale;",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        self.values[index] = left_value - right.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * scale - right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * scale - right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sub_scaled_ad_lhs(&mut self, index: usize, left: AdValue, right: usize, scale: f64) {",
        "        let right_value = self.values[right] * scale;",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left.value - right_value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] - right_node_derivatives[axis] * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] - right_branch_derivatives[axis] * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_mul_ad_rhs(&mut self, index: usize, left: usize, right: AdValue) {",
        "        let left_value = self.values[left];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        self.values[index] = left_value * right.value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * right.value + left_value * right.node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * right.value + left_value * right.branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
    "    fn store_mul_ad_lhs(&mut self, index: usize, left: AdValue, right: usize) {",
    "        let right_value = self.values[right];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        self.values[index] = left.value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] * right_value + left.value * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] * right_value + left.value * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul3_lhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {",
    "        let left_value = self.values[left];",
    "        let middle_value = self.values[middle];",
    "        let right_value = self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let middle_node_derivatives = self.node_derivatives[middle];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let middle_branch_derivatives = self.branch_derivatives[middle];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        let product_value = left_value * middle_value;",
    "        self.values[index] = product_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { let product_derivative = left_node_derivatives[axis] * middle_value + left_value * middle_node_derivatives[axis]; self.node_derivatives[index][axis] = product_derivative * right_value + product_value * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let product_derivative = left_branch_derivatives[axis] * middle_value + left_value * middle_branch_derivatives[axis]; self.branch_derivatives[index][axis] = product_derivative * right_value + product_value * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul3_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {",
    "        let left_value = self.values[left];",
    "        let middle_value = self.values[middle];",
    "        let right_value = self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let middle_node_derivatives = self.node_derivatives[middle];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let middle_branch_derivatives = self.branch_derivatives[middle];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        let product_value = middle_value * right_value;",
    "        self.values[index] = left_value * product_value;",
    "        for axis in 0..Instance::NODE_COUNT { let product_derivative = middle_node_derivatives[axis] * right_value + middle_value * right_node_derivatives[axis]; self.node_derivatives[index][axis] = left_node_derivatives[axis] * product_value + left_value * product_derivative; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let product_derivative = middle_branch_derivatives[axis] * right_value + middle_value * right_branch_derivatives[axis]; self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * product_value + left_value * product_derivative; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_ad_product_lhs(&mut self, index: usize, left: AdValue, right: AdValue, source: usize) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let product_value = left.value * right.value;",
    "        self.values[index] = product_value * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { let product_derivative = left.node_derivatives[axis] * right.value + left.value * right.node_derivatives[axis]; self.node_derivatives[index][axis] = product_derivative * source_value + product_value * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let product_derivative = left.branch_derivatives[axis] * right.value + left.value * right.branch_derivatives[axis]; self.branch_derivatives[index][axis] = product_derivative * source_value + product_value * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_ad_product_rhs(&mut self, index: usize, source: usize, left: AdValue, right: AdValue) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let product_value = left.value * right.value;",
    "        self.values[index] = source_value * product_value;",
    "        for axis in 0..Instance::NODE_COUNT { let product_derivative = left.node_derivatives[axis] * right.value + left.value * right.node_derivatives[axis]; self.node_derivatives[index][axis] = source_node_derivatives[axis] * product_value + source_value * product_derivative; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let product_derivative = left.branch_derivatives[axis] * right.value + left.value * right.branch_derivatives[axis]; self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * product_value + source_value * product_derivative; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul3_affine_lhs(&mut self, index: usize, left: usize, middle: usize, scale: f64, offset: f64, right: usize) {",
    "        let left_value = self.values[left];",
    "        let middle_value = self.values[middle];",
    "        let right_value = self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let middle_node_derivatives = self.node_derivatives[middle];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let middle_branch_derivatives = self.branch_derivatives[middle];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        let product_value = left_value * middle_value;",
    "        let affine_value = product_value * scale + offset;",
    "        self.values[index] = affine_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { let product_derivative = left_node_derivatives[axis] * middle_value + left_value * middle_node_derivatives[axis]; let affine_derivative = product_derivative * scale; self.node_derivatives[index][axis] = affine_derivative * right_value + affine_value * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let product_derivative = left_branch_derivatives[axis] * middle_value + left_value * middle_branch_derivatives[axis]; let affine_derivative = product_derivative * scale; self.branch_derivatives[index][axis] = affine_derivative * right_value + affine_value * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul3_affine_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize, scale: f64, offset: f64) {",
    "        let left_value = self.values[left];",
    "        let middle_value = self.values[middle];",
    "        let right_value = self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let middle_node_derivatives = self.node_derivatives[middle];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let middle_branch_derivatives = self.branch_derivatives[middle];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        let product_value = middle_value * right_value;",
    "        let affine_value = product_value * scale + offset;",
    "        self.values[index] = left_value * affine_value;",
    "        for axis in 0..Instance::NODE_COUNT { let product_derivative = middle_node_derivatives[axis] * right_value + middle_value * right_node_derivatives[axis]; let affine_derivative = product_derivative * scale; self.node_derivatives[index][axis] = left_node_derivatives[axis] * affine_value + left_value * affine_derivative; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let product_derivative = middle_branch_derivatives[axis] * right_value + middle_value * right_branch_derivatives[axis]; let affine_derivative = product_derivative * scale; self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * affine_value + left_value * affine_derivative; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_ad_affine_product_lhs(&mut self, index: usize, left: AdValue, right: AdValue, scale: f64, offset: f64, source: usize) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let product_value = left.value * right.value;",
    "        let affine_value = product_value * scale + offset;",
    "        self.values[index] = affine_value * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { let product_derivative = left.node_derivatives[axis] * right.value + left.value * right.node_derivatives[axis]; let affine_derivative = product_derivative * scale; self.node_derivatives[index][axis] = affine_derivative * source_value + affine_value * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let product_derivative = left.branch_derivatives[axis] * right.value + left.value * right.branch_derivatives[axis]; let affine_derivative = product_derivative * scale; self.branch_derivatives[index][axis] = affine_derivative * source_value + affine_value * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_ad_affine_product_rhs(&mut self, index: usize, source: usize, left: AdValue, right: AdValue, scale: f64, offset: f64) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let product_value = left.value * right.value;",
    "        let affine_value = product_value * scale + offset;",
    "        self.values[index] = source_value * affine_value;",
    "        for axis in 0..Instance::NODE_COUNT { let product_derivative = left.node_derivatives[axis] * right.value + left.value * right.node_derivatives[axis]; let affine_derivative = product_derivative * scale; self.node_derivatives[index][axis] = source_node_derivatives[axis] * affine_value + source_value * affine_derivative; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let product_derivative = left.branch_derivatives[axis] * right.value + left.value * right.branch_derivatives[axis]; let affine_derivative = product_derivative * scale; self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * affine_value + source_value * affine_derivative; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_add_lhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {",
    "        let left_value = self.values[left];",
    "        let middle_value = self.values[middle];",
    "        let right_value = self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let middle_node_derivatives = self.node_derivatives[middle];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let middle_branch_derivatives = self.branch_derivatives[middle];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        let sum = left_value + middle_value;",
    "        self.values[index] = sum * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] + middle_node_derivatives[axis]) * right_value + sum * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] + middle_branch_derivatives[axis]) * right_value + sum * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_add_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {",
    "        let left_value = self.values[left];",
    "        let middle_value = self.values[middle];",
    "        let right_value = self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let middle_node_derivatives = self.node_derivatives[middle];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let middle_branch_derivatives = self.branch_derivatives[middle];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        let sum = middle_value + right_value;",
    "        self.values[index] = left_value * sum;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * sum + left_value * (middle_node_derivatives[axis] + right_node_derivatives[axis]); }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * sum + left_value * (middle_branch_derivatives[axis] + right_branch_derivatives[axis]); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sub_lhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {",
    "        let left_value = self.values[left];",
    "        let middle_value = self.values[middle];",
    "        let right_value = self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let middle_node_derivatives = self.node_derivatives[middle];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let middle_branch_derivatives = self.branch_derivatives[middle];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        let difference = left_value - middle_value;",
    "        self.values[index] = difference * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] - middle_node_derivatives[axis]) * right_value + difference * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] - middle_branch_derivatives[axis]) * right_value + difference * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sub_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {",
    "        let left_value = self.values[left];",
    "        let middle_value = self.values[middle];",
    "        let right_value = self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let middle_node_derivatives = self.node_derivatives[middle];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let middle_branch_derivatives = self.branch_derivatives[middle];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        let difference = middle_value - right_value;",
    "        self.values[index] = left_value * difference;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * difference + left_value * (middle_node_derivatives[axis] - right_node_derivatives[axis]); }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * difference + left_value * (middle_branch_derivatives[axis] - right_branch_derivatives[axis]); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_add_ad_lhs(&mut self, index: usize, left: AdValue, right: AdValue, source: usize) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let sum = left.value + right.value;",
    "        self.values[index] = sum * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] + right.node_derivatives[axis]) * source_value + sum * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] + right.branch_derivatives[axis]) * source_value + sum * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_add_ad_rhs(&mut self, index: usize, source: usize, left: AdValue, right: AdValue) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let sum = left.value + right.value;",
    "        self.values[index] = source_value * sum;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = source_node_derivatives[axis] * sum + source_value * (left.node_derivatives[axis] + right.node_derivatives[axis]); }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * sum + source_value * (left.branch_derivatives[axis] + right.branch_derivatives[axis]); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sub_ad_lhs(&mut self, index: usize, left: AdValue, right: AdValue, source: usize) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let difference = left.value - right.value;",
    "        self.values[index] = difference * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] - right.node_derivatives[axis]) * source_value + difference * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] - right.branch_derivatives[axis]) * source_value + difference * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sub_ad_rhs(&mut self, index: usize, source: usize, left: AdValue, right: AdValue) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let difference = left.value - right.value;",
    "        self.values[index] = source_value * difference;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = source_node_derivatives[axis] * difference + source_value * (left.node_derivatives[axis] - right.node_derivatives[axis]); }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * difference + source_value * (left.branch_derivatives[axis] - right.branch_derivatives[axis]); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sub_from_scalar_lhs(&mut self, index: usize, scalar: f64, value: usize, source: usize) {",
    "        let left_value = scalar - self.values[value];",
    "        let source_value = self.values[source];",
    "        let value_node_derivatives = self.node_derivatives[value];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let value_branch_derivatives = self.branch_derivatives[value];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = left_value * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = -value_node_derivatives[axis] * source_value + left_value * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = -value_branch_derivatives[axis] * source_value + left_value * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sub_from_scalar_rhs(&mut self, index: usize, source: usize, scalar: f64, value: usize) {",
    "        let source_value = self.values[source];",
    "        let right_value = scalar - self.values[value];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let value_node_derivatives = self.node_derivatives[value];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let value_branch_derivatives = self.branch_derivatives[value];",
    "        self.values[index] = source_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = source_node_derivatives[axis] * right_value - source_value * value_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * right_value - source_value * value_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sub_from_scalar_ad_lhs(&mut self, index: usize, scalar: f64, value: AdValue, source: usize) {",
    "        let left_value = scalar - value.value;",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = left_value * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = -value.node_derivatives[axis] * source_value + left_value * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = -value.branch_derivatives[axis] * source_value + left_value * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sub_from_scalar_ad_rhs(&mut self, index: usize, source: usize, scalar: f64, value: AdValue) {",
    "        let source_value = self.values[source];",
    "        let right_value = scalar - value.value;",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = source_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = source_node_derivatives[axis] * right_value - source_value * value.node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * right_value - source_value * value.branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_div_from_scalar_lhs(&mut self, index: usize, scalar: f64, value: usize, source: usize) {",
    "        let source_value = self.values[source];",
    "        let denominator = self.values[value];",
    "        let reciprocal = 1.0 / denominator;",
    "        let quotient = scalar * reciprocal;",
    "        let derivative_scale = -quotient * reciprocal;",
    "        let value_node_derivatives = self.node_derivatives[value];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let value_branch_derivatives = self.branch_derivatives[value];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = quotient * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value_node_derivatives[axis] * derivative_scale * source_value + quotient * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value_branch_derivatives[axis] * derivative_scale * source_value + quotient * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_div_from_scalar_rhs(&mut self, index: usize, source: usize, scalar: f64, value: usize) {",
    "        let source_value = self.values[source];",
    "        let denominator = self.values[value];",
    "        let reciprocal = 1.0 / denominator;",
    "        let quotient = scalar * reciprocal;",
    "        let derivative_scale = -quotient * reciprocal;",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let value_node_derivatives = self.node_derivatives[value];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let value_branch_derivatives = self.branch_derivatives[value];",
    "        self.values[index] = source_value * quotient;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = source_node_derivatives[axis] * quotient + source_value * value_node_derivatives[axis] * derivative_scale; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * quotient + source_value * value_branch_derivatives[axis] * derivative_scale; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_div_from_scalar_ad_lhs(&mut self, index: usize, scalar: f64, value: AdValue, source: usize) {",
    "        let source_value = self.values[source];",
    "        let reciprocal = 1.0 / value.value;",
    "        let quotient = scalar * reciprocal;",
    "        let derivative_scale = -quotient * reciprocal;",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = quotient * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale * source_value + quotient * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale * source_value + quotient * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_div_from_scalar_ad_rhs(&mut self, index: usize, source: usize, scalar: f64, value: AdValue) {",
    "        let source_value = self.values[source];",
    "        let reciprocal = 1.0 / value.value;",
    "        let quotient = scalar * reciprocal;",
    "        let derivative_scale = -quotient * reciprocal;",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = source_value * quotient;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = source_node_derivatives[axis] * quotient + source_value * value.node_derivatives[axis] * derivative_scale; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * quotient + source_value * value.branch_derivatives[axis] * derivative_scale; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_pow_ad_lhs(&mut self, index: usize, left: AdValue, right: AdValue, source: usize) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let base = left.value;",
    "        let exponent = right.value;",
    "        let output = base.powf(exponent);",
    "        self.values[index] = output * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { let derivative = AdValue::pow_derivative(output, base, exponent, left.node_derivatives[axis], right.node_derivatives[axis]); self.node_derivatives[index][axis] = derivative * source_value + output * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let derivative = AdValue::pow_derivative(output, base, exponent, left.branch_derivatives[axis], right.branch_derivatives[axis]); self.branch_derivatives[index][axis] = derivative * source_value + output * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_pow_ad_rhs(&mut self, index: usize, source: usize, left: AdValue, right: AdValue) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let base = left.value;",
    "        let exponent = right.value;",
    "        let output = base.powf(exponent);",
    "        self.values[index] = source_value * output;",
    "        for axis in 0..Instance::NODE_COUNT { let derivative = AdValue::pow_derivative(output, base, exponent, left.node_derivatives[axis], right.node_derivatives[axis]); self.node_derivatives[index][axis] = source_node_derivatives[axis] * output + source_value * derivative; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let derivative = AdValue::pow_derivative(output, base, exponent, left.branch_derivatives[axis], right.branch_derivatives[axis]); self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * output + source_value * derivative; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_powf_ad_lhs(&mut self, index: usize, value: AdValue, exponent: f64, source: usize) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let base = value.value;",
    "        let output = base.powf(exponent);",
    "        self.values[index] = output * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { let derivative = AdValue::pow_derivative(output, base, exponent, value.node_derivatives[axis], 0.0); self.node_derivatives[index][axis] = derivative * source_value + output * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let derivative = AdValue::pow_derivative(output, base, exponent, value.branch_derivatives[axis], 0.0); self.branch_derivatives[index][axis] = derivative * source_value + output * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_powf_ad_rhs(&mut self, index: usize, source: usize, value: AdValue, exponent: f64) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let base = value.value;",
    "        let output = base.powf(exponent);",
    "        self.values[index] = source_value * output;",
    "        for axis in 0..Instance::NODE_COUNT { let derivative = AdValue::pow_derivative(output, base, exponent, value.node_derivatives[axis], 0.0); self.node_derivatives[index][axis] = source_node_derivatives[axis] * output + source_value * derivative; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let derivative = AdValue::pow_derivative(output, base, exponent, value.branch_derivatives[axis], 0.0); self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * output + source_value * derivative; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_neg_lhs(&mut self, index: usize, left: usize, right: usize) {",
    "        let left_value = -self.values[left];",
    "        let right_value = self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        self.values[index] = left_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = -left_node_derivatives[axis] * right_value + left_value * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = -left_branch_derivatives[axis] * right_value + left_value * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_neg_rhs(&mut self, index: usize, left: usize, right: usize) {",
    "        let left_value = self.values[left];",
    "        let right_value = -self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        self.values[index] = left_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * right_value - left_value * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * right_value - left_value * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_neg_ad_lhs(&mut self, index: usize, left: AdValue, right: usize) {",
    "        let left_value = -left.value;",
    "        let right_value = self.values[right];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        self.values[index] = left_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = -left.node_derivatives[axis] * right_value + left_value * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = -left.branch_derivatives[axis] * right_value + left_value * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_neg_ad_rhs(&mut self, index: usize, left: usize, right: AdValue) {",
    "        let left_value = self.values[left];",
    "        let right_value = -right.value;",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        self.values[index] = left_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * right_value - left_value * right.node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * right_value - left_value * right.branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_div_lhs(&mut self, index: usize, numerator: usize, denominator: usize, source: usize) {",
    "        let numerator_value = self.values[numerator];",
    "        let denominator_value = self.values[denominator];",
    "        let source_value = self.values[source];",
    "        let numerator_node_derivatives = self.node_derivatives[numerator];",
    "        let denominator_node_derivatives = self.node_derivatives[denominator];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let numerator_branch_derivatives = self.branch_derivatives[numerator];",
    "        let denominator_branch_derivatives = self.branch_derivatives[denominator];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let reciprocal = 1.0 / denominator_value;",
    "        let quotient = numerator_value * reciprocal;",
    "        let denominator_scale = -quotient * reciprocal;",
    "        self.values[index] = quotient * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { let quotient_derivative = numerator_node_derivatives[axis] * reciprocal + denominator_node_derivatives[axis] * denominator_scale; self.node_derivatives[index][axis] = quotient_derivative * source_value + quotient * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let quotient_derivative = numerator_branch_derivatives[axis] * reciprocal + denominator_branch_derivatives[axis] * denominator_scale; self.branch_derivatives[index][axis] = quotient_derivative * source_value + quotient * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_div_rhs(&mut self, index: usize, source: usize, numerator: usize, denominator: usize) {",
    "        let source_value = self.values[source];",
    "        let numerator_value = self.values[numerator];",
    "        let denominator_value = self.values[denominator];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let numerator_node_derivatives = self.node_derivatives[numerator];",
    "        let denominator_node_derivatives = self.node_derivatives[denominator];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let numerator_branch_derivatives = self.branch_derivatives[numerator];",
    "        let denominator_branch_derivatives = self.branch_derivatives[denominator];",
    "        let reciprocal = 1.0 / denominator_value;",
    "        let quotient = numerator_value * reciprocal;",
    "        let denominator_scale = -quotient * reciprocal;",
    "        self.values[index] = source_value * quotient;",
    "        for axis in 0..Instance::NODE_COUNT { let quotient_derivative = numerator_node_derivatives[axis] * reciprocal + denominator_node_derivatives[axis] * denominator_scale; self.node_derivatives[index][axis] = source_node_derivatives[axis] * quotient + source_value * quotient_derivative; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let quotient_derivative = numerator_branch_derivatives[axis] * reciprocal + denominator_branch_derivatives[axis] * denominator_scale; self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * quotient + source_value * quotient_derivative; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_div_ad_lhs(&mut self, index: usize, left: AdValue, right: AdValue, source: usize) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let reciprocal = 1.0 / right.value;",
    "        let quotient = left.value * reciprocal;",
    "        let denominator_scale = -quotient * reciprocal;",
    "        self.values[index] = quotient * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { let quotient_derivative = left.node_derivatives[axis] * reciprocal + right.node_derivatives[axis] * denominator_scale; self.node_derivatives[index][axis] = quotient_derivative * source_value + quotient * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let quotient_derivative = left.branch_derivatives[axis] * reciprocal + right.branch_derivatives[axis] * denominator_scale; self.branch_derivatives[index][axis] = quotient_derivative * source_value + quotient * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_div_ad_rhs(&mut self, index: usize, source: usize, left: AdValue, right: AdValue) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let reciprocal = 1.0 / right.value;",
    "        let quotient = left.value * reciprocal;",
    "        let denominator_scale = -quotient * reciprocal;",
    "        self.values[index] = source_value * quotient;",
    "        for axis in 0..Instance::NODE_COUNT { let quotient_derivative = left.node_derivatives[axis] * reciprocal + right.node_derivatives[axis] * denominator_scale; self.node_derivatives[index][axis] = source_node_derivatives[axis] * quotient + source_value * quotient_derivative; }",
    "        for axis in 0..Instance::BRANCH_COUNT { let quotient_derivative = left.branch_derivatives[axis] * reciprocal + right.branch_derivatives[axis] * denominator_scale; self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * quotient + source_value * quotient_derivative; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_square_lhs(&mut self, index: usize, value: usize, source: usize) {",
    "        let raw = self.values[value];",
    "        let square = raw * raw;",
    "        let source_value = self.values[source];",
    "        let value_node_derivatives = self.node_derivatives[value];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let value_branch_derivatives = self.branch_derivatives[value];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let derivative_scale = 2.0 * raw;",
    "        self.values[index] = square * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value_node_derivatives[axis] * derivative_scale * source_value + square * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value_branch_derivatives[axis] * derivative_scale * source_value + square * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_square_rhs(&mut self, index: usize, source: usize, value: usize) {",
    "        let source_value = self.values[source];",
    "        let raw = self.values[value];",
    "        let square = raw * raw;",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let value_node_derivatives = self.node_derivatives[value];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let value_branch_derivatives = self.branch_derivatives[value];",
    "        let derivative_scale = 2.0 * raw;",
    "        self.values[index] = source_value * square;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = source_node_derivatives[axis] * square + source_value * value_node_derivatives[axis] * derivative_scale; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * square + source_value * value_branch_derivatives[axis] * derivative_scale; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_unary_lhs(&mut self, index: usize, value_source: usize, source: usize, unary_value: f64, derivative_scale: f64) {",
    "        let source_value = self.values[source];",
    "        let value_node_derivatives = self.node_derivatives[value_source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let value_branch_derivatives = self.branch_derivatives[value_source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = unary_value * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value_node_derivatives[axis] * derivative_scale * source_value + unary_value * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value_branch_derivatives[axis] * derivative_scale * source_value + unary_value * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_unary_rhs(&mut self, index: usize, source: usize, value_source: usize, unary_value: f64, derivative_scale: f64) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let value_node_derivatives = self.node_derivatives[value_source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        let value_branch_derivatives = self.branch_derivatives[value_source];",
    "        self.values[index] = source_value * unary_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = source_node_derivatives[axis] * unary_value + source_value * value_node_derivatives[axis] * derivative_scale; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * unary_value + source_value * value_branch_derivatives[axis] * derivative_scale; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_unary_ad_lhs(&mut self, index: usize, value: AdValue, source: usize, unary_value: f64, derivative_scale: f64) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = unary_value * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * derivative_scale * source_value + unary_value * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * derivative_scale * source_value + unary_value * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_unary_ad_rhs(&mut self, index: usize, source: usize, value: AdValue, unary_value: f64, derivative_scale: f64) {",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = source_value * unary_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = source_node_derivatives[axis] * unary_value + source_value * value.node_derivatives[axis] * derivative_scale; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * unary_value + source_value * value.branch_derivatives[axis] * derivative_scale; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_exp_lhs(&mut self, index: usize, value_source: usize, source: usize) {",
    "        let unary_value = self.values[value_source].exp();",
    "        self.store_mul_unary_lhs(index, value_source, source, unary_value, unary_value);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_exp_rhs(&mut self, index: usize, source: usize, value_source: usize) {",
    "        let unary_value = self.values[value_source].exp();",
    "        self.store_mul_unary_rhs(index, source, value_source, unary_value, unary_value);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_exp_ad_lhs(&mut self, index: usize, value: AdValue, source: usize) {",
    "        let unary_value = value.value.exp();",
    "        self.store_mul_unary_ad_lhs(index, value, source, unary_value, unary_value);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_exp_ad_rhs(&mut self, index: usize, source: usize, value: AdValue) {",
    "        let unary_value = value.value.exp();",
    "        self.store_mul_unary_ad_rhs(index, source, value, unary_value, unary_value);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_ln_lhs(&mut self, index: usize, value_source: usize, source: usize) {",
    "        let raw = self.values[value_source];",
    "        self.store_mul_unary_lhs(index, value_source, source, raw.ln(), 1.0 / raw);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_ln_rhs(&mut self, index: usize, source: usize, value_source: usize) {",
    "        let raw = self.values[value_source];",
    "        self.store_mul_unary_rhs(index, source, value_source, raw.ln(), 1.0 / raw);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_ln_ad_lhs(&mut self, index: usize, value: AdValue, source: usize) {",
    "        let raw = value.value;",
    "        self.store_mul_unary_ad_lhs(index, value, source, raw.ln(), 1.0 / raw);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_ln_ad_rhs(&mut self, index: usize, source: usize, value: AdValue) {",
    "        let raw = value.value;",
    "        self.store_mul_unary_ad_rhs(index, source, value, raw.ln(), 1.0 / raw);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sqrt_lhs(&mut self, index: usize, value_source: usize, source: usize) {",
    "        let unary_value = self.values[value_source].sqrt();",
    "        self.store_mul_unary_lhs(index, value_source, source, unary_value, 1.0 / (2.0 * unary_value));",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sqrt_rhs(&mut self, index: usize, source: usize, value_source: usize) {",
    "        let unary_value = self.values[value_source].sqrt();",
    "        self.store_mul_unary_rhs(index, source, value_source, unary_value, 1.0 / (2.0 * unary_value));",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sqrt_ad_lhs(&mut self, index: usize, value: AdValue, source: usize) {",
    "        let unary_value = value.value.sqrt();",
    "        self.store_mul_unary_ad_lhs(index, value, source, unary_value, 1.0 / (2.0 * unary_value));",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_sqrt_ad_rhs(&mut self, index: usize, source: usize, value: AdValue) {",
    "        let unary_value = value.value.sqrt();",
    "        self.store_mul_unary_ad_rhs(index, source, value, unary_value, 1.0 / (2.0 * unary_value));",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_abs_lhs(&mut self, index: usize, value_source: usize, source: usize) {",
    "        let raw = self.values[value_source];",
    "        self.store_mul_unary_lhs(index, value_source, source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_abs_rhs(&mut self, index: usize, source: usize, value_source: usize) {",
    "        let raw = self.values[value_source];",
    "        self.store_mul_unary_rhs(index, source, value_source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_abs_ad_lhs(&mut self, index: usize, value: AdValue, source: usize) {",
    "        let raw = value.value;",
    "        self.store_mul_unary_ad_lhs(index, value, source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_abs_ad_rhs(&mut self, index: usize, source: usize, value: AdValue) {",
    "        let raw = value.value;",
    "        self.store_mul_unary_ad_rhs(index, source, value, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_cos_lhs(&mut self, index: usize, value_source: usize, source: usize) {",
    "        let raw = self.values[value_source];",
    "        self.store_mul_unary_lhs(index, value_source, source, raw.cos(), -raw.sin());",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_cos_rhs(&mut self, index: usize, source: usize, value_source: usize) {",
    "        let raw = self.values[value_source];",
    "        self.store_mul_unary_rhs(index, source, value_source, raw.cos(), -raw.sin());",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_cos_ad_lhs(&mut self, index: usize, value: AdValue, source: usize) {",
    "        let raw = value.value;",
    "        self.store_mul_unary_ad_lhs(index, value, source, raw.cos(), -raw.sin());",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_cos_ad_rhs(&mut self, index: usize, source: usize, value: AdValue) {",
    "        let raw = value.value;",
    "        self.store_mul_unary_ad_rhs(index, source, value, raw.cos(), -raw.sin());",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_tanh_lhs(&mut self, index: usize, value_source: usize, source: usize) {",
    "        let raw = self.values[value_source];",
    "        let cosh = raw.cosh();",
    "        self.store_mul_unary_lhs(index, value_source, source, raw.tanh(), 1.0 / (cosh * cosh));",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_tanh_rhs(&mut self, index: usize, source: usize, value_source: usize) {",
    "        let raw = self.values[value_source];",
    "        let cosh = raw.cosh();",
    "        self.store_mul_unary_rhs(index, source, value_source, raw.tanh(), 1.0 / (cosh * cosh));",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_tanh_ad_lhs(&mut self, index: usize, value: AdValue, source: usize) {",
    "        let raw = value.value;",
    "        let cosh = raw.cosh();",
    "        self.store_mul_unary_ad_lhs(index, value, source, raw.tanh(), 1.0 / (cosh * cosh));",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_tanh_ad_rhs(&mut self, index: usize, source: usize, value: AdValue) {",
    "        let raw = value.value;",
    "        let cosh = raw.cosh();",
    "        self.store_mul_unary_ad_rhs(index, source, value, raw.tanh(), 1.0 / (cosh * cosh));",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_limexp_lhs(&mut self, index: usize, value_source: usize, source: usize) {",
    "        let raw = self.values[value_source];",
    "        if raw < 80.0 { let value = raw.exp(); self.store_mul_unary_lhs(index, value_source, source, value, value); } else { self.store_mul_unary_lhs(index, value_source, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_limexp_rhs(&mut self, index: usize, source: usize, value_source: usize) {",
    "        let raw = self.values[value_source];",
    "        if raw < 80.0 { let value = raw.exp(); self.store_mul_unary_rhs(index, source, value_source, value, value); } else { self.store_mul_unary_rhs(index, source, value_source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_limexp_ad_lhs(&mut self, index: usize, value: AdValue, source: usize) {",
    "        let raw = value.value;",
    "        if raw < 80.0 { let output = raw.exp(); self.store_mul_unary_ad_lhs(index, value, source, output, output); } else { self.store_mul_unary_ad_lhs(index, value, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_limexp_ad_rhs(&mut self, index: usize, source: usize, value: AdValue) {",
    "        let raw = value.value;",
    "        if raw < 80.0 { let output = raw.exp(); self.store_mul_unary_ad_rhs(index, source, value, output, output); } else { self.store_mul_unary_ad_rhs(index, source, value, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_limited_exp_lhs(&mut self, index: usize, value_source: usize, source: usize) {",
    "        let raw = self.values[value_source];",
    "        if raw > 80.0 { self.store_mul_unary_lhs(index, value_source, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_lhs(index, value_source, source, 1.804851387e-35, 0.0); } else { let value = raw.exp(); self.store_mul_unary_lhs(index, value_source, source, value, value); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_limited_exp_rhs(&mut self, index: usize, source: usize, value_source: usize) {",
    "        let raw = self.values[value_source];",
    "        if raw > 80.0 { self.store_mul_unary_rhs(index, source, value_source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_rhs(index, source, value_source, 1.804851387e-35, 0.0); } else { let value = raw.exp(); self.store_mul_unary_rhs(index, source, value_source, value, value); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_limited_exp_ad_lhs(&mut self, index: usize, value: AdValue, source: usize) {",
    "        let raw = value.value;",
    "        if raw > 80.0 { self.store_mul_unary_ad_lhs(index, value, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_ad_lhs(index, value, source, 1.804851387e-35, 0.0); } else { let output = raw.exp(); self.store_mul_unary_ad_lhs(index, value, source, output, output); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_limited_exp_ad_rhs(&mut self, index: usize, source: usize, value: AdValue) {",
    "        let raw = value.value;",
    "        if raw > 80.0 { self.store_mul_unary_ad_rhs(index, source, value, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_ad_rhs(index, source, value, 1.804851387e-35, 0.0); } else { let output = raw.exp(); self.store_mul_unary_ad_rhs(index, source, value, output, output); }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_offset_lhs(&mut self, index: usize, left: usize, offset: f64, right: usize) {",
    "        let left_value = self.values[left] + offset;",
    "        let right_value = self.values[right];",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        self.values[index] = left_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * right_value + left_value * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * right_value + left_value * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_offset_rhs(&mut self, index: usize, left: usize, right: usize, offset: f64) {",
    "        let left_value = self.values[left];",
    "        let right_value = self.values[right] + offset;",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        self.values[index] = left_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * right_value + left_value * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * right_value + left_value * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_offset_ad_lhs(&mut self, index: usize, left: AdValue, offset: f64, right: usize) {",
    "        let left_value = left.value + offset;",
    "        let right_value = self.values[right];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        self.values[index] = left_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] * right_value + left_value * right_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] * right_value + left_value * right_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_offset_ad_rhs(&mut self, index: usize, left: usize, right: AdValue, offset: f64) {",
    "        let left_value = self.values[left];",
    "        let right_value = right.value + offset;",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        self.values[index] = left_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * right_value + left_value * right.node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * right_value + left_value * right.branch_derivatives[axis]; }",
    "    }",
    "",
    "    fn store_mul_scale_ad_lhs(&mut self, index: usize, value: AdValue, scale: f64, source: usize) {",
    "        let left_value = value.value * scale;",
    "        let source_value = self.values[source];",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = left_value * source_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = value.node_derivatives[axis] * scale * source_value + left_value * source_node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = value.branch_derivatives[axis] * scale * source_value + left_value * source_branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_scale_ad_rhs(&mut self, index: usize, source: usize, value: AdValue, scale: f64) {",
    "        let source_value = self.values[source];",
    "        let right_value = value.value * scale;",
    "        let source_node_derivatives = self.node_derivatives[source];",
    "        let source_branch_derivatives = self.branch_derivatives[source];",
    "        self.values[index] = source_value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = source_node_derivatives[axis] * right_value + source_value * value.node_derivatives[axis] * scale; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = source_branch_derivatives[axis] * right_value + source_value * value.branch_derivatives[axis] * scale; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_scaled_ad_rhs(&mut self, index: usize, left: usize, scale: f64, right: AdValue) {",
    "        let left_value = self.values[left] * scale;",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        self.values[index] = left_value * right.value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * scale * right.value + left_value * right.node_derivatives[axis]; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * scale * right.value + left_value * right.branch_derivatives[axis]; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_mul_scaled_ad_lhs(&mut self, index: usize, left: AdValue, right: usize, scale: f64) {",
    "        let right_value = self.values[right] * scale;",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        self.values[index] = left.value * right_value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] * right_value + left.value * right_node_derivatives[axis] * scale; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] * right_value + left.value * right_branch_derivatives[axis] * scale; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_div_ad_rhs(&mut self, index: usize, left: usize, right: AdValue) {",
        "        let left_value = self.values[left];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let reciprocal = 1.0 / right.value;",
        "        let quotient = left_value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * reciprocal + right.node_derivatives[axis] * right_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * reciprocal + right.branch_derivatives[axis] * right_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_div_ad_lhs(&mut self, index: usize, left: AdValue, right: usize) {",
        "        let right_value = self.values[right];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        let reciprocal = 1.0 / right_value;",
        "        let quotient = left.value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left.node_derivatives[axis] * reciprocal + right_node_derivatives[axis] * right_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left.branch_derivatives[axis] * reciprocal + right_branch_derivatives[axis] * right_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_div_ad_rhs(&mut self, index: usize, left: usize, right: AdValue, scale: f64) {",
        "        let left_value = self.values[left];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let reciprocal = 1.0 / right.value;",
        "        let quotient = left_value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] * reciprocal + right.node_derivatives[axis] * right_scale) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] * reciprocal + right.branch_derivatives[axis] * right_scale) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_div_ad_lhs(&mut self, index: usize, left: AdValue, right: usize, scale: f64) {",
        "        let right_value = self.values[right];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        let reciprocal = 1.0 / right_value;",
        "        let quotient = left.value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left.node_derivatives[axis] * reciprocal + right_node_derivatives[axis] * right_scale) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left.branch_derivatives[axis] * reciprocal + right_branch_derivatives[axis] * right_scale) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_add(&mut self, index: usize, left: usize, right: usize, scale: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = (left_value + right_value) * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] + right_node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] + right_branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_sub(&mut self, index: usize, left: usize, right: usize, scale: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = (left_value - right_value) * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] - right_node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] - right_branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_add_scaled_inputs(&mut self, index: usize, left: usize, left_scale: f64, right: usize, right_scale: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left_value * left_scale + right_value * right_scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * left_scale + right_node_derivatives[axis] * right_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * left_scale + right_branch_derivatives[axis] * right_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sub_scaled_inputs(&mut self, index: usize, left: usize, left_scale: f64, right: usize, right_scale: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left_value * left_scale - right_value * right_scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * left_scale - right_node_derivatives[axis] * right_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * left_scale - right_branch_derivatives[axis] * right_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_mul(&mut self, index: usize, left: usize, right: usize, scale: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left_value * right_value * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] * right_value + left_value * right_node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] * right_value + left_value * right_branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_div(&mut self, index: usize, left: usize, right: usize, scale: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        let reciprocal = 1.0 / right_value;",
        "        let quotient = left_value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient * scale;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] * reciprocal + right_node_derivatives[axis] * right_scale) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] * reciprocal + right_branch_derivatives[axis] * right_scale) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_add(&mut self, index: usize, left: usize, right: usize, offset: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left_value + right_value + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] + right_node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] + right_branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_sub(&mut self, index: usize, left: usize, right: usize, offset: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left_value - right_value + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] - right_node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] - right_branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_mul(&mut self, index: usize, left: usize, right: usize, offset: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left_value * right_value + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * right_value + left_value * right_node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * right_value + left_value * right_branch_derivatives[axis]; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_div(&mut self, index: usize, left: usize, right: usize, offset: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        let reciprocal = 1.0 / right_value;",
        "        let quotient = left_value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * reciprocal + right_node_derivatives[axis] * right_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * reciprocal + right_branch_derivatives[axis] * right_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_scaled_add(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = (left_value + right_value) * scale + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] + right_node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] + right_branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_scaled_sub(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = (left_value - right_value) * scale + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] - right_node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] - right_branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_scaled_mul(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = left_value * right_value * scale + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] * right_value + left_value * right_node_derivatives[axis]) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] * right_value + left_value * right_branch_derivatives[axis]) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_scaled_div(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        let reciprocal = 1.0 / right_value;",
        "        let quotient = left_value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = quotient * scale + offset;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] * reciprocal + right_node_derivatives[axis] * right_scale) * scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] * reciprocal + right_branch_derivatives[axis] * right_scale) * scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scale(&mut self, index: usize, source: usize, scale: f64) {",
        "        self.store_unary_scaled(index, source, self.values[source] * scale, scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset(&mut self, index: usize, source: usize, offset: f64) {",
        "        self.values[index] = self.values[source] + offset;",
        "        self.node_derivatives[index] = self.node_derivatives[source];",
        "        self.branch_derivatives[index] = self.branch_derivatives[source];",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_scaled(&mut self, index: usize, source: usize, scale: f64, offset: f64) {",
        "        self.store_unary_scaled(index, source, self.values[source] * scale + offset, scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_offset(&mut self, index: usize, source: usize, offset: f64, scale: f64) {",
        "        self.store_unary_scaled(index, source, (self.values[source] + offset) * scale, scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_neg(&mut self, index: usize, source: usize) {",
        "        self.store_scale(index, source, -1.0);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_abs(&mut self, index: usize, source: usize) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });",
        "    }",
        "",
        "    #[inline]",
        "    fn store_min_with_scalar(&mut self, index: usize, source: usize, scalar: f64) {",
        "        if self.values[source] <= scalar {",
        "            self.copy_ad(index, source);",
        "        } else {",
        "            self.store_scalar(index, scalar);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_max_with_scalar(&mut self, index: usize, source: usize, scalar: f64) {",
        "        if self.values[source] >= scalar {",
        "            self.copy_ad(index, source);",
        "        } else {",
        "            self.store_scalar(index, scalar);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_min(&mut self, index: usize, left: usize, right: usize) {",
        "        if self.values[left] <= self.values[right] {",
        "            self.copy_ad(index, left);",
        "        } else {",
        "            self.copy_ad(index, right);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_max(&mut self, index: usize, left: usize, right: usize) {",
        "        if self.values[left] >= self.values[right] {",
        "            self.copy_ad(index, left);",
        "        } else {",
        "            self.copy_ad(index, right);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_min3(&mut self, index: usize, first: usize, second: usize, third: usize) {",
        "        let mut selected = first;",
        "        if self.values[second] < self.values[selected] { selected = second; }",
        "        if self.values[third] < self.values[selected] { selected = third; }",
        "        self.copy_ad(index, selected);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_max3(&mut self, index: usize, first: usize, second: usize, third: usize) {",
        "        let mut selected = first;",
        "        if self.values[second] > self.values[selected] { selected = second; }",
        "        if self.values[third] > self.values[selected] { selected = third; }",
        "        self.copy_ad(index, selected);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sub_from_scalar(&mut self, index: usize, scalar: f64, source: usize) {",
        "        self.store_unary_scaled(index, source, scalar - self.values[source], -1.0);",
        "    }",
        "",
        "    #[inline]",
    "    fn store_div_from_scalar(&mut self, index: usize, scalar: f64, source: usize) {",
    "        let reciprocal = 1.0 / self.values[source];",
    "        let quotient = scalar * reciprocal;",
    "        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_div_from_scalar_offset_input(&mut self, index: usize, scalar: f64, source: usize, offset: f64) {",
    "        let denominator = self.values[source] + offset;",
    "        let reciprocal = 1.0 / denominator;",
    "        let quotient = scalar * reciprocal;",
    "        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_div_from_scalar_scaled_input(&mut self, index: usize, scalar: f64, source: usize, scale: f64) {",
    "        let denominator = self.values[source] * scale;",
    "        let reciprocal = 1.0 / denominator;",
    "        let quotient = scalar * reciprocal;",
    "        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal * scale);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_div_from_scalar_offset_scaled_input(&mut self, index: usize, scalar: f64, source: usize, scale: f64, offset: f64) {",
    "        let denominator = self.values[source] * scale + offset;",
    "        let reciprocal = 1.0 / denominator;",
    "        let quotient = scalar * reciprocal;",
    "        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal * scale);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_sqrt(&mut self, index: usize, source: usize) {",
    "        let value = self.values[source].sqrt();",
    "        self.store_unary_scaled(index, source, value, 1.0 / (2.0 * value));",
    "    }",
    "",
    "    #[inline]",
    "    fn store_sqrt_offset_scaled_input(&mut self, index: usize, source: usize, scale: f64, offset: f64) {",
    "        let raw = self.values[source] * scale + offset;",
    "        let value = raw.sqrt();",
    "        self.store_unary_scaled(index, source, value, scale / (2.0 * value));",
    "    }",
    "",
    "    #[inline]",
    "    fn store_sqrt_square_offset(&mut self, index: usize, source: usize, offset: f64) {",
    "        let source_value = self.values[source];",
    "        let value = (source_value * source_value + offset).sqrt();",
    "        self.store_unary_scaled(index, source, value, source_value / value);",
    "    }",
    "",
    "    #[inline]",
    "    fn store_sqrt_square_add(&mut self, index: usize, square_source: usize, add_source: usize) {",
    "        let square_value = self.values[square_source];",
    "        let value = (square_value * square_value + self.values[add_source]).sqrt();",
    "        let square_scale = square_value / value;",
    "        let add_scale = 1.0 / (2.0 * value);",
    "        let square_node_derivatives = self.node_derivatives[square_source];",
    "        let add_node_derivatives = self.node_derivatives[add_source];",
    "        let square_branch_derivatives = self.branch_derivatives[square_source];",
    "        let add_branch_derivatives = self.branch_derivatives[add_source];",
    "        self.values[index] = value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = square_node_derivatives[axis] * square_scale + add_node_derivatives[axis] * add_scale; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = square_branch_derivatives[axis] * square_scale + add_branch_derivatives[axis] * add_scale; }",
    "    }",
    "",
    "    #[inline]",
    "    fn store_sqrt_square_sum(&mut self, index: usize, left: usize, right: usize) {",
    "        let left_value = self.values[left];",
    "        let right_value = self.values[right];",
    "        let value = (left_value * left_value + right_value * right_value).sqrt();",
    "        let left_scale = left_value / value;",
    "        let right_scale = right_value / value;",
    "        let left_node_derivatives = self.node_derivatives[left];",
    "        let right_node_derivatives = self.node_derivatives[right];",
    "        let left_branch_derivatives = self.branch_derivatives[left];",
    "        let right_branch_derivatives = self.branch_derivatives[right];",
    "        self.values[index] = value;",
    "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = left_node_derivatives[axis] * left_scale + right_node_derivatives[axis] * right_scale; }",
    "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = left_branch_derivatives[axis] * left_scale + right_branch_derivatives[axis] * right_scale; }",
    "    }",
    "",
    "    #[inline]",
        "    fn store_exp(&mut self, index: usize, source: usize) {",
        "        let value = self.values[source].exp();",
        "        self.store_unary_scaled(index, source, value, value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limexp(&mut self, index: usize, source: usize) {",
        "        let raw = self.values[source];",
        "        if raw < 80.0 {",
        "            let value = raw.exp();",
        "            self.store_unary_scaled(index, source, value, value);",
        "        } else {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limited_exp(&mut self, index: usize, source: usize) {",
        "        let raw = self.values[source];",
        "        if raw > 80.0 {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);",
        "        } else if raw < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35);",
        "        } else {",
        "            let value = raw.exp();",
        "            self.store_unary_scaled(index, source, value, value);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_exp(&mut self, index: usize, source: usize, scale: f64) {",
        "        let value = self.values[source].exp() * scale;",
        "        self.store_unary_scaled(index, source, value, value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln(&mut self, index: usize, source: usize) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw.ln(), 1.0 / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn ln_one_plus_exp_raw(raw: f64) -> (f64, f64) {",
        "        if raw > 0.0 {",
        "            (raw + (-raw).exp().ln_1p(), 1.0 / (1.0 + (-raw).exp()))",
        "        } else {",
        "            let exp = raw.exp();",
        "            (exp.ln_1p(), exp / (1.0 + exp))",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_one_plus_exp(&mut self, index: usize, source: usize) {",
        "        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.values[source]);",
        "        self.store_unary_scaled(index, source, value, derivative_scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_ln_one_plus_exp(&mut self, index: usize, source: usize, scale: f64) {",
        "        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.values[source]);",
        "        self.store_unary_scaled(index, source, value * scale, derivative_scale * scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_sqrt(&mut self, index: usize, source: usize, scale: f64) {",
        "        let value = self.values[source].sqrt();",
        "        self.store_unary_scaled(index, source, value * scale, scale / (2.0 * value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_square(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw * raw * scale, 2.0 * raw * scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_abs(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw.abs() * scale, if raw >= 0.0 { scale } else { -scale });",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_ln(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw.ln() * scale, scale / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_limexp(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source];",
        "        if raw < 80.0 {",
        "            let value = raw.exp() * scale;",
        "            self.store_unary_scaled(index, source, value, value);",
        "        } else {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)) * scale, LIMEXP_MAX * scale);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_limited_exp(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source];",
        "        if raw > 80.0 {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0) * scale, LIMEXP_MAX * scale);",
        "        } else if raw < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35 * scale);",
        "        } else {",
        "            let value = raw.exp() * scale;",
        "            self.store_unary_scaled(index, source, value, value);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_scaled_input(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source] * scale;",
        "        let value = raw.sqrt();",
        "        self.store_unary_scaled(index, source, value, scale / (2.0 * value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_exp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source] * scale;",
        "        let value = raw.exp();",
        "        self.store_unary_scaled(index, source, value, value * scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limexp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source] * scale;",
        "        if raw < 80.0 {",
        "            let value = raw.exp();",
        "            self.store_unary_scaled(index, source, value, value * scale);",
        "        } else {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX * scale);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limited_exp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source] * scale;",
        "        if raw > 80.0 {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX * scale);",
        "        } else if raw < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35);",
        "        } else {",
        "            let value = raw.exp();",
        "            self.store_unary_scaled(index, source, value, value * scale);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_scaled_input(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source] * scale;",
        "        self.store_unary_scaled(index, source, raw.ln(), scale / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_one_plus_exp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {",
        "        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.values[source] * scale);",
        "        self.store_unary_scaled(index, source, value, derivative_scale * scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sin_scaled_input(&mut self, index: usize, source: usize, scale: f64) {",
        "        let raw = self.values[source] * scale;",
        "        self.store_unary_scaled(index, source, raw.sin(), raw.cos() * scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_sqrt_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {",
        "        let raw = self.values[source] * input_scale;",
        "        let value = raw.sqrt();",
        "        self.store_unary_scaled(index, source, value * output_scale, output_scale * input_scale / (2.0 * value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_exp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {",
        "        let raw = self.values[source] * input_scale;",
        "        let value = raw.exp();",
        "        self.store_unary_scaled(index, source, value * output_scale, value * output_scale * input_scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_limexp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {",
        "        let raw = self.values[source] * input_scale;",
        "        if raw < 80.0 {",
        "            let value = raw.exp();",
        "            self.store_unary_scaled(index, source, value * output_scale, value * output_scale * input_scale);",
        "        } else {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)) * output_scale, LIMEXP_MAX * output_scale * input_scale);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_limited_exp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {",
        "        let raw = self.values[source] * input_scale;",
        "        if raw > 80.0 {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0) * output_scale, LIMEXP_MAX * output_scale * input_scale);",
        "        } else if raw < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35 * output_scale);",
        "        } else {",
        "            let value = raw.exp();",
        "            self.store_unary_scaled(index, source, value * output_scale, value * output_scale * input_scale);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_ln_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {",
        "        let raw = self.values[source] * input_scale;",
        "        self.store_unary_scaled(index, source, raw.ln() * output_scale, output_scale * input_scale / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_ln_one_plus_exp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {",
        "        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.values[source] * input_scale);",
        "        self.store_unary_scaled(index, source, value * output_scale, derivative_scale * input_scale * output_scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_scaled_sin_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {",
        "        let raw = self.values[source] * input_scale;",
        "        self.store_unary_scaled(index, source, raw.sin() * output_scale, raw.cos() * output_scale * input_scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_offset_input(&mut self, index: usize, source: usize, offset: f64) {",
        "        let raw = self.values[source] + offset;",
        "        let value = raw.sqrt();",
        "        self.store_unary_scaled(index, source, value, 1.0 / (2.0 * value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_exp_offset_input(&mut self, index: usize, source: usize, offset: f64) {",
        "        let raw = self.values[source] + offset;",
        "        let value = raw.exp();",
        "        self.store_unary_scaled(index, source, value, value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_offset_input(&mut self, index: usize, source: usize, offset: f64) {",
        "        let raw = self.values[source] + offset;",
        "        self.store_unary_scaled(index, source, raw.ln(), 1.0 / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_square(&mut self, index: usize, source: usize, offset: f64) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw * raw + offset, 2.0 * raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_abs(&mut self, index: usize, source: usize, offset: f64) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw.abs() + offset, if raw >= 0.0 { 1.0 } else { -1.0 });",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_sqrt(&mut self, index: usize, source: usize, offset: f64) {",
        "        let value = self.values[source].sqrt();",
        "        self.store_unary_scaled(index, source, value + offset, 1.0 / (2.0 * value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_exp(&mut self, index: usize, source: usize, offset: f64) {",
        "        let value = self.values[source].exp();",
        "        self.store_unary_scaled(index, source, value + offset, value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_ln(&mut self, index: usize, source: usize, offset: f64) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw.ln() + offset, 1.0 / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_limexp(&mut self, index: usize, source: usize, offset: f64) {",
        "        let raw = self.values[source];",
        "        if raw < 80.0 {",
        "            let value = raw.exp();",
        "            self.store_unary_scaled(index, source, value + offset, value);",
        "        } else {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)) + offset, LIMEXP_MAX);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_offset_limited_exp(&mut self, index: usize, source: usize, offset: f64) {",
        "        let raw = self.values[source];",
        "        if raw > 80.0 {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0) + offset, LIMEXP_MAX);",
        "        } else if raw < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35 + offset);",
        "        } else {",
        "            let value = raw.exp();",
        "            self.store_unary_scaled(index, source, value + offset, value);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_neg_input(&mut self, index: usize, source: usize) {",
        "        let raw = -self.values[source];",
        "        let value = raw.sqrt();",
        "        self.store_unary_scaled(index, source, value, -1.0 / (2.0 * value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_exp_neg_input(&mut self, index: usize, source: usize) {",
        "        let value = (-self.values[source]).exp();",
        "        self.store_unary_scaled(index, source, value, -value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limexp_neg_input(&mut self, index: usize, source: usize) {",
        "        let raw = -self.values[source];",
        "        if raw < 80.0 {",
        "            let value = raw.exp();",
        "            self.store_unary_scaled(index, source, value, -value);",
        "        } else {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)), -LIMEXP_MAX);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limited_exp_neg_input(&mut self, index: usize, source: usize) {",
        "        let raw = -self.values[source];",
        "        if raw > 80.0 {",
        "            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0), -LIMEXP_MAX);",
        "        } else if raw < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35);",
        "        } else {",
        "            let value = raw.exp();",
        "            self.store_unary_scaled(index, source, value, -value);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_neg_input(&mut self, index: usize, source: usize) {",
        "        let raw = -self.values[source];",
        "        self.store_unary_scaled(index, source, raw.ln(), -1.0 / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_one_plus_exp_neg_input(&mut self, index: usize, source: usize) {",
        "        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(-self.values[source]);",
        "        self.store_unary_scaled(index, source, value, -derivative_scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_unary_add_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] + right_node_derivatives[axis]) * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] + right_branch_derivatives[axis]) * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_unary_sub_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] - right_node_derivatives[axis]) * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] - right_branch_derivatives[axis]) * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_unary_mul_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        self.values[index] = value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] * right_value + left_value * right_node_derivatives[axis]) * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] * right_value + left_value * right_branch_derivatives[axis]) * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_unary_div_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {",
        "        let left_value = self.values[left];",
        "        let right_value = self.values[right];",
        "        let left_node_derivatives = self.node_derivatives[left];",
        "        let right_node_derivatives = self.node_derivatives[right];",
        "        let left_branch_derivatives = self.branch_derivatives[left];",
        "        let right_branch_derivatives = self.branch_derivatives[right];",
        "        let reciprocal = 1.0 / right_value;",
        "        let quotient = left_value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        self.values[index] = value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = (left_node_derivatives[axis] * reciprocal + right_node_derivatives[axis] * right_scale) * derivative_scale; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = (left_branch_derivatives[axis] * reciprocal + right_branch_derivatives[axis] * right_scale) * derivative_scale; }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_add(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] + self.values[right];",
        "        let value = raw.sqrt();",
        "        self.store_unary_add_scaled(index, left, right, value, 1.0 / (2.0 * value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_sub(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] - self.values[right];",
        "        let value = raw.sqrt();",
        "        self.store_unary_sub_scaled(index, left, right, value, 1.0 / (2.0 * value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_mul(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] * self.values[right];",
        "        let value = raw.sqrt();",
        "        self.store_unary_mul_scaled(index, left, right, value, 1.0 / (2.0 * value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sqrt_div(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] / self.values[right];",
        "        let value = raw.sqrt();",
        "        self.store_unary_div_scaled(index, left, right, value, 1.0 / (2.0 * value));",
        "    }",
        "",
        "    #[inline]",
        "    fn store_exp_add(&mut self, index: usize, left: usize, right: usize) {",
        "        let value = (self.values[left] + self.values[right]).exp();",
        "        self.store_unary_add_scaled(index, left, right, value, value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_exp_sub(&mut self, index: usize, left: usize, right: usize) {",
        "        let value = (self.values[left] - self.values[right]).exp();",
        "        self.store_unary_sub_scaled(index, left, right, value, value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_exp_mul(&mut self, index: usize, left: usize, right: usize) {",
        "        let value = (self.values[left] * self.values[right]).exp();",
        "        self.store_unary_mul_scaled(index, left, right, value, value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_exp_div(&mut self, index: usize, left: usize, right: usize) {",
        "        let value = (self.values[left] / self.values[right]).exp();",
        "        self.store_unary_div_scaled(index, left, right, value, value);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_add(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] + self.values[right];",
        "        self.store_unary_add_scaled(index, left, right, raw.ln(), 1.0 / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_sub(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] - self.values[right];",
        "        self.store_unary_sub_scaled(index, left, right, raw.ln(), 1.0 / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_mul(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] * self.values[right];",
        "        self.store_unary_mul_scaled(index, left, right, raw.ln(), 1.0 / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_ln_div(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] / self.values[right];",
        "        self.store_unary_div_scaled(index, left, right, raw.ln(), 1.0 / raw);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limexp_add(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] + self.values[right];",
        "        if raw < 80.0 {",
        "            let value = raw.exp();",
        "            self.store_unary_add_scaled(index, left, right, value, value);",
        "        } else {",
        "            self.store_unary_add_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limexp_sub(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] - self.values[right];",
        "        if raw < 80.0 {",
        "            let value = raw.exp();",
        "            self.store_unary_sub_scaled(index, left, right, value, value);",
        "        } else {",
        "            self.store_unary_sub_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limexp_mul(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] * self.values[right];",
        "        if raw < 80.0 {",
        "            let value = raw.exp();",
        "            self.store_unary_mul_scaled(index, left, right, value, value);",
        "        } else {",
        "            self.store_unary_mul_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limexp_div(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] / self.values[right];",
        "        if raw < 80.0 {",
        "            let value = raw.exp();",
        "            self.store_unary_div_scaled(index, left, right, value, value);",
        "        } else {",
        "            self.store_unary_div_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limited_exp_add(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] + self.values[right];",
        "        if raw > 80.0 {",
        "            self.store_unary_add_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);",
        "        } else if raw < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35);",
        "        } else {",
        "            let value = raw.exp();",
        "            self.store_unary_add_scaled(index, left, right, value, value);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limited_exp_sub(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] - self.values[right];",
        "        if raw > 80.0 {",
        "            self.store_unary_sub_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);",
        "        } else if raw < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35);",
        "        } else {",
        "            let value = raw.exp();",
        "            self.store_unary_sub_scaled(index, left, right, value, value);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limited_exp_mul(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] * self.values[right];",
        "        if raw > 80.0 {",
        "            self.store_unary_mul_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);",
        "        } else if raw < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35);",
        "        } else {",
        "            let value = raw.exp();",
        "            self.store_unary_mul_scaled(index, left, right, value, value);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_limited_exp_div(&mut self, index: usize, left: usize, right: usize) {",
        "        let raw = self.values[left] / self.values[right];",
        "        if raw > 80.0 {",
        "            self.store_unary_div_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);",
        "        } else if raw < -80.0 {",
        "            self.store_scalar(index, 1.804851387e-35);",
        "        } else {",
        "            let value = raw.exp();",
        "            self.store_unary_div_scaled(index, left, right, value, value);",
        "        }",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sin(&mut self, index: usize, source: usize) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw.sin(), raw.cos());",
        "    }",
        "",
        "    #[inline]",
        "    fn store_sinh(&mut self, index: usize, source: usize) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw.sinh(), raw.cosh());",
        "    }",
        "",
        "    #[inline]",
        "    fn store_asinh(&mut self, index: usize, source: usize) {",
        "        let raw = self.values[source];",
        "        self.store_unary_scaled(index, source, raw.asinh(), 1.0 / ((raw * raw) + 1.0).sqrt());",
        "    }",
        "",
        "    #[inline]",
        "    fn store_powf(&mut self, index: usize, source: usize, exponent: f64) {",
        "        let base = self.values[source];",
        "        let value = base.powf(exponent);",
        "        let derivative_scale = AdValue::pow_derivative(value, base, exponent, 1.0, 0.0);",
        "        self.store_unary_scaled(index, source, value, derivative_scale);",
        "    }",
        "",
        "    #[inline]",
        "    fn store_unary_scaled(&mut self, index: usize, source: usize, value: f64, derivative_scale: f64) {",
        "        let node_derivatives = self.node_derivatives[source];",
        "        let branch_derivatives = self.branch_derivatives[source];",
        "        self.values[index] = value;",
        "        for axis in 0..Instance::NODE_COUNT { self.node_derivatives[index][axis] = derivative_scale * node_derivatives[axis]; }",
        "        for axis in 0..Instance::BRANCH_COUNT { self.branch_derivatives[index][axis] = derivative_scale * branch_derivatives[axis]; }",
        "    }",
        "",
    ]
    .join("\n")
}

fn generate_ad_value_struct() -> String {
    [
        "struct AdValue {",
        "    value: f64,",
        "    node_derivatives: [f64; Instance::NODE_COUNT],",
        "    branch_derivatives: [f64; Instance::BRANCH_COUNT],",
        "}",
        "",
        "impl AdValue {",
        "    #[inline]",
        "    fn constant(value: f64) -> Self {",
        "        Self { value, node_derivatives: [0.0; Instance::NODE_COUNT], branch_derivatives: [0.0; Instance::BRANCH_COUNT] }",
        "    }",
        "    #[inline]",
        "    fn voltage(ctx: &GeneratedEvalContext<'_>, nodes: &[usize; Instance::NODE_COUNT], pos: Option<usize>, neg: Option<usize>) -> Self {",
        "        let pos_value = pos.map(|index| ctx.node_voltage(nodes[index])).unwrap_or(0.0);",
        "        let neg_value = neg.map(|index| ctx.node_voltage(nodes[index])).unwrap_or(0.0);",
        "        let mut value = Self::constant(pos_value - neg_value);",
        "        if let Some(index) = pos { value.node_derivatives[index] += 1.0; }",
        "        if let Some(index) = neg { value.node_derivatives[index] -= 1.0; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn branch_current(ctx: &GeneratedEvalContext<'_>, branches: &[usize; Instance::BRANCH_COUNT], slot: usize) -> Self {",
        "        let mut value = Self::constant(ctx.branch_current(branches[slot]));",
        "        value.branch_derivatives[slot] = 1.0;",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn neg(mut value: Self) -> Self {",
        "        value.value = -value.value;",
        "        for derivative in &mut value.node_derivatives { *derivative = -*derivative; }",
        "        for derivative in &mut value.branch_derivatives { *derivative = -*derivative; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn add(left: Self, right: Self) -> Self {",
        "        let mut value = left;",
        "        value.value += right.value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] += right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] += right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn sub(left: Self, right: Self) -> Self {",
        "        let mut value = left;",
        "        value.value -= right.value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] -= right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] -= right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_inputs(left: Self, left_scale: f64, right: Self, right_scale: f64) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value * left_scale;",
        "        let right_value = right.value * right_scale;",
        "        value.value = left_value + right_value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * left_scale + right.node_derivatives[index] * right_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * left_scale + right.branch_derivatives[index] * right_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn sub_scaled_inputs(left: Self, left_scale: f64, right: Self, right_scale: f64) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value * left_scale;",
        "        let right_value = right.value * right_scale;",
        "        value.value = left_value - right_value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * left_scale - right.node_derivatives[index] * right_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * left_scale - right.branch_derivatives[index] * right_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_inputs3(first: Self, first_scale: f64, second: Self, second_scale: f64, third: Self, third_scale: f64) -> Self {",
        "        let mut value = first;",
        "        let first_value = value.value * first_scale;",
        "        let second_value = second.value * second_scale;",
        "        let third_value = third.value * third_scale;",
        "        value.value = (first_value + second_value) + third_value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * first_scale + second.node_derivatives[index] * second_scale) + third.node_derivatives[index] * third_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * first_scale + second.branch_derivatives[index] * second_scale) + third.branch_derivatives[index] * third_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_inputs3_offset(first: Self, first_scale: f64, second: Self, second_scale: f64, third: Self, third_scale: f64, offset: f64) -> Self {",
        "        let mut value = first;",
        "        let first_value = value.value * first_scale;",
        "        let second_value = second.value * second_scale;",
        "        let third_value = third.value * third_scale;",
        "        value.value = ((first_value + second_value) + third_value) + offset;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * first_scale + second.node_derivatives[index] * second_scale) + third.node_derivatives[index] * third_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * first_scale + second.branch_derivatives[index] * second_scale) + third.branch_derivatives[index] * third_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_product(value: Self, value_scale: f64, product_left: Self, product_right: Self, product_scale: f64) -> Self {",
        "        let mut result = value;",
        "        let value_term = result.value * value_scale;",
        "        let product_left_value = product_left.value;",
        "        let product_right_value = product_right.value;",
        "        let product_term = product_left_value * product_right_value * product_scale;",
        "        result.value = value_term + product_term;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = result.node_derivatives[index] * value_scale + (product_left.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = result.branch_derivatives[index] * value_scale + (product_left.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_scale; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_offset_product_lhs(value: Self, value_scale: f64, product_left: Self, product_left_offset: f64, product_right: Self, product_scale: f64) -> Self {",
        "        let mut result = value;",
        "        let value_term = result.value * value_scale;",
        "        let product_left_value = product_left.value + product_left_offset;",
        "        let product_right_value = product_right.value;",
        "        let product_term = product_left_value * product_right_value * product_scale;",
        "        result.value = value_term + product_term;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = result.node_derivatives[index] * value_scale + (product_left.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = result.branch_derivatives[index] * value_scale + (product_left.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_scale; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_offset_product_rhs(value: Self, value_scale: f64, product_left: Self, product_right: Self, product_right_offset: f64, product_scale: f64) -> Self {",
        "        let mut result = value;",
        "        let value_term = result.value * value_scale;",
        "        let product_left_value = product_left.value;",
        "        let product_right_value = product_right.value + product_right_offset;",
        "        let product_term = product_left_value * product_right_value * product_scale;",
        "        result.value = value_term + product_term;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = result.node_derivatives[index] * value_scale + (product_left.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = result.branch_derivatives[index] * value_scale + (product_left.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_scale; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_inputs_product(first: Self, first_scale: f64, second: Self, second_scale: f64, product_left: Self, product_right: Self, product_scale: f64) -> Self {",
        "        let mut result = first;",
        "        let first_value = result.value * first_scale;",
        "        let second_value = second.value * second_scale;",
        "        let product_left_value = product_left.value;",
        "        let product_right_value = product_right.value;",
        "        let product_term = product_left_value * product_right_value * product_scale;",
        "        result.value = first_value + second_value + product_term;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = result.node_derivatives[index] * first_scale + second.node_derivatives[index] * second_scale + (product_left.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = result.branch_derivatives[index] * first_scale + second.branch_derivatives[index] * second_scale + (product_left.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_scale; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_sub_value_product(scalar: f64, subtrahend: Self, value_scale: f64, product_left: Self, product_right: Self, product_scale: f64) -> Self {",
        "        let mut result = subtrahend;",
        "        let value_term = (scalar - result.value) * value_scale;",
        "        let product_left_value = product_left.value;",
        "        let product_right_value = product_right.value;",
        "        let product_term = product_left_value * product_right_value * product_scale;",
        "        result.value = value_term + product_term;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = -result.node_derivatives[index] * value_scale + (product_left.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = -result.branch_derivatives[index] * value_scale + (product_left.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_scale; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_sub_product_lhs(value: Self, value_scale: f64, scalar: f64, subtrahend: Self, product_right: Self, product_scale: f64) -> Self {",
        "        let mut result = value;",
        "        let value_term = result.value * value_scale;",
        "        let product_left_value = scalar - subtrahend.value;",
        "        let product_right_value = product_right.value;",
        "        let product_term = product_left_value * product_right_value * product_scale;",
        "        result.value = value_term + product_term;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = result.node_derivatives[index] * value_scale + (-subtrahend.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = result.branch_derivatives[index] * value_scale + (-subtrahend.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_scale; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_sub_product_rhs(value: Self, value_scale: f64, product_left: Self, scalar: f64, subtrahend: Self, product_scale: f64) -> Self {",
        "        let mut result = value;",
        "        let value_term = result.value * value_scale;",
        "        let product_left_value = product_left.value;",
        "        let product_right_value = scalar - subtrahend.value;",
        "        let product_term = product_left_value * product_right_value * product_scale;",
        "        result.value = value_term + product_term;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = result.node_derivatives[index] * value_scale + (product_left.node_derivatives[index] * product_right_value - product_left_value * subtrahend.node_derivatives[index]) * product_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = result.branch_derivatives[index] * value_scale + (product_left.branch_derivatives[index] * product_right_value - product_left_value * subtrahend.branch_derivatives[index]) * product_scale; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_square_product(square_value: Self, square_scale: f64, product_left: Self, product_right: Self, product_scale: f64) -> Self {",
        "        let mut result = square_value;",
        "        let square_raw = result.value;",
        "        let product_left_value = product_left.value;",
        "        let product_right_value = product_right.value;",
        "        let square_term = square_raw * square_raw * square_scale;",
        "        let product_term = product_left_value * product_right_value * product_scale;",
        "        let square_derivative_scale = 2.0 * square_raw * square_scale;",
        "        result.value = square_term + product_term;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = result.node_derivatives[index] * square_derivative_scale + (product_left.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = result.branch_derivatives[index] * square_derivative_scale + (product_left.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_scale; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn add_scaled_products(left_product_left: Self, left_product_right: Self, left_scale: f64, right_product_left: Self, right_product_right: Self, right_scale: f64) -> Self {",
        "        let mut result = left_product_left;",
        "        let left_product_left_value = result.value;",
        "        let left_product_right_value = left_product_right.value;",
        "        let right_product_left_value = right_product_left.value;",
        "        let right_product_right_value = right_product_right.value;",
        "        let left_product_term = left_product_left_value * left_product_right_value * left_scale;",
        "        let right_product_term = right_product_left_value * right_product_right_value * right_scale;",
        "        result.value = left_product_term + right_product_term;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = (result.node_derivatives[index] * left_product_right_value + left_product_left_value * left_product_right.node_derivatives[index]) * left_scale + (right_product_left.node_derivatives[index] * right_product_right_value + right_product_left_value * right_product_right.node_derivatives[index]) * right_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = (result.branch_derivatives[index] * left_product_right_value + left_product_left_value * left_product_right.branch_derivatives[index]) * left_scale + (right_product_left.branch_derivatives[index] * right_product_right_value + right_product_left_value * right_product_right.branch_derivatives[index]) * right_scale; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn mul(left: Self, right: Self) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value;",
        "        value.value = left_value * right.value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * right.value + left_value * right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * right.value + left_value * right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_scaled_lhs(left: Self, scale: f64, right: Self) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value;",
        "        let scaled_left_value = left_value * scale;",
        "        value.value = scaled_left_value * right.value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * right.value + left_value * right.node_derivatives[index]) * scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * right.value + left_value * right.branch_derivatives[index]) * scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_scaled_rhs(left: Self, right: Self, scale: f64) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value;",
        "        let scaled_right_value = right.value * scale;",
        "        value.value = left_value * scaled_right_value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * right.value + left_value * right.node_derivatives[index]) * scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * right.value + left_value * right.branch_derivatives[index]) * scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_scaled_output(left: Self, right: Self, scale: f64) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value;",
        "        let product = left_value * right.value;",
        "        value.value = product * scale;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * right.value + left_value * right.node_derivatives[index]) * scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * right.value + left_value * right.branch_derivatives[index]) * scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_offset_lhs(left: Self, offset: f64, right: Self) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value + offset;",
        "        let right_value = right.value;",
        "        value.value = left_value * right_value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * right_value + left_value * right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * right_value + left_value * right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_offset_rhs(left: Self, right: Self, offset: f64) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value;",
        "        let right_value = right.value + offset;",
        "        value.value = left_value * right_value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * right_value + left_value * right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * right_value + left_value * right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_offset_lhs_scaled_output(left: Self, offset: f64, right: Self, scale: f64) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value + offset;",
        "        let right_value = right.value;",
        "        let scaled_left_value = left_value * scale;",
        "        let scaled_right_value = right_value * scale;",
        "        value.value = scaled_left_value * right_value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * scaled_right_value + scaled_left_value * right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * scaled_right_value + scaled_left_value * right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_offset_rhs_scaled_output(left: Self, right: Self, offset: f64, scale: f64) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value;",
        "        let right_value = right.value + offset;",
        "        let scaled_left_value = left_value * scale;",
        "        let scaled_right_value = right_value * scale;",
        "        value.value = left_value * scaled_right_value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * scaled_right_value + scaled_left_value * right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * scaled_right_value + scaled_left_value * right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_sub_from_scalar_lhs(scalar: f64, value: Self, right: Self) -> Self {",
        "        let mut result = value;",
        "        let left_value = scalar - result.value;",
        "        result.value = left_value * right.value;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = -result.node_derivatives[index] * right.value + left_value * right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = -result.branch_derivatives[index] * right.value + left_value * right.branch_derivatives[index]; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_sub_from_scalar_rhs(left: Self, scalar: f64, value: Self) -> Self {",
        "        let mut result = left;",
        "        let left_value = result.value;",
        "        let right_value = scalar - value.value;",
        "        result.value = left_value * right_value;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = result.node_derivatives[index] * right_value - left_value * value.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = result.branch_derivatives[index] * right_value - left_value * value.branch_derivatives[index]; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_sub_from_scalar_lhs_scaled_output(scalar: f64, value: Self, right: Self, scale: f64) -> Self {",
        "        let mut result = value;",
        "        let left_value = scalar - result.value;",
        "        let scaled_left_value = left_value * scale;",
        "        let scaled_right_value = right.value * scale;",
        "        result.value = scaled_left_value * right.value;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = -result.node_derivatives[index] * scaled_right_value + scaled_left_value * right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = -result.branch_derivatives[index] * scaled_right_value + scaled_left_value * right.branch_derivatives[index]; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_sub_from_scalar_rhs_scaled_output(left: Self, scalar: f64, value: Self, scale: f64) -> Self {",
        "        let mut result = left;",
        "        let left_value = result.value;",
        "        let right_value = scalar - value.value;",
        "        let scaled_left_value = left_value * scale;",
        "        let scaled_right_value = right_value * scale;",
        "        result.value = left_value * scaled_right_value;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = result.node_derivatives[index] * scaled_right_value - scaled_left_value * value.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = result.branch_derivatives[index] * scaled_right_value - scaled_left_value * value.branch_derivatives[index]; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn mul_sub_from_scalar_scaled_offset_self(scalar: f64, value: Self, input_scale: f64, offset: f64, output_scale: f64) -> Self {",
        "        let mut result = value;",
        "        let sub_value = scalar - result.value;",
        "        let affine_value = sub_value * input_scale + offset;",
        "        result.value = sub_value * affine_value * output_scale;",
        "        let derivative_scale = -((2.0 * input_scale * sub_value + offset) * output_scale);",
        "        for derivative in &mut result.node_derivatives { *derivative *= derivative_scale; }",
        "        for derivative in &mut result.branch_derivatives { *derivative *= derivative_scale; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn mul3(left: Self, middle: Self, right: Self) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value;",
        "        let middle_value = middle.value;",
        "        let right_value = right.value;",
        "        let left_middle_value = left_value * middle_value;",
        "        let left_right_value = left_value * right_value;",
        "        let middle_right_value = middle_value * right_value;",
        "        value.value = left_middle_value * right_value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * middle_right_value + middle.node_derivatives[index] * left_right_value + right.node_derivatives[index] * left_middle_value; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * middle_right_value + middle.branch_derivatives[index] * left_right_value + right.branch_derivatives[index] * left_middle_value; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn mul3_scaled_output(left: Self, middle: Self, right: Self, scale: f64) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value;",
        "        let middle_value = middle.value;",
        "        let right_value = right.value;",
        "        let left_middle_value = left_value * middle_value;",
        "        let left_right_value = left_value * right_value;",
        "        let middle_right_value = middle_value * right_value;",
        "        let scaled_left_middle_value = left_middle_value * scale;",
        "        let scaled_left_right_value = left_right_value * scale;",
        "        let scaled_middle_right_value = middle_right_value * scale;",
        "        value.value = scaled_left_middle_value * right_value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * scaled_middle_right_value + middle.node_derivatives[index] * scaled_left_right_value + right.node_derivatives[index] * scaled_left_middle_value; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * scaled_middle_right_value + middle.branch_derivatives[index] * scaled_left_right_value + right.branch_derivatives[index] * scaled_left_middle_value; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn square(arg: Self) -> Self {",
        "        let mut value = arg;",
        "        let raw = value.value;",
        "        value.value = raw * raw;",
        "        let derivative_scale = 2.0 * raw;",
        "        for derivative in &mut value.node_derivatives { *derivative *= derivative_scale; }",
        "        for derivative in &mut value.branch_derivatives { *derivative *= derivative_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div(left: Self, right: Self) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value;",
        "        let reciprocal = 1.0 / right.value;",
        "        let quotient = left_value * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        value.value = quotient;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * reciprocal + right.node_derivatives[index] * right_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * reciprocal + right.branch_derivatives[index] * right_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_scaled_inputs(left: Self, left_scale: f64, right: Self, right_scale: f64) -> Self {",
        "        let mut value = left;",
        "        let left_value = value.value * left_scale;",
        "        let right_value = right.value * right_scale;",
        "        let reciprocal = 1.0 / right_value;",
        "        let quotient = left_value * reciprocal;",
        "        let left_derivative_scale = left_scale * reciprocal;",
        "        let right_derivative_scale = -quotient * reciprocal * right_scale;",
        "        value.value = quotient;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * left_derivative_scale + right.node_derivatives[index] * right_derivative_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * left_derivative_scale + right.branch_derivatives[index] * right_derivative_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_scaled_value_by_product(input: Self, input_scale: f64, denominator_left: Self, denominator_right: Self, denominator_scale: f64) -> Self {",
        "        let mut value = input;",
        "        let input_value = value.value;",
        "        let denominator_left_value = denominator_left.value;",
        "        let denominator_right_value = denominator_right.value;",
        "        let reciprocal = 1.0 / (denominator_left_value * denominator_right_value * denominator_scale);",
        "        let quotient = input_value * input_scale * reciprocal;",
        "        let input_derivative_scale = input_scale * reciprocal;",
        "        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;",
        "        value.value = quotient;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = value.node_derivatives[index] * input_derivative_scale + (denominator_left.node_derivatives[index] * denominator_right_value + denominator_left_value * denominator_right.node_derivatives[index]) * denominator_derivative_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = value.branch_derivatives[index] * input_derivative_scale + (denominator_left.branch_derivatives[index] * denominator_right_value + denominator_left_value * denominator_right.branch_derivatives[index]) * denominator_derivative_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_scaled_product(product_left: Self, product_right: Self, product_scale: f64, denominator: Self, denominator_scale: f64) -> Self {",
        "        let mut value = product_left;",
        "        let product_left_value = value.value;",
        "        let product_right_value = product_right.value;",
        "        let denominator_value = denominator.value * denominator_scale;",
        "        let reciprocal = 1.0 / denominator_value;",
        "        let product_value = product_left_value * product_right_value;",
        "        let scaled_product_value = product_value * product_scale;",
        "        let quotient = scaled_product_value * reciprocal;",
        "        let product_derivative_scale = product_scale * reciprocal;",
        "        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;",
        "        value.value = quotient;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_derivative_scale + denominator.node_derivatives[index] * denominator_derivative_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_derivative_scale + denominator.branch_derivatives[index] * denominator_derivative_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_scaled_product_by_product(product_left: Self, product_right: Self, product_scale: f64, denominator_left: Self, denominator_right: Self, denominator_scale: f64) -> Self {",
        "        let mut value = product_left;",
        "        let product_left_value = value.value;",
        "        let product_right_value = product_right.value;",
        "        let denominator_left_value = denominator_left.value;",
        "        let denominator_right_value = denominator_right.value;",
        "        let reciprocal = 1.0 / (denominator_left_value * denominator_right_value * denominator_scale);",
        "        let product_value = product_left_value * product_right_value;",
        "        let scaled_product_value = product_value * product_scale;",
        "        let quotient = scaled_product_value * reciprocal;",
        "        let product_derivative_scale = product_scale * reciprocal;",
        "        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;",
        "        value.value = quotient;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_derivative_scale + (denominator_left.node_derivatives[index] * denominator_right_value + denominator_left_value * denominator_right.node_derivatives[index]) * denominator_derivative_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_derivative_scale + (denominator_left.branch_derivatives[index] * denominator_right_value + denominator_left_value * denominator_right.branch_derivatives[index]) * denominator_derivative_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_scaled_product_offset_lhs(product_left: Self, product_left_offset: f64, product_right: Self, product_scale: f64, denominator: Self, denominator_scale: f64) -> Self {",
        "        let mut value = product_left;",
        "        let product_left_value = value.value + product_left_offset;",
        "        let product_right_value = product_right.value;",
        "        let denominator_value = denominator.value * denominator_scale;",
        "        let reciprocal = 1.0 / denominator_value;",
        "        let product_value = product_left_value * product_right_value;",
        "        let scaled_product_value = product_value * product_scale;",
        "        let quotient = scaled_product_value * reciprocal;",
        "        let product_derivative_scale = product_scale * reciprocal;",
        "        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;",
        "        value.value = quotient;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_derivative_scale + denominator.node_derivatives[index] * denominator_derivative_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_derivative_scale + denominator.branch_derivatives[index] * denominator_derivative_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_scaled_product_offset_rhs(product_left: Self, product_right: Self, product_right_offset: f64, product_scale: f64, denominator: Self, denominator_scale: f64) -> Self {",
        "        let mut value = product_left;",
        "        let product_left_value = value.value;",
        "        let product_right_value = product_right.value + product_right_offset;",
        "        let denominator_value = denominator.value * denominator_scale;",
        "        let reciprocal = 1.0 / denominator_value;",
        "        let product_value = product_left_value * product_right_value;",
        "        let scaled_product_value = product_value * product_scale;",
        "        let quotient = scaled_product_value * reciprocal;",
        "        let product_derivative_scale = product_scale * reciprocal;",
        "        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;",
        "        value.value = quotient;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_derivative_scale + denominator.node_derivatives[index] * denominator_derivative_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_derivative_scale + denominator.branch_derivatives[index] * denominator_derivative_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_scaled_product_offset_denominator(product_left: Self, product_right: Self, product_scale: f64, denominator: Self, denominator_offset: f64, denominator_scale: f64) -> Self {",
        "        let mut value = product_left;",
        "        let product_left_value = value.value;",
        "        let product_right_value = product_right.value;",
        "        let denominator_value = (denominator.value + denominator_offset) * denominator_scale;",
        "        let reciprocal = 1.0 / denominator_value;",
        "        let product_value = product_left_value * product_right_value;",
        "        let scaled_product_value = product_value * product_scale;",
        "        let quotient = scaled_product_value * reciprocal;",
        "        let product_derivative_scale = product_scale * reciprocal;",
        "        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;",
        "        value.value = quotient;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * product_right_value + product_left_value * product_right.node_derivatives[index]) * product_derivative_scale + denominator.node_derivatives[index] * denominator_derivative_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * product_right_value + product_left_value * product_right.branch_derivatives[index]) * product_derivative_scale + denominator.branch_derivatives[index] * denominator_derivative_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_scaled_product3(product_left: Self, product_middle: Self, product_right: Self, product_scale: f64, denominator: Self, denominator_scale: f64) -> Self {",
        "        let mut value = product_left;",
        "        let product_left_value = value.value;",
        "        let product_middle_value = product_middle.value;",
        "        let product_right_value = product_right.value;",
        "        let denominator_value = denominator.value * denominator_scale;",
        "        let reciprocal = 1.0 / denominator_value;",
        "        let left_middle_value = product_left_value * product_middle_value;",
        "        let left_right_value = product_left_value * product_right_value;",
        "        let middle_right_value = product_middle_value * product_right_value;",
        "        let scaled_product_value = left_middle_value * product_right_value * product_scale;",
        "        let quotient = scaled_product_value * reciprocal;",
        "        let product_derivative_scale = product_scale * reciprocal;",
        "        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;",
        "        value.value = quotient;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * middle_right_value + product_middle.node_derivatives[index] * left_right_value + product_right.node_derivatives[index] * left_middle_value) * product_derivative_scale + denominator.node_derivatives[index] * denominator_derivative_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * middle_right_value + product_middle.branch_derivatives[index] * left_right_value + product_right.branch_derivatives[index] * left_middle_value) * product_derivative_scale + denominator.branch_derivatives[index] * denominator_derivative_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_scaled_product3_by_product(product_left: Self, product_middle: Self, product_right: Self, product_scale: f64, denominator_left: Self, denominator_right: Self, denominator_scale: f64) -> Self {",
        "        let mut value = product_left;",
        "        let product_left_value = value.value;",
        "        let product_middle_value = product_middle.value;",
        "        let product_right_value = product_right.value;",
        "        let denominator_left_value = denominator_left.value;",
        "        let denominator_right_value = denominator_right.value;",
        "        let reciprocal = 1.0 / (denominator_left_value * denominator_right_value * denominator_scale);",
        "        let left_middle_value = product_left_value * product_middle_value;",
        "        let left_right_value = product_left_value * product_right_value;",
        "        let middle_right_value = product_middle_value * product_right_value;",
        "        let scaled_product_value = left_middle_value * product_right_value * product_scale;",
        "        let quotient = scaled_product_value * reciprocal;",
        "        let product_derivative_scale = product_scale * reciprocal;",
        "        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;",
        "        value.value = quotient;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = (value.node_derivatives[index] * middle_right_value + product_middle.node_derivatives[index] * left_right_value + product_right.node_derivatives[index] * left_middle_value) * product_derivative_scale + (denominator_left.node_derivatives[index] * denominator_right_value + denominator_left_value * denominator_right.node_derivatives[index]) * denominator_derivative_scale; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = (value.branch_derivatives[index] * middle_right_value + product_middle.branch_derivatives[index] * left_right_value + product_right.branch_derivatives[index] * left_middle_value) * product_derivative_scale + (denominator_left.branch_derivatives[index] * denominator_right_value + denominator_left_value * denominator_right.branch_derivatives[index]) * denominator_derivative_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn rem(left: Self, right: Self) -> Self {",
        "        let quotient = (left.value / right.value).trunc();",
        "        let mut value = left;",
        "        value.value %= right.value;",
        "        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] -= quotient * right.node_derivatives[index]; }",
        "        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] -= quotient * right.branch_derivatives[index]; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn rem_with_scalar(left: Self, right: f64) -> Self {",
        "        let mut value = left;",
        "        value.value %= right;",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn rem_from_scalar(left: f64, right: Self) -> Self {",
        "        let mut value = right;",
        "        let right_value = value.value;",
        "        let quotient = (left / right_value).trunc();",
        "        value.value = left % right_value;",
        "        for derivative in &mut value.node_derivatives { *derivative *= -quotient; }",
        "        for derivative in &mut value.branch_derivatives { *derivative *= -quotient; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn div_from_scalar(scalar: f64, right: Self) -> Self {",
        "        let mut value = right;",
        "        let reciprocal = 1.0 / value.value;",
        "        let quotient = scalar * reciprocal;",
        "        let right_scale = -quotient * reciprocal;",
        "        value.value = quotient;",
        "        for derivative in &mut value.node_derivatives { *derivative *= right_scale; }",
        "        for derivative in &mut value.branch_derivatives { *derivative *= right_scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn scale(mut value: Self, scale: f64) -> Self {",
        "        value.value *= scale;",
        "        for derivative in &mut value.node_derivatives { *derivative *= scale; }",
        "        for derivative in &mut value.branch_derivatives { *derivative *= scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn scale_offset(mut value: Self, scale: f64, offset: f64) -> Self {",
        "        value.value = value.value * scale + offset;",
        "        for derivative in &mut value.node_derivatives { *derivative *= scale; }",
        "        for derivative in &mut value.branch_derivatives { *derivative *= scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn scaled_offset(mut value: Self, offset: f64, scale: f64) -> Self {",
        "        value.value = (value.value + offset) * scale;",
        "        for derivative in &mut value.node_derivatives { *derivative *= scale; }",
        "        for derivative in &mut value.branch_derivatives { *derivative *= scale; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn offset(mut value: Self, offset: f64) -> Self {",
        "        value.value += offset;",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn sub_from_scalar(scalar: f64, mut value: Self) -> Self {",
        "        value.value = scalar - value.value;",
        "        for derivative in &mut value.node_derivatives { *derivative = -*derivative; }",
        "        for derivative in &mut value.branch_derivatives { *derivative = -*derivative; }",
        "        value",
        "    }",
        "",
        "    #[inline]",
        "    fn unary_intrinsic(mut arg: Self, value: f64, derivative_scale: f64) -> Self {",
        "        arg.value = value;",
        "        for derivative in &mut arg.node_derivatives { *derivative *= derivative_scale; }",
        "        for derivative in &mut arg.branch_derivatives { *derivative *= derivative_scale; }",
        "        arg",
        "    }",
        "",
        "    #[inline]",
        "    fn abs(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 }) }",
        "    #[inline]",
        "    fn abs_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; Self::unary_intrinsic(arg, raw.abs(), if raw >= 0.0 { scale } else { -scale }) }",
        "    #[inline]",
        "    fn sqrt(arg: Self) -> Self { let value = arg.value.sqrt(); Self::unary_intrinsic(arg, value, 1.0 / (2.0 * value)) }",
        "    #[inline]",
        "    fn sqrt_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; let value = raw.sqrt(); Self::unary_intrinsic(arg, value, scale / (2.0 * value)) }",
        "    #[inline]",
        "    fn exp(arg: Self) -> Self { let value = arg.value.exp(); Self::unary_intrinsic(arg, value, value) }",
        "    #[inline]",
        "    fn exp_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; let value = raw.exp(); Self::unary_intrinsic(arg, value, value * scale) }",
        "    #[inline]",
        "    fn limexp(arg: Self) -> Self { let raw = arg.value; if raw < 80.0 { let value = raw.exp(); Self::unary_intrinsic(arg, value, value) } else { Self::unary_intrinsic(arg, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX) } }",
        "    #[inline]",
        "    fn limexp_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; if raw < 80.0 { let value = raw.exp(); Self::unary_intrinsic(arg, value, value * scale) } else { Self::unary_intrinsic(arg, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX * scale) } }",
        "    #[inline]",
        "    fn limited_exp(arg: Self) -> Self { let raw = arg.value; if raw > 80.0 { Self::unary_intrinsic(arg, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX) } else if raw < -80.0 { Self::constant(1.804851387e-35) } else { let value = raw.exp(); Self::unary_intrinsic(arg, value, value) } }",
        "    #[inline]",
        "    fn limited_exp_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; if raw > 80.0 { Self::unary_intrinsic(arg, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX * scale) } else if raw < -80.0 { Self::constant(1.804851387e-35) } else { let value = raw.exp(); Self::unary_intrinsic(arg, value, value * scale) } }",
        "    #[inline]",
        "    fn ln(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.ln(), 1.0 / raw) }",
        "    #[inline]",
        "    fn ln_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; Self::unary_intrinsic(arg, raw.ln(), scale / raw) }",
        "    #[inline]",
        "    fn ln_one_plus_exp_raw(raw: f64) -> (f64, f64) { if raw > 0.0 { (raw + (-raw).exp().ln_1p(), 1.0 / (1.0 + (-raw).exp())) } else { let exp = raw.exp(); (exp.ln_1p(), exp / (1.0 + exp)) } }",
        "    #[inline]",
        "    fn ln_one_plus_exp(arg: Self) -> Self { let raw = arg.value; let (value, derivative_scale) = Self::ln_one_plus_exp_raw(raw); Self::unary_intrinsic(arg, value, derivative_scale) }",
        "    #[inline]",
        "    fn log10(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.log10(), 1.0 / (raw * std::f64::consts::LN_10)) }",
        "    #[inline]",
        "    fn sin(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.sin(), raw.cos()) }",
        "    #[inline]",
        "    fn cos(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.cos(), -raw.sin()) }",
        "    #[inline]",
        "    fn tan(arg: Self) -> Self { let raw = arg.value; let cos = raw.cos(); Self::unary_intrinsic(arg, raw.tan(), 1.0 / (cos * cos)) }",
        "    #[inline]",
        "    fn atan(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.atan(), 1.0 / (1.0 + raw * raw)) }",
        "    #[inline]",
        "    fn sinh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.sinh(), raw.cosh()) }",
        "    #[inline]",
        "    fn cosh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.cosh(), raw.sinh()) }",
        "    #[inline]",
        "    fn tanh(arg: Self) -> Self { let raw = arg.value; let cosh = raw.cosh(); Self::unary_intrinsic(arg, raw.tanh(), 1.0 / (cosh * cosh)) }",
        "    #[inline]",
        "    fn tanh_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; let cosh = raw.cosh(); Self::unary_intrinsic(arg, raw.tanh(), scale / (cosh * cosh)) }",
        "    #[inline]",
        "    fn asinh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.asinh(), 1.0 / ((raw * raw) + 1.0).sqrt()) }",
        "    #[inline]",
        "    fn acosh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.acosh(), 1.0 / ((raw - 1.0).sqrt() * (raw + 1.0).sqrt())) }",
        "    #[inline]",
        "    fn atanh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.atanh(), 1.0 / (1.0 - raw * raw)) }",
        "    #[inline]",
        "    fn floor(arg: Self) -> Self { Self::constant(arg.value.floor()) }",
        "    #[inline]",
        "    fn ceil(arg: Self) -> Self { Self::constant(arg.value.ceil()) }",
        "    #[inline]",
        "    fn pow_derivative(value: f64, base: f64, exponent: f64, dbase: f64, dexponent: f64) -> f64 {",
        "        if dexponent == 0.0 && exponent.is_finite() && exponent.fract() == 0.0 {",
        "            if exponent == 0.0 { 0.0 } else { exponent * base.powf(exponent - 1.0) * dbase }",
        "        } else {",
        "            value * (dexponent * base.ln() + exponent * (dbase / base))",
        "        }",
        "    }",
        "    #[inline]",
        "    fn powf(left: Self, exponent: f64) -> Self {",
        "        let base = left.value;",
        "        let value = base.powf(exponent);",
        "        let mut result = left;",
        "        result.value = value;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = Self::pow_derivative(value, base, exponent, result.node_derivatives[index], 0.0); }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = Self::pow_derivative(value, base, exponent, result.branch_derivatives[index], 0.0); }",
        "        result",
        "    }",
        "    #[inline]",
        "    fn pow_from_scalar(base: f64, right: Self) -> Self {",
        "        let exponent = right.value;",
        "        let value = base.powf(exponent);",
        "        let mut result = right;",
        "        result.value = value;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = Self::pow_derivative(value, base, exponent, 0.0, result.node_derivatives[index]); }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = Self::pow_derivative(value, base, exponent, 0.0, result.branch_derivatives[index]); }",
        "        result",
        "    }",
        "    #[inline]",
        "    fn pow(left: Self, right: Self) -> Self {",
        "        let base = left.value;",
        "        let exponent = right.value;",
        "        let value = base.powf(exponent);",
        "        let mut result = left;",
        "        result.value = value;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = Self::pow_derivative(value, base, exponent, result.node_derivatives[index], right.node_derivatives[index]); }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = Self::pow_derivative(value, base, exponent, result.branch_derivatives[index], right.branch_derivatives[index]); }",
        "        result",
        "    }",
        "    #[inline]",
        "    fn min(left: Self, right: Self) -> Self { if left.value <= right.value { left } else { right } }",
        "    #[inline]",
        "    fn min_with_scalar(left: Self, right: f64) -> Self { if left.value <= right { left } else { Self::constant(right) } }",
        "    #[inline]",
        "    fn min_from_scalar(left: f64, right: Self) -> Self { if left <= right.value { Self::constant(left) } else { right } }",
        "    #[inline]",
        "    fn max(left: Self, right: Self) -> Self { if left.value >= right.value { left } else { right } }",
        "    #[inline]",
        "    fn max_with_scalar(left: Self, right: f64) -> Self { if left.value >= right { left } else { Self::constant(right) } }",
        "    #[inline]",
        "    fn max_from_scalar(left: f64, right: Self) -> Self { if left >= right.value { Self::constant(left) } else { right } }",
        "    #[inline]",
        "    fn hypot(left: Self, right: Self) -> Self {",
        "        let left_value = left.value;",
        "        let right_value = right.value;",
        "        let value = left_value.hypot(right_value);",
        "        let mut result = left;",
        "        result.value = value;",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = (left_value * result.node_derivatives[index] + right_value * right.node_derivatives[index]) / value; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = (left_value * result.branch_derivatives[index] + right_value * right.branch_derivatives[index]) / value; }",
        "        result",
        "    }",
        "    #[inline]",
        "    fn atan2(y: Self, x: Self) -> Self {",
        "        let y_value = y.value;",
        "        let x_value = x.value;",
        "        let denominator = x_value * x_value + y_value * y_value;",
        "        let mut result = y;",
        "        result.value = y_value.atan2(x_value);",
        "        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = (x_value * result.node_derivatives[index] - y_value * x.node_derivatives[index]) / denominator; }",
        "        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = (x_value * result.branch_derivatives[index] - y_value * x.branch_derivatives[index]) / denominator; }",
        "        result",
        "    }",
        "",
        "    #[inline]",
        "    fn ddt(mut operand: Self, derivative_scale: f64, value: f64) -> Self {",
        "        operand.value = value;",
        "        for derivative in &mut operand.node_derivatives { *derivative *= derivative_scale; }",
        "        for derivative in &mut operand.branch_derivatives { *derivative *= derivative_scale; }",
        "        operand",
        "    }",
        "",
        "    #[inline]",
        "    fn idt(mut operand: Self, derivative_scale: f64, value: f64) -> Self {",
        "        operand.value = value;",
        "        for derivative in &mut operand.node_derivatives { *derivative *= derivative_scale; }",
        "        for derivative in &mut operand.branch_derivatives { *derivative *= derivative_scale; }",
        "        operand",
        "    }",
        "",
        "    #[inline]",
        "    fn ddx_projection(expr: &Self, pos: Option<usize>, neg: Option<usize>) -> f64 {",
        "        let pos = pos.map(|index| expr.node_derivatives[index]).unwrap_or(0.0);",
        "        if let Some(neg) = neg { 0.5 * (pos - expr.node_derivatives[neg]) } else { pos }",
        "    }",
        "}",
        "",
    ]
    .join("\n")
}

pub fn render_runtime_support_module() -> String {
    let mut support = String::new();
    support.push_str("#![allow(dead_code)]\n\n");
    support.push_str("use super::GeneratedEvalContext;\n\n");
    support.push_str("const LIMEXP_MAX: f64 = 5.54062238439351e34;\n\n");
    support.push_str(&generate_scratch_struct());
    support.push('\n');
    support.push_str(&generate_ad_value_struct());
    support.push('\n');

    support = support
        .replace(
            "struct Scratch {",
            "pub(crate) struct Scratch<const VARIABLE_COUNT: usize, const NODE_COUNT: usize, const BRANCH_COUNT: usize> {",
        )
        .replace(
            "impl Scratch {",
            "impl<const VARIABLE_COUNT: usize, const NODE_COUNT: usize, const BRANCH_COUNT: usize> Scratch<VARIABLE_COUNT, NODE_COUNT, BRANCH_COUNT> {",
        )
        .replace(
            "struct ReactiveScratch {",
            "pub(crate) struct ReactiveScratch<const VARIABLE_COUNT: usize, const NODE_COUNT: usize, const BRANCH_COUNT: usize> {",
        )
        .replace(
            "impl ReactiveScratch {",
            "impl<const VARIABLE_COUNT: usize, const NODE_COUNT: usize, const BRANCH_COUNT: usize> ReactiveScratch<VARIABLE_COUNT, NODE_COUNT, BRANCH_COUNT> {",
        )
        .replace(
            "struct AdValue {",
            "pub(crate) struct AdValue<const NODE_COUNT: usize, const BRANCH_COUNT: usize> {",
        )
        .replace(
            "impl AdValue {",
            "impl<const NODE_COUNT: usize, const BRANCH_COUNT: usize> AdValue<NODE_COUNT, BRANCH_COUNT> {",
        )
        .replace("Instance::VARIABLE_COUNT", "VARIABLE_COUNT")
        .replace("Instance::NODE_COUNT", "NODE_COUNT")
        .replace("Instance::BRANCH_COUNT", "BRANCH_COUNT")
        .replace("    values:", "    pub(crate) values:")
        .replace("\n    bool_values:", "\n    pub(crate) bool_values:")
        .replace(
            "    node_derivatives:",
            "    pub(crate) node_derivatives:",
        )
        .replace(
            "    branch_derivatives:",
            "    pub(crate) branch_derivatives:",
        )
        .replace("    reactive_values:", "    pub(crate) reactive_values:")
        .replace(
            "    reactive_node_derivatives:",
            "    pub(crate) reactive_node_derivatives:",
        )
        .replace(
            "    reactive_branch_derivatives:",
            "    pub(crate) reactive_branch_derivatives:",
        )
        .replace("    value:", "    pub(crate) value:")
        .replace("\n    fn ", "\n    pub(crate) fn ")
        .replace("-> AdValue {", "-> AdValue<NODE_COUNT, BRANCH_COUNT> {")
        .replace(
            "AdValue::pow_derivative",
            "AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative",
        )
        .replace(
            "value: &AdValue)",
            "value: &AdValue<NODE_COUNT, BRANCH_COUNT>)",
        )
        .replace(
            "right: AdValue)",
            "right: AdValue<NODE_COUNT, BRANCH_COUNT>)",
        )
        .replace(
            "right: AdValue,",
            "right: AdValue<NODE_COUNT, BRANCH_COUNT>,",
        )
        .replace(
            "left: AdValue,",
            "left: AdValue<NODE_COUNT, BRANCH_COUNT>,",
        )
        .replace(
            "value: AdValue)",
            "value: AdValue<NODE_COUNT, BRANCH_COUNT>)",
        )
        .replace(
            "value: AdValue,",
            "value: AdValue<NODE_COUNT, BRANCH_COUNT>,",
        )
        .replace("            pub(crate) values:", "            values:")
        .replace(
            "            pub(crate) node_derivatives:",
            "            node_derivatives:",
        )
        .replace(
            "            pub(crate) branch_derivatives:",
            "            branch_derivatives:",
        )
        .replace(
            "            pub(crate) reactive_values:",
            "            reactive_values:",
        )
        .replace(
            "            pub(crate) reactive_node_derivatives:",
            "            reactive_node_derivatives:",
        )
        .replace(
            "            pub(crate) reactive_branch_derivatives:",
            "            reactive_branch_derivatives:",
        );

    compact_runtime_support_surface(support)
}

fn compact_runtime_support_surface(mut source: String) -> String {
    for (from, to) in [
        ("reactive_node_derivatives", "rdn"),
        ("reactive_branch_derivatives", "rdb"),
        ("reactive_values", "rv"),
        ("bool_values", "b"),
        ("node_derivatives", "dn"),
        ("branch_derivatives", "db"),
        ("values", "v"),
    ] {
        source = source.replace(from, to);
    }
    source
}

fn emit_stamp_body(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    ddt_slots: &DdtSlots,
    transient_liveness: &TransientLiveness,
    reactive_liveness: &ReactiveLiveness,
    potential_branch_slots: &PotentialBranchSlots,
    reactive: bool,
    helper_prefix: &str,
    common_usage: StampCommonUsage,
    helper_modules: &mut StampHelperModules,
    out: &mut String,
) -> Result<(), RustBackendError> {
    if reactive && ddt_slots.len() == 0 {
        return Ok(());
    }

    let operator_usage = if reactive {
        StampOperatorUsage::default()
    } else {
        StampOperatorUsage::transient(ddt_slots)
    };
    if operator_usage.ddt {
        out.push_str("        let ddt_state_current = self.ddt_state_current.as_mut();\n");
        out.push_str("        let ddt_state_previous = self.ddt_state_previous.as_mut();\n");
        out.push_str("        let ddt_state_initialized = self.ddt_state_initialized.as_mut();\n");
    }
    if operator_usage.idt {
        out.push_str("        let idt_state_current = self.idt_state_current.as_mut();\n");
        out.push_str("        let idt_state_previous = self.idt_state_previous.as_mut();\n");
        out.push_str("        let idt_state_initialized = self.idt_state_initialized.as_mut();\n");
    }
    if operator_usage.has_any() {
        out.push_str("        let ddt_active = timestep.abs() > Instance::DDT_EPSILON;\n");
    }
    if operator_usage.ddt {
        out.push_str("        let ddt_scale = if ddt_active { 1.0 / timestep } else { 0.0 };\n");
    }
    if operator_usage.idt {
        out.push_str("        let idt_scale = if ddt_active { timestep } else { 0.0 };\n");
    }

    let uses_scratch = !artifact.hir.variables.is_empty();
    if uses_scratch {
        let scratch_field = if reactive {
            "reactive_scratch"
        } else {
            "scratch"
        };
        let scratch_type = if reactive {
            "ReactiveScratch"
        } else {
            "Scratch"
        };
        out.push_str(&format!(
            "        let s = match &mut self.{scratch_field} {{\n"
        ));
        out.push_str("            Some(buf) => buf.as_mut(),\n");
        out.push_str(&format!(
            "            slot @ None => slot.insert({scratch_type}::new_box()).as_mut(),\n"
        ));
        out.push_str("        };\n");
    }
    let mut variables = if uses_scratch {
        emit_variable_initializers(
            artifact,
            variable_fields,
            potential_branch_slots.current_slots().len(),
            reactive,
            reactive_liveness,
            out,
        )
    } else {
        HashMap::new()
    };
    let assignment_chunks = emit_assignment_statement_chunks(
        artifact,
        parameter_fields,
        variable_fields,
        &mut variables,
        ddt_slots,
        potential_branch_slots.current_slots(),
        transient_liveness,
        reactive,
        reactive_liveness,
    )?;
    emit_chunked_stamp_helpers(
        helper_prefix,
        reactive,
        assignment_chunks,
        common_usage,
        operator_usage,
        helper_modules,
        out,
    );

    if !reactive {
        for (slot, branch) in potential_branch_slots.branches().iter().enumerate() {
            out.push_str("        stamper.stamp_potential_branch_local(\n");
            out.push_str(&format!(
                "            {},\n",
                optional_node_local_expr(branch.pos_node)
            ));
            out.push_str(&format!(
                "            {},\n",
                optional_node_local_expr(branch.neg_node)
            ));
            out.push_str(&format!("            {slot},\n"));
            out.push_str("            self.multiplicity,\n");
            out.push_str("        );\n");
        }
        if !potential_branch_slots.branches().is_empty() {
            out.push('\n');
        }
    }

    let equation_inline = equation_inline_plan(artifact)?;
    let split_equations = uses_scratch && artifact.mir.equations.len() > 8;
    let mut branch_currents = HashMap::new();
    for (index, equation) in artifact.mir.equations.iter().enumerate() {
        if reactive && !reactive_liveness.is_equation_reactive(equation.id) {
            continue;
        }

        let helper_safe = split_equations && !equation_inline[index];
        let prefix = format!("eq{index}");
        if !reactive
            && should_emit_compact_equation_stamp(
                artifact,
                equation,
                potential_branch_slots.current_slots(),
                equation_inline[index],
            )?
        {
            if helper_safe {
                out.push_str(&format!(
                    "        // __rspice_equation_chunk_start {helper_prefix} {reactive} {index}\n"
                ));
            }
            emit_compact_equation_stamp(
                artifact,
                equation,
                &prefix,
                parameter_fields,
                &variables,
                ddt_slots,
                potential_branch_slots,
                out,
            )?;
            if helper_safe {
                out.push_str("        // __rspice_equation_chunk_end\n");
            }
            continue;
        }
        let lowered = if reactive {
            lower_reactive_expr_with_branch_currents(
                artifact,
                equation.expression.id,
                &prefix,
                parameter_fields,
                &variables,
                ddt_slots,
                &branch_currents,
                potential_branch_slots.current_slots(),
            )?
        } else {
            lower_equation_expr_with_branch_currents(
                artifact,
                equation.expression.id,
                &prefix,
                parameter_fields,
                &variables,
                ddt_slots,
                &branch_currents,
                potential_branch_slots.current_slots(),
            )?
        };
        if reactive {
            if lowered.has_reactive {
                if helper_safe {
                    out.push_str(&format!(
                        "        // __rspice_equation_chunk_start {helper_prefix} {reactive} {index}\n"
                    ));
                }
                for line in &lowered.lines {
                    out.push_str("        ");
                    out.push_str(line);
                    out.push('\n');
                }
                let dense_reactive = should_emit_dense_stamp(
                    &lowered.reactive_derivatives,
                    &lowered.reactive_branch_derivatives,
                );
                match equation.kind {
                    MirEquationKind::Current => {
                        if dense_reactive {
                            emit_dense_derivative_arrays(
                                out,
                                &format!("{prefix}_reactive"),
                                &lowered.reactive_derivatives,
                                &lowered.reactive_branch_derivatives,
                            );
                            out.push_str("        stamper.stamp_current_reactive_dense(\n");
                            out.push_str(&format!(
                                "            {},\n",
                                optional_node_expr(equation.branch.pos_node)
                            ));
                            out.push_str(&format!(
                                "            {},\n",
                                optional_node_expr(equation.branch.neg_node)
                            ));
                            out.push_str("            &self.nodes,\n");
                            out.push_str(&format!(
                                "            &{prefix}_reactive_node_derivatives,\n"
                            ));
                            out.push_str("            &self.branches,\n");
                            out.push_str(&format!(
                                "            &{prefix}_reactive_branch_derivatives,\n"
                            ));
                            out.push_str("            self.multiplicity,\n");
                            out.push_str("        );\n");
                        } else {
                            let emitted_fixed = emit_fixed_sparse_reactive_current_stamp(
                                out,
                                equation.branch.pos_node,
                                equation.branch.neg_node,
                                &lowered.reactive_derivatives,
                                &lowered.reactive_branch_derivatives,
                                Some("self.multiplicity"),
                            );
                            if !emitted_fixed {
                                out.push_str("        stamper.stamp_current_reactive(\n");
                                out.push_str(&format!(
                                    "            {},\n",
                                    optional_node_expr(equation.branch.pos_node)
                                ));
                                out.push_str(&format!(
                                    "            {},\n",
                                    optional_node_expr(equation.branch.neg_node)
                                ));
                                out.push_str("            &[\n");
                                for node_index in 0..artifact.mir.nodes.len() {
                                    if is_zero_derivative(&lowered.reactive_derivatives[node_index])
                                    {
                                        continue;
                                    }
                                    out.push_str(&format!(
                                        "                GeneratedDerivative::node(self.nodes[{node_index}], self.multiplicity * ({})),\n",
                                        lowered.reactive_derivatives[node_index]
                                    ));
                                }
                                for branch_index in 0..lowered.reactive_branch_derivatives.len() {
                                    if is_zero_derivative(
                                        &lowered.reactive_branch_derivatives[branch_index],
                                    ) {
                                        continue;
                                    }
                                    out.push_str(&format!(
                                        "                GeneratedDerivative::branch(self.branches[{branch_index}], self.multiplicity * ({})),\n",
                                        lowered.reactive_branch_derivatives[branch_index]
                                    ));
                                }
                                out.push_str("            ],\n");
                                out.push_str("        );\n");
                            }
                        }
                    }
                    MirEquationKind::Potential => {
                        let slot =
                            potential_branch_slots
                                .slot_for(equation.id)
                                .ok_or_else(|| {
                                    RustBackendError::internal(
                                        artifact.metadata.source_package.as_str(),
                                        artifact.mir.module_name.as_str(),
                                        format!(
                                            "potential equation {} has no generated branch slot",
                                            equation.id
                                        ),
                                    )
                                })?;
                        if dense_reactive {
                            emit_dense_derivative_arrays(
                                out,
                                &format!("{prefix}_reactive"),
                                &lowered.reactive_derivatives,
                                &lowered.reactive_branch_derivatives,
                            );
                            out.push_str("        stamper.stamp_potential_reactive_dense(\n");
                            out.push_str(&format!("            self.branches[{slot}],\n"));
                            out.push_str("            &self.nodes,\n");
                            out.push_str(&format!(
                                "            &{prefix}_reactive_node_derivatives,\n"
                            ));
                            out.push_str("            &self.branches,\n");
                            out.push_str(&format!(
                                "            &{prefix}_reactive_branch_derivatives,\n"
                            ));
                            out.push_str("        );\n");
                        } else {
                            let branch_expr = format!("self.branches[{slot}]");
                            let emitted_fixed = emit_fixed_sparse_reactive_potential_stamp(
                                out,
                                &branch_expr,
                                &lowered.reactive_derivatives,
                                &lowered.reactive_branch_derivatives,
                                None,
                            );
                            if !emitted_fixed {
                                out.push_str("        stamper.stamp_potential_reactive(\n");
                                out.push_str(&format!("            self.branches[{slot}],\n"));
                                out.push_str("            &[\n");
                                for node_index in 0..artifact.mir.nodes.len() {
                                    if is_zero_derivative(&lowered.reactive_derivatives[node_index])
                                    {
                                        continue;
                                    }
                                    out.push_str(&format!(
                                        "                GeneratedDerivative::node(self.nodes[{node_index}], {}),\n",
                                        lowered.reactive_derivatives[node_index]
                                    ));
                                }
                                for branch_index in 0..lowered.reactive_branch_derivatives.len() {
                                    if is_zero_derivative(
                                        &lowered.reactive_branch_derivatives[branch_index],
                                    ) {
                                        continue;
                                    }
                                    out.push_str(&format!(
                                        "                GeneratedDerivative::branch(self.branches[{branch_index}], {}),\n",
                                        lowered.reactive_branch_derivatives[branch_index]
                                    ));
                                }
                                out.push_str("            ],\n");
                                out.push_str("        );\n");
                            }
                        }
                    }
                    MirEquationKind::Indirect => {
                        return Err(unsupported(artifact, "indirect contributions"));
                    }
                }
                if helper_safe {
                    out.push_str("        // __rspice_equation_chunk_end\n");
                }
            }
            continue;
        }

        if helper_safe {
            out.push_str(&format!(
                "        // __rspice_equation_chunk_start {helper_prefix} {reactive} {index}\n"
            ));
        }
        for line in &lowered.lines {
            out.push_str("        ");
            out.push_str(line);
            out.push('\n');
        }

        let value = format!("{prefix}_value");
        out.push_str(&format!("        let {value}: f64 = {};\n", lowered.value));
        let dense_stamp =
            should_emit_dense_stamp(&lowered.derivatives, &lowered.branch_derivatives);
        let mut node_derivatives = Vec::with_capacity(lowered.derivatives.len());
        for (node_index, derivative) in lowered.derivatives.iter().enumerate() {
            if dense_stamp {
                node_derivatives.push(derivative.clone());
            } else if is_zero_derivative(derivative) {
                node_derivatives.push("0.0".to_string());
            } else if is_inline_derivative_expr(derivative) {
                node_derivatives.push(derivative.clone());
            } else {
                let local = format!("{prefix}_d_n{node_index}");
                out.push_str(&format!("        let {local}: f64 = {derivative};\n"));
                node_derivatives.push(local);
            }
        }
        let mut branch_derivatives = Vec::with_capacity(lowered.branch_derivatives.len());
        for (branch_index, derivative) in lowered.branch_derivatives.iter().enumerate() {
            if dense_stamp {
                branch_derivatives.push(derivative.clone());
            } else if is_zero_derivative(derivative) {
                branch_derivatives.push("0.0".to_string());
            } else if is_inline_derivative_expr(derivative) {
                branch_derivatives.push(derivative.clone());
            } else {
                let local = format!("{prefix}_d_b{branch_index}");
                out.push_str(&format!("        let {local}: f64 = {derivative};\n"));
                branch_derivatives.push(local);
            }
        }
        if equation.kind == MirEquationKind::Current {
            cache_named_branch_current(
                artifact,
                equation,
                &mut branch_currents,
                &value,
                &node_derivatives,
                &branch_derivatives,
                &lowered,
            );
        }
        let indexed_dense_stamp =
            dense_stamp && should_emit_indexed_dense_stamp(&node_derivatives, &branch_derivatives);
        match equation.kind {
            MirEquationKind::Current => {
                if indexed_dense_stamp {
                    emit_indexed_dense_derivative_arrays(
                        out,
                        &prefix,
                        &node_derivatives,
                        &branch_derivatives,
                    );
                    out.push_str("        stamper.stamp_current_indexed_dense_local(\n");
                    out.push_str(&format!(
                        "            {},\n",
                        optional_node_local_expr(equation.branch.pos_node)
                    ));
                    out.push_str(&format!(
                        "            {},\n",
                        optional_node_local_expr(equation.branch.neg_node)
                    ));
                    out.push_str(&format!("            self.multiplicity * ({value}),\n"));
                    out.push_str(&format!("            &{prefix}_node_derivative_indices,\n"));
                    out.push_str(&format!("            &{prefix}_node_derivatives,\n"));
                    out.push_str(&format!(
                        "            &{prefix}_branch_derivative_indices,\n"
                    ));
                    out.push_str(&format!("            &{prefix}_branch_derivatives,\n"));
                    out.push_str("            self.multiplicity,\n");
                    out.push_str("        );\n");
                } else if dense_stamp {
                    emit_dense_derivative_arrays(
                        out,
                        &prefix,
                        &node_derivatives,
                        &branch_derivatives,
                    );
                    out.push_str("        stamper.stamp_current_dense_local(\n");
                    out.push_str(&format!(
                        "            {},\n",
                        optional_node_local_expr(equation.branch.pos_node)
                    ));
                    out.push_str(&format!(
                        "            {},\n",
                        optional_node_local_expr(equation.branch.neg_node)
                    ));
                    out.push_str(&format!("            self.multiplicity * ({value}),\n"));
                    out.push_str(&format!("            &{prefix}_node_derivatives,\n"));
                    out.push_str(&format!("            &{prefix}_branch_derivatives,\n"));
                    out.push_str("            self.multiplicity,\n");
                    out.push_str("        );\n");
                } else {
                    let value_expr = format!("self.multiplicity * ({value})");
                    let emitted_fixed = emit_fixed_sparse_current_stamp(
                        out,
                        equation.branch.pos_node,
                        equation.branch.neg_node,
                        &value_expr,
                        &node_derivatives,
                        &branch_derivatives,
                        Some("self.multiplicity"),
                    );
                    if !emitted_fixed {
                        out.push_str("        stamper.stamp_current_local(\n");
                        out.push_str(&format!(
                            "            {},\n",
                            optional_node_local_expr(equation.branch.pos_node)
                        ));
                        out.push_str(&format!(
                            "            {},\n",
                            optional_node_local_expr(equation.branch.neg_node)
                        ));
                        out.push_str(&format!("            self.multiplicity * ({value}),\n"));
                        out.push_str("            &[\n");
                        for (node_index, derivative) in node_derivatives.iter().enumerate() {
                            if is_zero_derivative(derivative) {
                                continue;
                            }
                            out.push_str(&format!(
                                "                GeneratedDerivative::node({node_index}, self.multiplicity * {derivative}),\n"
                            ));
                        }
                        for (branch_index, derivative) in branch_derivatives.iter().enumerate() {
                            if is_zero_derivative(derivative) {
                                continue;
                            }
                            out.push_str(&format!(
                                "                GeneratedDerivative::branch({branch_index}, self.multiplicity * {derivative}),\n"
                            ));
                        }
                        out.push_str("            ],\n");
                        out.push_str("        );\n");
                    }
                }
            }
            MirEquationKind::Potential => {
                let slot = potential_branch_slots
                    .slot_for(equation.id)
                    .ok_or_else(|| {
                        RustBackendError::internal(
                            artifact.metadata.source_package.as_str(),
                            artifact.mir.module_name.as_str(),
                            format!(
                                "potential equation {} has no generated branch slot",
                                equation.id
                            ),
                        )
                    })?;
                if indexed_dense_stamp {
                    emit_indexed_dense_derivative_arrays(
                        out,
                        &prefix,
                        &node_derivatives,
                        &branch_derivatives,
                    );
                    out.push_str("        stamper.stamp_potential_indexed_dense_local(\n");
                    out.push_str(&format!("            {slot},\n"));
                    out.push_str(&format!("            {value},\n"));
                    out.push_str(&format!("            &{prefix}_node_derivative_indices,\n"));
                    out.push_str(&format!("            &{prefix}_node_derivatives,\n"));
                    out.push_str(&format!(
                        "            &{prefix}_branch_derivative_indices,\n"
                    ));
                    out.push_str(&format!("            &{prefix}_branch_derivatives,\n"));
                    out.push_str("        );\n");
                } else if dense_stamp {
                    emit_dense_derivative_arrays(
                        out,
                        &prefix,
                        &node_derivatives,
                        &branch_derivatives,
                    );
                    out.push_str("        stamper.stamp_potential_dense_local(\n");
                    out.push_str(&format!("            {slot},\n"));
                    out.push_str(&format!("            {value},\n"));
                    out.push_str(&format!("            &{prefix}_node_derivatives,\n"));
                    out.push_str(&format!("            &{prefix}_branch_derivatives,\n"));
                    out.push_str("        );\n");
                } else {
                    let branch_expr = format!("{slot}");
                    let emitted_fixed = emit_fixed_sparse_potential_stamp(
                        out,
                        &branch_expr,
                        &value,
                        &node_derivatives,
                        &branch_derivatives,
                        None,
                    );
                    if !emitted_fixed {
                        out.push_str("        stamper.stamp_potential_local(\n");
                        out.push_str(&format!("            {slot},\n"));
                        out.push_str(&format!("            {value},\n"));
                        out.push_str("            &[\n");
                        for (node_index, derivative) in node_derivatives.iter().enumerate() {
                            if is_zero_derivative(derivative) {
                                continue;
                            }
                            out.push_str(&format!(
                                "                GeneratedDerivative::node({node_index}, {derivative}),\n"
                            ));
                        }
                        for (branch_index, derivative) in branch_derivatives.iter().enumerate() {
                            if is_zero_derivative(derivative) {
                                continue;
                            }
                            out.push_str(&format!(
                                "                GeneratedDerivative::branch({branch_index}, {derivative}),\n"
                            ));
                        }
                        out.push_str("            ],\n");
                        out.push_str("        );\n");
                    }
                }
            }
            MirEquationKind::Indirect => {
                return Err(unsupported(artifact, "indirect contributions"));
            }
        }
        if helper_safe {
            out.push_str("        // __rspice_equation_chunk_end\n");
        }
    }
    Ok(())
}

fn should_emit_dense_stamp(node_derivatives: &[String], branch_derivatives: &[String]) -> bool {
    node_derivatives
        .iter()
        .chain(branch_derivatives.iter())
        .filter(|derivative| !is_zero_derivative(derivative))
        .count()
        > DENSE_STAMP_DERIVATIVE_THRESHOLD
}

fn should_emit_indexed_dense_stamp(
    node_derivatives: &[String],
    branch_derivatives: &[String],
) -> bool {
    let total = node_derivatives.len() + branch_derivatives.len();
    let active = node_derivatives
        .iter()
        .chain(branch_derivatives.iter())
        .filter(|derivative| !is_zero_derivative(derivative))
        .count();
    active < total
}

#[derive(Debug, Clone, Copy)]
struct SparseDerivativeTerm<'a> {
    index: usize,
    derivative: &'a str,
}

fn sparse_derivative_terms(derivatives: &[String]) -> Vec<SparseDerivativeTerm<'_>> {
    derivatives
        .iter()
        .enumerate()
        .filter_map(|(index, derivative)| {
            (!is_zero_derivative(derivative)).then_some(SparseDerivativeTerm { index, derivative })
        })
        .collect()
}

fn scaled_derivative_expr(scale: Option<&str>, derivative: &str) -> String {
    let derivative = derivative.trim();
    match scale {
        Some(scale) => format!("{scale} * ({derivative})"),
        None => derivative.to_string(),
    }
}

fn emit_fixed_sparse_current_stamp(
    out: &mut String,
    pos_node: Option<crate::canonical_ir::NodeId>,
    neg_node: Option<crate::canonical_ir::NodeId>,
    value_expr: &str,
    node_derivatives: &[String],
    branch_derivatives: &[String],
    derivative_scale: Option<&str>,
) -> bool {
    let node_terms = sparse_derivative_terms(node_derivatives);
    let branch_terms = sparse_derivative_terms(branch_derivatives);
    if node_terms.is_empty() && branch_terms.is_empty() {
        out.push_str("        stamper.stamp_current_const_local(\n");
        out.push_str(&format!(
            "            {},\n",
            optional_node_local_expr(pos_node)
        ));
        out.push_str(&format!(
            "            {},\n",
            optional_node_local_expr(neg_node)
        ));
        out.push_str(&format!("            {value_expr},\n"));
        out.push_str("        );\n");
        return true;
    }
    if branch_terms.is_empty() && (1..=3).contains(&node_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_current_node{}_local(\n",
            node_terms.len()
        ));
        out.push_str(&format!(
            "            {},\n",
            optional_node_local_expr(pos_node)
        ));
        out.push_str(&format!(
            "            {},\n",
            optional_node_local_expr(neg_node)
        ));
        out.push_str(&format!("            {value_expr},\n"));
        for term in node_terms {
            out.push_str(&format!("            {},\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        out.push_str("        );\n");
        return true;
    }
    if node_terms.is_empty() && (1..=2).contains(&branch_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_current_branch{}_local(\n",
            branch_terms.len()
        ));
        out.push_str(&format!(
            "            {},\n",
            optional_node_local_expr(pos_node)
        ));
        out.push_str(&format!(
            "            {},\n",
            optional_node_local_expr(neg_node)
        ));
        out.push_str(&format!("            {value_expr},\n"));
        for term in branch_terms {
            out.push_str(&format!("            {},\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        out.push_str("        );\n");
        return true;
    }
    if branch_terms.len() == 1 && (1..=2).contains(&node_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_current_node{}_branch1_local(\n",
            node_terms.len()
        ));
        out.push_str(&format!(
            "            {},\n",
            optional_node_local_expr(pos_node)
        ));
        out.push_str(&format!(
            "            {},\n",
            optional_node_local_expr(neg_node)
        ));
        out.push_str(&format!("            {value_expr},\n"));
        for term in node_terms {
            out.push_str(&format!("            {},\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        let term = branch_terms[0];
        out.push_str(&format!("            {},\n", term.index));
        out.push_str(&format!(
            "            {},\n",
            scaled_derivative_expr(derivative_scale, term.derivative)
        ));
        out.push_str("        );\n");
        return true;
    }
    false
}

fn emit_fixed_sparse_potential_stamp(
    out: &mut String,
    branch_expr: &str,
    value_expr: &str,
    node_derivatives: &[String],
    branch_derivatives: &[String],
    derivative_scale: Option<&str>,
) -> bool {
    let node_terms = sparse_derivative_terms(node_derivatives);
    let branch_terms = sparse_derivative_terms(branch_derivatives);
    if node_terms.is_empty() && branch_terms.is_empty() {
        out.push_str("        stamper.stamp_potential_const_local(\n");
        out.push_str(&format!("            {branch_expr},\n"));
        out.push_str(&format!("            {value_expr},\n"));
        out.push_str("        );\n");
        return true;
    }
    if branch_terms.is_empty() && (1..=2).contains(&node_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_potential_node{}_local(\n",
            node_terms.len()
        ));
        out.push_str(&format!("            {branch_expr},\n"));
        out.push_str(&format!("            {value_expr},\n"));
        for term in node_terms {
            out.push_str(&format!("            {},\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        out.push_str("        );\n");
        return true;
    }
    if node_terms.is_empty() && (1..=2).contains(&branch_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_potential_branch{}_local(\n",
            branch_terms.len()
        ));
        out.push_str(&format!("            {branch_expr},\n"));
        out.push_str(&format!("            {value_expr},\n"));
        for term in branch_terms {
            out.push_str(&format!("            {},\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        out.push_str("        );\n");
        return true;
    }
    if branch_terms.len() == 1 && (1..=2).contains(&node_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_potential_node{}_branch1_local(\n",
            node_terms.len()
        ));
        out.push_str(&format!("            {branch_expr},\n"));
        out.push_str(&format!("            {value_expr},\n"));
        for term in node_terms {
            out.push_str(&format!("            {},\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        let term = branch_terms[0];
        out.push_str(&format!("            {},\n", term.index));
        out.push_str(&format!(
            "            {},\n",
            scaled_derivative_expr(derivative_scale, term.derivative)
        ));
        out.push_str("        );\n");
        return true;
    }
    false
}

fn emit_fixed_sparse_reactive_current_stamp(
    out: &mut String,
    pos_node: Option<crate::canonical_ir::NodeId>,
    neg_node: Option<crate::canonical_ir::NodeId>,
    node_derivatives: &[String],
    branch_derivatives: &[String],
    derivative_scale: Option<&str>,
) -> bool {
    let node_terms = sparse_derivative_terms(node_derivatives);
    let branch_terms = sparse_derivative_terms(branch_derivatives);
    if node_terms.is_empty() && branch_terms.is_empty() {
        return true;
    }
    if branch_terms.is_empty() && (1..=3).contains(&node_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_current_reactive_node{}(\n",
            node_terms.len()
        ));
        out.push_str(&format!("            {},\n", optional_node_expr(pos_node)));
        out.push_str(&format!("            {},\n", optional_node_expr(neg_node)));
        for term in node_terms {
            out.push_str(&format!("            self.nodes[{}],\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        out.push_str("        );\n");
        return true;
    }
    if node_terms.is_empty() && (1..=2).contains(&branch_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_current_reactive_branch{}(\n",
            branch_terms.len()
        ));
        out.push_str(&format!("            {},\n", optional_node_expr(pos_node)));
        out.push_str(&format!("            {},\n", optional_node_expr(neg_node)));
        for term in branch_terms {
            out.push_str(&format!("            self.branches[{}],\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        out.push_str("        );\n");
        return true;
    }
    if branch_terms.len() == 1 && (1..=2).contains(&node_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_current_reactive_node{}_branch1(\n",
            node_terms.len()
        ));
        out.push_str(&format!("            {},\n", optional_node_expr(pos_node)));
        out.push_str(&format!("            {},\n", optional_node_expr(neg_node)));
        for term in node_terms {
            out.push_str(&format!("            self.nodes[{}],\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        let term = branch_terms[0];
        out.push_str(&format!("            self.branches[{}],\n", term.index));
        out.push_str(&format!(
            "            {},\n",
            scaled_derivative_expr(derivative_scale, term.derivative)
        ));
        out.push_str("        );\n");
        return true;
    }
    false
}

fn emit_fixed_sparse_reactive_potential_stamp(
    out: &mut String,
    branch_expr: &str,
    node_derivatives: &[String],
    branch_derivatives: &[String],
    derivative_scale: Option<&str>,
) -> bool {
    let node_terms = sparse_derivative_terms(node_derivatives);
    let branch_terms = sparse_derivative_terms(branch_derivatives);
    if node_terms.is_empty() && branch_terms.is_empty() {
        return true;
    }
    if branch_terms.is_empty() && (1..=2).contains(&node_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_potential_reactive_node{}(\n",
            node_terms.len()
        ));
        out.push_str(&format!("            {branch_expr},\n"));
        for term in node_terms {
            out.push_str(&format!("            self.nodes[{}],\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        out.push_str("        );\n");
        return true;
    }
    if node_terms.is_empty() && (1..=2).contains(&branch_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_potential_reactive_branch{}(\n",
            branch_terms.len()
        ));
        out.push_str(&format!("            {branch_expr},\n"));
        for term in branch_terms {
            out.push_str(&format!("            self.branches[{}],\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        out.push_str("        );\n");
        return true;
    }
    if branch_terms.len() == 1 && (1..=2).contains(&node_terms.len()) {
        out.push_str(&format!(
            "        stamper.stamp_potential_reactive_node{}_branch1(\n",
            node_terms.len()
        ));
        out.push_str(&format!("            {branch_expr},\n"));
        for term in node_terms {
            out.push_str(&format!("            self.nodes[{}],\n", term.index));
            out.push_str(&format!(
                "            {},\n",
                scaled_derivative_expr(derivative_scale, term.derivative)
            ));
        }
        let term = branch_terms[0];
        out.push_str(&format!("            self.branches[{}],\n", term.index));
        out.push_str(&format!(
            "            {},\n",
            scaled_derivative_expr(derivative_scale, term.derivative)
        ));
        out.push_str("        );\n");
        return true;
    }
    false
}

fn should_emit_compact_equation_stamp(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
    force_inline: bool,
) -> Result<bool, RustBackendError> {
    if force_inline || equation.kind == MirEquationKind::Indirect {
        return Ok(false);
    }
    let derivative_axis_count =
        artifact.mir.nodes.len() + branch_derivative_axis_count(branch_current_unknowns);
    if derivative_axis_count == 0 {
        return Ok(false);
    }
    Ok(expression_node_count(artifact, equation.expression.id)?
        >= COMPACT_EQUATION_EXPR_NODE_THRESHOLD)
}

#[allow(clippy::too_many_arguments)]
fn emit_compact_equation_stamp(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    prefix: &str,
    parameter_fields: &HashMap<String, String>,
    variables: &HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
    potential_branch_slots: &PotentialBranchSlots,
    out: &mut String,
) -> Result<(), RustBackendError> {
    let mut emitter = CompactAdEmitter {
        artifact,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        branch_current_unknowns: potential_branch_slots.current_slots(),
        emitted: HashMap::new(),
        lines: Vec::new(),
    };
    let lowered = emitter.lower(equation.expression.id)?;
    for line in emitter.lines {
        out.push_str("        ");
        out.push_str(&line);
        out.push('\n');
    }

    let ad_value = format!("{prefix}_ad");
    out.push_str(&format!("        let {ad_value}: AdValue = {lowered};\n"));
    match equation.kind {
        MirEquationKind::Current => {
            out.push_str("        stamper.stamp_current_dense_local(\n");
            out.push_str(&format!(
                "            {},\n",
                optional_node_local_expr(equation.branch.pos_node)
            ));
            out.push_str(&format!(
                "            {},\n",
                optional_node_local_expr(equation.branch.neg_node)
            ));
            out.push_str(&format!(
                "            self.multiplicity * {ad_value}.value,\n"
            ));
            out.push_str(&format!("            &{ad_value}.node_derivatives,\n"));
            out.push_str(&format!("            &{ad_value}.branch_derivatives,\n"));
            out.push_str("            self.multiplicity,\n");
            out.push_str("        );\n");
        }
        MirEquationKind::Potential => {
            let slot = potential_branch_slots
                .slot_for(equation.id)
                .ok_or_else(|| {
                    RustBackendError::internal(
                        artifact.metadata.source_package.as_str(),
                        artifact.mir.module_name.as_str(),
                        format!(
                            "potential equation {} has no generated branch slot",
                            equation.id
                        ),
                    )
                })?;
            out.push_str("        stamper.stamp_potential_dense_local(\n");
            out.push_str(&format!("            {slot},\n"));
            out.push_str(&format!("            {ad_value}.value,\n"));
            out.push_str(&format!("            &{ad_value}.node_derivatives,\n"));
            out.push_str(&format!("            &{ad_value}.branch_derivatives,\n"));
            out.push_str("        );\n");
        }
        MirEquationKind::Indirect => {
            return Err(unsupported(artifact, "indirect contributions"));
        }
    }
    Ok(())
}

fn emit_dense_derivative_arrays(
    out: &mut String,
    prefix: &str,
    node_derivatives: &[String],
    branch_derivatives: &[String],
) {
    out.push_str(&format!(
        "        let {prefix}_node_derivatives: [f64; {}] = [{}];\n",
        node_derivatives.len(),
        node_derivatives.join(", ")
    ));
    out.push_str(&format!(
        "        let {prefix}_branch_derivatives: [f64; {}] = [{}];\n",
        branch_derivatives.len(),
        branch_derivatives.join(", ")
    ));
}

fn emit_indexed_dense_derivative_arrays(
    out: &mut String,
    prefix: &str,
    node_derivatives: &[String],
    branch_derivatives: &[String],
) {
    let node_terms = indexed_derivative_terms(node_derivatives);
    let branch_terms = indexed_derivative_terms(branch_derivatives);
    let node_indices = node_terms
        .iter()
        .map(|(index, _)| index.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let node_values = node_terms
        .iter()
        .map(|(_, derivative)| derivative.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let branch_indices = branch_terms
        .iter()
        .map(|(index, _)| index.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let branch_values = branch_terms
        .iter()
        .map(|(_, derivative)| derivative.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    out.push_str(&format!(
        "        let {prefix}_node_derivative_indices: [usize; {}] = [{}];\n",
        node_terms.len(),
        node_indices
    ));
    out.push_str(&format!(
        "        let {prefix}_node_derivatives: [f64; {}] = [{}];\n",
        node_terms.len(),
        node_values
    ));
    out.push_str(&format!(
        "        let {prefix}_branch_derivative_indices: [usize; {}] = [{}];\n",
        branch_terms.len(),
        branch_indices
    ));
    out.push_str(&format!(
        "        let {prefix}_branch_derivatives: [f64; {}] = [{}];\n",
        branch_terms.len(),
        branch_values
    ));
}

fn indexed_derivative_terms(derivatives: &[String]) -> Vec<(usize, String)> {
    derivatives
        .iter()
        .enumerate()
        .filter_map(|(index, derivative)| {
            (!is_zero_derivative(derivative)).then(|| (index, derivative.clone()))
        })
        .collect()
}

fn cache_named_branch_current(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
    branch_currents: &mut HashMap<String, LoweredVariable>,
    value: &str,
    derivatives: &[String],
    branch_derivatives: &[String],
    lowered: &LoweredExpr,
) {
    let Some(branch_name) = declared_contribution_branch_name(artifact, equation) else {
        return;
    };

    let next = LoweredVariable {
        value: value.to_string(),
        condition: None,
        derivatives: derivatives.to_vec(),
        branch_derivatives: branch_derivatives.to_vec(),
        has_reactive: lowered.has_reactive,
        reactive_value: lowered.reactive_value.clone(),
        reactive_derivatives: lowered.reactive_derivatives.clone(),
        reactive_branch_derivatives: lowered.reactive_branch_derivatives.clone(),
    };

    branch_currents
        .entry(branch_name)
        .and_modify(|current| {
            current.value = format!("({} + {})", current.value, next.value);
            for (current_derivative, next_derivative) in
                current.derivatives.iter_mut().zip(next.derivatives.iter())
            {
                *current_derivative = format!("({current_derivative} + {next_derivative})");
            }
            for (current_derivative, next_derivative) in current
                .branch_derivatives
                .iter_mut()
                .zip(next.branch_derivatives.iter())
            {
                *current_derivative = format!("({current_derivative} + {next_derivative})");
            }
            if next.has_reactive {
                if current.has_reactive {
                    current.reactive_value =
                        format!("({} + {})", current.reactive_value, next.reactive_value);
                    for (current_derivative, next_derivative) in current
                        .reactive_derivatives
                        .iter_mut()
                        .zip(next.reactive_derivatives.iter())
                    {
                        *current_derivative = format!("({current_derivative} + {next_derivative})");
                    }
                    for (current_derivative, next_derivative) in current
                        .reactive_branch_derivatives
                        .iter_mut()
                        .zip(next.reactive_branch_derivatives.iter())
                    {
                        *current_derivative = format!("({current_derivative} + {next_derivative})");
                    }
                } else {
                    current.has_reactive = true;
                    current.reactive_value = next.reactive_value.clone();
                    current.reactive_derivatives = next.reactive_derivatives.clone();
                    current.reactive_branch_derivatives = next.reactive_branch_derivatives.clone();
                }
            }
        })
        .or_insert(next);
}

fn declared_contribution_branch_name(
    artifact: &CanonicalIrArtifact,
    equation: &MirEquation,
) -> Option<String> {
    if let Some(name) = equation.branch.declared_name.as_deref() {
        return Some(name.to_string());
    }
    let contribution = artifact
        .hir
        .contributions
        .get(usize::from(equation.contribution))?;
    let branch_name = contribution.branch.as_str();
    if let Some(branch) = artifact
        .mir
        .branches
        .iter()
        .find(|branch| branch.name.as_str() == branch_name)
    {
        return Some(branch.name.to_string());
    }

    let mut matches = artifact.mir.branches.iter().filter(|branch| {
        branch.pos_node == equation.branch.pos_node && branch.neg_node == equation.branch.neg_node
    });
    let first = matches.next()?;
    if matches.next().is_none() {
        Some(first.name.to_string())
    } else {
        None
    }
}

fn variable_local_names(artifact: &CanonicalIrArtifact) -> HashMap<String, String> {
    let names = artifact
        .hir
        .variables
        .iter()
        .map(|variable| variable.name.to_string())
        .collect::<Vec<_>>();
    unique_identifiers(&names)
}

fn emit_variable_initializers(
    artifact: &CanonicalIrArtifact,
    _variable_fields: &HashMap<String, String>,
    branch_axis_count: usize,
    reactive: bool,
    reactive_liveness: &ReactiveLiveness,
    out: &mut String,
) -> HashMap<String, LoweredVariable> {
    let mut variables = HashMap::new();
    for variable in &artifact.hir.variables {
        if reactive && !reactive_liveness.is_variable_live(variable.name.as_str()) {
            continue;
        }
        let variable_index = usize::from(variable.id);
        let mut derivatives = Vec::with_capacity(artifact.mir.nodes.len());
        let mut branch_derivatives = Vec::with_capacity(branch_axis_count);
        let reactive_value = format!("scratch.reactive_values[{variable_index}]");
        let mut reactive_derivatives = Vec::with_capacity(artifact.mir.nodes.len());
        let mut reactive_branch_derivatives = Vec::with_capacity(branch_axis_count);
        for node_index in 0..artifact.mir.nodes.len() {
            derivatives.push(format!(
                "scratch.node_derivatives[{variable_index}][{node_index}]"
            ));
            if reactive {
                reactive_derivatives.push(format!(
                    "scratch.reactive_node_derivatives[{variable_index}][{node_index}]"
                ));
            } else {
                reactive_derivatives.push("0.0".to_string());
            }
        }
        for branch_index in 0..branch_axis_count {
            branch_derivatives.push(format!(
                "scratch.branch_derivatives[{variable_index}][{branch_index}]"
            ));
            if reactive {
                reactive_branch_derivatives.push(format!(
                    "scratch.reactive_branch_derivatives[{variable_index}][{branch_index}]"
                ));
            } else {
                reactive_branch_derivatives.push("0.0".to_string());
            }
        }
        variables.insert(
            variable.name.to_string(),
            LoweredVariable {
                value: format!("scratch.values[{variable_index}]"),
                condition: None,
                derivatives,
                branch_derivatives,
                has_reactive: false,
                reactive_value: if reactive {
                    reactive_value
                } else {
                    "0.0".to_string()
                },
                reactive_derivatives,
                reactive_branch_derivatives,
            },
        );
    }
    if !artifact.hir.variables.is_empty() {
        out.push('\n');
    }
    variables
}

fn emit_assignment_statement_chunks(
    artifact: &CanonicalIrArtifact,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    ddt_slots: &DdtSlots,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
    transient_liveness: &TransientLiveness,
    reactive: bool,
    reactive_liveness: &ReactiveLiveness,
) -> Result<Vec<String>, RustBackendError> {
    let mut chunks = Vec::new();
    for (index, statement) in artifact.hir.statements.iter().enumerate() {
        let statement_prefix = format!("assign{index}");
        let mut chunk = String::new();
        emit_statement_list(
            artifact,
            std::slice::from_ref(statement),
            parameter_fields,
            variable_fields,
            variables,
            &mut chunk,
            ddt_slots,
            branch_current_unknowns,
            transient_liveness,
            reactive,
            reactive_liveness,
            "        ",
            &statement_prefix,
        )?;
        if !chunk.is_empty() {
            chunk.push('\n');
            chunks.push(chunk);
        }
    }
    Ok(chunks)
}

fn emit_chunked_stamp_helpers(
    helper_prefix: &str,
    reactive: bool,
    chunks: Vec<String>,
    common_usage: StampCommonUsage,
    operator_usage: StampOperatorUsage,
    helper_modules: &mut StampHelperModules,
    out: &mut String,
) {
    if chunks.is_empty() {
        return;
    }

    let total_lines: usize = chunks.iter().map(|chunk| chunk.lines().count()).sum();
    if total_lines <= MAX_STAMP_HELPER_LINES {
        for chunk in chunks {
            out.push_str(&chunk);
        }
        return;
    }

    let mut block_index = 0usize;
    let mut block = String::new();
    let mut block_lines = 0usize;
    for chunk in chunks {
        let chunk_lines = chunk.lines().count();
        if block_lines > 0 && block_lines + chunk_lines > MAX_STAMP_HELPER_LINES {
            let helper_common_usage = common_usage.for_helper_block(&block);
            let helper_operator_usage = operator_usage.for_helper_block(&block);
            emit_stamp_helper_method(
                helper_prefix,
                reactive,
                block_index,
                &block,
                helper_common_usage,
                helper_operator_usage,
                helper_modules,
            );
            let helper_args = stamp_helper_call_args(helper_common_usage, helper_operator_usage);
            out.push_str(&format!(
                "        Self::{helper_prefix}_block_{block_index}({helper_args});\n"
            ));
            block.clear();
            block_lines = 0;
            block_index += 1;
        }
        block_lines += chunk_lines;
        block.push_str(&chunk);
    }
    if !block.is_empty() {
        let helper_common_usage = common_usage.for_helper_block(&block);
        let helper_operator_usage = operator_usage.for_helper_block(&block);
        emit_stamp_helper_method(
            helper_prefix,
            reactive,
            block_index,
            &block,
            helper_common_usage,
            helper_operator_usage,
            helper_modules,
        );
        let helper_args = stamp_helper_call_args(helper_common_usage, helper_operator_usage);
        out.push_str(&format!(
            "        Self::{helper_prefix}_block_{block_index}({helper_args});\n"
        ));
    }
    out.push('\n');
}

fn split_marked_equation_chunks(
    out: &mut String,
    helper_modules: &mut StampHelperModules,
    common_usage: StampCommonUsage,
    operator_usage: StampOperatorUsage,
) {
    const START: &str = "// __rspice_equation_chunk_start ";
    const END: &str = "// __rspice_equation_chunk_end";

    struct PendingEquationHelper {
        helper_prefix: String,
        reactive: bool,
        block_index: usize,
        block: String,
        block_lines: usize,
    }

    fn flush_pending_equation_helper(
        pending: &mut Option<PendingEquationHelper>,
        rewritten: &mut String,
        helper_modules: &mut StampHelperModules,
        common_usage: StampCommonUsage,
        operator_usage: StampOperatorUsage,
    ) {
        let Some(pending) = pending.take() else {
            return;
        };
        let helper_operator_usage = if pending.reactive {
            StampOperatorUsage::default()
        } else {
            operator_usage
        }
        .for_helper_block(&pending.block);
        let helper_common_usage = common_usage.for_helper_block(&pending.block);
        let method_prefix = format!("{}_equations", pending.helper_prefix);
        emit_stamp_helper_method(
            &method_prefix,
            pending.reactive,
            pending.block_index,
            &pending.block,
            helper_common_usage,
            helper_operator_usage,
            helper_modules,
        );
        let helper_args = stamp_helper_call_args(helper_common_usage, helper_operator_usage);
        rewritten.push_str(&format!(
            "        Self::{method_prefix}_block_{}({helper_args});\n",
            pending.block_index
        ));
    }

    let mut rewritten = String::with_capacity(out.len());
    let mut lines = out.lines();
    let mut pending: Option<PendingEquationHelper> = None;
    let mut next_block_indices: HashMap<(String, bool), usize> = HashMap::new();
    while let Some(line) = lines.next() {
        let Some(marker) = line.trim_start().strip_prefix(START) else {
            flush_pending_equation_helper(
                &mut pending,
                &mut rewritten,
                helper_modules,
                common_usage,
                operator_usage,
            );
            rewritten.push_str(line);
            rewritten.push('\n');
            continue;
        };

        let mut parts = marker.split_whitespace();
        let Some(helper_prefix) = parts.next() else {
            flush_pending_equation_helper(
                &mut pending,
                &mut rewritten,
                helper_modules,
                common_usage,
                operator_usage,
            );
            continue;
        };
        let reactive = matches!(parts.next(), Some("true"));
        if parts.next().is_none() {
            flush_pending_equation_helper(
                &mut pending,
                &mut rewritten,
                helper_modules,
                common_usage,
                operator_usage,
            );
            continue;
        }
        let mut block = String::new();
        for block_line in lines.by_ref() {
            if block_line.trim_start() == END {
                break;
            }
            block.push_str(block_line);
            block.push('\n');
        }
        let block_lines = block.lines().count();

        let same_pending = pending
            .as_ref()
            .map(|pending| pending.helper_prefix == helper_prefix && pending.reactive == reactive)
            .unwrap_or(false);
        let would_overflow = pending
            .as_ref()
            .map(|pending| {
                pending.block_lines > 0
                    && pending.block_lines + block_lines > MAX_STAMP_HELPER_LINES
            })
            .unwrap_or(false);
        if !same_pending || would_overflow {
            flush_pending_equation_helper(
                &mut pending,
                &mut rewritten,
                helper_modules,
                common_usage,
                operator_usage,
            );
        }

        match &mut pending {
            Some(pending) => {
                pending.block.push_str(&block);
                pending.block_lines += block_lines;
            }
            None => {
                let key = (helper_prefix.to_string(), reactive);
                let block_index = next_block_indices.entry(key).or_insert(0);
                pending = Some(PendingEquationHelper {
                    helper_prefix: helper_prefix.to_string(),
                    reactive,
                    block_index: *block_index,
                    block,
                    block_lines,
                });
                *block_index += 1;
            }
        }
    }
    flush_pending_equation_helper(
        &mut pending,
        &mut rewritten,
        helper_modules,
        common_usage,
        operator_usage,
    );

    *out = rewritten;
}

fn emit_stamp_helper_method(
    helper_prefix: &str,
    reactive: bool,
    block_index: usize,
    block: &str,
    common_usage: StampHelperCommonUsage,
    operator_usage: StampOperatorUsage,
    helper_modules: &mut StampHelperModules,
) {
    let stamper_type = if reactive {
        "GeneratedReactiveStamper"
    } else {
        "GeneratedStamper"
    };
    let scratch_type = if reactive {
        "ReactiveScratch"
    } else {
        "Scratch"
    };
    let common_params = stamp_helper_common_params(common_usage, stamper_type, scratch_type);
    let operator_params = stamp_helper_operator_params(operator_usage);
    let mut method = format!(
        "\n    pub(super) fn {helper_prefix}_block_{block_index}(\n{common_params}{operator_params}    ) {{\n"
    );
    method.push_str(block);
    method.push_str("    }\n");
    helper_modules.push_method(method, reactive);
}

#[derive(Debug, Default)]
struct StampHelperModules {
    modules: Vec<StampHelperModule>,
}

impl StampHelperModules {
    fn push_method(&mut self, method_part: String, reactive: bool) {
        let needs_new_module = self
            .modules
            .last()
            .map(|module| module.method_count >= MAX_STAMP_HELPERS_PER_MODULE)
            .unwrap_or(true);
        if needs_new_module {
            let index = self.modules.len();
            self.modules.push(StampHelperModule {
                module_name: format!("stamp_blocks_{index}"),
                contents: String::new(),
                method_count: 0,
                uses_reactive_scratch: false,
                uses_transient_scratch: false,
            });
        }

        let module = self
            .modules
            .last_mut()
            .expect("helper module must exist after allocation");
        if method_part.starts_with("\n    pub(super) fn ") {
            module.method_count += 1;
            if reactive {
                module.uses_reactive_scratch = true;
            } else {
                module.uses_transient_scratch = true;
            }
        }
        module.contents.push_str(&method_part);
    }

    fn finish(self) -> Vec<GeneratedRustFile> {
        self.modules
            .into_iter()
            .map(|module| GeneratedRustFile {
                relative_path: format!("{}.rs", module.module_name),
                contents: module.finish(),
            })
            .collect()
    }
}

#[derive(Debug)]
struct StampHelperModule {
    module_name: String,
    contents: String,
    method_count: usize,
    uses_reactive_scratch: bool,
    uses_transient_scratch: bool,
}

impl StampHelperModule {
    fn finish(self) -> String {
        let mut imports = vec![
            "A",
            "ddt_jacobian",
            "eval_ddt",
            "eval_idt",
            "GeneratedDerivative",
            "GeneratedEvalContext",
            "GeneratedReactiveStamper",
            "GeneratedStamper",
            "idt_jacobian",
        ];
        if self.uses_reactive_scratch {
            imports.push("ReactiveScratch");
        }
        if self.uses_transient_scratch {
            imports.push("Scratch");
        }
        imports.push("LIMEXP_MAX");
        imports.push("THERMAL_VOLTAGE_PER_K");

        format!(
            "#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]\n\nuse super::{{{}}};\nuse super::super::state::{{Instance, Parameters}};\n\nimpl Instance {{\n{}}}\n",
            imports.join(", "),
            self.contents
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_statement_list(
    artifact: &CanonicalIrArtifact,
    statements: &[HirStatement],
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    out: &mut String,
    ddt_slots: &DdtSlots,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
    transient_liveness: &TransientLiveness,
    reactive: bool,
    reactive_liveness: &ReactiveLiveness,
    indent: &str,
    prefix: &str,
) -> Result<(), RustBackendError> {
    for (index, statement) in statements.iter().enumerate() {
        let statement_prefix = format!("{prefix}{index}");
        match statement {
            HirStatement::Assignment(assignment) => emit_assignment_statement(
                artifact,
                assignment,
                parameter_fields,
                variable_fields,
                variables,
                out,
                ddt_slots,
                branch_current_unknowns,
                transient_liveness,
                reactive,
                reactive_liveness,
                indent,
                &statement_prefix,
            )?,
            HirStatement::Loop(loop_statement) => emit_loop_statement(
                artifact,
                loop_statement,
                parameter_fields,
                variable_fields,
                variables,
                out,
                ddt_slots,
                branch_current_unknowns,
                transient_liveness,
                reactive,
                reactive_liveness,
                indent,
                &statement_prefix,
            )?,
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_assignment_statement(
    artifact: &CanonicalIrArtifact,
    assignment: &crate::canonical_ir::HirAssignment,
    parameter_fields: &HashMap<String, String>,
    _variable_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    out: &mut String,
    ddt_slots: &DdtSlots,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
    transient_liveness: &TransientLiveness,
    reactive: bool,
    reactive_liveness: &ReactiveLiveness,
    indent: &str,
    prefix: &str,
) -> Result<(), RustBackendError> {
    if reactive && !reactive_liveness.is_variable_live(assignment.target_name.as_str()) {
        return Ok(());
    }
    if !reactive && !transient_liveness.is_value_live(assignment.target_name.as_str()) {
        return Ok(());
    }
    if reactive
        && !expr_depends_on_ddt_or_dynamic(
            artifact,
            assignment.expr.id,
            &HashSet::new(),
            &mut HashSet::new(),
        )?
    {
        return emit_compact_assignment_statement(
            artifact,
            assignment,
            parameter_fields,
            variables,
            out,
            ddt_slots,
            branch_current_unknowns,
            indent,
            prefix,
        );
    }

    let derivatives_live =
        reactive || transient_liveness.is_derivative_live(assignment.target_name.as_str());
    if !reactive && derivatives_live {
        return emit_compact_assignment_statement(
            artifact,
            assignment,
            parameter_fields,
            variables,
            out,
            ddt_slots,
            branch_current_unknowns,
            indent,
            prefix,
        );
    }
    let lowered = if reactive {
        let branch_currents = HashMap::new();
        lower_reactive_assignment_expr_with_branch_currents(
            artifact,
            assignment.expr.id,
            prefix,
            parameter_fields,
            variables,
            ddt_slots,
            &branch_currents,
            branch_current_unknowns,
        )?
    } else if derivatives_live {
        let branch_currents = HashMap::new();
        lower_assignment_expr_with_branch_currents(
            artifact,
            assignment.expr.id,
            prefix,
            parameter_fields,
            variables,
            ddt_slots,
            &branch_currents,
            branch_current_unknowns,
        )?
    } else {
        let branch_currents = HashMap::new();
        lower_value_assignment_expr_with_branch_currents(
            artifact,
            assignment.expr.id,
            prefix,
            parameter_fields,
            variables,
            ddt_slots,
            &branch_currents,
            branch_current_unknowns,
        )?
    };
    for line in lowered.lines {
        out.push_str(indent);
        out.push_str(&line);
        out.push('\n');
    }

    let target = artifact
        .hir
        .variables
        .get(usize::from(assignment.target))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!(
                    "assignment target {} is outside HIR variable arena",
                    assignment.target
                ),
            )
        })?;
    let target_index = usize::from(target.id);
    let target_local = format!("scratch.values[{target_index}]");
    out.push_str(&format!("{indent}{target_local} = {};\n", lowered.value));
    let mut derivative_locals = Vec::with_capacity(lowered.derivatives.len());
    for (node_index, derivative) in lowered.derivatives.iter().enumerate() {
        if is_zero_derivative(derivative) {
            derivative_locals.push("0.0".to_string());
            continue;
        }
        if is_constant_derivative_expr(derivative) {
            derivative_locals.push(derivative.clone());
            continue;
        }
        let derivative_local = format!("scratch.node_derivatives[{target_index}][{node_index}]");
        out.push_str(&format!("{indent}{derivative_local} = {derivative};\n"));
        derivative_locals.push(derivative_local);
    }
    let mut branch_derivative_locals = Vec::with_capacity(lowered.branch_derivatives.len());
    for (branch_index, derivative) in lowered.branch_derivatives.iter().enumerate() {
        if is_zero_derivative(derivative) {
            branch_derivative_locals.push("0.0".to_string());
            continue;
        }
        if is_constant_derivative_expr(derivative) {
            branch_derivative_locals.push(derivative.clone());
            continue;
        }
        let derivative_local =
            format!("scratch.branch_derivatives[{target_index}][{branch_index}]");
        out.push_str(&format!("{indent}{derivative_local} = {derivative};\n"));
        branch_derivative_locals.push(derivative_local);
    }
    let target_reactive = format!("scratch.reactive_values[{target_index}]");
    let mut reactive_derivative_locals = Vec::with_capacity(lowered.reactive_derivatives.len());
    let mut reactive_branch_derivative_locals =
        Vec::with_capacity(lowered.reactive_branch_derivatives.len());
    if reactive {
        out.push_str(&format!(
            "{indent}{target_reactive} = {};\n",
            lowered.reactive_value
        ));
        for (node_index, derivative) in lowered.reactive_derivatives.iter().enumerate() {
            if is_zero_derivative(derivative) {
                reactive_derivative_locals.push("0.0".to_string());
                continue;
            }
            if is_constant_derivative_expr(derivative) {
                reactive_derivative_locals.push(derivative.clone());
                continue;
            }
            let derivative_local =
                format!("scratch.reactive_node_derivatives[{target_index}][{node_index}]");
            out.push_str(&format!("{indent}{derivative_local} = {derivative};\n"));
            reactive_derivative_locals.push(derivative_local);
        }
        for (branch_index, derivative) in lowered.reactive_branch_derivatives.iter().enumerate() {
            if is_zero_derivative(derivative) {
                reactive_branch_derivative_locals.push("0.0".to_string());
                continue;
            }
            if is_constant_derivative_expr(derivative) {
                reactive_branch_derivative_locals.push(derivative.clone());
                continue;
            }
            let derivative_local =
                format!("scratch.reactive_branch_derivatives[{target_index}][{branch_index}]");
            out.push_str(&format!("{indent}{derivative_local} = {derivative};\n"));
            reactive_branch_derivative_locals.push(derivative_local);
        }
    } else {
        reactive_derivative_locals = zero_derivative_vec(artifact.mir.nodes.len());
        reactive_branch_derivative_locals =
            zero_derivative_vec(branch_derivative_axis_count(branch_current_unknowns));
    }
    variables.insert(
        target.name.to_string(),
        LoweredVariable {
            value: target_local,
            condition: None,
            derivatives: derivative_locals,
            branch_derivatives: branch_derivative_locals,
            has_reactive: reactive && lowered.has_reactive,
            reactive_value: if reactive {
                target_reactive
            } else {
                "0.0".to_string()
            },
            reactive_derivatives: reactive_derivative_locals,
            reactive_branch_derivatives: reactive_branch_derivative_locals,
        },
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_compact_assignment_statement(
    artifact: &CanonicalIrArtifact,
    assignment: &crate::canonical_ir::HirAssignment,
    parameter_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    out: &mut String,
    ddt_slots: &DdtSlots,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
    indent: &str,
    prefix: &str,
) -> Result<(), RustBackendError> {
    let target = artifact
        .hir
        .variables
        .get(usize::from(assignment.target))
        .ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!(
                    "assignment target {} is outside HIR variable arena",
                    assignment.target
                ),
            )
        })?;
    let target_index = usize::from(target.id);
    let branch_axis_count = branch_derivative_axis_count(branch_current_unknowns);
    let mut emitter = CompactAdEmitter {
        artifact,
        prefix,
        parameter_fields,
        variables,
        ddt_slots,
        branch_current_unknowns,
        emitted: HashMap::new(),
        lines: Vec::new(),
    };

    if emit_compact_conditional_noop_assignment(
        &mut emitter,
        assignment.expr.id,
        target.name.as_str(),
        target_index,
        out,
        indent,
    )? {
        drop(emitter);
        variables.insert(
            target.name.to_string(),
            scratch_backed_variable(artifact, target_index, branch_axis_count, false),
        );
        return Ok(());
    }

    if emit_compact_conditional_scalar_branch_assignment(
        &mut emitter,
        assignment.expr.id,
        target_index,
        out,
        indent,
    )? {
        drop(emitter);
        variables.insert(
            target.name.to_string(),
            scratch_backed_variable(artifact, target_index, branch_axis_count, false),
        );
        return Ok(());
    }

    if emit_compact_conditional_ad_branch_assignment(
        &mut emitter,
        assignment.expr.id,
        target_index,
        out,
        indent,
    )? {
        drop(emitter);
        variables.insert(
            target.name.to_string(),
            scratch_backed_variable(artifact, target_index, branch_axis_count, false),
        );
        return Ok(());
    }

    if let Some(source_index) = emitter.ad_identifier_index(assignment.expr.id)? {
        if source_index != target_index {
            push_compact_ad_copy(out, indent, target_index, source_index);
        }
        variables.insert(
            target.name.to_string(),
            scratch_backed_variable(artifact, target_index, branch_axis_count, false),
        );
        return Ok(());
    }

    if let Some(condition) = emitter.boolean_expr_condition(assignment.expr.id)? {
        out.push_str(&format!(
            "{indent}scratch.bool_values[{target_index}] = {condition};\n"
        ));
        out.push_str(&format!(
            "{indent}scratch.values[{target_index}] = if scratch.bool_values[{target_index}] {{ 1.0 }} else {{ 0.0 }};\n"
        ));
        variables.insert(
            target.name.to_string(),
            boolean_scratch_variable(artifact, target_index, branch_axis_count),
        );
        return Ok(());
    }

    if let Some(value) = emitter.zero_derivative_value_expr(assignment.expr.id)? {
        out.push_str(&format!(
            "{indent}scratch.values[{target_index}] = {value};\n"
        ));
        variables.insert(
            target.name.to_string(),
            zero_derivative_scratch_variable(artifact, target_index, branch_axis_count),
        );
        return Ok(());
    }

    let value = emitter.lower(assignment.expr.id)?;
    let move_generated_local = compact_lines_declare_generated_ad_local(&emitter.lines, &value);
    for line in emitter.lines {
        out.push_str(indent);
        out.push_str(&line);
        out.push('\n');
    }

    push_compact_ad_value_store(out, indent, target_index, &value, move_generated_local);
    variables.insert(
        target.name.to_string(),
        scratch_backed_variable(artifact, target_index, branch_axis_count, false),
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_compact_conditional_noop_assignment(
    emitter: &mut CompactAdEmitter<'_>,
    expr: ExprId,
    target_name: &str,
    target_index: usize,
    out: &mut String,
    indent: &str,
) -> Result<bool, RustBackendError> {
    let Some((condition, store_expr, invert_condition)) =
        emitter.noop_conditional_assignment(expr, target_name)?
    else {
        return Ok(false);
    };

    let condition = emitter.lower_condition(condition)?;
    let scalar_value = emitter.zero_derivative_value_expr(store_expr)?;
    let branch = if scalar_value.is_none() {
        Some(emitter.lower_isolated_branch(store_expr)?)
    } else {
        None
    };
    for line in emitter.lines.drain(..) {
        out.push_str(indent);
        out.push_str(&line);
        out.push('\n');
    }

    if invert_condition {
        out.push_str(&format!("{indent}if {} {{\n", negate_condition(&condition)));
    } else {
        out.push_str(&format!("{indent}if {condition} {{\n"));
    }
    let branch_indent = format!("{indent}    ");
    if let Some(value) = scalar_value {
        push_compact_scalar_store(out, &branch_indent, target_index, &value);
    } else if let Some(branch) = branch {
        push_compact_ad_store(out, &branch_indent, target_index, &branch);
    }
    out.push_str(&format!("{indent}}}\n"));

    Ok(true)
}

fn emit_compact_conditional_scalar_branch_assignment(
    emitter: &mut CompactAdEmitter<'_>,
    expr: ExprId,
    target_index: usize,
    out: &mut String,
    indent: &str,
) -> Result<bool, RustBackendError> {
    let Some(conditional) = emitter.mixed_scalar_conditional_assignment(expr)? else {
        return Ok(false);
    };

    let condition = emitter.lower_condition(conditional.condition)?;
    let ad_branch = emitter.lower_isolated_branch(conditional.ad_expr)?;
    for line in emitter.lines.drain(..) {
        out.push_str(indent);
        out.push_str(&line);
        out.push('\n');
    }

    out.push_str(&format!("{indent}if {condition} {{\n"));
    let branch_indent = format!("{indent}    ");
    if conditional.scalar_when_condition_true {
        push_compact_scalar_store(out, &branch_indent, target_index, &conditional.scalar_value);
    } else {
        push_compact_ad_store(out, &branch_indent, target_index, &ad_branch);
    }
    out.push_str(&format!("{indent}}} else {{\n"));
    if conditional.scalar_when_condition_true {
        push_compact_ad_store(out, &branch_indent, target_index, &ad_branch);
    } else {
        push_compact_scalar_store(out, &branch_indent, target_index, &conditional.scalar_value);
    }
    out.push_str(&format!("{indent}}}\n"));

    Ok(true)
}

fn emit_compact_conditional_ad_branch_assignment(
    emitter: &mut CompactAdEmitter<'_>,
    expr: ExprId,
    target_index: usize,
    out: &mut String,
    indent: &str,
) -> Result<bool, RustBackendError> {
    let HirExprKind::Conditional {
        then_expr,
        else_expr,
        ..
    } = emitter.expression(expr)?.kind.clone()
    else {
        return Ok(false);
    };

    if emitter.zero_derivative_value_expr(then_expr)?.is_some()
        && emitter.zero_derivative_value_expr(else_expr)?.is_some()
    {
        return Ok(false);
    }

    push_compact_conditional_assignment_expr(emitter, expr, target_index, out, indent)?;
    Ok(true)
}

fn push_compact_conditional_assignment_expr(
    emitter: &mut CompactAdEmitter<'_>,
    expr: ExprId,
    target_index: usize,
    out: &mut String,
    indent: &str,
) -> Result<(), RustBackendError> {
    if let Some(value) = emitter.zero_derivative_value_expr(expr)? {
        push_compact_scalar_store(out, indent, target_index, &value);
        return Ok(());
    }

    let HirExprKind::Conditional {
        condition,
        then_expr,
        else_expr,
    } = emitter.expression(expr)?.kind.clone()
    else {
        let branch = emitter.lower_isolated_branch(expr)?;
        push_compact_ad_store(out, indent, target_index, &branch);
        return Ok(());
    };

    let condition = emitter.lower_condition(condition)?;
    for line in emitter.lines.drain(..) {
        out.push_str(indent);
        out.push_str(&line);
        out.push('\n');
    }

    out.push_str(&format!("{indent}if {condition} {{\n"));
    let branch_indent = format!("{indent}    ");
    push_compact_conditional_assignment_expr(
        emitter,
        then_expr,
        target_index,
        out,
        &branch_indent,
    )?;
    out.push_str(&format!("{indent}}} else {{\n"));
    push_compact_conditional_assignment_expr(
        emitter,
        else_expr,
        target_index,
        out,
        &branch_indent,
    )?;
    out.push_str(&format!("{indent}}}\n"));

    Ok(())
}

fn push_compact_ad_store(
    out: &mut String,
    indent: &str,
    target_index: usize,
    branch: &CompactBranch,
) {
    for line in &branch.lines {
        push_indented_compact_line(out, indent, line);
    }
    if let Some(source_index) = compact_scratch_ad_value_index(&branch.value) {
        if source_index != target_index {
            push_compact_ad_copy(out, indent, target_index, source_index);
        }
        return;
    }
    if let Some(line) = compact_scratch_store_helper_call(target_index, &branch.value) {
        push_indented_compact_line(out, indent, &line);
        return;
    }
    let move_generated_local =
        compact_lines_declare_generated_ad_local(&branch.lines, &branch.value);
    push_compact_ad_value_store(
        out,
        indent,
        target_index,
        &branch.value,
        move_generated_local,
    );
}

fn push_compact_ad_value_store(
    out: &mut String,
    indent: &str,
    target_index: usize,
    value: &str,
    move_generated_local: bool,
) {
    if let Some(source_index) = compact_scratch_ad_value_index(value) {
        if source_index != target_index {
            push_compact_ad_copy(out, indent, target_index, source_index);
        }
        return;
    }
    if let Some(line) = compact_scratch_store_helper_call(target_index, value) {
        push_indented_compact_line(out, indent, &line);
        return;
    }
    if move_generated_local || compact_ad_store_rvalue(value) {
        push_indented_compact_line(
            out,
            indent,
            &format!("scratch.store_ad_value({target_index}, {value});"),
        );
        return;
    }
    push_indented_compact_line(
        out,
        indent,
        &format!("scratch.store_ad({target_index}, &{value});"),
    );
}

fn compact_lines_declare_generated_ad_local(lines: &[String], value: &str) -> bool {
    let value = value.trim();
    if !compact_generated_ad_local(value) {
        return false;
    }
    let declaration = format!("let {value}: AdValue = ");
    lines
        .iter()
        .any(|line| line.trim_start().starts_with(&declaration))
}

fn push_compact_scalar_store(out: &mut String, indent: &str, target_index: usize, value: &str) {
    push_indented_compact_line(
        out,
        indent,
        &format!("scratch.store_scalar({target_index}, {value});"),
    );
}

fn push_compact_ad_copy(out: &mut String, indent: &str, target_index: usize, source_index: usize) {
    push_indented_compact_line(
        out,
        indent,
        &format!("scratch.copy_ad({target_index}, {source_index});"),
    );
}

fn compact_scratch_ad_value_index(value: &str) -> Option<usize> {
    value
        .trim()
        .strip_prefix("scratch.ad_value(")?
        .strip_suffix(')')?
        .parse()
        .ok()
}

fn compact_ad_store_rvalue(value: &str) -> bool {
    let value = value.trim();
    value.starts_with("AdValue::") || value.starts_with('{')
}

fn compact_non_atomic_ad_value(value: &str) -> bool {
    let value = value.trim();
    if compact_scratch_ad_value_index(value).is_some() {
        return false;
    }
    if compact_ad_call_args(value, "constant").is_some()
        || compact_ad_call_args(value, "voltage").is_some()
        || compact_ad_call_args(value, "branch_current").is_some()
    {
        return false;
    }
    value.starts_with("AdValue::") || value.starts_with('{') || compact_generated_ad_local(value)
}

fn compact_generated_ad_local(value: &str) -> bool {
    let value = value.trim();
    let Some((prefix, suffix)) = value.split_once("_ad_e") else {
        return false;
    };
    value.starts_with("assign")
        && prefix
            .chars()
            .all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
        && !suffix.is_empty()
        && suffix.chars().all(|ch| ch.is_ascii_digit())
}

fn compact_scratch_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "constant")
        && args.len() == 1
    {
        return Some(format!(
            "scratch.store_scalar({target_index}, {});",
            args[0]
        ));
    }

    if let Some(line) = compact_constant_ad_scalar_store_helper_call(target_index, value) {
        return Some(line);
    }

    if let Some(line) = compact_voltage_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_mul_voltage_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_div_voltage_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_sub_voltage_abs_voltage_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_fused_scale_offset_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_fused_scaled_add_sub_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_nested_scale_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_nested_offset_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_unary_binary_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_scaled_input_unary_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_offset_input_unary_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_negated_input_unary_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_sqrt_square_and_affine_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_scaled_binary_operand_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_add_sub_mixed_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_fused_product3_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_nested_mixed_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_offset_mixed_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) =
        compact_sub_from_scalar_mixed_multiply_store_helper_call(target_index, value)
    {
        return Some(line);
    }
    if let Some(line) =
        compact_div_from_scalar_mixed_multiply_store_helper_call(target_index, value)
    {
        return Some(line);
    }
    if let Some(line) = compact_pow_mixed_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_fused_scaled_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_scale_mixed_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_negated_mixed_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_division_mixed_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_square_mixed_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_unary_mixed_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_scaled_mixed_multiply_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_mixed_scaled_operand_add_sub_store_helper_call(target_index, value)
    {
        return Some(line);
    }
    if let Some(line) = compact_mixed_scratch_ad_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_div_from_scalar_affine_input_store_helper_call(target_index, value)
    {
        return Some(line);
    }
    if let Some(line) = compact_sqrt_general_ad_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_scratch_choice_store_helper_call(target_index, value) {
        return Some(line);
    }
    if let Some(line) = compact_general_ad_store_helper_call(target_index, value) {
        return Some(line);
    }

    for (name, helper) in [
        ("add", "store_add"),
        ("sub", "store_sub"),
        ("mul", "store_mul"),
        ("div", "store_div"),
    ] {
        if let Some(args) = compact_ad_call_args(value, name) {
            if args.len() != 2 {
                continue;
            }
            let left = compact_scratch_ad_value_index(args[0])?;
            let right = compact_scratch_ad_value_index(args[1])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {left}, {right});"
            ));
        }
    }

    for (name, helper) in [("scale", "store_scale"), ("offset", "store_offset")] {
        if let Some(args) = compact_ad_call_args(value, name) {
            if args.len() != 2 {
                continue;
            }
            let source = compact_scratch_ad_value_index(args[0])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {source}, {});",
                args[1]
            ));
        }
    }

    for (name, helper) in [
        ("sub_from_scalar", "store_sub_from_scalar"),
        ("div_from_scalar", "store_div_from_scalar"),
    ] {
        if let Some(args) = compact_ad_call_args(value, name) {
            if args.len() != 2 {
                continue;
            }
            let source = compact_scratch_ad_value_index(args[1])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {}, {source});",
                args[0]
            ));
        }
    }

    for (name, helper) in [
        ("min_with_scalar", "store_min_with_scalar"),
        ("max_with_scalar", "store_max_with_scalar"),
    ] {
        if let Some(args) = compact_ad_call_args(value, name) {
            if args.len() != 2 {
                continue;
            }
            let source = compact_scratch_ad_value_index(args[0])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {source}, {});",
                args[1]
            ));
        }
    }

    for (name, helper) in [
        ("neg", "store_neg"),
        ("abs", "store_abs"),
        ("square", "store_square"),
        ("sqrt", "store_sqrt"),
        ("exp", "store_exp"),
        ("limexp", "store_limexp"),
        ("limited_exp", "store_limited_exp"),
        ("ln", "store_ln"),
        ("ln_one_plus_exp", "store_ln_one_plus_exp"),
        ("sin", "store_sin"),
        ("sinh", "store_sinh"),
        ("asinh", "store_asinh"),
    ] {
        if let Some(args) = compact_ad_call_args(value, name) {
            if args.len() != 1 {
                continue;
            }
            let source = compact_scratch_ad_value_index(args[0])?;
            return Some(format!("scratch.{helper}({target_index}, {source});"));
        }
    }

    if let Some(args) = compact_ad_call_args(value, "powf")
        && args.len() == 2
    {
        let source = compact_scratch_ad_value_index(args[0])?;
        return Some(format!(
            "scratch.store_powf({target_index}, {source}, {});",
            args[1]
        ));
    }

    None
}

fn compact_constant_ad_scalar_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "scale") {
        if args.len() != 2 {
            return None;
        }
        let constant_args = compact_ad_call_args(args[0], "constant")?;
        if constant_args.len() != 1 {
            return None;
        }
        return Some(format!(
            "scratch.store_scalar({target_index}, ({} * {}));",
            constant_args[0], args[1]
        ));
    }

    None
}

fn compact_scratch_choice_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    for (name, helper, helper3) in [
        ("min", "store_min", "store_min3"),
        ("max", "store_max", "store_max3"),
    ] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 2 {
            return None;
        }

        if let (Some(left), Some(right)) = (
            compact_scratch_ad_value_index(args[0]),
            compact_scratch_ad_value_index(args[1]),
        ) {
            return Some(format!(
                "scratch.{helper}({target_index}, {left}, {right});"
            ));
        }

        if let Some(inner_args) = compact_ad_call_args(args[0], name) {
            if inner_args.len() != 2 {
                return None;
            }
            let first = compact_scratch_ad_value_index(inner_args[0])?;
            let second = compact_scratch_ad_value_index(inner_args[1])?;
            let third = compact_scratch_ad_value_index(args[1])?;
            return Some(format!(
                "scratch.{helper3}({target_index}, {first}, {second}, {third});"
            ));
        }

        if let Some(inner_args) = compact_ad_call_args(args[1], name) {
            if inner_args.len() != 2 {
                return None;
            }
            let first = compact_scratch_ad_value_index(args[0])?;
            let second = compact_scratch_ad_value_index(inner_args[0])?;
            let third = compact_scratch_ad_value_index(inner_args[1])?;
            return Some(format!(
                "scratch.{helper3}({target_index}, {first}, {second}, {third});"
            ));
        }
    }

    None
}

fn compact_voltage_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "voltage") {
        return compact_voltage_store_helper_line(target_index, &args, None);
    }

    if let Some(args) = compact_ad_call_args(value, "scale") {
        if args.len() != 2 {
            return None;
        }
        let voltage_args = compact_ad_call_args(args[0], "voltage")?;
        return compact_voltage_store_helper_line(target_index, &voltage_args, Some(args[1]));
    }

    if let Some(args) = compact_ad_call_args(value, "neg") {
        if args.len() != 1 {
            return None;
        }
        let voltage_args = compact_ad_call_args(args[0], "voltage")?;
        return compact_voltage_store_helper_line(target_index, &voltage_args, Some("-1.0"));
    }

    if let Some(args) = compact_ad_call_args(value, "offset") {
        if args.len() != 2 {
            return None;
        }
        let voltage_args = compact_ad_call_args(args[0], "voltage")?;
        return compact_offset_voltage_store_helper_line(target_index, &voltage_args, args[1]);
    }

    if let Some(args) = compact_ad_call_args(value, "abs") {
        if args.len() != 1 {
            return None;
        }
        let voltage_args = compact_ad_call_args(args[0], "voltage")?;
        return compact_abs_voltage_store_helper_line(target_index, &voltage_args);
    }

    None
}

fn compact_voltage_store_helper_line(
    target_index: usize,
    args: &[&str],
    scale: Option<&str>,
) -> Option<String> {
    if args.len() != 4 {
        return None;
    }
    let helper = if let Some(scale) = scale {
        format!(
            "scratch.store_scaled_voltage({target_index}, {}, {}, {}, {}, {scale});",
            args[0], args[1], args[2], args[3]
        )
    } else {
        format!(
            "scratch.store_voltage({target_index}, {}, {}, {}, {});",
            args[0], args[1], args[2], args[3]
        )
    };
    Some(helper)
}

fn compact_offset_voltage_store_helper_line(
    target_index: usize,
    args: &[&str],
    offset: &str,
) -> Option<String> {
    if args.len() != 4 {
        return None;
    }
    Some(format!(
        "scratch.store_offset_voltage({target_index}, {}, {}, {}, {}, {offset});",
        args[0], args[1], args[2], args[3]
    ))
}

fn compact_abs_voltage_store_helper_line(target_index: usize, args: &[&str]) -> Option<String> {
    if args.len() != 4 {
        return None;
    }
    Some(format!(
        "scratch.store_abs_voltage({target_index}, {}, {}, {}, {});",
        args[0], args[1], args[2], args[3]
    ))
}

fn compact_div_voltage_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    let args = compact_ad_call_args(value, "div")?;
    if args.len() != 2 {
        return None;
    }

    let voltage_args = compact_ad_call_args(args[0], "voltage")?;
    if voltage_args.len() != 4 {
        return None;
    }
    let right = compact_scratch_or_non_atomic_ad_arg(args[1])?;
    Some(format!(
        "scratch.store_div_voltage_by_ad({target_index}, {}, {}, {}, {}, {right});",
        voltage_args[0], voltage_args[1], voltage_args[2], voltage_args[3]
    ))
}

fn compact_sub_voltage_abs_voltage_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "sub")?;
    if args.len() != 2 {
        return None;
    }

    let left_voltage_args = compact_ad_call_args(args[0], "voltage")?;
    if left_voltage_args.len() != 4 {
        return None;
    }

    let abs_args = compact_ad_call_args(args[1], "abs")?;
    if abs_args.len() != 1 {
        return None;
    }
    let abs_voltage_args = compact_ad_call_args(abs_args[0], "voltage")?;
    if abs_voltage_args.len() != 4
        || left_voltage_args[0] != abs_voltage_args[0]
        || left_voltage_args[1] != abs_voltage_args[1]
    {
        return None;
    }

    Some(format!(
        "scratch.store_sub_voltage_abs_voltage({target_index}, {}, {}, {}, {}, {}, {});",
        left_voltage_args[0],
        left_voltage_args[1],
        left_voltage_args[2],
        left_voltage_args[3],
        abs_voltage_args[2],
        abs_voltage_args[3]
    ))
}

fn compact_mul_voltage_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    if let Some(voltage_args) = compact_ad_call_args(args[0], "voltage") {
        if voltage_args.len() != 4 {
            return None;
        }
        let value = compact_scratch_or_non_atomic_ad_arg(args[1])?;
        return Some(format!(
            "scratch.store_mul_voltage_ad({target_index}, {value}, {}, {}, {}, {});",
            voltage_args[0], voltage_args[1], voltage_args[2], voltage_args[3]
        ));
    }

    if let Some(voltage_args) = compact_ad_call_args(args[1], "voltage") {
        if voltage_args.len() != 4 {
            return None;
        }
        let value = compact_scratch_or_non_atomic_ad_arg(args[0])?;
        return Some(format!(
            "scratch.store_mul_voltage_ad({target_index}, {value}, {}, {}, {}, {});",
            voltage_args[0], voltage_args[1], voltage_args[2], voltage_args[3]
        ));
    }

    None
}

fn compact_general_ad_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "pow") {
        if args.len() != 2 {
            return None;
        }
        let left = compact_scratch_or_non_atomic_ad_arg(args[0])?;
        let right = compact_scratch_or_non_atomic_ad_arg(args[1])?;
        return Some(format!(
            "scratch.store_pow_ad({target_index}, {left}, {right});"
        ));
    }

    for (name, helper) in [("min", "store_min_ad"), ("max", "store_max_ad")] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 2 {
            return None;
        }
        let left = compact_scratch_or_non_atomic_ad_arg(args[0])?;
        let right = compact_scratch_or_non_atomic_ad_arg(args[1])?;
        return Some(format!(
            "scratch.{helper}({target_index}, {left}, {right});"
        ));
    }

    for (name, helper) in [
        ("add", "store_add_ad"),
        ("sub", "store_sub_ad"),
        ("mul", "store_mul_ad"),
        ("div", "store_div_ad"),
        ("rem", "store_rem_ad"),
        ("pow", "store_pow_ad"),
        ("min", "store_min_ad"),
        ("max", "store_max_ad"),
        ("hypot", "store_hypot_ad"),
        ("atan2", "store_atan2_ad"),
    ] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 2 {
            return None;
        }
        if !compact_non_atomic_ad_value(args[0]) || !compact_non_atomic_ad_value(args[1]) {
            return None;
        }
        return Some(format!(
            "scratch.{helper}({target_index}, {}, {});",
            args[0], args[1]
        ));
    }

    for (name, helper) in [
        ("scale", "store_scale_ad"),
        ("offset", "store_offset_ad"),
        ("rem_with_scalar", "store_rem_with_scalar_ad"),
        ("min_with_scalar", "store_min_with_scalar_ad"),
        ("max_with_scalar", "store_max_with_scalar_ad"),
    ] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 2 {
            return None;
        }
        if !compact_non_atomic_ad_value(args[0]) {
            return None;
        }
        return Some(format!(
            "scratch.{helper}({target_index}, {}, {});",
            args[0], args[1]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "pow_from_scalar") {
        if args.len() != 2 {
            return None;
        }
        let value = compact_scratch_or_non_atomic_ad_arg(args[1])?;
        return Some(format!(
            "scratch.store_pow_from_scalar_ad({target_index}, {}, {value});",
            args[0]
        ));
    }

    for (name, helper) in [
        ("sub_from_scalar", "store_sub_from_scalar_ad"),
        ("div_from_scalar", "store_div_from_scalar_ad"),
        ("rem_from_scalar", "store_rem_from_scalar_ad"),
        ("pow_from_scalar", "store_pow_from_scalar_ad"),
        ("min_from_scalar", "store_min_from_scalar_ad"),
        ("max_from_scalar", "store_max_from_scalar_ad"),
    ] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 2 {
            return None;
        }
        if !compact_non_atomic_ad_value(args[1]) {
            return None;
        }
        return Some(format!(
            "scratch.{helper}({target_index}, {}, {});",
            args[0], args[1]
        ));
    }

    for (name, helper) in [
        ("neg", "store_neg_ad"),
        ("square", "store_square_ad"),
        ("sqrt", "store_sqrt_ad"),
        ("exp", "store_exp_ad"),
        ("limexp", "store_limexp_ad"),
        ("limited_exp", "store_limited_exp_ad"),
        ("ln", "store_ln_ad"),
        ("log10", "store_log10_ad"),
        ("abs", "store_abs_ad"),
        ("sin", "store_sin_ad"),
        ("cos", "store_cos_ad"),
        ("tan", "store_tan_ad"),
        ("atan", "store_atan_ad"),
        ("sinh", "store_sinh_ad"),
        ("cosh", "store_cosh_ad"),
        ("tanh", "store_tanh_ad"),
        ("asinh", "store_asinh_ad"),
        ("acosh", "store_acosh_ad"),
        ("atanh", "store_atanh_ad"),
        ("floor", "store_floor_ad"),
        ("ceil", "store_ceil_ad"),
    ] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 1 {
            return None;
        }
        if !compact_non_atomic_ad_value(args[0]) {
            return None;
        }
        return Some(format!("scratch.{helper}({target_index}, {});", args[0]));
    }

    if let Some(args) = compact_ad_call_args(value, "powf") {
        if args.len() != 2 {
            return None;
        }
        if !compact_non_atomic_ad_value(args[0]) {
            return None;
        }
        return Some(format!(
            "scratch.store_powf_ad({target_index}, {}, {});",
            args[0], args[1]
        ));
    }

    None
}

fn compact_mixed_scratch_ad_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    for (name, rhs_helper, lhs_helper) in [
        ("add", "store_add_ad_rhs", "store_add_ad_lhs"),
        ("sub", "store_sub_ad_rhs", "store_sub_ad_lhs"),
        ("mul", "store_mul_ad_rhs", "store_mul_ad_lhs"),
        ("div", "store_div_ad_rhs", "store_div_ad_lhs"),
    ] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 2 {
            return None;
        }
        let left = compact_scratch_ad_value_index(args[0]);
        let right = compact_scratch_ad_value_index(args[1]);
        return match (left, right) {
            (Some(_), Some(_)) | (None, None) => None,
            (Some(left), None) => Some(format!(
                "scratch.{rhs_helper}({target_index}, {left}, {});",
                args[1]
            )),
            (None, Some(right)) => Some(format!(
                "scratch.{lhs_helper}({target_index}, {}, {right});",
                args[0]
            )),
        };
    }
    None
}

fn compact_add_sub_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    match (left, right) {
        (None, Some(right)) => {
            let (helper, left_arg, middle_arg) = compact_add_sub_args(args[0])?;
            if let (Some(left), Some(middle)) = (
                compact_scratch_ad_value_index(left_arg),
                compact_scratch_ad_value_index(middle_arg),
            ) {
                return Some(format!(
                    "scratch.{helper}_lhs({target_index}, {left}, {middle}, {right});"
                ));
            }
            let left = compact_scratch_or_non_atomic_ad_arg(left_arg)?;
            let middle = compact_scratch_or_non_atomic_ad_arg(middle_arg)?;
            Some(format!(
                "scratch.{helper}_ad_lhs({target_index}, {left}, {middle}, {right});"
            ))
        }
        (Some(left), None) => {
            let (helper, middle_arg, right_arg) = compact_add_sub_args(args[1])?;
            if let (Some(middle), Some(right)) = (
                compact_scratch_ad_value_index(middle_arg),
                compact_scratch_ad_value_index(right_arg),
            ) {
                return Some(format!(
                    "scratch.{helper}_rhs({target_index}, {left}, {middle}, {right});"
                ));
            }
            let middle = compact_scratch_or_non_atomic_ad_arg(middle_arg)?;
            let right = compact_scratch_or_non_atomic_ad_arg(right_arg)?;
            Some(format!(
                "scratch.{helper}_ad_rhs({target_index}, {left}, {middle}, {right});"
            ))
        }
        _ => None,
    }
}

fn compact_add_sub_args(value: &str) -> Option<(&'static str, &str, &str)> {
    for (name, helper) in [("add", "store_mul_add"), ("sub", "store_mul_sub")] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 2 {
            return None;
        }
        return Some((helper, args[0], args[1]));
    }

    None
}

fn compact_offset_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "mul_offset_lhs") {
        if args.len() != 3 {
            return None;
        }
        let right = compact_scratch_ad_value_index(args[2])?;
        if let Some(left) = compact_scratch_ad_value_index(args[0]) {
            return Some(format!(
                "scratch.store_mul_offset_lhs({target_index}, {left}, {}, {right});",
                args[1]
            ));
        }
        let left = compact_scratch_or_non_atomic_ad_arg(args[0])?;
        return Some(format!(
            "scratch.store_mul_offset_ad_lhs({target_index}, {left}, {}, {right});",
            args[1]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_offset_rhs") {
        if args.len() != 3 {
            return None;
        }
        let left = compact_scratch_ad_value_index(args[0])?;
        if let Some(right) = compact_scratch_ad_value_index(args[1]) {
            return Some(format!(
                "scratch.store_mul_offset_rhs({target_index}, {left}, {right}, {});",
                args[2]
            ));
        }
        let right = compact_scratch_or_non_atomic_ad_arg(args[1])?;
        return Some(format!(
            "scratch.store_mul_offset_ad_rhs({target_index}, {left}, {right}, {});",
            args[2]
        ));
    }

    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    match (left, right) {
        (None, Some(right)) => {
            let offset_args = compact_ad_call_args(args[0], "offset")?;
            if offset_args.len() != 2 {
                return None;
            }
            if let Some(left) = compact_scratch_ad_value_index(offset_args[0]) {
                return Some(format!(
                    "scratch.store_mul_offset_lhs({target_index}, {left}, {}, {right});",
                    offset_args[1]
                ));
            }
            let left = compact_scratch_or_non_atomic_ad_arg(offset_args[0])?;
            Some(format!(
                "scratch.store_mul_offset_ad_lhs({target_index}, {left}, {}, {right});",
                offset_args[1]
            ))
        }
        (Some(left), None) => {
            let offset_args = compact_ad_call_args(args[1], "offset")?;
            if offset_args.len() != 2 {
                return None;
            }
            if let Some(right) = compact_scratch_ad_value_index(offset_args[0]) {
                return Some(format!(
                    "scratch.store_mul_offset_rhs({target_index}, {left}, {right}, {});",
                    offset_args[1]
                ));
            }
            let right = compact_scratch_or_non_atomic_ad_arg(offset_args[0])?;
            Some(format!(
                "scratch.store_mul_offset_ad_rhs({target_index}, {left}, {right}, {});",
                offset_args[1]
            ))
        }
        _ => None,
    }
}

fn compact_sub_from_scalar_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "mul_sub_from_scalar_lhs") {
        if args.len() != 3 {
            return None;
        }
        let right = compact_scratch_ad_value_index(args[2])?;
        if let Some(value) = compact_scratch_ad_value_index(args[1]) {
            return Some(format!(
                "scratch.store_mul_sub_from_scalar_lhs({target_index}, {}, {value}, {right});",
                args[0]
            ));
        }
        let value = compact_scratch_or_non_atomic_ad_arg(args[1])?;
        return Some(format!(
            "scratch.store_mul_sub_from_scalar_ad_lhs({target_index}, {}, {value}, {right});",
            args[0]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_sub_from_scalar_rhs") {
        if args.len() != 3 {
            return None;
        }
        let left = compact_scratch_ad_value_index(args[0])?;
        if let Some(value) = compact_scratch_ad_value_index(args[2]) {
            return Some(format!(
                "scratch.store_mul_sub_from_scalar_rhs({target_index}, {left}, {}, {value});",
                args[1]
            ));
        }
        let value = compact_scratch_or_non_atomic_ad_arg(args[2])?;
        return Some(format!(
            "scratch.store_mul_sub_from_scalar_ad_rhs({target_index}, {left}, {}, {value});",
            args[1]
        ));
    }

    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    match (left, right) {
        (None, Some(right)) => {
            let sub_args = compact_ad_call_args(args[0], "sub_from_scalar")?;
            if sub_args.len() != 2 {
                return None;
            }
            if let Some(value) = compact_scratch_ad_value_index(sub_args[1]) {
                return Some(format!(
                    "scratch.store_mul_sub_from_scalar_lhs({target_index}, {}, {value}, {right});",
                    sub_args[0]
                ));
            }
            let value = compact_scratch_or_non_atomic_ad_arg(sub_args[1])?;
            Some(format!(
                "scratch.store_mul_sub_from_scalar_ad_lhs({target_index}, {}, {value}, {right});",
                sub_args[0]
            ))
        }
        (Some(left), None) => {
            let sub_args = compact_ad_call_args(args[1], "sub_from_scalar")?;
            if sub_args.len() != 2 {
                return None;
            }
            if let Some(value) = compact_scratch_ad_value_index(sub_args[1]) {
                return Some(format!(
                    "scratch.store_mul_sub_from_scalar_rhs({target_index}, {left}, {}, {value});",
                    sub_args[0]
                ));
            }
            let value = compact_scratch_or_non_atomic_ad_arg(sub_args[1])?;
            Some(format!(
                "scratch.store_mul_sub_from_scalar_ad_rhs({target_index}, {left}, {}, {value});",
                sub_args[0]
            ))
        }
        _ => None,
    }
}

fn compact_div_from_scalar_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    match (left, right) {
        (None, Some(right)) => {
            let div_args = compact_ad_call_args(args[0], "div_from_scalar")?;
            if div_args.len() != 2 {
                return None;
            }
            if let Some(value) = compact_scratch_ad_value_index(div_args[1]) {
                return Some(format!(
                    "scratch.store_mul_div_from_scalar_lhs({target_index}, {}, {value}, {right});",
                    div_args[0]
                ));
            }
            let value = compact_scratch_or_non_atomic_ad_arg(div_args[1])?;
            Some(format!(
                "scratch.store_mul_div_from_scalar_ad_lhs({target_index}, {}, {value}, {right});",
                div_args[0]
            ))
        }
        (Some(left), None) => {
            let div_args = compact_ad_call_args(args[1], "div_from_scalar")?;
            if div_args.len() != 2 {
                return None;
            }
            if let Some(value) = compact_scratch_ad_value_index(div_args[1]) {
                return Some(format!(
                    "scratch.store_mul_div_from_scalar_rhs({target_index}, {left}, {}, {value});",
                    div_args[0]
                ));
            }
            let value = compact_scratch_or_non_atomic_ad_arg(div_args[1])?;
            Some(format!(
                "scratch.store_mul_div_from_scalar_ad_rhs({target_index}, {left}, {}, {value});",
                div_args[0]
            ))
        }
        _ => None,
    }
}

fn compact_pow_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    match (left, right) {
        (None, Some(right)) => compact_pow_mixed_lhs_helper_line(target_index, args[0], right),
        (Some(left), None) => compact_pow_mixed_rhs_helper_line(target_index, left, args[1]),
        _ => None,
    }
}

fn compact_pow_mixed_lhs_helper_line(
    target_index: usize,
    value: &str,
    source: usize,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "pow") {
        if args.len() != 2 {
            return None;
        }
        let base = compact_scratch_or_non_atomic_ad_arg(args[0])?;
        let exponent = compact_scratch_or_non_atomic_ad_arg(args[1])?;
        return Some(format!(
            "scratch.store_mul_pow_ad_lhs({target_index}, {base}, {exponent}, {source});"
        ));
    }

    let args = compact_ad_call_args(value, "powf")?;
    if args.len() != 2 {
        return None;
    }
    let base = compact_scratch_or_non_atomic_ad_arg(args[0])?;
    Some(format!(
        "scratch.store_mul_powf_ad_lhs({target_index}, {base}, {}, {source});",
        args[1]
    ))
}

fn compact_pow_mixed_rhs_helper_line(
    target_index: usize,
    source: usize,
    value: &str,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "pow") {
        if args.len() != 2 {
            return None;
        }
        let base = compact_scratch_or_non_atomic_ad_arg(args[0])?;
        let exponent = compact_scratch_or_non_atomic_ad_arg(args[1])?;
        return Some(format!(
            "scratch.store_mul_pow_ad_rhs({target_index}, {source}, {base}, {exponent});"
        ));
    }

    let args = compact_ad_call_args(value, "powf")?;
    if args.len() != 2 {
        return None;
    }
    let base = compact_scratch_or_non_atomic_ad_arg(args[0])?;
    Some(format!(
        "scratch.store_mul_powf_ad_rhs({target_index}, {source}, {base}, {});",
        args[1]
    ))
}

fn compact_fused_scaled_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "mul_scaled_output") {
        if args.len() != 3 {
            return None;
        }
        return compact_output_scaled_multiply_helper_line(target_index, args[0], args[1], args[2]);
    }

    if let Some(args) = compact_ad_call_args(value, "mul_scaled_lhs") {
        if args.len() != 3 {
            return None;
        }
        return compact_fused_scaled_multiply_helper_line(
            target_index,
            args[0],
            args[2],
            args[1],
            true,
        );
    }

    let args = compact_ad_call_args(value, "mul_scaled_rhs")?;
    if args.len() != 3 {
        return None;
    }
    compact_fused_scaled_multiply_helper_line(target_index, args[0], args[1], args[2], false)
}

fn compact_output_scaled_multiply_helper_line(
    target_index: usize,
    left: &str,
    right: &str,
    scale: &str,
) -> Option<String> {
    let left_source = compact_scratch_ad_value_index(left);
    let right_source = compact_scratch_ad_value_index(right);
    match (left_source, right_source) {
        (Some(left), Some(right)) => Some(format!(
            "scratch.store_scaled_mul({target_index}, {left}, {right}, {scale});"
        )),
        (Some(left), None) if compact_non_atomic_ad_value(right) => Some(format!(
            "scratch.store_mul_scaled_ad_rhs({target_index}, {left}, {scale}, {right});"
        )),
        (None, Some(right)) if compact_non_atomic_ad_value(left) => Some(format!(
            "scratch.store_mul_scaled_ad_lhs({target_index}, {left}, {right}, {scale});"
        )),
        (None, None) if compact_non_atomic_ad_value(left) && compact_non_atomic_ad_value(right) => {
            Some(format!(
                "scratch.store_scaled_mul_ad({target_index}, {left}, {right}, {scale});"
            ))
        }
        _ => None,
    }
}

fn compact_fused_scale_offset_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "scaled_offset") {
        if args.len() != 3 {
            return None;
        }
        if let Some((binary, left, right)) = compact_direct_binary_scratch_args(args[0]) {
            let offset = compact_scale_product(args[1], args[2]);
            return Some(format!(
                "scratch.store_offset_scaled_{binary}({target_index}, {left}, {right}, {}, {offset});",
                args[2]
            ));
        }
        if let Some(source) = compact_scratch_ad_value_index(args[0]) {
            return Some(format!(
                "scratch.store_scaled_offset({target_index}, {source}, {}, {});",
                args[1], args[2]
            ));
        }
        if compact_non_atomic_ad_value(args[0]) {
            return Some(format!(
                "scratch.store_scaled_offset_ad({target_index}, {}, {}, {});",
                args[0], args[1], args[2]
            ));
        }
        return None;
    }

    let args = compact_ad_call_args(value, "scale_offset")?;
    if args.len() != 3 {
        return None;
    }
    if let Some((binary, left, right)) = compact_direct_binary_scratch_args(args[0]) {
        return Some(format!(
            "scratch.store_offset_scaled_{binary}({target_index}, {left}, {right}, {}, {});",
            args[1], args[2]
        ));
    }
    if let Some(source) = compact_scratch_ad_value_index(args[0]) {
        return Some(format!(
            "scratch.store_offset_scaled({target_index}, {source}, {}, {});",
            args[1], args[2]
        ));
    }
    if compact_non_atomic_ad_value(args[0]) {
        return Some(format!(
            "scratch.store_offset_scaled_ad({target_index}, {}, {}, {});",
            args[0], args[1], args[2]
        ));
    }
    None
}

fn compact_fused_scaled_add_sub_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "add_scaled_inputs") {
        if args.len() != 4 {
            return None;
        }
        return compact_fused_scaled_add_sub_helper_line(
            target_index,
            "add",
            args[0],
            args[1],
            args[2],
            args[3],
        );
    }

    let args = compact_ad_call_args(value, "sub_scaled_inputs")?;
    if args.len() != 4 {
        return None;
    }
    compact_fused_scaled_add_sub_helper_line(
        target_index,
        "sub",
        args[0],
        args[1],
        args[2],
        args[3],
    )
}

fn compact_fused_scaled_add_sub_helper_line(
    target_index: usize,
    op: &str,
    left: &str,
    left_scale: &str,
    right: &str,
    right_scale: &str,
) -> Option<String> {
    let left_source = compact_scratch_ad_value_index(left);
    let right_source = compact_scratch_ad_value_index(right);
    match (left_source, right_source) {
        (Some(left), Some(right)) if compact_scalar_same(left_scale, right_scale) => Some(format!(
            "scratch.store_scaled_{op}({target_index}, {left}, {right}, {left_scale});"
        )),
        (Some(left), Some(right)) => Some(format!(
            "scratch.store_{op}_scaled_inputs({target_index}, {left}, {left_scale}, {right}, {right_scale});"
        )),
        (Some(left), None)
            if compact_scalar_same(left_scale, right_scale)
                && compact_non_atomic_ad_value(right) =>
        {
            Some(format!(
                "scratch.store_scaled_{op}_ad_rhs({target_index}, {left}, {right}, {left_scale});"
            ))
        }
        (None, Some(right))
            if compact_scalar_same(left_scale, right_scale)
                && compact_non_atomic_ad_value(left) =>
        {
            Some(format!(
                "scratch.store_scaled_{op}_ad_lhs({target_index}, {left}, {right}, {left_scale});"
            ))
        }
        (None, None)
            if compact_scalar_same(left_scale, right_scale)
                && compact_non_atomic_ad_value(left)
                && compact_non_atomic_ad_value(right) =>
        {
            Some(format!(
                "scratch.store_scaled_{op}_ad({target_index}, {left}, {right}, {left_scale});"
            ))
        }
        (Some(left), None)
            if compact_scalar_same(right_scale, "1.0") && compact_non_atomic_ad_value(right) =>
        {
            Some(format!(
                "scratch.store_{op}_scaled_ad_rhs({target_index}, {left}, {left_scale}, {right});"
            ))
        }
        (None, Some(right))
            if compact_scalar_same(left_scale, "1.0") && compact_non_atomic_ad_value(left) =>
        {
            Some(format!(
                "scratch.store_{op}_scaled_ad_lhs({target_index}, {left}, {right}, {right_scale});"
            ))
        }
        _ => None,
    }
}

fn compact_fused_scaled_multiply_helper_line(
    target_index: usize,
    left: &str,
    right: &str,
    scale: &str,
    scaled_left: bool,
) -> Option<String> {
    let left_source = compact_scratch_ad_value_index(left);
    let right_source = compact_scratch_ad_value_index(right);

    if scaled_left {
        if let Some(right) = right_source
            && let Some(product) = compact_scaled_affine_product(left, scale)
        {
            return compact_nested_multiply_lhs_helper_line(target_index, &product, right);
        }
    } else if let Some(left) = left_source
        && let Some(product) = compact_scaled_affine_product(right, scale)
    {
        return compact_nested_multiply_rhs_helper_line(target_index, left, &product);
    }

    if compact_scalar_is_negative_one(scale) {
        match (left_source, right_source) {
            (Some(left), Some(right)) => {
                if scaled_left {
                    return Some(format!(
                        "scratch.store_mul_neg_lhs({target_index}, {left}, {right});"
                    ));
                }
                return Some(format!(
                    "scratch.store_mul_neg_rhs({target_index}, {left}, {right});"
                ));
            }
            (None, Some(right)) if scaled_left && compact_non_atomic_ad_value(left) => {
                return Some(format!(
                    "scratch.store_mul_neg_ad_lhs({target_index}, {left}, {right});"
                ));
            }
            (Some(left), None) if !scaled_left && compact_non_atomic_ad_value(right) => {
                return Some(format!(
                    "scratch.store_mul_neg_ad_rhs({target_index}, {left}, {right});"
                ));
            }
            _ => {}
        }
    }

    match (left_source, right_source) {
        (Some(left), Some(right)) => Some(format!(
            "scratch.store_scaled_mul({target_index}, {left}, {right}, {scale});"
        )),
        (Some(left), None) if compact_non_atomic_ad_value(right) => {
            if scaled_left {
                Some(format!(
                    "scratch.store_mul_scaled_ad_rhs({target_index}, {left}, {scale}, {right});"
                ))
            } else {
                Some(format!(
                    "scratch.store_mul_scale_ad_rhs({target_index}, {left}, {right}, {scale});"
                ))
            }
        }
        (None, Some(right)) if compact_non_atomic_ad_value(left) => {
            if scaled_left {
                Some(format!(
                    "scratch.store_mul_scale_ad_lhs({target_index}, {left}, {scale}, {right});"
                ))
            } else {
                Some(format!(
                    "scratch.store_mul_scaled_ad_lhs({target_index}, {left}, {right}, {scale});"
                ))
            }
        }
        (None, None) if compact_non_atomic_ad_value(left) && compact_non_atomic_ad_value(right) => {
            Some(format!(
                "scratch.store_scaled_mul_ad({target_index}, {left}, {right}, {scale});"
            ))
        }
        _ => None,
    }
}

fn compact_scale_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    match (left, right) {
        (None, Some(right)) => {
            let scale_args = compact_ad_call_args(args[0], "scale")?;
            if scale_args.len() != 2 {
                return None;
            }
            if let Some(value) = compact_scratch_ad_value_index(scale_args[0]) {
                return Some(format!(
                    "scratch.store_scaled_mul({target_index}, {value}, {right}, {});",
                    scale_args[1]
                ));
            }
            let value = compact_scratch_or_non_atomic_ad_arg(scale_args[0])?;
            Some(format!(
                "scratch.store_mul_scale_ad_lhs({target_index}, {value}, {}, {right});",
                scale_args[1]
            ))
        }
        (Some(left), None) => {
            let scale_args = compact_ad_call_args(args[1], "scale")?;
            if scale_args.len() != 2 {
                return None;
            }
            if let Some(value) = compact_scratch_ad_value_index(scale_args[0]) {
                return Some(format!(
                    "scratch.store_scaled_mul({target_index}, {left}, {value}, {});",
                    scale_args[1]
                ));
            }
            let value = compact_scratch_or_non_atomic_ad_arg(scale_args[0])?;
            Some(format!(
                "scratch.store_mul_scale_ad_rhs({target_index}, {left}, {value}, {});",
                scale_args[1]
            ))
        }
        _ => None,
    }
}

fn compact_negated_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    match (left, right) {
        (None, Some(right)) => {
            let neg_args = compact_ad_call_args(args[0], "neg")?;
            if neg_args.len() != 1 {
                return None;
            }
            if let Some(left) = compact_scratch_ad_value_index(neg_args[0]) {
                return Some(format!(
                    "scratch.store_mul_neg_lhs({target_index}, {left}, {right});"
                ));
            }
            let left = compact_scratch_or_non_atomic_ad_arg(neg_args[0])?;
            Some(format!(
                "scratch.store_mul_neg_ad_lhs({target_index}, {left}, {right});"
            ))
        }
        (Some(left), None) => {
            let neg_args = compact_ad_call_args(args[1], "neg")?;
            if neg_args.len() != 1 {
                return None;
            }
            if let Some(right) = compact_scratch_ad_value_index(neg_args[0]) {
                return Some(format!(
                    "scratch.store_mul_neg_rhs({target_index}, {left}, {right});"
                ));
            }
            let right = compact_scratch_or_non_atomic_ad_arg(neg_args[0])?;
            Some(format!(
                "scratch.store_mul_neg_ad_rhs({target_index}, {left}, {right});"
            ))
        }
        _ => None,
    }
}

fn compact_division_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    match (left, right) {
        (None, Some(source)) => {
            let div_args = compact_ad_call_args(args[0], "div")?;
            if div_args.len() != 2 {
                return None;
            }
            if let (Some(numerator), Some(denominator)) = (
                compact_scratch_ad_value_index(div_args[0]),
                compact_scratch_ad_value_index(div_args[1]),
            ) {
                return Some(format!(
                    "scratch.store_mul_div_lhs({target_index}, {numerator}, {denominator}, {source});"
                ));
            }
            let numerator = compact_scratch_or_non_atomic_ad_arg(div_args[0])?;
            let denominator = compact_scratch_or_non_atomic_ad_arg(div_args[1])?;
            Some(format!(
                "scratch.store_mul_div_ad_lhs({target_index}, {numerator}, {denominator}, {source});"
            ))
        }
        (Some(source), None) => {
            let div_args = compact_ad_call_args(args[1], "div")?;
            if div_args.len() != 2 {
                return None;
            }
            if let (Some(numerator), Some(denominator)) = (
                compact_scratch_ad_value_index(div_args[0]),
                compact_scratch_ad_value_index(div_args[1]),
            ) {
                return Some(format!(
                    "scratch.store_mul_div_rhs({target_index}, {source}, {numerator}, {denominator});"
                ));
            }
            let numerator = compact_scratch_or_non_atomic_ad_arg(div_args[0])?;
            let denominator = compact_scratch_or_non_atomic_ad_arg(div_args[1])?;
            Some(format!(
                "scratch.store_mul_div_ad_rhs({target_index}, {source}, {numerator}, {denominator});"
            ))
        }
        _ => None,
    }
}

fn compact_square_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    match (left, right) {
        (None, Some(source)) => {
            let square_args = compact_ad_call_args(args[0], "square")?;
            if square_args.len() != 1 {
                return None;
            }
            if let Some(value) = compact_scratch_ad_value_index(square_args[0]) {
                return Some(format!(
                    "scratch.store_mul_square_lhs({target_index}, {value}, {source});"
                ));
            }
            None
        }
        (Some(source), None) => {
            let square_args = compact_ad_call_args(args[1], "square")?;
            if square_args.len() != 1 {
                return None;
            }
            if let Some(value) = compact_scratch_ad_value_index(square_args[0]) {
                return Some(format!(
                    "scratch.store_mul_square_rhs({target_index}, {source}, {value});"
                ));
            }
            None
        }
        _ => None,
    }
}

fn compact_unary_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    for (name, helper) in [
        ("exp", "store_mul_exp"),
        ("ln", "store_mul_ln"),
        ("sqrt", "store_mul_sqrt"),
        ("limexp", "store_mul_limexp"),
        ("limited_exp", "store_mul_limited_exp"),
        ("abs", "store_mul_abs"),
        ("cos", "store_mul_cos"),
        ("tanh", "store_mul_tanh"),
    ] {
        match (left, right) {
            (None, Some(right)) => {
                let Some(unary_args) = compact_ad_call_args(args[0], name) else {
                    continue;
                };
                if unary_args.len() != 1 {
                    return None;
                }
                if let Some(value_source) = compact_scratch_ad_value_index(unary_args[0]) {
                    return Some(format!(
                        "scratch.{helper}_lhs({target_index}, {value_source}, {right});"
                    ));
                }
                let value = compact_scratch_or_non_atomic_ad_arg(unary_args[0])?;
                return Some(format!(
                    "scratch.{helper}_ad_lhs({target_index}, {value}, {right});"
                ));
            }
            (Some(left), None) => {
                let Some(unary_args) = compact_ad_call_args(args[1], name) else {
                    continue;
                };
                if unary_args.len() != 1 {
                    return None;
                }
                if let Some(value_source) = compact_scratch_ad_value_index(unary_args[0]) {
                    return Some(format!(
                        "scratch.{helper}_rhs({target_index}, {left}, {value_source});"
                    ));
                }
                let value = compact_scratch_or_non_atomic_ad_arg(unary_args[0])?;
                return Some(format!(
                    "scratch.{helper}_ad_rhs({target_index}, {left}, {value});"
                ));
            }
            _ => {}
        }
    }
    None
}

struct CompactAffineProduct<'a> {
    left: &'a str,
    right: &'a str,
    scale: String,
    offset: String,
    affine: bool,
}

fn compact_nested_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scratch_ad_value_index(args[0]);
    let right = compact_scratch_ad_value_index(args[1]);
    match (left, right) {
        (None, Some(right)) => {
            let product = compact_affine_product(args[0])?;
            compact_nested_multiply_lhs_helper_line(target_index, &product, right)
        }
        (Some(left), None) => {
            let product = compact_affine_product(args[1])?;
            compact_nested_multiply_rhs_helper_line(target_index, left, &product)
        }
        _ => None,
    }
}

fn compact_affine_product(value: &str) -> Option<CompactAffineProduct<'_>> {
    if let Some(args) = compact_ad_call_args(value, "mul") {
        if args.len() != 2 {
            return None;
        }
        return Some(CompactAffineProduct {
            left: args[0],
            right: args[1],
            scale: "1.0".to_string(),
            offset: "0.0".to_string(),
            affine: false,
        });
    }

    if let Some(args) = compact_ad_call_args(value, "scale") {
        if args.len() != 2 {
            return None;
        }
        let product_args = compact_ad_call_args(args[0], "mul")?;
        if product_args.len() != 2 {
            return None;
        }
        return Some(CompactAffineProduct {
            left: product_args[0],
            right: product_args[1],
            scale: args[1].to_string(),
            offset: "0.0".to_string(),
            affine: true,
        });
    }

    let args = compact_ad_call_args(value, "offset")?;
    if args.len() != 2 {
        return None;
    }
    let mut product = compact_affine_product(args[0])?;
    product.offset = compact_scalar_add(&product.offset, args[1]);
    product.affine = true;
    Some(product)
}

fn compact_scaled_affine_product<'a>(
    value: &'a str,
    scale: &str,
) -> Option<CompactAffineProduct<'a>> {
    let mut product = compact_affine_product(value)?;
    product.scale = compact_scalar_mul(&product.scale, scale);
    product.offset = compact_scalar_mul(&product.offset, scale);
    product.affine = true;
    Some(product)
}

fn compact_nested_multiply_lhs_helper_line(
    target_index: usize,
    product: &CompactAffineProduct<'_>,
    source: usize,
) -> Option<String> {
    let left = compact_scratch_ad_value_index(product.left);
    let right = compact_scratch_ad_value_index(product.right);
    if let (Some(left), Some(right)) = (left, right) {
        if product.affine {
            return Some(format!(
                "scratch.store_mul3_affine_lhs({target_index}, {left}, {right}, {}, {}, {source});",
                product.scale, product.offset
            ));
        }
        return Some(format!(
            "scratch.store_mul3_lhs({target_index}, {left}, {right}, {source});"
        ));
    }

    let left = compact_scratch_or_non_atomic_ad_arg(product.left)?;
    let right = compact_scratch_or_non_atomic_ad_arg(product.right)?;
    if product.affine {
        return Some(format!(
            "scratch.store_mul_ad_affine_product_lhs({target_index}, {left}, {right}, {}, {}, {source});",
            product.scale, product.offset
        ));
    }
    Some(format!(
        "scratch.store_mul_ad_product_lhs({target_index}, {left}, {right}, {source});"
    ))
}

fn compact_nested_multiply_rhs_helper_line(
    target_index: usize,
    source: usize,
    product: &CompactAffineProduct<'_>,
) -> Option<String> {
    let left = compact_scratch_ad_value_index(product.left);
    let right = compact_scratch_ad_value_index(product.right);
    if let (Some(left), Some(right)) = (left, right) {
        if product.affine {
            return Some(format!(
                "scratch.store_mul3_affine_rhs({target_index}, {source}, {left}, {right}, {}, {});",
                product.scale, product.offset
            ));
        }
        return Some(format!(
            "scratch.store_mul3_rhs({target_index}, {source}, {left}, {right});"
        ));
    }

    let left = compact_scratch_or_non_atomic_ad_arg(product.left)?;
    let right = compact_scratch_or_non_atomic_ad_arg(product.right)?;
    if product.affine {
        return Some(format!(
            "scratch.store_mul_ad_affine_product_rhs({target_index}, {source}, {left}, {right}, {}, {});",
            product.scale, product.offset
        ));
    }
    Some(format!(
        "scratch.store_mul_ad_product_rhs({target_index}, {source}, {left}, {right});"
    ))
}

fn compact_fused_product3_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "mul3") {
        if args.len() != 3 {
            return None;
        }
        return compact_fused_product3_store_helper_line(
            target_index,
            args[0],
            args[1],
            args[2],
            "1.0",
        );
    }

    let args = compact_ad_call_args(value, "mul3_scaled_output")?;
    if args.len() != 4 {
        return None;
    }
    compact_fused_product3_store_helper_line(target_index, args[0], args[1], args[2], args[3])
}

fn compact_fused_product3_store_helper_line(
    target_index: usize,
    left: &str,
    middle: &str,
    right: &str,
    scale: &str,
) -> Option<String> {
    let affine = !compact_scalar_same(scale, "1.0");
    if let Some(source) = compact_scratch_ad_value_index(right) {
        let product = CompactAffineProduct {
            left,
            right: middle,
            scale: scale.to_string(),
            offset: "0.0".to_string(),
            affine,
        };
        return compact_nested_multiply_lhs_helper_line(target_index, &product, source);
    }

    let source = compact_scratch_ad_value_index(left)?;
    let product = CompactAffineProduct {
        left: middle,
        right,
        scale: scale.to_string(),
        offset: "0.0".to_string(),
        affine,
    };
    compact_nested_multiply_rhs_helper_line(target_index, source, &product)
}

fn compact_scaled_mixed_multiply_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "mul")?;
    if args.len() != 2 {
        return None;
    }

    let left = compact_scaled_or_negated_scratch_arg(args[0]);
    let right = compact_scaled_or_negated_scratch_arg(args[1]);
    match (left, right) {
        (Some((left, scale)), None) if compact_non_atomic_ad_value(args[1]) => Some(format!(
            "scratch.store_mul_scaled_ad_rhs({target_index}, {left}, {scale}, {});",
            args[1]
        )),
        (None, Some((right, scale))) if compact_non_atomic_ad_value(args[0]) => Some(format!(
            "scratch.store_mul_scaled_ad_lhs({target_index}, {}, {right}, {scale});",
            args[0]
        )),
        _ => None,
    }
}

fn compact_mixed_scaled_operand_add_sub_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    for name in ["add", "sub"] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 2 {
            return None;
        }

        let left = compact_scaled_or_negated_scratch_arg(args[0]);
        let right = compact_scaled_or_negated_scratch_arg(args[1]);
        match (left, right) {
            (Some((left, scale)), None) if compact_non_atomic_ad_value(args[1]) => {
                let helper = if name == "add" {
                    "store_add_scaled_ad_rhs"
                } else {
                    "store_sub_scaled_ad_rhs"
                };
                return Some(format!(
                    "scratch.{helper}({target_index}, {left}, {scale}, {});",
                    args[1]
                ));
            }
            (None, Some((right, scale))) if compact_non_atomic_ad_value(args[0]) => {
                let helper = if name == "add" {
                    "store_add_scaled_ad_lhs"
                } else {
                    "store_sub_scaled_ad_lhs"
                };
                return Some(format!(
                    "scratch.{helper}({target_index}, {}, {right}, {scale});",
                    args[0]
                ));
            }
            _ => return None,
        }
    }

    None
}

fn compact_scaled_or_negated_scratch_arg(value: &str) -> Option<(usize, String)> {
    if let Some(scale_args) = compact_ad_call_args(value, "scale") {
        if scale_args.len() != 2 {
            return None;
        }
        if let Some(source) = compact_scratch_ad_value_index(scale_args[0]) {
            return Some((source, scale_args[1].to_string()));
        }
        if let Some(neg_args) = compact_ad_call_args(scale_args[0], "neg") {
            if neg_args.len() != 1 {
                return None;
            }
            let source = compact_scratch_ad_value_index(neg_args[0])?;
            return Some((source, compact_scalar_negate(scale_args[1])));
        }
    }

    if let Some(neg_args) = compact_ad_call_args(value, "neg") {
        if neg_args.len() != 1 {
            return None;
        }
        let source = compact_scratch_ad_value_index(neg_args[0])?;
        return Some((source, "-1.0".to_string()));
    }

    None
}

fn compact_scaled_input_unary_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    for (name, fused_name, helper) in [
        ("sqrt", "sqrt_scaled_input", "store_sqrt_scaled_input"),
        ("exp", "exp_scaled_input", "store_exp_scaled_input"),
        ("limexp", "limexp_scaled_input", "store_limexp_scaled_input"),
        (
            "limited_exp",
            "limited_exp_scaled_input",
            "store_limited_exp_scaled_input",
        ),
        ("ln", "ln_scaled_input", "store_ln_scaled_input"),
        (
            "ln_one_plus_exp",
            "ln_one_plus_exp_scaled_input",
            "store_ln_one_plus_exp_scaled_input",
        ),
        ("sin", "sin_scaled_input", "store_sin_scaled_input"),
    ] {
        if let Some(args) = compact_ad_call_args(value, fused_name) {
            if args.len() != 2 {
                return None;
            }
            if args[1] == "-1.0" {
                continue;
            }
            let source = compact_scratch_ad_value_index(args[0])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {source}, {});",
                args[1]
            ));
        }

        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 1 {
            return None;
        }
        let inner_args = compact_ad_call_args(args[0], "scale")?;
        if inner_args.len() != 2 {
            return None;
        }
        let source = compact_scratch_ad_value_index(inner_args[0])?;
        return Some(format!(
            "scratch.{helper}({target_index}, {source}, {});",
            inner_args[1]
        ));
    }

    None
}

fn compact_offset_input_unary_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    for (name, helper) in [
        ("sqrt", "store_sqrt_offset_input"),
        ("exp", "store_exp_offset_input"),
        ("ln", "store_ln_offset_input"),
    ] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 1 {
            return None;
        }
        let inner_args = compact_ad_call_args(args[0], "offset")?;
        if inner_args.len() != 2 {
            return None;
        }
        let source = compact_scratch_ad_value_index(inner_args[0])?;
        return Some(format!(
            "scratch.{helper}({target_index}, {source}, {});",
            inner_args[1]
        ));
    }

    None
}

fn compact_negated_input_unary_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    for (name, fused_name, helper) in [
        ("sqrt", "sqrt_scaled_input", "store_sqrt_neg_input"),
        ("exp", "exp_scaled_input", "store_exp_neg_input"),
        ("limexp", "limexp_scaled_input", "store_limexp_neg_input"),
        (
            "limited_exp",
            "limited_exp_scaled_input",
            "store_limited_exp_neg_input",
        ),
        ("ln", "ln_scaled_input", "store_ln_neg_input"),
        (
            "ln_one_plus_exp",
            "ln_one_plus_exp_scaled_input",
            "store_ln_one_plus_exp_neg_input",
        ),
    ] {
        if let Some(args) = compact_ad_call_args(value, fused_name) {
            if args.len() != 2 || args[1] != "-1.0" {
                continue;
            }
            let source = compact_scratch_ad_value_index(args[0])?;
            return Some(format!("scratch.{helper}({target_index}, {source});"));
        }

        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 1 {
            return None;
        }
        let Some(neg_args) = compact_ad_call_args(args[0], "neg") else {
            continue;
        };
        if neg_args.len() != 1 {
            return None;
        }
        let source = compact_scratch_ad_value_index(neg_args[0])?;
        return Some(format!("scratch.{helper}({target_index}, {source});"));
    }

    None
}

fn compact_scaled_binary_operand_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    for (name, helper) in [
        ("add", "store_add_scaled_inputs"),
        ("sub", "store_sub_scaled_inputs"),
    ] {
        let Some(args) = compact_ad_call_args(value, name) else {
            continue;
        };
        if args.len() != 2 {
            return None;
        }
        let left = compact_scaled_or_direct_scratch_arg(args[0])?;
        let right = compact_scaled_or_direct_scratch_arg(args[1])?;
        if !left.2 || !right.2 {
            return None;
        }
        return Some(format!(
            "scratch.{helper}({target_index}, {}, {}, {}, {});",
            left.0, left.1, right.0, right.1
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul") {
        if args.len() != 2 {
            return None;
        }
        let left = compact_scaled_or_direct_scratch_arg(args[0])?;
        let right = compact_scaled_or_direct_scratch_arg(args[1])?;
        if !left.2 && !right.2 {
            return None;
        }
        let scale = compact_scale_product(&left.1, &right.1);
        if left.0 == right.0 {
            return Some(format!(
                "scratch.store_scaled_square({target_index}, {}, {scale});",
                left.0
            ));
        }
        return Some(format!(
            "scratch.store_scaled_mul({target_index}, {}, {}, {scale});",
            left.0, right.0
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "div") {
        if args.len() != 2 {
            return None;
        }
        let left = compact_scaled_or_direct_scratch_arg(args[0])?;
        let right = compact_scaled_or_direct_scratch_arg(args[1])?;
        if !left.2 && !right.2 {
            return None;
        }
        let scale = compact_scale_ratio(&left.1, &right.1);
        return Some(format!(
            "scratch.store_scaled_div({target_index}, {}, {}, {scale});",
            left.0, right.0
        ));
    }

    if let Some((left, left_scale, right, right_scale)) =
        compact_div_scaled_inputs_scratch_args(value)
    {
        let scale = compact_scale_ratio(&left_scale, &right_scale);
        return Some(format!(
            "scratch.store_scaled_div({target_index}, {left}, {right}, {scale});"
        ));
    }

    None
}

fn compact_div_scaled_inputs_scratch_args(value: &str) -> Option<(usize, String, usize, String)> {
    let args = compact_ad_call_args(value, "div_scaled_inputs")?;
    if args.len() != 4 {
        return None;
    }
    let left = compact_scratch_ad_value_index(args[0])?;
    let right = compact_scratch_ad_value_index(args[2])?;
    Some((left, args[1].to_string(), right, args[3].to_string()))
}

fn compact_scaled_or_direct_scratch_arg(value: &str) -> Option<(usize, String, bool)> {
    if let Some(source) = compact_scratch_ad_value_index(value) {
        return Some((source, "1.0".to_string(), false));
    }

    let args = compact_ad_call_args(value, "scale")?;
    if args.len() != 2 {
        return None;
    }
    let source = compact_scratch_ad_value_index(args[0])?;
    Some((source, args[1].to_string(), true))
}

fn compact_scale_product(left: &str, right: &str) -> String {
    if left == "1.0" {
        right.to_string()
    } else if right == "1.0" {
        left.to_string()
    } else {
        format!("(({left}) * ({right}))")
    }
}

fn compact_scale_ratio(left: &str, right: &str) -> String {
    if right == "1.0" {
        left.to_string()
    } else if left == "1.0" {
        format!("(1.0 / ({right}))")
    } else {
        format!("(({left}) * 1.0 / ({right}))")
    }
}

fn compact_div_from_scalar_affine_input_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "div_from_scalar")?;
    if args.len() != 2 {
        return None;
    }
    let scalar = args[0];
    let denominator = args[1];

    if let Some(scale_offset_args) = compact_ad_call_args(denominator, "scale_offset") {
        if scale_offset_args.len() != 3 {
            return None;
        }
        if let Some(source) = compact_scratch_ad_value_index(scale_offset_args[0]) {
            return Some(format!(
                "scratch.store_div_from_scalar_offset_scaled_input({target_index}, {scalar}, {source}, {}, {});",
                scale_offset_args[1], scale_offset_args[2]
            ));
        }
    }

    if let Some(scaled_offset_args) = compact_ad_call_args(denominator, "scaled_offset") {
        if scaled_offset_args.len() != 3 {
            return None;
        }
        if let Some(source) = compact_scratch_ad_value_index(scaled_offset_args[0]) {
            let offset = compact_scale_product(scaled_offset_args[1], scaled_offset_args[2]);
            return Some(format!(
                "scratch.store_div_from_scalar_offset_scaled_input({target_index}, {scalar}, {source}, {}, {offset});",
                scaled_offset_args[2]
            ));
        }
    }

    if let Some(offset_args) = compact_ad_call_args(denominator, "offset") {
        if offset_args.len() != 2 {
            return None;
        }
        if let Some(scale_args) = compact_ad_call_args(offset_args[0], "scale") {
            if scale_args.len() != 2 {
                return None;
            }
            if let Some(source) = compact_scratch_ad_value_index(scale_args[0]) {
                return Some(format!(
                    "scratch.store_div_from_scalar_offset_scaled_input({target_index}, {scalar}, {source}, {}, {});",
                    scale_args[1], offset_args[1]
                ));
            }
            let value = compact_scratch_or_non_atomic_ad_arg(scale_args[0])?;
            return Some(format!(
                "scratch.store_div_from_scalar_offset_ad({target_index}, {scalar}, AdValue::scale({value}, {}), {});",
                scale_args[1], offset_args[1]
            ));
        }
        if let Some(source) = compact_scratch_ad_value_index(offset_args[0]) {
            return Some(format!(
                "scratch.store_div_from_scalar_offset_input({target_index}, {scalar}, {source}, {});",
                offset_args[1]
            ));
        }
        let value = compact_scratch_or_non_atomic_ad_arg(offset_args[0])?;
        return Some(format!(
            "scratch.store_div_from_scalar_offset_ad({target_index}, {scalar}, {value}, {});",
            offset_args[1]
        ));
    }

    if let Some(scale_args) = compact_ad_call_args(denominator, "scale") {
        if scale_args.len() != 2 {
            return None;
        }
        if let Some(offset_args) = compact_ad_call_args(scale_args[0], "offset") {
            if offset_args.len() != 2 {
                return None;
            }
            let offset = compact_scale_product(offset_args[1], scale_args[1]);
            if let Some(source) = compact_scratch_ad_value_index(offset_args[0]) {
                return Some(format!(
                    "scratch.store_div_from_scalar_offset_scaled_input({target_index}, {scalar}, {source}, {}, {offset});",
                    scale_args[1]
                ));
            }
            let value = compact_scratch_or_non_atomic_ad_arg(offset_args[0])?;
            return Some(format!(
                "scratch.store_div_from_scalar_scaled_ad({target_index}, {scalar}, AdValue::offset({value}, {}), {});",
                offset_args[1], scale_args[1]
            ));
        }
        if let Some(source) = compact_scratch_ad_value_index(scale_args[0]) {
            return Some(format!(
                "scratch.store_div_from_scalar_scaled_input({target_index}, {scalar}, {source}, {});",
                scale_args[1]
            ));
        }
        let value = compact_scratch_or_non_atomic_ad_arg(scale_args[0])?;
        return Some(format!(
            "scratch.store_div_from_scalar_scaled_ad({target_index}, {scalar}, {value}, {});",
            scale_args[1]
        ));
    }

    for (name, helper) in [
        ("add", "store_div_from_scalar_add_ad"),
        ("sub", "store_div_from_scalar_sub_ad"),
        ("mul", "store_div_from_scalar_mul_ad"),
        ("div", "store_div_from_scalar_div_ad"),
        ("pow", "store_div_from_scalar_pow_ad"),
    ] {
        if let Some(denominator_args) = compact_ad_call_args(denominator, name) {
            if denominator_args.len() != 2 {
                return None;
            }
            let left = compact_scratch_or_non_atomic_ad_arg(denominator_args[0])?;
            let right = compact_scratch_or_non_atomic_ad_arg(denominator_args[1])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {scalar}, {left}, {right});"
            ));
        }
    }

    for (name, helper) in [
        (
            "sub_from_scalar",
            "store_div_from_scalar_sub_from_scalar_ad",
        ),
        (
            "div_from_scalar",
            "store_div_from_scalar_div_from_scalar_ad",
        ),
    ] {
        if let Some(denominator_args) = compact_ad_call_args(denominator, name) {
            if denominator_args.len() != 2 {
                return None;
            }
            let value = compact_scratch_or_non_atomic_ad_arg(denominator_args[1])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {scalar}, {}, {value});",
                denominator_args[0]
            ));
        }
    }

    for (name, helper) in [
        ("sqrt", "store_div_from_scalar_sqrt_ad"),
        ("square", "store_div_from_scalar_square_ad"),
        ("exp", "store_div_from_scalar_exp_ad"),
        ("sin", "store_div_from_scalar_sin_ad"),
        ("sinh", "store_div_from_scalar_sinh_ad"),
    ] {
        if let Some(denominator_args) = compact_ad_call_args(denominator, name) {
            if denominator_args.len() != 1 {
                return None;
            }
            let value = compact_scratch_or_non_atomic_ad_arg(denominator_args[0])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {scalar}, {value});"
            ));
        }
    }

    if let Some(denominator_args) = compact_ad_call_args(denominator, "powf") {
        if denominator_args.len() != 2 {
            return None;
        }
        let value = compact_scratch_or_non_atomic_ad_arg(denominator_args[0])?;
        return Some(format!(
            "scratch.store_div_from_scalar_powf_ad({target_index}, {scalar}, {value}, {});",
            denominator_args[1]
        ));
    }

    None
}

fn compact_sqrt_general_ad_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "sqrt_scaled_input") {
        if args.len() != 2 {
            return None;
        }
        let source = compact_scratch_or_non_atomic_ad_arg(args[0])?;
        return Some(format!(
            "scratch.store_sqrt_scaled_ad({target_index}, {source}, {});",
            args[1]
        ));
    }

    let args = compact_ad_call_args(value, "sqrt")?;
    if args.len() != 1 {
        return None;
    }
    let inner = args[0];

    for (name, helper) in [
        ("offset", "store_sqrt_offset_ad"),
        ("scale", "store_sqrt_scaled_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 2 {
                return None;
            }
            let source = compact_scratch_or_non_atomic_ad_arg(inner_args[0])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {source}, {});",
                inner_args[1]
            ));
        }
    }

    for (name, helper) in [
        ("add", "store_sqrt_add_ad"),
        ("sub", "store_sqrt_sub_ad"),
        ("mul", "store_sqrt_mul_ad"),
        ("div", "store_sqrt_div_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 2 {
                return None;
            }
            let left = compact_scratch_or_non_atomic_ad_arg(inner_args[0])?;
            let right = compact_scratch_or_non_atomic_ad_arg(inner_args[1])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {left}, {right});"
            ));
        }
    }

    for (name, helper) in [
        ("sub_from_scalar", "store_sqrt_sub_from_scalar_ad"),
        ("div_from_scalar", "store_sqrt_div_from_scalar_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 2 {
                return None;
            }
            let source = compact_scratch_or_non_atomic_ad_arg(inner_args[1])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {}, {source});",
                inner_args[0]
            ));
        }
    }

    if let Some(inner_args) = compact_ad_call_args(inner, "abs") {
        if inner_args.len() != 1 {
            return None;
        }
        let source = compact_scratch_or_non_atomic_ad_arg(inner_args[0])?;
        return Some(format!(
            "scratch.store_sqrt_abs_ad({target_index}, {source});"
        ));
    }

    None
}

fn compact_sqrt_square_and_affine_store_helper_call(
    target_index: usize,
    value: &str,
) -> Option<String> {
    let args = compact_ad_call_args(value, "sqrt")?;
    if args.len() != 1 {
        return None;
    }
    let inner = args[0];

    if let Some(scale_offset_args) = compact_ad_call_args(inner, "scale_offset") {
        if scale_offset_args.len() != 3 {
            return None;
        }
        let source = compact_scratch_ad_value_index(scale_offset_args[0])?;
        return Some(format!(
            "scratch.store_sqrt_offset_scaled_input({target_index}, {source}, {}, {});",
            scale_offset_args[1], scale_offset_args[2]
        ));
    }

    if let Some(scaled_offset_args) = compact_ad_call_args(inner, "scaled_offset") {
        if scaled_offset_args.len() != 3 {
            return None;
        }
        let source = compact_scratch_ad_value_index(scaled_offset_args[0])?;
        let offset = compact_scale_product(scaled_offset_args[1], scaled_offset_args[2]);
        return Some(format!(
            "scratch.store_sqrt_offset_scaled_input({target_index}, {source}, {}, {offset});",
            scaled_offset_args[2]
        ));
    }

    if let Some(offset_args) = compact_ad_call_args(inner, "offset") {
        if offset_args.len() != 2 {
            return None;
        }
        if let Some(scale_args) = compact_ad_call_args(offset_args[0], "scale") {
            if scale_args.len() != 2 {
                return None;
            }
            let source = compact_scratch_ad_value_index(scale_args[0])?;
            return Some(format!(
                "scratch.store_sqrt_offset_scaled_input({target_index}, {source}, {}, {});",
                scale_args[1], offset_args[1]
            ));
        }
        if let Some(source) = compact_square_scratch_arg(offset_args[0]) {
            return Some(format!(
                "scratch.store_sqrt_square_offset({target_index}, {source}, {});",
                offset_args[1]
            ));
        }
    }

    if let Some(add_args) = compact_ad_call_args(inner, "add") {
        if add_args.len() != 2 {
            return None;
        }
        let left_square = compact_square_scratch_arg(add_args[0]);
        let right_square = compact_square_scratch_arg(add_args[1]);
        if let (Some(left), Some(right)) = (left_square, right_square) {
            return Some(format!(
                "scratch.store_sqrt_square_sum({target_index}, {left}, {right});"
            ));
        }
        if let Some(square_source) = left_square {
            let add_source = compact_scratch_ad_value_index(add_args[1])?;
            return Some(format!(
                "scratch.store_sqrt_square_add({target_index}, {square_source}, {add_source});"
            ));
        }
        if let Some(square_source) = right_square {
            let add_source = compact_scratch_ad_value_index(add_args[0])?;
            return Some(format!(
                "scratch.store_sqrt_square_add({target_index}, {square_source}, {add_source});"
            ));
        }
    }

    None
}

fn compact_square_scratch_arg(value: &str) -> Option<usize> {
    let args = compact_ad_call_args(value, "square")?;
    if args.len() != 1 {
        return None;
    }
    compact_scratch_ad_value_index(args[0])
}

fn compact_nested_scale_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    let args = compact_ad_call_args(value, "scale")?;
    if args.len() != 2 {
        return None;
    }
    let inner = args[0];
    let scale = args[1];

    for (name, scaled_helper, scaled_inputs_helper) in [
        ("add", "store_scaled_add", "store_add_scaled_inputs"),
        ("sub", "store_scaled_sub", "store_sub_scaled_inputs"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 2 {
                return None;
            }
            let left = compact_scaled_or_direct_scratch_arg(inner_args[0]);
            let right = compact_scaled_or_direct_scratch_arg(inner_args[1]);
            match (left, right) {
                (Some(left), Some(right)) => {
                    if left.2 || right.2 {
                        let left_scale = compact_scale_product(&left.1, scale);
                        let right_scale = compact_scale_product(&right.1, scale);
                        return Some(format!(
                            "scratch.{scaled_inputs_helper}({target_index}, {}, {left_scale}, {}, {right_scale});",
                            left.0, right.0
                        ));
                    }
                    return Some(format!(
                        "scratch.{scaled_helper}({target_index}, {}, {}, {scale});",
                        left.0, right.0
                    ));
                }
                (Some(left), None) if !left.2 && compact_non_atomic_ad_value(inner_args[1]) => {
                    let helper = if name == "add" {
                        "store_scaled_add_ad_rhs"
                    } else {
                        "store_scaled_sub_ad_rhs"
                    };
                    return Some(format!(
                        "scratch.{helper}({target_index}, {}, {}, {scale});",
                        left.0, inner_args[1]
                    ));
                }
                (None, Some(right)) if !right.2 && compact_non_atomic_ad_value(inner_args[0]) => {
                    let helper = if name == "add" {
                        "store_scaled_add_ad_lhs"
                    } else {
                        "store_scaled_sub_ad_lhs"
                    };
                    return Some(format!(
                        "scratch.{helper}({target_index}, {}, {}, {scale});",
                        inner_args[0], right.0
                    ));
                }
                (None, None)
                    if compact_non_atomic_ad_value(inner_args[0])
                        && compact_non_atomic_ad_value(inner_args[1]) =>
                {
                    let helper = if name == "add" {
                        "store_scaled_add_ad"
                    } else {
                        "store_scaled_sub_ad"
                    };
                    return Some(format!(
                        "scratch.{helper}({target_index}, {}, {}, {scale});",
                        inner_args[0], inner_args[1]
                    ));
                }
                _ => return None,
            }
        }
    }

    if let Some(inner_args) = compact_ad_call_args(inner, "mul") {
        if inner_args.len() != 2 {
            return None;
        }
        let left = compact_scaled_or_direct_scratch_arg(inner_args[0]);
        let right = compact_scaled_or_direct_scratch_arg(inner_args[1]);
        match (left, right) {
            (Some(left), Some(right)) => {
                let input_scale = compact_scale_product(&left.1, &right.1);
                let combined_scale = compact_scale_product(&input_scale, scale);
                if left.0 == right.0 {
                    return Some(format!(
                        "scratch.store_scaled_square({target_index}, {}, {combined_scale});",
                        left.0
                    ));
                }
                return Some(format!(
                    "scratch.store_scaled_mul({target_index}, {}, {}, {combined_scale});",
                    left.0, right.0
                ));
            }
            (Some(left), None) if compact_non_atomic_ad_value(inner_args[1]) => {
                let combined_scale = compact_scale_product(&left.1, scale);
                return Some(format!(
                    "scratch.store_mul_scaled_ad_rhs({target_index}, {}, {combined_scale}, {});",
                    left.0, inner_args[1]
                ));
            }
            (None, Some(right)) if compact_non_atomic_ad_value(inner_args[0]) => {
                let combined_scale = compact_scale_product(&right.1, scale);
                return Some(format!(
                    "scratch.store_mul_scaled_ad_lhs({target_index}, {}, {}, {combined_scale});",
                    inner_args[0], right.0
                ));
            }
            _ => {}
        }
        if compact_non_atomic_ad_value(inner_args[0]) && compact_non_atomic_ad_value(inner_args[1])
        {
            return Some(format!(
                "scratch.store_scaled_mul_ad({target_index}, {}, {}, {scale});",
                inner_args[0], inner_args[1]
            ));
        }
        return None;
    }

    if let Some(inner_args) = compact_ad_call_args(inner, "div") {
        if inner_args.len() != 2 {
            return None;
        }
        let left = compact_scaled_or_direct_scratch_arg(inner_args[0]);
        let right = compact_scaled_or_direct_scratch_arg(inner_args[1]);
        match (left, right) {
            (Some(left), Some(right)) => {
                let input_scale = compact_scale_ratio(&left.1, &right.1);
                let combined_scale = compact_scale_product(&input_scale, scale);
                return Some(format!(
                    "scratch.store_scaled_div({target_index}, {}, {}, {combined_scale});",
                    left.0, right.0
                ));
            }
            (Some(left), None) if compact_non_atomic_ad_value(inner_args[1]) => {
                let combined_scale = compact_scale_product(&left.1, scale);
                return Some(format!(
                    "scratch.store_scaled_div_ad_rhs({target_index}, {}, {}, {combined_scale});",
                    left.0, inner_args[1]
                ));
            }
            (None, Some(right)) if compact_non_atomic_ad_value(inner_args[0]) => {
                let combined_scale = compact_scale_ratio(scale, &right.1);
                return Some(format!(
                    "scratch.store_scaled_div_ad_lhs({target_index}, {}, {}, {combined_scale});",
                    inner_args[0], right.0
                ));
            }
            _ => {}
        }
        if compact_non_atomic_ad_value(inner_args[0]) && compact_non_atomic_ad_value(inner_args[1])
        {
            return Some(format!(
                "scratch.store_scaled_div_ad({target_index}, {}, {}, {scale});",
                inner_args[0], inner_args[1]
            ));
        }
        return None;
    }

    if let Some((left, left_scale, right, right_scale)) =
        compact_div_scaled_inputs_scratch_args(inner)
    {
        let input_scale = compact_scale_ratio(&left_scale, &right_scale);
        let combined_scale = compact_scale_product(&input_scale, scale);
        return Some(format!(
            "scratch.store_scaled_div({target_index}, {left}, {right}, {combined_scale});"
        ));
    }

    if let Some(inner_args) = compact_ad_call_args(inner, "neg") {
        if inner_args.len() != 1 {
            return None;
        }
        let source = compact_scratch_ad_value_index(inner_args[0])?;
        return Some(format!(
            "scratch.store_scale({target_index}, {source}, {});",
            compact_scalar_negate(scale)
        ));
    }

    if let Some(inner_args) = compact_ad_call_args(inner, "offset") {
        if inner_args.len() != 2 {
            return None;
        }
        if let Some(scaled_args) = compact_ad_call_args(inner_args[0], "scale") {
            if scaled_args.len() != 2 {
                return None;
            }
            if let Some((binary, left, right)) = compact_direct_binary_scratch_args(scaled_args[0])
            {
                let combined_scale = compact_scale_product(scaled_args[1], scale);
                let combined_offset = compact_scale_product(inner_args[1], scale);
                return Some(format!(
                    "scratch.store_offset_scaled_{binary}({target_index}, {left}, {right}, {combined_scale}, {combined_offset});"
                ));
            }
            let source = compact_scratch_ad_value_index(scaled_args[0])?;
            let combined_scale = compact_scale_product(scaled_args[1], scale);
            let combined_offset = compact_scale_product(inner_args[1], scale);
            return Some(format!(
                "scratch.store_offset_scaled({target_index}, {source}, {combined_scale}, {combined_offset});"
            ));
        }
        if let Some((binary, left, right)) = compact_direct_binary_scratch_args(inner_args[0]) {
            let combined_offset = compact_scale_product(inner_args[1], scale);
            return Some(format!(
                "scratch.store_offset_scaled_{binary}({target_index}, {left}, {right}, {scale}, {combined_offset});"
            ));
        }
        if compact_non_atomic_ad_value(inner_args[0]) {
            return Some(format!(
                "scratch.store_scaled_offset_ad({target_index}, {}, {}, {scale});",
                inner_args[0], inner_args[1]
            ));
        }
        let source = compact_scratch_ad_value_index(inner_args[0])?;
        return Some(format!(
            "scratch.store_scaled_offset({target_index}, {source}, {}, {scale});",
            inner_args[1]
        ));
    }

    for (name, fused_name, helper) in [
        (
            "sqrt",
            "sqrt_scaled_input",
            "store_scaled_sqrt_scaled_input",
        ),
        ("exp", "exp_scaled_input", "store_scaled_exp_scaled_input"),
        (
            "limexp",
            "limexp_scaled_input",
            "store_scaled_limexp_scaled_input",
        ),
        (
            "limited_exp",
            "limited_exp_scaled_input",
            "store_scaled_limited_exp_scaled_input",
        ),
        ("ln", "ln_scaled_input", "store_scaled_ln_scaled_input"),
        (
            "ln_one_plus_exp",
            "ln_one_plus_exp_scaled_input",
            "store_scaled_ln_one_plus_exp_scaled_input",
        ),
        ("sin", "sin_scaled_input", "store_scaled_sin_scaled_input"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, fused_name) {
            if inner_args.len() != 2 {
                return None;
            }
            let source = compact_scratch_ad_value_index(inner_args[0])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {source}, {}, {scale});",
                inner_args[1]
            ));
        }

        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 1 {
                return None;
            }
            let Some(scaled_args) = compact_ad_call_args(inner_args[0], "scale") else {
                continue;
            };
            if scaled_args.len() != 2 {
                return None;
            }
            let source = compact_scratch_ad_value_index(scaled_args[0])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {source}, {}, {scale});",
                scaled_args[1]
            ));
        }
    }

    for (name, helper) in [
        ("exp", "store_scaled_exp"),
        ("sqrt", "store_scaled_sqrt"),
        ("ln", "store_scaled_ln"),
        ("ln_one_plus_exp", "store_scaled_ln_one_plus_exp"),
        ("limexp", "store_scaled_limexp"),
        ("limited_exp", "store_scaled_limited_exp"),
        ("square", "store_scaled_square"),
        ("abs", "store_scaled_abs"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 1 {
                return None;
            }
            if let Some(source) = compact_scratch_ad_value_index(inner_args[0]) {
                return Some(format!(
                    "scratch.{helper}({target_index}, {source}, {scale});"
                ));
            }
            continue;
        }
    }

    for (name, helper) in [
        ("sub_from_scalar", "store_scaled_sub_from_scalar_ad"),
        ("div_from_scalar", "store_scaled_div_from_scalar_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 2 {
                return None;
            }
            if compact_non_atomic_ad_value(inner_args[1]) {
                return Some(format!(
                    "scratch.{helper}({target_index}, {}, {}, {scale});",
                    inner_args[0], inner_args[1]
                ));
            }
            return None;
        }
    }

    for (name, helper) in [
        ("exp", "store_scaled_exp_ad"),
        ("sqrt", "store_scaled_sqrt_ad"),
        ("ln", "store_scaled_ln_ad"),
        ("limexp", "store_scaled_limexp_ad"),
        ("limited_exp", "store_scaled_limited_exp_ad"),
        ("abs", "store_scaled_abs_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 1 {
                return None;
            }
            if compact_non_atomic_ad_value(inner_args[0]) {
                return Some(format!(
                    "scratch.{helper}({target_index}, {}, {scale});",
                    inner_args[0]
                ));
            }
            return None;
        }
    }

    if let Some(inner_args) = compact_ad_call_args(inner, "powf") {
        if inner_args.len() != 2 {
            return None;
        }
        if compact_non_atomic_ad_value(inner_args[0]) {
            return Some(format!(
                "scratch.store_scaled_powf_ad({target_index}, {}, {}, {scale});",
                inner_args[0], inner_args[1]
            ));
        }
        return None;
    }

    None
}

fn compact_offset_fused_scaled_multiply_store_helper_line(
    target_index: usize,
    inner: &str,
    offset: &str,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(inner, "mul_scaled_output") {
        if args.len() != 3 {
            return None;
        }
        return compact_offset_scaled_mul_helper_line(
            target_index,
            args[0],
            args[1],
            args[2],
            offset,
        );
    }

    if let Some(args) = compact_ad_call_args(inner, "mul_scaled_lhs") {
        if args.len() != 3 {
            return None;
        }
        return compact_offset_scaled_mul_helper_line(
            target_index,
            args[0],
            args[2],
            args[1],
            offset,
        );
    }

    let args = compact_ad_call_args(inner, "mul_scaled_rhs")?;
    if args.len() != 3 {
        return None;
    }
    compact_offset_scaled_mul_helper_line(target_index, args[0], args[1], args[2], offset)
}

fn compact_offset_scaled_mul_helper_line(
    target_index: usize,
    left: &str,
    right: &str,
    scale: &str,
    offset: &str,
) -> Option<String> {
    let left = compact_scratch_ad_value_index(left)?;
    let right = compact_scratch_ad_value_index(right)?;
    Some(format!(
        "scratch.store_offset_scaled_mul({target_index}, {left}, {right}, {scale}, {offset});"
    ))
}

fn compact_offset_fused_scaled_add_sub_store_helper_line(
    target_index: usize,
    inner: &str,
    offset: &str,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(inner, "add_scaled_inputs") {
        if args.len() != 4 {
            return None;
        }
        return compact_offset_scaled_add_sub_helper_line(
            target_index,
            "add",
            args[0],
            args[1],
            args[2],
            args[3],
            offset,
        );
    }

    let args = compact_ad_call_args(inner, "sub_scaled_inputs")?;
    if args.len() != 4 {
        return None;
    }
    compact_offset_scaled_add_sub_helper_line(
        target_index,
        "sub",
        args[0],
        args[1],
        args[2],
        args[3],
        offset,
    )
}

fn compact_offset_scaled_add_sub_helper_line(
    target_index: usize,
    op: &str,
    left: &str,
    left_scale: &str,
    right: &str,
    right_scale: &str,
    offset: &str,
) -> Option<String> {
    if !compact_scalar_same(left_scale, right_scale) {
        return None;
    }
    let left = compact_scratch_ad_value_index(left)?;
    let right = compact_scratch_ad_value_index(right)?;
    Some(format!(
        "scratch.store_offset_scaled_{op}({target_index}, {left}, {right}, {left_scale}, {offset});"
    ))
}

fn compact_nested_offset_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    let args = compact_ad_call_args(value, "offset")?;
    if args.len() != 2 {
        return None;
    }
    let inner = args[0];
    let offset = args[1];

    if let Some(line) =
        compact_offset_fused_scaled_multiply_store_helper_line(target_index, inner, offset)
    {
        return Some(line);
    }

    if let Some(line) =
        compact_offset_fused_scaled_add_sub_store_helper_line(target_index, inner, offset)
    {
        return Some(line);
    }

    if let Some(inner_args) = compact_ad_call_args(inner, "scale") {
        if inner_args.len() != 2 {
            return None;
        }
        if let Some((binary, left, right)) = compact_direct_binary_scratch_args(inner_args[0]) {
            return Some(format!(
                "scratch.store_offset_scaled_{binary}({target_index}, {left}, {right}, {}, {offset});",
                inner_args[1]
            ));
        }
        if let Some(source) = compact_scratch_ad_value_index(inner_args[0]) {
            return Some(format!(
                "scratch.store_offset_scaled({target_index}, {source}, {}, {offset});",
                inner_args[1]
            ));
        }
        if compact_non_atomic_ad_value(inner_args[0]) {
            return Some(format!(
                "scratch.store_offset_scaled_ad({target_index}, {}, {}, {offset});",
                inner_args[0], inner_args[1]
            ));
        }
        return None;
    }

    if let Some(inner_args) = compact_ad_call_args(inner, "offset") {
        if inner_args.len() != 2 {
            return None;
        }
        let combined_offset = format!("(({}) + ({}))", inner_args[1], offset);
        if let Some(source) = compact_scratch_ad_value_index(inner_args[0]) {
            return Some(format!(
                "scratch.store_offset({target_index}, {source}, {combined_offset});",
            ));
        }
        if compact_dynamic_ad_value(inner_args[0]) {
            return Some(format!(
                "scratch.store_offset_scaled_ad({target_index}, {}, 1.0, {combined_offset});",
                inner_args[0]
            ));
        }
        return None;
    }

    if let Some(inner_args) = compact_ad_call_args(inner, "neg") {
        if inner_args.len() != 1 {
            return None;
        }
        if let Some(source) = compact_scratch_ad_value_index(inner_args[0]) {
            return Some(format!(
                "scratch.store_offset_scaled({target_index}, {source}, -1.0, {offset});"
            ));
        }
        if compact_non_atomic_ad_value(inner_args[0]) {
            return Some(format!(
                "scratch.store_offset_scaled_ad({target_index}, {}, -1.0, {offset});",
                inner_args[0]
            ));
        }
        return None;
    }

    for (name, direct_helper, ad_helper) in [
        ("add", "store_offset_add", "store_offset_add_ad"),
        ("sub", "store_offset_sub", "store_offset_sub_ad"),
        ("mul", "store_offset_mul", "store_offset_mul_ad"),
        ("div", "store_offset_div", "store_offset_div_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 2 {
                return None;
            }
            let left = compact_scratch_ad_value_index(inner_args[0]);
            let right = compact_scratch_ad_value_index(inner_args[1]);
            if let (Some(left), Some(right)) = (left, right) {
                if name == "mul" && left == right {
                    return Some(format!(
                        "scratch.store_offset_square({target_index}, {left}, {offset});"
                    ));
                }
                return Some(format!(
                    "scratch.{direct_helper}({target_index}, {left}, {right}, {offset});"
                ));
            }
            let left = compact_scratch_or_non_atomic_ad_arg(inner_args[0])?;
            let right = compact_scratch_or_non_atomic_ad_arg(inner_args[1])?;
            return Some(format!(
                "scratch.{ad_helper}({target_index}, {left}, {right}, {offset});"
            ));
        }
    }

    for (name, helper) in [
        ("rem", "store_offset_rem_ad"),
        ("pow", "store_offset_pow_ad"),
        ("min", "store_offset_min_ad"),
        ("max", "store_offset_max_ad"),
        ("hypot", "store_offset_hypot_ad"),
        ("atan2", "store_offset_atan2_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 2 {
                return None;
            }
            let left = compact_scratch_or_non_atomic_ad_arg(inner_args[0])?;
            let right = compact_scratch_or_non_atomic_ad_arg(inner_args[1])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {left}, {right}, {offset});"
            ));
        }
    }

    for (name, helper) in [
        ("sub_from_scalar", "store_offset_sub_from_scalar_ad"),
        ("div_from_scalar", "store_offset_div_from_scalar_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 2 {
                return None;
            }
            let value = compact_scratch_or_non_atomic_ad_arg(inner_args[1])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {}, {value}, {offset});",
                inner_args[0]
            ));
        }
    }

    for (name, helper) in [
        ("rem_from_scalar", "store_offset_rem_from_scalar_ad"),
        ("pow_from_scalar", "store_offset_pow_from_scalar_ad"),
        ("min_from_scalar", "store_offset_min_from_scalar_ad"),
        ("max_from_scalar", "store_offset_max_from_scalar_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 2 {
                return None;
            }
            let value = compact_scratch_or_non_atomic_ad_arg(inner_args[1])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {}, {value}, {offset});",
                inner_args[0]
            ));
        }
    }

    for (name, helper) in [
        ("rem_with_scalar", "store_offset_rem_with_scalar_ad"),
        ("min_with_scalar", "store_offset_min_with_scalar_ad"),
        ("max_with_scalar", "store_offset_max_with_scalar_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 2 {
                return None;
            }
            let value = compact_scratch_or_non_atomic_ad_arg(inner_args[0])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {value}, {}, {offset});",
                inner_args[1]
            ));
        }
    }

    for (name, helper) in [
        ("sqrt", "store_offset_sqrt"),
        ("exp", "store_offset_exp"),
        ("ln", "store_offset_ln"),
        ("limexp", "store_offset_limexp"),
        ("limited_exp", "store_offset_limited_exp"),
        ("square", "store_offset_square"),
        ("abs", "store_offset_abs"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 1 {
                return None;
            }
            if let Some(source) = compact_scratch_ad_value_index(inner_args[0]) {
                return Some(format!(
                    "scratch.{helper}({target_index}, {source}, {offset});"
                ));
            }
            continue;
        }
    }

    for (name, helper) in [
        ("sqrt", "store_offset_sqrt_ad"),
        ("exp", "store_offset_exp_ad"),
        ("ln", "store_offset_ln_ad"),
        ("limexp", "store_offset_limexp_ad"),
        ("limited_exp", "store_offset_limited_exp_ad"),
        ("square", "store_offset_square_ad"),
        ("abs", "store_offset_abs_ad"),
        ("log10", "store_offset_log10_ad"),
        ("sin", "store_offset_sin_ad"),
        ("cos", "store_offset_cos_ad"),
        ("tan", "store_offset_tan_ad"),
        ("atan", "store_offset_atan_ad"),
        ("sinh", "store_offset_sinh_ad"),
        ("cosh", "store_offset_cosh_ad"),
        ("tanh", "store_offset_tanh_ad"),
        ("asinh", "store_offset_asinh_ad"),
        ("acosh", "store_offset_acosh_ad"),
        ("atanh", "store_offset_atanh_ad"),
        ("floor", "store_offset_floor_ad"),
        ("ceil", "store_offset_ceil_ad"),
    ] {
        if let Some(inner_args) = compact_ad_call_args(inner, name) {
            if inner_args.len() != 1 {
                return None;
            }
            let value = compact_scratch_or_non_atomic_ad_arg(inner_args[0])?;
            return Some(format!(
                "scratch.{helper}({target_index}, {value}, {offset});"
            ));
        }
    }

    if let Some(inner_args) = compact_ad_call_args(inner, "powf") {
        if inner_args.len() != 2 {
            return None;
        }
        let value = compact_scratch_or_non_atomic_ad_arg(inner_args[0])?;
        return Some(format!(
            "scratch.store_offset_powf_ad({target_index}, {value}, {}, {offset});",
            inner_args[1]
        ));
    }

    None
}

fn compact_scratch_or_non_atomic_ad_arg(value: &str) -> Option<String> {
    if let Some(source) = compact_scratch_ad_value_index(value) {
        return Some(format!("scratch.ad_value({source})"));
    }
    if compact_dynamic_ad_value(value) {
        return Some(value.to_string());
    }
    None
}

fn compact_dynamic_ad_value(value: &str) -> bool {
    if compact_ad_call_args(value, "constant").is_some() {
        return false;
    }
    value.starts_with("AdValue::") || value.starts_with('{') || compact_generated_ad_local(value)
}

fn compact_direct_binary_scratch_args(value: &str) -> Option<(&'static str, usize, usize)> {
    for binary in ["add", "sub", "mul", "div"] {
        let Some(args) = compact_ad_call_args(value, binary) else {
            continue;
        };
        if args.len() != 2 {
            return None;
        }
        let left = compact_scratch_ad_value_index(args[0])?;
        let right = compact_scratch_ad_value_index(args[1])?;
        return Some((binary, left, right));
    }

    None
}

fn compact_unary_binary_store_helper_call(target_index: usize, value: &str) -> Option<String> {
    for (unary, helper_prefix) in [
        ("sqrt", "store_sqrt"),
        ("exp", "store_exp"),
        ("ln", "store_ln"),
        ("limexp", "store_limexp"),
        ("limited_exp", "store_limited_exp"),
    ] {
        let Some(args) = compact_ad_call_args(value, unary) else {
            continue;
        };
        if args.len() != 1 {
            return None;
        }
        for binary in ["add", "sub", "mul", "div"] {
            let Some(binary_args) = compact_ad_call_args(args[0], binary) else {
                continue;
            };
            if binary_args.len() != 2 {
                return None;
            }
            let left = compact_scratch_ad_value_index(binary_args[0])?;
            let right = compact_scratch_ad_value_index(binary_args[1])?;
            return Some(format!(
                "scratch.{helper_prefix}_{binary}({target_index}, {left}, {right});"
            ));
        }
    }

    None
}

fn compact_ad_call_args<'a>(value: &'a str, name: &str) -> Option<Vec<&'a str>> {
    let value = value.trim();
    let prefix = format!("AdValue::{name}(");
    let inner = value.strip_prefix(prefix.as_str())?.strip_suffix(')')?;
    split_top_level_args(inner)
}

fn split_top_level_args(input: &str) -> Option<Vec<&str>> {
    let mut args = Vec::new();
    let mut start = 0;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;

    for (index, ch) in input.char_indices() {
        match ch {
            '(' => paren_depth = paren_depth.checked_add(1)?,
            ')' => paren_depth = paren_depth.checked_sub(1)?,
            '[' => bracket_depth = bracket_depth.checked_add(1)?,
            ']' => bracket_depth = bracket_depth.checked_sub(1)?,
            '{' => brace_depth = brace_depth.checked_add(1)?,
            '}' => brace_depth = brace_depth.checked_sub(1)?,
            ',' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                args.push(input[start..index].trim());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }

    if paren_depth != 0 || bracket_depth != 0 || brace_depth != 0 {
        return None;
    }
    args.push(input[start..].trim());
    Some(args)
}

fn compact_scaled_ad_expression(value: &str) -> Option<(&str, String)> {
    if let Some(args) = compact_ad_call_args(value, "scale") {
        if args.len() != 2 {
            return None;
        }
        return Some((args[0], args[1].to_string()));
    }

    let args = compact_ad_call_args(value, "neg")?;
    if args.len() != 1 {
        return None;
    }
    Some((args[0], "-1.0".to_string()))
}

struct CompactProduct2<'a> {
    left: Cow<'a, str>,
    right: Cow<'a, str>,
    scale: String,
}

struct CompactProduct3<'a> {
    left: &'a str,
    middle: &'a str,
    right: &'a str,
    scale: String,
}

struct CompactQuotient<'a> {
    numerator: Option<&'a str>,
    numerator_scale: String,
    denominator: &'a str,
    denominator_scale: String,
}

fn compact_product2_ad_expression(value: &str) -> Option<CompactProduct2<'_>> {
    if let Some(args) = compact_ad_call_args(value, "mul") {
        if args.len() != 2 {
            return None;
        }
        return Some(CompactProduct2 {
            left: args[0].into(),
            right: args[1].into(),
            scale: "1.0".to_string(),
        });
    }

    if let Some(args) = compact_ad_call_args(value, "mul_offset_lhs") {
        if args.len() != 3 {
            return None;
        }
        return Some(CompactProduct2 {
            left: format!("AdValue::offset({}, {})", args[0], args[1]).into(),
            right: args[2].into(),
            scale: "1.0".to_string(),
        });
    }

    if let Some(args) = compact_ad_call_args(value, "mul_offset_rhs") {
        if args.len() != 3 {
            return None;
        }
        return Some(CompactProduct2 {
            left: args[0].into(),
            right: format!("AdValue::offset({}, {})", args[1], args[2]).into(),
            scale: "1.0".to_string(),
        });
    }

    if let Some(args) = compact_ad_call_args(value, "mul_offset_lhs_scaled_output") {
        if args.len() != 4 {
            return None;
        }
        return Some(CompactProduct2 {
            left: format!("AdValue::offset({}, {})", args[0], args[1]).into(),
            right: args[2].into(),
            scale: args[3].to_string(),
        });
    }

    if let Some(args) = compact_ad_call_args(value, "mul_offset_rhs_scaled_output") {
        if args.len() != 4 {
            return None;
        }
        return Some(CompactProduct2 {
            left: args[0].into(),
            right: format!("AdValue::offset({}, {})", args[1], args[2]).into(),
            scale: args[3].to_string(),
        });
    }

    if let Some(args) = compact_ad_call_args(value, "mul_scaled_output") {
        if args.len() != 3 {
            return None;
        }
        return Some(CompactProduct2 {
            left: args[0].into(),
            right: args[1].into(),
            scale: args[2].to_string(),
        });
    }

    if let Some(args) = compact_ad_call_args(value, "mul_scaled_lhs") {
        if args.len() != 3 {
            return None;
        }
        return Some(CompactProduct2 {
            left: args[0].into(),
            right: args[2].into(),
            scale: args[1].to_string(),
        });
    }

    let args = compact_ad_call_args(value, "mul_scaled_rhs")?;
    if args.len() != 3 {
        return None;
    }
    Some(CompactProduct2 {
        left: args[0].into(),
        right: args[1].into(),
        scale: args[2].to_string(),
    })
}

fn compact_quotient_ad_expression(value: &str) -> Option<CompactQuotient<'_>> {
    let (value, output_scale) = compact_scaled_ad_expression(value)
        .map(|(value, scale)| (value, scale))
        .unwrap_or((value, "1.0".to_string()));

    if let Some(args) = compact_ad_call_args(value, "div") {
        if args.len() != 2 {
            return None;
        }
        return Some(CompactQuotient {
            numerator: Some(args[0]),
            numerator_scale: output_scale,
            denominator: args[1],
            denominator_scale: "1.0".to_string(),
        });
    }

    if let Some(args) = compact_ad_call_args(value, "div_scaled_inputs") {
        if args.len() != 4 {
            return None;
        }
        return Some(CompactQuotient {
            numerator: Some(args[0]),
            numerator_scale: compact_scalar_mul(args[1], &output_scale),
            denominator: args[2],
            denominator_scale: args[3].to_string(),
        });
    }

    let args = compact_ad_call_args(value, "div_from_scalar")?;
    if args.len() != 2 {
        return None;
    }
    Some(CompactQuotient {
        numerator: None,
        numerator_scale: compact_scalar_mul(args[0], &output_scale),
        denominator: args[1],
        denominator_scale: "1.0".to_string(),
    })
}

fn compact_product3_ad_expression(value: &str) -> Option<CompactProduct3<'_>> {
    if let Some(args) = compact_ad_call_args(value, "mul3") {
        if args.len() != 3 {
            return None;
        }
        return Some(CompactProduct3 {
            left: args[0],
            middle: args[1],
            right: args[2],
            scale: "1.0".to_string(),
        });
    }

    let args = compact_ad_call_args(value, "mul3_scaled_output")?;
    if args.len() != 4 {
        return None;
    }
    Some(CompactProduct3 {
        left: args[0],
        middle: args[1],
        right: args[2],
        scale: args[3].to_string(),
    })
}

fn compact_square_ad_expression(value: &str) -> Option<&str> {
    let args = compact_ad_call_args(value, "square")?;
    if args.len() != 1 {
        return None;
    }
    Some(args[0])
}

fn compact_scaled_square_ad_expression(value: &str) -> Option<(&str, String)> {
    if let Some((inner, scale)) = compact_scaled_ad_expression(value) {
        return compact_square_ad_expression(inner).map(|inner| (inner, scale));
    }

    compact_square_ad_expression(value).map(|inner| (inner, "1.0".to_string()))
}

fn compact_div_product2_ad_expression(value: &str) -> Option<CompactProduct2<'_>> {
    if let Some(product) = compact_product2_ad_expression(value) {
        return Some(product);
    }

    let square = compact_square_ad_expression(value)?;
    Some(CompactProduct2 {
        left: square.into(),
        right: square.into(),
        scale: "1.0".to_string(),
    })
}

fn compact_scaled_factor_ad_expression(value: &str) -> (&str, String) {
    compact_scaled_ad_expression(value).unwrap_or((value, "1.0".to_string()))
}

struct CompactAffineTerm<'a> {
    value: &'a str,
    scale: String,
}

fn compact_accumulate_scalar_offset(offset: &mut String, term: &str) {
    *offset = compact_scalar_add(offset, term);
}

fn compact_collect_affine_ad_terms<'a>(
    value: &'a str,
    scale: &str,
    terms: &mut Vec<CompactAffineTerm<'a>>,
    max_terms: usize,
) -> Option<()> {
    if terms.len() >= max_terms {
        return None;
    }

    if let Some((inner, inner_scale)) = compact_scaled_ad_expression(value) {
        let combined_scale = compact_scalar_mul(scale, &inner_scale);
        return compact_collect_affine_ad_terms(inner, &combined_scale, terms, max_terms);
    }

    if let Some(args) = compact_ad_call_args(value, "add") {
        if args.len() != 2 {
            return None;
        }
        compact_collect_affine_ad_terms(args[0], scale, terms, max_terms)?;
        compact_collect_affine_ad_terms(args[1], scale, terms, max_terms)?;
        return Some(());
    }

    if let Some(args) = compact_ad_call_args(value, "sub") {
        if args.len() != 2 {
            return None;
        }
        compact_collect_affine_ad_terms(args[0], scale, terms, max_terms)?;
        let right_scale = compact_scalar_negate(scale);
        compact_collect_affine_ad_terms(args[1], &right_scale, terms, max_terms)?;
        return Some(());
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_inputs") {
        if args.len() != 4 {
            return None;
        }
        compact_collect_affine_ad_terms(
            args[0],
            &compact_scalar_mul(scale, args[1]),
            terms,
            max_terms,
        )?;
        compact_collect_affine_ad_terms(
            args[2],
            &compact_scalar_mul(scale, args[3]),
            terms,
            max_terms,
        )?;
        return Some(());
    }

    if let Some(args) = compact_ad_call_args(value, "sub_scaled_inputs") {
        if args.len() != 4 {
            return None;
        }
        compact_collect_affine_ad_terms(
            args[0],
            &compact_scalar_mul(scale, args[1]),
            terms,
            max_terms,
        )?;
        compact_collect_affine_ad_terms(
            args[2],
            &compact_scalar_negate(&compact_scalar_mul(scale, args[3])),
            terms,
            max_terms,
        )?;
        return Some(());
    }

    terms.push(CompactAffineTerm {
        value,
        scale: scale.to_string(),
    });
    Some(())
}

fn compact_collect_affine_offset_ad_terms<'a>(
    value: &'a str,
    scale: &str,
    terms: &mut Vec<CompactAffineTerm<'a>>,
    offset: &mut String,
    max_terms: usize,
) -> Option<()> {
    if terms.len() >= max_terms {
        return None;
    }

    if let Some((inner, inner_scale)) = compact_scaled_ad_expression(value) {
        let combined_scale = compact_scalar_mul(scale, &inner_scale);
        return compact_collect_affine_offset_ad_terms(
            inner,
            &combined_scale,
            terms,
            offset,
            max_terms,
        );
    }

    if let Some(args) = compact_ad_call_args(value, "scale_offset") {
        if args.len() != 3 {
            return None;
        }
        compact_accumulate_scalar_offset(offset, &compact_scalar_mul(scale, args[2]));
        return compact_collect_affine_offset_ad_terms(
            args[0],
            &compact_scalar_mul(scale, args[1]),
            terms,
            offset,
            max_terms,
        );
    }

    if let Some(args) = compact_ad_call_args(value, "scaled_offset") {
        if args.len() != 3 {
            return None;
        }
        compact_accumulate_scalar_offset(
            offset,
            &compact_scalar_mul(scale, &compact_scalar_mul(args[1], args[2])),
        );
        return compact_collect_affine_offset_ad_terms(
            args[0],
            &compact_scalar_mul(scale, args[2]),
            terms,
            offset,
            max_terms,
        );
    }

    if let Some(args) = compact_ad_call_args(value, "offset") {
        if args.len() != 2 {
            return None;
        }
        compact_accumulate_scalar_offset(offset, &compact_scalar_mul(scale, args[1]));
        return compact_collect_affine_offset_ad_terms(args[0], scale, terms, offset, max_terms);
    }

    if let Some(args) = compact_ad_call_args(value, "sub_from_scalar") {
        if args.len() != 2 {
            return None;
        }
        compact_accumulate_scalar_offset(offset, &compact_scalar_mul(scale, args[0]));
        return compact_collect_affine_offset_ad_terms(
            args[1],
            &compact_scalar_negate(scale),
            terms,
            offset,
            max_terms,
        );
    }

    if let Some(args) = compact_ad_call_args(value, "add") {
        if args.len() != 2 {
            return None;
        }
        compact_collect_affine_offset_ad_terms(args[0], scale, terms, offset, max_terms)?;
        compact_collect_affine_offset_ad_terms(args[1], scale, terms, offset, max_terms)?;
        return Some(());
    }

    if let Some(args) = compact_ad_call_args(value, "sub") {
        if args.len() != 2 {
            return None;
        }
        compact_collect_affine_offset_ad_terms(args[0], scale, terms, offset, max_terms)?;
        let right_scale = compact_scalar_negate(scale);
        compact_collect_affine_offset_ad_terms(args[1], &right_scale, terms, offset, max_terms)?;
        return Some(());
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_inputs") {
        if args.len() != 4 {
            return None;
        }
        compact_collect_affine_offset_ad_terms(
            args[0],
            &compact_scalar_mul(scale, args[1]),
            terms,
            offset,
            max_terms,
        )?;
        compact_collect_affine_offset_ad_terms(
            args[2],
            &compact_scalar_mul(scale, args[3]),
            terms,
            offset,
            max_terms,
        )?;
        return Some(());
    }

    if let Some(args) = compact_ad_call_args(value, "sub_scaled_inputs") {
        if args.len() != 4 {
            return None;
        }
        compact_collect_affine_offset_ad_terms(
            args[0],
            &compact_scalar_mul(scale, args[1]),
            terms,
            offset,
            max_terms,
        )?;
        compact_collect_affine_offset_ad_terms(
            args[2],
            &compact_scalar_negate(&compact_scalar_mul(scale, args[3])),
            terms,
            offset,
            max_terms,
        )?;
        return Some(());
    }

    terms.push(CompactAffineTerm {
        value,
        scale: scale.to_string(),
    });
    Some(())
}

fn compact_add_sub_affine3_ad_expressions(helper: &str, left: &str, right: &str) -> Option<String> {
    let mut terms = Vec::with_capacity(3);
    compact_collect_affine_ad_terms(left, "1.0", &mut terms, 3)?;
    let right_scale = if helper == "sub" { "-1.0" } else { "1.0" };
    compact_collect_affine_ad_terms(right, right_scale, &mut terms, 3)?;
    if terms.len() != 3 {
        return None;
    }
    Some(format!(
        "AdValue::add_scaled_inputs3({}, {}, {}, {}, {}, {})",
        terms[0].value,
        terms[0].scale,
        terms[1].value,
        terms[1].scale,
        terms[2].value,
        terms[2].scale
    ))
}

fn compact_add_sub_affine3_offset_ad_expressions(
    helper: &str,
    left: &str,
    right: &str,
) -> Option<String> {
    let mut terms = Vec::with_capacity(3);
    let mut offset = "0.0".to_string();
    compact_collect_affine_offset_ad_terms(left, "1.0", &mut terms, &mut offset, 3)?;
    let right_scale = if helper == "sub" { "-1.0" } else { "1.0" };
    compact_collect_affine_offset_ad_terms(right, right_scale, &mut terms, &mut offset, 3)?;
    if terms.len() != 3 || compact_scalar_same(&offset, "0.0") {
        return None;
    }
    Some(format!(
        "AdValue::add_scaled_inputs3_offset({}, {}, {}, {}, {}, {}, {})",
        terms[0].value,
        terms[0].scale,
        terms[1].value,
        terms[1].scale,
        terms[2].value,
        terms[2].scale,
        offset
    ))
}

fn compact_mul3_ad_expression(left: &str, middle: &str, right: &str, scale: &str) -> String {
    if compact_scalar_same(scale, "1.0") {
        format!("AdValue::mul3({left}, {middle}, {right})")
    } else {
        format!("AdValue::mul3_scaled_output({left}, {middle}, {right}, {scale})")
    }
}

fn compact_multiply_product3_ad_expressions(
    left: &str,
    right: &str,
    output_scale: &str,
) -> Option<String> {
    if let Some(product) = compact_product2_ad_expression(left) {
        let right_is_product = compact_product2_ad_expression(right).is_some();
        let (right, right_scale) = if right_is_product {
            (right, "1.0".to_string())
        } else {
            compact_scaled_factor_ad_expression(right)
        };
        let scale = compact_scalar_mul(
            &compact_scalar_mul(&product.scale, &right_scale),
            output_scale,
        );
        return Some(compact_mul3_ad_expression(
            product.left.as_ref(),
            product.right.as_ref(),
            right,
            &scale,
        ));
    }

    let product = compact_product2_ad_expression(right)?;
    let (left, left_scale) = compact_scaled_factor_ad_expression(left);
    let scale = compact_scalar_mul(
        &compact_scalar_mul(&left_scale, &product.scale),
        output_scale,
    );
    Some(compact_mul3_ad_expression(
        left,
        product.left.as_ref(),
        product.right.as_ref(),
        &scale,
    ))
}

fn compact_multiply_scaled_ad_expressions(left: &str, right: &str) -> Option<String> {
    if let Some(fused) = compact_multiply_product3_ad_expressions(left, right, "1.0") {
        return Some(fused);
    }

    let left_scaled = compact_scaled_ad_expression(left);
    let right_scaled = compact_scaled_ad_expression(right);
    let left_value = left_scaled
        .as_ref()
        .map(|(value, _)| *value)
        .unwrap_or(left);
    let right_value = right_scaled
        .as_ref()
        .map(|(value, _)| *value)
        .unwrap_or(right);
    let output_scale = compact_scalar_mul(
        left_scaled
            .as_ref()
            .map(|(_, scale)| scale.as_str())
            .unwrap_or("1.0"),
        right_scaled
            .as_ref()
            .map(|(_, scale)| scale.as_str())
            .unwrap_or("1.0"),
    );
    if let Some(fused) =
        compact_multiply_sub_from_scalar_ad_expression(left_value, right_value, &output_scale)
    {
        return Some(fused);
    }
    if let Some(fused) =
        compact_multiply_offset_ad_expression(left_value, right_value, &output_scale)
    {
        return Some(fused);
    }

    match (left_scaled, right_scaled) {
        (Some((left, left_scale)), Some((right, right_scale))) => Some(format!(
            "AdValue::mul_scaled_output({left}, {right}, {})",
            compact_scalar_mul(&left_scale, &right_scale)
        )),
        (Some((left, scale)), None) => {
            Some(format!("AdValue::mul_scaled_lhs({left}, {scale}, {right})"))
        }
        (None, Some((right, scale))) => {
            Some(format!("AdValue::mul_scaled_rhs({left}, {right}, {scale})"))
        }
        (None, None) => None,
    }
}

fn compact_multiply_offset_ad_expression(
    left: &str,
    right: &str,
    output_scale: &str,
) -> Option<String> {
    if let Some(args) = compact_ad_call_args(left, "offset") {
        if args.len() != 2 {
            return None;
        }
        return if compact_scalar_same(output_scale, "1.0") {
            Some(format!(
                "AdValue::mul_offset_lhs({}, {}, {right})",
                args[0], args[1]
            ))
        } else {
            Some(format!(
                "AdValue::mul_offset_lhs_scaled_output({}, {}, {right}, {output_scale})",
                args[0], args[1]
            ))
        };
    }

    if let Some(args) = compact_ad_call_args(right, "offset") {
        if args.len() != 2 {
            return None;
        }
        return if compact_scalar_same(output_scale, "1.0") {
            Some(format!(
                "AdValue::mul_offset_rhs({left}, {}, {})",
                args[0], args[1]
            ))
        } else {
            Some(format!(
                "AdValue::mul_offset_rhs_scaled_output({left}, {}, {}, {output_scale})",
                args[0], args[1]
            ))
        };
    }

    None
}

fn compact_multiply_sub_from_scalar_ad_expression(
    left: &str,
    right: &str,
    output_scale: &str,
) -> Option<String> {
    if let Some(fused) =
        compact_multiply_sub_from_scalar_scaled_offset_self_expression(left, right, output_scale)
    {
        return Some(fused);
    }

    if let Some(args) = compact_ad_call_args(left, "sub_from_scalar") {
        if args.len() != 2 {
            return None;
        }
        return if compact_scalar_same(output_scale, "1.0") {
            Some(format!(
                "AdValue::mul_sub_from_scalar_lhs({}, {}, {right})",
                args[0], args[1]
            ))
        } else {
            Some(format!(
                "AdValue::mul_sub_from_scalar_lhs_scaled_output({}, {}, {right}, {output_scale})",
                args[0], args[1]
            ))
        };
    }

    if let Some(args) = compact_ad_call_args(right, "sub_from_scalar") {
        if args.len() != 2 {
            return None;
        }
        return if compact_scalar_same(output_scale, "1.0") {
            Some(format!(
                "AdValue::mul_sub_from_scalar_rhs({left}, {}, {})",
                args[0], args[1]
            ))
        } else {
            Some(format!(
                "AdValue::mul_sub_from_scalar_rhs_scaled_output({left}, {}, {}, {output_scale})",
                args[0], args[1]
            ))
        };
    }

    None
}

fn compact_multiply_sub_from_scalar_scaled_offset_self_expression(
    left: &str,
    right: &str,
    output_scale: &str,
) -> Option<String> {
    let left_args = compact_ad_call_args(left, "sub_from_scalar")?;
    if left_args.len() != 2 {
        return None;
    }

    let right_args = compact_ad_call_args(right, "scale_offset")?;
    if right_args.len() != 3 {
        return None;
    }
    let right_sub_args = compact_ad_call_args(right_args[0], "sub_from_scalar")?;
    if right_sub_args.len() != 2 {
        return None;
    }

    if !compact_scalar_same(left_args[0], right_sub_args[0])
        || !compact_ad_expression_same(left_args[1], right_sub_args[1])
    {
        return None;
    }

    Some(format!(
        "AdValue::mul_sub_from_scalar_scaled_offset_self({}, {}, {}, {}, {})",
        left_args[0], left_args[1], right_args[1], right_args[2], output_scale
    ))
}

fn compact_div_scaled_ad_expressions(left: &str, right: &str) -> Option<String> {
    let left_scaled = compact_scaled_ad_expression(left);
    let right_scaled = compact_scaled_ad_expression(right);
    if left_scaled.is_none() && right_scaled.is_none() {
        return None;
    }
    let (left, left_scale) = left_scaled.unwrap_or((left, "1.0".to_string()));
    let (right, right_scale) = right_scaled.unwrap_or((right, "1.0".to_string()));
    Some(format!(
        "AdValue::div_scaled_inputs({left}, {left_scale}, {right}, {right_scale})"
    ))
}

fn compact_div_product_ad_expression(left: &str, right: &str) -> Option<String> {
    let (denominator, denominator_scale) = compact_scaled_factor_ad_expression(right);
    if let Some(product) = compact_product3_ad_expression(left) {
        if let Some(quotient) = compact_quotient_ad_expression(product.left) {
            let product_scale = compact_scalar_mul(&product.scale, &quotient.numerator_scale);
            let denominator_scale =
                compact_scalar_mul(&denominator_scale, &quotient.denominator_scale);
            if let Some(numerator) = quotient.numerator {
                return Some(format!(
                    "AdValue::div_scaled_product3_by_product({}, {}, {}, {}, {}, {}, {})",
                    numerator,
                    product.middle,
                    product.right,
                    product_scale,
                    quotient.denominator,
                    denominator,
                    denominator_scale
                ));
            }
            return Some(format!(
                "AdValue::div_scaled_product_by_product({}, {}, {}, {}, {}, {})",
                product.middle,
                product.right,
                product_scale,
                quotient.denominator,
                denominator,
                denominator_scale
            ));
        }
        if let Some(quotient) = compact_quotient_ad_expression(product.middle) {
            let product_scale = compact_scalar_mul(&product.scale, &quotient.numerator_scale);
            let denominator_scale =
                compact_scalar_mul(&denominator_scale, &quotient.denominator_scale);
            if let Some(numerator) = quotient.numerator {
                return Some(format!(
                    "AdValue::div_scaled_product3_by_product({}, {}, {}, {}, {}, {}, {})",
                    product.left,
                    numerator,
                    product.right,
                    product_scale,
                    quotient.denominator,
                    denominator,
                    denominator_scale
                ));
            }
            return Some(format!(
                "AdValue::div_scaled_product_by_product({}, {}, {}, {}, {}, {})",
                product.left,
                product.right,
                product_scale,
                quotient.denominator,
                denominator,
                denominator_scale
            ));
        }
        if let Some(quotient) = compact_quotient_ad_expression(product.right) {
            let product_scale = compact_scalar_mul(&product.scale, &quotient.numerator_scale);
            let denominator_scale =
                compact_scalar_mul(&denominator_scale, &quotient.denominator_scale);
            if let Some(numerator) = quotient.numerator {
                return Some(format!(
                    "AdValue::div_scaled_product3_by_product({}, {}, {}, {}, {}, {}, {})",
                    product.left,
                    product.middle,
                    numerator,
                    product_scale,
                    quotient.denominator,
                    denominator,
                    denominator_scale
                ));
            }
            return Some(format!(
                "AdValue::div_scaled_product_by_product({}, {}, {}, {}, {}, {})",
                product.left,
                product.middle,
                product_scale,
                quotient.denominator,
                denominator,
                denominator_scale
            ));
        }
        if let Some(denominator_product) = compact_product2_ad_expression(denominator) {
            let denominator_product_scale =
                compact_scalar_mul(&denominator_product.scale, &denominator_scale);
            return Some(format!(
                "AdValue::div_scaled_product3_by_product({}, {}, {}, {}, {}, {}, {})",
                product.left,
                product.middle,
                product.right,
                product.scale,
                denominator_product.left,
                denominator_product.right,
                denominator_product_scale
            ));
        }
        return Some(format!(
            "AdValue::div_scaled_product3({}, {}, {}, {}, {denominator}, {denominator_scale})",
            product.left, product.middle, product.right, product.scale
        ));
    }

    let product = compact_div_product2_ad_expression(left)?;
    if let Some(quotient) = compact_quotient_ad_expression(product.left.as_ref()) {
        let product_scale = compact_scalar_mul(&product.scale, &quotient.numerator_scale);
        let denominator_scale = compact_scalar_mul(&denominator_scale, &quotient.denominator_scale);
        if let Some(numerator) = quotient.numerator {
            return Some(format!(
                "AdValue::div_scaled_product_by_product({}, {}, {}, {}, {}, {})",
                numerator,
                product.right,
                product_scale,
                quotient.denominator,
                denominator,
                denominator_scale
            ));
        }
        return Some(format!(
            "AdValue::div_scaled_value_by_product({}, {}, {}, {}, {})",
            product.right, product_scale, quotient.denominator, denominator, denominator_scale
        ));
    }
    if let Some(quotient) = compact_quotient_ad_expression(product.right.as_ref()) {
        let product_scale = compact_scalar_mul(&product.scale, &quotient.numerator_scale);
        let denominator_scale = compact_scalar_mul(&denominator_scale, &quotient.denominator_scale);
        if let Some(numerator) = quotient.numerator {
            return Some(format!(
                "AdValue::div_scaled_product_by_product({}, {}, {}, {}, {}, {})",
                product.left,
                numerator,
                product_scale,
                quotient.denominator,
                denominator,
                denominator_scale
            ));
        }
        return Some(format!(
            "AdValue::div_scaled_value_by_product({}, {}, {}, {}, {})",
            product.left, product_scale, quotient.denominator, denominator, denominator_scale
        ));
    }
    if let Some(denominator_product) = compact_product2_ad_expression(denominator) {
        let denominator_product_scale =
            compact_scalar_mul(&denominator_product.scale, &denominator_scale);
        return Some(format!(
            "AdValue::div_scaled_product_by_product({}, {}, {}, {}, {}, {})",
            product.left,
            product.right,
            product.scale,
            denominator_product.left,
            denominator_product.right,
            denominator_product_scale
        ));
    }
    if let Some(args) = compact_ad_call_args(denominator, "offset") {
        if args.len() == 2 {
            return Some(format!(
                "AdValue::div_scaled_product_offset_denominator({}, {}, {}, {}, {}, {denominator_scale})",
                product.left, product.right, product.scale, args[0], args[1]
            ));
        }
    }
    if let Some(args) = compact_ad_call_args(product.left.as_ref(), "offset") {
        if args.len() == 2 {
            return Some(format!(
                "AdValue::div_scaled_product_offset_lhs({}, {}, {}, {}, {denominator}, {denominator_scale})",
                args[0], args[1], product.right, product.scale
            ));
        }
    }
    if let Some(args) = compact_ad_call_args(product.right.as_ref(), "offset") {
        if args.len() == 2 {
            return Some(format!(
                "AdValue::div_scaled_product_offset_rhs({}, {}, {}, {}, {denominator}, {denominator_scale})",
                product.left, args[0], args[1], product.scale
            ));
        }
    }
    Some(format!(
        "AdValue::div_scaled_product({}, {}, {}, {denominator}, {denominator_scale})",
        product.left, product.right, product.scale
    ))
}

fn compact_add_sub_square_product_ad_expressions(
    helper: &str,
    left: &str,
    right: &str,
) -> Option<String> {
    let subtract = helper == "sub";
    if let (Some((square, square_scale)), Some(product)) = (
        compact_scaled_square_ad_expression(left),
        compact_product2_ad_expression(right),
    ) {
        let product_scale = if subtract {
            compact_scalar_negate(&product.scale)
        } else {
            product.scale
        };
        return Some(format!(
            "AdValue::add_scaled_square_product({square}, {square_scale}, {}, {}, {product_scale})",
            product.left, product.right
        ));
    }

    if let (Some(product), Some((square, square_scale))) = (
        compact_product2_ad_expression(left),
        compact_scaled_square_ad_expression(right),
    ) {
        let square_scale = if subtract {
            compact_scalar_negate(&square_scale)
        } else {
            square_scale
        };
        return Some(format!(
            "AdValue::add_scaled_square_product({square}, {square_scale}, {}, {}, {})",
            product.left, product.right, product.scale
        ));
    }

    None
}

fn compact_add_sub_product_ad_expressions(helper: &str, left: &str, right: &str) -> Option<String> {
    if let Some(fused) = compact_add_sub_square_product_ad_expressions(helper, left, right) {
        return Some(fused);
    }

    let subtract = helper == "sub";
    let left_product = compact_product2_ad_expression(left);
    let right_product = compact_product2_ad_expression(right);
    match (left_product, right_product) {
        (Some(left_product), Some(right_product)) => {
            let right_scale = if subtract {
                compact_scalar_negate(&right_product.scale)
            } else {
                right_product.scale
            };
            Some(format!(
                "AdValue::add_scaled_products({}, {}, {}, {}, {}, {})",
                left_product.left,
                left_product.right,
                left_product.scale,
                right_product.left,
                right_product.right,
                right_scale
            ))
        }
        (Some(product), None) => {
            let (value, value_scale) = compact_scaled_factor_ad_expression(right);
            let value_scale = if subtract {
                compact_scalar_negate(&value_scale)
            } else {
                value_scale
            };
            Some(compact_add_scaled_product_ad_expression(
                value,
                &value_scale,
                &product,
            ))
        }
        (None, Some(product)) => {
            let (value, value_scale) = compact_scaled_factor_ad_expression(left);
            let product_scale = if subtract {
                compact_scalar_negate(&product.scale)
            } else {
                product.scale
            };
            Some(compact_add_scaled_product_ad_expression(
                value,
                &value_scale,
                &CompactProduct2 {
                    left: product.left,
                    right: product.right,
                    scale: product_scale,
                },
            ))
        }
        (None, None) => None,
    }
}

fn compact_add_scaled_product_ad_expression(
    value: &str,
    value_scale: &str,
    product: &CompactProduct2<'_>,
) -> String {
    if let Some(args) = compact_ad_call_args(value, "add_scaled_inputs") {
        if args.len() == 4 {
            return format!(
                "AdValue::add_scaled_inputs_product({}, {}, {}, {}, {}, {}, {})",
                args[0],
                compact_scalar_mul(args[1], value_scale),
                args[2],
                compact_scalar_mul(args[3], value_scale),
                product.left,
                product.right,
                product.scale
            );
        }
    }

    if let Some(args) = compact_ad_call_args(value, "sub_scaled_inputs") {
        if args.len() == 4 {
            let second_scale = compact_scalar_negate(&compact_scalar_mul(args[3], value_scale));
            return format!(
                "AdValue::add_scaled_inputs_product({}, {}, {}, {}, {}, {}, {})",
                args[0],
                compact_scalar_mul(args[1], value_scale),
                args[2],
                second_scale,
                product.left,
                product.right,
                product.scale
            );
        }
    }

    if let Some(args) = compact_ad_call_args(value, "add") {
        if args.len() == 2 {
            return format!(
                "AdValue::add_scaled_inputs_product({}, {value_scale}, {}, {value_scale}, {}, {}, {})",
                args[0], args[1], product.left, product.right, product.scale
            );
        }
    }

    if let Some(args) = compact_ad_call_args(value, "sub") {
        if args.len() == 2 {
            return format!(
                "AdValue::add_scaled_inputs_product({}, {value_scale}, {}, {}, {}, {}, {})",
                args[0],
                args[1],
                compact_scalar_negate(value_scale),
                product.left,
                product.right,
                product.scale
            );
        }
    }

    if let Some(args) = compact_ad_call_args(value, "sub_from_scalar") {
        if args.len() == 2 {
            return format!(
                "AdValue::add_scaled_sub_value_product({}, {}, {value_scale}, {}, {}, {})",
                args[0], args[1], product.left, product.right, product.scale
            );
        }
    }

    if let Some(args) = compact_ad_call_args(product.left.as_ref(), "offset") {
        if args.len() == 2 {
            return format!(
                "AdValue::add_scaled_offset_product_lhs({value}, {value_scale}, {}, {}, {}, {})",
                args[0], args[1], product.right, product.scale
            );
        }
    }

    if let Some(args) = compact_ad_call_args(product.right.as_ref(), "offset") {
        if args.len() == 2 {
            return format!(
                "AdValue::add_scaled_offset_product_rhs({value}, {value_scale}, {}, {}, {}, {})",
                product.left, args[0], args[1], product.scale
            );
        }
    }

    if let Some(args) = compact_ad_call_args(product.left.as_ref(), "sub_from_scalar") {
        if args.len() == 2 {
            return format!(
                "AdValue::add_scaled_sub_product_lhs({value}, {value_scale}, {}, {}, {}, {})",
                args[0], args[1], product.right, product.scale
            );
        }
    }

    if let Some(args) = compact_ad_call_args(product.right.as_ref(), "sub_from_scalar") {
        if args.len() == 2 {
            return format!(
                "AdValue::add_scaled_sub_product_rhs({value}, {value_scale}, {}, {}, {}, {})",
                product.left, args[0], args[1], product.scale
            );
        }
    }

    format!(
        "AdValue::add_scaled_product({value}, {value_scale}, {}, {}, {})",
        product.left, product.right, product.scale
    )
}

fn compact_add_sub_scaled_ad_expressions(helper: &str, left: &str, right: &str) -> Option<String> {
    if let Some(fused) = compact_add_sub_product_ad_expressions(helper, left, right) {
        return Some(fused);
    }
    if let Some(fused) = compact_add_sub_affine3_offset_ad_expressions(helper, left, right) {
        return Some(fused);
    }
    if let Some(fused) = compact_add_sub_affine3_ad_expressions(helper, left, right) {
        return Some(fused);
    }

    let left_scaled = compact_scaled_ad_expression(left);
    let right_scaled = compact_scaled_ad_expression(right);
    if left_scaled.is_none() && right_scaled.is_none() {
        return None;
    }
    let (left, left_scale) = left_scaled.unwrap_or((left, "1.0".to_string()));
    let (right, right_scale) = right_scaled.unwrap_or((right, "1.0".to_string()));
    Some(format!(
        "AdValue::{helper}_scaled_inputs({left}, {left_scale}, {right}, {right_scale})"
    ))
}

fn compact_scale_ad_value_expression(value: &str, scale: &str) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "scale") {
        if args.len() != 2 {
            return None;
        }
        return Some(format!(
            "AdValue::scale({}, {})",
            args[0],
            compact_scalar_mul(args[1], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "offset") {
        if args.len() != 2 {
            return None;
        }
        return Some(format!(
            "AdValue::scaled_offset({}, {}, {scale})",
            args[0], args[1]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "scale_offset") {
        if args.len() != 3 {
            return None;
        }
        return Some(format!(
            "AdValue::scale_offset({}, {}, {})",
            args[0],
            compact_scale_product(args[1], scale),
            compact_scale_product(args[2], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "scaled_offset") {
        if args.len() != 3 {
            return None;
        }
        return Some(format!(
            "AdValue::scaled_offset({}, {}, {})",
            args[0],
            args[1],
            compact_scale_product(args[2], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add") {
        if args.len() != 2 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_inputs({}, {scale}, {}, {scale})",
            args[0], args[1]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "sub") {
        if args.len() != 2 {
            return None;
        }
        return Some(format!(
            "AdValue::sub_scaled_inputs({}, {scale}, {}, {scale})",
            args[0], args[1]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_inputs") {
        if args.len() != 4 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_inputs({}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            compact_scalar_mul(args[3], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "sub_scaled_inputs") {
        if args.len() != 4 {
            return None;
        }
        return Some(format!(
            "AdValue::sub_scaled_inputs({}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            compact_scalar_mul(args[3], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_inputs3") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_inputs3({}, {}, {}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            compact_scalar_mul(args[3], scale),
            args[4],
            compact_scalar_mul(args[5], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_inputs3_offset") {
        if args.len() != 7 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_inputs3_offset({}, {}, {}, {}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            compact_scalar_mul(args[3], scale),
            args[4],
            compact_scalar_mul(args[5], scale),
            compact_scalar_mul(args[6], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "div_scaled_inputs") {
        if args.len() != 4 {
            return None;
        }
        return Some(format!(
            "AdValue::div_scaled_inputs({}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            args[3]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "div_scaled_value_by_product") {
        if args.len() != 5 {
            return None;
        }
        return Some(format!(
            "AdValue::div_scaled_value_by_product({}, {}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            args[3],
            args[4]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "div_scaled_product") {
        if args.len() != 5 {
            return None;
        }
        return Some(format!(
            "AdValue::div_scaled_product({}, {}, {}, {}, {})",
            args[0],
            args[1],
            compact_scalar_mul(args[2], scale),
            args[3],
            args[4]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "div_scaled_product_by_product") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::div_scaled_product_by_product({}, {}, {}, {}, {}, {})",
            args[0],
            args[1],
            compact_scalar_mul(args[2], scale),
            args[3],
            args[4],
            args[5]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "div_scaled_product_offset_lhs") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::div_scaled_product_offset_lhs({}, {}, {}, {}, {}, {})",
            args[0],
            args[1],
            args[2],
            compact_scalar_mul(args[3], scale),
            args[4],
            args[5]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "div_scaled_product_offset_rhs") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::div_scaled_product_offset_rhs({}, {}, {}, {}, {}, {})",
            args[0],
            args[1],
            args[2],
            compact_scalar_mul(args[3], scale),
            args[4],
            args[5]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "div_scaled_product_offset_denominator") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::div_scaled_product_offset_denominator({}, {}, {}, {}, {}, {})",
            args[0],
            args[1],
            compact_scalar_mul(args[2], scale),
            args[3],
            args[4],
            args[5]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "div_scaled_product3_by_product") {
        if args.len() != 7 {
            return None;
        }
        return Some(format!(
            "AdValue::div_scaled_product3_by_product({}, {}, {}, {}, {}, {}, {})",
            args[0],
            args[1],
            args[2],
            compact_scalar_mul(args[3], scale),
            args[4],
            args[5],
            args[6]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "div_scaled_product3") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::div_scaled_product3({}, {}, {}, {}, {}, {})",
            args[0],
            args[1],
            args[2],
            compact_scalar_mul(args[3], scale),
            args[4],
            args[5]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_product") {
        if args.len() != 5 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_product({}, {}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            args[3],
            compact_scalar_mul(args[4], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_offset_product_lhs") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_offset_product_lhs({}, {}, {}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            args[3],
            args[4],
            compact_scalar_mul(args[5], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_offset_product_rhs") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_offset_product_rhs({}, {}, {}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            args[3],
            args[4],
            compact_scalar_mul(args[5], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_inputs_product") {
        if args.len() != 7 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_inputs_product({}, {}, {}, {}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            compact_scalar_mul(args[3], scale),
            args[4],
            args[5],
            compact_scalar_mul(args[6], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_sub_value_product") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_sub_value_product({}, {}, {}, {}, {}, {})",
            args[0],
            args[1],
            compact_scalar_mul(args[2], scale),
            args[3],
            args[4],
            compact_scalar_mul(args[5], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_sub_product_lhs") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_sub_product_lhs({}, {}, {}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            args[3],
            args[4],
            compact_scalar_mul(args[5], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_sub_product_rhs") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_sub_product_rhs({}, {}, {}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            args[3],
            args[4],
            compact_scalar_mul(args[5], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_square_product") {
        if args.len() != 5 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_square_product({}, {}, {}, {}, {})",
            args[0],
            compact_scalar_mul(args[1], scale),
            args[2],
            args[3],
            compact_scalar_mul(args[4], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "add_scaled_products") {
        if args.len() != 6 {
            return None;
        }
        return Some(format!(
            "AdValue::add_scaled_products({}, {}, {}, {}, {}, {})",
            args[0],
            args[1],
            compact_scalar_mul(args[2], scale),
            args[3],
            args[4],
            compact_scalar_mul(args[5], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_scaled_output") {
        if args.len() != 3 {
            return None;
        }
        let output_scale = compact_scalar_mul(args[2], scale);
        if let Some(fused) = compact_multiply_offset_ad_expression(args[0], args[1], &output_scale)
        {
            return Some(fused);
        }
        return Some(format!(
            "AdValue::mul_scaled_output({}, {}, {})",
            args[0], args[1], output_scale
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_scaled_lhs") {
        if args.len() != 3 {
            return None;
        }
        let output_scale = compact_scalar_mul(args[1], scale);
        if let Some(fused) = compact_multiply_offset_ad_expression(args[0], args[2], &output_scale)
        {
            return Some(fused);
        }
        return Some(format!(
            "AdValue::mul_scaled_output({}, {}, {})",
            args[0], args[2], output_scale
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_scaled_rhs") {
        if args.len() != 3 {
            return None;
        }
        let output_scale = compact_scalar_mul(args[2], scale);
        if let Some(fused) = compact_multiply_offset_ad_expression(args[0], args[1], &output_scale)
        {
            return Some(fused);
        }
        return Some(format!(
            "AdValue::mul_scaled_output({}, {}, {})",
            args[0], args[1], output_scale
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_offset_lhs") {
        if args.len() != 3 {
            return None;
        }
        return Some(format!(
            "AdValue::mul_offset_lhs_scaled_output({}, {}, {}, {scale})",
            args[0], args[1], args[2]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_offset_rhs") {
        if args.len() != 3 {
            return None;
        }
        return Some(format!(
            "AdValue::mul_offset_rhs_scaled_output({}, {}, {}, {scale})",
            args[0], args[1], args[2]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_offset_lhs_scaled_output") {
        if args.len() != 4 {
            return None;
        }
        return Some(format!(
            "AdValue::mul_offset_lhs_scaled_output({}, {}, {}, {})",
            args[0],
            args[1],
            args[2],
            compact_scalar_mul(args[3], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_offset_rhs_scaled_output") {
        if args.len() != 4 {
            return None;
        }
        return Some(format!(
            "AdValue::mul_offset_rhs_scaled_output({}, {}, {}, {})",
            args[0],
            args[1],
            args[2],
            compact_scalar_mul(args[3], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_sub_from_scalar_lhs") {
        if args.len() != 3 {
            return None;
        }
        return Some(format!(
            "AdValue::mul_sub_from_scalar_lhs_scaled_output({}, {}, {}, {scale})",
            args[0], args[1], args[2]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_sub_from_scalar_rhs") {
        if args.len() != 3 {
            return None;
        }
        return Some(format!(
            "AdValue::mul_sub_from_scalar_rhs_scaled_output({}, {}, {}, {scale})",
            args[0], args[1], args[2]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_sub_from_scalar_lhs_scaled_output") {
        if args.len() != 4 {
            return None;
        }
        return Some(format!(
            "AdValue::mul_sub_from_scalar_lhs_scaled_output({}, {}, {}, {})",
            args[0],
            args[1],
            args[2],
            compact_scalar_mul(args[3], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_sub_from_scalar_rhs_scaled_output") {
        if args.len() != 4 {
            return None;
        }
        return Some(format!(
            "AdValue::mul_sub_from_scalar_rhs_scaled_output({}, {}, {}, {})",
            args[0],
            args[1],
            args[2],
            compact_scalar_mul(args[3], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul_sub_from_scalar_scaled_offset_self") {
        if args.len() != 5 {
            return None;
        }
        return Some(format!(
            "AdValue::mul_sub_from_scalar_scaled_offset_self({}, {}, {}, {}, {})",
            args[0],
            args[1],
            args[2],
            args[3],
            compact_scalar_mul(args[4], scale)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul3") {
        if args.len() != 3 {
            return None;
        }
        return Some(format!(
            "AdValue::mul3_scaled_output({}, {}, {}, {scale})",
            args[0], args[1], args[2]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "mul3_scaled_output") {
        if args.len() != 4 {
            return None;
        }
        return Some(format!(
            "AdValue::mul3_scaled_output({}, {}, {}, {})",
            args[0],
            args[1],
            args[2],
            compact_scalar_mul(args[3], scale)
        ));
    }

    let args = compact_ad_call_args(value, "neg")?;
    if args.len() != 1 {
        return None;
    }
    Some(format!(
        "AdValue::scale({}, {})",
        args[0],
        compact_scalar_negate(scale)
    ))
}

fn compact_offset_ad_value_expression(value: &str, offset: &str) -> Option<String> {
    if let Some(args) = compact_ad_call_args(value, "offset") {
        if args.len() != 2 {
            return None;
        }
        return Some(format!(
            "AdValue::offset({}, {})",
            args[0],
            compact_scalar_add_grouped(args[1], offset)
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "scale") {
        if args.len() != 2 {
            return None;
        }
        return Some(format!(
            "AdValue::scale_offset({}, {}, {offset})",
            args[0], args[1]
        ));
    }

    if let Some(args) = compact_ad_call_args(value, "scaled_offset") {
        if args.len() != 3 {
            return None;
        }
        return Some(format!(
            "AdValue::scale_offset({}, {}, {})",
            args[0],
            args[2],
            compact_scalar_add_grouped(&compact_scale_product(args[1], args[2]), offset)
        ));
    }

    let args = compact_ad_call_args(value, "scale_offset")?;
    if args.len() != 3 {
        return None;
    }
    Some(format!(
        "AdValue::scale_offset({}, {}, {})",
        args[0],
        args[1],
        compact_scalar_add_grouped(args[2], offset)
    ))
}

fn scratch_backed_variable(
    artifact: &CanonicalIrArtifact,
    variable_index: usize,
    branch_axis_count: usize,
    reactive: bool,
) -> LoweredVariable {
    let mut derivatives = Vec::with_capacity(artifact.mir.nodes.len());
    let mut branch_derivatives = Vec::with_capacity(branch_axis_count);
    let mut reactive_derivatives = Vec::with_capacity(artifact.mir.nodes.len());
    let mut reactive_branch_derivatives = Vec::with_capacity(branch_axis_count);
    for node_index in 0..artifact.mir.nodes.len() {
        derivatives.push(format!(
            "scratch.node_derivatives[{variable_index}][{node_index}]"
        ));
        if reactive {
            reactive_derivatives.push(format!(
                "scratch.reactive_node_derivatives[{variable_index}][{node_index}]"
            ));
        } else {
            reactive_derivatives.push("0.0".to_string());
        }
    }
    for branch_index in 0..branch_axis_count {
        branch_derivatives.push(format!(
            "scratch.branch_derivatives[{variable_index}][{branch_index}]"
        ));
        if reactive {
            reactive_branch_derivatives.push(format!(
                "scratch.reactive_branch_derivatives[{variable_index}][{branch_index}]"
            ));
        } else {
            reactive_branch_derivatives.push("0.0".to_string());
        }
    }

    LoweredVariable {
        value: format!("scratch.values[{variable_index}]"),
        condition: None,
        derivatives,
        branch_derivatives,
        has_reactive: false,
        reactive_value: if reactive {
            format!("scratch.reactive_values[{variable_index}]")
        } else {
            "0.0".to_string()
        },
        reactive_derivatives,
        reactive_branch_derivatives,
    }
}

fn zero_derivative_scratch_variable(
    artifact: &CanonicalIrArtifact,
    variable_index: usize,
    branch_axis_count: usize,
) -> LoweredVariable {
    LoweredVariable {
        value: format!("scratch.values[{variable_index}]"),
        condition: None,
        derivatives: zero_derivative_vec(artifact.mir.nodes.len()),
        branch_derivatives: zero_derivative_vec(branch_axis_count),
        has_reactive: false,
        reactive_value: "0.0".to_string(),
        reactive_derivatives: zero_derivative_vec(artifact.mir.nodes.len()),
        reactive_branch_derivatives: zero_derivative_vec(branch_axis_count),
    }
}

fn boolean_scratch_variable(
    artifact: &CanonicalIrArtifact,
    variable_index: usize,
    branch_axis_count: usize,
) -> LoweredVariable {
    let mut variable =
        zero_derivative_scratch_variable(artifact, variable_index, branch_axis_count);
    variable.condition = Some(format!("scratch.bool_values[{variable_index}]"));
    variable
}

struct CompactAdEmitter<'a> {
    artifact: &'a CanonicalIrArtifact,
    prefix: &'a str,
    parameter_fields: &'a HashMap<String, String>,
    variables: &'a HashMap<String, LoweredVariable>,
    ddt_slots: &'a DdtSlots,
    branch_current_unknowns: &'a HashMap<String, BranchCurrentSlot>,
    emitted: HashMap<ExprId, String>,
    lines: Vec<String>,
}

impl CompactAdEmitter<'_> {
    fn lower(&mut self, id: ExprId) -> Result<String, RustBackendError> {
        if let Some(value) = self.emitted.get(&id) {
            return Ok(value.clone());
        }
        let kind = self.expression(id)?.kind.clone();
        if let Some(value) = self.inline_leaf(&kind)? {
            return Ok(value);
        }
        let base = format!("{}_ad_e{}", self.prefix, id.index());
        let rhs = match &kind {
            HirExprKind::Number { value, .. } => {
                format!("AdValue::constant({})", format_f64(*value))
            }
            HirExprKind::Identifier { name } => self.lower_identifier(name.as_str())?,
            HirExprKind::BranchAccess { access, pos, neg } => {
                if access == "I" {
                    if let Some(slot) =
                        self.branch_current_slot_for_nodes(pos.as_str(), neg.as_deref())?
                    {
                        let value = format!(
                            "AdValue::branch_current(ctx, &self.branches, {})",
                            slot.slot
                        );
                        if slot.sign < 0.0 {
                            format!("AdValue::neg({value})")
                        } else {
                            value
                        }
                    } else {
                        return Err(
                            self.unsupported(format!("branch access '{access}' in expression"))
                        );
                    }
                } else {
                    format!(
                        "AdValue::voltage(ctx, &self.nodes, {}, {})",
                        compact_optional_node(self.node_index(pos.as_str())?),
                        compact_optional_node(
                            neg.as_deref()
                                .map(|node| self.node_index(node))
                                .transpose()?
                                .flatten()
                        )
                    )
                }
            }
            HirExprKind::NamedBranchAccess { access, name } => {
                self.lower_named_branch_access(access.as_str(), name.as_str())?
            }
            HirExprKind::Unary { op, operand } => {
                if op.as_str() == "Not" {
                    let condition = self.lower_condition(*operand)?;
                    format!(
                        "AdValue::constant(if {} {{ 1.0 }} else {{ 0.0 }})",
                        negate_condition(&condition)
                    )
                } else {
                    let operand = self.lower(*operand)?;
                    match op.as_str() {
                        "Neg" => format!("AdValue::neg({operand})"),
                        "Pos" => operand,
                        _ => return Err(self.unsupported(format!("unary operator {op}"))),
                    }
                }
            }
            HirExprKind::Binary { op, left, right } => {
                if let Some(operator) = comparison_operator(op.as_str()) {
                    let condition = self.comparison_condition(operator, *left, *right)?;
                    format!("AdValue::constant(if {condition} {{ 1.0 }} else {{ 0.0 }})")
                } else if op.as_str() == "And" || op.as_str() == "Or" {
                    let condition = self.lower_condition(id)?;
                    format!("AdValue::constant(if {condition} {{ 1.0 }} else {{ 0.0 }})")
                } else if let Some(scalar) = self.scalar_binary_value(op.as_str(), *left, *right)? {
                    format!("AdValue::constant({scalar})")
                } else {
                    match op.as_str() {
                        "Add" => {
                            if self.is_numeric_zero(*left)? {
                                self.lower(*right)?
                            } else if self.is_numeric_zero(*right)? {
                                self.lower(*left)?
                            } else if self.same_ad_operand(*left, *right)? {
                                self.lower_scaled(*left, 2.0)?
                            } else if let Some(operand) = self.negated_ad_operand(*left)? {
                                if let Some(scalar) = self.scalar_constant(*right)? {
                                    self.lower_sub_from_scalar(scalar, operand)?
                                } else {
                                    let right = self.lower(*right)?;
                                    let operand = self.lower(operand)?;
                                    format!("AdValue::sub({right}, {operand})")
                                }
                            } else if let Some(operand) = self.negated_ad_operand(*right)? {
                                if let Some(scalar) = self.scalar_constant(*left)? {
                                    self.lower_sub_from_scalar(scalar, operand)?
                                } else {
                                    let left = self.lower(*left)?;
                                    let operand = self.lower(operand)?;
                                    format!("AdValue::sub({left}, {operand})")
                                }
                            } else if let Some(offset) = self.scalar_constant(*left)? {
                                self.lower_offset(*right, offset)?
                            } else if let Some(offset) = self.scalar_constant(*right)? {
                                self.lower_offset(*left, offset)?
                            } else {
                                let left = self.lower(*left)?;
                                let right = self.lower(*right)?;
                                compact_add_sub_scaled_ad_expressions("add", &left, &right)
                                    .unwrap_or_else(|| format!("AdValue::add({left}, {right})"))
                            }
                        }
                        "Sub" => {
                            if self.is_numeric_zero(*right)? {
                                self.lower(*left)?
                            } else if self.is_numeric_zero(*left)? {
                                self.lower_scaled(*right, -1.0)?
                            } else if self.same_ad_operand(*left, *right)? {
                                "AdValue::constant(0.0)".to_string()
                            } else if let Some(offset) = self.scalar_constant(*right)? {
                                self.lower_offset(*left, compact_scalar_negate(&offset))?
                            } else if let Some(scalar) = self.scalar_constant(*left)? {
                                self.lower_sub_from_scalar(scalar, *right)?
                            } else {
                                let left = self.lower(*left)?;
                                let right = self.lower(*right)?;
                                compact_add_sub_scaled_ad_expressions("sub", &left, &right)
                                    .unwrap_or_else(|| format!("AdValue::sub({left}, {right})"))
                            }
                        }
                        "Mul" => {
                            if let Some(scale) = self.numeric_literal(*left)? {
                                self.lower_scaled(*right, scale)?
                            } else if let Some(scale) = self.numeric_literal(*right)? {
                                self.lower_scaled(*left, scale)?
                            } else if let Some(scale) = self.scalar_constant(*left)? {
                                self.lower_scaled_by_expr(*right, scale)?
                            } else if let Some(scale) = self.scalar_constant(*right)? {
                                self.lower_scaled_by_expr(*left, scale)?
                            } else if self.same_ad_operand(*left, *right)? {
                                let operand = self.lower(*left)?;
                                format!("AdValue::square({operand})")
                            } else if let Some((operand, scale)) = self.scaled_ad_operand(*left)? {
                                if let Some((right_operand, right_scale)) =
                                    self.scaled_ad_operand(*right)?
                                {
                                    let left = self.lower(operand)?;
                                    let right = self.lower(right_operand)?;
                                    let scale = compact_scalar_mul(&scale, &right_scale);
                                    compact_multiply_product3_ad_expressions(&left, &right, &scale)
                                        .or_else(|| {
                                            compact_multiply_sub_from_scalar_ad_expression(
                                                &left, &right, &scale,
                                            )
                                        })
                                        .unwrap_or_else(|| {
                                            format!(
                                                "AdValue::mul_scaled_lhs({left}, {scale}, {right})"
                                            )
                                        })
                                } else {
                                    let left = self.lower(operand)?;
                                    let right = self.lower(*right)?;
                                    compact_multiply_product3_ad_expressions(&left, &right, &scale)
                                        .or_else(|| {
                                            compact_multiply_sub_from_scalar_ad_expression(
                                                &left, &right, &scale,
                                            )
                                        })
                                        .unwrap_or_else(|| {
                                            format!(
                                                "AdValue::mul_scaled_lhs({left}, {scale}, {right})"
                                            )
                                        })
                                }
                            } else if let Some((operand, scale)) = self.scaled_ad_operand(*right)? {
                                let left = self.lower(*left)?;
                                let right = self.lower(operand)?;
                                compact_multiply_product3_ad_expressions(&left, &right, &scale)
                                    .or_else(|| {
                                        compact_multiply_sub_from_scalar_ad_expression(
                                            &left, &right, &scale,
                                        )
                                    })
                                    .unwrap_or_else(|| {
                                        format!("AdValue::mul_scaled_rhs({left}, {right}, {scale})")
                                    })
                            } else {
                                let left = self.lower(*left)?;
                                let right = self.lower(*right)?;
                                compact_multiply_scaled_ad_expressions(&left, &right)
                                    .unwrap_or_else(|| format!("AdValue::mul({left}, {right})"))
                            }
                        }
                        "Div" => {
                            if let Some(scale) = self.numeric_literal(*right)? {
                                self.lower_scaled(*left, 1.0 / scale)?
                            } else if let Some(scale) = self.scalar_constant(*right)? {
                                self.lower_scaled_by_expr(*left, format!("1.0 / ({scale})"))?
                            } else if let Some(scalar) = self.scalar_constant(*left)? {
                                self.lower_div_from_scalar(scalar, *right)?
                            } else {
                                let left = self.lower(*left)?;
                                let right = self.lower(*right)?;
                                compact_div_product_ad_expression(&left, &right)
                                    .or_else(|| compact_div_scaled_ad_expressions(&left, &right))
                                    .unwrap_or_else(|| format!("AdValue::div({left}, {right})"))
                            }
                        }
                        "Mod" => {
                            if let Some(modulus) = self.scalar_constant(*right)? {
                                let left = self.lower(*left)?;
                                format!("AdValue::rem_with_scalar({left}, {modulus})")
                            } else if let Some(scalar) = self.scalar_constant(*left)? {
                                let right = self.lower(*right)?;
                                format!("AdValue::rem_from_scalar({scalar}, {right})")
                            } else {
                                let left = self.lower(*left)?;
                                let right = self.lower(*right)?;
                                format!("AdValue::rem({left}, {right})")
                            }
                        }
                        "Pow" => {
                            if let Some(exponent) = self.scalar_constant(*right)? {
                                let left = self.lower(*left)?;
                                format!("AdValue::powf({left}, {exponent})")
                            } else if let Some(base) = self.scalar_constant(*left)? {
                                let right = self.lower(*right)?;
                                format!("AdValue::pow_from_scalar({base}, {right})")
                            } else {
                                let left = self.lower(*left)?;
                                let right = self.lower(*right)?;
                                format!("AdValue::pow({left}, {right})")
                            }
                        }
                        _ => return Err(self.unsupported(format!("binary operator {op}"))),
                    }
                }
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => self.lower_conditional(*condition, *then_expr, *else_expr)?,
            HirExprKind::SystemFunction { name, args } => {
                self.lower_system_function(name.as_str(), args.as_slice())?
            }
            HirExprKind::Call { name, args } if is_ddt_name(name.as_str()) => {
                self.lower_ddt(id, args.as_slice())?
            }
            HirExprKind::Call { name, args } if is_idt_name(name.as_str()) => {
                let (expr, ic) = compact_idt_operands(args.as_slice(), self)?;
                self.lower_idt(id, expr, ic)?
            }
            HirExprKind::Call { name, args } if is_analysis_name(name.as_str()) => {
                let condition = self.analysis_condition(args.as_slice())?;
                format!("AdValue::constant(if {condition} {{ 1.0 }} else {{ 0.0 }})")
            }
            HirExprKind::Call { name, args } if is_ddx_name(name.as_str()) => {
                let (expr, probe) = compact_ddx_operands(args.as_slice(), self)?;
                self.lower_ddx(expr, probe)?
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {
                "AdValue::constant(0.0)".to_string()
            }
            HirExprKind::Call { name, args } if expr_is_intrinsic_name(name.as_str()) => {
                self.lower_intrinsic(name.as_str(), args.as_slice())?
            }
            HirExprKind::NoiseSource { .. } => "AdValue::constant(0.0)".to_string(),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddt { expr, abstol },
            } => {
                if abstol.is_some() {
                    return Err(self.unsupported("ddt abstol argument"));
                }
                self.lower_ddt(id, &[*expr])?
            }
            HirExprKind::AnalogOperator {
                op:
                    HirAnalogOperator::Idt {
                        expr,
                        ic,
                        assert,
                        abstol,
                    },
            } => {
                if assert.is_some() || abstol.is_some() {
                    return Err(self.unsupported("idt assert/abstol argument"));
                }
                self.lower_idt(id, *expr, *ic)?
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddx { expr, probe },
            } => self.lower_ddx(*expr, *probe)?,
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { expr },
            } => format!("AdValue::limexp({})", self.lower(*expr)?),
            other => return Err(self.unsupported(format!("expression kind {other:?}"))),
        };
        if rhs.len() <= 512 && !compact_kind_has_side_effect(&kind) {
            return Ok(rhs);
        }
        self.lines.push(format!("let {base}: AdValue = {rhs};"));
        self.emitted.insert(id, base.clone());
        Ok(base)
    }

    fn lower_conditional(
        &mut self,
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    ) -> Result<String, RustBackendError> {
        let condition = self.lower_condition(condition)?;
        let then_branch = self.lower_isolated_branch(then_expr)?;
        let else_branch = self.lower_isolated_branch(else_expr)?;
        Ok(compact_lazy_conditional(
            &condition,
            &then_branch.lines,
            &then_branch.value,
            &else_branch.lines,
            &else_branch.value,
        ))
    }

    fn lower_isolated_branch(&self, expr: ExprId) -> Result<CompactBranch, RustBackendError> {
        let mut branch = CompactAdEmitter {
            artifact: self.artifact,
            prefix: self.prefix,
            parameter_fields: self.parameter_fields,
            variables: self.variables,
            ddt_slots: self.ddt_slots,
            branch_current_unknowns: self.branch_current_unknowns,
            emitted: self.emitted.clone(),
            lines: Vec::new(),
        };
        let value = branch.lower(expr)?;
        Ok(CompactBranch {
            lines: branch.lines,
            value,
        })
    }

    fn noop_conditional_assignment(
        &self,
        expr: ExprId,
        target_name: &str,
    ) -> Result<Option<(ExprId, ExprId, bool)>, RustBackendError> {
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = self.expression(expr)?.kind.clone()
        else {
            return Ok(None);
        };

        if self.is_identifier_named(else_expr, target_name)? {
            Ok(Some((condition, then_expr, false)))
        } else if self.is_identifier_named(then_expr, target_name)? {
            Ok(Some((condition, else_expr, true)))
        } else {
            Ok(None)
        }
    }

    fn mixed_scalar_conditional_assignment(
        &self,
        expr: ExprId,
    ) -> Result<Option<MixedScalarConditional>, RustBackendError> {
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = self.expression(expr)?.kind.clone()
        else {
            return Ok(None);
        };

        match (
            self.zero_derivative_value_expr(then_expr)?,
            self.zero_derivative_value_expr(else_expr)?,
        ) {
            (Some(scalar_value), None) => Ok(Some(MixedScalarConditional {
                condition,
                scalar_value,
                ad_expr: else_expr,
                scalar_when_condition_true: true,
            })),
            (None, Some(scalar_value)) => Ok(Some(MixedScalarConditional {
                condition,
                scalar_value,
                ad_expr: then_expr,
                scalar_when_condition_true: false,
            })),
            _ => Ok(None),
        }
    }

    fn is_identifier_named(&self, expr: ExprId, expected: &str) -> Result<bool, RustBackendError> {
        Ok(matches!(
            &self.expression(expr)?.kind,
            HirExprKind::Identifier { name } if name.as_str() == expected
        ))
    }

    fn ad_identifier_index(&self, expr: ExprId) -> Result<Option<usize>, RustBackendError> {
        let HirExprKind::Identifier { name } = &self.expression(expr)?.kind else {
            return Ok(None);
        };
        let Some(variable) = self.variables.get(name.as_str()) else {
            return Ok(None);
        };
        if lowered_variable_has_zero_derivatives(variable) {
            return Ok(None);
        }
        self.variable_index(name.as_str())
    }

    fn expression(
        &self,
        id: ExprId,
    ) -> Result<&crate::canonical_ir::HirExpression, RustBackendError> {
        self.artifact
            .mir
            .expressions
            .get(usize::from(id))
            .ok_or_else(|| self.internal(format!("expression {id} is outside MIR arena")))
    }

    fn lower_identifier(&self, name: &str) -> Result<String, RustBackendError> {
        if let Some(field) = self.parameter_fields.get(name) {
            Ok(format!("AdValue::constant(params.{field})"))
        } else if let Some(variable) = self.variables.get(name) {
            if lowered_variable_has_zero_derivatives(variable) {
                return Ok(format!("AdValue::constant({})", variable.value));
            }
            let index = self
                .artifact
                .hir
                .variables
                .iter()
                .find(|variable| variable.name.as_str() == name)
                .map(|variable| usize::from(variable.id))
                .ok_or_else(|| self.unsupported(format!("unknown variable '{name}'")))?;
            Ok(format!("scratch.ad_value({index})"))
        } else {
            Err(self.unsupported(format!(
                "identifier '{name}' is not a parameter or scalar variable"
            )))
        }
    }

    fn inline_leaf(&self, kind: &HirExprKind) -> Result<Option<String>, RustBackendError> {
        match kind {
            HirExprKind::Number { value, .. } => {
                Ok(Some(format!("AdValue::constant({})", format_f64(*value))))
            }
            HirExprKind::Identifier { name } => self.lower_identifier(name.as_str()).map(Some),
            _ => Ok(None),
        }
    }

    fn numeric_literal(&self, id: ExprId) -> Result<Option<f64>, RustBackendError> {
        Ok(match &self.expression(id)?.kind {
            HirExprKind::Number { value, .. } => Some(*value),
            _ => None,
        })
    }

    fn is_numeric_zero(&self, id: ExprId) -> Result<bool, RustBackendError> {
        Ok(self
            .numeric_literal(id)?
            .map(|value| value == 0.0)
            .unwrap_or(false))
    }

    fn is_numeric_one(&self, id: ExprId) -> Result<bool, RustBackendError> {
        Ok(self
            .numeric_literal(id)?
            .map(|value| value == 1.0)
            .unwrap_or(false))
    }

    fn scalar_constant(&self, id: ExprId) -> Result<Option<String>, RustBackendError> {
        let kind = self.expression(id)?.kind.clone();
        match kind {
            HirExprKind::Number { value, .. } => Ok(Some(format_f64(value))),
            HirExprKind::Identifier { name } => {
                if let Some(field) = self.parameter_fields.get(name.as_str()) {
                    Ok(Some(format!("params.{field}")))
                } else if let Some(variable) = self.variables.get(name.as_str()) {
                    Ok(lowered_variable_has_zero_derivatives(variable)
                        .then(|| variable.value.clone()))
                } else {
                    Ok(None)
                }
            }
            HirExprKind::Unary { op, operand } => {
                if op.as_str() == "Not" {
                    return self.zero_derivative_value_expr(id);
                }
                let Some(operand) = self.scalar_constant(operand)? else {
                    return Ok(None);
                };
                match op.as_str() {
                    "Pos" => Ok(Some(operand)),
                    "Neg" => Ok(Some(compact_scalar_negate(&operand))),
                    _ => Ok(None),
                }
            }
            HirExprKind::Binary { op, left, right } => {
                if comparison_operator(op.as_str()).is_some()
                    || op.as_str() == "And"
                    || op.as_str() == "Or"
                {
                    return self.zero_derivative_value_expr(id);
                }
                let Some(left) = self.scalar_constant(left)? else {
                    return Ok(None);
                };
                let Some(right) = self.scalar_constant(right)? else {
                    return Ok(None);
                };
                let value = match op.as_str() {
                    "Add" => compact_scalar_add(&left, &right),
                    "Sub" => compact_scalar_sub(&left, &right),
                    "Mul" => compact_scalar_mul(&left, &right),
                    "Div" => compact_scalar_div(&left, &right),
                    "Mod" => compact_scalar_mod(&left, &right),
                    "Pow" => compact_scalar_pow(&left, &right),
                    _ => return Ok(None),
                };
                Ok(Some(value))
            }
            HirExprKind::Conditional { .. }
            | HirExprKind::SystemFunction { .. }
            | HirExprKind::NoiseSource { .. } => self.zero_derivative_value_expr(id),
            HirExprKind::Call { name, .. }
                if expr_is_intrinsic_name(name.as_str()) || is_noise_name(name.as_str()) =>
            {
                self.zero_derivative_value_expr(id)
            }
            _ => Ok(None),
        }
    }

    fn scalar_binary_value(
        &self,
        op: &str,
        left: ExprId,
        right: ExprId,
    ) -> Result<Option<String>, RustBackendError> {
        let Some(left) = self.scalar_constant(left)? else {
            return Ok(None);
        };
        let Some(right) = self.scalar_constant(right)? else {
            return Ok(None);
        };
        let value = match op {
            "Add" => compact_scalar_add(&left, &right),
            "Sub" => compact_scalar_sub(&left, &right),
            "Mul" => compact_scalar_mul(&left, &right),
            "Div" => compact_scalar_div(&left, &right),
            "Mod" => compact_scalar_mod(&left, &right),
            "Pow" => compact_scalar_pow(&left, &right),
            _ => return Ok(None),
        };
        Ok(Some(value))
    }

    fn zero_derivative_value_expr(&self, id: ExprId) -> Result<Option<String>, RustBackendError> {
        let kind = self.expression(id)?.kind.clone();
        match kind {
            HirExprKind::Number { value, .. } => Ok(Some(format_f64(value))),
            HirExprKind::Identifier { name } => {
                if let Some(field) = self.parameter_fields.get(name.as_str()) {
                    Ok(Some(format!("params.{field}")))
                } else if let Some(variable) = self.variables.get(name.as_str()) {
                    Ok(lowered_variable_has_zero_derivatives(variable)
                        .then(|| variable.value.clone()))
                } else {
                    Ok(None)
                }
            }
            HirExprKind::Unary { op, operand } => match op.as_str() {
                "Pos" => self.zero_derivative_value_expr(operand),
                "Neg" => Ok(self
                    .zero_derivative_value_expr(operand)?
                    .map(|operand| compact_scalar_negate(&operand))),
                "Not" => Ok(self.condition_value_expr(operand)?.map(|condition| {
                    format!(
                        "if {} {{ 1.0 }} else {{ 0.0 }}",
                        negate_condition(&condition)
                    )
                })),
                _ => Ok(None),
            },
            HirExprKind::Binary { op, left, right } => {
                if comparison_operator(op.as_str()).is_some() {
                    return Ok(self
                        .condition_value_expr(id)?
                        .map(|condition| format!("if {condition} {{ 1.0 }} else {{ 0.0 }}")));
                }
                if op.as_str() == "And" || op.as_str() == "Or" {
                    return Ok(self
                        .condition_value_expr(id)?
                        .map(|condition| format!("if {condition} {{ 1.0 }} else {{ 0.0 }}")));
                }
                let Some(left) = self.zero_derivative_value_expr(left)? else {
                    return Ok(None);
                };
                let Some(right) = self.zero_derivative_value_expr(right)? else {
                    return Ok(None);
                };
                Ok(match op.as_str() {
                    "Add" => Some(compact_scalar_add(&left, &right)),
                    "Sub" => Some(compact_scalar_sub(&left, &right)),
                    "Mul" => Some(compact_scalar_mul(&left, &right)),
                    "Div" => Some(compact_scalar_div(&left, &right)),
                    "Mod" => Some(compact_scalar_mod(&left, &right)),
                    "Pow" => Some(compact_scalar_pow(&left, &right)),
                    _ => None,
                })
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let Some(condition) = self.condition_value_expr(condition)? else {
                    return Ok(None);
                };
                let Some(then_expr) = self.zero_derivative_value_expr(then_expr)? else {
                    return Ok(None);
                };
                let Some(else_expr) = self.zero_derivative_value_expr(else_expr)? else {
                    return Ok(None);
                };
                Ok(Some(format!(
                    "(if {condition} {{ {then_expr} }} else {{ {else_expr} }})"
                )))
            }
            HirExprKind::SystemFunction { name, args } => {
                self.zero_derivative_system_function_value_expr(name.as_str(), args.as_slice())
            }
            HirExprKind::Call { name, args } if is_analysis_name(name.as_str()) => {
                Ok(Some(format!(
                    "if {} {{ 1.0 }} else {{ 0.0 }}",
                    self.analysis_condition(args.as_slice())?
                )))
            }
            HirExprKind::Call { name, args } if expr_is_intrinsic_name(name.as_str()) => {
                self.zero_derivative_intrinsic_value_expr(name.as_str(), args.as_slice())
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {
                Ok(Some("0.0".to_string()))
            }
            HirExprKind::NoiseSource { .. } => Ok(Some("0.0".to_string())),
            _ => Ok(None),
        }
    }

    fn zero_derivative_system_function_value_expr(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<Option<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        Ok(match normalized.as_str() {
            "$temperature" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("ctx.temperature()".to_string())
            }
            "$abstime" | "$realtime" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("self.time".to_string())
            }
            "$mfactor" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("self.multiplicity".to_string())
            }
            "$vt" | "$thermal_vt" => match args {
                [] => Some("ctx.thermal_voltage()".to_string()),
                [temperature] => self
                    .zero_derivative_value_expr(*temperature)?
                    .map(|temperature| format!("({temperature} * THERMAL_VOLTAGE_PER_K)")),
                _ => {
                    return Err(self.unsupported(format!(
                        "{normalized} expects zero or one argument, found {}",
                        args.len()
                    )));
                }
            },
            "$simparam" => match args {
                [name] => Some(format_f64(self.simparam_default(*name)?)),
                [_, default] => self.zero_derivative_value_expr(*default)?,
                _ => {
                    return Err(self.unsupported(format!(
                        "$simparam expects one or two arguments, found {}",
                        args.len()
                    )));
                }
            },
            "$param_given" => {
                let index = self.param_given_index(args)?;
                Some(format!(
                    "if self.param_given[{index}] {{ 1.0 }} else {{ 0.0 }}"
                ))
            }
            "$port_connected" => {
                self.expect_system_arity(&normalized, args, 1)?;
                Some("1.0".to_string())
            }
            _ => None,
        })
    }

    fn zero_derivative_intrinsic_value_expr(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<Option<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        let arg = |index: usize, this: &Self| -> Result<Option<String>, RustBackendError> {
            let Some(id) = args.get(index).copied() else {
                return Err(this.unsupported(format!(
                    "intrinsic function '{normalized}' missing argument {index}"
                )));
            };
            this.zero_derivative_value_expr(id)
        };
        Ok(match normalized.as_str() {
            "abs" | "fabs" => {
                arg(0, self)?.map(|value| format!("{}.abs()", compact_f64_receiver(&value)))
            }
            "sqrt" => arg(0, self)?.map(|value| format!("{}.sqrt()", compact_f64_receiver(&value))),
            "exp" => arg(0, self)?.map(|value| format!("{}.exp()", compact_f64_receiver(&value))),
            "limexp" => arg(0, self)?.map(compact_scalar_limexp),
            "__rspice_limited_exp" => arg(0, self)?.map(compact_scalar_limited_exp),
            "ln" | "log" => {
                arg(0, self)?.map(|value| format!("{}.ln()", compact_f64_receiver(&value)))
            }
            "log10" => {
                arg(0, self)?.map(|value| format!("{}.log10()", compact_f64_receiver(&value)))
            }
            "sin" => arg(0, self)?.map(|value| format!("{}.sin()", compact_f64_receiver(&value))),
            "cos" => arg(0, self)?.map(|value| format!("{}.cos()", compact_f64_receiver(&value))),
            "tan" => arg(0, self)?.map(|value| format!("{}.tan()", compact_f64_receiver(&value))),
            "atan" => arg(0, self)?.map(|value| format!("{}.atan()", compact_f64_receiver(&value))),
            "sinh" => arg(0, self)?.map(|value| format!("{}.sinh()", compact_f64_receiver(&value))),
            "cosh" => arg(0, self)?.map(|value| format!("{}.cosh()", compact_f64_receiver(&value))),
            "tanh" => arg(0, self)?.map(|value| format!("{}.tanh()", compact_f64_receiver(&value))),
            "asinh" => {
                arg(0, self)?.map(|value| format!("{}.asinh()", compact_f64_receiver(&value)))
            }
            "acosh" => {
                arg(0, self)?.map(|value| format!("{}.acosh()", compact_f64_receiver(&value)))
            }
            "atanh" => {
                arg(0, self)?.map(|value| format!("{}.atanh()", compact_f64_receiver(&value)))
            }
            "floor" => {
                arg(0, self)?.map(|value| format!("{}.floor()", compact_f64_receiver(&value)))
            }
            "ceil" => arg(0, self)?.map(|value| format!("{}.ceil()", compact_f64_receiver(&value))),
            "pow" => match (arg(0, self)?, arg(1, self)?) {
                (Some(base), Some(exponent)) => {
                    Some(format!("{}.powf({exponent})", compact_f64_receiver(&base)))
                }
                _ => None,
            },
            "min" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!(
                    "{}.min({right})",
                    compact_f64_binary_receiver(&left)
                )),
                _ => None,
            },
            "max" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!(
                    "{}.max({right})",
                    compact_f64_binary_receiver(&left)
                )),
                _ => None,
            },
            "hypot" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!(
                    "{}.hypot({right})",
                    compact_f64_binary_receiver(&left)
                )),
                _ => None,
            },
            "atan2" => match (arg(0, self)?, arg(1, self)?) {
                (Some(y), Some(x)) => {
                    Some(format!("{}.atan2({x})", compact_f64_binary_receiver(&y)))
                }
                _ => None,
            },
            _ => None,
        })
    }

    fn same_ad_operand(&self, left: ExprId, right: ExprId) -> Result<bool, RustBackendError> {
        if left == right {
            return Ok(true);
        }
        let left = &self.expression(left)?.kind;
        let right = &self.expression(right)?.kind;
        Ok(match (left, right) {
            (HirExprKind::Identifier { name: left }, HirExprKind::Identifier { name: right }) => {
                left == right && self.variables.contains_key(left.as_str())
            }
            (
                HirExprKind::BranchAccess {
                    access: left_access,
                    pos: left_pos,
                    neg: left_neg,
                },
                HirExprKind::BranchAccess {
                    access: right_access,
                    pos: right_pos,
                    neg: right_neg,
                },
            ) => {
                left_access == right_access
                    && left_access != "I"
                    && left_pos == right_pos
                    && left_neg == right_neg
            }
            (
                HirExprKind::NamedBranchAccess {
                    access: left_access,
                    name: left_name,
                },
                HirExprKind::NamedBranchAccess {
                    access: right_access,
                    name: right_name,
                },
            ) => left_access == right_access && left_name == right_name,
            _ => false,
        })
    }

    fn negated_ad_operand(&self, id: ExprId) -> Result<Option<ExprId>, RustBackendError> {
        let HirExprKind::Unary { op, operand } = self.expression(id)?.kind.clone() else {
            return Ok(None);
        };
        if op.as_str() != "Neg" || !self.reorderable_ad_operand(operand)? {
            return Ok(None);
        }
        Ok(Some(operand))
    }

    fn reorderable_ad_operand(&self, id: ExprId) -> Result<bool, RustBackendError> {
        Ok(match &self.expression(id)?.kind {
            HirExprKind::Identifier { name } => self.variables.contains_key(name.as_str()),
            HirExprKind::BranchAccess { access, .. } => access != "I",
            HirExprKind::NamedBranchAccess { .. } => true,
            _ => false,
        })
    }

    fn comparison_condition(
        &mut self,
        operator: &str,
        left: ExprId,
        right: ExprId,
    ) -> Result<String, RustBackendError> {
        if let Some(condition) = self.boolean_numeric_comparison_condition(operator, left, right)? {
            return Ok(condition);
        }
        let left = self.comparison_operand_value(left)?;
        let right = self.comparison_operand_value(right)?;
        Ok(format!("({left} {operator} {right})"))
    }

    fn comparison_operand_value(&mut self, id: ExprId) -> Result<String, RustBackendError> {
        if let Some(value) = self.value_expr(id)? {
            Ok(value)
        } else {
            let value = self.lower(id)?;
            Ok(format!("{value}.value"))
        }
    }

    fn lower_scaled(&mut self, operand: ExprId, scale: f64) -> Result<String, RustBackendError> {
        if scale == 1.0 {
            self.lower(operand)
        } else if scale == -1.0 {
            let operand = self.lower(operand)?;
            Ok(format!("AdValue::neg({operand})"))
        } else {
            self.lower_scaled_by_expr(operand, format_f64(scale))
        }
    }

    fn lower_scaled_by_expr(
        &mut self,
        operand: ExprId,
        scale: String,
    ) -> Result<String, RustBackendError> {
        let kind = self.expression(operand)?.kind.clone();
        if let HirExprKind::Binary { op, left, right } = kind {
            match op.as_str() {
                "Mul" => {
                    if let Some(inner_scale) = self.scalar_constant(left)? {
                        return self.lower_scaled_by_expr(
                            right,
                            compact_scalar_mul(&inner_scale, scale.as_str()),
                        );
                    }
                    if let Some(inner_scale) = self.scalar_constant(right)? {
                        return self.lower_scaled_by_expr(
                            left,
                            compact_scalar_mul(&inner_scale, scale.as_str()),
                        );
                    }
                    if self.same_ad_operand(left, right)? {
                        let operand = self.lower(left)?;
                        return Ok(format!(
                            "AdValue::scale(AdValue::square({operand}), {scale})"
                        ));
                    }
                    let mut left_operand = left;
                    let mut right_operand = right;
                    let mut combined_scale = scale;
                    if let Some((operand, operand_scale)) = self.scaled_ad_operand(left)? {
                        left_operand = operand;
                        combined_scale = compact_scalar_mul(&operand_scale, &combined_scale);
                    }
                    if let Some((operand, operand_scale)) = self.scaled_ad_operand(right)? {
                        right_operand = operand;
                        combined_scale = compact_scalar_mul(&operand_scale, &combined_scale);
                    }
                    let left = self.lower(left_operand)?;
                    let right = self.lower(right_operand)?;
                    return Ok(compact_multiply_product3_ad_expressions(
                        &left,
                        &right,
                        &combined_scale,
                    )
                    .or_else(|| {
                        compact_multiply_sub_from_scalar_ad_expression(
                            &left,
                            &right,
                            &combined_scale,
                        )
                    })
                    .or_else(|| {
                        compact_multiply_offset_ad_expression(&left, &right, &combined_scale)
                    })
                    .unwrap_or_else(|| {
                        format!("AdValue::mul_scaled_output({left}, {right}, {combined_scale})")
                    }));
                }
                "Add" if self.same_ad_operand(left, right)? => {
                    return self.lower_scaled_by_expr(left, compact_scalar_mul("2.0", &scale));
                }
                "Sub" if self.same_ad_operand(left, right)? => {
                    return Ok("AdValue::constant(0.0)".to_string());
                }
                _ => {}
            }
        }
        let operand = self.lower(operand)?;
        Ok(compact_scale_ad_value_expression(&operand, &scale)
            .unwrap_or_else(|| format!("AdValue::scale({operand}, {scale})")))
    }

    fn lower_offset(
        &mut self,
        operand: ExprId,
        offset: String,
    ) -> Result<String, RustBackendError> {
        let operand = self.lower(operand)?;
        Ok(compact_offset_ad_value_expression(&operand, &offset)
            .unwrap_or_else(|| format!("AdValue::offset({operand}, {offset})")))
    }

    fn lower_sub_from_scalar(
        &mut self,
        scalar: String,
        operand: ExprId,
    ) -> Result<String, RustBackendError> {
        let operand = self.lower(operand)?;
        Ok(format!("AdValue::sub_from_scalar({scalar}, {operand})"))
    }

    fn lower_div_from_scalar(
        &mut self,
        scalar: String,
        operand: ExprId,
    ) -> Result<String, RustBackendError> {
        let operand = self.lower(operand)?;
        Ok(format!("AdValue::div_from_scalar({scalar}, {operand})"))
    }

    fn lower_named_branch_access(
        &self,
        access: &str,
        name: &str,
    ) -> Result<String, RustBackendError> {
        if access == "I" {
            if let Some(slot) = self.branch_current_unknowns.get(name) {
                let value = format!(
                    "AdValue::branch_current(ctx, &self.branches, {})",
                    slot.slot
                );
                return Ok(if slot.sign < 0.0 {
                    format!("AdValue::neg({value})")
                } else {
                    value
                });
            }
            return Err(self.unsupported(format!(
                "named branch current access '{name}' before a current contribution is available"
            )));
        }
        let branch = self
            .artifact
            .mir
            .branches
            .iter()
            .find(|branch| branch.name.as_str() == name)
            .ok_or_else(|| self.unsupported(format!("unknown named branch access '{name}'")))?;
        Ok(format!(
            "AdValue::voltage(ctx, &self.nodes, {}, {})",
            compact_optional_node(branch.pos_node.map(usize::from)),
            compact_optional_node(branch.neg_node.map(usize::from))
        ))
    }

    fn lower_condition(&mut self, id: ExprId) -> Result<String, RustBackendError> {
        if let Some(condition) = self.direct_boolean_condition_expr(id)? {
            return Ok(condition);
        }
        let kind = self.expression(id)?.kind.clone();
        match &kind {
            HirExprKind::Binary { op, left, right }
                if comparison_operator(op.as_str()).is_some() =>
            {
                let operator = comparison_operator(op.as_str()).expect("checked above");
                self.comparison_condition(operator, *left, *right)
            }
            HirExprKind::Binary { op, left, right } if op.as_str() == "And" => {
                let left = self.lower_condition(*left)?;
                let right = self.lower_condition(*right)?;
                Ok(format!("({left} && {right})"))
            }
            HirExprKind::Binary { op, left, right } if op.as_str() == "Or" => {
                let left = self.lower_condition(*left)?;
                let right = self.lower_condition(*right)?;
                Ok(format!("({left} || {right})"))
            }
            HirExprKind::Unary { op, operand } if op.as_str() == "Not" => {
                let operand = self.lower_condition(*operand)?;
                Ok(negate_condition(&operand))
            }
            _ => {
                if let Some(value) = self.value_expr(id)? {
                    Ok(format!("({value} != 0.0)"))
                } else {
                    let value = self.lower(id)?;
                    Ok(format!("({value}.value != 0.0)"))
                }
            }
        }
    }

    fn direct_boolean_condition_expr(
        &self,
        id: ExprId,
    ) -> Result<Option<String>, RustBackendError> {
        let kind = self.expression(id)?.kind.clone();
        Ok(match kind {
            HirExprKind::Identifier { name } => self
                .variables
                .get(name.as_str())
                .and_then(|variable| variable.condition.clone()),
            HirExprKind::SystemFunction { name, args }
                if name.eq_ignore_ascii_case("$param_given") =>
            {
                let index = self.param_given_index(args.as_slice())?;
                Some(format!("self.param_given[{index}]"))
            }
            HirExprKind::SystemFunction { name, args }
                if name.eq_ignore_ascii_case("$port_connected") =>
            {
                self.expect_system_arity("$port_connected", args.as_slice(), 1)?;
                Some("true".to_string())
            }
            HirExprKind::Call { name, args } if is_analysis_name(name.as_str()) => {
                Some(self.analysis_condition(args.as_slice())?)
            }
            _ => None,
        })
    }

    fn boolean_expr_condition(&self, id: ExprId) -> Result<Option<String>, RustBackendError> {
        if let Some(condition) = self.direct_boolean_condition_expr(id)? {
            return Ok(Some(condition));
        }
        let kind = self.expression(id)?.kind.clone();
        match kind {
            HirExprKind::Unary { op, .. } if op.as_str() == "Not" => self.condition_value_expr(id),
            HirExprKind::Binary { op, .. }
                if comparison_operator(op.as_str()).is_some()
                    || op.as_str() == "And"
                    || op.as_str() == "Or" =>
            {
                self.condition_value_expr(id)
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let Some(condition) = self.condition_value_expr(condition)? else {
                    return Ok(None);
                };
                let Some(then_condition) = self.boolean_branch_condition(then_expr)? else {
                    return Ok(None);
                };
                let Some(else_condition) = self.boolean_branch_condition(else_expr)? else {
                    return Ok(None);
                };
                if then_condition == else_condition {
                    Ok(Some(then_condition))
                } else if then_condition == "true" && else_condition == "false" {
                    Ok(Some(condition))
                } else if then_condition == "false" && else_condition == "true" {
                    Ok(Some(negate_condition(&condition)))
                } else {
                    Ok(Some(format!(
                        "(if {condition} {{ {then_condition} }} else {{ {else_condition} }})"
                    )))
                }
            }
            _ => Ok(None),
        }
    }

    fn boolean_branch_condition(&self, id: ExprId) -> Result<Option<String>, RustBackendError> {
        if let Some(value) = self.numeric_boolean_literal(id)? {
            return Ok(Some(rust_bool_literal(value)));
        }
        self.boolean_expr_condition(id)
    }

    fn boolean_numeric_comparison_condition(
        &self,
        operator: &str,
        left: ExprId,
        right: ExprId,
    ) -> Result<Option<String>, RustBackendError> {
        if let Some(condition) = self.direct_boolean_condition_expr(left)? {
            if let Some(expected) = self.numeric_boolean_literal(right)? {
                return Ok(boolean_numeric_condition(condition, operator, expected));
            }
        }
        if let Some(condition) = self.direct_boolean_condition_expr(right)? {
            if let Some(expected) = self.numeric_boolean_literal(left)? {
                return Ok(boolean_numeric_condition(condition, operator, expected));
            }
        }
        Ok(None)
    }

    fn numeric_boolean_literal(&self, id: ExprId) -> Result<Option<bool>, RustBackendError> {
        Ok(match self.numeric_literal(id)? {
            Some(value) if value == 0.0 => Some(false),
            Some(value) if value == 1.0 => Some(true),
            _ => None,
        })
    }

    fn lower_intrinsic(&mut self, name: &str, args: &[ExprId]) -> Result<String, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        if (normalized == "ln" || normalized == "log")
            && let Some(operand) = self.softplus_operand(args)?
        {
            return Ok(format!(
                "AdValue::ln_one_plus_exp({})",
                self.lower(operand)?
            ));
        }
        if let Some((operand, scale)) = self.scaled_intrinsic_operand(args)? {
            if let Some(helper) = match normalized.as_str() {
                "abs" | "fabs" => Some("abs_scaled_input"),
                "sqrt" => Some("sqrt_scaled_input"),
                "exp" => Some("exp_scaled_input"),
                "limexp" => Some("limexp_scaled_input"),
                "__rspice_limited_exp" => Some("limited_exp_scaled_input"),
                "ln" | "log" => Some("ln_scaled_input"),
                "tanh" => Some("tanh_scaled_input"),
                _ => None,
            } {
                return Ok(format!(
                    "AdValue::{helper}({}, {scale})",
                    self.lower(operand)?
                ));
            }
        }
        if normalized == "pow" {
            let base = args
                .first()
                .copied()
                .ok_or_else(|| self.unsupported("intrinsic function 'pow' missing argument 0"))?;
            let exponent = args
                .get(1)
                .copied()
                .ok_or_else(|| self.unsupported("intrinsic function 'pow' missing argument 1"))?;
            if let Some(exponent) = self.scalar_constant(exponent)? {
                return Ok(format!("AdValue::powf({}, {exponent})", self.lower(base)?));
            }
            if let Some(base) = self.scalar_constant(base)? {
                return Ok(format!(
                    "AdValue::pow_from_scalar({base}, {})",
                    self.lower(exponent)?
                ));
            }
            return Ok(format!(
                "AdValue::pow({}, {})",
                self.lower(base)?,
                self.lower(exponent)?
            ));
        }
        if normalized == "min" || normalized == "max" {
            let left = args.first().copied().ok_or_else(|| {
                self.unsupported(format!(
                    "intrinsic function '{normalized}' missing argument 0"
                ))
            })?;
            let right = args.get(1).copied().ok_or_else(|| {
                self.unsupported(format!(
                    "intrinsic function '{normalized}' missing argument 1"
                ))
            })?;
            if let Some(left_scalar) = self.scalar_constant(left)? {
                return Ok(format!(
                    "AdValue::{normalized}_from_scalar({left_scalar}, {})",
                    self.lower(right)?
                ));
            }
            if let Some(right_scalar) = self.scalar_constant(right)? {
                return Ok(format!(
                    "AdValue::{normalized}_with_scalar({}, {right_scalar})",
                    self.lower(left)?
                ));
            }
        }
        let mut lower_arg = |index: usize| -> Result<String, RustBackendError> {
            args.get(index)
                .copied()
                .ok_or_else(|| {
                    self.unsupported(format!(
                        "intrinsic function '{normalized}' missing argument {index}"
                    ))
                })
                .and_then(|arg| self.lower(arg))
        };
        Ok(match normalized.as_str() {
            "abs" | "fabs" => format!("AdValue::abs({})", lower_arg(0)?),
            "sqrt" => format!("AdValue::sqrt({})", lower_arg(0)?),
            "exp" => format!("AdValue::exp({})", lower_arg(0)?),
            "limexp" => format!("AdValue::limexp({})", lower_arg(0)?),
            "__rspice_limited_exp" => format!("AdValue::limited_exp({})", lower_arg(0)?),
            "ln" | "log" => format!("AdValue::ln({})", lower_arg(0)?),
            "log10" => format!("AdValue::log10({})", lower_arg(0)?),
            "sin" => format!("AdValue::sin({})", lower_arg(0)?),
            "cos" => format!("AdValue::cos({})", lower_arg(0)?),
            "tan" => format!("AdValue::tan({})", lower_arg(0)?),
            "atan" => format!("AdValue::atan({})", lower_arg(0)?),
            "sinh" => format!("AdValue::sinh({})", lower_arg(0)?),
            "cosh" => format!("AdValue::cosh({})", lower_arg(0)?),
            "tanh" => format!("AdValue::tanh({})", lower_arg(0)?),
            "asinh" => format!("AdValue::asinh({})", lower_arg(0)?),
            "acosh" => format!("AdValue::acosh({})", lower_arg(0)?),
            "atanh" => format!("AdValue::atanh({})", lower_arg(0)?),
            "floor" => format!("AdValue::floor({})", lower_arg(0)?),
            "ceil" => format!("AdValue::ceil({})", lower_arg(0)?),
            "min" => format!("AdValue::min({}, {})", lower_arg(0)?, lower_arg(1)?),
            "max" => format!("AdValue::max({}, {})", lower_arg(0)?, lower_arg(1)?),
            "hypot" => format!("AdValue::hypot({}, {})", lower_arg(0)?, lower_arg(1)?),
            "atan2" => format!("AdValue::atan2({}, {})", lower_arg(0)?, lower_arg(1)?),
            _ => return Err(self.unsupported(format!("intrinsic function '{name}'"))),
        })
    }

    fn scaled_intrinsic_operand(
        &self,
        args: &[ExprId],
    ) -> Result<Option<(ExprId, String)>, RustBackendError> {
        let [arg] = args else {
            return Ok(None);
        };
        self.scaled_ad_operand(*arg)
    }

    fn scaled_ad_operand(&self, id: ExprId) -> Result<Option<(ExprId, String)>, RustBackendError> {
        let kind = self.expression(id)?.kind.clone();
        match kind {
            HirExprKind::Unary { op, operand } if op.as_str() == "Neg" => Ok(Some(
                self.combine_scaled_ad_operand(operand, "-1.0".to_string())?,
            )),
            HirExprKind::Binary { op, left, right } if op.as_str() == "Mul" => {
                if let Some(scale) = self.scalar_constant(left)? {
                    Ok(Some(self.combine_scaled_ad_operand(right, scale)?))
                } else if let Some(scale) = self.scalar_constant(right)? {
                    Ok(Some(self.combine_scaled_ad_operand(left, scale)?))
                } else {
                    Ok(None)
                }
            }
            HirExprKind::Binary { op, left, right } if op.as_str() == "Div" => {
                if let Some(scale) = self.scalar_constant(right)? {
                    Ok(Some(self.combine_scaled_ad_operand(
                        left,
                        format!("1.0 / ({scale})"),
                    )?))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    fn combine_scaled_ad_operand(
        &self,
        operand: ExprId,
        scale: String,
    ) -> Result<(ExprId, String), RustBackendError> {
        if let Some((inner_operand, inner_scale)) = self.scaled_ad_operand(operand)? {
            Ok((
                inner_operand,
                compact_scalar_mul(&inner_scale, scale.as_str()),
            ))
        } else {
            Ok((operand, scale))
        }
    }

    fn softplus_operand(&self, args: &[ExprId]) -> Result<Option<ExprId>, RustBackendError> {
        let [arg] = args else {
            return Ok(None);
        };
        let HirExprKind::Binary { op, left, right } = self.expression(*arg)?.kind.clone() else {
            return Ok(None);
        };
        if op.as_str() != "Add" {
            return Ok(None);
        }
        if self.is_numeric_one(left)? {
            return self.exp_intrinsic_operand(right);
        }
        if self.is_numeric_one(right)? {
            return self.exp_intrinsic_operand(left);
        }
        Ok(None)
    }

    fn exp_intrinsic_operand(&self, id: ExprId) -> Result<Option<ExprId>, RustBackendError> {
        let HirExprKind::Call { name, args } = self.expression(id)?.kind.clone() else {
            return Ok(None);
        };
        if !name.eq_ignore_ascii_case("exp") {
            return Ok(None);
        }
        Ok(match args.as_slice() {
            [operand] => Some(*operand),
            _ => None,
        })
    }

    fn lower_system_function(
        &mut self,
        name: &str,
        args: &[ExprId],
    ) -> Result<String, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        Ok(match normalized.as_str() {
            "$temperature" => {
                self.expect_system_arity(&normalized, args, 0)?;
                "AdValue::constant(ctx.temperature())".to_string()
            }
            "$abstime" | "$realtime" => {
                self.expect_system_arity(&normalized, args, 0)?;
                "AdValue::constant(self.time)".to_string()
            }
            "$mfactor" => {
                self.expect_system_arity(&normalized, args, 0)?;
                "AdValue::constant(self.multiplicity)".to_string()
            }
            "$vt" | "$thermal_vt" => match args {
                [] => "AdValue::constant(ctx.thermal_voltage())".to_string(),
                [temperature] => {
                    let temperature = self.lower(*temperature)?;
                    format!("AdValue::scale({temperature}, THERMAL_VOLTAGE_PER_K)")
                }
                _ => {
                    return Err(self.unsupported(format!(
                        "{normalized} expects zero or one argument, found {}",
                        args.len()
                    )));
                }
            },
            "$simparam" => self.lower_simparam(args)?,
            "$param_given" => {
                let index = self.param_given_index(args)?;
                format!("AdValue::constant(if self.param_given[{index}] {{ 1.0 }} else {{ 0.0 }})")
            }
            "$port_connected" => {
                self.expect_system_arity(&normalized, args, 1)?;
                "AdValue::constant(1.0)".to_string()
            }
            _ => return Err(self.unsupported(format!("system function '{name}'"))),
        })
    }

    fn lower_simparam(&mut self, args: &[ExprId]) -> Result<String, RustBackendError> {
        match args {
            [name] => Ok(format!(
                "AdValue::constant({})",
                format_f64(self.simparam_default(*name)?)
            )),
            [_, default] => self.lower(*default),
            _ => Err(self.unsupported(format!(
                "$simparam expects one or two arguments, found {}",
                args.len()
            ))),
        }
    }

    fn lower_ddt(&mut self, id: ExprId, args: &[ExprId]) -> Result<String, RustBackendError> {
        let [operand] = args else {
            return Err(self.unsupported(format!("ddt expects one operand, found {}", args.len())));
        };
        let operand = self.lower(*operand)?;
        let slot = self.ddt_slots.slot_for(id).ok_or_else(|| {
            self.internal(format!("ddt expression {id} has no generated state slot"))
        })?;
        let derivative_scale = self.ddt_scale_expr();
        Ok(format!(
            "AdValue::ddt({operand}, {derivative_scale}, eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, {slot}, {operand}.value))"
        ))
    }

    fn lower_idt(
        &mut self,
        id: ExprId,
        expr: ExprId,
        ic: Option<ExprId>,
    ) -> Result<String, RustBackendError> {
        let operand = self.lower(expr)?;
        let ic = if let Some(ic) = ic {
            self.lower(ic)?
        } else {
            "AdValue::constant(0.0)".to_string()
        };
        let slot = self.ddt_slots.idt_slot_for(id).ok_or_else(|| {
            self.internal(format!("idt expression {id} has no generated state slot"))
        })?;
        let derivative_scale = self.idt_scale_expr();
        Ok(format!(
            "AdValue::idt({operand}, {derivative_scale}, eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, {slot}, {operand}.value, {ic}.value))"
        ))
    }

    fn ddt_scale_expr(&mut self) -> &'static str {
        "ddt_scale"
    }

    fn idt_scale_expr(&mut self) -> &'static str {
        "idt_scale"
    }

    fn lower_ddx(&mut self, expr: ExprId, probe: ExprId) -> Result<String, RustBackendError> {
        let expr = self.lower(expr)?;
        let (pos, neg) = self.ddx_probe_nodes(probe)?;
        Ok(format!(
            "AdValue::constant(AdValue::ddx_projection(&{expr}, {}, {}))",
            compact_optional_node(pos),
            compact_optional_node(neg)
        ))
    }

    fn ddx_probe_nodes(
        &self,
        probe: ExprId,
    ) -> Result<(Option<usize>, Option<usize>), RustBackendError> {
        let expression = self.expression(probe)?;
        match &expression.kind {
            HirExprKind::BranchAccess { access, pos, neg } if access.as_str() != "I" => Ok((
                self.node_index(pos.as_str())?,
                neg.as_deref()
                    .map(|node| self.node_index(node))
                    .transpose()?
                    .flatten(),
            )),
            HirExprKind::NamedBranchAccess { access, name } if access.as_str() != "I" => {
                let branch = self
                    .artifact
                    .mir
                    .branches
                    .iter()
                    .find(|branch| branch.name.as_str() == name)
                    .ok_or_else(|| {
                        self.unsupported(format!("unknown named ddx probe branch '{name}'"))
                    })?;
                Ok((
                    branch.pos_node.map(usize::from),
                    branch.neg_node.map(usize::from),
                ))
            }
            other => Err(self.unsupported(format!(
                "ddx probe must be a voltage access, found {other:?}"
            ))),
        }
    }

    fn node_index(&self, name: &str) -> Result<Option<usize>, RustBackendError> {
        if name == "0"
            || self
                .artifact
                .mir
                .ground_nodes
                .iter()
                .any(|ground| ground.as_str() == name)
        {
            return Ok(None);
        }
        self.artifact
            .mir
            .nodes
            .iter()
            .find(|node| node.name.as_str() == name)
            .map(|node| Some(usize::from(node.id)))
            .ok_or_else(|| self.unsupported(format!("unknown branch access node '{name}'")))
    }

    fn branch_current_slot_for_nodes(
        &self,
        pos: &str,
        neg: Option<&str>,
    ) -> Result<Option<BranchCurrentSlot>, RustBackendError> {
        let pos = self.node_index(pos)?;
        let neg = neg.map(|node| self.node_index(node)).transpose()?.flatten();
        Ok(self
            .branch_current_unknowns
            .get(&branch_pair_key(pos, neg))
            .copied())
    }

    fn simparam_default(&self, name: ExprId) -> Result<f64, RustBackendError> {
        let expression = self.expression(name)?;
        let HirExprKind::StringLiteral { value } = &expression.kind else {
            return Ok(0.0);
        };
        Ok(match value.as_str() {
            "gmin" => 1.0e-12,
            "tnom" => 300.15,
            "simulatorVersion" => 1.0,
            _ => 0.0,
        })
    }

    fn analysis_condition(&self, args: &[ExprId]) -> Result<String, RustBackendError> {
        let query = self.analysis_query(args)?;
        Ok(format!("ctx.analysis({query:?})"))
    }

    fn analysis_query(&self, args: &[ExprId]) -> Result<String, RustBackendError> {
        let [name] = args else {
            return Err(self.unsupported(format!(
                "analysis expects one argument, found {}",
                args.len()
            )));
        };
        let expression = self.expression(*name)?;
        let HirExprKind::StringLiteral { value } = &expression.kind else {
            return Err(self.unsupported("analysis expects a string literal argument"));
        };
        normalize_analysis_query(value)
            .ok_or_else(|| self.unsupported(format!("analysis() unknown analysis name '{value}'")))
    }

    fn param_given_index(&self, args: &[ExprId]) -> Result<usize, RustBackendError> {
        let [parameter] = args else {
            return Err(self.unsupported(format!(
                "$param_given expects one parameter argument, found {}",
                args.len()
            )));
        };
        let expression = self.expression(*parameter)?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return Err(self.unsupported("$param_given expects a parameter identifier"));
        };
        self.artifact
            .mir
            .parameters
            .iter()
            .position(|parameter| parameter.name.as_str() == name)
            .ok_or_else(|| self.unsupported(format!("unknown $param_given parameter '{name}'")))
    }

    fn value_expr(&self, id: ExprId) -> Result<Option<String>, RustBackendError> {
        let kind = self.expression(id)?.kind.clone();
        match kind {
            HirExprKind::Number { value, .. } => Ok(Some(format_f64(value))),
            HirExprKind::Identifier { name } => {
                if let Some(field) = self.parameter_fields.get(name.as_str()) {
                    Ok(Some(format!("params.{field}")))
                } else if let Some(index) = self.variable_index(name.as_str())? {
                    Ok(Some(format!("scratch.values[{index}]")))
                } else {
                    Ok(None)
                }
            }
            HirExprKind::BranchAccess { access, pos, neg } => {
                if access.as_str() == "I" {
                    Ok(self
                        .branch_current_slot_for_nodes(pos.as_str(), neg.as_deref())?
                        .map(|slot| {
                            slot.signed_value(format!(
                                "ctx.branch_current(self.branches[{}])",
                                slot.slot
                            ))
                        }))
                } else {
                    Ok(Some(
                        self.branch_voltage_value(pos.as_str(), neg.as_deref())?,
                    ))
                }
            }
            HirExprKind::NamedBranchAccess { access, name } => match access.as_str() {
                "I" => Ok(self.branch_current_unknowns.get(name.as_str()).map(|slot| {
                    slot.signed_value(format!("ctx.branch_current(self.branches[{}])", slot.slot))
                })),
                _ => {
                    let branch = self
                        .artifact
                        .mir
                        .branches
                        .iter()
                        .find(|branch| branch.name.as_str() == name)
                        .ok_or_else(|| {
                            self.unsupported(format!("unknown named branch access '{name}'"))
                        })?;
                    Ok(Some(self.branch_ref_voltage_value(branch)?))
                }
            },
            HirExprKind::Unary { op, operand } => match op.as_str() {
                "Pos" => self.value_expr(operand),
                "Neg" => Ok(self
                    .value_expr(operand)?
                    .map(|operand| compact_scalar_negate(&operand))),
                "Not" => Ok(self.condition_value_expr(operand)?.map(|condition| {
                    format!(
                        "if {} {{ 1.0 }} else {{ 0.0 }}",
                        negate_condition(&condition)
                    )
                })),
                _ => Ok(None),
            },
            HirExprKind::Binary { op, left, right } => {
                if comparison_operator(op.as_str()).is_some() {
                    return Ok(self
                        .condition_value_expr(id)?
                        .map(|condition| format!("if {condition} {{ 1.0 }} else {{ 0.0 }}")));
                }
                if op.as_str() == "And" || op.as_str() == "Or" {
                    return Ok(self
                        .condition_value_expr(id)?
                        .map(|condition| format!("if {condition} {{ 1.0 }} else {{ 0.0 }}")));
                }
                let Some(left) = self.value_expr(left)? else {
                    return Ok(None);
                };
                let Some(right) = self.value_expr(right)? else {
                    return Ok(None);
                };
                Ok(match op.as_str() {
                    "Add" => Some(compact_scalar_add(&left, &right)),
                    "Sub" => Some(compact_scalar_sub(&left, &right)),
                    "Mul" => Some(compact_scalar_mul(&left, &right)),
                    "Div" => Some(compact_scalar_div(&left, &right)),
                    "Mod" => Some(compact_scalar_mod(&left, &right)),
                    "Pow" => Some(compact_scalar_pow(&left, &right)),
                    _ => None,
                })
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let Some(condition) = self.condition_value_expr(condition)? else {
                    return Ok(None);
                };
                let Some(then_expr) = self.value_expr(then_expr)? else {
                    return Ok(None);
                };
                let Some(else_expr) = self.value_expr(else_expr)? else {
                    return Ok(None);
                };
                Ok(Some(format!(
                    "(if {condition} {{ {then_expr} }} else {{ {else_expr} }})"
                )))
            }
            HirExprKind::SystemFunction { name, args } => {
                self.system_function_value_expr(name.as_str(), args.as_slice())
            }
            HirExprKind::Call { name, args } if is_analysis_name(name.as_str()) => {
                Ok(Some(format!(
                    "if {} {{ 1.0 }} else {{ 0.0 }}",
                    self.analysis_condition(args.as_slice())?
                )))
            }
            HirExprKind::Call { name, args } if expr_is_intrinsic_name(name.as_str()) => {
                self.intrinsic_value_expr(name.as_str(), args.as_slice())
            }
            HirExprKind::Call { name, .. } if is_noise_name(name.as_str()) => {
                Ok(Some("0.0".to_string()))
            }
            HirExprKind::NoiseSource { .. } => Ok(Some("0.0".to_string())),
            _ => Ok(None),
        }
    }

    fn condition_value_expr(&self, id: ExprId) -> Result<Option<String>, RustBackendError> {
        if let Some(condition) = self.direct_boolean_condition_expr(id)? {
            return Ok(Some(condition));
        }
        let kind = self.expression(id)?.kind.clone();
        match &kind {
            HirExprKind::Binary { op, left, right }
                if comparison_operator(op.as_str()).is_some() =>
            {
                let operator = comparison_operator(op.as_str()).expect("checked above");
                if let Some(condition) =
                    self.boolean_numeric_comparison_condition(operator, *left, *right)?
                {
                    return Ok(Some(condition));
                }
                let Some(left) = self.value_expr(*left)? else {
                    return Ok(None);
                };
                let Some(right) = self.value_expr(*right)? else {
                    return Ok(None);
                };
                Ok(Some(format!("({left} {operator} {right})")))
            }
            HirExprKind::Binary { op, left, right } if op.as_str() == "And" => {
                let Some(left) = self.condition_value_expr(*left)? else {
                    return Ok(None);
                };
                let Some(right) = self.condition_value_expr(*right)? else {
                    return Ok(None);
                };
                Ok(Some(format!("({left} && {right})")))
            }
            HirExprKind::Binary { op, left, right } if op.as_str() == "Or" => {
                let Some(left) = self.condition_value_expr(*left)? else {
                    return Ok(None);
                };
                let Some(right) = self.condition_value_expr(*right)? else {
                    return Ok(None);
                };
                Ok(Some(format!("({left} || {right})")))
            }
            HirExprKind::Unary { op, operand } if op.as_str() == "Not" => {
                let Some(operand) = self.condition_value_expr(*operand)? else {
                    return Ok(None);
                };
                Ok(Some(negate_condition(&operand)))
            }
            _ => {
                let Some(value) = self.value_expr(id)? else {
                    return Ok(None);
                };
                Ok(Some(format!("({value} != 0.0)")))
            }
        }
    }

    fn variable_index(&self, name: &str) -> Result<Option<usize>, RustBackendError> {
        if !self.variables.contains_key(name) {
            return Ok(None);
        }
        self.artifact
            .hir
            .variables
            .iter()
            .find(|variable| variable.name.as_str() == name)
            .map(|variable| Some(usize::from(variable.id)))
            .ok_or_else(|| self.unsupported(format!("unknown variable '{name}'")))
    }

    fn branch_voltage_value(
        &self,
        pos: &str,
        neg: Option<&str>,
    ) -> Result<String, RustBackendError> {
        let pos = self.node_index(pos)?;
        let neg = neg.map(|node| self.node_index(node)).transpose()?.flatten();
        Ok(compact_voltage_value(pos, neg))
    }

    fn branch_ref_voltage_value(
        &self,
        branch: &crate::canonical_ir::MirBranch,
    ) -> Result<String, RustBackendError> {
        Ok(compact_voltage_value(
            branch.pos_node.map(usize::from),
            branch.neg_node.map(usize::from),
        ))
    }

    fn system_function_value_expr(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<Option<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        Ok(match normalized.as_str() {
            "$temperature" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("ctx.temperature()".to_string())
            }
            "$abstime" | "$realtime" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("self.time".to_string())
            }
            "$mfactor" => {
                self.expect_system_arity(&normalized, args, 0)?;
                Some("self.multiplicity".to_string())
            }
            "$vt" | "$thermal_vt" => match args {
                [] => Some("ctx.thermal_voltage()".to_string()),
                [temperature] => self
                    .value_expr(*temperature)?
                    .map(|temperature| format!("({temperature} * THERMAL_VOLTAGE_PER_K)")),
                _ => {
                    return Err(self.unsupported(format!(
                        "{normalized} expects zero or one argument, found {}",
                        args.len()
                    )));
                }
            },
            "$simparam" => match args {
                [name] => Some(format_f64(self.simparam_default(*name)?)),
                [_, default] => self.value_expr(*default)?,
                _ => {
                    return Err(self.unsupported(format!(
                        "$simparam expects one or two arguments, found {}",
                        args.len()
                    )));
                }
            },
            "$param_given" => {
                let index = self.param_given_index(args)?;
                Some(format!(
                    "if self.param_given[{index}] {{ 1.0 }} else {{ 0.0 }}"
                ))
            }
            "$port_connected" => {
                self.expect_system_arity(&normalized, args, 1)?;
                Some("1.0".to_string())
            }
            _ => None,
        })
    }

    fn intrinsic_value_expr(
        &self,
        name: &str,
        args: &[ExprId],
    ) -> Result<Option<String>, RustBackendError> {
        let normalized = name.to_ascii_lowercase();
        let arg = |index: usize, this: &Self| -> Result<Option<String>, RustBackendError> {
            let Some(id) = args.get(index).copied() else {
                return Err(this.unsupported(format!(
                    "intrinsic function '{normalized}' missing argument {index}"
                )));
            };
            this.value_expr(id)
        };
        Ok(match normalized.as_str() {
            "abs" | "fabs" => {
                arg(0, self)?.map(|value| format!("{}.abs()", compact_f64_receiver(&value)))
            }
            "sqrt" => arg(0, self)?.map(|value| format!("{}.sqrt()", compact_f64_receiver(&value))),
            "exp" => arg(0, self)?.map(|value| format!("{}.exp()", compact_f64_receiver(&value))),
            "limexp" => arg(0, self)?.map(compact_scalar_limexp),
            "ln" | "log" => {
                arg(0, self)?.map(|value| format!("{}.ln()", compact_f64_receiver(&value)))
            }
            "log10" => {
                arg(0, self)?.map(|value| format!("{}.log10()", compact_f64_receiver(&value)))
            }
            "sin" => arg(0, self)?.map(|value| format!("{}.sin()", compact_f64_receiver(&value))),
            "cos" => arg(0, self)?.map(|value| format!("{}.cos()", compact_f64_receiver(&value))),
            "tan" => arg(0, self)?.map(|value| format!("{}.tan()", compact_f64_receiver(&value))),
            "atan" => arg(0, self)?.map(|value| format!("{}.atan()", compact_f64_receiver(&value))),
            "sinh" => arg(0, self)?.map(|value| format!("{}.sinh()", compact_f64_receiver(&value))),
            "cosh" => arg(0, self)?.map(|value| format!("{}.cosh()", compact_f64_receiver(&value))),
            "tanh" => arg(0, self)?.map(|value| format!("{}.tanh()", compact_f64_receiver(&value))),
            "asinh" => {
                arg(0, self)?.map(|value| format!("{}.asinh()", compact_f64_receiver(&value)))
            }
            "acosh" => {
                arg(0, self)?.map(|value| format!("{}.acosh()", compact_f64_receiver(&value)))
            }
            "atanh" => {
                arg(0, self)?.map(|value| format!("{}.atanh()", compact_f64_receiver(&value)))
            }
            "floor" => {
                arg(0, self)?.map(|value| format!("{}.floor()", compact_f64_receiver(&value)))
            }
            "ceil" => arg(0, self)?.map(|value| format!("{}.ceil()", compact_f64_receiver(&value))),
            "pow" => match (arg(0, self)?, arg(1, self)?) {
                (Some(base), Some(exponent)) => {
                    Some(format!("{}.powf({exponent})", compact_f64_receiver(&base)))
                }
                _ => None,
            },
            "min" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!(
                    "{}.min({right})",
                    compact_f64_binary_receiver(&left)
                )),
                _ => None,
            },
            "max" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!(
                    "{}.max({right})",
                    compact_f64_binary_receiver(&left)
                )),
                _ => None,
            },
            "hypot" => match (arg(0, self)?, arg(1, self)?) {
                (Some(left), Some(right)) => Some(format!(
                    "{}.hypot({right})",
                    compact_f64_binary_receiver(&left)
                )),
                _ => None,
            },
            "atan2" => match (arg(0, self)?, arg(1, self)?) {
                (Some(y), Some(x)) => {
                    Some(format!("{}.atan2({x})", compact_f64_binary_receiver(&y)))
                }
                _ => None,
            },
            _ => None,
        })
    }

    fn expect_system_arity(
        &self,
        name: &str,
        args: &[ExprId],
        expected: usize,
    ) -> Result<(), RustBackendError> {
        if args.len() == expected {
            Ok(())
        } else {
            Err(self.unsupported(format!(
                "{name} expects {expected} argument(s), found {}",
                args.len()
            )))
        }
    }

    fn unsupported(&self, feature: impl Into<String>) -> RustBackendError {
        RustBackendError::unsupported(
            self.artifact.metadata.source_package.as_str(),
            self.artifact.mir.module_name.as_str(),
            feature,
        )
    }

    fn internal(&self, message: impl Into<String>) -> RustBackendError {
        RustBackendError::internal(
            self.artifact.metadata.source_package.as_str(),
            self.artifact.mir.module_name.as_str(),
            message,
        )
    }
}

fn compact_scalar_add(left: &str, right: &str) -> String {
    if left == "0.0" {
        right.to_string()
    } else if right == "0.0" {
        left.to_string()
    } else {
        format!("({left} + {right})")
    }
}

fn compact_scalar_add_grouped(left: &str, right: &str) -> String {
    if left == "0.0" {
        right.to_string()
    } else if right == "0.0" {
        left.to_string()
    } else {
        format!("(({left}) + ({right}))")
    }
}

fn compact_scalar_sub(left: &str, right: &str) -> String {
    if right == "0.0" {
        left.to_string()
    } else if left == "0.0" {
        compact_scalar_negate(right)
    } else {
        format!("({left} - {right})")
    }
}

fn compact_scalar_negate(value: &str) -> String {
    let value = value.trim();
    if value == "0.0" || value == "-0.0" {
        "0.0".to_string()
    } else if let Some(positive) = value.strip_prefix('-') {
        if scan_numeric_literal(positive).is_some() {
            positive.to_string()
        } else {
            format!("(-{value})")
        }
    } else {
        format!("(-{value})")
    }
}

fn compact_scalar_is_negative_one(value: &str) -> bool {
    let mut value = value.trim();
    while let Some(inner) = value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
    {
        value = inner.trim();
    }
    value == "-1.0" || value == "-1"
}

fn compact_scalar_same(left: &str, right: &str) -> bool {
    left.trim() == right.trim()
}

fn compact_ad_expression_same(left: &str, right: &str) -> bool {
    left.trim() == right.trim()
}

fn compact_scalar_mul(left: &str, right: &str) -> String {
    if left == "0.0" || right == "0.0" {
        "0.0".to_string()
    } else if left == "1.0" {
        right.to_string()
    } else if right == "1.0" {
        left.to_string()
    } else if left == "-1.0" {
        compact_scalar_negate(right)
    } else if right == "-1.0" {
        compact_scalar_negate(left)
    } else {
        format!("({left} * {right})")
    }
}

fn compact_scalar_div(left: &str, right: &str) -> String {
    if left == "0.0" {
        "0.0".to_string()
    } else if right == "1.0" {
        left.to_string()
    } else {
        format!("({left} / {right})")
    }
}

fn compact_scalar_mod(left: &str, right: &str) -> String {
    if left == "0.0" {
        "0.0".to_string()
    } else {
        format!("({left} % {right})")
    }
}

fn compact_scalar_pow(left: &str, right: &str) -> String {
    if right == "0.0" {
        "1.0".to_string()
    } else if right == "1.0" {
        left.to_string()
    } else {
        format!("{}.powf({right})", compact_f64_receiver(left))
    }
}

fn compact_scalar_limexp(value: String) -> String {
    format!(
        "{{ let limexp_arg = {value}; if limexp_arg < 80.0 {{ limexp_arg.exp() }} else {{ LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) }} }}"
    )
}

fn compact_scalar_limited_exp(value: String) -> String {
    format!(
        "{{ let limited_exp_arg = {value}; if limited_exp_arg > 80.0 {{ LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) }} else if limited_exp_arg < -80.0 {{ 1.804851387e-35 }} else {{ limited_exp_arg.exp() }} }}"
    )
}

fn compact_f64_receiver(value: &str) -> String {
    format!("(({value}) as f64)")
}

fn compact_f64_binary_receiver(value: &str) -> String {
    if let Some(typed) = typed_f64_literal(value) {
        format!("({typed})")
    } else {
        format!("({value})")
    }
}

fn typed_f64_literal(value: &str) -> Option<String> {
    let value = value.trim();
    if value.ends_with("_f64") || value.starts_with("f64::") {
        return None;
    }
    let saw_float_marker = scan_numeric_literal(value)?;
    if saw_float_marker {
        Some(format!("{value}_f64"))
    } else {
        None
    }
}

fn scan_numeric_literal(value: &str) -> Option<bool> {
    let unsigned = value.strip_prefix('-').unwrap_or(value);
    if unsigned.is_empty() || !unsigned.as_bytes()[0].is_ascii_digit() {
        return None;
    }

    let mut saw_digit = false;
    let mut saw_float_marker = false;
    let mut previous_was_exponent = false;
    for byte in unsigned.bytes() {
        match byte {
            b'0'..=b'9' => {
                saw_digit = true;
                previous_was_exponent = false;
            }
            b'.' => {
                saw_float_marker = true;
                previous_was_exponent = false;
            }
            b'e' | b'E' => {
                saw_float_marker = true;
                previous_was_exponent = true;
            }
            b'+' | b'-' if previous_was_exponent => {
                previous_was_exponent = false;
            }
            _ => return None,
        }
    }

    if saw_digit {
        Some(saw_float_marker)
    } else {
        None
    }
}

struct CompactBranch {
    lines: Vec<String>,
    value: String,
}

struct MixedScalarConditional {
    condition: ExprId,
    scalar_value: String,
    ad_expr: ExprId,
    scalar_when_condition_true: bool,
}

fn compact_lazy_conditional(
    condition: &str,
    then_lines: &[String],
    then_value: &str,
    else_lines: &[String],
    else_value: &str,
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str("    if ");
    out.push_str(condition);
    out.push_str(" {\n");
    push_compact_branch(&mut out, then_lines, then_value);
    out.push_str("    } else {\n");
    push_compact_branch(&mut out, else_lines, else_value);
    out.push_str("    }\n");
    out.push('}');
    out
}

fn boolean_numeric_condition(condition: String, operator: &str, expected: bool) -> Option<String> {
    match (operator, expected) {
        ("==", true) | ("!=", false) => Some(condition),
        ("==", false) | ("!=", true) => Some(format!("(!{condition})")),
        _ => None,
    }
}

fn rust_bool_literal(value: bool) -> String {
    if value {
        "true".to_string()
    } else {
        "false".to_string()
    }
}

fn negate_condition(condition: &str) -> String {
    let condition = condition.trim();
    if let Some(inner) = condition
        .strip_prefix("(!")
        .and_then(|inner| inner.strip_suffix(')'))
    {
        return inner.to_string();
    }
    if let Some(inner) = condition
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
    {
        if let Some(value) = inner.strip_suffix(" != 0.0") {
            return format!("({value} == 0.0)");
        }
        if let Some(value) = inner.strip_suffix(" == 0.0") {
            return format!("({value} != 0.0)");
        }
    }
    format!("(!{condition})")
}

fn push_compact_branch(out: &mut String, lines: &[String], value: &str) {
    for line in lines {
        push_indented_compact_line(out, "        ", line);
    }
    push_indented_compact_line(out, "        ", value);
}

fn push_indented_compact_line(out: &mut String, indent: &str, text: &str) {
    for line in text.lines() {
        out.push_str(indent);
        out.push_str(line);
        out.push('\n');
    }
}

fn compact_optional_node(node: Option<usize>) -> String {
    node.map(|node| format!("Some({node})"))
        .unwrap_or_else(|| "None".to_string())
}

fn compact_voltage_value(pos: Option<usize>, neg: Option<usize>) -> String {
    let pos = pos
        .map(|index| format!("ctx.node_voltage(self.nodes[{index}])"))
        .unwrap_or_else(|| "0.0".to_string());
    let neg = neg
        .map(|index| format!("ctx.node_voltage(self.nodes[{index}])"))
        .unwrap_or_else(|| "0.0".to_string());
    compact_scalar_sub(&pos, &neg)
}

fn compact_kind_has_side_effect(kind: &HirExprKind) -> bool {
    match kind {
        HirExprKind::Call { name, .. }
            if is_ddt_name(name.as_str()) || is_idt_name(name.as_str()) =>
        {
            true
        }
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Ddt { .. },
        } => true,
        HirExprKind::AnalogOperator {
            op: HirAnalogOperator::Idt { .. },
        } => true,
        _ => false,
    }
}

fn compact_idt_operands(
    args: &[ExprId],
    emitter: &CompactAdEmitter<'_>,
) -> Result<(ExprId, Option<ExprId>), RustBackendError> {
    match args {
        [expr] => Ok((*expr, None)),
        [expr, ic] => Ok((*expr, Some(*ic))),
        _ => Err(emitter.unsupported(format!(
            "idt expects one or two operands, found {}",
            args.len()
        ))),
    }
}

fn compact_ddx_operands(
    args: &[ExprId],
    emitter: &CompactAdEmitter<'_>,
) -> Result<(ExprId, ExprId), RustBackendError> {
    match args {
        [expr, probe] => Ok((*expr, *probe)),
        _ => Err(emitter.unsupported(format!("ddx expects two operands, found {}", args.len()))),
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_loop_statement(
    artifact: &CanonicalIrArtifact,
    loop_statement: &crate::canonical_ir::HirLoop,
    parameter_fields: &HashMap<String, String>,
    variable_fields: &HashMap<String, String>,
    variables: &mut HashMap<String, LoweredVariable>,
    out: &mut String,
    ddt_slots: &DdtSlots,
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
    transient_liveness: &TransientLiveness,
    reactive: bool,
    reactive_liveness: &ReactiveLiveness,
    indent: &str,
    prefix: &str,
) -> Result<(), RustBackendError> {
    if reactive && !reactive_liveness.loop_has_live_assignment(loop_statement) {
        return Ok(());
    }
    if !reactive && !transient_liveness.loop_has_live_value_assignment(loop_statement) {
        return Ok(());
    }

    let condition_prefix = format!("{prefix}_cond");
    let branch_currents = HashMap::new();
    let condition = lower_equation_expr_with_branch_currents(
        artifact,
        loop_statement.condition.id,
        &condition_prefix,
        parameter_fields,
        variables,
        ddt_slots,
        &branch_currents,
        branch_current_unknowns,
    )?;
    let guard = format!("{prefix}_loop_guard");
    out.push_str(&format!("{indent}let mut {guard}: usize = 0;\n"));
    out.push_str(&format!("{indent}while {{\n"));
    let inner_indent = format!("{indent}    ");
    for line in condition.lines {
        out.push_str(&inner_indent);
        out.push_str(&line);
        out.push('\n');
    }
    out.push_str(&format!("{inner_indent}{} != 0.0\n", condition.value));
    out.push_str(&format!("{indent}}} {{\n"));
    out.push_str(&format!("{inner_indent}{guard} += 1;\n"));
    out.push_str(&format!(
        "{inner_indent}assert!({guard} <= Self::MAX_ANALOG_LOOP_ITERATIONS, \"generated Verilog-A analog loop exceeded iteration guard\");\n"
    ));
    let body_prefix = format!("{prefix}_body");
    emit_statement_list(
        artifact,
        &loop_statement.body,
        parameter_fields,
        variable_fields,
        variables,
        out,
        ddt_slots,
        branch_current_unknowns,
        transient_liveness,
        reactive,
        reactive_liveness,
        &inner_indent,
        &body_prefix,
    )?;
    out.push_str(&format!("{indent}}}\n"));
    Ok(())
}

fn optional_node_expr(node: Option<crate::canonical_ir::NodeId>) -> String {
    node.map(|node| format!("Some(self.nodes[{}])", node.index()))
        .unwrap_or_else(|| "None".to_string())
}

fn optional_node_local_expr(node: Option<crate::canonical_ir::NodeId>) -> String {
    node.map(|node| format!("Some({})", node.index()))
        .unwrap_or_else(|| "None".to_string())
}

fn unsupported(artifact: &CanonicalIrArtifact, feature: impl Into<String>) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        feature,
    )
}

fn format_f64(value: f64) -> String {
    if value.is_nan() {
        "f64::NAN".to_string()
    } else if value == f64::INFINITY {
        "f64::INFINITY".to_string()
    } else if value == f64::NEG_INFINITY {
        "f64::NEG_INFINITY".to_string()
    } else {
        format!("{value:?}")
    }
}

fn is_zero_derivative(derivative: &str) -> bool {
    derivative.trim() == "0.0"
}

fn lowered_variable_has_zero_derivatives(variable: &LoweredVariable) -> bool {
    variable
        .derivatives
        .iter()
        .all(|value| is_zero_derivative(value))
        && variable
            .branch_derivatives
            .iter()
            .all(|value| is_zero_derivative(value))
        && (!variable.has_reactive
            || (is_zero_derivative(&variable.reactive_value)
                && variable
                    .reactive_derivatives
                    .iter()
                    .all(|value| is_zero_derivative(value))
                && variable
                    .reactive_branch_derivatives
                    .iter()
                    .all(|value| is_zero_derivative(value))))
}

fn is_inline_derivative_expr(derivative: &str) -> bool {
    let derivative = derivative.trim();
    is_zero_derivative(derivative)
        || derivative == "1.0"
        || derivative == "-1.0"
        || is_generated_scratch_derivative_access(derivative)
        || is_rust_identifier(derivative)
}

fn is_constant_derivative_expr(derivative: &str) -> bool {
    let derivative = derivative.trim();
    is_zero_derivative(derivative) || derivative == "1.0" || derivative == "-1.0"
}

fn is_generated_scratch_derivative_access(value: &str) -> bool {
    let value = value.trim();
    let has_known_prefix = value.starts_with("scratch.node_derivatives[")
        || value.starts_with("scratch.branch_derivatives[")
        || value.starts_with("scratch.reactive_node_derivatives[")
        || value.starts_with("scratch.reactive_branch_derivatives[");
    has_known_prefix
        && value.ends_with(']')
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '[' | ']'))
}

fn is_rust_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn zero_derivative_vec(count: usize) -> Vec<String> {
    (0..count).map(|_| "0.0".to_string()).collect()
}

fn branch_derivative_axis_count(
    branch_current_unknowns: &HashMap<String, BranchCurrentSlot>,
) -> usize {
    branch_current_unknowns
        .values()
        .map(|slot| slot.slot)
        .max()
        .map(|slot| slot + 1)
        .unwrap_or(0)
}
