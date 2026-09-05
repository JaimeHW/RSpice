//! Checked construction of one AArch64 executable model image.
//!
//! Every callable entry is 16-byte aligned, begins with BTI C, is decoded
//! independently before insertion, and is decoded again at finalization so
//! direct calls can be authenticated against the complete entry-point set.

use super::codegen::{
    A64FusedKernelEntries, compile_assignment_dispatch_function, compile_assignment_pass_function,
    compile_fused_evaluation_driver, compile_fused_evaluation_kernel, compile_fused_stamp_driver,
    compile_fused_stamp_kernel, compile_loop_dispatch_function,
    compile_segmented_assignment_driver, compile_segmented_indexed_assignment_driver,
    compile_segmented_program, compile_segmented_value_driver, compile_value_function,
    compile_value_function_from_ssa,
};
use super::unwind::{A64UnwindFunction, analyze_function};
use super::verifier::{DirectBranchKind, VerifiedA64Code, verify_exact_function_at};
use crate::native::assignment::{NativeAssignment, chunk_ranges, shareable_batch_ranges};
use crate::native::expr::{NativeProgram, native_op_name};
use crate::native::model::{CodeOffset, NativeEntryStarts};
use crate::native::plan_program::{PlanProgram, PlanProgramRef};
use crate::native::ssa::{AssignmentProgram, Program};
use crate::native::value_cache::ValueEntryCache;
use crate::native::{JitError, JitResult};

const MODEL: &str = "native-aarch64-image";
const ENTRY_ALIGNMENT: usize = 16;
// The size past which an entry is cut into separately published pieces where
// its form allows it, rather than emitted as one function.
//
// The number is the largest function one Windows ARM64 `.xdata` record can
// describe — its Function Length field is an 18-bit instruction count — and it
// was a ceiling when that was the only shape the metadata had. It is not one
// any more, which is why the name says threshold. Nothing in the instruction
// stream stops here: `B`/`BL` carry an imm26 word displacement and reach 128
// MiB, `B.cond`/`CBZ`/`CBNZ` are always emitted in the long form (the inverse
// condition over an unconditional `B`, see `A64Encoder`), and `LDR` (literal)
// reaches its constants through inline islands. Nor does the metadata: a
// function past this size is described by several `.pdata`/`.xdata` fragments,
// which `append_windows_unwind_data` emits.
//
// So what is left is a preference. A postfix program over this size is cut into
// numbered pieces behind a driver; a block program cannot be cut at an
// arbitrary operation and is emitted whole. The common shape stays one function
// with one unwind record, and only what cannot be cut pays for several.
pub(crate) const A64_SEGMENT_THRESHOLD_BYTES: usize = 0x3ffff * 4;
const A64_NOP: [u8; 4] = 0xD503_201F_u32.to_le_bytes();
const A64_BTI_C: [u8; 4] = 0xD503_245F_u32.to_le_bytes();

#[derive(Debug)]
struct FunctionLayout {
    start: CodeOffset,
    end: usize,
    entry_kind: Box<str>,
    verified: VerifiedA64Code,
}

#[derive(Debug, Default)]
pub(super) struct A64ImageBuilder {
    image: Vec<u8>,
    functions: Vec<FunctionLayout>,
    value_entries: ValueEntryCache<CodeOffset>,
}

impl A64ImageBuilder {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn prepare_entry_offset(&mut self) -> CodeOffset {
        while self.image.len() % ENTRY_ALIGNMENT != 0 {
            self.image.extend_from_slice(&A64_NOP);
        }
        CodeOffset::new(self.image.len())
    }

    pub(super) fn append_function(
        &mut self,
        bytes: Vec<u8>,
        entry_kind: &str,
    ) -> JitResult<CodeOffset> {
        let offset = self.prepare_entry_offset();
        self.append_function_at(offset, bytes, entry_kind)?;
        Ok(offset)
    }

    pub(super) fn append_function_at(
        &mut self,
        offset: CodeOffset,
        bytes: Vec<u8>,
        entry_kind: &str,
    ) -> JitResult<()> {
        if offset.as_usize() != self.image.len() {
            return Err(internal_error(format!(
                "AArch64 {entry_kind} offset {} does not match image length {}",
                offset.as_usize(),
                self.image.len()
            )));
        }
        if bytes.get(..A64_BTI_C.len()) != Some(A64_BTI_C.as_slice()) {
            return Err(JitError::Verifier {
                model: MODEL.into(),
                detail: format!("AArch64 {entry_kind} does not begin with BTI C").into(),
            });
        }
        let verified = verify_exact_function_at(&bytes, entry_kind, offset.as_usize())?;
        let end = offset
            .as_usize()
            .checked_add(bytes.len())
            .ok_or_else(|| internal_error("AArch64 function image range overflow"))?;
        self.image.extend_from_slice(&bytes);
        self.functions.push(FunctionLayout {
            start: offset,
            end,
            entry_kind: entry_kind.into(),
            verified,
        });
        Ok(())
    }

