//! Reaching-definition snapshots for the values equations read.
//!
//! # The defect this closes
//!
//! A Verilog-AMS analog block is a *sequence* of statements (LRM 1800.2-2023
//! section 5), and a contribution reads the values in effect where it is
//! written. The MIR route does not execute it that way: [`crate::ir::DeviceIR`]
//! splits the block into an assignment pass and a set of equations, the runtime
//! runs the *whole* pass and only then evaluates the equations, and an equation
//! that reads a variable reads whatever the pass left in the slot.
//!
//! That is the same value the author wrote only while nothing reassigns the
//! variable after the contribution. A scratch variable reused further down the
//! block breaks it, and breaks it silently — the stamp is a different number,
//! not an error. `ekv3_302.00` writes
//!
//! ```verilog
//! tmp = M * 2.0 / rdsb_t;
//! I(bi, bsi) <+ V(bi, bsi) * tmp;
//! ```
//!
//! in `ekv3_extrinsic_rc.va` and reuses `tmp` in three files included after it,
//! so the substrate conductance was stamped from the last of those writes.
//!
//! # The repair
//!
//! Before derivative shadows are built, every equation read of a variable that
//! a later statement reassigns is redirected to a fresh *snapshot* variable,
//! and a copy into that snapshot is spliced into the assignment sequence at the
//! reading equation's own program point. The equation then reads the reaching
//! definition on every route, because every route runs the same assignment
//! sequence.
//!
//! Three properties make this the cheapest correct representation:
//!
//! * A snapshot is an ordinary [`VarDef`], so the VM, the three JITs and the
//!   fused stamp kernels consume one bytecode with no per-backend case, and
//!   `NativeRequiredStorage`/`preallocate_vm_runtime_state` size it by counting
//!   variables exactly as they already do.
//! * The splice happens *before* `build_shadow_assignments`, so forward-mode AD
//!   differentiates the copy like any other assignment and the snapshot's
//!   derivative shadows are captured at the same point for free. Jacobian and
//!   noise-gain entries therefore reach the same definition the residual does,
//!   without this module knowing anything about derivatives.
//! * A model no equation reads a reassigned variable in is left untouched — no
//!   variable is allocated and no item is spliced — so its bytecode is
//!   byte-identical to the pre-repair compiler's.
//!
//! # Where the order comes from
//!
//! [`crate::semantic::AnalogSiteId`]s are minted from one monotonic counter
//! immediately before each analyzed statement and contribution is pushed, by
//! one ordered walk, and hierarchy elaboration rebases a child's onto the
//! flattened parent with the same base it appends the child's statements at.
//! Site order is therefore execution order, which is the only fact this module
//! needs. It is checked rather than assumed: [`insert_equation_snapshots`]
//! fails closed if the statement sites it is handed are not ascending.

use crate::error::{CodeGenError, CodeGenErrorKind, CompileResult};
use crate::ir::{
    ArrayDef, EquationSnapshotReads, IrAssignmentItem, IrExpr, ReachingSnapshotCopy,
    ReachingSnapshotPlan, VarAssignment, VarDef,
};
use crate::semantic::AnalogSiteId;
use smol_str::SmolStr;
use std::collections::HashMap;

/// The separator every snapshot name carries.
///
/// It is the one the compiler already builds derivative-shadow names with, so
/// a snapshot cannot collide with a variable the author declared — and a
/// snapshot slot is identifiable in [`crate::codegen::CompiledModel`]'s
/// `variable_names` by nothing more than reading it. That is what surveys the
/// estate: a model with no `@snap` slot has no equation read the assignment
/// pass would have overwritten, and one that has them names the variable and
/// the slot in each.
pub(crate) const SNAPSHOT_MARKER: &str = "@snap";

