//! Tests for report digests and revision history.
//!
//! Digests must be deterministic over canonical bytes, and revision-context
//! validation must fail atomically - a rejected history leaves the document
//! exactly as it was.

use super::*;

fn digest(seed: u8) -> ContentDigest {
    ContentDigest::from_bytes([seed; 32])
}

fn artifact(media_type: &str, seed: u8) -> FrozenReportArtifact {
    FrozenReportArtifact::new(media_type, vec![seed; 32]).unwrap()
}

fn dataset_snapshot(seed: u8) -> (ReportReferenceSnapshot, DatasetBinding) {
    let binding = DatasetBinding::new(DatasetId::new(), digest(seed));
    (
        ReportReferenceSnapshot::new(
            ReportSourceId::Dataset {
                dataset_id: binding.dataset_id,
            },
            None,
            binding.content_digest,
            vec![binding],
        )
        .unwrap(),
        binding,
    )
}

fn visualization_snapshot(
    revision: ObjectRevision,
    seed: u8,
) -> (ReportReferenceSnapshot, DatasetBinding) {
    let binding = DatasetBinding::new(DatasetId::new(), digest(seed));
    (
        ReportReferenceSnapshot::new(
            ReportSourceId::VisualizationDocument {
                document_id: ResultDocumentId::new(),
            },
            Some(revision),
            digest(seed.wrapping_add(1)),
            vec![binding],
        )
        .unwrap(),
        binding,
    )
}

fn external_snapshot(namespace: &str, seed: u8) -> ReportReferenceSnapshot {
    ReportReferenceSnapshot::new(
        ReportSourceId::ExternalRecord {
            namespace: namespace.to_owned(),
            key: format!("record-{seed}"),
        },
        Some(ObjectRevision::INITIAL),
        digest(seed),
        Vec::new(),
    )
    .unwrap()
}

fn evidence_snapshot(seed: u8) -> ReportReferenceSnapshot {
    ReportReferenceSnapshot::new(
        ReportSourceId::VerificationEvidence {
            evidence_id: VerificationEvidenceId::new(),
        },
        Some(ObjectRevision::INITIAL),
        digest(seed),
        Vec::new(),
    )
    .unwrap()
}

fn document_with_section() -> (ReportDocument, ReportPageId, ReportSectionId) {
    let mut document = ReportDocument::new("Release verification 4.2").unwrap();
    let receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddPage {
                title: "Performance".to_owned(),
            }],
            10,
        )
        .unwrap();
    let page_id = match receipt.created[0] {
        ReportEntityRef::Page(id) => id,
        _ => unreachable!(),
    };
    let receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddSection {
                page_id,
                title: "Nominal and corners".to_owned(),
            }],
            11,
        )
        .unwrap();
    let section_id = match receipt.created[0] {
        ReportEntityRef::Section(id) => id,
        _ => unreachable!(),
    };
    (document, page_id, section_id)
}

fn all_block_kinds() -> Vec<ReportBlockKind> {
    let (plot_snapshot, _) = visualization_snapshot(ObjectRevision::INITIAL, 10);
    let (table_snapshot, _) = dataset_snapshot(20);
    let (datasheet_snapshot, _) = dataset_snapshot(21);
    vec![
        ReportBlockKind::PlotFigure(PlotFigureBlock {
            caption: "Closed-loop gain".to_owned(),
            alternative_text: "Gain and phase across the requested frequency range.".to_owned(),
            sizing: FigureSizing::FitWidth,
            reference: ReportReferenceMode::Frozen {
                snapshot: plot_snapshot,
                artifact: FrozenReportArtifact::new(
                    "image/svg+xml",
                    b"<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_vec(),
                )
                .unwrap(),
            },
        }),
        ReportBlockKind::DataTable(DataTableBlock {
            title: "Corner summary".to_owned(),
            columns: vec![
                TableColumn {
                    key: "corner".to_owned(),
                    heading: "Corner".to_owned(),
                    unit: None,
                },
                TableColumn {
                    key: "gain".to_owned(),
                    heading: "Gain".to_owned(),
                    unit: Some("dB".to_owned()),
                },
            ],
            rows: vec![vec![
                TableCell::Text("tt".to_owned()),
                TableCell::Number {
                    value: 42.25,
                    unit: Some("dB".to_owned()),
                },
            ]],
            reference: ReportReferenceMode::Frozen {
                snapshot: table_snapshot,
                artifact: artifact("application/json", 22),
            },
        }),
        ReportBlockKind::Datasheet(DatasheetBlock {
            title: "Device summary".to_owned(),
            fields: vec![DatasheetField {
                key: "vdd".to_owned(),
                label: "Supply voltage".to_owned(),
                value: "1.8".to_owned(),
                unit: Some("V".to_owned()),
            }],
            reference: ReportReferenceMode::Linked {
                snapshot: datasheet_snapshot,
            },
        }),
        ReportBlockKind::Requirements(RequirementsBlock {
            title: "Requirements".to_owned(),
            entries: vec![RequirementEntry {
                requirement_id: "REQ-AMP-001".to_owned(),
                statement: "Nominal gain shall exceed 40 dB.".to_owned(),
                disposition: RequirementDisposition::Passed,
                evidence_label: Some("AC gain measurement".to_owned()),
            }],
            reference: ReportReferenceMode::Linked {
                snapshot: external_snapshot("requirements", 30),
            },
        }),
        ReportBlockKind::Specifications(SpecificationsBlock {
            title: "Specifications".to_owned(),
            entries: vec![SpecificationEntry {
                expression: "max(gain_db)".to_owned(),
                limit: ">= 40 dB".to_owned(),
                measured: Some("42.25 dB".to_owned()),
                disposition: SpecificationDisposition::InSpecification,
            }],
            reference: ReportReferenceMode::Frozen {
                snapshot: external_snapshot("specifications", 31),
                artifact: artifact("application/json", 32),
            },
        }),
        ReportBlockKind::Prose(ProseBlock {
            style: ProseStyle::Method,
            markdown: "The amplifier was evaluated over the governed corner plan.".to_owned(),
        }),
        ReportBlockKind::ReviewNote(ReviewNoteBlock {
            author: "A. Reviewer".to_owned(),
            status: ReviewNoteStatus::Open,
            message: "Confirm the hot-corner phase margin.".to_owned(),
            created_at_unix_ms: 100,
            resolved_at_unix_ms: None,
        }),
        ReportBlockKind::Evidence(EvidenceBlock {
            title: "Verification receipt".to_owned(),
            summary: "Immutable evidence retained by the verification plan.".to_owned(),
            reference: ReportReferenceMode::Frozen {
                snapshot: evidence_snapshot(40),
                artifact: artifact("application/json", 41),
            },
        }),
    ]
}

