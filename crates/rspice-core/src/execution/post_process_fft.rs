//! Authored transient `.FFT` post-processing, named by the canonical plan.
//!
//! The transient runner already evaluates every authored `.FFT` card, and the
//! canonical [`DeckPlan`] mints one `fft-NNN` identity per card. This module is
//! the single place that joins the two, and the single place that decides what
//! unit an `.FFT` coefficient carries, so no frontend has to pair a spectrum
//! with an identity — or invent a unit for it — on its own.
//!
//! It is the `.FFT` counterpart of [`super::post_process`], which does the same
//! for `.FOUR` operands.

use crate::abort_signal::AbortSignal;
use crate::engine::{SimulationError, TransientFftResult};
use crate::netlist::FftFormat;

use super::plan::{AnalysisInstanceId, DeckPlan, PostProcessSource};
use super::schema::SignalUnit;

/// One authored `.FFT` card's spectrum, bound to the identity the plan minted
/// for it and carrying the unit its coefficients are measured in.
///
/// The spectrum itself is borrowed from the transient result rather than
/// cloned: a `.FFT` bundle is the largest thing a transient publishes beside
/// its own trajectory, and copying it to hand it to a document builder would
/// double the peak footprint of a run for no gain.
#[derive(Debug, Clone, PartialEq)]
pub struct PlannedFftSpectrum<'result> {
    /// Canonical identity of this spectrum, such as `fft-001`.
    pub analysis: AnalysisInstanceId,
    /// The transient this spectrum was computed from.
    pub parent: AnalysisInstanceId,
    /// Unit of the complex coefficients and their magnitudes.
    pub output_unit: SignalUnit,
    /// The spectrum itself, exactly as the transient runner produced it.
    pub result: &'result TransientFftResult,
}

/// Declared unit of one `.FFT` spectrum's coefficients.
///
/// A normalized spectrum is each bin divided by the calibrated fundamental, so
/// it really is a pure ratio. An unnormalized spectrum keeps the probed
/// column's own unit; a braced parameter expression has a unit the deck never
/// declared, which is [`SignalUnit::Unspecified`] and not dimensionless —
/// exactly the answer `super::post_process::transient_output_unit` gives the
/// same column for `.FOUR`.
pub fn transient_fft_output_unit(
    physical_type: &str,
    format: FftFormat,
) -> Result<SignalUnit, SimulationError> {
    if matches!(format, FftFormat::Normalized) {
        return Ok(SignalUnit::Dimensionless);
    }
    super::post_process::transient_output_unit(physical_type)
}

