//! Architecture-neutral exact cache for value-entry machine code.
//!
//! Compact-model Jacobian tables often contain the same expression in more
//! than one logical slot. The runtime keeps those slots distinct, but their
//! immutable value-entry ABI is identical, so both native backends can safely
//! point them at one authenticated function body.

use super::expr::{NativeOp, NativeProgram};
use super::model::CodeOffset;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug)]
struct CachedValueEntry {
    ops: Box<[NativeOp]>,
    offset: CodeOffset,
}

#[derive(Debug, Default)]
pub(super) struct ValueEntryCache {
    buckets: HashMap<u64, Vec<CachedValueEntry>>,
}

impl ValueEntryCache {
    pub(super) fn lookup(&self, program: &NativeProgram) -> Option<CodeOffset> {
        self.buckets
            .get(&program_hash(program))?
            .iter()
            .find(|entry| ops_are_codegen_identical(&entry.ops, program.ops()))
            .map(|entry| entry.offset)
    }

    pub(super) fn insert(&mut self, program: &NativeProgram, offset: CodeOffset) {
        self.buckets
            .entry(program_hash(program))
            .or_default()
            .push(CachedValueEntry {
                ops: program.ops().into(),
                offset,
            });
    }
}

fn program_hash(program: &NativeProgram) -> u64 {
    let mut hasher = DefaultHasher::new();
    for op in program.ops() {
        hash_native_op(*op, &mut hasher);
    }
    hasher.finish()
}

fn ops_are_codegen_identical(left: &[NativeOp], right: &[NativeOp]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| native_ops_are_codegen_identical(*left, *right))
}

pub(super) fn native_op_hash(op: NativeOp, operands: impl IntoIterator<Item = usize>) -> u64 {
    let mut hasher = DefaultHasher::new();
    hash_native_op(op, &mut hasher);
    for operand in operands {
        operand.hash(&mut hasher);
    }
    hasher.finish()
}

fn hash_native_op(op: NativeOp, hasher: &mut DefaultHasher) {
    struct HashWriter<'a>(&'a mut DefaultHasher);

    impl std::fmt::Write for HashWriter<'_> {
        fn write_str(&mut self, value: &str) -> std::fmt::Result {
            self.0.write(value.as_bytes());
            Ok(())
        }
    }

    let mut writer = HashWriter(hasher);
    write!(&mut writer, "{op:?};").expect("hash writer cannot fail");
}

pub(super) fn native_ops_are_codegen_identical(left: NativeOp, right: NativeOp) -> bool {
    match (left, right) {
        (NativeOp::Const(left), NativeOp::Const(right))
        | (NativeOp::AddConst(left), NativeOp::AddConst(right))
        | (NativeOp::SubConst(left), NativeOp::SubConst(right))
        | (NativeOp::MulConst(left), NativeOp::MulConst(right))
        | (NativeOp::DivConst(left), NativeOp::DivConst(right))
        | (NativeOp::SubFromConst(left), NativeOp::SubFromConst(right))
        | (NativeOp::DivFromConst(left), NativeOp::DivFromConst(right)) => {
            left.to_bits() == right.to_bits()
        }
        (NativeOp::CompareConst(left_op, left), NativeOp::CompareConst(right_op, right)) => {
            left_op == right_op && left.to_bits() == right.to_bits()
        }
        (NativeOp::ExtremumConst(left_op, left), NativeOp::ExtremumConst(right_op, right))
        | (
            NativeOp::ExtremumConstLhs(left_op, left),
            NativeOp::ExtremumConstLhs(right_op, right),
        ) => left_op == right_op && left.to_bits() == right.to_bits(),
        _ => left == right,
    }
}

#[cfg(test)]
mod tests {
    use super::ValueEntryCache;
    use crate::native::expr::{NativeOp, NativeProgram};
    use crate::native::model::CodeOffset;

    fn program(value: f64) -> NativeProgram {
        NativeProgram::from_ops_for_test(vec![NativeOp::Const(value)], 1, Vec::new(), Vec::new())
    }

    #[test]
    fn reuses_only_bit_identical_programs() {
        let mut cache = ValueEntryCache::default();
        let positive_zero = program(0.0);
        let negative_zero = program(-0.0);
        cache.insert(&positive_zero, CodeOffset::new(16));
        assert_eq!(cache.lookup(&positive_zero), Some(CodeOffset::new(16)));
        assert_eq!(cache.lookup(&negative_zero), None);

        let nan = program(f64::from_bits(0x7ff8_0000_0000_0042));
        let same_nan = program(f64::from_bits(0x7ff8_0000_0000_0042));
        let other_nan = program(f64::from_bits(0x7ff8_0000_0000_0043));
        cache.insert(&nan, CodeOffset::new(32));
        assert_eq!(cache.lookup(&same_nan), Some(CodeOffset::new(32)));
        assert_eq!(cache.lookup(&other_nan), None);
    }
}