#[test]
fn complete_typed_composer_graph_round_trips_and_retains_exact_bindings() {
    let (mut document, _, section_id) = document_with_section();
    let kinds = all_block_kinds();
    let expected_bindings: Vec<_> = kinds
        .iter()
        .filter_map(ReportBlockKind::reference)
        .flat_map(|reference| reference.snapshot().dataset_bindings.iter().copied())
        .collect();
    let receipt = document
        .transact(
            document.revision(),
            kinds
                .into_iter()
                .map(|kind| ReportEdit::AddBlock { section_id, kind })
                .collect(),
            12,
        )
        .unwrap();
    assert_eq!(receipt.created.len(), 8);
    assert_eq!(document.section(section_id).unwrap().blocks().len(), 8);

    let encoded = serde_json::to_vec(&document).unwrap();
    let decoded: ReportDocument = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(decoded, document);
    let decoded_bindings: Vec<_> = decoded
        .pages()
        .iter()
        .flat_map(|page| page.sections())
        .flat_map(|section| section.blocks())
        .filter_map(|block| block.kind().reference())
        .flat_map(|reference| reference.snapshot().dataset_bindings.iter().copied())
        .collect();
    assert_eq!(decoded_bindings, expected_bindings);
}

#[test]
fn transactions_are_optimistic_atomic_and_reject_stale_entity_revisions() {
    let (mut document, page_id, section_id) = document_with_section();
    let before = document.clone();
    let error = document
        .transact(
            ObjectRevision::INITIAL,
            vec![ReportEdit::AddBlock {
                section_id,
                kind: all_block_kinds().remove(0),
            }],
            20,
        )
        .unwrap_err();
    assert!(matches!(
        error,
        ReportError::DocumentRevisionConflict { .. }
    ));
    assert_eq!(document, before);

    let page_revision = document.page(page_id).unwrap().revision();
    let before = document.clone();
    let error = document
        .transact(
            document.revision(),
            vec![
                ReportEdit::UpdatePageTitle {
                    page_id,
                    expected_page_revision: page_revision,
                    title: "Updated".to_owned(),
                },
                ReportEdit::UpdatePageTitle {
                    page_id,
                    expected_page_revision: page_revision,
                    title: "Stale second edit".to_owned(),
                },
            ],
            21,
        )
        .unwrap_err();
    assert!(matches!(error, ReportError::EntityRevisionConflict { .. }));
    assert_eq!(document, before);
}

#[test]
fn linked_and_frozen_reference_currentness_is_explicit_and_auditable() {
    let (mut document, _, section_id) = document_with_section();
    let (linked_snapshot, linked_dataset) = visualization_snapshot(ObjectRevision::INITIAL, 50);
    let linked_source = linked_snapshot.source.clone();
    let linked_digest = linked_snapshot.content_digest;
    let (frozen_snapshot, _) = dataset_snapshot(60);
    let receipt = document
        .transact(
            document.revision(),
            vec![
                ReportEdit::AddBlock {
                    section_id,
                    kind: ReportBlockKind::PlotFigure(PlotFigureBlock {
                        caption: "Gain".to_owned(),
                        alternative_text: "Gain versus frequency.".to_owned(),
                        sizing: FigureSizing::FitWidth,
                        reference: ReportReferenceMode::Linked {
                            snapshot: linked_snapshot.clone(),
                        },
                    }),
                },
                ReportEdit::AddBlock {
                    section_id,
                    kind: ReportBlockKind::Datasheet(DatasheetBlock {
                        title: "Frozen operating point".to_owned(),
                        fields: vec![DatasheetField {
                            key: "id".to_owned(),
                            label: "Drain current".to_owned(),
                            value: "2.4".to_owned(),
                            unit: Some("mA".to_owned()),
                        }],
                        reference: ReportReferenceMode::Frozen {
                            snapshot: frozen_snapshot,
                            artifact: artifact("application/json", 61),
                        },
                    }),
                },
            ],
            30,
        )
        .unwrap();
    let linked_block = match receipt.created[0] {
        ReportEntityRef::Block(id) => id,
        _ => unreachable!(),
    };
    let inventory = ReportReferenceInventory {
        sources: vec![
            ReportReferenceInventoryEntry::new(
                linked_source.clone(),
                Some(ObjectRevision::new(2).unwrap()),
                linked_digest,
                vec![linked_dataset],
            )
            .unwrap(),
        ],
        available_datasets: vec![linked_dataset],
    };
    let first = document.audit_references(&inventory).unwrap();
    assert_eq!(
        first.entries[0].currentness,
        ReportReferenceCurrentness::UpdateAvailable
    );
    assert_eq!(
        first.entries[1].currentness,
        ReportReferenceCurrentness::Frozen
    );
    assert!(!first.is_current_for_sign_off());

    let refreshed = ReportReferenceSnapshot::new(
        linked_source,
        Some(ObjectRevision::new(2).unwrap()),
        linked_digest,
        vec![linked_dataset],
    )
    .unwrap();
    let block_revision = document.block(linked_block).unwrap().revision();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::UpdateBlockReference {
                block_id: linked_block,
                expected_block_revision: block_revision,
                reference: ReportReferenceMode::Linked {
                    snapshot: refreshed,
                },
            }],
            31,
        )
        .unwrap();
    let second = document.audit_references(&inventory).unwrap();
    assert!(second.is_current_for_sign_off());
    assert_ne!(first.audit_digest, second.audit_digest);
}

#[test]
fn audit_distinguishes_missing_source_dataset_and_changed_content() {
    let (mut document, _, section_id) = document_with_section();
    let (snapshot, binding) = visualization_snapshot(ObjectRevision::INITIAL, 70);
    let source = snapshot.source.clone();
    let receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddBlock {
                section_id,
                kind: ReportBlockKind::PlotFigure(PlotFigureBlock {
                    caption: "Noise".to_owned(),
                    alternative_text: "Input-referred noise density.".to_owned(),
                    sizing: FigureSizing::FitWidth,
                    reference: ReportReferenceMode::Linked {
                        snapshot: snapshot.clone(),
                    },
                }),
            }],
            40,
        )
        .unwrap();
    assert!(matches!(receipt.created[0], ReportEntityRef::Block(_)));

    let missing = document
        .audit_references(&ReportReferenceInventory::default())
        .unwrap();
    assert_eq!(
        missing.entries[0].currentness,
        ReportReferenceCurrentness::SourceMissing
    );

    let changed_entry = ReportReferenceInventoryEntry::new(
        source,
        snapshot.source_revision,
        digest(99),
        vec![binding],
    )
    .unwrap();
    let changed = document
        .audit_references(&ReportReferenceInventory {
            sources: vec![changed_entry.clone()],
            available_datasets: vec![binding],
        })
        .unwrap();
    assert_eq!(
        changed.entries[0].currentness,
        ReportReferenceCurrentness::SourceContentChanged
    );

    let missing_dataset = document
        .audit_references(&ReportReferenceInventory {
            sources: vec![ReportReferenceInventoryEntry {
                content_digest: snapshot.content_digest,
                ..changed_entry
            }],
            available_datasets: Vec::new(),
        })
        .unwrap();
    assert_eq!(
        missing_dataset.entries[0].currentness,
        ReportReferenceCurrentness::DatasetMissing
    );
    assert_eq!(
        missing_dataset.entries[0].missing_dataset_bindings,
        vec![binding]
    );
}