/// Bind every `.FFT` spectrum one transient produced to the identity the
/// canonical plan minted for it.
///
/// `parent` is the transient instance `results` came from; only the plan's
/// `.FFT` cards bound to that transient are paired. A deck with two `.TRAN`
/// cards binds its `.FFT` cards to one of them, and the transient runner
/// evaluates the cards on every trajectory it solves, so the transient the plan
/// did not name yields no planned spectra — publishing its trajectory's
/// transforms would put two different spectra under one `fft-NNN` identity.
///
/// A nonzero spectrum count that disagrees with the plan is a typed error: the
/// identities are assigned by authored card order, and pairing them with a
/// different set of spectra would publish one card's transform under another
/// card's name.
pub fn planned_transient_fft_spectra<'result>(
    plan: &DeckPlan,
    parent: AnalysisInstanceId,
    results: &'result [TransientFftResult],
    abort: &dyn AbortSignal,
) -> Result<Vec<PlannedFftSpectrum<'result>>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let planned = plan
        .post_process_analyses()
        .iter()
        .filter(|post| {
            post.parent() == parent && matches!(post.source(), PostProcessSource::Fft { .. })
        })
        .collect::<Vec<_>>();
    if planned.is_empty() {
        return Ok(Vec::new());
    }
    if planned.len() != results.len() {
        return Err(SimulationError::Netlist(format!(
            "the deck plans {} .FFT spectra for {parent} but the transient produced {}",
            planned.len(),
            results.len()
        )));
    }
    let mut spectra = Vec::new();
    spectra
        .try_reserve_exact(planned.len())
        .map_err(|_| SimulationError::Circuit("planned .FFT spectra".to_owned()))?;
    for (index, (post, result)) in planned.into_iter().zip(results).enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let PostProcessSource::Fft { card_index } = post.source() else {
            return Err(SimulationError::Circuit(format!(
                "{} is not a planned .FFT card",
                post.id()
            )));
        };
        if *card_index != index {
            return Err(SimulationError::Netlist(format!(
                "planned .FFT card {} of {parent} is bound to authored card {}",
                index + 1,
                card_index + 1
            )));
        }
        spectra.push(PlannedFftSpectrum {
            analysis: post.id(),
            parent,
            output_unit: transient_fft_output_unit(result.physical_type, result.format)?,
            result,
        });
    }
    Ok(spectra)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{ImmediateAbort, NoAbort};
    use crate::netlist::Netlist;

    const TWO_CARD_DECK: &str = "planned fft identities\n\
         V1 out 0 SIN(0 1 1k)\n\
         R1 out 0 1k\n\
         .tran 1u 1m\n\
         .fft v(out) np=8 format=unorm freq=1k\n\
         .fft i(V1) np=8 format=unorm freq=1k\n\
         .end\n";

    fn planned_deck(source: &str) -> (Netlist, DeckPlan) {
        let netlist = Netlist::parse_validated(source).expect("the fixture deck parses");
        let plan = DeckPlan::from_netlist(&netlist, &crate::resource::ResourceLimits::default())
            .expect("the fixture deck plans");
        (netlist, plan)
    }

    fn transient_with_fft(netlist: &Netlist) -> crate::engine::TransientResult {
        crate::engine::Engine::new(crate::engine::SimulationConfig::default())
            .run_tran_with_abort(netlist, 1.0e-3, 1.0e-6, &NoAbort)
            .expect("the fixture transient solves")
    }

    #[test]
    fn every_authored_card_is_paired_with_its_planned_identity_in_order() {
        let (netlist, plan) = planned_deck(TWO_CARD_DECK);
        let result = transient_with_fft(&netlist);
        let parent = plan
            .analyses()
            .iter()
            .find(|analysis| analysis.id().kind() == crate::execution::AnalysisKind::Tran)
            .expect("the deck plans a transient")
            .id();
        let spectra = planned_transient_fft_spectra(&plan, parent, &result.fft_results, &NoAbort)
            .expect("the planned spectra pair");
        let tags = spectra
            .iter()
            .map(|spectrum| spectrum.analysis.tag())
            .collect::<Vec<_>>();
        assert_eq!(tags, vec!["fft-001".to_owned(), "fft-002".to_owned()]);
        assert!(spectra.iter().all(|spectrum| spectrum.parent == parent));
        assert_eq!(spectra[0].output_unit, SignalUnit::Volt);
        assert_eq!(spectra[1].output_unit, SignalUnit::Ampere);
    }

    #[test]
    fn a_spectrum_count_that_disagrees_with_the_plan_is_refused() {
        let (netlist, plan) = planned_deck(TWO_CARD_DECK);
        let result = transient_with_fft(&netlist);
        let parent = plan
            .analyses()
            .iter()
            .find(|analysis| analysis.id().kind() == crate::execution::AnalysisKind::Tran)
            .expect("the deck plans a transient")
            .id();
        let error =
            planned_transient_fft_spectra(&plan, parent, &result.fft_results[..1], &NoAbort)
                .expect_err("a short spectrum sequence must be refused");
        assert!(matches!(error, SimulationError::Netlist(_)), "{error}");
    }

    #[test]
    fn a_transient_the_plan_did_not_bind_publishes_no_spectra() {
        let source = "two transients one fft binding\n\
             V1 out 0 SIN(0 1 1k)\n\
             R1 out 0 1k\n\
             .tran 1u 1m\n\
             .tran 2u 1m\n\
             .fft v(out) np=8 format=unorm freq=1k\n\
             .end\n";
        let (netlist, plan) = planned_deck(source);
        let result = transient_with_fft(&netlist);
        assert_eq!(
            result.fft_results.len(),
            1,
            "the runner evaluates the card on every trajectory"
        );
        let transients = plan
            .analyses()
            .iter()
            .filter(|analysis| analysis.id().kind() == crate::execution::AnalysisKind::Tran)
            .map(|analysis| analysis.id())
            .collect::<Vec<_>>();
        assert_eq!(transients.len(), 2);
        assert_eq!(
            planned_transient_fft_spectra(&plan, transients[0], &result.fft_results, &NoAbort)
                .expect("the bound transient pairs")
                .len(),
            1
        );
        assert!(
            planned_transient_fft_spectra(&plan, transients[1], &result.fft_results, &NoAbort)
                .expect("an unbound transient is not an error")
                .is_empty(),
            "a second spectrum under the same fft-001 identity must not be published"
        );
    }

    #[test]
    fn pairing_is_cancellable() {
        let (netlist, plan) = planned_deck(TWO_CARD_DECK);
        let result = transient_with_fft(&netlist);
        let parent = plan
            .analyses()
            .iter()
            .find(|analysis| analysis.id().kind() == crate::execution::AnalysisKind::Tran)
            .expect("the deck plans a transient")
            .id();
        assert!(matches!(
            planned_transient_fft_spectra(&plan, parent, &result.fft_results, &ImmediateAbort),
            Err(SimulationError::Aborted)
        ));
    }

    #[test]
    fn a_normalized_spectrum_is_a_ratio_and_a_parameter_column_declares_no_unit() {
        assert_eq!(
            transient_fft_output_unit("voltage", FftFormat::Normalized)
                .expect("a normalized voltage spectrum has a unit"),
            SignalUnit::Dimensionless
        );
        assert_eq!(
            transient_fft_output_unit("parameter", FftFormat::Unnormalized)
                .expect("a parameter column has a declared missing unit"),
            SignalUnit::Unspecified
        );
        assert!(transient_fft_output_unit("charge", FftFormat::Unnormalized).is_err());
    }
}
