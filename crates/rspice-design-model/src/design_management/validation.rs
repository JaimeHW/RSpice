//! Shared normalization, invariant checking, ordering, and semantic digest helpers.

use super::*;

pub(super) fn normalize_sheet_definition(mut definition: SheetDefinition) -> SheetDefinition {
    definition.name = normalize_text(&definition.name);
    definition
}

pub(super) fn validate_sheet_definition(
    definition: &SheetDefinition,
) -> Result<(), DesignManagementError> {
    validate_name("sheet name", &definition.name)?;
    if definition.explicit_page_number == Some(0) {
        return Err(DesignManagementError::NumericRange("sheet page number"));
    }
    Ok(())
}

pub(super) fn validate_cross_sheet_port_definition(
    definition: &CrossSheetPortDefinition,
    sheet_ids: &HashSet<SheetId>,
) -> Result<(), DesignManagementError> {
    validate_name("cross-sheet net name", &definition.net_name)?;
    definition.first.anchor.validate()?;
    definition.second.anchor.validate()?;
    if definition.first.sheet_id == definition.second.sheet_id {
        return Err(DesignManagementError::CrossSheetPortSameSheet);
    }
    for endpoint in [&definition.first, &definition.second] {
        if !sheet_ids.contains(&endpoint.sheet_id) {
            return Err(DesignManagementError::MissingReference {
                domain: "cross-sheet port endpoint sheet",
                identity: endpoint.sheet_id.to_string(),
            });
        }
    }
    match (definition.signal_type, definition.discipline) {
        (CrossSheetSignalType::Logic, CrossSheetDiscipline::Logic)
        | (CrossSheetSignalType::Analog, CrossSheetDiscipline::Electrical)
        | (CrossSheetSignalType::Analog, CrossSheetDiscipline::Wreal)
        | (CrossSheetSignalType::Analog, CrossSheetDiscipline::Thermal)
        | (CrossSheetSignalType::Power, CrossSheetDiscipline::Electrical)
        | (CrossSheetSignalType::Power, CrossSheetDiscipline::Thermal) => Ok(()),
        _ => Err(DesignManagementError::IncompatiblePortContract),
    }
}

pub(super) fn canonical_port_key(
    definition: &CrossSheetPortDefinition,
) -> Result<(SheetId, ContentDigest, SheetId, ContentDigest, String), DesignManagementError> {
    let first = (
        definition.first.sheet_id,
        digest(
            "rspice-cross-sheet-port-anchor-semantic/v1",
            &definition.first.anchor,
        )?,
    );
    let second = (
        definition.second.sheet_id,
        digest(
            "rspice-cross-sheet-port-anchor-semantic/v1",
            &definition.second.anchor,
        )?,
    );
    let (first, second) = if first <= second {
        (first, second)
    } else {
        (second, first)
    };
    Ok((
        first.0,
        first.1,
        second.0,
        second.1,
        case_fold(&definition.net_name),
    ))
}

pub(super) fn normalize_variant_definition(
    mut definition: AssemblyVariantDefinition,
) -> AssemblyVariantDefinition {
    definition.name = normalize_text(&definition.name);
    for value in definition.overrides.values_mut() {
        match value {
            VariantObjectOverride::Substitute { replacement } => {
                replacement.library = normalize_text(&replacement.library);
                replacement.cell = normalize_text(&replacement.cell);
                replacement.view = normalize_text(&replacement.view);
                replacement.value_override = replacement
                    .value_override
                    .take()
                    .map(|value| normalize_text(&value));
                replacement.model_section = replacement
                    .model_section
                    .take()
                    .map(|value| normalize_text(&value));
            }
            VariantObjectOverride::DoNotPopulate { approval_reference } => {
                *approval_reference = normalize_text(approval_reference);
            }
        }
    }
    definition
}