#[test]
fn cascade_removal_records_every_identity_and_complete_receipt_chain() {
    let (mut document, page_id, section_id) = document_with_section();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::AddBlock {
                section_id,
                kind: all_block_kinds().remove(5),
            }],
            50,
        )
        .unwrap();
    let page_revision = document.page(page_id).unwrap().revision();
    let receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::Remove {
                entity: ReportEntityRef::Page(page_id),
                expected_entity_revision: page_revision,
            }],
            51,
        )
        .unwrap();
    assert_eq!(receipt.tombstoned.len(), 3);
    assert!(document.pages().is_empty());
    assert_eq!(document.tombstones().len(), 3);
    let decoded: ReportDocument =
        serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
    assert_eq!(decoded, document);
}

#[test]
fn pages_sections_and_blocks_can_be_reordered_without_identity_loss() {
    let (mut document, first_page, first_section) = document_with_section();
    let page_receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddPage {
                title: "Appendix".to_owned(),
            }],
            60,
        )
        .unwrap();
    let second_page = match page_receipt.created[0] {
        ReportEntityRef::Page(id) => id,
        _ => unreachable!(),
    };
    let section_receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddSection {
                page_id: second_page,
                title: "Raw evidence".to_owned(),
            }],
            61,
        )
        .unwrap();
    let second_section = match section_receipt.created[0] {
        ReportEntityRef::Section(id) => id,
        _ => unreachable!(),
    };
    let blocks = document
        .transact(
            document.revision(),
            vec![
                ReportEdit::AddBlock {
                    section_id: first_section,
                    kind: all_block_kinds().remove(5),
                },
                ReportEdit::AddBlock {
                    section_id: first_section,
                    kind: all_block_kinds().remove(6),
                },
            ],
            62,
        )
        .unwrap();
    let first_block = match blocks.created[0] {
        ReportEntityRef::Block(id) => id,
        _ => unreachable!(),
    };
    let first_block_revision = document.block(first_block).unwrap().revision();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::MoveBlock {
                block_id: first_block,
                expected_block_revision: first_block_revision,
                target_section_id: second_section,
                before: None,
            }],
            63,
        )
        .unwrap();
    assert_eq!(
        document.section(second_section).unwrap().blocks()[0].id(),
        first_block
    );

    let section_revision = document.section(first_section).unwrap().revision();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::MoveSection {
                section_id: first_section,
                expected_section_revision: section_revision,
                target_page_id: second_page,
                before: Some(second_section),
            }],
            64,
        )
        .unwrap();
    assert_eq!(
        document.page(second_page).unwrap().sections()[0].id(),
        first_section
    );

    let page_revision = document.page(second_page).unwrap().revision();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::MovePage {
                page_id: second_page,
                expected_page_revision: page_revision,
                before: Some(first_page),
            }],
            65,
        )
        .unwrap();
    assert_eq!(document.pages()[0].id(), second_page);
}

#[test]
fn page_move_handles_forward_one_forward_many_backward_and_end_positions() {
    let mut document = ReportDocument::new("Ordering").unwrap();
    let receipt = document
        .transact(
            document.revision(),
            ["A", "B", "C", "D"]
                .into_iter()
                .map(|title| ReportEdit::AddPage {
                    title: title.to_owned(),
                })
                .collect(),
            66,
        )
        .unwrap();
    let ids: Vec<_> = receipt
        .created
        .iter()
        .map(|entity| match entity {
            ReportEntityRef::Page(id) => *id,
            _ => unreachable!(),
        })
        .collect();
    let titles = |document: &ReportDocument| {
        document
            .pages()
            .iter()
            .map(|page| page.title().to_owned())
            .collect::<Vec<_>>()
    };

    document
        .transact(
            document.revision(),
            vec![ReportEdit::MovePage {
                page_id: ids[0],
                expected_page_revision: document.page(ids[0]).unwrap().revision(),
                before: Some(ids[2]),
            }],
            67,
        )
        .unwrap();
    assert_eq!(titles(&document), ["B", "A", "C", "D"]);

    document
        .transact(
            document.revision(),
            vec![ReportEdit::MovePage {
                page_id: ids[0],
                expected_page_revision: document.page(ids[0]).unwrap().revision(),
                before: None,
            }],
            68,
        )
        .unwrap();
    assert_eq!(titles(&document), ["B", "C", "D", "A"]);

    document
        .transact(
            document.revision(),
            vec![ReportEdit::MovePage {
                page_id: ids[0],
                expected_page_revision: document.page(ids[0]).unwrap().revision(),
                before: Some(ids[2]),
            }],
            69,
        )
        .unwrap();
    assert_eq!(titles(&document), ["B", "A", "C", "D"]);

    document
        .transact(
            document.revision(),
            vec![ReportEdit::MovePage {
                page_id: ids[0],
                expected_page_revision: document.page(ids[0]).unwrap().revision(),
                before: Some(ids[1]),
            }],
            70,
        )
        .unwrap();
    assert_eq!(titles(&document), ["A", "B", "C", "D"]);

    let before = document.clone();
    assert!(matches!(
        document.transact(
            document.revision(),
            vec![ReportEdit::MovePage {
                page_id: ids[2],
                expected_page_revision: document.page(ids[2]).unwrap().revision(),
                before: Some(ids[3]),
            }],
            71,
        ),
        Err(ReportError::NoChanges)
    ));
    assert_eq!(document, before);
}

