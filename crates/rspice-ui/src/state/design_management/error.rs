//! Exhaustive errors for governed design-management operations.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DesignManagementError {
    #[error("{domain} schema {actual} is unsupported")]
    UnsupportedSchema { domain: &'static str, actual: u16 },
    #[error("{domain} contains {actual} entries; maximum is {maximum}")]
    LimitExceeded {
        domain: &'static str,
        actual: usize,
        maximum: usize,
    },
    #[error("{0} identity must not be nil")]
    NilIdentity(&'static str),
    #[error("{domain} identity {identity} is duplicated")]
    DuplicateIdentity {
        domain: &'static str,
        identity: String,
    },
    #[error("{domain} {identity} has revision zero")]
    ZeroRevision {
        domain: &'static str,
        identity: String,
    },
    #[error("{domain} {identity} revision conflict: expected {expected}, current is {actual}")]
    RevisionConflict {
        domain: &'static str,
        identity: String,
        expected: u64,
        actual: u64,
    },
    #[error("{domain} {identity} revision space is exhausted")]
    RevisionExhausted {
        domain: &'static str,
        identity: String,
    },
    #[error("{domain} {identity} semantic digest does not match its content")]
    SemanticDigestMismatch {
        domain: &'static str,
        identity: String,
    },
    #[error("{domain} name {name:?} is already in use")]
    DuplicateName { domain: &'static str, name: String },
    #[error("{domain} references missing identity {identity}")]
    MissingReference {
        domain: &'static str,
        identity: String,
    },
    #[error("a non-empty {0} catalog requires an active selection")]
    ActiveSelectionRequired(&'static str),
    #[error("the active {0} cannot be removed")]
    ActiveRemoval(&'static str),
    #[error("{0} has no semantic changes")]
    NoChanges(&'static str),
    #[error("{field} value {value:?} is empty, padded, or contains control characters")]
    InvalidText { field: &'static str, value: String },
    #[error("{field} contains {actual} bytes; maximum is {maximum}")]
    TextTooLong {
        field: &'static str,
        actual: usize,
        maximum: usize,
    },
    #[error("{field} path {value:?} is not canonical")]
    InvalidPath { field: &'static str, value: String },
    #[error("{field} contains duplicate entry {value:?}")]
    DuplicateListEntry { field: &'static str, value: String },
    #[error("{0} is outside the supported numeric range")]
    NumericRange(&'static str),
    #[error("explicit sheet page number {0} is duplicated")]
    DuplicateSheetPage(u32),
    #[error("sheet reorder must contain every current sheet exactly once")]
    InvalidSheetOrder,
    #[error("schematic object identity must be greater than zero")]
    ZeroSchematicObject,
    #[error("schematic object {0} is duplicated")]
    DuplicateSchematicObject(u64),
    #[error("project schematic object {0} is duplicated")]
    DuplicateScopedSchematicObject(SchematicObjectKey),
    #[error("schematic object {0} has no explicit sheet assignment")]
    UnassignedSchematicObject(u64),
    #[error("the selection spans more than one source sheet")]
    MixedSourceSheets,
    #[error("the operation requires at least one selected object")]
    EmptySelection,
    #[error("explicit boundary-port resolution must contain at least one port")]
    EmptyExplicitBoundaryPorts,
    #[error("a boundary port does not connect the source and destination sheets")]
    BoundaryPortOutsideMove,
    #[error("a cross-sheet port must connect two distinct sheets")]
    CrossSheetPortSameSheet,
    #[error("the cross-sheet port signal type and discipline are incompatible")]
    IncompatiblePortContract,
    #[error("a cross-sheet port already owns the same net and endpoints")]
    DuplicateCrossSheetPort,
    #[error(
        "cross-sheet port anchor object {object_id} belongs to {actual:?}; expected sheet {expected}"
    )]
    CrossSheetPortAnchorSheetMismatch {
        object_id: u64,
        expected: SheetId,
        actual: Option<SheetId>,
    },
    #[error("assembly variant {0} has dependent child variants and is immutable")]
    VariantHasDependents(AssemblyVariantId),
    #[error("assembly variant parent chain contains a cycle at {0}")]
    VariantParentCycle(AssemblyVariantId),
    #[error("assembly variant {child} retains a stale parent snapshot {parent}")]
    StaleVariantParent {
        child: AssemblyVariantId,
        parent: AssemblyVariantId,
    },
    #[error("a variant cannot be compared with itself")]
    SameVariantComparison,
    #[error("variant matrix cell ({variant}, object {object}) is duplicated")]
    DuplicateVariantMatrixCell {
        variant: AssemblyVariantId,
        object: SchematicObjectKey,
    },
    #[error("object {0} has no replacement and the matrix policy blocks missing replacements")]
    MissingReplacement(SchematicObjectKey),
    #[error("object {0} replacement is not qualified")]
    UnqualifiedReplacement(SchematicObjectKey),
    #[error("annotation reserved range {first}..{last} is invalid")]
    InvalidAnnotationRange { first: u32, last: u32 },
    #[error("annotation reserved range must name at least one prefix")]
    EmptyAnnotationPrefixes,
    #[error("annotation reserved ranges overlap for a shared scope and prefix")]
    OverlappingAnnotationRanges,
    #[error("annotation prefix {0:?} must contain 1..={MAX_PREFIX_BYTES} ASCII letters")]
    InvalidAnnotationPrefix(String),
    #[error("reference designator {0:?} is invalid")]
    InvalidReferenceDesignator(String),
    #[error("reference designator {0:?} is duplicated")]
    DuplicateReferenceDesignator(String),
    #[error("annotation range for prefix {0:?} is exhausted")]
    AnnotationRangeExhausted(String),
    #[error("renumber scope contains no eligible objects")]
    EmptyRenumberScope,
    #[error("protected reference on object {0} requires explicit review")]
    ProtectedReferenceReviewRequired(SchematicObjectKey),
    #[error("renumber preview is stale relative to policy or schematic objects")]
    StaleRenumberPreview,
    #[error("annotation journal sequence is {actual}; expected {expected}")]
    InvalidAnnotationSequence { expected: u64, actual: u64 },
    #[error("annotation object authority contains a redirect cycle at {0}")]
    AnnotationAuthorityCycle(SchematicObjectKey),
    #[error("annotation authority merges unrelated objects {first} and {second} into {target}")]
    AnnotationAuthorityConflation {
        target: SchematicObjectKey,
        first: SchematicObjectKey,
        second: SchematicObjectKey,
    },
    #[error("annotation request refers to renamed or deleted object authority {0}")]
    InactiveAnnotationObjectAuthority(SchematicObjectKey),
    #[error("hierarchy audit requires at least one subject")]
    EmptyHierarchyAudit,
    #[error("hierarchy audit instance path {0:?} is duplicated")]
    DuplicateHierarchyPath(String),
    #[error("protected boundary evidence {0:?} is duplicated")]
    DuplicateProtectedBoundary(String),
    #[error("hierarchy audit sequence is {actual}; expected {expected}")]
    InvalidHierarchyAuditSequence { expected: u64, actual: u64 },
    #[error("cell-view key {0:?} is invalid")]
    InvalidCellViewKey(String),
    #[error("schematic object key {0:?} is invalid")]
    InvalidSchematicObjectKey(String),
    #[error("cell-view {field} segment {value:?} is invalid")]
    InvalidCellViewSegment { field: &'static str, value: String },
    #[error("cell-view key {0:?} is not stored canonically")]
    NonCanonicalCellViewKey(String),
    #[error("sheet-catalog ownership key {0:?} already exists")]
    SheetCatalogOwnershipCollision(String),
    #[error("sheet {0} remains referenced by the annotation policy")]
    SheetCatalogReferenced(SheetId),
    #[error("assembly variant {variant} still owns deleted schematic object {object}")]
    LiveVariantObjectReference {
        variant: AssemblyVariantId,
        object: SchematicObjectKey,
    },
    #[error("cell-view key {0:?} already has a sheet catalog")]
    AlreadyBootstrapped(String),
    #[error("design-management data could not be serialized: {0}")]
    Serialization(String),
}