/// Redirect every equation read of a later-reassigned variable to a snapshot.
///
/// `assignments` and `statement_sites` are index-aligned over the module's
/// top-level statements; `equations` and `equation_sites` likewise over its
/// contributions. Both `assignments` and `equations` are rewritten in place.
/// Returns the plan a route that replays statements needs to place the copies
/// and resolve the redirected reads — empty for a module whose bytecode this
/// pass leaves byte-identical.
///
/// The plan's statement indices are into the sequence handed in here, which is
/// the sequence *before* the copies are spliced: they name the definition each
/// copy follows, so they stay meaningful for a route that never sees the
/// splice.
///
/// Called between contribution conversion and `build_shadow_assignments`; see
/// the module comment for why that window and no other.
pub(crate) fn insert_equation_snapshots(
    assignments: &mut Vec<IrAssignmentItem>,
    variables: &mut Vec<VarDef>,
    arrays: &[ArrayDef],
    statement_sites: &[AnalogSiteId],
    equations: &mut [IrExpr],
    equation_sites: &[AnalogSiteId],
) -> CompileResult<ReachingSnapshotPlan> {
    if assignments.len() != statement_sites.len() || equations.len() != equation_sites.len() {
        return Err(internal(format!(
            "analog site alignment lost: {} statements against {} sites, \
             {} equations against {} sites",
            assignments.len(),
            statement_sites.len(),
            equations.len(),
            equation_sites.len()
        )));
    }
    for pair in statement_sites.windows(2) {
        if pair[0] >= pair[1] {
            return Err(internal(format!(
                "analog statement sites are not in execution order: {} precedes {}",
                pair[0], pair[1]
            )));
        }
    }

    // Ascending write points per variable slot. `partition_point` over one of
    // these answers both questions this module asks: which definition reaches a
    // program point, and whether any definition follows it.
    let mut writes: HashMap<usize, Vec<usize>> = HashMap::new();
    for (index, item) in assignments.iter().enumerate() {
        record_writes(item, index, &mut writes);
    }
    if writes.is_empty() {
        return Ok(ReachingSnapshotPlan::default());
    }

    // Duplicate names resolve to the last slot, matching how the code
    // generator builds `variable_indices`; anything else would snapshot one
    // slot and rewrite a read of another.
    let mut slot_of_name: HashMap<SmolStr, usize> = HashMap::new();
    for (slot, variable) in variables.iter().enumerate() {
        slot_of_name.insert(variable.name.clone(), slot);
    }
    let array_runs: HashMap<SmolStr, (usize, usize)> = arrays
        .iter()
        .map(|array| (array.name.clone(), (array.base, array.len)))
        .collect();

    // Keyed by the definition being captured, not by the equation reading it:
    // contributions that read one variable between the same pair of writes
    // share a slot, so a model with many equations over one scratch variable
    // pays for the values it snapshots rather than the reads it makes.
    let mut snapshots: HashMap<(usize, Option<usize>), SmolStr> = HashMap::new();
    let mut splices: Vec<(usize, VarAssignment)> = Vec::new();
    let mut plan = ReachingSnapshotPlan::default();

    for (equation, expr) in equations.iter_mut().enumerate() {
        // Statements before the contribution are exactly those minted earlier,
        // its own side-effect statements among them.
        let site = equation_sites[equation];
        let point = statement_sites.partition_point(|statement| *statement < site);

        let mut reads = variable_reads(expr);
        reads.sort_unstable();
        reads.dedup();

        let mut renames: HashMap<SmolStr, SmolStr> = HashMap::new();
        for name in reads {
            // A runtime-indexed read addresses a whole contiguous run, so a
            // later write to any element of it reaches the read. Redirecting
            // one would mean copying the run and re-basing the read, which no
            // shipped model or estate fixture needs; the compiler refuses
            // rather than stamping the wrong element.
            if let Some(&(base, len)) = array_runs.get(&name) {
                if (base..base + len).any(|slot| reassigned_after(&writes, slot, point)) {
                    return Err(internal(format!(
                        "equation {equation} reads array '{name}' through a runtime index and a \
                         later statement reassigns an element of it; the reaching-definition \
                         snapshot does not cover runtime-indexed reads"
                    )));
                }
                continue;
            }
            let Some(&slot) = slot_of_name.get(&name) else {
                continue;
            };
            if !reassigned_after(&writes, slot, point) {
                continue;
            }

            let reaching = reaching_write(&writes, slot, point);
            let snapshot = snapshots
                .entry((slot, reaching))
                .or_insert_with(|| {
                    let snapshot: SmolStr =
                        format!("{name}{SNAPSHOT_MARKER}{}", variables.len()).into();
                    plan.copies.push(ReachingSnapshotCopy {
                        definition_statement: reaching,
                        slot: variables.len(),
                    });
                    variables.push(VarDef {
                        name: snapshot.clone(),
                        is_state: false,
                    });
                    // Immediately after the reaching definition: no write to
                    // the slot separates that point from the read, so it is
                    // the earliest point every equation sharing this
                    // definition can read.
                    splices.push((
                        reaching.map_or(0, |index| index + 1),
                        VarAssignment {
                            var_index: variables.len() - 1,
                            index: None,
                            expr: IrExpr::Var(name.clone()),
                        },
                    ));
                    snapshot
                })
                .clone();
            renames.insert(name, snapshot);
        }

        if !renames.is_empty() {
            let mut reads = renames
                .iter()
                .map(|(name, snapshot)| (name.clone(), snapshot.clone()))
                .collect::<Vec<_>>();
            // Ordered so the plan an equation carries is a function of the
            // module and not of a hash iteration.
            reads.sort_unstable();
            plan.reads.push(EquationSnapshotReads { equation, reads });
            *expr = rename_variable_reads(expr, &renames);
        }
    }

    if splices.is_empty() {
        return Ok(ReachingSnapshotPlan::default());
    }

    // Splice back to front so the untouched prefix keeps the indices the plan
    // was built against. Equal points retain the order they were planned in,
    // which is the order their reading equations appear.
    splices.sort_by_key(|(point, _)| *point);
    for (point, assignment) in splices.into_iter().rev() {
        assignments.insert(point, IrAssignmentItem::Assign(assignment));
    }
    Ok(plan)
}