    /// Publish one plan entry, reusing an identical body if one is already in
    /// the image.
    ///
    /// Both forms reach the same emitter, allocator and verifier: a postfix
    /// entry is lifted into SSA as it always was, a block entry is already
    /// there.
    ///
    /// The one thing only a postfix entry can do is *segment*. A postfix entry
    /// over [`A64_SEGMENT_THRESHOLD_BYTES`] is split into numbered pieces behind a
    /// driver — a rewrite of the operand stream that has no block-form
    /// counterpart, because a block program's control flow does not survive
    /// being cut at an arbitrary operation. A block entry over that size is
    /// emitted whole: the code is correct at any length, and the only thing
    /// that cannot describe it is one platform's unwind metadata, which is
    /// where that refusal belongs.
    pub(super) fn append_value(
        &mut self,
        program: PlanProgramRef<'_>,
        entry_kind: &str,
    ) -> JitResult<CodeOffset> {
        if let Some(offset) = self.value_entries.lookup(program) {
            return Ok(offset);
        }
        let bytes = match program {
            PlanProgramRef::Postfix(program) => compile_value_function(program)?,
            PlanProgramRef::Blocks(program) => compile_value_function_from_ssa(program.ssa())?,
        };
        let offset = match program {
            PlanProgramRef::Postfix(postfix) if bytes.len() > A64_SEGMENT_THRESHOLD_BYTES => {
                self.append_segmented_value(postfix, entry_kind)
            }
            PlanProgramRef::Postfix(_) | PlanProgramRef::Blocks(_) => {
                self.append_function(bytes, entry_kind)
            }
        }?;
        self.value_entries.insert(program, offset);
        Ok(offset)
    }

    pub(super) fn append_assignment_pass(
        &mut self,
        assignments: &[NativeAssignment],
        entry_kind: &str,
    ) -> JitResult<CodeOffset> {
        if std::env::var_os("RSPICE_NATIVE_A64_IMAGE_TRACE").is_some() {
            let mut direct_count = 0_usize;
            let mut native_operations = 0_usize;
            let mut ssa_instructions = 0_usize;
            let mut largest_native = 0_usize;
            let mut largest_ssa = 0_usize;
            for assignment in assignments {
                if let NativeAssignment::Direct { program, .. } = assignment {
                    direct_count += 1;
                    native_operations = native_operations.saturating_add(program.ops().len());
                    largest_native = largest_native.max(program.ops().len());
                    let lowered = Program::lower(program)?;
                    ssa_instructions =
                        ssa_instructions.saturating_add(lowered.instructions().len());
                    largest_ssa = largest_ssa.max(lowered.instructions().len());
                }
            }
            let mut shared_batch_count = 0_usize;
            let mut shared_ssa_instructions = 0_usize;
            let mut largest_batch_assignments = 0_usize;
            let mut largest_batch_ssa = 0_usize;
            let mut operation_kinds = std::collections::BTreeMap::<&str, usize>::new();
            for range in shareable_batch_ranges(assignments) {
                let batch = &assignments[range];
                if !matches!(batch.first(), Some(NativeAssignment::Direct { .. })) {
                    continue;
                }
                let direct = batch
                    .iter()
                    .map(|assignment| match assignment {
                        NativeAssignment::Direct { var_index, program } => (*var_index, program),
                        NativeAssignment::Indexed { .. } | NativeAssignment::Loop { .. } => {
                            unreachable!("shareable direct batch")
                        }
                    })
                    .collect::<Vec<_>>();
                let shared = AssignmentProgram::lower(&direct)?;
                shared_batch_count += 1;
                shared_ssa_instructions =
                    shared_ssa_instructions.saturating_add(shared.program().instructions().len());
                largest_batch_assignments = largest_batch_assignments.max(direct.len());
                largest_batch_ssa = largest_batch_ssa.max(shared.program().instructions().len());
                for instruction in shared.program().instructions() {
                    *operation_kinds
                        .entry(native_op_name(&instruction.op()))
                        .or_default() += 1;
                }
            }
            eprintln!(
                "RSPICE A64 assignment plan kind={entry_kind:?} direct={direct_count} native_ops={native_operations} ssa_instructions={ssa_instructions} largest_native={largest_native} largest_ssa={largest_ssa} shared_batches={shared_batch_count} shared_ssa={shared_ssa_instructions} largest_batch_assignments={largest_batch_assignments} largest_batch_ssa={largest_batch_ssa}"
            );
            eprintln!("RSPICE A64 shared assignment operations={operation_kinds:?}");
        }
        let ranges = chunk_ranges(assignments);
        if ranges.is_empty() {
            return self
                .append_function(compile_assignment_pass_function(assignments)?, entry_kind);
        }

        let mut chunks = Vec::with_capacity(ranges.len());
        for (index, range) in ranges.into_iter().enumerate() {
            let chunk_kind = format!("{entry_kind} chunk {index}");
            self.append_bounded_assignment_chunk(&assignments[range], &chunk_kind, &mut chunks)?;
        }
        if let [only] = chunks.as_slice() {
            return Ok(*only);
        }

        let dispatcher = self.prepare_entry_offset();
        let bytes = compile_assignment_dispatch_function(dispatcher.as_usize(), &chunks)?;
        self.append_function_at(dispatcher, bytes, entry_kind)?;
        Ok(dispatcher)
    }

