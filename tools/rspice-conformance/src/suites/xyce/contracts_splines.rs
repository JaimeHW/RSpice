use super::*;

impl XyceTestRunner {
    pub(super) fn abm_lookup_order_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceAbmLookupOrderFamilyContract, String>> {
        let (spec, role) = XyceAbmLookupOrderRole::for_record(&deck.relative_path)?;
        Some((|| {
            const LABEL: &str = "ABM_SPLINES inline-lookup ordering family";
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
            let control_path = self.root.join(spec.control_path);
            let expected_path = match role {
                XyceAbmLookupOrderRole::WrapperOwner => &owner_path,
                XyceAbmLookupOrderRole::SortedControl => &control_path,
            };
            if !Self::same_path(&deck.path, expected_path) {
                return Err(format!(
                    "recognized {LABEL} role {role:?} is not backed by its canonical path"
                ));
            }
            let contract = XyceAbmLookupOrderFamilyContract {
                relational: XyceBaselineFamilyContract {
                    kind: XyceBaselineFamilyKind::AbmLookupOrder,
                    comparison: XyceBaselineFamilyComparison::ExactPrn,
                    family: spec.family.to_string(),
                    baseline_path: control_path.clone(),
                    member_paths: vec![owner_path.clone(), control_path.clone()],
                    target_path: Some(expected_path.clone()),
                },
                role,
            };
            Ok(contract)
        })())
    }
}