pub(super) fn remap_variant_object_owners(
    catalog: &mut AssemblyVariantCatalog,
    source_library: &str,
    source_cell: &str,
    destination_library: &str,
    destination_cell: &str,
) -> Result<usize, DesignManagementError> {
    catalog.validate()?;
    let original = catalog.clone();
    let mut ids = original
        .variants
        .iter()
        .map(|variant| {
            let mut depth = 0usize;
            let mut cursor = variant.definition.parent.as_ref().map(|parent| parent.id);
            while let Some(id) = cursor {
                depth = depth
                    .checked_add(1)
                    .ok_or(DesignManagementError::NumericRange("variant parent depth"))?;
                cursor = original
                    .find(id)
                    .and_then(|parent| parent.definition.parent.as_ref().map(|entry| entry.id));
            }
            Ok((depth, variant.id))
        })
        .collect::<Result<Vec<_>, DesignManagementError>>()?;
    ids.sort_by_key(|(depth, id)| (*depth, *id));
    let mut rebuilt: BTreeMap<AssemblyVariantId, AssemblyVariant> = BTreeMap::new();
    let mut remapped_objects = 0usize;
    for (_, id) in ids {
        let source = original.find(id).expect("variant catalog was validated");
        let mut definition = source.definition.clone();
        let mut overrides = BTreeMap::new();
        for (object, value) in &definition.overrides {
            let target = object
                .remap_cell_owner(
                    source_library,
                    source_cell,
                    destination_library,
                    destination_cell,
                )?
                .unwrap_or_else(|| object.clone());
            if &target != object {
                remapped_objects += 1;
            }
            if overrides.insert(target.clone(), value.clone()).is_some() {
                return Err(DesignManagementError::DuplicateScopedSchematicObject(
                    target,
                ));
            }
        }
        definition.overrides = overrides;
        if let Some(parent) = &mut definition.parent {
            let rebuilt_parent = rebuilt
                .get(&parent.id)
                .expect("parent variants are rebuilt before their children");
            parent.revision = rebuilt_parent.revision;
            parent.semantic_digest = rebuilt_parent.semantic_digest;
        }
        let mut target = source.clone();
        if definition != source.definition {
            target.revision =
                next_revision(source.revision, "assembly variant", source.id.to_string())?;
            target.semantic_digest = digest("rspice-assembly-variant-semantic/v1", &definition)?;
            target.definition = definition;
        }
        rebuilt.insert(id, target);
    }
    if remapped_objects == 0 {
        return Ok(0);
    }
    catalog.variants = original
        .variants
        .iter()
        .map(|variant| {
            rebuilt
                .remove(&variant.id)
                .expect("every variant was rebuilt")
        })
        .collect();
    catalog.validate()?;
    Ok(remapped_objects)
}

pub(super) fn validate_variant_definition(
    definition: &AssemblyVariantDefinition,
) -> Result<(), DesignManagementError> {
    validate_name("assembly variant name", &definition.name)?;
    require_limit(
        "assembly variant overrides",
        definition.overrides.len(),
        MAX_VARIANT_OVERRIDES,
    )?;
    if let Some(parent) = &definition.parent {
        require_non_nil(parent.id.as_uuid(), "parent assembly variant")?;
        require_nonzero_revision(
            parent.revision,
            "parent assembly variant",
            parent.id.to_string(),
        )?;
    }
    for (object, value) in &definition.overrides {
        object.validate()?;
        match value {
            VariantObjectOverride::Substitute { replacement } => {
                validate_substitution(replacement)?;
            }
            VariantObjectOverride::DoNotPopulate { approval_reference } => {
                validate_value("DNP approval reference", approval_reference, false)?;
            }
        }
    }
    Ok(())
}

pub(super) fn validate_substitution(
    replacement: &ComponentSubstitution,
) -> Result<(), DesignManagementError> {
    validate_name("replacement library", &replacement.library)?;
    validate_name("replacement cell", &replacement.cell)?;
    validate_name("replacement view", &replacement.view)?;
    if let Some(value) = &replacement.value_override {
        validate_value("replacement value", value, true)?;
    }
    if let Some(section) = &replacement.model_section {
        validate_name("replacement model section", section)?;
    }
    Ok(())
}

