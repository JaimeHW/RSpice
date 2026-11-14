//! Function bodies recorded before encoding, and the local allocation that
//! keeps a value entry inside the browsers' locals limit.
//!
//! The SSA encoder gives every value its own local, which is what makes its
//! edge copies need no sequencing. A large model's assignment pass has tens of
//! thousands of values, though, and every browser engine refuses a function
//! declaring more than [`MAX_FUNCTION_LOCALS`]. Almost none of those values
//! are live at once, so a value entry is recorded first and its locals are
//! then shared by values whose live ranges do not meet: the encoded body reads
//! and writes the same values in the same order, through far fewer locals.
//!
//! The allocation reads the recorded instructions, not the SSA, so it holds
//! for whatever the encoder emits. Execution through structured WebAssembly
//! only ever moves forward through the instruction sequence, except through
//! a `loop`'s back edge; so a local's live range is the span from its first to
//! its last access, widened to the whole of any `loop` it is live across.
//! Every other read is dominated by a write on every path, because the SSA it
//! came from is; a local whose first access is a read depends on its zero
//! initialization instead, and keeps a local of its own.
//!
//! Sharing alone bounds a body by the values live at once, not by the limit.
//! A body that would still declare more than the limit keeps its most used
//! locals and moves the rest to mutable globals, one access for one access.
//! Value entries call only the capability imports and kernels call them one
//! after another, so no two of them are ever active together and every body
//! draws on the same globals ([`SpillGlobals`]). A spilled local that relied
//! on zero initialization is zeroed on entry, the way a local would be.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use wasm_encoder::{ConstExpr, Function, GlobalSection, GlobalType, Instruction, ValType};

/// The most locals one function may declare: the WebAssembly JavaScript API's
/// implementation limit (<https://webassembly.github.io/spec/js-api/#limits>),
/// which V8, SpiderMonkey and JavaScriptCore enforce and the module validator
/// checks before a module leaves the compiler. Parameters count against it.
pub(super) const MAX_FUNCTION_LOCALS: u32 = 50_000;

/// The mutable globals a module's value entries spill to, in declaration
/// order. Each body asks for its `n`th global of a type and gets the same one
/// every other body gets for that ordinal, so the module declares as many of
/// each type as the most demanding body needs.
#[derive(Debug, Default)]
pub(super) struct SpillGlobals {
    types: Vec<ValType>,
    by_type: Vec<(ValType, Vec<u32>)>,
}

impl SpillGlobals {
    /// The globals to declare, in index order. Empty unless a body spilled.
    #[cfg(test)]
    pub(super) fn types(&self) -> &[ValType] {
        &self.types
    }

    /// The section declaring these globals, each mutable and starting at
    /// zero; `None` when no body spilled.
    pub(super) fn section(&self) -> Option<GlobalSection> {
        if self.types.is_empty() {
            return None;
        }
        let mut section = GlobalSection::new();
        for ty in &self.types {
            let init = match ty {
                ValType::I32 => ConstExpr::i32_const(0),
                ValType::I64 => ConstExpr::i64_const(0),
                ValType::F32 => ConstExpr::f32_const(0.0.into()),
                ValType::F64 => ConstExpr::f64_const(0.0.into()),
                _ => unreachable!("only a type with a zero is spilled"),
            };
            section.global(
                GlobalType {
                    val_type: *ty,
                    mutable: true,
                    shared: false,
                },
                &init,
            );
        }
        Some(section)
    }

    fn global(&mut self, ty: ValType, ordinal: usize) -> u32 {
        let position = match self.by_type.iter().position(|(seen, _)| *seen == ty) {
            Some(position) => position,
            None => {
                self.by_type.push((ty, Vec::new()));
                self.by_type.len() - 1
            }
        };
        while self.by_type[position].1.len() <= ordinal {
            let index = self.types.len() as u32;
            self.types.push(ty);
            self.by_type[position].1.push(index);
        }
        self.by_type[position].1[ordinal]
    }
}

/// One function body as instructions, before it is encoded.
#[derive(Debug, Clone)]
pub(super) struct WasmBody {
    /// Parameters precede the declared locals in the index space.
    params: u32,
    locals: Vec<(u32, ValType)>,
    code: Vec<Instruction<'static>>,
}