    fn append_bounded_assignment_chunk(
        &mut self,
        assignments: &[NativeAssignment],
        entry_kind: &str,
        chunks: &mut Vec<CodeOffset>,
    ) -> JitResult<()> {
        let bytes = compile_assignment_pass_function(assignments)?;
        if bytes.len() <= A64_SEGMENT_THRESHOLD_BYTES {
            chunks.push(self.append_function(bytes, entry_kind)?);
            return Ok(());
        }
        if let [NativeAssignment::Direct { var_index, program }] = assignments {
            chunks.push(self.append_segmented_assignment(program, *var_index, entry_kind)?);
            return Ok(());
        }
        if let [
            NativeAssignment::Indexed {
                base,
                len,
                lower,
                index,
                value,
            },
        ] = assignments
        {
            chunks.push(self.append_segmented_indexed_assignment(
                *base, *len, *lower, index, value, entry_kind,
            )?);
            return Ok(());
        }
        if let [NativeAssignment::Loop { condition, body }] = assignments {
            chunks.push(self.append_segmented_loop(condition, body, entry_kind)?);
            return Ok(());
        }
        debug_assert!(assignments.len() > 1);

        let midpoint = assignments.len() / 2;
        self.append_bounded_assignment_chunk(
            &assignments[..midpoint],
            &format!("{entry_kind}.0"),
            chunks,
        )?;
        self.append_bounded_assignment_chunk(
            &assignments[midpoint..],
            &format!("{entry_kind}.1"),
            chunks,
        )
    }

    fn append_segmented_value(
        &mut self,
        program: &NativeProgram,
        entry_kind: &str,
    ) -> JitResult<CodeOffset> {
        let segmented = compile_segmented_program(program)?;
        let mut segment_entries = Vec::with_capacity(segmented.functions.len());
        for (index, bytes) in segmented.functions.into_iter().enumerate() {
            segment_entries
                .push(self.append_function(bytes, &format!("{entry_kind} segment {index}"))?);
        }
        let driver = self.prepare_entry_offset();
        let bytes = compile_segmented_value_driver(
            driver.as_usize(),
            &segment_entries,
            segmented.value_count,
            segmented.result_index,
        )?;
        self.append_function_at(driver, bytes, entry_kind)?;
        Ok(driver)
    }

    fn append_segmented_assignment(
        &mut self,
        program: &NativeProgram,
        variable_index: usize,
        entry_kind: &str,
    ) -> JitResult<CodeOffset> {
        let segmented = compile_segmented_program(program)?;
        let mut segment_entries = Vec::with_capacity(segmented.functions.len());
        for (index, bytes) in segmented.functions.into_iter().enumerate() {
            segment_entries
                .push(self.append_function(bytes, &format!("{entry_kind} segment {index}"))?);
        }
        let driver = self.prepare_entry_offset();
        let bytes = compile_segmented_assignment_driver(
            driver.as_usize(),
            &segment_entries,
            segmented.value_count,
            segmented.result_index,
            variable_index,
        )?;
        self.append_function_at(driver, bytes, entry_kind)?;
        Ok(driver)
    }

    #[allow(clippy::too_many_arguments)]
    fn append_segmented_indexed_assignment(
        &mut self,
        base: usize,
        len: usize,
        lower: i64,
        index: &NativeProgram,
        value: &NativeProgram,
        entry_kind: &str,
    ) -> JitResult<CodeOffset> {
        let segmented_index = compile_segmented_program(index)?;
        let mut index_entries = Vec::with_capacity(segmented_index.functions.len());
        for (segment, bytes) in segmented_index.functions.into_iter().enumerate() {
            index_entries.push(
                self.append_function(bytes, &format!("{entry_kind} index segment {segment}"))?,
            );
        }
        let segmented_value = compile_segmented_program(value)?;
        let mut value_entries = Vec::with_capacity(segmented_value.functions.len());
        for (segment, bytes) in segmented_value.functions.into_iter().enumerate() {
            value_entries.push(
                self.append_function(bytes, &format!("{entry_kind} value segment {segment}"))?,
            );
        }
        let driver = self.prepare_entry_offset();
        let bytes = compile_segmented_indexed_assignment_driver(
            driver.as_usize(),
            &index_entries,
            segmented_index.value_count,
            segmented_index.result_index,
            &value_entries,
            segmented_value.value_count,
            segmented_value.result_index,
            base,
            len,
            lower,
        )?;
        self.append_function_at(driver, bytes, entry_kind)?;
        Ok(driver)
    }

