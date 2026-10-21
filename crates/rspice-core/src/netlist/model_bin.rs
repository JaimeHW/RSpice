//! Geometry-bin bounds of a model card, and the containment rule that
//! selects a card for an instance's L, W and NFIN.
//!
//! Plain data and arithmetic on it. The engine evaluates a card's bounds in
//! the deck's parameter scope and reports what it selected; anything else that
//! reads the bounds applies the same containment test and tolerance from here,
//! without depending on the engine. `engine` re-exports every type.

/// One resolved axis of a geometry-bin card.
///
/// Bounds are expressed in the netlist's canonical SI domain. An absent pair
/// means that axis does not participate in selection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModelBinAxisRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl ModelBinAxisRange {
    /// Whether this range declares a genuine reversal under the same boundary
    /// tolerance used by model-bin validation.
    #[must_use]
    pub fn is_reversed(self) -> bool {
        self.min
            .zip(self.max)
            .is_some_and(|(min, max)| min > max && !bin_bound_equal(min, max))
    }
}

/// Fully evaluated geometry bounds for one model card.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModelBinCardGeometry {
    pub length: ModelBinAxisRange,
    pub width: ModelBinAxisRange,
    pub nfin: ModelBinAxisRange,
}

/// One source-ordered model card participating in geometry selection.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelBinCardInspection {
    pub model: String,
    pub family: String,
    pub model_type: String,
    pub declaration_order: usize,
    pub geometry: ModelBinCardGeometry,
}

/// Why one MOS instance selected its reported model card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelBinSelectionKind {
    /// The instance named the complete card directly.
    ExactCard,
    /// Exactly one family card contained the instance geometry.
    FamilyMatch,
    /// Multiple inclusive cards met only on a shared boundary; the card the
    /// geometry sits at the lower edge of was selected (`min <= x < max`).
    SharedBoundary,
}

/// Authoritative geometry-bin decision for one flattened MOS instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelBinInstanceInspection {
    pub element: String,
    pub requested_model: String,
    pub selected_model: String,
    pub selection: ModelBinSelectionKind,
    pub match_count: usize,
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub nfin: Option<f64>,
    pub multiplier: Option<f64>,
}

/// One flattened MOS instance whose geometry no card of its bin family covers,
/// which circuit construction refuses.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelBinCoverageGap {
    pub element: String,
    /// The family the instance names, as it names it.
    pub family: String,
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub nfin: Option<f64>,
    /// The family card whose bounds lie closest to the instance geometry,
    /// each axis's excess measured relative to the bound it passes.
    pub nearest: ModelBinCardInspection,
}

/// Complete successful model-bin inspection of one exact parsed netlist.
///
/// Construction fails instead of returning a partial receipt when any card is
/// malformed or any family selection is ambiguous/uncovered. Consequently a
/// retained receipt is positive evidence that every reported decision used the
/// same resolver as circuit construction.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModelBinInspection {
    pub cards: Vec<ModelBinCardInspection>,
    pub instances: Vec<ModelBinInstanceInspection>,
}

impl ModelBinCardGeometry {
    /// Whether an instance of this geometry selects this card, by the test
    /// circuit construction applies: every bounded axis inclusive at both
    /// ends within the bin tolerance, and an instance that omits a dimension
    /// the card bounds outside it.
    #[must_use]
    pub fn contains(self, length: Option<f64>, width: Option<f64>, nfin: Option<f64>) -> bool {
        bin_range_contains(length, self.length.min, self.length.max)
            && bin_range_contains(width, self.width.min, self.width.max)
            && bin_range_contains(nfin, self.nfin.min, self.nfin.max)
    }

    /// Whether two cards overlap over a positive region on every active
    /// geometry axis. Shared inclusive boundaries are not positive-area
    /// overlaps and remain deterministic by declaration order.
    pub fn overlaps_with_positive_area(self, other: Self) -> bool {
        bin_ranges_overlap_with_positive_area(self.length, other.length)
            && bin_ranges_overlap_with_positive_area(self.width, other.width)
            && bin_ranges_overlap_with_positive_area(self.nfin, other.nfin)
    }
}

