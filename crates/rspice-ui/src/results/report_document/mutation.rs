//! Applying an edit to a report, and proving the result is still coherent.
//!
//! Every mutation is validated against the document it would produce and
//! leaves a receipt, and every removal leaves a tombstone — so the revision
//! history can be replayed to explain any entity that is no longer present.
//! Validation checks that history rather than trusting it: a receipt or
//! tombstone that does not correspond to a real entity is an error, not noise.

use super::*;

impl ReportDocument {
    pub(super) fn apply_edit(
        &mut self,
        edit: ReportEdit,
        committed_revision: ObjectRevision,
        created: &mut Vec<ReportEntityRef>,
        changed: &mut Vec<ReportEntityRef>,
        tombstoned: &mut Vec<ReportEntityRef>,
    ) -> Result<(), ReportError> {
        match edit {
            ReportEdit::SetDocumentTitle { title } => {
                validate_label("report-document.title", &title, 512)?;
                if self.title == title {
                    return Err(ReportError::NoChanges);
                }
                self.title = title;
            }
            ReportEdit::SetTemplate { template } => {
                if self.template == template {
                    return Err(ReportError::NoChanges);
                }
                self.template = template;
            }
            ReportEdit::AddPage { title } => {
                validate_label("report-page.title", &title, 512)?;
                if self.pages.len() >= MAX_PAGES {
                    return Err(ReportError::CapacityExceeded("report pages"));
                }
                let page = ReportPage {
                    id: ReportPageId::new(),
                    created_at_document_revision: committed_revision,
                    revision: ObjectRevision::INITIAL,
                    title,
                    update_policy: ReportPageUpdatePolicy::RefreshLinkedAutomatically,
                    inclusion: ReportPageInclusion::Included,
                    evidence_binding: ReportPageEvidenceBinding::Unbound,
                    blocked_gate_text_policy: ReportBlockedGateTextPolicy::VerbatimFromSource,
                    sections: Vec::new(),
                };
                created.push(ReportEntityRef::Page(page.id));
                self.pages.push(page);
            }
            ReportEdit::UpdatePageTitle {
                page_id,
                expected_page_revision,
                title,
            } => {
                validate_label("report-page.title", &title, 512)?;
                let page = self.page_mut(page_id)?;
                require_entity_revision(
                    ReportEntityRef::Page(page_id),
                    expected_page_revision,
                    page.revision,
                )?;
                if page.title == title {
                    return Err(ReportError::NoChanges);
                }
                page.title = title;
                page.revision = page.revision.next()?;
                changed.push(ReportEntityRef::Page(page_id));
            }
            ReportEdit::SetPageUpdatePolicy {
                page_id,
                expected_page_revision,
                update_policy,
            } => {
                let page = self.page_mut(page_id)?;
                require_entity_revision(
                    ReportEntityRef::Page(page_id),
                    expected_page_revision,
                    page.revision,
                )?;
                if page.update_policy == update_policy {
                    return Err(ReportError::NoChanges);
                }
                page.update_policy = update_policy;
                page.revision = page.revision.next()?;
                changed.push(ReportEntityRef::Page(page_id));
            }
            ReportEdit::SetPageInclusion {
                page_id,
                expected_page_revision,
                inclusion,
            } => {
                let page = self.page_mut(page_id)?;
                require_entity_revision(
                    ReportEntityRef::Page(page_id),
                    expected_page_revision,
                    page.revision,
                )?;
                if page.inclusion == inclusion {
                    return Err(ReportError::NoChanges);
                }
                page.inclusion = inclusion;
                page.revision = page.revision.next()?;
                changed.push(ReportEntityRef::Page(page_id));
            }
            ReportEdit::SetPageEvidenceBinding {
                page_id,
                expected_page_revision,
                evidence_binding,
            } => {
                let page = self.page_mut(page_id)?;
                require_entity_revision(
                    ReportEntityRef::Page(page_id),
                    expected_page_revision,
                    page.revision,
                )?;
                if page.evidence_binding == evidence_binding {
                    return Err(ReportError::NoChanges);
                }
                page.evidence_binding = evidence_binding;
                page.revision = page.revision.next()?;
                changed.push(ReportEntityRef::Page(page_id));
            }
            ReportEdit::SetPageBlockedGateTextPolicy {
                page_id,
                expected_page_revision,
                policy,
            } => {
                let page = self.page_mut(page_id)?;
                require_entity_revision(
                    ReportEntityRef::Page(page_id),
                    expected_page_revision,
                    page.revision,
                )?;
                if page.blocked_gate_text_policy == policy {
                    return Err(ReportError::NoChanges);
                }
                page.blocked_gate_text_policy = policy;
                page.revision = page.revision.next()?;
                changed.push(ReportEntityRef::Page(page_id));
            }
            ReportEdit::MovePage {
                page_id,
                expected_page_revision,
                before,
            } => self.move_page(page_id, expected_page_revision, before, changed)?,
            ReportEdit::AddSection { page_id, title } => {
                validate_label("report-section.title", &title, 512)?;
                if self.section_count() >= MAX_SECTIONS_TOTAL {
                    return Err(ReportError::CapacityExceeded("report sections"));
                }
                let page = self.page_mut(page_id)?;
                if page.sections.len() >= MAX_SECTIONS_PER_PAGE {
                    return Err(ReportError::CapacityExceeded("sections per report page"));
                }
                let section = ReportSection {
                    id: ReportSectionId::new(),
                    created_at_document_revision: committed_revision,
                    revision: ObjectRevision::INITIAL,
                    title,
                    blocks: Vec::new(),
                };
                created.push(ReportEntityRef::Section(section.id));
                page.sections.push(section);
                page.revision = page.revision.next()?;
                changed.push(ReportEntityRef::Page(page_id));
            }
            ReportEdit::UpdateSectionTitle {
                section_id,
                expected_section_revision,
                title,
            } => {
                validate_label("report-section.title", &title, 512)?;
                let (_, _, section) = self.section_mut(section_id)?;
                require_entity_revision(
                    ReportEntityRef::Section(section_id),
                    expected_section_revision,
                    section.revision,
                )?;
                if section.title == title {
                    return Err(ReportError::NoChanges);
                }
                section.title = title;
                section.revision = section.revision.next()?;
                changed.push(ReportEntityRef::Section(section_id));
            }
            ReportEdit::MoveSection {
                section_id,
                expected_section_revision,
                target_page_id,
                before,
            } => self.move_section(
                section_id,
                expected_section_revision,
                target_page_id,
                before,
                changed,
            )?,
            ReportEdit::AddBlock { section_id, kind } => {
                kind.validate()?;
                if self.block_count() >= MAX_BLOCKS_TOTAL {
                    return Err(ReportError::CapacityExceeded("report content blocks"));
                }
                let (_, _, section) = self.section_mut(section_id)?;
                if section.blocks.len() >= MAX_BLOCKS_PER_SECTION {
                    return Err(ReportError::CapacityExceeded(
                        "content blocks per report section",
                    ));
                }
                let block = ReportBlock {
                    id: ReportBlockId::new(),
                    created_at_document_revision: committed_revision,
                    revision: ObjectRevision::INITIAL,
                    enabled: true,
                    kind,
                };
                created.push(ReportEntityRef::Block(block.id));
                section.blocks.push(block);
                section.revision = section.revision.next()?;
                changed.push(ReportEntityRef::Section(section_id));
            }
            ReportEdit::AddBlockToPage {
                page_id,
                expected_page_revision,
                kind,
            } => {
                kind.validate()?;
                if self.block_count() >= MAX_BLOCKS_TOTAL {
                    return Err(ReportError::CapacityExceeded("report content blocks"));
                }
                let page_index = self
                    .pages
                    .iter()
                    .position(|page| page.id == page_id)
                    .ok_or(ReportError::EntityNotFound(ReportEntityRef::Page(page_id)))?;
                require_entity_revision(
                    ReportEntityRef::Page(page_id),
                    expected_page_revision,
                    self.pages[page_index].revision,
                )?;
                if self.pages[page_index].sections.is_empty() {
                    if self.section_count() >= MAX_SECTIONS_TOTAL {
                        return Err(ReportError::CapacityExceeded("report sections"));
                    }
                    let block = ReportBlock {
                        id: ReportBlockId::new(),
                        created_at_document_revision: committed_revision,
                        revision: ObjectRevision::INITIAL,
                        enabled: true,
                        kind,
                    };
                    let section = ReportSection {
                        id: ReportSectionId::new(),
                        created_at_document_revision: committed_revision,
                        revision: ObjectRevision::INITIAL,
                        title: "Page content".to_owned(),
                        blocks: vec![block],
                    };
                    created.push(ReportEntityRef::Section(section.id));
                    created.push(ReportEntityRef::Block(section.blocks[0].id));
                    self.pages[page_index].sections.push(section);
                    self.pages[page_index].revision = self.pages[page_index].revision.next()?;
                    changed.push(ReportEntityRef::Page(page_id));
                } else {
                    let section = &mut self.pages[page_index].sections[0];
                    if section.blocks.len() >= MAX_BLOCKS_PER_SECTION {
                        return Err(ReportError::CapacityExceeded(
                            "content blocks per report section",
                        ));
                    }
                    let block = ReportBlock {
                        id: ReportBlockId::new(),
                        created_at_document_revision: committed_revision,
                        revision: ObjectRevision::INITIAL,
                        enabled: true,
                        kind,
                    };
                    created.push(ReportEntityRef::Block(block.id));
                    section.blocks.push(block);
                    section.revision = section.revision.next()?;
                    changed.push(ReportEntityRef::Section(section.id));
                }
            }
            ReportEdit::SetBlockEnabled {
                block_id,
                expected_block_revision,
                enabled,
            } => {
                let (_, _, _, _, block) = self.block_mut(block_id)?;
                require_entity_revision(
                    ReportEntityRef::Block(block_id),
                    expected_block_revision,
                    block.revision,
                )?;
                if block.enabled == enabled {
                    return Err(ReportError::NoChanges);
                }
                block.enabled = enabled;
                block.revision = block.revision.next()?;
                changed.push(ReportEntityRef::Block(block_id));
            }
            ReportEdit::ReplaceBlock {
                block_id,
                expected_block_revision,
                kind,
            } => {
                kind.validate()?;
                let (_, _, _, _, block) = self.block_mut(block_id)?;
                require_entity_revision(
                    ReportEntityRef::Block(block_id),
                    expected_block_revision,
                    block.revision,
                )?;
                if block.kind == kind {
                    return Err(ReportError::NoChanges);
                }
                block.kind = kind;
                block.revision = block.revision.next()?;
                changed.push(ReportEntityRef::Block(block_id));
            }
            ReportEdit::UpdateBlockReference {
                block_id,
                expected_block_revision,
                reference,
            } => {
                reference.validate()?;
                let (_, _, _, _, block) = self.block_mut(block_id)?;
                require_entity_revision(
                    ReportEntityRef::Block(block_id),
                    expected_block_revision,
                    block.revision,
                )?;
                if block.kind.reference() == Some(&reference) {
                    return Err(ReportError::NoChanges);
                }
                block.kind.set_reference(reference)?;
                block.revision = block.revision.next()?;
                changed.push(ReportEntityRef::Block(block_id));
            }
            ReportEdit::MoveBlock {
                block_id,
                expected_block_revision,
                target_section_id,
                before,
            } => self.move_block(
                block_id,
                expected_block_revision,
                target_section_id,
                before,
                changed,
            )?,
            ReportEdit::Remove {
                entity,
                expected_entity_revision,
            } => self.remove_entity(
                entity,
                expected_entity_revision,
                committed_revision,
                changed,
                tombstoned,
            )?,
        }
        Ok(())
    }

