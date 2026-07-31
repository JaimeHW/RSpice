//! Authoritative inspection of PDK corner-to-section contracts.
//!
//! The inspector seals the same authenticated source closure used by prepared
//! simulation runs and materializes every named section. UI status therefore
//! cannot claim a binding is resolved merely because a string happens to be
//! present in project state.

use super::*;

/// One materialized section proven to exist in the sealed execution source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CornerSectionInspection {
    pub domains: Vec<super::super::CornerSectionDomain>,
    pub source_path: PathBuf,
    pub section: String,
    pub content_digest: ContentDigest,
    pub model_card_count: usize,
}

/// Validation result for one library/corner row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CornerBindingInspectionRow {
    pub library_name: String,
    pub corner_name: String,
    pub selected: bool,
    pub resolved_sections: Vec<CornerSectionInspection>,
    pub issues: Vec<String>,
}

impl CornerBindingInspectionRow {
    #[must_use]
    pub fn is_resolved(&self) -> bool {
        self.issues.is_empty() && !self.resolved_sections.is_empty()
    }
}

/// Complete result for one exact model execution catalogue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CornerBindingInspection {
    pub catalog_digest: ContentDigest,
    pub rows: Vec<CornerBindingInspectionRow>,
    pub global_issues: Vec<String>,
}

impl CornerBindingInspection {
    #[must_use]
    pub fn resolved_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_resolved()).count()
    }

    #[must_use]
    pub fn unresolved_count(&self) -> usize {
        self.rows.iter().filter(|row| !row.is_resolved()).count()
    }

    #[must_use]
    pub fn is_resolved(&self) -> bool {
        self.global_issues.is_empty()
            && !self.rows.is_empty()
            && self
                .rows
                .iter()
                .all(CornerBindingInspectionRow::is_resolved)
    }
}