/// Whether any statement at or after `point` writes `slot`.
fn reassigned_after(writes: &HashMap<usize, Vec<usize>>, slot: usize, point: usize) -> bool {
    writes
        .get(&slot)
        .and_then(|indices| indices.last())
        .is_some_and(|last| *last >= point)
}

/// The last statement before `point` that writes `slot`, if any.
fn reaching_write(writes: &HashMap<usize, Vec<usize>>, slot: usize, point: usize) -> Option<usize> {
    let indices = writes.get(&slot)?;
    let before = indices.partition_point(|index| *index < point);
    before.checked_sub(1).map(|last| indices[last])
}

/// Record every slot one top-level statement can write, loop bodies included.
///
/// A runtime-indexed write names an element only at runtime, so it counts as a
/// write to the whole declared run.
fn record_writes(item: &IrAssignmentItem, index: usize, out: &mut HashMap<usize, Vec<usize>>) {
    match item {
        IrAssignmentItem::Assign(assignment) => {
            let span = match &assignment.index {
                Some(target) => target.len,
                None => 1,
            };
            for slot in assignment.var_index..assignment.var_index + span {
                let indices = out.entry(slot).or_default();
                if indices.last() != Some(&index) {
                    indices.push(index);
                }
            }
        }
        IrAssignmentItem::Loop { body, .. } => {
            for nested in body {
                record_writes(nested, index, out);
            }
        }
    }
}

/// The operand programs an operator owns, which the generic walk stops at.
///
/// `map_expr` and `visit_expr` treat every event and noise node as a leaf,
/// because an operator's operands are compiled into programs of their own
/// rather than into the expression holding it. Those programs are evaluated
/// with the equation all the same — a noise magnitude is read at the operating
/// point the residual was stamped from — so they read the same definitions and
/// must be captured the same way. Nothing else the two walks stop at owns a
/// sub-expression: a Laplace or Zi coefficient list is numbers, a companion is
/// a slot ordinal.
fn operator_operands(expr: &IrExpr) -> Vec<&IrExpr> {
    fn optional<'a>(out: &mut Vec<&'a IrExpr>, operand: &'a Option<Box<IrExpr>>) {
        if let Some(operand) = operand {
            out.push(operand);
        }
    }

    let mut operands = Vec::new();
    match expr {
        IrExpr::WhiteNoise { power, .. } => operands.push(power.as_ref()),
        IrExpr::FlickerNoise {
            power, exponent, ..
        } => {
            operands.push(power);
            operands.push(exponent);
        }
        IrExpr::Cross {
            expr,
            direction,
            time_tol,
            expr_tol,
            enable,
        } => {
            operands.push(expr);
            optional(&mut operands, direction);
            optional(&mut operands, time_tol);
            optional(&mut operands, expr_tol);
            optional(&mut operands, enable);
        }
        IrExpr::Above {
            expr,
            time_tol,
            expr_tol,
            enable,
        } => {
            operands.push(expr);
            optional(&mut operands, time_tol);
            optional(&mut operands, expr_tol);
            optional(&mut operands, enable);
        }
        IrExpr::Timer {
            start_time,
            period,
            time_tol,
            enable,
        } => {
            operands.push(start_time);
            optional(&mut operands, period);
            optional(&mut operands, time_tol);
            optional(&mut operands, enable);
        }
        IrExpr::LastCrossing { expr, .. } => operands.push(expr),
        _ => {}
    }
    operands
}