#[test]
fn section_and_block_moves_use_pre_removal_indices_correctly() {
    let mut document = ReportDocument::new("Nested ordering").unwrap();
    let page_receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddPage {
                title: "Page".to_owned(),
            }],
            71,
        )
        .unwrap();
    let page_id = match page_receipt.created[0] {
        ReportEntityRef::Page(id) => id,
        _ => unreachable!(),
    };
    let sections = document
        .transact(
            document.revision(),
            ["A", "B", "C"]
                .into_iter()
                .map(|title| ReportEdit::AddSection {
                    page_id,
                    title: title.to_owned(),
                })
                .collect(),
            72,
        )
        .unwrap();
    let section_ids: Vec<_> = sections
        .created
        .iter()
        .map(|entity| match entity {
            ReportEntityRef::Section(id) => *id,
            _ => unreachable!(),
        })
        .collect();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::MoveSection {
                section_id: section_ids[0],
                expected_section_revision: document.section(section_ids[0]).unwrap().revision(),
                target_page_id: page_id,
                before: Some(section_ids[2]),
            }],
            73,
        )
        .unwrap();
    assert_eq!(
        document
            .page(page_id)
            .unwrap()
            .sections()
            .iter()
            .map(ReportSection::title)
            .collect::<Vec<_>>(),
        ["B", "A", "C"]
    );
    document
        .transact(
            document.revision(),
            vec![ReportEdit::MoveSection {
                section_id: section_ids[0],
                expected_section_revision: document.section(section_ids[0]).unwrap().revision(),
                target_page_id: page_id,
                before: None,
            }],
            74,
        )
        .unwrap();
    assert_eq!(
        document
            .page(page_id)
            .unwrap()
            .sections()
            .iter()
            .map(ReportSection::title)
            .collect::<Vec<_>>(),
        ["B", "C", "A"]
    );

    let target_section = section_ids[1];
    let blocks = document
        .transact(
            document.revision(),
            (0..3)
                .map(|index| ReportEdit::AddBlock {
                    section_id: target_section,
                    kind: ReportBlockKind::Prose(ProseBlock {
                        style: ProseStyle::Body,
                        markdown: format!("Block {index}"),
                    }),
                })
                .collect(),
            75,
        )
        .unwrap();
    let block_ids: Vec<_> = blocks
        .created
        .iter()
        .map(|entity| match entity {
            ReportEntityRef::Block(id) => *id,
            _ => unreachable!(),
        })
        .collect();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::MoveBlock {
                block_id: block_ids[0],
                expected_block_revision: document.block(block_ids[0]).unwrap().revision(),
                target_section_id: target_section,
                before: Some(block_ids[2]),
            }],
            76,
        )
        .unwrap();
    assert_eq!(
        document
            .section(target_section)
            .unwrap()
            .blocks()
            .iter()
            .map(ReportBlock::id)
            .collect::<Vec<_>>(),
        [block_ids[1], block_ids[0], block_ids[2]]
    );
    document
        .transact(
            document.revision(),
            vec![ReportEdit::MoveBlock {
                block_id: block_ids[0],
                expected_block_revision: document.block(block_ids[0]).unwrap().revision(),
                target_section_id: target_section,
                before: None,
            }],
            77,
        )
        .unwrap();
    assert_eq!(
        document
            .section(target_section)
            .unwrap()
            .blocks()
            .iter()
            .map(ReportBlock::id)
            .collect::<Vec<_>>(),
        [block_ids[1], block_ids[2], block_ids[0]]
    );
}

#[test]
fn document_template_and_page_update_policy_are_transactional() {
    let (mut document, page_id, _) = document_with_section();
    assert_eq!(document.template(), ReportTemplate::ReleaseVerification42);
    assert_eq!(
        document.page(page_id).unwrap().update_policy(),
        ReportPageUpdatePolicy::RefreshLinkedAutomatically
    );
    let page_revision = document.page(page_id).unwrap().revision();
    document
        .transact(
            document.revision(),
            vec![
                ReportEdit::SetTemplate {
                    template: ReportTemplate::ModelQualification,
                },
                ReportEdit::SetPageUpdatePolicy {
                    page_id,
                    expected_page_revision: page_revision,
                    update_policy: ReportPageUpdatePolicy::FreezeSelectedRevision,
                },
            ],
            72,
        )
        .unwrap();
    assert_eq!(document.template(), ReportTemplate::ModelQualification);
    assert_eq!(
        document.page(page_id).unwrap().update_policy(),
        ReportPageUpdatePolicy::FreezeSelectedRevision
    );
}

#[test]
fn page_publication_settings_are_revisioned_and_exactly_bound() {
    let (mut document, page_id, _) = document_with_section();
    let (_, binding) = dataset_snapshot(73);
    let initial_page_revision = document.page(page_id).unwrap().revision();
    let second_page_revision = initial_page_revision.next().unwrap();
    let third_page_revision = second_page_revision.next().unwrap();

    document
        .transact(
            document.revision(),
            vec![
                ReportEdit::SetPageInclusion {
                    page_id,
                    expected_page_revision: initial_page_revision,
                    inclusion: ReportPageInclusion::AppendixOnly,
                },
                ReportEdit::SetPageEvidenceBinding {
                    page_id,
                    expected_page_revision: second_page_revision,
                    evidence_binding: ReportPageEvidenceBinding::ExactDataset { binding },
                },
                ReportEdit::SetPageBlockedGateTextPolicy {
                    page_id,
                    expected_page_revision: third_page_revision,
                    policy: ReportBlockedGateTextPolicy::SummarizeWithLink,
                },
            ],
            73,
        )
        .unwrap();

    let page = document.page(page_id).unwrap();
    assert_eq!(page.inclusion(), ReportPageInclusion::AppendixOnly);
    assert_eq!(
        page.evidence_binding(),
        ReportPageEvidenceBinding::ExactDataset { binding }
    );
    assert_eq!(
        page.blocked_gate_text_policy(),
        ReportBlockedGateTextPolicy::SummarizeWithLink
    );
    let restored: ReportDocument =
        serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
    assert_eq!(restored, document);
}

#[test]
fn report_block_inclusion_is_revision_checked_and_persistent() {
    let (mut document, _, section_id) = document_with_section();
    let receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddBlock {
                section_id,
                kind: ReportBlockKind::Prose(ProseBlock {
                    style: ProseStyle::Body,
                    markdown: "Nominal response remains inside specification.".to_owned(),
                }),
            }],
            75,
        )
        .unwrap();
    let block_id = match receipt.created[0] {
        ReportEntityRef::Block(id) => id,
        _ => unreachable!(),
    };
    let initial_revision = document.block(block_id).unwrap().revision();

    document
        .transact(
            document.revision(),
            vec![ReportEdit::SetBlockEnabled {
                block_id,
                expected_block_revision: initial_revision,
                enabled: false,
            }],
            76,
        )
        .unwrap();

    let block = document.block(block_id).unwrap();
    assert!(!block.enabled());
    assert_eq!(block.revision(), initial_revision.next().unwrap());
    let unchanged = document.clone();
    assert!(matches!(
        document.transact(
            document.revision(),
            vec![ReportEdit::SetBlockEnabled {
                block_id,
                expected_block_revision: initial_revision,
                enabled: true,
            }],
            77,
        ),
        Err(ReportError::EntityRevisionConflict { .. })
    ));
    assert_eq!(document, unchanged);

    let restored: ReportDocument =
        serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
    assert_eq!(restored, document);
}

#[test]
fn add_block_to_sectionless_page_is_one_atomic_revision() {
    let mut document = ReportDocument::new("Atomic report").unwrap();
    let receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddPage {
                title: "Summary".to_owned(),
            }],
            81,
        )
        .unwrap();
    let page_id = match receipt.created[0] {
        ReportEntityRef::Page(id) => id,
        _ => unreachable!(),
    };
    let expected_page_revision = document.page(page_id).unwrap().revision();
    let document_revision = document.revision();
    let receipt = document
        .transact(
            document_revision,
            vec![ReportEdit::AddBlockToPage {
                page_id,
                expected_page_revision,
                kind: ReportBlockKind::Prose(ProseBlock {
                    style: ProseStyle::ExecutiveSummary,
                    markdown: "One atomic page-content insertion.".to_owned(),
                }),
            }],
            82,
        )
        .unwrap();

    assert_eq!(document.revision(), document_revision.next().unwrap());
    assert_eq!(receipt.created.len(), 2);
    assert!(matches!(receipt.created[0], ReportEntityRef::Section(_)));
    assert!(matches!(receipt.created[1], ReportEntityRef::Block(_)));
    let page = document.page(page_id).unwrap();
    assert_eq!(page.sections().len(), 1);
    assert_eq!(page.sections()[0].title(), "Page content");
    assert_eq!(page.sections()[0].blocks().len(), 1);
}