impl WasmBody {
    /// A body taking the single frame-offset parameter every entry takes.
    pub(super) fn new(locals: impl IntoIterator<Item = (u32, ValType)>) -> Self {
        Self::with_params(1, locals)
    }

    pub(super) fn with_params(
        params: u32,
        locals: impl IntoIterator<Item = (u32, ValType)>,
    ) -> Self {
        Self {
            params,
            locals: locals
                .into_iter()
                .filter(|(count, _)| *count != 0)
                .collect(),
            code: Vec::new(),
        }
    }

    pub(super) fn instruction(&mut self, instruction: &Instruction<'static>) -> &mut Self {
        self.code.push(instruction.clone());
        self
    }

    /// The body with the locals it declared.
    pub(super) fn into_function(self) -> Function {
        encode(&self.locals, &self.code)
    }

    /// The body with its locals shared between values whose live ranges do
    /// not meet, and spilled to `globals` past `limit` locals, parameters
    /// included ([`MAX_FUNCTION_LOCALS`] outside tests). See the module
    /// documentation.
    pub(super) fn into_allocated_function(
        self,
        limit: u32,
        globals: &mut SpillGlobals,
    ) -> Function {
        let (locals, code) = allocate_locals(self.params, &self.locals, self.code, limit, globals);
        encode(&locals, &code)
    }

    /// The locals the body declares after allocation, parameters excluded.
    #[cfg(test)]
    pub(super) fn allocated_local_count(&self) -> u32 {
        let (locals, _) = allocate_locals(
            self.params,
            &self.locals,
            self.code.clone(),
            MAX_FUNCTION_LOCALS,
            &mut SpillGlobals::default(),
        );
        locals.iter().map(|(count, _)| count).sum()
    }
}

