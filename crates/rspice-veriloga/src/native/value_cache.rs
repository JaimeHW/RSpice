//! Architecture-neutral exact cache for value-entry machine code.
//!
//! Compact-model Jacobian tables often contain the same expression in more
//! than one logical slot. The runtime keeps those slots distinct, but their
//! immutable value-entry ABI is identical, so both native backends can safely
//! point them at one authenticated function body.

#![cfg_attr(not(feature = "native"), allow(dead_code))]

use super::expr::NativeOp;
use super::plan_program::PlanProgramRef;
use super::ssa;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::hash::{DefaultHasher, Hash, Hasher};

/// The key one cached entry was published under.
///
/// Kept in the two forms a plan entry can take rather than in one normalized
/// form, because normalizing would mean lifting every postfix program into
/// blocks just to look it up, and because a postfix entry and a block entry are
/// never interchangeable anyway: they reach different emitters.
#[derive(Debug)]
enum CachedKey {
    Postfix(Box<[NativeOp]>),
    Blocks(ssa::Program),
}

impl CachedKey {
    fn matches(&self, program: PlanProgramRef<'_>) -> bool {
        match (self, program) {
            (Self::Postfix(ops), PlanProgramRef::Postfix(program)) => {
                ops_are_codegen_identical(ops, program.ops())
            }
            (Self::Blocks(cached), PlanProgramRef::Blocks(program)) => {
                cached.is_codegen_identical_to(program.ssa())
            }
            _ => false,
        }
    }
}

#[derive(Debug)]
struct CachedValueEntry<ArtifactId> {
    key: CachedKey,
    artifact_id: ArtifactId,
}

#[derive(Debug)]
pub(crate) struct ValueEntryCache<ArtifactId> {
    buckets: HashMap<u64, Vec<CachedValueEntry<ArtifactId>>>,
}

impl<ArtifactId> Default for ValueEntryCache<ArtifactId> {
    fn default() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }
}

impl<ArtifactId: Copy> ValueEntryCache<ArtifactId> {
    pub(crate) fn lookup(&self, program: PlanProgramRef<'_>) -> Option<ArtifactId> {
        self.buckets
            .get(&program_hash(program))?
            .iter()
            .find(|entry| entry.key.matches(program))
            .map(|entry| entry.artifact_id)
    }

    pub(crate) fn insert(&mut self, program: PlanProgramRef<'_>, artifact_id: ArtifactId) {
        self.buckets
            .entry(program_hash(program))
            .or_default()
            .push(CachedValueEntry {
                key: match program {
                    PlanProgramRef::Postfix(program) => CachedKey::Postfix(program.ops().into()),
                    PlanProgramRef::Blocks(program) => CachedKey::Blocks(program.ssa().clone()),
                },
                artifact_id,
            });
    }
}