    fn move_page(
        &mut self,
        page_id: ReportPageId,
        expected_revision: ObjectRevision,
        before: Option<ReportPageId>,
        changed: &mut Vec<ReportEntityRef>,
    ) -> Result<(), ReportError> {
        let from = self
            .pages
            .iter()
            .position(|page| page.id == page_id)
            .ok_or(ReportError::EntityNotFound(ReportEntityRef::Page(page_id)))?;
        require_entity_revision(
            ReportEntityRef::Page(page_id),
            expected_revision,
            self.pages[from].revision,
        )?;
        if before == Some(page_id) {
            return Err(ReportError::InvalidMoveTarget);
        }
        let mut page = self.pages.remove(from);
        let target = match before {
            Some(target_id) => self
                .pages
                .iter()
                .position(|candidate| candidate.id == target_id)
                .ok_or(ReportError::EntityNotFound(ReportEntityRef::Page(
                    target_id,
                )))?,
            None => self.pages.len(),
        };
        if target == from {
            // Put the removed value back at its original position before
            // reporting a no-op so the cloned candidate remains internally
            // usable for diagnostic inspection.
            self.pages.insert(from, page);
            return Err(ReportError::NoChanges);
        }
        page.revision = page.revision.next()?;
        self.pages.insert(target, page);
        changed.push(ReportEntityRef::Page(page_id));
        Ok(())
    }

