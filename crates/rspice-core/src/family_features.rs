//! Compile-time agreement on feature flags between the crates of the family.
//!
//! Every crate of the `rspice-core` family declares the same features and
//! forwards each one to the family crates it depends on. A feature declared
//! but not forwarded reads false in the crate below without any error, and a
//! suite testing the code behind it then passes with that code compiled out.
//! `family_features!` turns the disagreement into a compile error. Invoked
//! at a crate's root, it states the crate's own features as
//! `FAMILY_FEATURES`; for each family crate it names, it asserts that every
//! flag matches that crate's `FAMILY_FEATURES`.
//!
//! `default` is not a flag: a family edge turns the dependency's defaults off,
//! so the two crates' `default` differ by design. `test-support` is not one
//! either, because each crate's own dev-dependencies enable it.
//! `tools/ci/core_family.py` checks the same agreement in the manifests.

/// Whether each feature of the family is on in the crate that stated it: one
/// flag per feature of `rspice-core`'s manifest except `default`, named for
/// the feature with `-` spelled `_`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[doc(hidden)]
pub struct FamilyFeatures {
    pub wasm: bool,
    pub parallel: bool,
    pub faer_parallel: bool,
    pub veriloga: bool,
    pub veriloga_native: bool,
    pub veriloga_wasm_jit: bool,
    pub veriloga_builtins_base: bool,
    pub veriloga_builtins_noise: bool,
    pub veriloga_model_diode_cmc: bool,
    pub veriloga_model_epfl_hemt_10a: bool,
    pub veriloga_model_juncap200: bool,
    pub veriloga_model_psp104tva: bool,
    pub veriloga_model_psp104va: bool,
    pub veriloga_model_pspnqs104va: bool,
    pub veriloga_model_angelov: bool,
    pub veriloga_model_angelov_gan: bool,
    pub veriloga_model_asmesd: bool,
    pub veriloga_model_asmesd_dio: bool,
    pub veriloga_model_asmhemt: bool,
    pub veriloga_model_bjt505_va: bool,
    pub veriloga_model_bjt505t_va: bool,
    pub veriloga_model_bjtd505_va: bool,
    pub veriloga_model_bjtd505t_va: bool,
    pub veriloga_model_bsimbulk: bool,
    pub veriloga_model_bsimcmg_va: bool,
    pub veriloga_model_bsimimg: bool,
    pub veriloga_model_bsimsoi_18c250bc: bool,
    pub veriloga_model_bsimsoi_e2aff994: bool,
    pub veriloga_model_bsimsoi_va: bool,
    pub veriloga_model_ekv3_rf: bool,
    pub veriloga_model_ekv_va: bool,
    pub veriloga_model_hicuml0va: bool,
    pub veriloga_model_hicuml2va: bool,
    pub veriloga_model_hisimhv_n4_va: bool,
    pub veriloga_model_hisimhv_n5_va: bool,
    pub veriloga_model_hisimhv_va: bool,
    pub veriloga_model_hisimsoi_va_5be18005: bool,
    pub veriloga_model_hisimsoi_va_242bc21d: bool,
    pub veriloga_model_hisimsoi_va_38074d06: bool,
    pub veriloga_model_hisimsotb_va: bool,
    pub veriloga_model_l_utsoi_832ce87d: bool,
    pub veriloga_model_l_utsoi_485e0ac9: bool,
    pub veriloga_model_mosvar: bool,
    pub veriloga_model_mvsg_cmc: bool,
    pub veriloga_model_r2_cmc: bool,
    pub veriloga_model_r2_et_cmc: bool,
    pub veriloga_model_r3_cmc: bool,
    pub veriloga_model_vbic13: bool,
    pub veriloga_model_vbic13_3t_et: bool,
    pub veriloga_model_vbic13_4t: bool,
    pub veriloga_model_vbic_4t_et_cf: bool,
    pub veriloga_builtins_models: bool,
    pub veriloga_builtins: bool,
    pub simd: bool,
}

