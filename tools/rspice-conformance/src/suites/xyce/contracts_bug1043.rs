use super::*;

const LABEL: &str = "BUG_1043_SON AC DATA parameter family";

impl XyceTestRunner {
    pub(super) fn bug1043_ac_data_parameter_ac_comparator_tolerance() -> XyceAcComparatorTolerance {
        XyceAcComparatorTolerance::new(6.0e-5, 1.0e-4, 1.0e-6, 1.0e-6)
            .expect("Release 7.10 BUG_1043_SON ACComparator tolerance is valid")
    }

    pub(super) fn bug1043_ac_data_parameter_grid_matches(frequencies: &[Value]) -> bool {
        frequencies.len() == XYCE_BUG1043_FREQUENCY_GRID.len()
            && frequencies
                .iter()
                .copied()
                .zip(XYCE_BUG1043_FREQUENCY_GRID)
                .all(|(actual, expected)| {
                    actual.is_finite()
                        && (actual.to_bits() == expected.to_bits()
                            || (actual - expected).abs()
                                <= expected.abs() * XYCE_BUG1043_FREQUENCY_GRID_RELATIVE_ROUNDOFF)
                })
    }

    pub(super) fn bug1043_ac_data_parameter_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug1043AcDataParameterFamilyContract, String>> {
        let (spec, role) = XyceBug1043AcDataParameterRole::for_record(&deck.relative_path)?;
        Some((|| {
            if deck.section != XyceDeckSection::Netlists
                || Self::normalize_manifest_key(&self.relative_key(&deck.path))
                    != Self::normalize_manifest_key(&deck.relative_path)
            {
                return Err(format!(
                    "recognized {LABEL} record '{}' is not backed by its exact Netlists path",
                    deck.relative_path
                ));
            }
            let owner_path = self.root.join(spec.owner_path);
            let baseline_path = self.root.join(spec.baseline_path);
            let expected_path = match role {
                XyceBug1043AcDataParameterRole::DataWrapperOwner => &owner_path,
                XyceBug1043AcDataParameterRole::ExpressionBaseline => &baseline_path,
            };
            if !Self::same_path(&deck.path, expected_path) {
                return Err(format!(
                    "recognized {LABEL} role {role:?} is not backed by its canonical path"
                ));
            }
            let contract = XyceBug1043AcDataParameterFamilyContract {
                relational: XyceBaselineFamilyContract {
                    kind: XyceBaselineFamilyKind::Bug1043AcDataParameters,
                    comparison: XyceBaselineFamilyComparison::AcComparator(
                        Self::bug1043_ac_data_parameter_ac_comparator_tolerance(),
                    ),
                    family: spec.family.to_string(),
                    baseline_path: baseline_path.clone(),
                    member_paths: vec![owner_path.clone(), baseline_path.clone()],
                    target_path: Some(expected_path.clone()),
                },
                owner_path,
                baseline_path,
                role,
            };
            self.validate_bug1043_ac_data_parameter_semantics(&contract)?;
            Ok(contract)
        })())
    }

    /// The DATA owner and the expression baseline must parse to one six-row AC circuit.
    fn validate_bug1043_ac_data_parameter_semantics(
        &self,
        contract: &XyceBug1043AcDataParameterFamilyContract,
    ) -> Result<(), String> {
        let owner_plan = self
            .bug1043_relational_ac_plan_for_path(&contract.owner_path)
            .map_err(|error| format!("{LABEL} DATA owner plan failed: {error}"))?;
        let baseline_plan = self
            .bug1043_relational_ac_plan_for_path(&contract.baseline_path)
            .map_err(|error| format!("{LABEL} expression baseline plan failed: {error}"))?;
        Self::validate_bug1043_ac_data_parameter_ac_plan(&owner_plan)?;
        Self::validate_bug1043_ac_data_parameter_ac_plan(&baseline_plan)?;
        if !Self::bug1043_ac_data_parameter_grid_matches(&owner_plan.ac.frequencies)
            || !Self::bug1043_ac_data_parameter_grid_matches(&baseline_plan.ac.frequencies)
        {
            return Err(format!("{LABEL} six-row frequency grid changed"));
        }
        let owner_netlist = Self::relational_ac_plan_netlist_for_kind(
            XyceBaselineFamilyKind::Bug1043AcDataParameters,
            &owner_plan,
        )?;
        let baseline_netlist = Self::relational_ac_plan_netlist_for_kind(
            XyceBaselineFamilyKind::Bug1043AcDataParameters,
            &baseline_plan,
        )?;
        let owner_snapshot =
            Self::bug1043_ac_data_parameter_family_snapshot(&owner_plan, &owner_netlist)?;
        let baseline_snapshot =
            Self::bug1043_ac_data_parameter_family_snapshot(&baseline_plan, &baseline_netlist)?;
        Self::compare_bug1043_ac_data_parameter_snapshots(&baseline_snapshot, &owner_snapshot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce")
    }

    fn canonical_deck(relative_path: &str) -> XyceDeck {
        XyceDeck {
            path: corpus_root().join(relative_path),
            section: XyceDeckSection::Netlists,
            relative_path: relative_path.to_string(),
        }
    }

    #[test]
    fn bug1043_canonical_pair_has_directional_typed_contract() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for (relative_path, expected_role) in [
            (
                XYCE_BUG1043_OWNER_PATH,
                XyceBug1043AcDataParameterRole::DataWrapperOwner,
            ),
            (
                XYCE_BUG1043_EXPRESSION_BASELINE_PATH,
                XyceBug1043AcDataParameterRole::ExpressionBaseline,
            ),
        ] {
            let contract = runner
                .bug1043_ac_data_parameter_family_contract(&canonical_deck(relative_path))
                .expect("BUG1043 pair member is recognized")
                .unwrap_or_else(|error| panic!("canonical BUG1043 member failed: {error}"));
            assert_eq!(contract.role, expected_role);
            assert_eq!(
                contract.role.result_contract(),
                expected_role.result_contract()
            );
            assert!(
                !contract
                    .relational
                    .kind
                    .ac_comparator_member_is_good_waveform()
            );
        }
        assert!(
            runner
                .bug1043_ac_data_parameter_family_contract(&canonical_deck(
                    XYCE_BUG1043_ANALYTIC_PATH
                ))
                .is_none()
        );
    }
}