impl ModelLibraryManager {
    /// Inspect every source-backed corner against the exact immutable source
    /// snapshot a simulation would receive.
    ///
    /// This deliberately returns a report rather than failing at the first
    /// defect. A broken source seal blocks all rows because none of them can be
    /// proven against executable bytes, while contract-local defects remain
    /// attached to their owning row.
    #[must_use]
    pub fn inspect_corner_bindings(&self) -> CornerBindingInspection {
        let catalog_digest = self.execution_catalog_digest();
        let mut rows = Vec::new();
        for library in self.libraries_sorted() {
            let mut corners = library
                .corners
                .values()
                .filter(|corner| {
                    corner.file_path.is_some()
                        || !corner.section_bindings.is_empty()
                        || !corner.required_domains.is_empty()
                })
                .collect::<Vec<_>>();
            corners.sort_by(|left, right| {
                left.name
                    .to_ascii_lowercase()
                    .cmp(&right.name.to_ascii_lowercase())
                    .then_with(|| left.name.cmp(&right.name))
            });
            for corner in corners {
                let issues = corner.validate_contract().err().unwrap_or_default();
                rows.push(CornerBindingInspectionRow {
                    library_name: library.name.clone(),
                    corner_name: corner.name.clone(),
                    selected: self.selected_library.as_deref() == Some(&library.name)
                        && library.selected_corner.as_deref() == Some(&corner.name),
                    resolved_sections: Vec::new(),
                    issues,
                });
            }
        }

        let sealed = match self.seal_execution_sources() {
            Ok(sealed) => sealed,
            Err(error) => {
                return CornerBindingInspection {
                    catalog_digest,
                    rows,
                    global_issues: vec![error],
                };
            }
        };

        for row in &mut rows {
            if !row.issues.is_empty() {
                continue;
            }
            let Some(library) = sealed
                .libraries
                .iter()
                .find(|library| library.name == row.library_name)
            else {
                row.issues.push(
                    "library is not present in the authenticated execution source set".to_owned(),
                );
                continue;
            };
            let Some(corner) = library
                .corners
                .iter()
                .find(|corner| corner.name == row.corner_name)
            else {
                row.issues.push(
                    "corner is not present in the authenticated execution source set".to_owned(),
                );
                continue;
            };
            match sealed.materialize_library_corner(library, corner) {
                Ok(materialized) => {
                    for section in materialized {
                        if section.materialized_model_cards.trim().is_empty() {
                            row.issues.push(format!(
                                "section '{}' in '{}' materialized no model cards",
                                section.section,
                                section.path.display()
                            ));
                            continue;
                        }
                        let model_card_count = section
                            .materialized_model_cards
                            .lines()
                            .filter(|line| {
                                let token = line.split_whitespace().next().unwrap_or_default();
                                token.eq_ignore_ascii_case(".model")
                                    || token.eq_ignore_ascii_case(".subckt")
                            })
                            .count();
                        row.resolved_sections.push(CornerSectionInspection {
                            domains: section.domains,
                            source_path: section.path,
                            section: section.section,
                            content_digest: ContentDigest::from_bytes(
                                Sha256::digest(section.materialized_model_cards.as_bytes()).into(),
                            ),
                            model_card_count,
                        });
                    }
                }
                Err(error) => row.issues.push(error),
            }
        }

        CornerBindingInspection {
            catalog_digest,
            rows,
            global_issues: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manager_with_source(source: &[u8]) -> ModelLibraryManager {
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_bytes("corners.lib", source.to_vec(), Some("TT"))
            .expect("source imports");
        manager
    }

    #[test]
    fn imported_composite_sections_are_proven_from_sealed_bytes() {
        let manager = manager_with_source(
            b".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n\
              .lib FF\n.model nch NMOS (LEVEL=1 KP=2e-3)\n.endl FF\n",
        );

        let inspection = manager.inspect_corner_bindings();
        assert!(inspection.global_issues.is_empty());
        assert_eq!(inspection.rows.len(), 2);
        assert!(inspection.rows.iter().all(|row| row.is_resolved()));
        assert!(
            inspection
                .rows
                .iter()
                .all(|row| row.resolved_sections[0].model_card_count == 1)
        );
    }

    #[test]
    fn missing_required_domain_blocks_only_that_draft_contract() {
        let mut manager =
            manager_with_source(b".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n");
        let library = manager.get_library_mut("corners").expect("library exists");
        let corner = library.corners.get_mut("TT").expect("corner exists");
        corner.required_domains = vec![
            super::super::super::CornerSectionDomain::Composite,
            super::super::super::CornerSectionDomain::Aging,
        ];

        let inspection = manager.inspect_corner_bindings();
        assert_eq!(inspection.rows.len(), 1);
        assert!(!inspection.rows[0].is_resolved());
        assert!(
            inspection.rows[0]
                .issues
                .iter()
                .any(|issue| issue.contains("Aging section is required"))
        );
        assert!(inspection.global_issues.is_empty());
        assert!(
            manager
                .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
                .is_err(),
            "a requested unresolved draft still fails closed"
        );
    }

    #[test]
    fn independent_domains_layer_into_one_process_binding_set() {
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_bytes(
                "corners.lib",
                b".lib mos_tt\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl mos_tt\n\
                  .lib pass_tt\n.model rpoly R (RSH=10)\n.endl pass_tt\n"
                    .to_vec(),
                Some("mos_tt"),
            )
            .expect("source imports");
        let library = manager.get_library_mut("corners").expect("library exists");
        let mut corner = ProcessCorner::new("TT");
        corner.file_path = library.root_path.clone();
        corner.section_bindings = vec![
            super::super::super::CornerSectionBinding::new(
                super::super::super::CornerSectionDomain::Mos,
                "mos_tt",
            ),
            super::super::super::CornerSectionBinding::new(
                super::super::super::CornerSectionDomain::Passives,
                "pass_tt",
            ),
        ];
        corner.required_domains = vec![
            super::super::super::CornerSectionDomain::Mos,
            super::super::super::CornerSectionDomain::Passives,
        ];
        library.corners.clear();
        library.corners.insert("TT".to_owned(), corner);
        library.selected_corner = Some("TT".to_owned());

        let bindings = manager
            .corner_model_bindings(&[CornerProcess::TT])
            .expect("typed bindings materialize");
        assert_eq!(bindings.len(), 2);
        assert!(
            bindings
                .iter()
                .any(|binding| binding.section.as_deref() == Some("mos_tt"))
        );
        assert!(
            bindings
                .iter()
                .any(|binding| binding.section.as_deref() == Some("pass_tt"))
        );
    }
}