    fn move_section(
        &mut self,
        section_id: ReportSectionId,
        expected_revision: ObjectRevision,
        target_page_id: ReportPageId,
        before: Option<ReportSectionId>,
        changed: &mut Vec<ReportEntityRef>,
    ) -> Result<(), ReportError> {
        if before == Some(section_id) {
            return Err(ReportError::InvalidMoveTarget);
        }
        let (source_page_index, source_section_index) = self.section_position(section_id)?;
        require_entity_revision(
            ReportEntityRef::Section(section_id),
            expected_revision,
            self.pages[source_page_index].sections[source_section_index].revision,
        )?;
        let target_page_index = self
            .pages
            .iter()
            .position(|page| page.id == target_page_id)
            .ok_or(ReportError::EntityNotFound(ReportEntityRef::Page(
                target_page_id,
            )))?;
        if source_page_index != target_page_index
            && self.pages[target_page_index].sections.len() >= MAX_SECTIONS_PER_PAGE
        {
            return Err(ReportError::CapacityExceeded("sections per report page"));
        }
        let target_index_before_removal = match before {
            Some(target_id) => {
                let (page_index, section_index) = self.section_position(target_id)?;
                if page_index != target_page_index {
                    return Err(ReportError::InvalidMoveTarget);
                }
                section_index
            }
            None => self.pages[target_page_index].sections.len(),
        };
        let no_change = source_page_index == target_page_index
            && (target_index_before_removal == source_section_index
                || (before.is_some() && target_index_before_removal == source_section_index + 1)
                || (before.is_none()
                    && source_section_index + 1 == self.pages[source_page_index].sections.len()));
        if no_change {
            return Err(ReportError::NoChanges);
        }
        let mut section = self.pages[source_page_index]
            .sections
            .remove(source_section_index);
        let target_index = if source_page_index == target_page_index
            && target_index_before_removal > source_section_index
        {
            target_index_before_removal - 1
        } else {
            target_index_before_removal
        };
        section.revision = section.revision.next()?;
        self.pages[target_page_index]
            .sections
            .insert(target_index, section);
        self.pages[source_page_index].revision = self.pages[source_page_index].revision.next()?;
        if source_page_index != target_page_index {
            self.pages[target_page_index].revision =
                self.pages[target_page_index].revision.next()?;
        }
        changed.push(ReportEntityRef::Section(section_id));
        changed.push(ReportEntityRef::Page(self.pages[source_page_index].id));
        changed.push(ReportEntityRef::Page(self.pages[target_page_index].id));
        Ok(())
    }