/// Every variable an equation reads, operator operands included.
fn variable_reads(expr: &IrExpr) -> Vec<SmolStr> {
    /// One generic walk, queueing the operand programs it stops at.
    ///
    /// The operands are queued by value because `visit_expr` hands its
    /// callback a reference that may not outlive the call. They are an
    /// operator's arguments — a noise magnitude, a crossing tolerance — so the
    /// copy is bounded by what one operator was written with.
    fn scan(expr: &IrExpr, reads: &mut Vec<SmolStr>, queue: &mut Vec<IrExpr>) {
        crate::ir::autodiff::visit_expr(expr, &mut |node| {
            match node {
                IrExpr::Var(name) => reads.push(name.clone()),
                IrExpr::VarIndexed { array, .. } => reads.push(array.clone()),
                _ => {}
            }
            queue.extend(operator_operands(node).into_iter().cloned());
        });
    }

    let mut reads = Vec::new();
    let mut queue = Vec::new();
    scan(expr, &mut reads, &mut queue);
    while !queue.is_empty() {
        for operand in std::mem::take(&mut queue) {
            scan(&operand, &mut reads, &mut queue);
        }
    }
    reads
}

/// Replace the named variable reads with their snapshots, operator operands
/// included.
fn rename_variable_reads(expr: &IrExpr, renames: &HashMap<SmolStr, SmolStr>) -> IrExpr {
    fn rename_optional(
        operand: &Option<Box<IrExpr>>,
        renames: &HashMap<SmolStr, SmolStr>,
    ) -> Option<Box<IrExpr>> {
        operand
            .as_ref()
            .map(|operand| Box::new(rename_variable_reads(operand, renames)))
    }

    crate::ir::autodiff::map_expr(expr, &mut |node| match node {
        IrExpr::Var(name) => renames
            .get(name)
            .map(|snapshot| IrExpr::Var(snapshot.clone())),
        IrExpr::WhiteNoise { site, power, name } => Some(IrExpr::WhiteNoise {
            site: *site,
            power: Box::new(rename_variable_reads(power, renames)),
            name: name.clone(),
        }),
        IrExpr::FlickerNoise {
            site,
            power,
            exponent,
            name,
        } => Some(IrExpr::FlickerNoise {
            site: *site,
            power: Box::new(rename_variable_reads(power, renames)),
            exponent: Box::new(rename_variable_reads(exponent, renames)),
            name: name.clone(),
        }),
        IrExpr::Cross {
            expr,
            direction,
            time_tol,
            expr_tol,
            enable,
        } => Some(IrExpr::Cross {
            expr: Box::new(rename_variable_reads(expr, renames)),
            direction: rename_optional(direction, renames),
            time_tol: rename_optional(time_tol, renames),
            expr_tol: rename_optional(expr_tol, renames),
            enable: rename_optional(enable, renames),
        }),
        IrExpr::Above {
            expr,
            time_tol,
            expr_tol,
            enable,
        } => Some(IrExpr::Above {
            expr: Box::new(rename_variable_reads(expr, renames)),
            time_tol: rename_optional(time_tol, renames),
            expr_tol: rename_optional(expr_tol, renames),
            enable: rename_optional(enable, renames),
        }),
        IrExpr::Timer {
            start_time,
            period,
            time_tol,
            enable,
        } => Some(IrExpr::Timer {
            start_time: Box::new(rename_variable_reads(start_time, renames)),
            period: rename_optional(period, renames),
            time_tol: rename_optional(time_tol, renames),
            enable: rename_optional(enable, renames),
        }),
        IrExpr::LastCrossing { expr, direction } => Some(IrExpr::LastCrossing {
            expr: Box::new(rename_variable_reads(expr, renames)),
            direction: *direction,
        }),
        _ => None,
    })
}