#[test]
fn invalid_tables_notes_sources_and_duplicate_bindings_fail_closed() {
    let (snapshot, binding) = dataset_snapshot(80);
    let invalid_table = ReportBlockKind::DataTable(DataTableBlock {
        title: "Broken".to_owned(),
        columns: vec![TableColumn {
            key: "value".to_owned(),
            heading: "Value".to_owned(),
            unit: None,
        }],
        rows: vec![vec![TableCell::Integer(1), TableCell::Integer(2)]],
        reference: ReportReferenceMode::Linked { snapshot },
    });
    assert!(invalid_table.validate().is_err());

    let invalid_note = ReportBlockKind::ReviewNote(ReviewNoteBlock {
        author: "Reviewer".to_owned(),
        status: ReviewNoteStatus::Accepted,
        message: "Accepted without a time.".to_owned(),
        created_at_unix_ms: 1,
        resolved_at_unix_ms: None,
    });
    assert!(invalid_note.validate().is_err());

    let wrong_plot = ReportBlockKind::PlotFigure(PlotFigureBlock {
        caption: "Wrong source".to_owned(),
        alternative_text: "This must fail source-kind validation.".to_owned(),
        sizing: FigureSizing::Natural,
        reference: ReportReferenceMode::Linked {
            snapshot: dataset_snapshot(81).0,
        },
    });
    assert!(matches!(
        wrong_plot.validate(),
        Err(ReportError::InvalidReferenceKind { .. })
    ));

    assert!(matches!(
        ReportReferenceSnapshot::new(
            ReportSourceId::Dataset {
                dataset_id: binding.dataset_id
            },
            None,
            binding.content_digest,
            vec![binding, binding],
        ),
        Err(ReportError::DuplicateDatasetBinding(_))
    ));
}

#[test]
fn serde_rejects_broken_receipts_nil_ids_and_unsupported_versions() {
    let (mut document, _, section_id) = document_with_section();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::AddBlock {
                section_id,
                kind: all_block_kinds().remove(5),
            }],
            70,
        )
        .unwrap();
    let value = serde_json::to_value(&document).unwrap();

    let mut broken = value.clone();
    broken["receipts"][0]["committed_document_revision"] = serde_json::json!(99);
    assert!(serde_json::from_value::<ReportDocument>(broken).is_err());

    let mut nil = value.clone();
    nil["pages"][0]["id"] = serde_json::json!(Uuid::nil());
    assert!(serde_json::from_value::<ReportDocument>(nil).is_err());

    let mut future = value;
    future["schema_version"] = serde_json::json!(u16::MAX);
    assert!(serde_json::from_value::<ReportDocument>(future).is_err());
}

#[test]
fn receipt_validation_rejects_changes_before_creation_and_after_tombstone() {
    let mut future_change = ReportDocument::new("Temporal audit").unwrap();
    future_change
        .transact(
            future_change.revision(),
            vec![ReportEdit::SetDocumentTitle {
                title: "Temporal audit updated".to_owned(),
            }],
            73,
        )
        .unwrap();
    let created = future_change
        .transact(
            future_change.revision(),
            vec![ReportEdit::AddPage {
                title: "Created later".to_owned(),
            }],
            74,
        )
        .unwrap();
    let page = created.created[0];
    let mut corrupted = serde_json::to_value(&future_change).unwrap();
    corrupted["receipts"][0]["changed"] = serde_json::json!([page]);
    assert!(serde_json::from_value::<ReportDocument>(corrupted).is_err());

    let page_id = match page {
        ReportEntityRef::Page(id) => id,
        _ => unreachable!(),
    };
    let page_revision = future_change.page(page_id).unwrap().revision();
    future_change
        .transact(
            future_change.revision(),
            vec![ReportEdit::Remove {
                entity: page,
                expected_entity_revision: page_revision,
            }],
            75,
        )
        .unwrap();
    future_change
        .transact(
            future_change.revision(),
            vec![ReportEdit::SetDocumentTitle {
                title: "Changed after deletion".to_owned(),
            }],
            76,
        )
        .unwrap();
    let mut corrupted = serde_json::to_value(&future_change).unwrap();
    corrupted["receipts"][3]["changed"] = serde_json::json!([page]);
    assert!(serde_json::from_value::<ReportDocument>(corrupted).is_err());
}

#[test]
fn frozen_artifacts_use_bounded_authenticated_base64_wire_payloads() {
    let artifact = FrozenReportArtifact::new("image/svg+xml", b"<svg/>".to_vec()).unwrap();
    let encoded = serde_json::to_value(&artifact).unwrap();
    assert!(encoded["payload_base64"].is_string());
    assert!(encoded.get("payload").is_none());
    let decoded: FrozenReportArtifact = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(decoded, artifact);

    let mut tampered = encoded;
    tampered["payload_base64"] = serde_json::json!(BASE64_STANDARD.encode(b"<svg>x</svg>"));
    assert!(serde_json::from_value::<FrozenReportArtifact>(tampered).is_err());

    let oversized = "A".repeat(MAX_FROZEN_ARTIFACT_BASE64_BYTES + 1);
    let value = serde_json::json!({
        "media_type": "application/octet-stream",
        "payload_base64": oversized,
        "content_digest": digest(1),
    });
    assert!(serde_json::from_value::<FrozenReportArtifact>(value).is_err());
}

#[test]
fn aggregate_frozen_payload_capacity_uses_checked_addition() {
    assert!(
        validate_aggregate_frozen_payload_bytes([
            MAX_FROZEN_ARTIFACT_BYTES_PER_DOCUMENT / 2,
            MAX_FROZEN_ARTIFACT_BYTES_PER_DOCUMENT / 2,
        ])
        .is_ok()
    );
    assert!(matches!(
        validate_aggregate_frozen_payload_bytes([MAX_FROZEN_ARTIFACT_BYTES_PER_DOCUMENT, 1,]),
        Err(ReportError::CapacityExceeded(_))
    ));
    assert!(matches!(
        validate_aggregate_frozen_payload_bytes([usize::MAX, 1]),
        Err(ReportError::CapacityExceeded(_))
    ));
}