fn encode(locals: &[(u32, ValType)], code: &[Instruction<'static>]) -> Function {
    let mut function = Function::new(locals.iter().copied());
    for instruction in code {
        function.instruction(instruction);
    }
    function
}

/// The instruction that pushes a zero of `ty`, for the types a global can be
/// spilled to and zeroed as.
fn zero(ty: ValType) -> Option<Instruction<'static>> {
    match ty {
        ValType::I32 => Some(Instruction::I32Const(0)),
        ValType::I64 => Some(Instruction::I64Const(0)),
        ValType::F32 => Some(Instruction::F32Const(0.0.into())),
        ValType::F64 => Some(Instruction::F64Const(0.0.into())),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
enum Storage {
    Local(u32),
    Global(u32),
}

/// Share declared locals between values whose live ranges are disjoint,
/// spill past `limit` (parameters included) to `globals`, rewrite every
/// access, and return the new declaration with the rewritten code.
fn allocate_locals(
    params: u32,
    locals: &[(u32, ValType)],
    mut code: Vec<Instruction<'static>>,
    limit: u32,
    globals: &mut SpillGlobals,
) -> (Vec<(u32, ValType)>, Vec<Instruction<'static>>) {
    const UNUSED: usize = usize::MAX;
    let declared: Vec<ValType> = locals
        .iter()
        .flat_map(|(count, ty)| std::iter::repeat_n(*ty, *count as usize))
        .collect();
    let mut first = vec![UNUSED; declared.len()];
    let mut last = vec![0_usize; declared.len()];
    let mut pinned = vec![false; declared.len()];
    let mut loops = Vec::new();
    let mut open = Vec::new();
    for (position, instruction) in code.iter().enumerate() {
        match instruction {
            Instruction::LocalGet(index)
            | Instruction::LocalSet(index)
            | Instruction::LocalTee(index)
                if *index >= params =>
            {
                let local = (*index - params) as usize;
                if first[local] == UNUSED {
                    first[local] = position;
                    pinned[local] = matches!(instruction, Instruction::LocalGet(_));
                }
                last[local] = position;
            }
            Instruction::Block(_) | Instruction::If(_) => open.push(None),
            Instruction::Loop(_) => open.push(Some(position)),
            Instruction::End => {
                if let Some(Some(start)) = open.pop() {
                    loops.push((start, position));
                }
            }
            _ => {}
        }
    }
    // A value live into a loop stays live to its end, since the back edge
    // reads it again; one defined inside and read after it is live from the
    // loop's start. Widening for one loop can carry a range into another.
    let mut widened = !loops.is_empty();
    while widened {
        widened = false;
        for &(start, end) in &loops {
            for local in 0..declared.len() {
                if first[local] == UNUSED {
                    continue;
                }
                if first[local] < start && last[local] >= start && last[local] < end {
                    last[local] = end;
                    widened = true;
                }
                if first[local] > start && first[local] <= end && last[local] > end {
                    first[local] = start;
                    widened = true;
                }
            }
        }
    }

    // One pool of locals per value type, in the order the declaration first
    // names each type; the lowest free local is reused first.
    struct Pool {
        ty: ValType,
        count: u32,
        free: BinaryHeap<Reverse<u32>>,
        active: BinaryHeap<Reverse<(usize, u32)>>,
        pinned: Vec<bool>,
    }
    let mut pools: Vec<Pool> = Vec::new();
    let mut order: Vec<usize> = (0..declared.len())
        .filter(|&local| first[local] != UNUSED)
        .collect();
    order.sort_by_key(|&local| (first[local], local));
    let mut assigned = vec![(0_usize, 0_u32); declared.len()];
    for local in order {
        let ty = declared[local];
        let pool_index = match pools.iter().position(|pool| pool.ty == ty) {
            Some(index) => index,
            None => {
                pools.push(Pool {
                    ty,
                    count: 0,
                    free: BinaryHeap::new(),
                    active: BinaryHeap::new(),
                    pinned: Vec::new(),
                });
                pools.len() - 1
            }
        };
        let pool = &mut pools[pool_index];
        while let Some(&Reverse((end, slot))) = pool.active.peek() {
            if end >= first[local] {
                break;
            }
            pool.active.pop();
            pool.free.push(Reverse(slot));
        }
        let slot = match (pinned[local], pool.free.pop()) {
            (false, Some(Reverse(slot))) => slot,
            (true, Some(free)) => {
                pool.free.push(free);
                pool.count += 1;
                pool.count - 1
            }
            (_, None) => {
                pool.count += 1;
                pool.count - 1
            }
        };
        pool.pinned.resize(pool.count as usize, false);
        if pinned[local] {
            pool.pinned[slot as usize] = true;
        } else {
            pool.active.push(Reverse((last[local], slot)));
        }
        assigned[local] = (pool_index, slot);
    }

    let budget = limit.saturating_sub(params);
    let total: u64 = pools.iter().map(|pool| u64::from(pool.count)).sum();
    let mut declaration = Vec::with_capacity(pools.len());
    let mut storage: Vec<Vec<Storage>> = Vec::with_capacity(pools.len());
    let mut zeroed = Vec::new();
    if total <= u64::from(budget) {
        let mut next = params;
        for pool in &pools {
            storage.push(
                (0..pool.count)
                    .map(|slot| Storage::Local(next + slot))
                    .collect(),
            );
            declaration.push((pool.count, pool.ty));
            next += pool.count;
        }
    } else {
        // Keep the most used locals; a type with no zero to spill as stays.
        let mut uses: Vec<Vec<u64>> = pools
            .iter()
            .map(|pool| vec![0; pool.count as usize])
            .collect();
        for instruction in &code {
            if let Instruction::LocalGet(index)
            | Instruction::LocalSet(index)
            | Instruction::LocalTee(index) = instruction
                && *index >= params
            {
                let (pool, slot) = assigned[(*index - params) as usize];
                uses[pool][slot as usize] += 1;
            }
        }
        let mut keep: Vec<Vec<bool>> = pools
            .iter()
            .map(|pool| vec![zero(pool.ty).is_none(); pool.count as usize])
            .collect();
        let fixed = keep.iter().flatten().filter(|kept| **kept).count();
        let room = (budget as usize).saturating_sub(fixed);
        let mut ranked: Vec<(usize, u32)> = pools
            .iter()
            .enumerate()
            .filter(|(_, pool)| zero(pool.ty).is_some())
            .flat_map(|(index, pool)| (0..pool.count).map(move |slot| (index, slot)))
            .collect();
        ranked.sort_by_key(|&(pool, slot)| (Reverse(uses[pool][slot as usize]), pool, slot));
        for &(pool, slot) in ranked.iter().take(room) {
            keep[pool][slot as usize] = true;
        }
        let mut next = params;
        for (index, pool) in pools.iter().enumerate() {
            let mut kept = 0;
            let mut spilled = 0;
            let mut slots = Vec::with_capacity(pool.count as usize);
            for (&kept_here, &pinned) in keep[index].iter().zip(&pool.pinned) {
                if kept_here {
                    slots.push(Storage::Local(next + kept));
                    kept += 1;
                } else {
                    let global = globals.global(pool.ty, spilled);
                    spilled += 1;
                    if pinned {
                        zeroed.push((global, pool.ty));
                    }
                    slots.push(Storage::Global(global));
                }
            }
            storage.push(slots);
            if kept != 0 {
                declaration.push((kept, pool.ty));
            }
            next += kept;
        }
    }

    let spills = !zeroed.is_empty()
        || storage
            .iter()
            .flatten()
            .any(|storage| matches!(storage, Storage::Global(_)));
    let at = |index: u32| {
        storage[assigned[(index - params) as usize].0]
            [assigned[(index - params) as usize].1 as usize]
    };
    if !spills {
        for instruction in code.iter_mut() {
            if let Instruction::LocalGet(index)
            | Instruction::LocalSet(index)
            | Instruction::LocalTee(index) = instruction
                && *index >= params
                && let Storage::Local(local) = at(*index)
            {
                *index = local;
            }
        }
        return (declaration, code);
    }
    let mut rewritten = Vec::with_capacity(code.len() + 2 * zeroed.len());
    for (global, ty) in zeroed {
        rewritten.extend(zero(ty));
        rewritten.push(Instruction::GlobalSet(global));
    }
    for instruction in code {
        let storage = match &instruction {
            Instruction::LocalGet(index)
            | Instruction::LocalSet(index)
            | Instruction::LocalTee(index)
                if *index >= params =>
            {
                at(*index)
            }
            _ => {
                rewritten.push(instruction);
                continue;
            }
        };
        match (instruction, storage) {
            (Instruction::LocalGet(_), Storage::Local(local)) => {
                rewritten.push(Instruction::LocalGet(local));
            }
            (Instruction::LocalSet(_), Storage::Local(local)) => {
                rewritten.push(Instruction::LocalSet(local));
            }
            (_, Storage::Local(local)) => rewritten.push(Instruction::LocalTee(local)),
            (Instruction::LocalGet(_), Storage::Global(global)) => {
                rewritten.push(Instruction::GlobalGet(global));
            }
            (Instruction::LocalSet(_), Storage::Global(global)) => {
                rewritten.push(Instruction::GlobalSet(global));
            }
            (_, Storage::Global(global)) => {
                rewritten.push(Instruction::GlobalSet(global));
                rewritten.push(Instruction::GlobalGet(global));
            }
        }
    }
    (declaration, rewritten)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_encoder::BlockType;

    fn body(locals: u32, code: &[Instruction<'static>]) -> WasmBody {
        let mut body = WasmBody::new([(locals, ValType::F64)]);
        for instruction in code {
            body.instruction(instruction);
        }
        body
    }

    fn rewritten(body: &WasmBody) -> Vec<Instruction<'static>> {
        rewritten_within(body, MAX_FUNCTION_LOCALS, &mut SpillGlobals::default()).1
    }

    fn rewritten_within(
        body: &WasmBody,
        limit: u32,
        globals: &mut SpillGlobals,
    ) -> (Vec<(u32, ValType)>, Vec<Instruction<'static>>) {
        allocate_locals(body.params, &body.locals, body.code.clone(), limit, globals)
    }

    fn locals_of(code: &[Instruction<'static>]) -> Vec<u32> {
        code.iter()
            .filter_map(|instruction| match instruction {
                Instruction::LocalGet(index)
                | Instruction::LocalSet(index)
                | Instruction::LocalTee(index) => Some(*index),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_chain_of_short_lived_values_shares_one_pair_of_locals() {
        use Instruction::*;
        // v1 = 1; v2 = v1 + 1; v3 = v2 + 1; ... each value dies at the next.
        let mut code = vec![F64Const(1.0.into()), LocalSet(1)];
        for value in 2..=100 {
            code.extend([
                LocalGet(value - 1),
                F64Const(1.0.into()),
                F64Add,
                LocalSet(value),
            ]);
        }
        code.push(End);
        let body = body(100, &code);
        assert_eq!(body.allocated_local_count(), 1);
    }

    #[test]
    fn values_live_at_once_keep_distinct_locals_in_an_edge_copy() {
        use Instruction::*;
        // p1, p2 = a1, a2 as the encoder sequences a forward edge: a2 is still
        // live when p1 is written, so the two may not share a local.
        let code = [
            F64Const(1.0.into()),
            LocalSet(1),
            F64Const(2.0.into()),
            LocalSet(2),
            LocalGet(1),
            LocalSet(3),
            LocalGet(2),
            LocalSet(4),
            LocalGet(3),
            LocalGet(4),
            F64Sub,
            Drop,
            End,
        ];
        let code = rewritten(&body(4, &code));
        let accessed = locals_of(&code);
        // a2 (second set) and p1 (third set) are distinct.
        assert_ne!(accessed[1], accessed[3]);
        // p1 and p2 are both read at the end, so they are distinct too.
        assert_ne!(accessed[6], accessed[7]);
    }

    #[test]
    fn a_value_live_into_a_loop_is_not_reused_inside_it() {
        use Instruction::*;
        // v1 is set before the loop and read at its top on every trip; v2 is
        // set and dies inside the body, after v1's last textual read. Sharing
        // v1's local with v2 would clobber v1 for the next trip.
        let code = [
            F64Const(1.0.into()),
            LocalSet(1),
            Loop(BlockType::Empty),
            LocalGet(1),
            Drop,
            F64Const(2.0.into()),
            LocalSet(2),
            LocalGet(2),
            Drop,
            I32Const(0),
            BrIf(0),
            End,
            End,
        ];
        let code = rewritten(&body(2, &code));
        let accessed = locals_of(&code);
        assert_ne!(accessed[0], accessed[2]);
    }

    #[test]
    fn a_local_read_before_it_is_written_keeps_its_own_local() {
        use Instruction::*;
        let code = [
            F64Const(1.0.into()),
            LocalSet(1),
            LocalGet(1),
            Drop,
            LocalGet(2),
            Drop,
            End,
        ];
        let code = rewritten(&body(2, &code));
        let accessed = locals_of(&code);
        assert_ne!(accessed[0], accessed[2]);
    }

    #[test]
    fn locals_of_different_types_never_share() {
        use Instruction::*;
        let mut body = WasmBody::new([(1, ValType::F64), (1, ValType::I32)]);
        for instruction in [
            F64Const(1.0.into()),
            LocalSet(1),
            LocalGet(1),
            Drop,
            I32Const(1),
            LocalSet(2),
            LocalGet(2),
            Drop,
            End,
        ] {
            body.instruction(&instruction);
        }
        let code = rewritten(&body);
        let accessed = locals_of(&code);
        assert_ne!(accessed[0], accessed[2]);
        assert_eq!(body.allocated_local_count(), 2);
    }

    /// Run a body taking no parameters and returning one `f64` in an
    /// independent engine, `calls` times, with locals allowed up to `limit`.
    fn run_within(body: &WasmBody, limit: u32, calls: usize) -> (Vec<f64>, Vec<ValType>) {
        use wasm_encoder::{
            CodeSection, ExportKind, ExportSection, FunctionSection, Module, TypeSection,
        };
        let mut globals = SpillGlobals::default();
        let (locals, code) = rewritten_within(body, limit, &mut globals);
        let mut module = Module::new();
        let mut types = TypeSection::new();
        types.ty().function([], [ValType::F64]);
        module.section(&types);
        let mut functions = FunctionSection::new();
        functions.function(0);
        module.section(&functions);
        if let Some(section) = globals.section() {
            module.section(&section);
        }
        let mut exports = ExportSection::new();
        exports.export("f", ExportKind::Func, 0);
        module.section(&exports);
        let mut bodies = CodeSection::new();
        bodies.function(&encode(&locals, &code));
        module.section(&bodies);
        let bytes = module.finish();
        wasmparser::Validator::new()
            .validate_all(&bytes)
            .expect("spilled body validates");
        let engine = wasmi::Engine::default();
        let module = wasmi::Module::new(&engine, &bytes[..]).expect("compile spilled body");
        let mut store = wasmi::Store::new(&engine, ());
        let instance = wasmi::Linker::<()>::new(&engine)
            .instantiate_and_start(&mut store, &module)
            .expect("instantiate spilled body");
        let f = instance
            .get_typed_func::<(), f64>(&store, "f")
            .expect("export f");
        let results = (0..calls)
            .map(|_| f.call(&mut store, ()).expect("run spilled body"))
            .collect();
        (results, globals.types().to_vec())
    }

    fn parameterless(locals: &[(u32, ValType)], code: &[Instruction<'static>]) -> WasmBody {
        let mut body = WasmBody::with_params(0, locals.iter().copied());
        for instruction in code {
            body.instruction(instruction);
        }
        body
    }

    #[test]
    fn a_body_past_the_limit_spills_its_least_used_locals_and_computes_the_same() {
        use Instruction::*;
        // (1 + 2) * (3 + 4) through a tee, scaled by the first value read
        // four more times: the busiest local, so the one kept.
        let code = [
            F64Const(1.0.into()),
            LocalSet(0),
            F64Const(2.0.into()),
            LocalSet(1),
            LocalGet(0),
            LocalGet(1),
            F64Add,
            LocalTee(2),
            Drop,
            F64Const(3.0.into()),
            LocalSet(3),
            F64Const(4.0.into()),
            LocalSet(4),
            LocalGet(3),
            LocalGet(4),
            F64Add,
            LocalSet(5),
            LocalGet(2),
            LocalGet(5),
            F64Mul,
            LocalGet(0),
            F64Mul,
            LocalGet(0),
            F64Mul,
            LocalGet(0),
            F64Mul,
            End,
        ];
        let body = parameterless(&[(6, ValType::F64)], &code);
        let (unbounded, unspilled) = run_within(&body, MAX_FUNCTION_LOCALS, 1);
        assert!(unspilled.is_empty());
        let (spilled, globals) = run_within(&body, 1, 2);
        assert_eq!(unbounded, vec![21.0]);
        assert_eq!(spilled, vec![21.0, 21.0]);
        assert!(!globals.is_empty());
        assert!(globals.iter().all(|ty| *ty == ValType::F64));
        let (locals, code) = rewritten_within(&body, 1, &mut SpillGlobals::default());
        assert_eq!(locals, vec![(1, ValType::F64)]);
        // The kept local is the first value's: every read of it is still a
        // local read.
        assert_eq!(
            code.iter()
                .filter(|instruction| matches!(instruction, LocalGet(0)))
                .count(),
            4
        );
    }

    #[test]
    fn a_spilled_local_read_before_it_is_written_starts_every_call_at_zero() {
        use Instruction::*;
        // Return what v0 held on entry, then write 7 to it: a local reads 0 on
        // every call, so the global it spills to must too.
        let code = [
            LocalGet(0),
            LocalSet(1),
            F64Const(7.0.into()),
            LocalSet(0),
            LocalGet(1),
            End,
        ];
        let body = parameterless(&[(2, ValType::F64)], &code);
        let (results, globals) = run_within(&body, 0, 3);
        assert_eq!(results, vec![0.0, 0.0, 0.0]);
        assert_eq!(globals.len(), 2);
    }

    #[test]
    fn bodies_share_spill_globals_by_type_and_ordinal() {
        let mut globals = SpillGlobals::default();
        assert_eq!(globals.global(ValType::F64, 0), 0);
        assert_eq!(globals.global(ValType::I32, 0), 1);
        assert_eq!(globals.global(ValType::F64, 2), 3);
        assert_eq!(globals.global(ValType::F64, 1), 2);
        assert_eq!(globals.global(ValType::I32, 0), 1);
        assert_eq!(
            globals.types(),
            &[ValType::F64, ValType::I32, ValType::F64, ValType::F64]
        );
    }
}