pub(super) fn normalize_annotation_policy_definition(
    mut definition: AnnotationPolicyDefinition,
) -> AnnotationPolicyDefinition {
    for range in &mut definition.reserved_ranges {
        if let AnnotationRangeScope::Hierarchy { path } = &mut range.scope {
            *path = normalize_text(path);
        }
        for prefix in &mut range.prefixes {
            *prefix = normalize_prefix(prefix);
        }
        range.prefixes.sort();
        range.prefixes.dedup();
    }
    definition.reserved_ranges.sort_by(|left, right| {
        left.scope
            .cmp(&right.scope)
            .then_with(|| left.first.cmp(&right.first))
            .then_with(|| left.last.cmp(&right.last))
            .then_with(|| left.prefixes.cmp(&right.prefixes))
    });
    definition
}

pub(super) fn validate_annotation_policy_definition(
    definition: &AnnotationPolicyDefinition,
) -> Result<(), DesignManagementError> {
    require_limit(
        "annotation reserved ranges",
        definition.reserved_ranges.len(),
        MAX_ANNOTATION_RANGES,
    )?;
    for range in &definition.reserved_ranges {
        if range.first == 0 || range.first > range.last {
            return Err(DesignManagementError::InvalidAnnotationRange {
                first: range.first,
                last: range.last,
            });
        }
        if range.prefixes.is_empty() {
            return Err(DesignManagementError::EmptyAnnotationPrefixes);
        }
        for prefix in &range.prefixes {
            validate_prefix(prefix)?;
        }
        if let AnnotationRangeScope::Hierarchy { path } = &range.scope {
            validate_path("annotation hierarchy range", path)?;
        }
    }
    for (index, left) in definition.reserved_ranges.iter().enumerate() {
        for right in definition.reserved_ranges.iter().skip(index + 1) {
            if left.scope == right.scope
                && left.first <= right.last
                && right.first <= left.last
                && left
                    .prefixes
                    .iter()
                    .any(|prefix| right.prefixes.contains(prefix))
            {
                return Err(DesignManagementError::OverlappingAnnotationRanges);
            }
        }
    }
    Ok(())
}

