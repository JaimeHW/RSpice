use super::*;

impl XyceTestRunner {
    pub(super) fn abm_lookup_order_historical_oracle_provenance_records() -> Vec<String> {
        let mut artifacts = XYCE_ABM_LOOKUP_ORDER_CASES
            .iter()
            .map(|spec| {
                (
                    spec.wrapper_path,
                    spec.wrapper_bytes,
                    spec.wrapper_sha256,
                    spec.wrapper_blake3,
                )
            })
            .chain([(
                XYCE_ABM_LOOKUP_ORDER_HISTORICAL_EXCLUDE_PATH,
                XYCE_ABM_LOOKUP_ORDER_HISTORICAL_EXCLUDE_BYTES,
                XYCE_ABM_LOOKUP_ORDER_HISTORICAL_EXCLUDE_SHA256,
                XYCE_ABM_LOOKUP_ORDER_HISTORICAL_EXCLUDE_BLAKE3,
            )])
            .chain([(
                XYCE_RELEASE_710_XYCE_VERIFY_PATH,
                XYCE_RELEASE_710_XYCE_VERIFY_BYTES,
                XYCE_RELEASE_710_XYCE_VERIFY_SHA256,
                XYCE_RELEASE_710_XYCE_VERIFY_BLAKE3,
            )])
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_ABM_LOOKUP_ORDER_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_ABM_LOOKUP_ORDER_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        artifacts.sort();
        artifacts
    }

