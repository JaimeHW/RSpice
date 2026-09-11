//! Branch ownership shared by semantic analysis and backend lowering.

use smol_str::SmolStr;

/// A declared branch is independent of every other branch across the same
/// nodes. Only unnamed branches are identified by their unordered endpoints.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum BranchIdentity<N> {
    Named(SmolStr),
    Nodes(N, N),
}

impl<N: Ord> BranchIdentity<N> {
    pub(crate) fn new(name: Option<&SmolStr>, pos: N, neg: N) -> Self {
        match name {
            Some(name) => Self::Named(name.clone()),
            None if pos <= neg => Self::Nodes(pos, neg),
            None => Self::Nodes(neg, pos),
        }
    }
}