pub(super) fn validate_renumber_request(
    request: &RenumberRequest,
) -> Result<(), DesignManagementError> {
    require_limit(
        "renumber objects",
        request.objects.len(),
        MAX_ANNOTATION_MAPPINGS_PER_ENTRY,
    )?;
    match &request.scope {
        RenumberScope::CurrentHierarchy { path } => {
            validate_path("renumber hierarchy path", path)?;
        }
        RenumberScope::CurrentSheet { sheet_id } => {
            require_non_nil(sheet_id.as_uuid(), "renumber sheet")?;
        }
        RenumberScope::WholeProject => {}
    }
    let mut ids = HashSet::with_capacity(request.objects.len());
    let mut references = HashSet::with_capacity(request.objects.len());
    for object in &request.objects {
        object.object.validate()?;
        if !ids.insert(object.object.clone()) {
            return Err(DesignManagementError::DuplicateScopedSchematicObject(
                object.object.clone(),
            ));
        }
        validate_reference_designator(&object.current_reference)?;
        validate_name("annotation device family", &object.device_family)?;
        validate_path("annotation hierarchy path", &object.hierarchy_path)?;
        if let Some(sheet_id) = object.sheet_id {
            require_non_nil(sheet_id.as_uuid(), "annotation sheet")?;
        }
        if !references.insert(case_fold(&object.current_reference)) {
            return Err(DesignManagementError::DuplicateReferenceDesignator(
                object.current_reference.clone(),
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_annotation_mappings(
    mappings: &BTreeMap<SchematicObjectKey, AnnotationMapping>,
) -> Result<(), DesignManagementError> {
    let mut new_refs = HashSet::with_capacity(mappings.len());
    for (object, mapping) in mappings {
        object.validate()?;
        validate_reference_designator(&mapping.old_reference)?;
        validate_reference_designator(&mapping.new_reference)?;
        if !new_refs.insert(case_fold(&mapping.new_reference)) {
            return Err(DesignManagementError::DuplicateReferenceDesignator(
                mapping.new_reference.clone(),
            ));
        }
    }
    Ok(())
}

pub(super) fn object_in_scope(object: &AnnotationObject, scope: &RenumberScope) -> bool {
    match scope {
        RenumberScope::WholeProject => true,
        RenumberScope::CurrentHierarchy { path } => {
            object.hierarchy_path == *path
                || object
                    .hierarchy_path
                    .strip_prefix(path)
                    .is_some_and(|suffix| suffix.starts_with('/'))
        }
        RenumberScope::CurrentSheet { sheet_id } => object.sheet_id == Some(*sheet_id),
    }
}

pub(super) fn sort_annotation_objects(objects: &mut [AnnotationObject], order: RenumberOrder) {
    objects.sort_by(|left, right| match order {
        RenumberOrder::HierarchyThenCoordinates => left
            .hierarchy_path
            .cmp(&right.hierarchy_path)
            .then_with(|| left.position.y.cmp(&right.position.y))
            .then_with(|| left.position.x.cmp(&right.position.x))
            .then_with(|| left.object.cmp(&right.object)),
        RenumberOrder::SheetThenCoordinates => left
            .sheet_id
            .cmp(&right.sheet_id)
            .then_with(|| left.position.y.cmp(&right.position.y))
            .then_with(|| left.position.x.cmp(&right.position.x))
            .then_with(|| left.object.cmp(&right.object)),
        RenumberOrder::ConnectivityOrder => left
            .connectivity_order
            .unwrap_or(u64::MAX)
            .cmp(&right.connectivity_order.unwrap_or(u64::MAX))
            .then_with(|| left.object.cmp(&right.object)),
    });
}

pub(super) fn annotation_prefix(
    object: &AnnotationObject,
    allocation: AnnotationPrefixAllocation,
) -> Result<String, DesignManagementError> {
    let from_reference = object
        .current_reference
        .chars()
        .take_while(|character| character.is_ascii_alphabetic())
        .collect::<String>();
    let candidate = match allocation {
        AnnotationPrefixAllocation::ByDeviceFamily => {
            let family = object
                .device_family
                .chars()
                .filter(char::is_ascii_alphabetic)
                .collect::<String>();
            if family.is_empty() {
                from_reference
            } else {
                family
            }
        }
        AnnotationPrefixAllocation::BySheet | AnnotationPrefixAllocation::ByHierarchy => {
            from_reference
        }
    };
    let prefix = normalize_prefix(&candidate);
    validate_prefix(&prefix)?;
    Ok(prefix)
}

pub(super) fn matching_annotation_ranges<'a>(
    ranges: &'a [AnnotationReservedRange],
    object: &AnnotationObject,
    prefix: &str,
) -> Vec<&'a AnnotationReservedRange> {
    let mut matches = ranges
        .iter()
        .filter(|range| {
            range.prefixes.iter().any(|entry| entry == prefix)
                && match &range.scope {
                    AnnotationRangeScope::Project => true,
                    AnnotationRangeScope::Sheet { sheet_id } => object.sheet_id == Some(*sheet_id),
                    AnnotationRangeScope::Hierarchy { path } => {
                        object.hierarchy_path == *path
                            || object
                                .hierarchy_path
                                .strip_prefix(path)
                                .is_some_and(|suffix| suffix.starts_with('/'))
                    }
                }
        })
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| {
        annotation_scope_specificity(&right.scope)
            .cmp(&annotation_scope_specificity(&left.scope))
            .then_with(|| left.first.cmp(&right.first))
    });
    matches
}

pub(super) fn annotation_scope_specificity(scope: &AnnotationRangeScope) -> u8 {
    match scope {
        AnnotationRangeScope::Project => 0,
        AnnotationRangeScope::Sheet { .. } => 1,
        AnnotationRangeScope::Hierarchy { .. } => 2,
    }
}

pub(super) fn allocate_reference(
    prefix: &str,
    ranges: &[&AnnotationReservedRange],
    occupied: &HashSet<String>,
) -> Result<String, DesignManagementError> {
    if ranges.is_empty() {
        for number in 1..=u32::MAX {
            let candidate = format!("{prefix}{number}");
            if !occupied.contains(&case_fold(&candidate)) {
                return Ok(candidate);
            }
        }
    } else {
        for range in ranges {
            for number in range.first..=range.last {
                let candidate = format!("{prefix}{number}");
                if !occupied.contains(&case_fold(&candidate)) {
                    return Ok(candidate);
                }
            }
        }
    }
    Err(DesignManagementError::AnnotationRangeExhausted(
        prefix.to_owned(),
    ))
}

pub(super) fn validate_hierarchy_audit_request(
    request: &HierarchyAuditRequest,
) -> Result<(), DesignManagementError> {
    if let HierarchyAuditConfiguration::ConfigurationSet { id, revision, .. } =
        &request.configuration
    {
        if id.is_nil() {
            return Err(DesignManagementError::NilIdentity(
                "hierarchy audit configuration",
            ));
        }
        require_nonzero_revision(*revision, "hierarchy audit configuration", id.to_string())?;
    }
    require_limit(
        "hierarchy audit subjects",
        request.subjects.len(),
        MAX_HIERARCHY_AUDIT_SUBJECTS,
    )?;
    if request.subjects.is_empty() {
        return Err(DesignManagementError::EmptyHierarchyAudit);
    }
    let mut paths = HashSet::with_capacity(request.subjects.len());
    for subject in &request.subjects {
        validate_path("hierarchy instance path", &subject.instance_path)?;
        validate_name("hierarchy cell name", &subject.cell_name)?;
        validate_name("hierarchy design view", &subject.design_view)?;
        validate_string_list("declared hierarchy fallback", &subject.declared_fallbacks)?;
        if let Some(view) = &subject.resolved_simulation_view {
            validate_name("resolved simulation view", view)?;
        }
        if let Some(view) = &subject.fallback_used {
            validate_name("used hierarchy fallback", view)?;
        }
        for child in &subject.child_instance_paths {
            validate_path("hierarchy child path", child)?;
        }
        if let Some(boundary) = &subject.protected_boundary_id {
            validate_name("protected boundary identity", boundary)?;
        }
        if !paths.insert(subject.instance_path.clone()) {
            return Err(DesignManagementError::DuplicateHierarchyPath(
                subject.instance_path.clone(),
            ));
        }
    }
    let mut boundaries = HashSet::with_capacity(request.boundary_evidence.len());
    for evidence in &request.boundary_evidence {
        validate_name("protected boundary identity", &evidence.boundary_id)?;
        if !boundaries.insert(evidence.boundary_id.clone()) {
            return Err(DesignManagementError::DuplicateProtectedBoundary(
                evidence.boundary_id.clone(),
            ));
        }
    }
    Ok(())
}

pub(super) fn evaluate_hierarchy_audit(
    request: &HierarchyAuditRequest,
) -> Result<Vec<HierarchyAuditFinding>, DesignManagementError> {
    let by_path = request
        .subjects
        .iter()
        .map(|subject| (subject.instance_path.as_str(), subject))
        .collect::<BTreeMap<_, _>>();
    let boundaries = request
        .boundary_evidence
        .iter()
        .map(|evidence| (evidence.boundary_id.as_str(), evidence))
        .collect::<BTreeMap<_, _>>();
    let mut findings = Vec::new();
    for subject in &request.subjects {
        if subject.resolved_simulation_view.is_none() {
            findings.push(HierarchyAuditFinding {
                kind: HierarchyAuditFindingKind::UnresolvedView,
                instance_path: subject.instance_path.clone(),
                detail: format!("{} has no resolved simulation view", subject.cell_name),
            });
        }
        if let Some(fallback) = &subject.fallback_used
            && !subject
                .declared_fallbacks
                .iter()
                .any(|view| view == fallback)
        {
            findings.push(HierarchyAuditFinding {
                kind: HierarchyAuditFindingKind::UndeclaredFallback,
                instance_path: subject.instance_path.clone(),
                detail: format!("fallback {fallback:?} is not declared"),
            });
        }
        for child in &subject.child_instance_paths {
            if !by_path.contains_key(child.as_str()) {
                findings.push(HierarchyAuditFinding {
                    kind: HierarchyAuditFindingKind::MissingChild,
                    instance_path: subject.instance_path.clone(),
                    detail: format!("declared child {child:?} is absent"),
                });
            }
        }
        if let Some(boundary_id) = &subject.protected_boundary_id {
            match boundaries.get(boundary_id.as_str()) {
                None => findings.push(HierarchyAuditFinding {
                    kind: HierarchyAuditFindingKind::MissingProtectedBoundaryEvidence,
                    instance_path: subject.instance_path.clone(),
                    detail: format!("protected boundary {boundary_id:?} has no evidence"),
                }),
                Some(evidence) => {
                    if request.protected_boundaries
                        == ProtectedBoundaryChecks::ValidateSignaturesAndPins
                        && !evidence.signature_valid
                    {
                        findings.push(HierarchyAuditFinding {
                            kind: HierarchyAuditFindingKind::InvalidProtectedBoundarySignature,
                            instance_path: subject.instance_path.clone(),
                            detail: format!(
                                "protected boundary {boundary_id:?} signature is invalid"
                            ),
                        });
                    }
                    if !evidence.pins_match {
                        findings.push(HierarchyAuditFinding {
                            kind: HierarchyAuditFindingKind::ProtectedBoundaryPinMismatch,
                            instance_path: subject.instance_path.clone(),
                            detail: format!("protected boundary {boundary_id:?} pins do not match"),
                        });
                    }
                }
            }
        }
    }

    let mut visited = HashSet::new();
    let mut active = HashSet::new();
    for subject in &request.subjects {
        detect_hierarchy_cycles(
            subject.instance_path.as_str(),
            &by_path,
            &mut visited,
            &mut active,
            &mut findings,
        );
    }
    findings.sort_by(|left, right| {
        left.instance_path
            .cmp(&right.instance_path)
            .then_with(|| left.kind.cmp(&right.kind))
            .then_with(|| left.detail.cmp(&right.detail))
    });
    require_limit(
        "hierarchy audit findings",
        findings.len(),
        MAX_HIERARCHY_AUDIT_FINDINGS,
    )?;
    Ok(findings)
}

pub(super) fn detect_hierarchy_cycles(
    path: &str,
    by_path: &BTreeMap<&str, &HierarchyAuditSubject>,
    visited: &mut HashSet<String>,
    active: &mut HashSet<String>,
    findings: &mut Vec<HierarchyAuditFinding>,
) {
    if visited.contains(path) {
        return;
    }
    if !active.insert(path.to_owned()) {
        findings.push(HierarchyAuditFinding {
            kind: HierarchyAuditFindingKind::HierarchyCycle,
            instance_path: path.to_owned(),
            detail: format!("hierarchy cycle reaches {path:?}"),
        });
        return;
    }
    if let Some(subject) = by_path.get(path) {
        for child in &subject.child_instance_paths {
            if active.contains(child) {
                findings.push(HierarchyAuditFinding {
                    kind: HierarchyAuditFindingKind::HierarchyCycle,
                    instance_path: path.to_owned(),
                    detail: format!("hierarchy cycle reaches {child:?}"),
                });
            } else if by_path.contains_key(child.as_str()) {
                detect_hierarchy_cycles(child, by_path, visited, active, findings);
            }
        }
    }
    active.remove(path);
    visited.insert(path.to_owned());
}

pub(super) fn canonical_cell_view_key(value: &str) -> Result<String, DesignManagementError> {
    let normalized = normalize_text(value);
    validate_value("cell-view key", &normalized, false)?;
    if normalized.len() > MAX_DESIGN_PATH_BYTES {
        return Err(DesignManagementError::TextTooLong {
            field: "cell-view key",
            actual: normalized.len(),
            maximum: MAX_DESIGN_PATH_BYTES,
        });
    }
    if normalized.contains('\\') {
        return Err(DesignManagementError::InvalidCellViewKey(normalized));
    }
    let segments = normalized.split('/').collect::<Vec<_>>();
    if segments.len() != 3 {
        return Err(DesignManagementError::InvalidCellViewKey(normalized));
    }
    let library = canonical_cell_view_segment("library", segments[0])?;
    let cell = canonical_cell_view_segment("cell", segments[1])?;
    let view = canonical_cell_view_segment("view", segments[2])?;
    Ok(format!("{library}/{cell}/{view}"))
}

pub(super) fn canonical_cell_view_segment(
    field: &'static str,
    value: &str,
) -> Result<String, DesignManagementError> {
    validate_name(field, value)?;
    if value
        .chars()
        .any(|character| !character.is_alphanumeric() && character != '_')
    {
        return Err(DesignManagementError::InvalidCellViewSegment {
            field,
            value: value.to_owned(),
        });
    }
    Ok(value.to_lowercase())
}

pub(super) fn cell_view_key_segments(value: &str) -> Result<[&str; 3], DesignManagementError> {
    let mut segments = value.split('/');
    let library = segments
        .next()
        .ok_or_else(|| DesignManagementError::InvalidCellViewKey(value.to_owned()))?;
    let cell = segments
        .next()
        .ok_or_else(|| DesignManagementError::InvalidCellViewKey(value.to_owned()))?;
    let view = segments
        .next()
        .ok_or_else(|| DesignManagementError::InvalidCellViewKey(value.to_owned()))?;
    if segments.next().is_some() {
        return Err(DesignManagementError::InvalidCellViewKey(value.to_owned()));
    }
    Ok([library, cell, view])
}

pub(super) fn validate_name(field: &'static str, value: &str) -> Result<(), DesignManagementError> {
    validate_value(field, value, false)?;
    if value.len() > MAX_DESIGN_NAME_BYTES {
        return Err(DesignManagementError::TextTooLong {
            field,
            actual: value.len(),
            maximum: MAX_DESIGN_NAME_BYTES,
        });
    }
    Ok(())
}

pub(super) fn validate_path(field: &'static str, value: &str) -> Result<(), DesignManagementError> {
    validate_value(field, value, false)?;
    if value.len() > MAX_DESIGN_PATH_BYTES {
        return Err(DesignManagementError::TextTooLong {
            field,
            actual: value.len(),
            maximum: MAX_DESIGN_PATH_BYTES,
        });
    }
    if !value.starts_with('/') || value.contains("//") || value.split('/').any(|part| part == "..")
    {
        return Err(DesignManagementError::InvalidPath {
            field,
            value: value.to_owned(),
        });
    }
    Ok(())
}

pub(super) fn validate_value(
    field: &'static str,
    value: &str,
    allow_empty: bool,
) -> Result<(), DesignManagementError> {
    let normalized = value.nfc().collect::<String>();
    if value.trim() != value || normalized != value || (!allow_empty && value.is_empty()) {
        return Err(DesignManagementError::InvalidText {
            field,
            value: value.to_owned(),
        });
    }
    if value.len() > MAX_DESIGN_VALUE_BYTES {
        return Err(DesignManagementError::TextTooLong {
            field,
            actual: value.len(),
            maximum: MAX_DESIGN_VALUE_BYTES,
        });
    }
    if value.chars().any(char::is_control) {
        return Err(DesignManagementError::InvalidText {
            field,
            value: value.to_owned(),
        });
    }
    Ok(())
}

pub(super) fn validate_string_list(
    field: &'static str,
    values: &[String],
) -> Result<(), DesignManagementError> {
    let mut unique = HashSet::with_capacity(values.len());
    for value in values {
        validate_name(field, value)?;
        if !unique.insert(case_fold(value)) {
            return Err(DesignManagementError::DuplicateListEntry {
                field,
                value: value.clone(),
            });
        }
    }
    Ok(())
}

pub(super) fn validate_prefix(prefix: &str) -> Result<(), DesignManagementError> {
    if prefix.is_empty()
        || prefix.len() > MAX_PREFIX_BYTES
        || !prefix
            .chars()
            .all(|character| character.is_ascii_alphabetic())
        || normalize_prefix(prefix) != prefix
    {
        return Err(DesignManagementError::InvalidAnnotationPrefix(
            prefix.to_owned(),
        ));
    }
    Ok(())
}

pub(super) fn validate_reference_designator(value: &str) -> Result<(), DesignManagementError> {
    validate_name("reference designator", value)?;
    let prefix_len = value
        .chars()
        .take_while(|character| character.is_ascii_alphabetic())
        .count();
    let (prefix, number) = value.split_at(prefix_len);
    validate_prefix(&normalize_prefix(prefix))?;
    if number.is_empty()
        || !number.chars().all(|character| character.is_ascii_digit())
        || number
            .parse::<u32>()
            .ok()
            .filter(|number| *number > 0)
            .is_none()
    {
        return Err(DesignManagementError::InvalidReferenceDesignator(
            value.to_owned(),
        ));
    }
    Ok(())
}

pub(super) fn normalize_text(value: &str) -> String {
    value.trim().nfc().collect()
}

pub(super) fn normalize_prefix(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}

pub(super) fn case_fold(value: &str) -> String {
    value.to_lowercase()
}

pub(super) fn unique_object_ids(
    ids: impl IntoIterator<Item = u64>,
) -> Result<Vec<u64>, DesignManagementError> {
    let mut seen = HashSet::new();
    let mut values = Vec::new();
    for id in ids {
        require_object_id(id)?;
        if !seen.insert(id) {
            return Err(DesignManagementError::DuplicateSchematicObject(id));
        }
        values.push(id);
    }
    values.sort_unstable();
    Ok(values)
}

pub(super) fn require_object_id(id: u64) -> Result<(), DesignManagementError> {
    if id == 0 {
        Err(DesignManagementError::ZeroSchematicObject)
    } else {
        Ok(())
    }
}

pub(super) fn require_limit(
    domain: &'static str,
    actual: usize,
    maximum: usize,
) -> Result<(), DesignManagementError> {
    if actual > maximum {
        Err(DesignManagementError::LimitExceeded {
            domain,
            actual,
            maximum,
        })
    } else {
        Ok(())
    }
}

pub(super) fn require_non_nil(
    value: Uuid,
    domain: &'static str,
) -> Result<(), DesignManagementError> {
    if value.is_nil() {
        Err(DesignManagementError::NilIdentity(domain))
    } else {
        Ok(())
    }
}

pub(super) fn require_nonzero_revision(
    revision: u64,
    domain: &'static str,
    identity: String,
) -> Result<(), DesignManagementError> {
    if revision == 0 {
        Err(DesignManagementError::ZeroRevision { domain, identity })
    } else {
        Ok(())
    }
}

pub(super) fn require_revision(
    expected: u64,
    actual: u64,
    domain: &'static str,
    identity: String,
) -> Result<(), DesignManagementError> {
    if expected == actual {
        Ok(())
    } else {
        Err(DesignManagementError::RevisionConflict {
            domain,
            identity,
            expected,
            actual,
        })
    }
}

pub(super) fn next_revision(
    revision: u64,
    domain: &'static str,
    identity: String,
) -> Result<u64, DesignManagementError> {
    revision
        .checked_add(1)
        .ok_or(DesignManagementError::RevisionExhausted { domain, identity })
}

pub(super) fn require_digest(
    actual: ContentDigest,
    expected: ContentDigest,
    domain: &'static str,
    identity: String,
) -> Result<(), DesignManagementError> {
    if actual == expected {
        Ok(())
    } else {
        Err(DesignManagementError::SemanticDigestMismatch { domain, identity })
    }
}

pub(super) fn digest<T: Serialize>(
    schema: &'static str,
    value: &T,
) -> Result<ContentDigest, DesignManagementError> {
    #[derive(Serialize)]
    struct Material<'a, T> {
        schema: &'static str,
        value: &'a T,
    }
    let bytes = serde_json::to_vec(&Material { schema, value })
        .map_err(|error| DesignManagementError::Serialization(error.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(ContentDigest::from_bytes(hasher.finalize().into()))
}

pub(super) fn digest_infallible<T: Serialize>(schema: &'static str, value: &T) -> ContentDigest {
    digest(schema, value).expect("serializing a compile-time domain structure cannot fail")
}

pub(super) const fn empty_digest() -> ContentDigest {
    ContentDigest::from_bytes([0; 32])
}