    fn move_block(
        &mut self,
        block_id: ReportBlockId,
        expected_revision: ObjectRevision,
        target_section_id: ReportSectionId,
        before: Option<ReportBlockId>,
        changed: &mut Vec<ReportEntityRef>,
    ) -> Result<(), ReportError> {
        if before == Some(block_id) {
            return Err(ReportError::InvalidMoveTarget);
        }
        let (source_page, source_section, source_block) = self.block_position(block_id)?;
        require_entity_revision(
            ReportEntityRef::Block(block_id),
            expected_revision,
            self.pages[source_page].sections[source_section].blocks[source_block].revision,
        )?;
        let (target_page, target_section) = self.section_position(target_section_id)?;
        if (source_page, source_section) != (target_page, target_section)
            && self.pages[target_page].sections[target_section]
                .blocks
                .len()
                >= MAX_BLOCKS_PER_SECTION
        {
            return Err(ReportError::CapacityExceeded(
                "content blocks per report section",
            ));
        }
        let target_index_before_removal = match before {
            Some(target_id) => {
                let (page, section, index) = self.block_position(target_id)?;
                if (page, section) != (target_page, target_section) {
                    return Err(ReportError::InvalidMoveTarget);
                }
                index
            }
            None => self.pages[target_page].sections[target_section]
                .blocks
                .len(),
        };
        let same_container = (source_page, source_section) == (target_page, target_section);
        let no_change = same_container
            && (target_index_before_removal == source_block
                || (before.is_some() && target_index_before_removal == source_block + 1)
                || (before.is_none()
                    && source_block + 1
                        == self.pages[source_page].sections[source_section]
                            .blocks
                            .len()));
        if no_change {
            return Err(ReportError::NoChanges);
        }
        let mut block = self.pages[source_page].sections[source_section]
            .blocks
            .remove(source_block);
        let target_index = if same_container && target_index_before_removal > source_block {
            target_index_before_removal - 1
        } else {
            target_index_before_removal
        };
        block.revision = block.revision.next()?;
        self.pages[target_page].sections[target_section]
            .blocks
            .insert(target_index, block);
        self.pages[source_page].sections[source_section].revision = self.pages[source_page]
            .sections[source_section]
            .revision
            .next()?;
        if !same_container {
            self.pages[target_page].sections[target_section].revision = self.pages[target_page]
                .sections[target_section]
                .revision
                .next()?;
        }
        changed.push(ReportEntityRef::Block(block_id));
        changed.push(ReportEntityRef::Section(
            self.pages[source_page].sections[source_section].id,
        ));
        changed.push(ReportEntityRef::Section(
            self.pages[target_page].sections[target_section].id,
        ));
        Ok(())
    }