/// States the calling crate's features at its root and fails the build when
/// a named family crate was built with different ones.
///
/// `family_features!()` defines `FAMILY_FEATURES`, each flag read by `cfg!`
/// in the calling crate. `family_features!(rspice_core_base)` also asserts,
/// flag by flag, that it equals `rspice_core_base::FAMILY_FEATURES`.
#[doc(hidden)]
#[macro_export]
macro_rules! family_features {
    (@define [$($dependency:ident)*] {$($field:ident $feature:tt)*}) => {
        #[doc(hidden)]
        pub const FAMILY_FEATURES: $crate::family_features::FamilyFeatures =
            $crate::family_features::FamilyFeatures { $($field: cfg!(feature = $feature)),* };
        $crate::family_features!(@agree [$($dependency)*] {$($field $feature)*});
    };
    (@agree [] $flags:tt) => {};
    (@agree [$dependency:ident $($rest:ident)*] {$($field:ident $feature:tt)*}) => {
        $(
            const _: () = assert!(
                cfg!(feature = $feature) == $dependency::FAMILY_FEATURES.$field,
                concat!(
                    "rspice-core family feature `", $feature, "` differs from `",
                    stringify!($dependency), "`"
                ),
            );
        )*
        $crate::family_features!(@agree [$($rest)*] {$($field $feature)*});
    };
    ($($dependency:ident),* $(,)?) => {
        $crate::family_features!(@define [$($dependency)*] {
            wasm "wasm"
            parallel "parallel"
            faer_parallel "faer-parallel"
            veriloga "veriloga"
            veriloga_native "veriloga-native"
            veriloga_wasm_jit "veriloga-wasm-jit"
            veriloga_builtins_base "veriloga-builtins-base"
            veriloga_builtins_noise "veriloga-builtins-noise"
            veriloga_model_diode_cmc "veriloga-model-diode-cmc"
            veriloga_model_epfl_hemt_10a "veriloga-model-epfl-hemt-10a"
            veriloga_model_juncap200 "veriloga-model-juncap200"
            veriloga_model_psp104tva "veriloga-model-psp104tva"
            veriloga_model_psp104va "veriloga-model-psp104va"
            veriloga_model_pspnqs104va "veriloga-model-pspnqs104va"
            veriloga_model_angelov "veriloga-model-angelov"
            veriloga_model_angelov_gan "veriloga-model-angelov-gan"
            veriloga_model_asmesd "veriloga-model-asmesd"
            veriloga_model_asmesd_dio "veriloga-model-asmesd-dio"
            veriloga_model_asmhemt "veriloga-model-asmhemt"
            veriloga_model_bjt505_va "veriloga-model-bjt505-va"
            veriloga_model_bjt505t_va "veriloga-model-bjt505t-va"
            veriloga_model_bjtd505_va "veriloga-model-bjtd505-va"
            veriloga_model_bjtd505t_va "veriloga-model-bjtd505t-va"
            veriloga_model_bsimbulk "veriloga-model-bsimbulk"
            veriloga_model_bsimcmg_va "veriloga-model-bsimcmg-va"
            veriloga_model_bsimimg "veriloga-model-bsimimg"
            veriloga_model_bsimsoi_18c250bc "veriloga-model-bsimsoi-18c250bc"
            veriloga_model_bsimsoi_e2aff994 "veriloga-model-bsimsoi-e2aff994"
            veriloga_model_bsimsoi_va "veriloga-model-bsimsoi-va"
            veriloga_model_ekv3_rf "veriloga-model-ekv3-rf"
            veriloga_model_ekv_va "veriloga-model-ekv-va"
            veriloga_model_hicuml0va "veriloga-model-hicuml0va"
            veriloga_model_hicuml2va "veriloga-model-hicuml2va"
            veriloga_model_hisimhv_n4_va "veriloga-model-hisimhv-n4-va"
            veriloga_model_hisimhv_n5_va "veriloga-model-hisimhv-n5-va"
            veriloga_model_hisimhv_va "veriloga-model-hisimhv-va"
            veriloga_model_hisimsoi_va_5be18005 "veriloga-model-hisimsoi-va-5be18005"
            veriloga_model_hisimsoi_va_242bc21d "veriloga-model-hisimsoi-va-242bc21d"
            veriloga_model_hisimsoi_va_38074d06 "veriloga-model-hisimsoi-va-38074d06"
            veriloga_model_hisimsotb_va "veriloga-model-hisimsotb-va"
            veriloga_model_l_utsoi_832ce87d "veriloga-model-l-utsoi-832ce87d"
            veriloga_model_l_utsoi_485e0ac9 "veriloga-model-l-utsoi-485e0ac9"
            veriloga_model_mosvar "veriloga-model-mosvar"
            veriloga_model_mvsg_cmc "veriloga-model-mvsg-cmc"
            veriloga_model_r2_cmc "veriloga-model-r2-cmc"
            veriloga_model_r2_et_cmc "veriloga-model-r2-et-cmc"
            veriloga_model_r3_cmc "veriloga-model-r3-cmc"
            veriloga_model_vbic13 "veriloga-model-vbic13"
            veriloga_model_vbic13_3t_et "veriloga-model-vbic13-3t-et"
            veriloga_model_vbic13_4t "veriloga-model-vbic13-4t"
            veriloga_model_vbic_4t_et_cf "veriloga-model-vbic-4t-et-cf"
            veriloga_builtins_models "veriloga-builtins-models"
            veriloga_builtins "veriloga-builtins"
            simd "simd"
        });
    };
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::FamilyFeatures;

    /// The flags an invocation lists: each `field "feature"` line of the
    /// table in `family_features!`, read from this file.
    fn listed_flags() -> Vec<(&'static str, &'static str)> {
        let source = include_str!("family_features.rs");
        let table = source
            .split_once("@define [$($dependency)*] {")
            .and_then(|(_, rest)| rest.split_once("});"))
            .map(|(table, _)| table)
            .expect("the invocation table of `family_features!`");
        table
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (field, feature) = line.split_once(' ').expect("a `field \"feature\"` line");
                let feature = feature
                    .strip_prefix('"')
                    .and_then(|feature| feature.strip_suffix('"'))
                    .expect("a quoted feature name");
                (field, feature)
            })
            .collect()
    }

    /// Every feature `rspice-core`'s manifest declares, `default` aside.
    fn manifest_features() -> BTreeSet<&'static str> {
        let mut in_features = false;
        let mut features = BTreeSet::new();
        for line in include_str!("../Cargo.toml").lines() {
            if line.starts_with('[') {
                in_features = line == "[features]";
            } else if in_features && line.starts_with(|c: char| c.is_ascii_lowercase()) {
                let (name, _) = line.split_once(" = ").expect("a feature definition");
                features.insert(name);
            }
        }
        features.remove("default");
        features
    }

    #[test]
    fn every_feature_of_the_manifest_has_one_flag_named_for_it() {
        let flags = listed_flags();
        for (field, feature) in &flags {
            assert_eq!(
                field.replace('_', "-"),
                *feature,
                "the flag `{field}` reads the cfg of another feature"
            );
        }
        let listed: BTreeSet<_> = flags.iter().map(|(_, feature)| *feature).collect();
        assert_eq!(listed.len(), flags.len(), "a feature is listed twice");
        assert_eq!(listed, manifest_features());
    }

    #[test]
    fn the_crate_root_states_the_features_of_this_build() {
        let stated = crate::FAMILY_FEATURES;
        assert_eq!(stated.wasm, cfg!(feature = "wasm"));
        assert_eq!(stated.parallel, cfg!(feature = "parallel"));
        assert_eq!(stated.faer_parallel, cfg!(feature = "faer-parallel"));
        assert_eq!(stated.simd, cfg!(feature = "simd"));
        assert_eq!(stated.veriloga, cfg!(feature = "veriloga"));
        assert_eq!(stated.veriloga_native, cfg!(feature = "veriloga-native"));
        assert_eq!(
            stated.veriloga_wasm_jit,
            cfg!(feature = "veriloga-wasm-jit")
        );
        assert_eq!(
            stated.veriloga_builtins_base,
            cfg!(feature = "veriloga-builtins-base")
        );
        assert_eq!(
            stated.veriloga_builtins_noise,
            cfg!(feature = "veriloga-builtins-noise")
        );
        assert_eq!(
            stated.veriloga_model_diode_cmc,
            cfg!(feature = "veriloga-model-diode-cmc")
        );
        assert_eq!(
            stated.veriloga_builtins_models,
            cfg!(feature = "veriloga-builtins-models")
        );
        assert_eq!(
            stated.veriloga_builtins,
            cfg!(feature = "veriloga-builtins")
        );
        // A default build has `parallel`, `faer-parallel` and `simd` and nothing
        // else, so no other flag, a model's included, may read true.
        if cfg!(all(
            feature = "default",
            not(feature = "veriloga"),
            not(feature = "veriloga-builtins-base"),
            not(feature = "wasm")
        )) {
            assert_eq!(
                format!("{stated:?}").matches(": true").count(),
                3,
                "{stated:?}"
            );
        }
    }

    /// A crate built with the same features as the one it names agrees with
    /// it: these modules only compile because every assertion holds.
    mod dependency {
        pub(super) const FAMILY_FEATURES: super::FamilyFeatures = crate::FAMILY_FEATURES;
    }

    mod dependent {
        use super::dependency;

        crate::family_features!(dependency);
    }

    #[test]
    fn a_crate_naming_a_family_dependency_states_the_same_flags() {
        assert_eq!(dependent::FAMILY_FEATURES, dependency::FAMILY_FEATURES);
    }
}