    fn append_segmented_loop(
        &mut self,
        condition: &NativeProgram,
        body: &[NativeAssignment],
        entry_kind: &str,
    ) -> JitResult<CodeOffset> {
        let mut body_chunks = Vec::new();
        for (index, range) in chunk_ranges(body).into_iter().enumerate() {
            self.append_bounded_assignment_chunk(
                &body[range],
                &format!("{entry_kind} body chunk {index}"),
                &mut body_chunks,
            )?;
        }
        let driver = self.prepare_entry_offset();
        let bytes = compile_loop_dispatch_function(driver.as_usize(), condition, &body_chunks)?;
        self.append_function_at(driver, bytes, entry_kind)?;
        Ok(driver)
    }

    pub(super) fn append_fused_evaluation_kernel(
        &mut self,
        assignment: CodeOffset,
        prelude: Option<CodeOffset>,
        stamp_values: &[PlanProgram],
        stamp_value_entries: &[CodeOffset],
        published_current_pairs: &[Option<(usize, usize)>],
    ) -> JitResult<CodeOffset> {
        let offset = self.prepare_entry_offset();
        let inline = compile_fused_evaluation_kernel(
            offset.as_usize(),
            assignment,
            prelude,
            stamp_values,
            A64FusedKernelEntries {
                stamp_values: stamp_value_entries,
                jacobians: &[],
            },
            published_current_pairs,
        )?;
        let bytes = if inline.len() <= A64_SEGMENT_THRESHOLD_BYTES {
            inline
        } else {
            compile_fused_evaluation_driver(
                offset.as_usize(),
                assignment,
                prelude,
                stamp_value_entries,
                published_current_pairs,
            )?
        };
        self.append_function_at(offset, bytes, "fused evaluation kernel")?;
        Ok(offset)
    }

    pub(super) fn append_fused_stamp_kernel(
        &mut self,
        assignment: CodeOffset,
        prelude: Option<CodeOffset>,
        stamp_values: &[PlanProgram],
        jacobians: &[Vec<PlanProgram>],
        stamp_value_entries: &[CodeOffset],
        jacobian_entries: &[Vec<CodeOffset>],
        published_current_pairs: &[Option<(usize, usize)>],
    ) -> JitResult<CodeOffset> {
        let offset = self.prepare_entry_offset();
        let inline = compile_fused_stamp_kernel(
            offset.as_usize(),
            assignment,
            prelude,
            stamp_values,
            jacobians,
            A64FusedKernelEntries {
                stamp_values: stamp_value_entries,
                jacobians: jacobian_entries,
            },
            published_current_pairs,
        )?;
        let bytes = if inline.len() <= A64_SEGMENT_THRESHOLD_BYTES {
            inline
        } else {
            compile_fused_stamp_driver(
                offset.as_usize(),
                assignment,
                prelude,
                stamp_value_entries,
                jacobian_entries,
                published_current_pairs,
            )?
        };
        self.append_function_at(offset, bytes, "fused stamp kernel")?;
        Ok(offset)
    }