    fn remove_entity(
        &mut self,
        entity: ReportEntityRef,
        expected_revision: ObjectRevision,
        committed_revision: ObjectRevision,
        changed: &mut Vec<ReportEntityRef>,
        tombstoned: &mut Vec<ReportEntityRef>,
    ) -> Result<(), ReportError> {
        match entity {
            ReportEntityRef::Page(page_id) => {
                let index = self
                    .pages
                    .iter()
                    .position(|page| page.id == page_id)
                    .ok_or(ReportError::EntityNotFound(entity))?;
                require_entity_revision(entity, expected_revision, self.pages[index].revision)?;
                let page = self.pages.remove(index);
                for section in page.sections {
                    for block in section.blocks {
                        self.record_tombstone(
                            ReportEntityRef::Block(block.id),
                            block.created_at_document_revision,
                            block.revision,
                            committed_revision,
                            tombstoned,
                        );
                    }
                    self.record_tombstone(
                        ReportEntityRef::Section(section.id),
                        section.created_at_document_revision,
                        section.revision,
                        committed_revision,
                        tombstoned,
                    );
                }
                self.record_tombstone(
                    ReportEntityRef::Page(page.id),
                    page.created_at_document_revision,
                    page.revision,
                    committed_revision,
                    tombstoned,
                );
            }
            ReportEntityRef::Section(section_id) => {
                let (page_index, section_index) = self.section_position(section_id)?;
                require_entity_revision(
                    entity,
                    expected_revision,
                    self.pages[page_index].sections[section_index].revision,
                )?;
                let section = self.pages[page_index].sections.remove(section_index);
                for block in section.blocks {
                    self.record_tombstone(
                        ReportEntityRef::Block(block.id),
                        block.created_at_document_revision,
                        block.revision,
                        committed_revision,
                        tombstoned,
                    );
                }
                self.record_tombstone(
                    ReportEntityRef::Section(section.id),
                    section.created_at_document_revision,
                    section.revision,
                    committed_revision,
                    tombstoned,
                );
                self.pages[page_index].revision = self.pages[page_index].revision.next()?;
                changed.push(ReportEntityRef::Page(self.pages[page_index].id));
            }
            ReportEntityRef::Block(block_id) => {
                let (page_index, section_index, block_index) = self.block_position(block_id)?;
                require_entity_revision(
                    entity,
                    expected_revision,
                    self.pages[page_index].sections[section_index].blocks[block_index].revision,
                )?;
                let block = self.pages[page_index].sections[section_index]
                    .blocks
                    .remove(block_index);
                self.record_tombstone(
                    ReportEntityRef::Block(block.id),
                    block.created_at_document_revision,
                    block.revision,
                    committed_revision,
                    tombstoned,
                );
                self.pages[page_index].sections[section_index].revision = self.pages[page_index]
                    .sections[section_index]
                    .revision
                    .next()?;
                changed.push(ReportEntityRef::Section(
                    self.pages[page_index].sections[section_index].id,
                ));
            }
        }
        Ok(())
    }

