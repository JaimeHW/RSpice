//! The shape, capability and normal-distribution statistics against scipy.

use super::*;

/// Skewness and kurtosis are the sample-adjusted G1 and G2, checked against
/// `scipy.stats.skew` and `scipy.stats.kurtosis` with `bias=False`
/// (scipy 1.x): G1 1.4218614948128678 and G2 2.4152972055114557 for this
/// set, where the biased g1 and g2 read 1.1400 and 0.4835.
#[test]
fn skewness_and_kurtosis_are_the_sample_adjusted_g1_and_g2() {
    let values = [1.2, 2.5, 2.9, 3.1, 4.8, 7.4, 3.3, 2.2];
    let g1 = skewness(&values).expect("eight samples");
    assert!((g1 - 1.421_861_494_812_867_8).abs() < 1.0e-12, "{g1}");
    let g2 = excess_kurtosis(&values).expect("eight samples");
    assert!((g2 - 2.415_297_205_511_455_7).abs() < 1.0e-12, "{g2}");
    let skewed_three = skewness(&[1.0, 2.0, 6.0]).expect("three samples");
    assert!((skewed_three - 1.457_862_967_321_304_6).abs() < 1.0e-12);
}

/// Two samples have no G1 and no G2: the sheet says so rather than reading
/// a kurtosis below the theoretical floor of −2, as a mixed estimator did
/// (−2.75). Three samples have a skewness and still no kurtosis; a set with
/// no spread has neither.
#[test]
fn too_few_samples_have_no_shape_rather_than_an_impossible_one() {
    assert_eq!(skewness(&[1.0, 3.0]), None);
    assert_eq!(excess_kurtosis(&[1.0, 3.0]), None);
    assert_eq!(excess_kurtosis(&[1.0, 2.0, 6.0]), None);
    assert_eq!(skewness(&[2.5; 9]), None);
    assert_eq!(excess_kurtosis(&[0.1; 9]), None);
}

/// Φ and Φ⁻¹ agree with `scipy.stats.norm.cdf` and `.ppf` to a double's
/// precision, in the tails as at the centre.
#[test]
fn the_normal_distribution_reads_as_scipy_does() {
    for (z, cdf, pdf) in [
        (
            -3.0,
            0.001_349_898_031_630_093_3,
            0.004_431_848_411_938_007_5,
        ),
        (-1.0, 0.158_655_253_931_457_07, 0.241_970_724_519_143_37),
        (0.0, 0.5, 0.398_942_280_401_432_7),
        (0.5, 0.691_462_461_274_013_1, 0.352_065_326_764_299_5),
        (4.0, 0.999_968_328_758_166_9, 0.000_133_830_225_764_885_37),
    ] {
        assert!(
            (normal_cdf(z) - cdf).abs() <= 2.0e-16 * cdf.max(1.0),
            "Φ({z})"
        );
        assert!((normal_pdf(z) - pdf).abs() <= 1.0e-16, "φ({z})");
    }
    for (p, z) in [
        (1.0e-9, -5.997_807_015_007_686_5),
        (0.001, -3.090_232_306_167_813),
        (0.025, -1.959_963_984_540_054_5),
        (0.3, -0.524_400_512_708_040_9),
        (0.5, 0.0),
        (0.975, 1.959_963_984_540_054),
        (0.999, 3.090_232_306_167_813),
    ] {
        let got = normal_quantile(p).expect("an open-interval probability");
        assert!((got - z).abs() < 1.0e-13, "Φ⁻¹({p}) = {got}");
    }
    assert_eq!(normal_quantile(0.0), None);
    assert_eq!(normal_quantile(1.0), None);
}

/// The probability plot stands each sorted sample at the normal quantile of
/// Filliben's position and reports the plot's correlation, both as
/// `scipy.stats.probplot(values, dist="norm")` returns them.
#[test]
fn the_probability_plot_matches_scipy_probplot() {
    let plot =
        normal_probability_plot(&[1.2, 2.2, 2.5, 2.9, 3.1, 3.3, 4.8, 7.4]).expect("eight samples");
    let expected = [
        -1.385_198_060_554_378_8,
        -0.837_571_564_143_199_1,
        -0.465_794_194_114_738_94,
        -0.150_393_367_458_632_02,
        0.150_393_367_458_631_88,
        0.465_794_194_114_738_77,
        0.837_571_564_143_199_1,
        1.385_198_060_554_378_8,
    ];
    for ((quantile, _), expected) in plot.points.iter().zip(expected) {
        assert!((quantile - expected).abs() < 1.0e-12, "{quantile}");
    }
    let samples: Vec<f64> = plot.points.iter().map(|(_, sample)| *sample).collect();
    assert_eq!(samples, [1.2, 2.2, 2.5, 2.9, 3.1, 3.3, 4.8, 7.4]);
    let r = plot.correlation.expect("a spread");
    assert!((r - 0.926_694_639_365_992_7).abs() < 1.0e-12, "{r}");
}

/// Cp is the width over six sigma and only two-sided; Cpk the nearer side
/// in three-sigma units, one-sided with one limit.
#[test]
fn capability_is_the_textbook_cp_and_cpk() {
    let both = capability(10.0, 1.0, Some(7.0), Some(16.0));
    assert_eq!(both.cp, Some(1.5));
    assert_eq!(both.cpk, Some(1.0));
    let upper = capability(10.0, 1.0, None, Some(16.0));
    assert_eq!((upper.cp, upper.cpk), (None, Some(2.0)));
    let none = capability(10.0, 0.0, Some(7.0), Some(16.0));
    assert_eq!(none, Capability::default());
}