/// The bucket key for a plan entry.
///
/// A postfix entry hashes exactly as it always has — its operation sequence,
/// nothing prepended — so the shipped corpus buckets identically to before the
/// cache learned about blocks.
///
/// A block entry cannot be keyed that way. Its operations are not the whole of
/// it: two block programs can carry the same instructions in the same order and
/// still compile differently because a terminator sends control elsewhere or a
/// merge binds different arguments. So it is keyed over its blocks in layout
/// order, each block's parameters, its instructions and its terminator, and it
/// is tagged so a block entry can never share a bucket with a postfix one.
///
/// Bucketing is never load-bearing on its own: two entries this hash separates
/// are different, and membership within a bucket is decided by
/// [`CachedKey::matches`].
fn program_hash(program: PlanProgramRef<'_>) -> u64 {
    let mut hasher = DefaultHasher::new();
    match program {
        PlanProgramRef::Postfix(program) => {
            for op in program.ops() {
                hash_native_op(*op, &mut hasher);
            }
        }
        PlanProgramRef::Blocks(program) => {
            hasher.write(b"rspice-plan-block-program\0");
            program.ssa().codegen_identity_hash(&mut hasher);
        }
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

pub(crate) fn native_op_hash(op: NativeOp, operands: impl IntoIterator<Item = usize>) -> u64 {
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

pub(crate) fn native_ops_are_codegen_identical(left: NativeOp, right: NativeOp) -> bool {
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
    use crate::jit::expr::{NativeOp, NativeProgram};
    use crate::jit::plan_program::{BlockProgram, PlanProgramRef};
    use crate::jit::ssa;

    fn program(value: f64) -> NativeProgram {
        NativeProgram::from_ops_for_test(vec![NativeOp::Const(value)], 1, Vec::new(), Vec::new())
    }

    #[test]
    fn reuses_only_bit_identical_programs() {
        let mut cache = ValueEntryCache::default();
        let positive_zero = program(0.0);
        let negative_zero = program(-0.0);
        cache.insert(PlanProgramRef::Postfix(&positive_zero), 16_usize);
        assert_eq!(
            cache.lookup(PlanProgramRef::Postfix(&positive_zero)),
            Some(16_usize)
        );
        assert_eq!(cache.lookup(PlanProgramRef::Postfix(&negative_zero)), None);

        let nan = program(f64::from_bits(0x7ff8_0000_0000_0042));
        let same_nan = program(f64::from_bits(0x7ff8_0000_0000_0042));
        let other_nan = program(f64::from_bits(0x7ff8_0000_0000_0043));
        cache.insert(PlanProgramRef::Postfix(&nan), 32_usize);
        assert_eq!(
            cache.lookup(PlanProgramRef::Postfix(&same_nan)),
            Some(32_usize)
        );
        assert_eq!(cache.lookup(PlanProgramRef::Postfix(&other_nan)), None);
    }

    /// The reason a block program needs a key of its own.
    ///
    /// Two branch-form programs built from the same conditional carry the same
    /// instructions; what tells them apart is where the terminators send
    /// control. Keying a block program on its operations alone would publish
    /// one body for both and the second entry would evaluate the wrong arm.
    #[test]
    fn a_block_entry_deduplicates_on_structure_not_on_operations_alone() {
        fn branched(then_value: f64, else_value: f64) -> BlockProgram {
            let source = NativeProgram::from_ops_for_test(
                vec![
                    NativeOp::LoadParam(0),
                    NativeOp::Const(then_value),
                    NativeOp::Const(else_value),
                    NativeOp::IfElse,
                ],
                3,
                Vec::new(),
                Vec::new(),
            );
            let split = ssa::Program::lower(&source)
                .expect("lift the postfix")
                .with_branching_conditionals()
                .expect("split the conditional");
            assert!(!split.is_single_block(), "the fixture has to branch");
            BlockProgram::adopt_unrooted(split)
        }

        let mut cache = ValueEntryCache::default();
        let first = branched(1.0, 2.0);
        let same = branched(1.0, 2.0);
        cache.insert(PlanProgramRef::Blocks(&first), 8_usize);
        assert_eq!(
            cache.lookup(PlanProgramRef::Blocks(&same)),
            Some(8_usize),
            "two structurally identical block programs share one body"
        );

        // Same instruction multiset, different control flow: the arms are
        // swapped, so the terminators differ and nothing else does.
        let swapped = branched(2.0, 1.0);
        assert_eq!(
            cache.lookup(PlanProgramRef::Blocks(&swapped)),
            None,
            "programs whose blocks differ must not share a body"
        );
    }

    /// A block program and a postfix program never share a body, however alike
    /// their operations look: they reach different emitters.
    #[test]
    fn block_and_postfix_entries_are_separate_key_spaces() {
        let flat = program(4.0);
        let blocks =
            BlockProgram::adopt_unrooted(ssa::Program::lower(&flat).expect("lift the postfix"));

        let mut cache = ValueEntryCache::default();
        cache.insert(PlanProgramRef::Postfix(&flat), 1_usize);
        assert_eq!(cache.lookup(PlanProgramRef::Blocks(&blocks)), None);

        cache.insert(PlanProgramRef::Blocks(&blocks), 2_usize);
        assert_eq!(
            cache.lookup(PlanProgramRef::Postfix(&flat)),
            Some(1_usize),
            "the postfix entry keeps its own body"
        );
        assert_eq!(cache.lookup(PlanProgramRef::Blocks(&blocks)), Some(2_usize));
    }

    /// Bit identity is the rule for a block program too: a constant that
    /// differs only in its sign bit is a different program.
    #[test]
    fn a_block_entry_keeps_zeroes_of_opposite_sign_apart() {
        fn blocks(value: f64) -> BlockProgram {
            let source = NativeProgram::from_ops_for_test(
                vec![NativeOp::Const(value)],
                1,
                Vec::new(),
                Vec::new(),
            );
            BlockProgram::adopt_unrooted(ssa::Program::lower(&source).expect("lift the postfix"))
        }

        let mut cache = ValueEntryCache::default();
        let positive_zero = blocks(0.0);
        let negative_zero = blocks(-0.0);
        cache.insert(PlanProgramRef::Blocks(&positive_zero), 64_usize);
        assert_eq!(
            cache.lookup(PlanProgramRef::Blocks(&positive_zero)),
            Some(64_usize)
        );
        assert_eq!(cache.lookup(PlanProgramRef::Blocks(&negative_zero)), None);
    }
}