    pub(super) fn validate_abm_lookup_order_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::abm_lookup_order_historical_oracle_provenance_records();
        let provenance_hash = blake3::hash(records.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if records.len() != XYCE_ABM_LOOKUP_ORDER_HISTORICAL_ORACLE_RECORD_COUNT
            || provenance_hash != XYCE_ABM_LOOKUP_ORDER_HISTORICAL_ORACLE_BLAKE3
        {
            return Err(format!(
                "ABM_SPLINES lookup-order Release-7.10 wrapper/exclude/xyce_verify provenance changed: records={}/{provenance_hash}",
                records.len()
            ));
        }
        if UPSTREAM_EXCLUSIONS_SOURCE_COMMIT != XYCE_ABM_LOOKUP_ORDER_PRETRIM_COMMIT
            || UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE
                != "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d"
        {
            return Err(
                "ABM_SPLINES lookup-order pre-trim manifest commit/tree provenance changed"
                    .to_string(),
            );
        }
        Ok(())
    }

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
                owner_path,
                control_path,
                spec,
                role,
            };
            self.validate_abm_lookup_order_provenance(&contract)?;
            Ok(contract)
        })())
    }

    pub(super) fn validate_abm_lookup_order_provenance(
        &self,
        contract: &XyceAbmLookupOrderFamilyContract,
    ) -> Result<(), String> {
        const LABEL: &str = "ABM_SPLINES inline-lookup ordering family";
        Self::validate_abm_lookup_order_historical_oracle_provenance()?;
        let expected_target = match contract.role {
            XyceAbmLookupOrderRole::WrapperOwner => &contract.owner_path,
            XyceAbmLookupOrderRole::SortedControl => &contract.control_path,
        };
        if contract.relational.kind != XyceBaselineFamilyKind::AbmLookupOrder
            || contract.relational.comparison != XyceBaselineFamilyComparison::ExactPrn
            || contract.relational.family != contract.spec.family
            || !Self::same_path(&contract.relational.baseline_path, &contract.control_path)
            || contract.relational.member_paths.len() != 2
            || !Self::same_path(&contract.relational.member_paths[0], &contract.owner_path)
            || !Self::same_path(&contract.relational.member_paths[1], &contract.control_path)
            || !contract
                .relational
                .target_path
                .as_ref()
                .is_some_and(|path| Self::same_path(path, expected_target))
        {
            return Err(format!(
                "{LABEL} contract is not the exact owner/control directional pair"
            ));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let wrapper_records = Self::load_upstream_wrapper_decks(&self.root);
        let mut candidates = Vec::with_capacity(XYCE_ABM_LOOKUP_ORDER_CANDIDATE_COUNT);
        let mut candidate_content = Vec::with_capacity(XYCE_ABM_LOOKUP_ORDER_CANDIDATE_COUNT);
        let mut owner_rows = Vec::with_capacity(XYCE_ABM_LOOKUP_ORDER_OWNER_COUNT);
        let mut historical_exclusion_rows =
            Vec::with_capacity(XYCE_ABM_LOOKUP_ORDER_EXCLUSION_COUNT);

        for spec in &XYCE_ABM_LOOKUP_ORDER_CASES {
            for (relative_path, record, expected_hash, role) in [
                (
                    spec.owner_path,
                    spec.owner_record,
                    spec.owner_content_blake3,
                    XyceAbmLookupOrderRole::WrapperOwner,
                ),
                (
                    spec.control_path,
                    spec.control_record,
                    spec.control_content_blake3,
                    XyceAbmLookupOrderRole::SortedControl,
                ),
            ] {
                let path = self.root.join(relative_path);
                let relative = self.relative_key(&path);
                if Self::normalize_manifest_key(&relative) != record {
                    return Err(format!("{LABEL} canonical path '{record}' is unavailable"));
                }
                let metadata = fs::symlink_metadata(&path).map_err(|error| {
                    format!("failed to inspect {LABEL} record '{relative}': {error}")
                })?;
                if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                    return Err(format!(
                        "{LABEL} record '{relative}' must be a regular non-symlink file"
                    ));
                }
                let bytes = fs::read(&path)
                    .map_err(|error| format!("failed to read {LABEL} record: {error}"))?;
                let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
                let content_hash = blake3::hash(&canonical).to_hex().to_string();
                if canonical.is_empty() || content_hash != expected_hash {
                    return Err(format!(
                        "{LABEL} record '{relative}' identity changed: expected {expected_hash}, got {content_hash}"
                    ));
                }
                let key = Self::normalize_manifest_key(&relative);
                candidates.push(relative.clone());
                candidate_content.push(format!("{relative}\t{content_hash}"));
                match role {
                    XyceAbmLookupOrderRole::WrapperOwner => {
                        if !self.requires_upstream_wrapper(&relative)
                            || !wrapper_records.contains(&key)
                            || exclusions.contains_key(&key)
                        {
                            return Err(format!(
                                "{LABEL} owner '{relative}' lost exclusive removed-wrapper provenance"
                            ));
                        }
                        owner_rows
                            .push(format!("{relative}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}"));
                    }
                    XyceAbmLookupOrderRole::SortedControl => {
                        if self.requires_upstream_wrapper(&relative)
                            || wrapper_records.contains(&key)
                        {
                            return Err(format!(
                                "{LABEL} sorted control '{relative}' must not own a wrapper"
                            ));
                        }
                        let exclusion = exclusions.get(&key).ok_or_else(|| {
                            format!(
                                "{LABEL} sorted control '{relative}' lost historical exclusion provenance"
                            )
                        })?;
                        if !matches!(
                            &exclusion.disposition,
                            XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                                if expected_contract == XYCE_ABM_LOOKUP_ORDER_SORTED_CONTROL_CONTRACT
                        ) {
                            return Err(format!(
                                "{LABEL} sorted control '{relative}' lacks its exact independent qualification contract"
                            ));
                        }
                        historical_exclusion_rows.push(format!(
                            "{relative}\t{}\t{UPSTREAM_EXCLUDED_DISPOSITION}",
                            exclusion.source
                        ));
                    }
                }
                self.reject_wrapper_output_artifacts(&path)
                    .map_err(|error| format!("{LABEL} record '{relative}' {error}"))?;
            }
        }

        candidates.sort();
        candidate_content.sort();
        owner_rows.sort();
        historical_exclusion_rows.sort();
        let candidate_hash = blake3::hash(candidates.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(candidate_content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let owner_hash = blake3::hash(owner_rows.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let exclusion_hash = blake3::hash(historical_exclusion_rows.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if candidates.len() != XYCE_ABM_LOOKUP_ORDER_CANDIDATE_COUNT
            || candidate_hash != XYCE_ABM_LOOKUP_ORDER_CANDIDATE_BLAKE3
            || content_hash != XYCE_ABM_LOOKUP_ORDER_CANDIDATE_CONTENT_BLAKE3
            || owner_rows.len() != XYCE_ABM_LOOKUP_ORDER_OWNER_COUNT
            || owner_hash != XYCE_ABM_LOOKUP_ORDER_OWNER_MANIFEST_BLAKE3
            || historical_exclusion_rows.len() != XYCE_ABM_LOOKUP_ORDER_EXCLUSION_COUNT
            || exclusion_hash != XYCE_ABM_LOOKUP_ORDER_HISTORICAL_EXCLUSION_BLAKE3
        {
            return Err(format!(
                "{LABEL} provenance changed: candidates={}/{candidate_hash}/{content_hash}, owners={}/{owner_hash}, exclusions={}/{exclusion_hash}",
                candidates.len(),
                owner_rows.len(),
                historical_exclusion_rows.len()
            ));
        }
        Ok(())
    }
}