/// Absolute slack on a bin boundary comparison, in metres.
///
/// ngspice's `is_equal` (inpgmod.c) treats two dimensions within 1 nm as the
/// same number before testing a bin, and the tolerance is doing real work: a
/// deck writing `W=0.22u` produces `0.22 × 1e-6`, which is one ULP *below* the
/// `wmin = 2.2e-007` on the model card. Compared exactly, such a device
/// matches no bin at all and the whole family fails to resolve.
const BIN_BOUND_TOLERANCE: f64 = 1e-9;

pub(crate) fn bin_bound_equal(left: f64, right: f64) -> bool {
    (left - right).abs() < BIN_BOUND_TOLERANCE
}

/// ngspice's `in_range`: **inclusive at both ends**, within the tolerance
/// above.
///
/// Inclusive at both ends means adjacent bins overlap on their shared
/// boundary — `L=0.5u` satisfies both `lmax=5e-7` and `lmin=5e-7` — which is
/// resolved by taking the first matching bin rather than by narrowing the
/// ranges. See `find_binned_model_def`.
fn bin_range_contains(value: Option<f64>, min: Option<f64>, max: Option<f64>) -> bool {
    if min.is_none() && max.is_none() {
        return true;
    }
    let Some(value) = value else {
        return false;
    };
    min.is_none_or(|min| value > min || bin_bound_equal(value, min))
        && max.is_none_or(|max| value < max || bin_bound_equal(value, max))
}

fn bin_ranges_overlap_with_positive_area(
    left: ModelBinAxisRange,
    right: ModelBinAxisRange,
) -> bool {
    let minimum = match (left.min, right.min) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (left, right) => left.or(right),
    };
    let maximum = match (left.max, right.max) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (left, right) => left.or(right),
    };
    match (minimum, maximum) {
        (Some(minimum), Some(maximum)) => maximum - minimum > BIN_BOUND_TOLERANCE,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::bin_range_contains;

    /// ngspice's `in_range` is inclusive at *both* ends, so adjacent bins
    /// overlap on their shared edge. `find_binned_model_def` resolves the
    /// overlap by taking the first matching bin, which is the lower one.
    #[test]
    fn model_bin_ranges_are_inclusive_at_both_ends() {
        assert!(bin_range_contains(Some(1.0), Some(1.0), Some(2.0)));
        assert!(bin_range_contains(Some(1.5), Some(1.0), Some(2.0)));
        assert!(bin_range_contains(Some(2.0), Some(1.0), Some(2.0)));
        assert!(!bin_range_contains(Some(0.5), Some(1.0), Some(2.0)));
        assert!(!bin_range_contains(Some(2.5), Some(1.0), Some(2.0)));
        assert!(bin_range_contains(Some(2.0), Some(1.0), None));
        assert!(bin_range_contains(Some(1.0), None, Some(2.0)));
        assert!(!bin_range_contains(None, Some(1.0), Some(2.0)));
    }

    /// A deck writes `W=0.22u`; the model card writes `wmin = 2.2e-007`. Those
    /// are not the same double — `0.22 × 1e-6` is one ULP low — so an exact
    /// comparison puts the device below the narrowest bin in the table and it
    /// resolves to no model at all. GF180MCU's minimum-width devices are
    /// exactly this case.
    #[test]
    fn bin_bounds_absorb_deck_round_off() {
        let drawn = 0.22 * 1e-6;
        let card = 2.2e-007;
        assert!(drawn < card, "the premise: mantissa × suffix lands low");
        assert!(bin_range_contains(Some(drawn), Some(card), Some(5e-7)));

        // The slack is 1 nm, not a licence to match a neighbouring bin: GF180's
        // narrowest W band is 280 nm wide, so nothing else comes close.
        assert!(!bin_range_contains(Some(2.2e-7), Some(5e-7), Some(1.2e-6)));
    }
}