    pub(super) fn finish(self) -> JitResult<(Vec<u8>, NativeEntryStarts, Vec<A64UnwindFunction>)> {
        if self.functions.is_empty() {
            return Err(internal_error(
                "cannot publish an empty AArch64 executable image",
            ));
        }
        let mut starts = self
            .functions
            .iter()
            .map(|function| function.start)
            .collect::<Vec<_>>();
        starts.sort_unstable();
        starts.dedup();

        if std::env::var_os("RSPICE_NATIVE_A64_IMAGE_TRACE").is_some() {
            let mut categories = std::collections::BTreeMap::<&str, (usize, usize)>::new();
            for function in &self.functions {
                let category = function
                    .entry_kind
                    .split_ascii_whitespace()
                    .next()
                    .unwrap_or("unknown");
                let aggregate = categories.entry(category).or_default();
                aggregate.0 += 1;
                aggregate.1 += function.end - function.start.as_usize();
            }
            for (category, (functions, bytes)) in categories {
                eprintln!(
                    "native-aarch64-image category={category} functions={functions} bytes={bytes}"
                );
            }
            let mut largest = self
                .functions
                .iter()
                .map(|function| {
                    (
                        function.end - function.start.as_usize(),
                        function.entry_kind.as_ref(),
                    )
                })
                .collect::<Vec<_>>();
            largest.sort_unstable_by(|left, right| right.0.cmp(&left.0));
            for (bytes, kind) in largest.into_iter().take(40) {
                eprintln!("native-aarch64-image function_bytes={bytes} entry={kind}");
            }
        }

        let mut unwind_functions = Vec::with_capacity(self.functions.len());
        for function in &self.functions {
            if function.end > self.image.len() || function.start.as_usize() >= function.end {
                return Err(internal_error(
                    "AArch64 function range is outside its image",
                ));
            }
            for branch in &function.verified.direct_branches {
                if branch.kind != DirectBranchKind::Call {
                    continue;
                }
                let target = i64::try_from(function.start.as_usize())
                    .ok()
                    .and_then(|start| start.checked_add(branch.target_offset))
                    .and_then(|target| usize::try_from(target).ok())
                    .map(CodeOffset::new)
                    .ok_or_else(|| {
                        relocation_error(format!(
                            "AArch64 call at image byte {} has an overflowing target",
                            function.start.as_usize() + branch.instruction_offset
                        ))
                    })?;
                if starts.binary_search(&target).is_err() {
                    return Err(relocation_error(format!(
                        "AArch64 call at image byte {} targets non-entry byte {}",
                        function.start.as_usize() + branch.instruction_offset,
                        target.as_usize()
                    )));
                }
            }
            unwind_functions.push(analyze_function(
                function.start,
                &self.image[function.start.as_usize()
                    ..function.start.as_usize() + function.verified.code_bytes],
                "published image entry",
            )?);
        }

        Ok((self.image, NativeEntryStarts::new(starts), unwind_functions))
    }
}

fn internal_error(detail: impl Into<String>) -> JitError {
    JitError::InternalCompilerError {
        model: MODEL.into(),
        detail: detail.into().into(),
    }
}