fn internal(message: String) -> crate::error::CompileError {
    CodeGenError::new(CodeGenErrorKind::Internal(message)).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CompilerOptions, VerilogACompiler};

    /// The snapshot slots one module allocated, as the estate survey reads
    /// them: the compiled artifact names them, so nothing has to be exported
    /// for a census to find them.
    fn snapshot_slots(source: &str) -> Vec<String> {
        let model = VerilogACompiler::new(CompilerOptions::default())
            .compile(source)
            .expect("compilation failed");
        model
            .variable_names
            .iter()
            .filter(|name| name.contains(SNAPSHOT_MARKER))
            .map(|name| name.to_string())
            .collect()
    }

    /// The shape `ekv3_302.00` has: a scratch variable read by a contribution
    /// and written again below it.
    const REUSED_AFTER_READ: &str = r#"
`include "disciplines.vams"
module reuse(p, n);
    inout p, n;
    electrical p, n;
    real tmp;
    analog begin
        tmp = 2.0;
        I(p, n) <+ V(p, n) * tmp;
        tmp = 1000.0;
        I(p, n) <+ tmp * 0.0;
    end
endmodule
"#;

    /// The same module with the reuse spelled as a second variable, which is
    /// what the author would have written and what the repair compiles to
    /// exactly what it always did.
    const NO_REUSE: &str = r#"
`include "disciplines.vams"
module reuse(p, n);
    inout p, n;
    electrical p, n;
    real tmp;
    real other;
    analog begin
        tmp = 2.0;
        I(p, n) <+ V(p, n) * tmp;
        other = 1000.0;
        I(p, n) <+ other * 0.0;
    end
