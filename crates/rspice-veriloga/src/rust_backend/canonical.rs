//! Generating a device from the canonical CFG, rather than from a tier.
//!
//! This is the emitter the rebuild exists to produce, wired to the device
//! contract the tiers already satisfy: `state.rs` holds the parameters and the
//! per-instance state, `stamp.rs` evaluates the body and writes the matrix, and
//! `noise.rs` is unchanged. Only the middle file changes, and it is now the
//! output of [`super::emit`] over a differentiated, simplified, scheduled CFG.
//!
//! ## What the tiers did that this does not
//!
//! *Scalarised derivatives.* A tier gives every lane its own value, so a wide
//! MOSFET carries a hundred thousand of them. Here a derivative is one packed
//! value over its own live lane set.
//!
//! *Flattened guards.* A tier turns `if` into arithmetic over both arms. Here
//! the control flow survives into the generated Rust, so the code skips the work
//! the model said to skip.
//!
//! *Zeros.* 202 of 931 stamp arguments in the tier output are literal
//! `multiplicity * 0.0`. [`super::stamp_plan`] decides which entries exist at
//! all, and the ones that do not are simply absent.
//!
//! ## Stages are functions, and that is why outputs need slots
//!
//! [`crate::canonical_ir::schedule::split`] cuts the body by how often each
//! value goes stale. Every class coarser than Newton becomes its own `fn` on
//! `Instance` that writes what later readers need into a slot array; the Newton
//! body runs in `stamp` and reads those slots. The instance and temperature
//! stages are guarded by validity flags, which is where the caching is. The
//! timestep stage runs on every call — nothing in the device contract tells
//! `stamp` that a new timestep began, and recomputing is correct, merely
//! uncached.
//!
//! Splitting at all is decided per model by
//! [`crate::canonical_ir::schedule::worth_splitting`], because a body that is
//! 97% Newton pays for the staged loads and gets nothing back. Most compact
//! models decline it.
//!
//! ## Charge storage
//!
//! A reactive stamp writes `d(charge)/d(unknown)`, not the residual's Jacobian,
//! so it needs the `ddt` operand rather than the `ddt` result. A contribution is
//! reactive when its residual *is* a `ddt` — which is how MIR presents it, one
//! equation per `<+` statement — and the charge is that operator's input,
//! differentiated by the same pass in the same body.
//!
//! ## What it refuses
//!
//! `$limit`, indirect contributions, and an unresolved flow probe. Each is a
//! piece the canonical level has not finished, and a device that quietly
//! computed something else would be worse than one that is not generated: the
//! caller falls back to a tier, which is what the tiers are still there for.

use std::collections::HashMap;
use std::fmt::Write as _;

use crate::canonical_ir::cfg::{CfgBinaryOp, CfgFunction, CfgValueKind};
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::schedule::{InvalidationClass, Stage, schedule, split, worth_splitting};
use crate::canonical_ir::{
    AdSeed, CanonicalIrArtifact, ExprId, MirEquationKind, ValueId, differentiate, optimize_cfg,
};

use super::emit::{EmitBindings, RUNTIME_PRELUDE, emit_body};
use super::expr::parameter_field_names;
use super::stamp_plan::{StampPlan, StampRow};
use super::{GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames};
use super::{RustTranspileOptions, device};

pub fn generate_device(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
) -> Result<GeneratedRustDevice, RustBackendError> {
    let plan = ModelPlan::build(artifact)?;

    let names = RustDeviceNames::new(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        artifact.metadata.source_digest.as_str(),
    );
    let parameter_fields = parameter_field_names(artifact);

    let stamp = plan.stamp_file(artifact, options)?;
    let state = device::generate_state_file_with_extensions(
        artifact,
        options,
        &parameter_fields,
        plan.ddt_slots.len(),
        0,
        artifact.mir.branch_unknowns.len(),
        &plan.state_extensions(),
    )?;

    let files = vec![
        GeneratedRustFile {
            relative_path: "mod.rs".to_string(),
            contents: device::generate_mod_file(),
        },
        GeneratedRustFile {
            relative_path: "state.rs".to_string(),
            contents: state,
        },
        GeneratedRustFile {
            relative_path: "stamp.rs".to_string(),
            contents: stamp,
        },
        super::noise::generate_noise_file(artifact, options)?,
    ];

    Ok(GeneratedRustDevice {
        module_name: artifact.mir.module_name.to_string(),
        public_model_name: names.public_model_name,
        folder_name: names.folder,
        source_digest: artifact.metadata.source_digest.to_string(),
        files,
    })
}