#[test]
fn frozen_plot_is_publishable_without_inventory_and_artifact_changes_affect_audit() {
    let (mut document, _, section_id) = document_with_section();
    let block_receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddBlock {
                section_id,
                kind: all_block_kinds().remove(0),
            }],
            77,
        )
        .unwrap();
    let block_id = match block_receipt.created[0] {
        ReportEntityRef::Block(id) => id,
        _ => unreachable!(),
    };
    let first = document
        .audit_references(&ReportReferenceInventory::default())
        .unwrap();
    assert!(first.is_current_for_sign_off());
    assert_eq!(
        first.entries[0].frozen_artifact_media_type.as_deref(),
        Some("image/svg+xml")
    );
    assert!(first.entries[0].frozen_artifact_digest.is_some());

    let block = document.block(block_id).unwrap();
    let block_revision = block.revision();
    let mut changed_kind = block.kind().clone();
    let ReportBlockKind::PlotFigure(figure) = &mut changed_kind else {
        unreachable!()
    };
    let ReportReferenceMode::Frozen { artifact, .. } = &mut figure.reference else {
        unreachable!()
    };
    *artifact = FrozenReportArtifact::new(
        "image/svg+xml",
        b"<svg xmlns=\"http://www.w3.org/2000/svg\"><path/></svg>".to_vec(),
    )
    .unwrap();
    let mut changed = document.clone();
    changed
        .transact(
            changed.revision(),
            vec![ReportEdit::ReplaceBlock {
                block_id,
                expected_block_revision: block_revision,
                kind: changed_kind,
            }],
            78,
        )
        .unwrap();
    let second = changed
        .audit_references(&ReportReferenceInventory::default())
        .unwrap();
    assert_ne!(
        first.entries[0].frozen_artifact_digest,
        second.entries[0].frozen_artifact_digest
    );
    assert_ne!(first.audit_digest, second.audit_digest);
}

#[test]
fn version_one_initial_snapshots_migrate_without_fabricating_history() {
    let page = ReportPage {
        id: ReportPageId::new(),
        created_at_document_revision: ObjectRevision::INITIAL,
        revision: ObjectRevision::INITIAL,
        title: "Imported page".to_owned(),
        update_policy: ReportPageUpdatePolicy::RefreshLinkedAutomatically,
        inclusion: ReportPageInclusion::Included,
        evidence_binding: ReportPageEvidenceBinding::Unbound,
        blocked_gate_text_policy: ReportBlockedGateTextPolicy::VerbatimFromSource,
        sections: vec![ReportSection {
            id: ReportSectionId::new(),
            created_at_document_revision: ObjectRevision::INITIAL,
            revision: ObjectRevision::INITIAL,
            title: "Imported section".to_owned(),
            blocks: vec![ReportBlock {
                id: ReportBlockId::new(),
                created_at_document_revision: ObjectRevision::INITIAL,
                revision: ObjectRevision::INITIAL,
                enabled: true,
                kind: all_block_kinds().remove(5),
            }],
        }],
    };
    let legacy = ReportDocument {
        schema_version: 1,
        id: ResultDocumentId::new(),
        revision: ObjectRevision::INITIAL,
        title: "Legacy report".to_owned(),
        template: ReportTemplate::ReleaseVerification42,
        output_formats: ReportOutputFormats::default(),
        publication_profile: ReportPublicationProfile::default(),
        pages: vec![page],
        receipts: Vec::new(),
        tombstones: Vec::new(),
        legacy_origin_entities: Vec::new(),
        revision_history: ReportRevisionHistory {
            origin: ReportRevisionHistoryOrigin::Native,
            records: Vec::new(),
        },
    };
    let mut legacy_value = serde_json::to_value(&legacy).unwrap();
    legacy_value
        .as_object_mut()
        .unwrap()
        .remove("revision_history");
    let mut migrated: ReportDocument = serde_json::from_value(legacy_value).unwrap();
    assert_eq!(migrated.schema_version(), ReportDocument::SCHEMA_VERSION);
    assert_eq!(migrated.legacy_origin_entities.len(), 3);
    migrated
        .transact(
            migrated.revision(),
            vec![ReportEdit::SetDocumentTitle {
                title: "Migrated report".to_owned(),
            }],
            80,
        )
        .unwrap();

    let mut unsafe_legacy = serde_json::to_value(&legacy).unwrap();
    unsafe_legacy
        .as_object_mut()
        .unwrap()
        .remove("revision_history");
    unsafe_legacy["revision"] = serde_json::json!(2);
    assert!(serde_json::from_value::<ReportDocument>(unsafe_legacy).is_err());
}

#[test]
fn revision_history_reconstructs_complete_source_at_every_native_revision() {
    let mut document = ReportDocument::new("Verification report").unwrap();
    let initial_id = document.revision_history().records()[0].revision_identity();
    document
        .transact_with_context(
            document.revision(),
            vec![ReportEdit::AddPage {
                title: "Summary".to_owned(),
            }],
            101,
            "james@example.com",
            "Add governed report summary page",
        )
        .unwrap();
    document
        .transact_with_context(
            document.revision(),
            vec![ReportEdit::SetDocumentTitle {
                title: "Release verification".to_owned(),
            }],
            102,
            "james@example.com",
            "Name the release report",
        )
        .unwrap();

    let records = document.revision_history().records();
    assert_eq!(
        document.revision_history().origin(),
        ReportRevisionHistoryOrigin::Native
    );
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].prior_revision_identity(), None);
    assert_eq!(records[1].prior_revision_identity(), Some(initial_id));
    assert_eq!(
        records[1].prior_record_digest(),
        Some(records[0].record_digest())
    );
    assert_eq!(
        records[2].prior_revision_identity(),
        Some(records[1].revision_identity())
    );
    assert_eq!(
        records[2].prior_record_digest(),
        Some(records[1].record_digest())
    );
    assert_eq!(records[1].actor(), "james@example.com");
    assert_eq!(
        records[1].revision_note(),
        "Add governed report summary page"
    );
    assert_eq!(records[1].timestamp_unix_ms(), 101);
    assert!(records[1].snapshot_serialized_bytes() > 0);
    assert_ne!(records[1].snapshot_digest(), records[2].snapshot_digest());

    let initial = document
        .reconstruct_revision(document.id(), ObjectRevision::INITIAL)
        .unwrap();
    assert_eq!(initial.title(), "Verification report");
    assert!(initial.pages().is_empty());
    assert!(initial.receipts().is_empty());
    assert_eq!(initial.revision_history().records().len(), 1);

    let revision_two = document
        .reconstruct_revision(document.id(), ObjectRevision::new(2).unwrap())
        .unwrap();
    assert_eq!(revision_two.title(), "Verification report");
    assert_eq!(revision_two.pages().len(), 1);
    assert_eq!(revision_two.receipts().len(), 1);
    assert_eq!(revision_two.revision_history().records().len(), 2);

    let wrong_document = ResultDocumentId::new();
    assert!(
        document
            .revision_record(wrong_document, ObjectRevision::INITIAL)
            .is_none()
    );
    assert!(matches!(
        document.reconstruct_revision(wrong_document, ObjectRevision::INITIAL),
        Err(ReportError::RevisionNotRetained { .. })
    ));

    let encoded = serde_json::to_vec(&document).unwrap();
    let restored: ReportDocument = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(restored, document);
    assert_eq!(
        restored.revision_history().records()[2].record_digest(),
        records[2].record_digest()
    );
}