endmodule
"#;

    #[test]
    fn a_variable_written_below_the_equation_that_reads_it_is_snapshotted() {
        let slots = snapshot_slots(REUSED_AFTER_READ);
        assert_eq!(
            slots.len(),
            1,
            "one reaching definition to capture: {slots:?}"
        );
        assert!(
            slots[0].starts_with("tmp@snap"),
            "the snapshot names the variable it captures: {slots:?}"
        );
    }

    /// The property that keeps every unaffected model's bytecode — and so the
    /// machine-code identity census — exactly where it was: a module with no
    /// equation read of a reassigned variable allocates nothing.
    #[test]
    fn a_module_without_reuse_allocates_no_snapshot_slot() {
        assert!(snapshot_slots(NO_REUSE).is_empty());
    }

    /// Two contributions between the same pair of writes read one definition,
    /// so they share one slot rather than each minting its own.
    #[test]
    fn equations_sharing_a_reaching_definition_share_one_snapshot() {
        let slots = snapshot_slots(
            r#"
`include "disciplines.vams"
module reuse(p, n);
    inout p, n;
    electrical p, n;
    real tmp;
    analog begin
        tmp = 2.0;
        I(p, n) <+ V(p, n) * tmp;
        I(p, n) <+ V(p, n) * tmp * 2.0;
        tmp = 1000.0;
        I(p, n) <+ tmp * 0.0;
    end
endmodule
"#,
        );
        assert_eq!(slots.len(), 1, "{slots:?}");
    }

    /// Writes on either side of a contribution are two definitions, and the
    /// contribution below the second reads the second.
    #[test]
    fn a_second_definition_read_below_it_gets_its_own_snapshot() {
        let slots = snapshot_slots(
            r#"
`include "disciplines.vams"
module reuse(p, n);
    inout p, n;
    electrical p, n;
    real tmp;
    analog begin
        tmp = 2.0;
        I(p, n) <+ V(p, n) * tmp;
        tmp = 3.0;
        I(p, n) <+ V(p, n) * tmp;
        tmp = 1000.0;
    end
endmodule
"#,
        );
        assert_eq!(slots.len(), 2, "{slots:?}");
    }

    /// A write the author guarded is still a write: the analyzer folds the
    /// guard into `guard ? value : previous`, so the slot is assigned on every
    /// evaluation and the equation above it must not read the result.
    #[test]
    fn a_reassignment_inside_a_conditional_is_snapshotted() {
        let slots = snapshot_slots(
            r#"
`include "disciplines.vams"
module reuse(p, n);
    inout p, n;
    electrical p, n;
    real tmp;
    analog begin
        tmp = 2.0;
        I(p, n) <+ V(p, n) * tmp;
        if (V(p, n) > 0.5) begin
            tmp = 1000.0;
        end
        I(p, n) <+ tmp * 0.0;
    end
endmodule
"#,
        );
        assert_eq!(slots.len(), 1, "{slots:?}");
    }

    /// A write inside a runtime-bounded loop reaches the equations below the
    /// loop and none above it, and the loop is one statement in the sequence
    /// the snapshot is spliced into.
    #[test]
    fn a_reassignment_inside_a_runtime_loop_is_snapshotted() {
        let slots = snapshot_slots(
            r#"
`include "disciplines.vams"
module reuse(p, n);
    inout p, n;
    electrical p, n;
    parameter integer count = 3;
    real tmp;
    integer i;
    analog begin
        tmp = 2.0;
        I(p, n) <+ V(p, n) * tmp;
        i = 0;
        while (i < count) begin
            tmp = tmp + 1.0;
            i = i + 1;
        end
        I(p, n) <+ tmp * 0.0;
    end
endmodule
"#,
        );
        assert_eq!(slots.len(), 1, "{slots:?}");
    }

    /// Site order is the only fact this pass takes on trust, so it is checked.
    #[test]
    fn statement_sites_out_of_execution_order_fail_closed() {
        let mut assignments = vec![
            IrAssignmentItem::Assign(VarAssignment {
                var_index: 0,
                index: None,
                expr: IrExpr::Const(1.0),
            }),
            IrAssignmentItem::Assign(VarAssignment {
                var_index: 0,
                index: None,
                expr: IrExpr::Const(2.0),
            }),
        ];
        let mut variables = vec![VarDef {
            name: "tmp".into(),
            is_state: false,
        }];
        let mut equations = vec![IrExpr::Var("tmp".into())];
        let error = insert_equation_snapshots(
            &mut assignments,
            &mut variables,
            &[],
            &[AnalogSiteId(4), AnalogSiteId(1)],
            &mut equations,
            &[AnalogSiteId(2)],
        )
        .expect_err("descending statement sites are refused");
        assert!(error.to_string().contains("execution order"), "{error}");
    }
}

/// The estate survey behind the repair: which shipped modules read a variable
/// a statement below the reading equation reassigns, and which variable in
/// each.
///
/// `#[ignore]`d for the reason every other census is — it front-end compiles
/// the shipped corpus — and narrowed by `RSPICE_CFG_CENSUS_FILTER` the same
/// way, so one module can be surveyed without paying for forty-two.
///
/// Gated to x86-64 like every other census, because that is the gate on the
/// provider they share: `native::census_models` is
/// `all(test, feature = "native", target_arch = "x86_64")`, and a survey that
/// asked for it on AArch64 did not merely skip — it failed to resolve, and one
/// unresolved import takes the whole test binary out. Nothing here is
/// arch-specific; the corpus compile these censuses run is a front-end pass.
#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod survey {
    use super::SNAPSHOT_MARKER;
    use crate::native::census_models::shipped_census_models_matching;

    #[test]
    #[ignore]
    fn shipped_models_reusing_a_variable_an_equation_reads() {
        let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
        let mut surveyed = 0_usize;
        let mut affected = 0_usize;
        for shipped in shipped_census_models_matching(filter.as_deref()) {
            surveyed += 1;
            let slots = shipped
                .model
                .variable_names
                .iter()
                .map(|name| name.as_str())
                .filter(|name| name.contains(SNAPSHOT_MARKER))
                .collect::<Vec<&str>>();
            if slots.is_empty() {
                println!("reaching-survey model={} snapshots=0", shipped.name);
                continue;
            }
            affected += 1;
            println!(
                "reaching-survey model={} snapshots={} slots={}",
                shipped.name,
                slots.len(),
                slots.join(",")
            );
        }
        println!("reaching-survey surveyed={surveyed} affected={affected}");
        assert!(surveyed > 0, "the survey compiled nothing");
    }
}