/// One body's worth of pipeline output: the function to emit, the rows to write
/// from it, and where each row's values sit in the emitted output list.
struct Body {
    function: CfgFunction,
    rows: Vec<StampRow>,
    /// Parallel to `rows`: `(residual, one per surviving derivative)`, as
    /// indices into `outputs`.
    positions: Vec<(usize, Vec<usize>)>,
    outputs: Vec<ValueId>,
}

struct ModelPlan {
    conduction: Body,
    /// Absent when no contribution stores charge, so there is nothing for the
    /// reactive matrix and `stamp_reactive` has no work to do.
    reactive: Option<Body>,
    /// The conduction body cut by invalidation class, or empty when the split
    /// was measured not to be worth taking for this model.
    stages: Vec<Stage>,
    slots: usize,
    node_count: usize,
    /// Branch-unknown ordinal per equation, for the potential stamps.
    branch_of_equation: Vec<Option<usize>>,
    /// One history slot per `ddt` in the body, allocated from the CFG.
    ///
    /// Not from `device::collect_ddt_slots`, and the reason is worth stating.
    /// That walks `mir.equations` and `hir.statements`; the CFG is lowered from
    /// `hir.body`, the structured region tree. The front end builds those from
    /// *separate copies* of the same expression tree — a two-terminal capacitor
    /// arena holds `ddt` twice, at ids 4 and 8 — so an operator id the CFG
    /// carries is not one that walk ever saw, and every lookup missed. The CFG
    /// is what this backend emits, so it is also what decides how many slots
    /// there are and which is which.
    ddt_slots: HashMap<ExprId, usize>,
}

impl ModelPlan {
    fn build(artifact: &CanonicalIrArtifact) -> Result<Self, RustBackendError> {
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).map_err(|diagnostics| {
            let mut reasons: Vec<String> = diagnostics
                .iter()
                .map(|diagnostic| diagnostic.message.to_string())
                .collect();
            reasons.sort();
            reasons.dedup();
            reasons.truncate(4);
            unsupported(
                artifact,
                format!("the body does not lower to a CFG: {}", reasons.join("; ")),
            )
        })?;
        reject_unsupported_kinds(artifact, &cfg.function)?;

        // In value order, which is the lowering's order, so the numbering is a
        // property of the model rather than of a hash map's iteration.
        let mut ddt_slots: HashMap<ExprId, usize> = HashMap::new();
        for value in &cfg.function.values {
            if let CfgValueKind::Ddt { operator, .. } = &value.kind {
                let next = ddt_slots.len();
                ddt_slots.entry(*operator).or_insert(next);
            }
        }

        let seeds: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .chain(
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| AdSeed::BranchUnknownFlow(index.into())),
            )
            .collect();
        let mut differentiated = differentiate(&cfg.function, &seeds)
            .map_err(|error| unsupported(artifact, format!("differentiation: {error}")))?;

        // Every read-out first, and both bodies' worth of them: taking a lane
        // appends an instruction, so a row taken after a simplification would
        // name values the simplified function does not have.
        let residuals: Vec<ValueId> = artifact
            .mir
            .equations
            .iter()
            .map(|equation| cfg.residuals[usize::from(equation.contribution)])
            .collect();
        let conduction_rows: Vec<Vec<Option<ValueId>>> = residuals
            .iter()
            .map(|residual| differentiated.derivative_row(*residual))
            .collect();
        let charges: Vec<Option<ValueId>> = residuals
            .iter()
            .map(|residual| stored_charge(&differentiated.function, *residual))
            .collect();
        let reactive_rows: Vec<Vec<Option<ValueId>>> = charges
            .iter()
            .map(|charge| match charge {
                Some(charge) => differentiated.derivative_row(*charge),
                None => Vec::new(),
            })
            .collect();

        let conduction = Body::build(
            &differentiated.function,
            artifact,
            &residuals,
            &conduction_rows,
        );
        let reactive = if charges.iter().any(Option::is_some) {
            // A contribution with no `ddt` stores no charge. Its row stays in
            // place so the two bodies remain parallel to `mir.equations`, with
            // nothing in it to write.
            let values: Vec<ValueId> = charges
                .iter()
                .zip(&residuals)
                .map(|(charge, residual)| charge.unwrap_or(*residual))
                .collect();
            let mut body = Body::build(&differentiated.function, artifact, &values, &reactive_rows);
            for (index, charge) in charges.iter().enumerate() {
                if charge.is_none() {
                    body.rows[index].derivatives.clear();
                    body.positions[index].1.clear();
                }
            }
            Some(body)
        } else {
            None
        };

        let schedule = schedule(&conduction.function);
        let stages = split(&conduction.function, &schedule, &conduction.outputs)
            .map_err(|error| unsupported(artifact, format!("invalidation split: {error}")))?;
        let (stages, slots) = if worth_splitting(&conduction.function, &stages) {
            let slots = stages
                .iter()
                .flat_map(|stage| stage.exports.iter().map(|(slot, _)| *slot as usize + 1))
                .max()
                .unwrap_or(0);
            (stages, slots)
        } else {
            (Vec::new(), 0)
        };

        let branch_of_equation = artifact
            .mir
            .equations
            .iter()
            .map(|equation| {
                artifact
                    .mir
                    .branch_unknowns
                    .iter()
                    .find(|unknown| unknown.equation == equation.id)
                    .map(|unknown| usize::from(unknown.id))
            })
            .collect();

        Ok(Self {
            conduction,
            reactive,
            stages,
            slots,
            node_count: artifact.mir.nodes.len(),
            branch_of_equation,
            ddt_slots,
        })
    }
}