#[test]
fn revision_history_deserialization_fails_closed_on_snapshot_or_chain_tampering() {
    let mut document = ReportDocument::new("Audit report").unwrap();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::AddPage {
                title: "Evidence".to_owned(),
            }],
            200,
        )
        .unwrap();
    let value = serde_json::to_value(&document).unwrap();

    let mut changed_snapshot = value.clone();
    changed_snapshot["revision_history"]["records"][1]["snapshot"]["title"] =
        serde_json::json!("Tampered report");
    assert!(serde_json::from_value::<ReportDocument>(changed_snapshot).is_err());

    let mut changed_actor = value.clone();
    changed_actor["revision_history"]["records"][1]["actor"] = serde_json::json!("other-actor");
    assert!(serde_json::from_value::<ReportDocument>(changed_actor).is_err());

    let mut falsified_origin = value.clone();
    falsified_origin["revision_history"]["origin"] =
        serde_json::json!("imported-schema-one-baseline");
    assert!(serde_json::from_value::<ReportDocument>(falsified_origin).is_err());

    let mut broken_chain = value.clone();
    broken_chain["revision_history"]["records"][1]["prior_revision_identity"] =
        serde_json::json!(ReportRevisionId::new());
    assert!(serde_json::from_value::<ReportDocument>(broken_chain).is_err());

    let mut false_size = value.clone();
    false_size["revision_history"]["records"][1]["snapshot_serialized_bytes"] =
        serde_json::json!(1);
    assert!(serde_json::from_value::<ReportDocument>(false_size).is_err());

    let mut unknown_current_field = value.clone();
    unknown_current_field["future_history_contract"] = serde_json::json!({"version": 4});
    assert!(serde_json::from_value::<ReportDocument>(unknown_current_field).is_err());

    let mut missing_history = value;
    missing_history
        .as_object_mut()
        .unwrap()
        .remove("revision_history");
    assert!(serde_json::from_value::<ReportDocument>(missing_history).is_err());

    let mut downgraded_with_null_history = serde_json::to_value(&document).unwrap();
    downgraded_with_null_history["schema_version"] = serde_json::json!(2);
    downgraded_with_null_history["revision_history"] = serde_json::Value::Null;
    assert!(serde_json::from_value::<ReportDocument>(downgraded_with_null_history).is_err());
}

#[test]
fn schema_two_migration_retains_an_explicit_current_source_baseline() {
    let (document, _, _) = document_with_section();
    assert_eq!(document.revision().get(), 3);
    let expected_pages = document.pages().to_vec();
    let expected_receipts = document.receipts().to_vec();
    let mut schema_two = serde_json::to_value(&document).unwrap();
    schema_two["schema_version"] = serde_json::json!(2);
    schema_two
        .as_object_mut()
        .unwrap()
        .remove("revision_history");

    let first_migration: ReportDocument = serde_json::from_value(schema_two.clone()).unwrap();
    let second_migration: ReportDocument = serde_json::from_value(schema_two).unwrap();
    assert_eq!(first_migration, second_migration);
    assert_eq!(
        serde_json::to_vec(&first_migration).unwrap(),
        serde_json::to_vec(&second_migration).unwrap()
    );
    let mut migrated = first_migration;
    assert_eq!(migrated.schema_version(), ReportDocument::SCHEMA_VERSION);
    assert_eq!(migrated.pages(), expected_pages);
    assert_eq!(migrated.receipts(), expected_receipts);
    assert_eq!(
        migrated.revision_history().origin(),
        ReportRevisionHistoryOrigin::ImportedSchemaTwoBaseline
    );
    assert_eq!(migrated.revision_history().records().len(), 1);
    assert_eq!(
        migrated.revision_history().records()[0].revision(),
        migrated.revision()
    );
    assert_eq!(
        migrated
            .reconstruct_revision(migrated.id(), migrated.revision())
            .unwrap()
            .pages(),
        expected_pages
    );

    migrated
        .transact_with_context(
            migrated.revision(),
            vec![ReportEdit::SetDocumentTitle {
                title: "Migrated and edited".to_owned(),
            }],
            300,
            "migration-reviewer",
            "Continue editing from imported baseline",
        )
        .unwrap();
    assert_eq!(migrated.revision_history().records().len(), 2);
    assert_eq!(
        migrated.revision_history().records()[1].prior_revision_identity(),
        Some(migrated.revision_history().records()[0].revision_identity())
    );
}

#[test]
fn schema_three_migration_preserves_authenticated_default_page_policies() {
    let (document, _, _) = document_with_section();
    let mut schema_three = serde_json::to_value(&document).unwrap();
    schema_three["schema_version"] = serde_json::json!(3);

    let migrated: ReportDocument = serde_json::from_value(schema_three).unwrap();

    assert_eq!(migrated, document);
    assert_eq!(migrated.schema_version(), ReportDocument::SCHEMA_VERSION);
    for page in migrated.pages() {
        assert_eq!(page.inclusion(), ReportPageInclusion::Included);
        assert_eq!(page.evidence_binding(), ReportPageEvidenceBinding::Unbound);
        assert_eq!(
            page.blocked_gate_text_policy(),
            ReportBlockedGateTextPolicy::VerbatimFromSource
        );
    }
}

#[test]
fn schema_three_migration_rejects_mislabeled_schema_four_page_policies() {
    let (mut document, page_id, _) = document_with_section();
    let page_revision = document.page(page_id).unwrap().revision();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::SetPageInclusion {
                page_id,
                expected_page_revision: page_revision,
                inclusion: ReportPageInclusion::AppendixOnly,
            }],
            74,
        )
        .unwrap();
    let mut mislabeled = serde_json::to_value(document).unwrap();
    mislabeled["schema_version"] = serde_json::json!(3);

    assert!(matches!(
        serde_json::from_value::<ReportDocument>(mislabeled),
        Err(_)
    ));
}

#[test]
fn schema_four_migration_preserves_authenticated_default_block_inclusion() {
    let (mut document, _, section_id) = document_with_section();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::AddBlock {
                section_id,
                kind: ReportBlockKind::Prose(ProseBlock {
                    style: ProseStyle::Method,
                    markdown: "Retained schema-four block.".to_owned(),
                }),
            }],
            78,
        )
        .unwrap();
    let mut schema_four = serde_json::to_value(&document).unwrap();
    schema_four["schema_version"] = serde_json::json!(4);

    let migrated: ReportDocument = serde_json::from_value(schema_four).unwrap();

    assert_eq!(migrated, document);
    assert_eq!(migrated.schema_version(), ReportDocument::SCHEMA_VERSION);
    assert!(
        migrated
            .pages()
            .iter()
            .flat_map(|page| page.sections())
            .flat_map(|section| section.blocks())
            .all(ReportBlock::enabled)
    );
}