    fn record_tombstone(
        &mut self,
        entity: ReportEntityRef,
        created_at_document_revision: ObjectRevision,
        last_entity_revision: ObjectRevision,
        removed_at_document_revision: ObjectRevision,
        receipt: &mut Vec<ReportEntityRef>,
    ) {
        self.tombstones.push(ReportTombstone {
            entity,
            created_at_document_revision,
            last_entity_revision,
            removed_at_document_revision,
        });
        receipt.push(entity);
    }

    fn page_mut(&mut self, page_id: ReportPageId) -> Result<&mut ReportPage, ReportError> {
        self.pages
            .iter_mut()
            .find(|page| page.id == page_id)
            .ok_or(ReportError::EntityNotFound(ReportEntityRef::Page(page_id)))
    }

    fn section_mut(
        &mut self,
        section_id: ReportSectionId,
    ) -> Result<(usize, usize, &mut ReportSection), ReportError> {
        let (page_index, section_index) = self.section_position(section_id)?;
        let section = &mut self.pages[page_index].sections[section_index];
        Ok((page_index, section_index, section))
    }

    fn block_mut(
        &mut self,
        block_id: ReportBlockId,
    ) -> Result<(usize, usize, usize, ReportSectionId, &mut ReportBlock), ReportError> {
        let (page_index, section_index, block_index) = self.block_position(block_id)?;
        let section_id = self.pages[page_index].sections[section_index].id;
        let block = &mut self.pages[page_index].sections[section_index].blocks[block_index];
        Ok((page_index, section_index, block_index, section_id, block))
    }

    fn section_position(&self, section_id: ReportSectionId) -> Result<(usize, usize), ReportError> {
        for (page_index, page) in self.pages.iter().enumerate() {
            if let Some(section_index) = page
                .sections
                .iter()
                .position(|section| section.id == section_id)
            {
                return Ok((page_index, section_index));
            }
        }
        Err(ReportError::EntityNotFound(ReportEntityRef::Section(
            section_id,
        )))
    }

    fn block_position(
        &self,
        block_id: ReportBlockId,
    ) -> Result<(usize, usize, usize), ReportError> {
        for (page_index, page) in self.pages.iter().enumerate() {
            for (section_index, section) in page.sections.iter().enumerate() {
                if let Some(block_index) =
                    section.blocks.iter().position(|block| block.id == block_id)
                {
                    return Ok((page_index, section_index, block_index));
                }
            }
        }
        Err(ReportError::EntityNotFound(ReportEntityRef::Block(
            block_id,
        )))
    }

    fn block_count(&self) -> usize {
        self.pages
            .iter()
            .flat_map(|page| page.sections.iter())
            .map(|section| section.blocks.len())
            .sum()
    }

    fn section_count(&self) -> usize {
        self.pages.iter().map(|page| page.sections.len()).sum()
    }

    pub fn validate(&self) -> Result<(), ReportError> {
        if self.schema_version != Self::SCHEMA_VERSION {
            return Err(ReportError::UnsupportedSchemaVersion(self.schema_version));
        }
        self.validate_content()?;
        self.validate_revision_history()
    }