impl Body {
    /// `values` is parallel to `mir.equations`, holding whichever quantity this
    /// body stamps — the residual for conduction, the stored charge for the
    /// reactive matrix.
    fn build(
        function: &CfgFunction,
        artifact: &CanonicalIrArtifact,
        values: &[ValueId],
        rows: &[Vec<Option<ValueId>>],
    ) -> Self {
        let mut plan = StampPlan {
            rows: Vec::with_capacity(artifact.mir.equations.len()),
            structurally_absent: 0,
            folded_to_zero: 0,
        };
        for (index, equation) in artifact.mir.equations.iter().enumerate() {
            let row = rows.get(index).cloned().unwrap_or_default();
            plan.structurally_absent += row.iter().filter(|entry| entry.is_none()).count();
            plan.rows.push(StampRow {
                pos: equation.branch.pos_node,
                neg: equation.branch.neg_node,
                kind: equation.kind,
                residual: values[index],
                derivatives: row
                    .into_iter()
                    .enumerate()
                    .filter_map(|(unknown, entry)| Some((unknown, entry?)))
                    .collect(),
            });
        }

        let wanted = plan.wanted();
        let (function, mapped) = optimize_cfg(function, &wanted);
        plan.remap(&mapped);
        plan.drop_zeros(&function);

        // The body computes exactly the values the stamps read, as one list, and
        // each row records where its own landed.
        let mut outputs = Vec::new();
        let mut positions = Vec::with_capacity(plan.rows.len());
        for row in &plan.rows {
            let residual = outputs.len();
            outputs.push(row.residual);
            let derivatives = row
                .derivatives
                .iter()
                .map(|(_, value)| {
                    outputs.push(*value);
                    outputs.len() - 1
                })
                .collect();
            positions.push((residual, derivatives));
        }

        Self {
            function,
            rows: plan.rows,
            positions,
            outputs,
        }
    }
}

impl ModelPlan {
    fn stamp_file(
        &self,
        artifact: &CanonicalIrArtifact,
        options: &RustTranspileOptions,

    ) -> Result<String, RustBackendError> {
        let mut out = String::new();
        out.push_str(
            "#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]\n\n",
        );
        let _ = writeln!(
            out,
            "use super::state::Instance;\nuse {}::{{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper}};",
            options.runtime_path
        );
        out.push_str(RUNTIME_PRELUDE);
        out.push_str(EVAL_DDT);
        out.push_str("impl Instance {\n");

        for stage in &self.stages {
            if stage.class == InvalidationClass::Newton {
                continue;
            }
            self.emit_cached_stage(artifact, stage, &mut out)?;
        }
        self.emit_stamp(artifact, &mut out)?;
        self.emit_stamp_reactive(artifact, &mut out)?;

        out.push_str("}\n");
        Ok(out)
    }