#[test]
fn schema_four_migration_rejects_mislabeled_disabled_blocks() {
    let (mut document, _, section_id) = document_with_section();
    let receipt = document
        .transact(
            document.revision(),
            vec![ReportEdit::AddBlock {
                section_id,
                kind: ReportBlockKind::Prose(ProseBlock {
                    style: ProseStyle::Warning,
                    markdown: "Disabled only in schema five.".to_owned(),
                }),
            }],
            79,
        )
        .unwrap();
    let block_id = match receipt.created[0] {
        ReportEntityRef::Block(id) => id,
        _ => unreachable!(),
    };
    let block_revision = document.block(block_id).unwrap().revision();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::SetBlockEnabled {
                block_id,
                expected_block_revision: block_revision,
                enabled: false,
            }],
            80,
        )
        .unwrap();
    let mut mislabeled = serde_json::to_value(document).unwrap();
    mislabeled["schema_version"] = serde_json::json!(4);

    assert!(serde_json::from_value::<ReportDocument>(mislabeled).is_err());
}

#[test]
fn publication_policy_is_revisioned_and_reconstructable() {
    let mut document = ReportDocument::new("Publication policy").unwrap();
    let initial_revision = document.revision();
    let output_formats = ReportOutputFormats {
        pdf_a: true,
        html_bundle: false,
        canonical_json: true,
        selected_csv: true,
    };
    let publication_profile = ReportPublicationProfile {
        template: ReportPublicationTemplate::CustomerDatasheet,
        page_size: ReportPublicationPageSize::A3Landscape,
        draft_marking: ReportDraftMarking::NeverWatermark,
        numbering: ReportPageNumbering::ContinuousPageNumbers,
        table_precision: ReportTablePrecision::FullStoredF64,
    };

    document
        .transact_with_context(
            document.revision(),
            vec![
                ReportEdit::SetOutputFormats { output_formats },
                ReportEdit::SetPublicationProfile {
                    publication_profile,
                },
            ],
            81,
            "publication-editor",
            "Set exact report publication policy",
        )
        .unwrap();

    assert_eq!(document.output_formats(), output_formats);
    assert_eq!(document.publication_profile(), publication_profile);
    let prior = document
        .reconstruct_revision(document.id(), initial_revision)
        .unwrap();
    assert_eq!(prior.output_formats(), ReportOutputFormats::default());
    assert_eq!(
        prior.publication_profile(),
        ReportPublicationProfile::default()
    );
    let current = document
        .reconstruct_revision(document.id(), document.revision())
        .unwrap();
    assert_eq!(current.output_formats(), output_formats);
    assert_eq!(current.publication_profile(), publication_profile);
}

#[test]
fn report_rejects_disabling_every_output_format_atomically() {
    let mut document = ReportDocument::new("Publication policy").unwrap();
    let before = document.clone();
    let result = document.transact(
        document.revision(),
        vec![ReportEdit::SetOutputFormats {
            output_formats: ReportOutputFormats {
                pdf_a: false,
                html_bundle: false,
                canonical_json: false,
                selected_csv: false,
            },
        }],
        82,
    );

    assert!(matches!(
        result,
        Err(ReportError::InvalidValue {
            field: "report-document.output-formats",
            ..
        })
    ));
    assert_eq!(document, before);
}

#[test]
fn schema_five_migration_preserves_authenticated_default_publication_policy() {
    let document = ReportDocument::new("Schema five report").unwrap();
    let mut schema_five = serde_json::to_value(&document).unwrap();
    schema_five["schema_version"] = serde_json::json!(5);

    let migrated: ReportDocument = serde_json::from_value(schema_five).unwrap();

    assert_eq!(migrated, document);
    assert_eq!(migrated.schema_version(), ReportDocument::SCHEMA_VERSION);
    assert_eq!(migrated.output_formats(), ReportOutputFormats::default());
    assert_eq!(
        migrated.publication_profile(),
        ReportPublicationProfile::default()
    );
}

#[test]
fn schema_five_migration_rejects_mislabeled_publication_policy() {
    let mut document = ReportDocument::new("Schema six report").unwrap();
    document
        .transact(
            document.revision(),
            vec![ReportEdit::SetOutputFormats {
                output_formats: ReportOutputFormats {
                    selected_csv: true,
                    ..ReportOutputFormats::default()
                },
            }],
            83,
        )
        .unwrap();
    let mut mislabeled = serde_json::to_value(document).unwrap();
    mislabeled["schema_version"] = serde_json::json!(5);

    assert!(serde_json::from_value::<ReportDocument>(mislabeled).is_err());
}

#[test]
fn revision_history_snapshot_capacity_uses_checked_aggregate_bytes() {
    assert!(
        validate_revision_history_snapshot_capacity([
            MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES / 2,
            MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES / 2,
        ])
        .is_ok()
    );
    assert!(matches!(
        validate_revision_history_snapshot_capacity([
            MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES,
            1,
        ]),
        Err(ReportError::CapacityExceeded(
            "report source revision snapshot bytes"
        ))
    ));
    assert!(matches!(
        validate_revision_history_snapshot_capacity([u64::MAX, 1]),
        Err(ReportError::CapacityExceeded(
            "report source revision snapshot bytes"
        ))
    ));
}

#[test]
fn streaming_snapshot_digest_matches_canonical_json_bytes() {
    let document = ReportDocument::new("Streaming digest").unwrap();
    let snapshot = document.current_snapshot();
    let canonical = serde_json::to_vec(&snapshot).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-report-source-snapshot-v1\0");
    hasher.update(&canonical);
    let (digest, serialized_bytes) = report_snapshot_digest_and_size(&snapshot).unwrap();
    assert_eq!(serialized_bytes, canonical.len() as u64);
    assert_eq!(digest, ContentDigest::from_bytes(hasher.finalize().into()));
}

#[test]
fn revision_history_sequence_deserializer_enforces_record_limit() {
    let document = ReportDocument::new("Bounded history").unwrap();
    let mut value = serde_json::to_value(&document).unwrap();
    let record = value["revision_history"]["records"][0].clone();
    value["revision_history"]["records"] =
        serde_json::Value::Array(vec![record; MAX_REPORT_REVISION_HISTORY_RECORDS + 1]);
    assert!(serde_json::from_value::<ReportDocument>(value).is_err());
}

#[test]
fn revision_context_validation_fails_atomically() {
    let mut document = ReportDocument::new("Atomic report").unwrap();
    let before = document.clone();
    let error = document
        .transact_with_context(
            document.revision(),
            vec![ReportEdit::SetDocumentTitle {
                title: "Must not commit".to_owned(),
            }],
            400,
            " ",
            "Invalid actor must reject the transaction",
        )
        .unwrap_err();
    assert!(matches!(
        error,
        ReportError::InvalidValue {
            field: "report-revision.actor",
            ..
        }
    ));
    assert_eq!(document, before);
}

#[test]
fn reference_audit_digest_is_deterministic_for_identical_inputs() {
    let (document, _, _) = document_with_section();
    let inventory = ReportReferenceInventory::default();
    let first = document.audit_references(&inventory).unwrap();
    let second = document.audit_references(&inventory).unwrap();
    assert_eq!(first.audit_digest, second.audit_digest);
    assert!(first.is_current_for_sign_off());
}