    fn validate_content(&self) -> Result<(), ReportError> {
        ReportDocumentContentView::from_document(self).validate()
    }

    fn validate_revision_history(&self) -> Result<(), ReportError> {
        let records = &self.revision_history.records;
        if records.is_empty() || records.len() > MAX_REPORT_REVISION_HISTORY_RECORDS {
            return Err(invalid_revision_history());
        }
        let baseline_revision = match self.revision_history.origin {
            ReportRevisionHistoryOrigin::Native
            | ReportRevisionHistoryOrigin::ImportedSchemaOneBaseline => ObjectRevision::INITIAL,
            ReportRevisionHistoryOrigin::ImportedSchemaTwoBaseline => records[0].revision,
        };
        if records[0].revision != baseline_revision
            || records[0].prior_revision_identity.is_some()
            || records[0].prior_record_digest.is_some()
            || records.last().map(|record| record.revision) != Some(self.revision)
        {
            return Err(invalid_revision_history());
        }

        let mut revision_identities = HashSet::with_capacity(records.len());
        let mut prior_revision_identity = None;
        let mut prior_record_digest = None;
        let mut aggregate_snapshot_bytes = 0_u64;
        for (index, record) in records.iter().enumerate() {
            let expected_revision = ObjectRevision::new(
                baseline_revision
                    .get()
                    .checked_add(
                        u64::try_from(index).map_err(|_| ReportError::RevisionSpaceExhausted)?,
                    )
                    .ok_or(ReportError::RevisionSpaceExhausted)?,
            )?;
            validate_label("report-revision.actor", &record.actor, 256)?;
            validate_label("report-revision.note", &record.revision_note, 4_096)?;
            if record.document_id != self.id
                || record.revision != expected_revision
                || record.prior_revision_identity != prior_revision_identity
                || record.prior_record_digest != prior_record_digest
                || !revision_identities.insert(record.revision_identity)
                || record.snapshot.document_id != self.id
                || record.snapshot.revision != record.revision
            {
                return Err(invalid_revision_history());
            }

            ReportDocumentContentView::from_snapshot(&record.snapshot).validate()?;

            let (snapshot_digest, snapshot_serialized_bytes) =
                report_snapshot_digest_and_size(&record.snapshot)?;
            aggregate_snapshot_bytes = aggregate_snapshot_bytes
                .checked_add(snapshot_serialized_bytes)
                .ok_or(ReportError::CapacityExceeded(
                    "report source revision snapshot bytes",
                ))?;
            let record_digest = report_revision_record_digest(
                record.revision_identity,
                record.document_id,
                record.revision,
                self.revision_history.origin,
                record.prior_revision_identity,
                record.prior_record_digest,
                record.timestamp_unix_ms,
                &record.actor,
                &record.revision_note,
                snapshot_serialized_bytes,
                snapshot_digest,
            )?;
            if snapshot_serialized_bytes != record.snapshot_serialized_bytes
                || aggregate_snapshot_bytes > MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES
                || snapshot_digest != record.snapshot_digest
                || record_digest != record.record_digest
            {
                return Err(invalid_revision_history());
            }

            if record.revision == ObjectRevision::INITIAL {
                if record.timestamp_unix_ms != 0 {
                    return Err(invalid_revision_history());
                }
            } else if record.snapshot.receipts.last().is_none_or(|receipt| {
                receipt.committed_document_revision != record.revision
                    || receipt.timestamp_unix_ms != record.timestamp_unix_ms
            }) {
                return Err(invalid_revision_history());
            }

            prior_revision_identity = Some(record.revision_identity);
            prior_record_digest = Some(record.record_digest);
        }
        if records.last().is_none_or(|record| {
            let snapshot = &record.snapshot;
            snapshot.document_id != self.id
                || snapshot.revision != self.revision
                || snapshot.title != self.title
                || snapshot.template != self.template
                || snapshot.pages != self.pages
                || snapshot.receipts != self.receipts
                || snapshot.tombstones != self.tombstones
                || snapshot.legacy_origin_entities != self.legacy_origin_entities
        }) {
            return Err(invalid_revision_history());
        }
        Ok(())
    }
}
