//! Qualified pole stability and numerical summaries of retained roots.

use super::data::{ComplexRoot, PoleZeroData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoleStabilityVerdict {
    Stable,
    Unstable,
    Indeterminate,
}

impl PoleStabilityVerdict {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Unstable => "unstable",
            Self::Indeterminate => "indeterminate",
        }
    }
}

pub fn pole_stability(data: &PoleZeroData) -> PoleStabilityVerdict {
    let poles = data
        .roots
        .iter()
        .filter(|root| root.is_pole())
        .collect::<Vec<_>>();
    if !data.pole_evidence.is_qualified()
        || !data.pole_evidence.is_consistent_with_count(poles.len())
    {
        return PoleStabilityVerdict::Indeterminate;
    }
    if poles.iter().all(|pole| pole.real < 0.0) {
        PoleStabilityVerdict::Stable
    } else {
        PoleStabilityVerdict::Unstable
    }
}

#[derive(Debug)]
pub struct PoleZeroSummary<'a> {
    pub pole_count: usize,
    pub zero_count: usize,
    pub dominant_pole: Option<&'a ComplexRoot>,
    pub worst_q: Option<f64>,
    pub right_half_plane_poles: usize,
    pub imaginary_axis_poles: usize,
}

fn imaginary_axis_tolerance(root: &ComplexRoot) -> f64 {
    64.0 * f64::EPSILON * root.real.abs().max(root.imag.abs()).max(1.0)
}

pub fn summarize_roots(data: &PoleZeroData) -> PoleZeroSummary<'_> {
    let poles = data
        .roots
        .iter()
        .filter(|root| root.is_pole())
        .collect::<Vec<_>>();
    let dominant_pole = poles.iter().copied().max_by(|left, right| {
        left.real
            .total_cmp(&right.real)
            .then_with(|| left.imag.total_cmp(&right.imag))
    });
    let mut worst_q: Option<f64> = None;
    let mut right_half_plane_poles = 0;
    let mut imaginary_axis_poles = 0;
    for pole in &poles {
        let tolerance = imaginary_axis_tolerance(pole);
        if pole.real > tolerance {
            right_half_plane_poles += 1;
        } else if pole.real.abs() <= tolerance {
            imaginary_axis_poles += 1;
        }
        let q = if pole.imag.abs() <= tolerance {
            None
        } else if pole.real.abs() <= tolerance {
            Some(f64::INFINITY)
        } else {
            let q = pole.natural_frequency() / (2.0 * pole.real.abs());
            (q.is_finite() && q >= 0.0).then_some(q)
        };
        if let Some(q) = q
            && worst_q.is_none_or(|current| q > current)
        {
            worst_q = Some(q);
        }
    }

    PoleZeroSummary {
        pole_count: poles.len(),
        zero_count: data.roots.len().saturating_sub(poles.len()),
        dominant_pole,
        worst_q,
        right_half_plane_poles,
        imaginary_axis_poles,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn qualified_evidence(root_count: u64) -> crate::pole_zero::PoleZeroRootSetEvidence {
        let certificate = crate::pole_zero::PoleZeroSpectrumCertificate {
            problem_order: root_count,
            infinite_count: 0,
            max_backward_error: 1.0e-14,
            qualification_tolerance:
                crate::pole_zero::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(
                    root_count,
                )
                .unwrap(),
        };
        if root_count == 0 {
            crate::pole_zero::PoleZeroRootSetEvidence::QualifiedEmpty { certificate }
        } else {
            crate::pole_zero::PoleZeroRootSetEvidence::Qualified { certificate }
        }
    }

    #[test]
    fn qualified_imaginary_axis_poles_are_unstable() {
        let mut data = PoleZeroData::new("axis pole");
        data.roots.push(ComplexRoot::pole(0.0, 10.0));
        data.roots.push(ComplexRoot::pole(0.0, -10.0));
        data.pole_evidence = qualified_evidence(2);

        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Unstable);
    }

    #[test]
    fn right_half_plane_pole_overrides_marginal_poles() {
        let mut data = PoleZeroData::new("unstable");
        data.roots.push(ComplexRoot::pole(0.0, 10.0));
        data.roots.push(ComplexRoot::pole(0.5, 0.0));
        data.pole_evidence = qualified_evidence(2);

        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Unstable);
    }

    #[test]
    fn strictly_left_half_plane_poles_are_stable() {
        let mut data = PoleZeroData::new("stable");
        data.roots.push(ComplexRoot::pole(-0.5, 0.0));
        data.roots.push(ComplexRoot::pole(-2.0, 10.0));
        data.pole_evidence = qualified_evidence(2);

        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Stable);
    }

    #[test]
    fn unqualified_pole_evidence_is_always_indeterminate() {
        let mut data = PoleZeroData::new("unqualified");
        data.roots.push(ComplexRoot::pole(-1.0, 0.0));
        for evidence in [
            crate::pole_zero::PoleZeroRootSetEvidence::NotRequested,
            crate::pole_zero::PoleZeroRootSetEvidence::LegacyUnknown,
            crate::pole_zero::PoleZeroRootSetEvidence::Approximate {
                certificate: crate::pole_zero::PoleZeroSpectrumCertificate {
                    problem_order: 1,
                    infinite_count: 0,
                    max_backward_error: 1.0e-9,
                    qualification_tolerance: crate::pole_zero::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1).unwrap(),
                },
            },
        ] {
            data.pole_evidence = evidence;
            assert_eq!(pole_stability(&data), PoleStabilityVerdict::Indeterminate);
        }

        data.roots.clear();
        data.pole_evidence = crate::pole_zero::PoleZeroRootSetEvidence::NotRequested;
        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Indeterminate);
        data.pole_evidence = qualified_evidence(0);
        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Stable);
    }

    #[test]
    fn root_summary_derives_mockup_metrics_only_from_retained_roots() {
        let mut data = PoleZeroData::new("summary");
        data.roots.push(ComplexRoot::pole(-100.0, 0.0));
        data.roots.push(ComplexRoot::pole(-10.0, 40.0));
        data.roots.push(ComplexRoot::pole(-10.0, -40.0));
        data.roots.push(ComplexRoot::zero(-3.0, 0.0));

        let summary = summarize_roots(&data);
        assert_eq!(summary.pole_count, 3);
        assert_eq!(summary.zero_count, 1);
        assert_eq!(summary.dominant_pole.map(|pole| pole.real), Some(-10.0));
        assert_eq!(summary.right_half_plane_poles, 0);
        assert_eq!(summary.imaginary_axis_poles, 0);
        assert!((summary.worst_q.unwrap() - 1700.0_f64.sqrt() / 20.0).abs() < 1.0e-12);
    }

    #[test]
    fn real_poles_do_not_invent_quality_factor() {
        let mut data = PoleZeroData::new("real poles");
        data.roots.push(ComplexRoot::pole(-1.0, 0.0));
        data.roots.push(ComplexRoot::pole(-10.0, 0.0));

        assert_eq!(summarize_roots(&data).worst_q, None);
    }
}