fn relocation_error(detail: impl Into<String>) -> JitError {
    JitError::Relocation {
        model: MODEL.into(),
        detail: detail.into().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::A64ImageBuilder;
    #[cfg(target_arch = "aarch64")]
    use crate::native::aarch64::codegen::{
        compile_assignment_dispatch_function, compile_assignment_pass_function,
        compile_loop_dispatch_function, compile_segmented_program,
    };
    use crate::native::aarch64::encoder::A64Encoder;
    #[cfg(target_arch = "aarch64")]
    use crate::native::assignment::NativeAssignment;
    use crate::native::expr::{NativeOp, NativeProgram};
    #[cfg(target_arch = "aarch64")]
    use crate::native::model::{
        NativeCurrentDependencies, NativeEntryOffsets, NativeModel, NativeRequiredStorage,
        NativeStampKernelIo,
    };
    #[cfg(target_arch = "aarch64")]
    use crate::native::runtime::ExecutableMemory;
    #[cfg(target_arch = "aarch64")]
    use crate::native::{EvalContext, JitResult};

    fn program(ops: Vec<NativeOp>, max_stack_depth: usize) -> NativeProgram {
        NativeProgram::from_ops_for_test(ops, max_stack_depth, Vec::new(), Vec::new())
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn publishes_aligned_entries_and_authenticates_dispatch_calls() -> JitResult<()> {
        let chunks = [
            vec![NativeAssignment::Direct {
                var_index: 0,
                program: program(vec![NativeOp::Const(1.0)], 1),
            }],
            vec![NativeAssignment::Direct {
                var_index: 1,
                program: program(vec![NativeOp::LoadVariable(0), NativeOp::AddConst(2.0)], 1),
            }],
        ];
        let mut builder = A64ImageBuilder::new();
        let mut chunk_offsets = Vec::new();
        for chunk in &chunks {
            chunk_offsets.push(
                builder.append_function(
                    compile_assignment_pass_function(chunk)?,
                    "assignment chunk",
                )?,
            );
        }
        let dispatcher = builder.prepare_entry_offset();
        let bytes = compile_assignment_dispatch_function(dispatcher.as_usize(), &chunk_offsets)?;
        builder.append_function_at(dispatcher, bytes, "assignment dispatcher")?;
        let (image, _, _) = builder.finish()?;
        let memory = ExecutableMemory::allocate(&image)?;
        let entry: extern "C" fn(*const EvalContext, *mut f64) = unsafe {
            std::mem::transmute(
                memory
                    .ptr_at(dispatcher.as_usize())
                    .expect("AArch64 image dispatcher pointer"),
            )
        };
        let context = EvalContext::empty_for_test();
        let mut variables = [100.0_f64, 200.0];
        entry(&context, variables.as_mut_ptr());
        assert_eq!(variables, [1.0, 3.0]);
        Ok(())
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn segmented_value_executes_across_native_function_boundaries() -> JitResult<()> {
        let operation_count = 2_050;
        let mut ops = Vec::with_capacity(operation_count + 1);
        ops.push(NativeOp::LoadVariable(0));
        ops.extend(std::iter::repeat_n(
            NativeOp::AddConst(0.25),
            operation_count,
        ));
        let expression = program(ops, 1);
        let segmented = compile_segmented_program(&expression)?;
        assert_eq!(segmented.functions.len(), 3);

        let mut builder = A64ImageBuilder::new();
        let entry = builder.append_segmented_value(&expression, "segmented value")?;
        let (image, _, _) = builder.finish()?;
        let memory = ExecutableMemory::allocate(&image)?;
        let function: extern "C" fn(*const EvalContext, *const f64) -> f64 = unsafe {
            std::mem::transmute(
                memory
                    .ptr_at(entry.as_usize())
                    .expect("AArch64 segmented value pointer"),
            )
        };
        let context = EvalContext::empty_for_test();
        let variables = [2.0_f64];
        assert_eq!(
            function(&context, variables.as_ptr()),
            2.0 + operation_count as f64 * 0.25
        );
        assert!(context.take_runtime_error().is_none());
        Ok(())
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn loop_dispatch_executes_native_body_chunks_in_source_order() -> JitResult<()> {
        let chunks = [
            vec![NativeAssignment::Direct {
                var_index: 1,
                program: program(vec![NativeOp::LoadVariable(1), NativeOp::AddConst(1.0)], 1),
            }],
            vec![NativeAssignment::Direct {
                var_index: 0,
                program: program(vec![NativeOp::LoadVariable(0), NativeOp::SubConst(1.0)], 1),
            }],
        ];
        let mut builder = A64ImageBuilder::new();
        let mut chunk_offsets = Vec::with_capacity(chunks.len());
        for chunk in &chunks {
            chunk_offsets.push(
                builder
                    .append_function(compile_assignment_pass_function(chunk)?, "loop body chunk")?,
            );
        }
        let condition = program(vec![NativeOp::LoadVariable(0)], 1);
        let dispatcher = builder.prepare_entry_offset();
        let bytes =
            compile_loop_dispatch_function(dispatcher.as_usize(), &condition, &chunk_offsets)?;
        builder.append_function_at(dispatcher, bytes, "loop dispatcher")?;
        let (image, _, _) = builder.finish()?;
        let memory = ExecutableMemory::allocate(&image)?;
        let entry: extern "C" fn(*const EvalContext, *mut f64) = unsafe {
            std::mem::transmute(
                memory
                    .ptr_at(dispatcher.as_usize())
                    .expect("AArch64 loop dispatcher pointer"),
            )
        };
        let context = EvalContext::empty_for_test();
        let mut variables = [3.0_f64, 10.0];
        entry(&context, variables.as_mut_ptr());
        assert_eq!(variables, [0.0, 13.0]);
        assert!(context.take_runtime_error().is_none());
        Ok(())
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn segmented_indexed_assignment_preserves_index_then_value_semantics() -> JitResult<()> {
        let mut index_ops = Vec::with_capacity(2_051);
        index_ops.push(NativeOp::Const(1.0));
        for _ in 0..1_025 {
            index_ops.push(NativeOp::AddConst(0.25));
            index_ops.push(NativeOp::SubConst(0.25));
        }
        let mut value_ops = Vec::with_capacity(2_051);
        value_ops.push(NativeOp::Const(4.0));
        value_ops.extend(std::iter::repeat_n(NativeOp::AddConst(0.5), 2_050));
        let index = program(index_ops, 1);
        let value = program(value_ops, 1);

        let mut builder = A64ImageBuilder::new();
        let entry = builder.append_segmented_indexed_assignment(
            0,
            2,
            1,
            &index,
            &value,
            "segmented indexed assignment",
        )?;
        let (image, _, _) = builder.finish()?;
        let memory = ExecutableMemory::allocate(&image)?;
        let function: extern "C" fn(*const EvalContext, *mut f64) = unsafe {
            std::mem::transmute(
                memory
                    .ptr_at(entry.as_usize())
                    .expect("AArch64 segmented indexed-assignment pointer"),
            )
        };
        let context = EvalContext::empty_for_test();
        let mut variables = [10.0_f64, 20.0];
        function(&context, variables.as_mut_ptr());
        assert_eq!(variables, [4.0 + 2_050.0 * 0.5, 20.0]);
        assert!(context.take_runtime_error().is_none());
        Ok(())
    }

    #[test]
    fn rejects_calls_to_bytes_that_are_not_entry_starts() {
        let mut builder = A64ImageBuilder::new();
        let mut first = A64Encoder::new();
        first.bti_c();
        first.ret();
        let first_offset = builder
            .append_function(first.into_bytes(), "first")
            .expect("append first function");

        let second_offset = builder.prepare_entry_offset();
        let mut second = A64Encoder::new();
        second.bti_c();
        let call = second.bl_placeholder();
        second
            .patch_branch_to_image_offset(
                call,
                second_offset.as_usize(),
                first_offset.as_usize() + 4,
            )
            .expect("patch call into first function");
        second.ret();
        builder
            .append_function_at(second_offset, second.into_bytes(), "second")
            .expect("append second function");
        let error = builder.finish().expect_err("non-entry call must fail");
        assert!(error.to_string().contains("non-entry byte"));
    }

    #[test]
    fn rejects_authenticated_image_calls_to_non_entry_bytes() {
        use crate::native::aarch64::encoder::XReg;

        let mut builder = A64ImageBuilder::new();
        let mut first = A64Encoder::new();
        first.bti_c();
        first.ret();
        let first_offset = builder
            .append_function(first.into_bytes(), "first")
            .expect("append first function");

        let second_offset = builder.prepare_entry_offset();
        let mut second = A64Encoder::new();
        second.bti_c();
        second
            .image_call(
                second_offset.as_usize(),
                first_offset.as_usize() + 4,
                XReg::X16,
            )
            .expect("encode image call into first function");
        second.ret();
        builder
            .append_function_at(second_offset, second.into_bytes(), "second")
            .expect("append second function");
        let error = builder
            .finish()
            .expect_err("non-entry image call must fail");
        assert!(error.to_string().contains("non-entry byte"));
    }

    #[test]
    fn rejects_entries_without_bti() {
        let mut encoder = A64Encoder::new();
        encoder.ret();
        let error = A64ImageBuilder::new()
            .append_function(encoder.into_bytes(), "unsafe entry")
            .expect_err("missing BTI must fail");
        assert!(error.to_string().contains("BTI C"));
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn complete_native_model_image_executes_every_entry_family() -> JitResult<()> {
        let assignments = [NativeAssignment::Direct {
            var_index: 0,
            program: program(vec![NativeOp::Const(2.0)], 1),
        }];
        let post_assignments = [NativeAssignment::Direct {
            var_index: 1,
            program: program(vec![NativeOp::LoadVariable(0), NativeOp::AddConst(3.0)], 1),
        }];
        let parameter_default = program(vec![NativeOp::Const(10.0)], 1);
        let static_condition = program(vec![NativeOp::Const(1.0)], 1);
        let stamp_value = program(vec![NativeOp::LoadVariable(0), NativeOp::MulConst(4.0)], 1);
        let jacobian = program(vec![NativeOp::Const(5.0)], 1);
        let reactive_jacobian = program(vec![NativeOp::Const(6.0)], 1);
        let noise_psd = program(vec![NativeOp::Const(7.0)], 1);
        let noise_exponent = program(vec![NativeOp::Const(8.0)], 1);

        let mut builder = A64ImageBuilder::new();
        let assignment = builder.append_assignment_pass(&assignments, "assignment")?;
        let post_assignment =
            builder.append_assignment_pass(&post_assignments, "post-assignment")?;
        let parameter_default_entry =
            builder.append_value(&parameter_default, "parameter default")?;
        let static_condition_entry = builder.append_value(&static_condition, "static condition")?;
        let stamp_value_entry = builder.append_value(&stamp_value, "stamp value")?;
        let jacobian_entry = builder.append_value(&jacobian, "Jacobian")?;
        let reactive_jacobian_entry =
            builder.append_value(&reactive_jacobian, "reactive Jacobian")?;
        let noise_psd_entry = builder.append_value(&noise_psd, "noise PSD")?;
        let noise_exponent_entry = builder.append_value(&noise_exponent, "noise exponent")?;
        let stamp_programs = [stamp_value];
        let jacobian_programs = [vec![jacobian]];
        let published_pairs = [None];
        let evaluation_kernel = builder.append_fused_evaluation_kernel(
            assignment,
            &stamp_programs,
            &[stamp_value_entry],
            &published_pairs,
        )?;
        let stamp_kernel = builder.append_fused_stamp_kernel(
            assignment,
            &stamp_programs,
            &jacobian_programs,
            &[stamp_value_entry],
            &[vec![jacobian_entry]],
            &published_pairs,
        )?;
        let (image, entry_starts, _) = builder.finish()?;

        let entries = NativeEntryOffsets {
            assignment,
            post_assignment: Some(post_assignment),
            evaluation_kernel: Some(evaluation_kernel),
            stamp_kernel: Some(stamp_kernel),
            parameter_defaults: vec![Some(parameter_default_entry)],
            static_conditions: vec![Some(static_condition_entry)],
            stamp_values: vec![stamp_value_entry],
            jacobians: vec![vec![jacobian_entry]],
            reactive_jacobians: vec![vec![reactive_jacobian_entry]],
            noise_psd: vec![noise_psd_entry],
            noise_exponents: vec![Some(noise_exponent_entry)],
        };
        let dependencies = NativeCurrentDependencies {
            static_condition_branch_unknowns: vec![Vec::new()],
            stamp_values: vec![Vec::new()],
            stamp_value_prior_currents: vec![Vec::new()],
            stamp_value_branch_unknowns: vec![Vec::new()],
            jacobians: vec![vec![Vec::new()]],
            jacobian_prior_currents: vec![vec![Vec::new()]],
            jacobian_branch_unknowns: vec![vec![Vec::new()]],
            reactive_jacobians: vec![vec![Vec::new()]],
            reactive_jacobian_prior_currents: vec![vec![Vec::new()]],
            reactive_jacobian_branch_unknowns: vec![vec![Vec::new()]],
            noise_psd: vec![Vec::new()],
            noise_psd_prior_currents: vec![Vec::new()],
            noise_psd_branch_unknowns: vec![Vec::new()],
            noise_exponents: vec![Vec::new()],
            noise_exponent_prior_currents: vec![Vec::new()],
            noise_exponent_branch_unknowns: vec![Vec::new()],
            ..NativeCurrentDependencies::default()
        };
        let native = NativeModel::from_executable_image_with_dependencies(
            0,
            0,
            2,
            1,
            0,
            entries.stamp_values.len(),
            ExecutableMemory::allocate(&image)?,
            entries,
            entry_starts,
            dependencies,
            NativeRequiredStorage::default(),
            crate::jit::model_plan::NativeAssignmentCoverage::ObservableVariables,
        )?;

        let mut currents = [-1.0_f64];
        let mut context = EvalContext::empty_for_test();
        context.currents = currents.as_mut_ptr();
        context.currents_len = currents.len();
        let mut variables = [0.0_f64, 0.0];
        native.run_assignments(&context, variables.as_mut_ptr());
        assert_eq!(variables, [2.0, 0.0]);
        assert!(native.run_post_assignments(&context, variables.as_mut_ptr()));
        assert_eq!(variables, [2.0, 5.0]);
        assert_eq!(
            native.run_parameter_default(0, &context, variables.as_ptr()),
            Some(10.0)
        );
        assert_eq!(
            native.run_static_condition(0, &context, variables.as_ptr()),
            Some(1.0)
        );
        assert_eq!(
            native.run_stamp_value(0, &context, variables.as_ptr()),
            Some(8.0)
        );
        assert_eq!(
            native.run_jacobian(0, 0, &context, variables.as_ptr()),
            Some(5.0)
        );
        assert_eq!(
            native.run_reactive_jacobian(0, 0, &context, variables.as_ptr()),
            Some(6.0)
        );
        assert_eq!(
            native.run_noise_psd(0, &context, variables.as_ptr()),
            Some(7.0)
        );
        assert_eq!(
            native.run_noise_exponent(0, &context, variables.as_ptr()),
            Some(8.0)
        );

        let active = [1_u8];
        let evaluation_io = NativeStampKernelIo {
            program_active: active.as_ptr(),
            jacobians: std::ptr::null_mut(),
        };
        currents[0] = -1.0;
        variables[0] = 0.0;
        assert!(native.run_evaluation_kernel(&context, variables.as_mut_ptr(), &evaluation_io,));
        assert_eq!(variables[0], 2.0);
        assert_eq!(currents, [8.0]);

        let mut jacobian_values = [-1.0_f64];
        let stamp_io = NativeStampKernelIo {
            program_active: active.as_ptr(),
            jacobians: jacobian_values.as_mut_ptr(),
        };
        currents[0] = -1.0;
        variables[0] = 0.0;
        assert!(native.run_stamp_kernel(&context, variables.as_mut_ptr(), &stamp_io));
        assert_eq!(variables[0], 2.0);
        assert_eq!(currents, [8.0]);
        assert_eq!(jacobian_values, [5.0]);
        assert!(native.evaluation_kernel_is_eligible());
        assert!(native.stamp_kernel_is_eligible());
        assert_eq!(native.plan_stats().total_entry_points(), 11);
        assert!(context.take_runtime_error().is_none());
        Ok(())
    }
}