    /// A stage coarser than Newton: run it once and cache what later readers
    /// take from it.
    fn emit_cached_stage(
        &self,
        artifact: &CanonicalIrArtifact,
        stage: &Stage,

        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let name = stage_fn_name(stage.class);
        let produced: Vec<ValueId> = stage.exports.iter().map(|(_, value)| *value).collect();
        let (body, names) = emit_body(&stage.function, &produced, &bindings())
            .map_err(|error| unsupported(artifact, format!("{name}: {error}")))?;

        let _ = writeln!(
            out,
            "    fn {name}(&mut self, ctx: &GeneratedEvalContext<'_>) {{"
        );
        match stage.class {
            InvalidationClass::Temperature => out.push_str(
                "        let temperature = ctx.temperature();\n\
                 \x20       let thermal_voltage = ctx.thermal_voltage();\n\
                 \x20       if self.canonical_temperature_valid\n\
                 \x20           && self.canonical_temperature == temperature\n\
                 \x20           && self.canonical_thermal_voltage == thermal_voltage\n\
                 \x20       {\n            return;\n        }\n",
            ),
            InvalidationClass::Instance => out
                .push_str("        if self.canonical_instance_valid {\n            return;\n        }\n"),
            // Nothing tells `stamp` that a new timestep began, so this one is
            // recomputed rather than cached.
            _ => {}
        }

        // Captured through a block so the immutable borrow of the slot array
        // ends before the writes into it begin.
        let _ = writeln!(
            out,
            "        let produced: [f64; {}] = {{",
            produced.len().max(1)
        );
        self.emit_prologue(artifact, &stage.function, 3, out)?;
        out.push_str(&indent(&body, 3));
        if produced.is_empty() {
            out.push_str("            [0.0]\n");
        } else {
            let _ = writeln!(out, "            [{}]", names.join(", "));
        }
        out.push_str("        };\n");
        for (index, (slot, _)) in stage.exports.iter().enumerate() {
            let _ = writeln!(
                out,
                "        self.canonical_staged[{slot}] = produced[{index}];"
            );
        }
        match stage.class {
            InvalidationClass::Temperature => out.push_str(
                "        self.canonical_temperature = temperature;\n\
                 \x20       self.canonical_thermal_voltage = thermal_voltage;\n\
                 \x20       self.canonical_temperature_valid = true;\n",
            ),
            InvalidationClass::Instance => {
                out.push_str("        self.canonical_instance_valid = true;\n")
            }
            _ => {}
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    fn emit_stamp(
        &self,
        artifact: &CanonicalIrArtifact,

        out: &mut String,
    ) -> Result<(), RustBackendError> {
        out.push_str(
            "    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n",
        );
        for stage in &self.stages {
            if stage.class == InvalidationClass::Newton {
                continue;
            }
            let _ = writeln!(out, "        self.{}(ctx);", stage_fn_name(stage.class));
        }

        let newton = self
            .stages
            .iter()
            .find(|stage| stage.class == InvalidationClass::Newton);
        let function = newton.map_or(&self.conduction.function, |stage| &stage.function);
        let (body, values) = self.newton_outputs(artifact, newton)?;
        self.emit_prologue(artifact, function, 2, out)?;
        out.push_str(&indent(&body, 2));

        for (index, row) in self.conduction.rows.iter().enumerate() {
            let (residual, derivatives) = &self.conduction.positions[index];
            self.emit_row(
                row,
                &values[*residual],
                &derivatives.iter().map(|at| values[*at].clone()).collect::<Vec<_>>(),
                index,
                Reactive::No,
                out,
            )?;
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    fn emit_stamp_reactive(
        &self,
        artifact: &CanonicalIrArtifact,

        out: &mut String,
    ) -> Result<(), RustBackendError> {
        out.push_str(
            "    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {\n",
        );
        let Some(reactive) = &self.reactive else {
            out.push_str("    }\n");
            return Ok(());
        };
        for stage in &self.stages {
            if stage.class == InvalidationClass::Newton {
                continue;
            }
            let _ = writeln!(out, "        self.{}(ctx);", stage_fn_name(stage.class));
        }

        // The charge is bias-dependent, so the reactive body is the whole thing
        // rather than a Newton slice — it runs once per AC point, not once per
        // Newton iteration, and slicing it would buy nothing.
        let (body, names) = emit_body(&reactive.function, &reactive.outputs, &bindings())
            .map_err(|error| unsupported(artifact, format!("reactive body: {error}")))?;
        self.emit_prologue(artifact, &reactive.function, 2, out)?;
        out.push_str(&indent(&body, 2));

        for (index, row) in reactive.rows.iter().enumerate() {
            if row.derivatives.is_empty() {
                continue;
            }
            let (residual, derivatives) = &reactive.positions[index];
            self.emit_row(
                row,
                &names[*residual],
                &derivatives
                    .iter()
                    .map(|at| names[*at].clone())
                    .collect::<Vec<_>>(),
                index,
                Reactive::Yes,
                out,
            )?;
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    /// The Newton body, and an expression per conduction output.
    ///
    /// When the body is split, an output a coarser stage owns is read from its
    /// slot rather than recomputed, which is the whole point of the split.
    fn newton_outputs(
        &self,
        artifact: &CanonicalIrArtifact,
        newton: Option<&Stage>,
    ) -> Result<(String, Vec<String>), RustBackendError> {
        let Some(newton) = newton else {
            let (body, names) = emit_body(
                &self.conduction.function,
                &self.conduction.outputs,
                &bindings(),
            )
            .map_err(|error| unsupported(artifact, format!("body: {error}")))?;
            return Ok((body, names));
        };

        let owned: Vec<(usize, ValueId)> = newton
            .outputs
            .iter()
            .enumerate()
            .filter_map(|(index, value)| value.map(|value| (index, value)))
            .collect();
        let (body, names) = emit_body(
            &newton.function,
            &owned.iter().map(|(_, value)| *value).collect::<Vec<_>>(),
            &bindings(),
        )
        .map_err(|error| unsupported(artifact, format!("newton stage: {error}")))?;

        let mut values = vec![String::new(); self.conduction.outputs.len()];
        for ((index, _), name) in owned.iter().zip(names) {
            values[*index] = name;
        }
        for (index, value) in values.iter_mut().enumerate() {
            if !value.is_empty() {
                continue;
            }
            let slot = self
                .stages
                .iter()
                .find_map(|stage| stage.outputs[index].and_then(|held| stage.slot_of(held)))
                .ok_or_else(|| {
                    // `split` is supposed to make this unreachable by demanding
                    // every output at the deepest class; if it ever is reached,
                    // the alternative is a silent zero in the matrix.
                    unsupported(
                        artifact,
                        format!("stamp output {index} is computed by no stage and cached by none"),
                    )
                })?;
            *value = format!("staged[{slot}]");
        }
        Ok((body, values))
    }

    /// One equation's stamper calls.
    fn emit_row(
        &self,
        row: &StampRow,
        residual: &str,
        derivatives: &[String],
        equation: usize,
        reactive: Reactive,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let mut nodes = Vec::new();
        let mut node_values = Vec::new();
        let mut branches = Vec::new();
        let mut branch_values = Vec::new();
        for ((unknown, _), value) in row.derivatives.iter().zip(derivatives) {
            if *unknown < self.node_count {
                nodes.push(unknown.to_string());
                node_values.push(value.clone());
            } else {
                branches.push((unknown - self.node_count).to_string());
                branch_values.push(value.clone());
            }
        }
        let pos = optional_node(row.pos);
        let neg = optional_node(row.neg);

        match (row.kind, reactive) {
            (MirEquationKind::Current, Reactive::No) => {
                let _ = writeln!(
                    out,
                    "        stamper.stamp_current_sparse_local::<{}, {}>(\n\
                     \x20           {pos},\n            {neg},\n            multiplicity * ({residual}),\n\
                     \x20           [{}],\n            [{}],\n            [{}],\n            [{}],\n\
                     \x20           multiplicity,\n        );",
                    nodes.len(),
                    branches.len(),
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Current, Reactive::Yes) => {
                let _ = writeln!(
                    out,
                    "        stamper.stamp_current_reactive_indexed_dense_local(\n\
                     \x20           {pos},\n            {neg},\n            &[{}],\n            &[{}],\n\
                     \x20           &[{}],\n            &[{}],\n            multiplicity,\n        );",
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Potential, Reactive::No) => {
                let branch = self.branch_of_equation[equation].ok_or_else(|| {
                    RustBackendError::internal(
                        "",
                        "",
                        format!("potential equation {equation} has no branch unknown"),
                    )
                })?;
                let _ = writeln!(
                    out,
                    "        stamper.stamp_potential_branch_local({pos}, {neg}, {branch}, multiplicity);"
                );
                let _ = writeln!(
                    out,
                    "        stamper.stamp_potential_sparse_local::<{}, {}>(\n\
                     \x20           {branch},\n            {residual},\n\
                     \x20           [{}],\n            [{}],\n            [{}],\n            [{}],\n        );",
                    nodes.len(),
                    branches.len(),
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Potential, Reactive::Yes) => {
                let branch = self.branch_of_equation[equation].ok_or_else(|| {
                    RustBackendError::internal(
                        "",
                        "",
                        format!("potential equation {equation} has no branch unknown"),
                    )
                })?;
                let _ = writeln!(
                    out,
                    "        stamper.stamp_potential_reactive_indexed_dense_local(\n\
                     \x20           {branch},\n            &[{}],\n            &[{}],\n\
                     \x20           &[{}],\n            &[{}],\n        );",
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Indirect, _) => {}
        }
        Ok(())
    }

    /// Everything an emitted body expects to find in scope.
    ///
    /// Only what the body actually reads: a leaf the function does not carry
    /// would otherwise emit a `ctx` call for a quantity nothing wants, and in
    /// the instance stage there is no bias to read it from.
    fn emit_prologue(
        &self,
        artifact: &CanonicalIrArtifact,
        function: &CfgFunction,

        depth: usize,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let pad = "    ".repeat(depth);
        let mut wants = Wants::default();
        for value in &function.values {
            wants.observe(&value.kind);
        }
        // Every stamper call scales by it, whether or not the body reads it.
        wants.multiplicity = true;

        if wants.parameters {
            let _ = writeln!(out, "{pad}let parameters = &self.params.values;");
        }
        if wants.parameter_given {
            let _ = writeln!(out, "{pad}let parameter_given = &*self.param_given;");
        }
        if wants.multiplicity {
            let _ = writeln!(out, "{pad}let multiplicity = self.multiplicity;");
        }
        if wants.time {
            let _ = writeln!(out, "{pad}let time = self.time;");
        }
        if wants.temperature {
            let _ = writeln!(out, "{pad}let temperature = ctx.temperature();");
        }
        if wants.thermal_voltage {
            let _ = writeln!(out, "{pad}let thermal_voltage = ctx.thermal_voltage();");
        }
        if wants.staged {
            let _ = writeln!(out, "{pad}let staged = &*self.canonical_staged;");
        }
        if wants.node_potentials {
            let _ = writeln!(
                out,
                "{pad}let node_potentials = [{}];",
                (0..self.node_count)
                    .map(|index| format!("ctx.node_voltage(self.nodes[{index}])"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if wants.branch_unknown_flows {
            let _ = writeln!(
                out,
                "{pad}let branch_unknown_flows = [{}];",
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| format!("ctx.branch_current(self.branches[{index}])"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if wants.ddt_scale {
            let _ = writeln!(
                out,
                "{pad}let ddt_scale_value = self.ddt_coefficients.derivative_scale;"
            );
            let _ = writeln!(out, "{pad}let ddt_scale = move || ddt_scale_value;");
        }
        if wants.ddt {
            // `ddt` is the one binding that is a call rather than an expression,
            // because it reads and writes per-instance history. The operator id
            // the CFG carries is a source expression, so the slot it was
            // assigned is resolved here rather than looked up at run time.
            let mut arms: Vec<String> = Vec::new();
            for value in &function.values {
                if let CfgValueKind::Ddt { operator, .. } = &value.kind {
                    let slot = self.ddt_slots.get(operator).copied().ok_or_else(|| {
                        unsupported(
                            artifact,
                            format!("a ddt at {operator} with no generated state slot"),
                        )
                    })?;
                    let arm = format!("{} => {slot}usize, ", usize::from(*operator));
                    if !arms.contains(&arm) {
                        arms.push(arm);
                    }
                }
            }
            // The arms cover every `ddt` the body holds, so the fallback is
            // unreachable — and it resolves to an out-of-range slot rather than
            // to slot zero, because if the invariant ever breaks, integrating a
            // charge into the wrong history is the one failure that would look
            // like a converged answer.
            let resolve = match arms.len() {
                1 => arms[0]
                    .split_once("=> ")
                    .map(|(_, slot)| slot.trim_end_matches(", ").to_string())
                    .unwrap_or_else(|| "usize::MAX".to_string()),
                _ => format!("match operator {{ {}_ => usize::MAX }}", arms.concat()),
            };
            let _ = writeln!(out, "{pad}let ddt_state = self.stamp_state.as_mut();");
            let _ = writeln!(
                out,
                "{pad}let ddt_active = self.ddt_coefficients.active;\n\
                 {pad}let ddt_coefficients = self.ddt_coefficients;\n\
                 {pad}let mut ddt = |operator: usize, value: f64| -> f64 {{\n\
                 {pad}    let _ = operator;\n\
                 {pad}    let slot = {resolve};\n\
                 {pad}    rspice_eval_ddt(\n\
                 {pad}        &mut ddt_state.ddt_current,\n\
                 {pad}        &mut ddt_state.ddt_previous,\n\
                 {pad}        &mut ddt_state.ddt_older,\n\
                 {pad}        &mut ddt_state.ddt_initialized,\n\
                 {pad}        &mut ddt_state.ddt_derivative_current,\n\
                 {pad}        &mut ddt_state.ddt_derivative_previous,\n\
                 {pad}        ddt_active,\n\
                 {pad}        ddt_coefficients.derivative_scale,\n\
                 {pad}        ddt_coefficients.previous_value_scale,\n\
                 {pad}        ddt_coefficients.older_value_scale,\n\
                 {pad}        ddt_coefficients.previous_derivative_scale,\n\
                 {pad}        slot,\n\
                 {pad}        value,\n\
                 {pad}    )\n\
                 {pad}}};"
            );
        }
        Ok(())
    }

    fn state_extensions(&self) -> device::StateFileExtensions {
        let mut extensions = device::StateFileExtensions::default();
        if self.slots == 0 {
            return extensions;
        }
        let slots = self.slots;
        extensions.support_types.push_str(
            "fn canonical_boxed_zero_f64<const N: usize>() -> Box<[f64; N]> {\n\
             \x20   // SAFETY: every slot is an f64, and all-zero bytes are 0.0.\n\
             \x20   let mut boxed = Box::<[f64; N]>::new_uninit();\n\
             \x20   unsafe {\n\
             \x20       std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);\n\
             \x20       boxed.assume_init()\n\
             \x20   }\n\
             }\n\n",
        );
        let _ = write!(
            extensions.instance_fields,
            "    pub(crate) canonical_staged: Box<[f64; {slots}]>,\n\
             \x20   pub(crate) canonical_instance_valid: bool,\n\
             \x20   pub(crate) canonical_temperature_valid: bool,\n\
             \x20   pub(crate) canonical_temperature: f64,\n\
             \x20   pub(crate) canonical_thermal_voltage: f64,\n"
        );
        extensions.clone_fields.push_str(
            "            canonical_staged: self.canonical_staged.clone(),\n\
             \x20           canonical_instance_valid: self.canonical_instance_valid,\n\
             \x20           canonical_temperature_valid: self.canonical_temperature_valid,\n\
             \x20           canonical_temperature: self.canonical_temperature,\n\
             \x20           canonical_thermal_voltage: self.canonical_thermal_voltage,\n",
        );
        extensions.new_initializers.push_str(
            "            canonical_staged: canonical_boxed_zero_f64(),\n\
             \x20           canonical_instance_valid: false,\n\
             \x20           canonical_temperature_valid: false,\n\
             \x20           canonical_temperature: 0.0,\n\
             \x20           canonical_thermal_voltage: 0.0,\n",
        );
        // A parameter write invalidates both caches, because a parameter is
        // read by every class.
        extensions.set_parameter_hook.push_str(
            "self.canonical_instance_valid = false;\nself.canonical_temperature_valid = false;\n",
        );
        extensions
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Reactive {
    No,
    Yes,
}

/// Which bindings a body actually reads.
#[derive(Default)]
struct Wants {
    parameters: bool,
    parameter_given: bool,
    node_potentials: bool,
    branch_unknown_flows: bool,
    temperature: bool,
    thermal_voltage: bool,
    multiplicity: bool,
    time: bool,
    ddt: bool,
    ddt_scale: bool,
    staged: bool,
}

impl Wants {
    fn observe(&mut self, kind: &CfgValueKind) {
        match kind {
            CfgValueKind::Parameter(_) => self.parameters = true,
            CfgValueKind::ParameterGiven(_) => self.parameter_given = true,
            CfgValueKind::NodePotential(_) => self.node_potentials = true,
            CfgValueKind::BranchUnknownFlow(_) => self.branch_unknown_flows = true,
            CfgValueKind::Temperature => self.temperature = true,
            CfgValueKind::ThermalVoltage => self.thermal_voltage = true,
            CfgValueKind::Multiplicity => self.multiplicity = true,
            CfgValueKind::Time => self.time = true,
            CfgValueKind::Ddt { .. } => self.ddt = true,
            CfgValueKind::DdtScale => self.ddt_scale = true,
            CfgValueKind::Staged { .. } => self.staged = true,
            _ => {}
        }
    }
}

/// The charge a contribution stores, if it is a `ddt` and nothing else.
///
/// A residual is an accumulator, so `I(a, b) <+ ddt(q)` arrives as `0 + ddt(q)`
/// and simplification has not run yet — the zero is peeled here rather than
/// relied on. What is deliberately *not* accepted is a residual mixing stored
/// charge with conduction in one statement: separating those needs the reactive
/// part tracked through the arithmetic, and calling the whole expression a
/// charge would put conduction into the reactive matrix.
fn stored_charge(function: &CfgFunction, residual: ValueId) -> Option<ValueId> {
    let mut value = residual;
    loop {
        match &function.value(value).kind {
            CfgValueKind::Ddt { input, .. } => return Some(*input),
            CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left,
                right,
            } => {
                let zero = |value: ValueId| {
                    matches!(
                        function.value(value).kind,
                        CfgValueKind::RealConstant(constant) if constant == 0.0
                    )
                };
                value = match (zero(*left), zero(*right)) {
                    (true, false) => *right,
                    (false, true) => *left,
                    _ => return None,
                };
            }
            _ => return None,
        }
    }
}

fn reject_unsupported_kinds(
    artifact: &CanonicalIrArtifact,
    function: &CfgFunction,
) -> Result<(), RustBackendError> {
    for value in &function.values {
        match &value.kind {
            CfgValueKind::Limit { selector, .. } => {
                return Err(unsupported(
                    artifact,
                    format!("$limit ({selector}) has no state slot in the canonical backend yet"),
                ));
            }
            CfgValueKind::BranchFlow(branch) => {
                return Err(unsupported(
                    artifact,
                    format!("an unresolved flow probe on {branch}"),
                ));
            }
            _ => {}
        }
    }
    if artifact
        .mir
        .equations
        .iter()
        .any(|equation| equation.kind == MirEquationKind::Indirect)
    {
        return Err(unsupported(artifact, "an indirect contribution"));
    }
    Ok(())
}

fn stage_fn_name(class: InvalidationClass) -> &'static str {
    match class {
        InvalidationClass::Instance => "canonical_instance_stage",
        InvalidationClass::Temperature => "canonical_temperature_stage",
        InvalidationClass::Timestep => "canonical_timestep_stage",
        InvalidationClass::Newton => "canonical_newton_stage",
    }
}

/// A branch endpoint, as the stamper wants it.
///
/// The *local* ordinal, not `self.nodes[..]`. The stamper resolves a node to a
/// matrix axis through its own per-instance cache, which is keyed by the
/// model's node numbering; handing it the global index would look plausible and
/// address a different node. `ctx.node_voltage`, by contrast, does take the
/// global one — the two are a real distinction and not interchangeable.
fn optional_node(node: Option<crate::canonical_ir::NodeId>) -> String {
    node.map(|node| format!("Some({})", usize::from(node)))
        .unwrap_or_else(|| "None".to_string())
}

fn bindings() -> EmitBindings {
    EmitBindings {
        analysis: "ctx.analysis".into(),
        simparam: "ctx.simparam_or".into(),
        ..EmitBindings::default()
    }
}

fn indent(body: &str, levels: usize) -> String {
    let pad = "    ".repeat(levels);
    let mut out = String::with_capacity(body.len() + body.len() / 8);
    for line in body.lines() {
        if !line.is_empty() {
            out.push_str(&pad);
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

fn unsupported(artifact: &CanonicalIrArtifact, feature: impl Into<String>) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        feature,
    )
}

/// The one runtime helper an emitted body cannot express: `ddt` reads and writes
/// per-instance history, so it is a call rather than an expression.
const EVAL_DDT: &str = r#"
#[inline]
fn rspice_eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    active: bool,
    scale: f64,
    previous_value_scale: f64,
    older_value_scale: f64,
    previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if active {
        let result = value * scale
            - previous_value * previous_value_scale
            - older_value * older_value_scale
            - derivative_previous[slot] * previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

"#;
