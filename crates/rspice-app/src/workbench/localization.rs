//! Repository-owned UI message catalog.
//!
//! Engineering identifiers, paths, expressions, and source text are never
//! translated. Callers provide those as named arguments; pseudolocalization
//! expands only catalog-authored literal segments so qualification cannot
//! corrupt data while exercising layout, truncation, and focus behavior.

use std::collections::HashMap;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default,
)]
#[serde(rename_all = "kebab-case")]
pub enum UiTextLocale {
    #[default]
    EnglishUnitedStates,
    /// Layout-qualification locale. It is deliberately persisted so native
    /// and browser screenshot campaigns can restart in the same mode.
    PseudoExpanded,
}

impl UiTextLocale {
    #[cfg(test)]
    pub const ALL: [Self; 2] = [Self::EnglishUnitedStates, Self::PseudoExpanded];

    #[cfg(test)]
    pub const fn label(self) -> &'static str {
        match self {
            Self::EnglishUnitedStates => "English (United States)",
            Self::PseudoExpanded => "Pseudolocalized (qualification)",
        }
    }
}

macro_rules! define_messages {
    ($($id:ident => $template:expr,)+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum MessageId {
            $($id,)+
        }

        impl MessageId {
            #[cfg(test)]
            pub const ALL: &'static [Self] = &[$(Self::$id,)+];

            const fn english_template(self) -> &'static str {
                match self {
                    $(Self::$id => $template,)+
                }
            }

            #[cfg(test)]
            const fn stable_key(self) -> &'static str {
                match self {
                    $(Self::$id => stringify!($id),)+
                }
            }
        }
    };
}

define_messages! {
    EditorGeneratedSpiceNetlistViewer => "Generated SPICE netlist viewer",
    EditorProjectSpiceNetlistEditor => "Project SPICE netlist editor",
    EditorProjectSpiceNetlistViewer => "Project SPICE netlist viewer",
    EditorGeneratedSpiceComparisonViewer => "Generated SPICE comparison viewer",
    EditorRunSnapshotSpiceNetlistViewer => "Run snapshot SPICE netlist viewer",
    EditorVerilogASourceEditor => "Verilog-A source editor",
    EditorVerilogASourceViewer => "Verilog-A source viewer",
    EditorAutomationSourceEditor => "RSpice Automation source editor",
    EditorAutomationSourceViewer => "RSpice Automation source viewer",
    EditorPythonSourceEditor => "Python source editor",
    EditorPythonSourceViewer => "Python source viewer",
    EditorRunPlanEditor => "RSpice run-plan editor",
    EditorRunPlanViewer => "RSpice run-plan viewer",
    EditorTomlManifestEditor => "TOML manifest editor",
    EditorTomlManifestViewer => "TOML manifest viewer",
    EditorAccessiblePositionEditable => "Line {line} of {total_lines}, column {column}. Editable.",
    EditorAccessiblePositionReadOnly => "Line {line} of {total_lines}, column {column}. Read-only.",
    EditorAccessibleOneCharacterSelected => "One character selected.",
    EditorAccessibleCharactersSelected => "{count} characters selected.",
    EditorAccessibleSelectionBeyondExcerpt => "The selection extends beyond the active-line excerpt.",
    EditorAccessibleMultipleCursors => "{count} cursors active.",
    EditorAccessibleBreakpoint => "Breakpoint on this line.",
    EditorAccessibleBreakpointToggleHint => "Press F9 to toggle a breakpoint on the active line.",
    EditorAccessibleCurrentStatement => "Current debugger statement.",
    EditorAccessibleCrossProbe => "Cross-probed source line.",
    EditorAccessibleModifiedLine => "Modified line.",
    EditorAccessibleSourceExcerpt => "Source excerpt columns {first_column} through {last_column} of {total_columns}.",
    EditorAccessibleSeverityHint => "hint",
    EditorAccessibleSeverityInformation => "information",
    EditorAccessibleSeverityWarning => "warning",
    EditorAccessibleSeverityError => "error",
    NetlistFindActiveDocument => "Find in active netlist document",
    NetlistSearchUnavailable => "Open or generate a netlist document before searching.",
    NetlistReturnRoot => "Return to root",
    NetlistTopDeck => "Top deck",
    NetlistTopDeckCatalog => "Project top decks",
    NetlistPublishedOrigin => "Published origin",
    NetlistNotPublished => "Not published to a reopenable native path",
    NetlistCopyProject => "Copy to project",
    NetlistRelink => "Relink…",
    NetlistRelinkTooltip => "Relink dependency source or permission",
    NetlistOpenEditable => "Open editable source",
    NetlistOpenEditableCompact => "Open source",
    NetlistCreateEditable => "Create editable source from generated netlist…",
    NetlistCreateEditableCompact => "Create source…",
    NetlistSaveSourceDeck => "Save source deck",
    NetlistValidateSource => "Validate source",
    NetlistReturnPrimary => "Return to primary",
    NetlistReturnGeneratedPrimary => "Return to generated primary",
    NetlistGenerateBeforeAction => "Generate a primary netlist before using this action.",
    NetlistValidateBeforeSave => "No modified owned-source bytes need to be saved.",
    NetlistLanguageGenerated => "SPICE · GENERATED · READ-ONLY · SOURCE MAPPED",
    NetlistLanguageOwned => "SPICE · OWNED · EDITABLE",
    NetlistLanguageParameterOverride => "SPICE INCLUDE · PARAMETER OVERRIDE · EDITABLE",
    NetlistLanguageIncludeOrderOverride => "SPICE INCLUDE · INCLUDE ORDER · EDITABLE",
    NetlistLanguageAnalysisDeck => "SPICE · ANALYSIS DECK · EDITABLE",
    NetlistLanguageDiff => "SPICE DIFF · GENERATED COMPARISON · READ-ONLY",
    NetlistLanguageOwnedInclude => "SPICE INCLUDE · PROJECT OWNED · EDITABLE",
    NetlistLanguageExternalInclude => "SPICE INCLUDE · EXTERNAL REFERENCE · READ-ONLY",
    NetlistLanguageVendorInclude => "SPICE INCLUDE · VENDOR SOURCE · READ-ONLY",
    NetlistLanguageTechnologyInclude => "SPICE INCLUDE · TECHNOLOGY PACKAGE · READ-ONLY",
    NetlistLanguageStandardInclude => "SPICE INCLUDE · STANDARD LIBRARY · READ-ONLY",
    NetlistGenerationBlocked => "generation blocked",
    NetlistGenerationStaleBlocked => "stale · generation blocked",
    NetlistSyntaxErrorSingular => "{count} syntax error",
    NetlistSyntaxErrors => "{count} syntax errors",
    NetlistErrors => "Errors",
    NetlistAdvisories => "Advisories",
    NetlistMoreInProblems => "{count} more in Problems",
    NetlistParameterExploration => "Parameter exploration",
    NetlistTuneParameters => "Tune parameters…",
    NetlistIncludeProvenance => "Include",
    NetlistDisplayName => "Name",
    NetlistLogicalIdentity => "Logical identity",
    NetlistRequestedAs => "Written as",
    NetlistOrigin => "From",
    NetlistRetainedAuthenticatedSource => "Stored in the project",
    NetlistRelationship => "Relationship",
    NetlistDirectInclude => "Direct include",
    NetlistTransitiveInclude => "Transitive include",
    NetlistParent => "Parent",
    NetlistOwnership => "Ownership",
    NetlistProjectOwnedEditable => "Project-owned · editable",
    NetlistExternalReferenceReadOnly => "External reference · read-only",
    NetlistVendorSourceReadOnly => "Vendor source · read-only",
    NetlistTechnologyPackageReadOnly => "Technology package · read-only",
    NetlistStandardLibraryReadOnly => "Standard library · read-only",
    NetlistUnresolved => "Not found",
    NetlistCopySucceeded => "Copied the include into the project. Edits now apply when the deck runs and are saved with the project.",
    NetlistReleaseProjectOwnership => "Release project ownership",
    NetlistReleaseOwnershipDescription => "Keep the include line and the file's current text, but make this document a read-only reference again. It can be copied into the project again later.",
    NetlistReleaseSucceeded => "Released project ownership of the include. Its text and include line were kept, read-only.",
    NetlistRelinkSucceeded => "Relinked the dependency source and stored it with the project.",
    NetlistRootValidationRequired => "An include file cannot be validated on its own. Return to the root deck to validate it with all of its includes.",
    NetlistDependencyCannotRun => "Dependency documents cannot run on their own; return to the root deck, validate it with its includes, and run it.",
    NetlistRunDeck => "Run deck",
    NetlistStopRun => "Stop",
    NetlistRunStripRun => "Run {id}",
    NetlistRunStripEdited => "Run {id} · edited",
    NetlistRunStripRunning => "Run {id} · running",
    NetlistRunStripCurrentTooltip => "This deck is byte-for-byte the deck Run {id} executed.",
    NetlistRunStripEditedTooltip => "This deck has been edited since Run {id} executed it.",
    NetlistRunStripRunningTooltip => "Run {id} is executing this deck. Its deck and results are sealed when it finishes.",
    NetlistRunStripIdentity => "deck {digest}… · revision {revision}",
    NetlistRunStripIdentityShort => "{digest}… · {revision}",
    NetlistRunStripExecutedPoint => "executed deck · {point} · revision {revision}",
    NetlistRunStripExecutedPointShort => "{point} · {revision}",
    NetlistRunStripOpenDeck => "Open the deck this run used",
    NetlistRunStripOpenResults => "Open in Results",
    NetlistRunStripOpenTaskDeck => crate::workbench::commands::vocabulary::OPEN_TASK_DECK,
    NetlistRunStripOpenTaskDeckTooltip => "Opens the deck Run {id} gave its first task, as a read-only document saved with the run.",
    NetlistRunStripNextTask => "Task {index}/{count}",
    NetlistRunStripNextTaskTooltip => "Showing task {index} of {count}. Opens the next task's executed deck.",
    NetlistRunStripDeckUnmatched => "Differs from the run's deck",
    NetlistRunStripDeckUnmatchedTooltip => "Nothing retained for Run {id} reproduces the source digest sealed on its receipt. Whatever this deck is, the run was not authorized over it.",
    NetlistRunSnapshotCompare => "Compare",
    NetlistRunSnapshotCompareTooltip => "Compare this run's deck with the working revision",
    NetlistRunSnapshotOpenRun => "Open Run {id}",
    NetlistRunSnapshotDocument => "Run {id} deck",
    NetlistLanguageRunSnapshot => "SPICE · RUN SNAPSHOT · READ-ONLY",
    CommonCancel => "Cancel",
    CommonClose => "Close",
    CommonFind => "Find",
    CommonReplace => "Replace",
    CommonScope => "Scope",
    NetlistGeneratedUnavailable => "Generated netlist unavailable",
    NetlistGeneratedUnavailableDescription => "Add a circuit before generating the primary netlist.",
    NetlistEmptyWorkspaceTitle => "Start a SPICE top deck",
    NetlistEmptyWorkspaceDescription => "Create an authored project-owned deck or import an existing SPICE file. No schematic or generated baseline is required.",
    NetlistNewTopDeck => "New top deck…",
    NetlistImportDeck => "Import deck…",
    NetlistOwnershipCreateSourceTitle => "Create editable source from generated netlist",
    NetlistOwnershipCreateOverrideTitle => "Create generated-netlist override",
    NetlistOwnershipCreateSource => "Create owned source",
    NetlistOwnershipCreatePatch => "Create override patch",
    NetlistOwnershipDescription => "Create an editable source in the project. The generated netlist stays unchanged; the new source records the revision it was based on and how it follows regeneration.",
    NetlistOwnershipNotice => "The netlist generated from the schematic stays read-only. The editable copy records its base revision, ownership, and how it follows regeneration.",
    NetlistArtifactName => "Artifact name",
    NetlistBaseRevision => "Base revision",
    NetlistEditStrategy => "Edit strategy",
    NetlistStrategyOwnedSource => "Owned source derived from generated output",
    NetlistStrategyParameterOverride => "Parameter and option override",
    NetlistStrategyIncludeOverride => "Include-order override",
    NetlistStrategyAnalysisDeck => "Analysis-only deck",
    NetlistStrategyOwnedSourceDescription => "Own the complete deck as an independently editable source revision.",
    NetlistStrategyParameterOverrideDescription => "Own only .param, .option, and .temp cards; execution composes them with the frozen generated base.",
    NetlistStrategyIncludeOverrideDescription => "Own include and library ordering while retaining the frozen generated circuit and analyses.",
    NetlistStrategyAnalysisDeckDescription => "Own analysis, measurement, save, and probe cards while retaining the frozen generated circuit.",
    NetlistComparisonOwnedTitle => "Owned source history",
    NetlistComparisonGeneratedTitle => "Compare generated revisions",
    NetlistCompareRevisions => "Compare revisions",
    NetlistComparisonOwnedDescription => "Compare a content-complete project revision with the current owned source, or restore it as a new monotonic working revision.",
    NetlistComparisonGeneratedDescription => "Select an earlier generated revision and compare its source with the current generated netlist.",
    NetlistRestoreAsNewRevision => "Restore as new revision",
    NetlistComparisonOwnedPrompt => "Select a retained project-owned source revision. Restore never deletes the current working state; it journals that state first.",
    NetlistComparisonGeneratedPrompt => "Select an earlier generated revision to compare with the current generated netlist.",
    NetlistPriorRevision => "Prior revision",
    NetlistOwnedRevisionChoice => "Revision {revision} · {digest}… · {message}",
    NetlistGeneratedRevisionChoice => "Input revision {revision} · {digest}…",
    NetlistSaveTitle => "Save owned source",
    NetlistSave => "Save source",
    NetlistSaveSourceAs => "Save source as…",
    NetlistDownloadSourceCopy => "Download source copy",
    NetlistDownloadSourceDescription => "Download the source as a separate file copy. A download does not save the project; save the RSpice project to keep the working source and its history.",
    NetlistSaveDescription => "Save the source as written, without changing validation, run readiness, or the generated netlist.",
    NetlistOwnedSpiceSource => "Owned SPICE source",
    NetlistSaveValidatedNotice => "Source persistence is independent from validation and execution. Incomplete or temporarily invalid authored work can be saved safely.",
    NetlistSaveStaleNotice => "The modified authored bytes are ready to save; validation status will not change.",
    NetlistGeneratedBaseRevision => "Generated base {digest}… · owned revision {revision}",
    NetlistSaveImmutableNotice => "Saving never mutates or rebases the generated primary.",
    NetlistRevisionMessage => "Revision note (optional)",
    NetlistRevisionMessageInvalid => "Use one line of at most 240 characters.",
    NetlistAlreadyPublished => "This source is already saved.",
    NetlistExternalConflictFree => "Automatic three-way merge is conflict-free",
    NetlistExternalConflictResolutionRequired => "Conflict markers must be resolved in the editor before validation",
    NetlistExternalKeepLocalHint => "The external bytes are acknowledged; local source remains dirty",
    NetlistExternalReloadHint => "External bytes become the current clean editor source",
    NetlistExternalTitle => "Resolve externally modified source",
    NetlistExternalApply => "Apply to editor",
    NetlistExternalDescription => "The file changed after RSpice imported or saved it. Choose an explicit resolution; the current working state is journaled before any source transition.",
    NetlistExternalEvidenceSummary => "Expected SHA-256 {expected}… · observed {observed}… · {encoding}",
    NetlistExternalResolution => "Resolution",
    NetlistExternalMerge => "Three-way merge into editor",
    NetlistExternalKeepLocal => "Keep local editor source",
    NetlistExternalReload => "Reload external source",
    NetlistExternalEvidence => "Three-way merge",
    NetlistExternalMergeConflictsSingular => "A retained publication base is available. The merge produced {count} conflict.",
    NetlistExternalMergeConflicts => "A retained publication base is available. The merge produced {count} conflicts.",
    NetlistExternalNoBase => "No retained publication base is available; merge is a fail-closed two-way conflict.",
    NetlistExternalPreviewLimited => "Candidate preview is limited to 200 lines; applying uses the complete retained bytes.",
    NetlistExternalComparison => "Local ↔ external comparison",
    NetlistExportBundle => "Export bundle",
    NetlistExportDeck => "Export generated deck",
    NetlistDependencySealedSingular => "{count} dependency · sealed",
    NetlistDependencySealed => "{count} dependencies · sealed",
    NetlistDependencyResolutionSingular => "{count} dependency · resolution required",
    NetlistDependencyResolution => "{count} dependencies · resolution required",
    NetlistExportTitle => "Export generated netlist",
    NetlistExportDescription => "Export the current generated revision as a deck, or as a bundle with its dependencies and a checksum manifest, without changing project source.",
    NetlistExportSnapshotNotice => "The exported deck is a snapshot of the current generated revision. Editing the export never changes the schematic or the project-owned source graph.",
    NetlistDialect => "Dialect",
    NetlistExportDependencyBundle => "Create self-contained dependency bundle (.zip)",
    NetlistExportBundleRequired => "This generated deck references project-owned source bytes and must be exported as a reproducible bundle.",
    NetlistExportSourceMap => "Include generated source map",
    NetlistExportSourceMapSpiceOnly => "Source-map positions refer to the SPICE deck before dialect translation.",
    NetlistExportExternalSealedSingular => "{count} external dependency · sealed",
    NetlistExportExternalSealed => "{count} external dependencies · sealed",
    NetlistExportExternalResolutionSingular => "{count} external dependency · resolution required",
    NetlistExportExternalResolution => "{count} external dependencies · resolution required",
    NetlistExportBundleBehavior => "Bundle paths are rewritten to deterministic internal entries and accompanied by an integrity manifest.",
    NetlistExportDeckBehavior => "A single dialect deck will be exported without dependency members.",
    NetlistImportBlockingSingular => "{count} blocking issue",
    NetlistImportBlocking => "{count} blocking issues",
    NetlistImportCompatibilityRequired => "Compatibility acceptance required",
    NetlistImportReady => "Ready to import",
    NetlistImportOpenTitle => "Open netlist-first project",
    NetlistImportDeckTitle => "Import SPICE deck",
    NetlistImportReviewProfileTitle => "Review owned-source execution profile",
    NetlistImportOpen => "Open project",
    NetlistImportRecordProfile => "Record profile",
    NetlistImportDescription => "Review the selected file, its detected dialect, the parser result, and every transformation before the project changes.",
    NetlistImportBundleSourceSingular => "RSpice bundle · {count} dependency",
    NetlistImportBundleSource => "RSpice bundle · {count} dependencies",
    NetlistImportLosslessSource => "Lossless source file",
    NetlistImportProjectSourceSnapshot => "Project source and its dependencies",
    NetlistImportSourceSummary => "{count} bytes · SHA-256 {digest}… · {encoding} · {line_ending}",
    NetlistImportBrowserSnapshot => "File selected in the browser",
    NetlistImportInvalidUnicodePath => "Source path is not valid Unicode",
    NetlistSourceDialect => "Source dialect",
    NetlistImportDetectedNoMarker => "Detected: {dialect} · no vendor-specific marker found",
    NetlistImportDetectedEvidence => "Detected: {dialect} · markers listed below",
    NetlistImportExecutionProfile => "Execution profile",
    NetlistImportExecutionProfileReceipt => "{profile} · versioned parser, device-default, and analysis contract recorded with project source",
    NetlistImportNoExecutionProfile => "No executable profile is available for the selected source dialect.",
    NetlistImportAcceptProfile => "Accept the reviewed {dialect} compatibility profile ({profile})",
    NetlistImportAcceptanceNotice => "Accepting records this execution profile and its version. It never permits silent statement translation, fallback to default behavior, or unsupported statements.",
    NetlistImportTransformations => "Transformations",
    NetlistImportValidation => "Validation",
    NetlistImportSourcePreview => "Source preview",
    NetlistImportPreviewLimited => "Preview limited to the first 200 lines; validation used the complete bounded source.",
    NetlistProfileReviewRequired => "Execution profile review required",
    NetlistProfileReviewRequiredDescription => "Simulation is blocked until the execution profile is reviewed.",
    NetlistProfileReviewAction => "Review execution profile",
    NetlistControlRegionDisposition => ".control not executed",
    NetlistControlRegionCounts => "{promoted} promoted · {dropped} dropped",
    NetlistControlRegionDetail => "RSpice runs SPICE cards and has no .control interpreter. {promoted} command(s) in this deck were promoted into declarative directives; {dropped} were read and ignored.",
    NetlistFindEnterTextHint => "Enter text to search",
    NetlistFindCorrectExpressionHint => "Correct the search expression",
    NetlistFindGeneratedImmutableHint => "Generated netlists are read-only",
    NetlistFindTitle => "Find and replace in source",
    NetlistFindNext => "Find next",
    NetlistFindDescription => "Search editable source and generated references; replacement remains limited to project-owned source artifacts.",
    NetlistFindExpressionHint => "Symbol, text, or expression",
    NetlistFindMatchCase => "Match case",
    NetlistFindWholeSymbol => "Whole symbol",
    NetlistFindRegularExpression => "Regular expression",
    NetlistFindIncludeComments => "Include comments",
    NetlistFindIncludeGeneratedReferences => "Include generated references",
    NetlistFindSelectionRequired => "Select source text before using the Selection scope.",
    NetlistFindEnterExactArtifact => "Enter text to search the current document.",
    NetlistFindMatchSingular => "{count} match",
    NetlistFindMatches => "{count} matches",
    NetlistFindFirstMatches => "First {count} matches. Narrow the search to reach the rest.",
    NetlistFindPrevious => "Previous",
    NetlistFindReplaceAll => "Replace all",
    NetlistFindCurrentDocument => "Current document",
    NetlistFindSelection => "Selection",
    NetlistFindOpenDocuments => "Open documents",
    NetlistFindActiveLanguageProject => "Active language project",
    NetlistFindProjectReferences => "Project references (find only)",
    NetlistFindReplacedSingular => "Replaced {count} match in owned SPICE source.",
    NetlistFindReplaced => "Replaced {count} matches in owned SPICE source.",
    NetlistNavigatorStructure => "Structure",
    NetlistNavigatorIncludes => "Includes",
    NetlistNavigatorSemanticIndex => "Semantic index",
    NetlistNavigatorProjectSymbols => "Project symbols",
    NetlistNavigatorSymbolSingular => "{count} symbol across {sources} source(s)",
    NetlistNavigatorSymbols => "{count} symbols across {sources} source(s)",
    NetlistNavigatorProjectReferences => "Project references",
    NetlistNavigatorReferenceSingular => "{count} reference",
    NetlistNavigatorReferences => "{count} references",
    NetlistNavigatorHierarchyInstanceSingular => "{count} elaborated instance",
    NetlistNavigatorHierarchyInstances => "{count} elaborated instances",
    NetlistNavigatorHierarchyBounded => "bounded view",
    NetlistNavigatorHierarchyRecursive => "recursive edge",
    NetlistLanguageTools => "SPICE language tools",
    NetlistSignatureHelp => "Signature help",
    NetlistGoToDeclaration => "Go to declaration",
    NetlistGoToDefinition => "Go to definition",
    NetlistFindReferences => "Find references",
    NetlistRenameSymbol => "Rename symbol…",
    NetlistWorkspaceSymbols => "Workspace symbols…",
    NetlistCodeActions => "Code actions",
    NetlistApplyQuickFix => "Apply preferred quick fix",
    NetlistFormatDocument => "Format document",
    NetlistRenameTitle => "Rename SPICE symbol",
    NetlistRenameDescription => "Rename the definition and every indexed reference to it across the project's editable netlist sources.",
    NetlistRenameNewName => "New name",
    NetlistRename => "Rename",
    NetlistRenameSucceeded => "Renamed {count} definition/reference occurrence(s) as one undoable project transaction.",
    NetlistNavigatorDirectiveSingular => "{count} directive",
    NetlistNavigatorDirectives => "{count} directives",
    NetlistNavigatorSourceMapping => "Source mapping",
    NetlistNavigatorProvenance => "provenance",
    NetlistNavigatorNoMatches => "No symbol, directive, or source line matches this filter.",
    NetlistNavigatorDependencyUnavailable => "unavailable",
    NetlistNavigatorDependencyError => "error",
    NetlistNavigatorDependencyUnresolved => "unresolved",
    NetlistNavigatorDependencyMissing => "missing",
    NetlistNavigatorAuthorityExternal => "external reference",
    NetlistNavigatorAuthorityVendor => "vendor source",
    NetlistNavigatorAuthorityTechnology => "technology package",
    NetlistNavigatorAuthorityStandard => "standard library",
    NetlistIncludeViaIncludingFile => "via including-file dir",
    NetlistIncludeViaTopLevel => "via top-level dir",
    NetlistIncludeViaExecution => "via execution dir",
    NetlistIncludeViaSearchPath => "via search path {index}",
    NetlistIncludeViaLibraryFolder => "via library folder",
    NetlistIncludeViaConventional => "via {directory}/",
    NetlistIncludeViaAbsolute => "via absolute path",
    NetlistIncludeViaDriveRelative => "via drive-relative path",
    NetlistIncludeViaRetainedBundle => "via retained bundle",
    NetlistIncludeShadowMarker => "shadows",
    NetlistIncludeChainHeader => "Search chain for {name}",
    NetlistIncludeChainWinner => "resolved here",
    NetlistIncludeChainShadowed => "also present — shadowed",
    NetlistIncludeChainAbsent => "absent",
    NetlistIncludeChainUntraced => "No resolution has been recorded for this include yet. Validate the deck to walk the search chain.",
    NetlistIncludeSearchPaths => "Include search paths",
    NetlistIncludeSearchPathCount => "{count} in order",
    NetlistIncludeSearchPathEmpty => "No search paths. Includes resolve beside the deck, then in lib/ and models/.",
    NetlistIncludeSearchPathEmptyWithLibraryFolders => "No search paths. Includes resolve beside the deck, then in the library folders set in Preferences, then in lib/ and models/.",
    NetlistIncludeSearchPathThenLibraryFolders => "Then the library folders set in Preferences.",
    NetlistIncludeSearchPathAdd => "Add",
    NetlistIncludeSearchPathBrowse => "Browse\u{2026}",
    NetlistIncludeSearchPathRemove => "Remove",
    NetlistIncludeSearchPathMoveUp => "Move up",
    NetlistIncludeSearchPathMoveDown => "Move down",
    NetlistIncludeSearchPathMissing => "missing",
    NetlistIncludeSearchPathHint => "Directory, absolute or relative to the project folder",
    NetlistIncludeSearchPathUnsaved => "Save the project before a relative search path can be placed.",
    NetlistNavigatorRevisionComparison => "Revision comparison",
    NetlistNavigatorDiffTotals => "+{added} -{removed}",
    NetlistNavigatorNoDifference => "The two revisions declare the same source.",
    NetlistNavigatorNoChangedRegion => "No changed region matches this filter.",
    NetlistNavigatorChangedRegion => "Changed region",
    NetlistNavigatorRemovedBeforeLine => "Removed before line {line}",
    NetlistNavigatorHunkLine => "Line {line}",
    NetlistNavigatorHunkLines => "Lines {first}–{last}",
    NetlistNavigatorParameters => "Parameters",
    NetlistNavigatorInstances => "Instances",
    NetlistNavigatorModels => "Model bindings",
    NetlistNavigatorAnalyses => "Analyses",
    NetlistNavigatorMeasurements => "Measurements",
    NetlistNavigatorHierarchy => "Hierarchy",
    NetlistNavigatorUnitDefinition => "definition",
    NetlistNavigatorUnitDefinitions => "definitions",
    NetlistNavigatorGlobals => "Globals",
    NetlistNavigatorUnitDeclaration => "declaration",
    NetlistNavigatorUnitDeclarations => "declarations",
    NetlistNavigatorFunctions => "Functions",
    NetlistNavigatorOptions => "Solver options",
    NetlistNavigatorUnitCard => "card",
    NetlistNavigatorUnitCards => "cards",
    NetlistNavigatorOutputs => "Save and probe",
    NetlistNavigatorUnitDirective => "directive",
    NetlistNavigatorUnitDirectives => "directives",
    NetlistNavigatorConditionals => "Conditionals",
    NetlistNavigatorControls => "Control",
    NetlistNavigatorCounted => "{count} {unit}",
    NetlistNavigatorLine => "line {line}",
    NetlistNavigatorNoProvenance => "This line has no schematic source.",
    NetlistNavigatorMappedLine => "Line {line} · {cell}",
    NetlistNavigatorInstance => "Instance {instance}",
    NetlistNavigatorCrossProbe => "Cross-probe schematic component",
    NetlistNavigatorCrossProbeSucceeded => "Cross-probed generated line {line} to component {component}.",
    NetlistShowInstanceLocated => "Located instance {instance} at generated line {line}.",
    NetlistShowInstanceStale => "Located instance {instance} at generated line {line} in a stale deck; regenerating it may move the card.",
    CodePageNetlist => "Netlist",
    CodePageAutomation => "Automation",
    CodeNavigatorFiles => "Files",
    CodeNavigatorWorkflowFiles => "Workflow files",
    CodeNavigatorNoBundle => "This project owns no source bundle for this page yet. Create or import one from the document well.",
    CodeNavigatorIncludes => "Includes",
    CodeNavigatorIncludeBuiltIn => "built-in",
    CodeNavigatorIncludeNotFound => "Not found in this cell's files or the built-in headers",
    CodeNavigatorOutline => "Outline",
    CodeNavigatorOutlineModule => "Module",
    CodeNavigatorOutlinePorts => "Ports",
    CodeNavigatorOutlineParameters => "Parameters",
    CodeNavigatorOutlineVariables => "Variables",
    CodeNavigatorOutlineNets => "Branches and nets",
    CodeNavigatorOutlineFunctions => "Analog functions",
    CodeNavigatorUsedIn => "Used in",
    CodeNavigatorRoleBuildProfile => "build profile",
    CodeNavigatorRoleEntry => "entry",
    CodeNavigatorRoleRunPlan => "run plan",
    CodeNavigatorRoleEnvironmentLock => "environment lock",
    CodeNavigatorRolePermissions => "permissions",
    CodeSourceLifecycleTitle => "Manage source document",
    CodeSourceLifecycleDescription => "Review ownership, persistence, includes, and revision history for {document}, and run the file operations this project source graph allows.",
    CodeSourceOperationsHeld => "File operations are held",
    CodeSourceReadOnlyDocument => "Read-only document",
    CodeSourceReadOnlyDocumentDetail => "This document is not editable in the active project, so operations that rewrite it are unavailable.",
    CodeSourceCurrentDocument => "Current document",
    CodeSourceOwnedReadOnly => "project-owned · read-only",
    CodeSourcePath => "Path",
    CodeSourceIdentity => "Identity",
    CodeSourceIdentityNotRetained => "not retained",
    CodeSourceContent => "Content",
    CodeSourceContentShape => "{lines} lines · {bytes} bytes",
    CodeSourceLineEndings => "Line endings",
    CodeSourceLineEndingsNone => "none · single line",
    CodeSourceLineEndingsLf => "LF",
    CodeSourceLineEndingsCrLf => "CRLF",
    CodeSourceLineEndingsMixed => "mixed LF and CRLF",
    CodeSourceBundle => "Bundle",
    CodeSourceValidation => "Validation",
    CodeSourceValidationStale => "Out of date",
    CodeSourceRecordedRole => "Recorded role",
    CodeSourceRoleOrdinary => "ordinary source",
    CodeSourceRoleBuildProfile => "Verilog-A build profile",
    CodeSourceRoleAutomationEntry => "Automation entry",
    CodeSourceRoleRunPlan => "Automation run plan",
    CodeSourceRoleEnvironmentLock => "Automation environment lock",
    CodeSourceRolePermissionManifest => "Automation permission manifest",
    CodeSourceFileOperations => "File operations",
    CodeSourceNewFile => "New source…",
    CodeSourceRename => "Rename…",
    CodeSourceMove => "Move…",
    CodeSourceDuplicate => "Duplicate…",
    CodeSourceDelete => "Delete…",
    CodeSourceRevisionHistory => "Revision history…",
    CodeSourceImportDependency => "Import dependency…",
    CodeSourceReplaceFromFile => "Replace from file…",
    CodeSourceRootDeleteBlocked => "The bundle root cannot be deleted; rename or replace it instead.",
    CodeSourceEnvironmentLockMutationBlocked => "The managed Automation environment lock is read-only. Reassign its role above, or duplicate it, before changing the source graph.",
    CodeSourceCreateWorkspace => "Create source workspace…",
    CodeSourceWorkspaceTitle => "Create source workspace",
    CodeSourceWorkspacePrimary => "Create workspace",
    CodeSourceWorkspaceDescription => "Create the project-owned {language} source closure. Every path is portable and logical; none of these names carries semantic authority on its own.",
    CodeSourceWorkspaceRefused => "The source workspace cannot be created",
    CodeSourceWorkspaceClosure => "Closure",
    CodeSourceWorkspaceRootPath => "Root document",
    CodeSourceWorkspaceRunPlanPath => "Run plan",
    CodeSourceWorkspaceEnvironmentLockPath => "Environment lock",
    CodeSourceWorkspacePermissionPath => "Permission manifest",
    CodeSourceWorkspaceBoundDeck => "Bound deck",
    CodeInspectorNetlistTitle => "Netlist problems",
    CodeInspectorAutomationTitle => "Script",
    CodeInspectorContextUnavailable => "No script selected.",
    CodeInspectorOwnership => "Ownership",
    CodeInspectorGeneratedReadOnly => "Generated · read-only",
    CodeInspectorProjectOwned => "Project-owned · editable",
    CodeInspectorExternalReadOnly => "External · read-only",
    CodeInspectorComparisonReadOnly => "Comparison · read-only",
    CodeInspectorGovernedReadOnly => "Governed · read-only",
    CodeSourceCreateTitle => "New source document",
    CodeSourceRenameTitle => "Rename document",
    CodeSourceMoveTitle => "Move document",
    CodeSourceDuplicateTitle => "Duplicate document",
    CodeSourceDeleteTitle => "Delete document",
    CodeSourceCreatePrimary => "Create",
    CodeSourceRenamePrimary => "Rename",
    CodeSourceMovePrimary => "Move",
    CodeSourceDuplicatePrimary => "Duplicate",
    CodeSourceDeletePrimary => "Delete",
    CodeSourceRemoving => "Removing",
    CodeSourceLogicalPath => "Logical path",
    CodeSourceAgainst => "Against",
    CodeSourceAgainstRevision => "revision {revision} · {digest}",
    CodeSourceDeleteWarning => "Removing a document rewrites the source graph as a new bundle revision.",
    CodeSourceRecordedIncludes => "Recorded includes",
    CodeSourceNoIncludes => "This document has no recorded include edge in either direction.",
    CodeSourceImports => "Imports",
    CodeSourceImportedBy => "Imported by",
    CodeSourceRetainedRevisions => "Retained bundle revisions",
    CodeSourceNoRetainedRevision => "This bundle has no retained revision to restore yet.",
    CodeSourceRetainedClosures => "Retained closures",
    CodeSourceNewestLast => "newest last",
    CodeSourceRevision => "revision {revision}",
    CodeSourceDocumentSingular => "{count} document",
    CodeSourceDocuments => "{count} documents",
    CodeSourceCurrentClosure => "current",
    CodeSourceRevisionAccessible => "Revision {revision}, {digest}, {documents}.",
    CodeSourceRevisionAccessibleCurrent => "Revision {revision}, {digest}, {documents}. This is the current closure.",
    CodeSourceRestoreAsNewRevision => "Restore as new revision",
    CodeSourceCloseHistory => "Close history",
    CodeSearchTitle => "Find in source",
    CodeSearchDescription => "Search the project-owned source closure. Finding changes nothing; replacement is one atomic bundle revision that skips every ineligible source.",
    CodeSearchCannotRun => "This search cannot run",
    CodeSearchReplacementHeld => "Replacement held",
    CodeSearchQuery => "Query",
    CodeSearchActiveDocument => "Active document",
    CodeSearchReplaceWith => "Replace with",
    CodeSearchReplaceUnavailable => "unavailable · this document is read-only",
    CodeSearchSelection => "Selection",
    CodeSearchCurrentDocument => "Current document",
    CodeSearchOpenDocuments => "Open documents",
    CodeSearchActiveLanguage => "Active language project",
    CodeSearchMatchCase => "Match case",
    CodeSearchWholeSymbol => "Whole symbol",
    CodeSearchRegularExpression => "Regular expression",
    CodeSearchIncludeComments => "Include comments",
    CodeSearchIncludeGenerated => "Include generated deck · find only",
    CodeSearchIncludeGeneratedHint => "The generated deck is only searched alongside the whole language project.",
    CodeSearchReplaceAllEligible => "Replace all eligible",
    CodeSearchReadOnlyMatchSingular => "{count} match sits in a read-only source and stays unchanged.",
    CodeSearchReadOnlyMatches => "{count} matches sit in read-only sources and stay unchanged.",
    CodeSearchMatchesHeading => "Matches",
    CodeSearchMatchSingular => "{count} match",
    CodeSearchMatches => "{count} matches",
    CodeSearchNoMatches => "No match in the selected scope.",
    CodeSearchResultAccessible => "{path}, line {line}, column {column}. {preview}",
    CodeSearchResultAccessibleReadOnly => "{path}, line {line}, column {column}, in a read-only source. {preview}",
    CodeSearchShowingFirst => "Showing the first {limit} of {count} matches. Narrow the query to see the rest.",
    CodeSearchShowingFirstStreamStopped => "Showing the first {limit} of {count} matches, and the search stopped at its streaming limit. Narrow the query to see the rest.",
    CodeToolsTitle => "Source navigation and language tools",
    CodeToolsDescription => "Navigate symbols, references, and diagnostics across the indexed source files. Every answer names the revision it was computed from.",
    CodeToolsTransactionRefused => "The language transaction was refused",
    CodeToolsIndexBehind => "Index is behind the project",
    CodeToolsStaleIndex => "The source bundle changed after this index was built. Navigation still answers from the indexed revision; reopen the tools to write against the current one.",
    CodeToolsIndexedClosure => "Indexed closure",
    CodeToolsIndexRevision => "revision {revision}",
    CodeToolsDocument => "Document",
    CodeToolsIndexedSymbols => "Indexed symbols",
    CodeToolsSymbol => "Symbol",
    CodeToolsDocumentSymbols => "Document symbols",
    CodeToolsCompletion => "Completion",
    CodeToolsHover => "Hover",
    CodeToolsSignatureHelp => "Signature help",
    CodeToolsDefinitions => "Definitions",
    CodeToolsDeclarations => "Declarations",
    CodeToolsReferences => "References",
    CodeToolsWorkspaceSymbols => "Workspace symbols",
    CodeToolsCodeActions => "Code actions",
    CodeToolsResultAccessible => "{name}, {kind}, {path} line {line}. {detail}",
    CodeToolsNoAnswer => "The indexed closure answers nothing for this name.",
    CodeToolsHoverNoSymbol => "No indexed symbol matches this name.",
    CodeToolsSignatureNoCall => "The caret is not inside a call this index can resolve.",
    CodeToolsNoCompletion => "Nothing completes the identifier at the captured caret.",
    CodeToolsNoCodeActions => "This document already satisfies every available action.",
    CodeToolsApply => "Apply",
    CodeToolsRenameSymbol => "Rename symbol",
    CodeToolsRenaming => "Renaming",
    CodeToolsRenameTo => "New name",
    CodeToolsRenameOccurrenceSingular => "Rename {count} occurrence",
    CodeToolsRenameOccurrences => "Rename {count} occurrences",
    CodeToolsRenameNotAllowed => "Rename rewrites project source, which this document or this index revision does not allow.",
    CodeToolsRenameAtomic => "Rename is atomic: it refuses outright if any occurrence sits in a governed read-only source.",
    CodeToolsDefinitionSuffix => "definition",
    CodeToolsReferenceSuffix => "reference",
    CodeSymbolKindModule => "module",
    CodeSymbolKindFunction => "function",
    CodeSymbolKindClass => "class",
    CodeSymbolKindParameter => "parameter",
    CodeSymbolKindPort => "port",
    CodeSymbolKindNet => "net",
    CodeSymbolKindVariable => "variable",
    CodeSymbolKindImport => "import",
    CodeSymbolKindConfiguration => "configuration",
    CodeSymbolKindDiscipline => "discipline",
    CodeSymbolKindNature => "nature",
    CodeGeneratedNetlist => "Generated netlist",
    VerilogANoSource => "No Verilog-A source",
    VerilogADropOneSource => "Drop one Verilog-A source at a time; each drop is recorded as its own source change.",
    VerilogAImportTitle => "Import Verilog-A",
    VerilogAImportPrimary => "Create cells",
    VerilogAImportDescription => "Each top module the file declares becomes a cell with a veriloga view that owns a copy of the file.",
    VerilogAImportFile => "File",
    VerilogAImportLibrary => "Library",
    VerilogAImportCreates => "Creates",
    VerilogAImportNoLibrary => "No library can be written; create or unlock one first.",
    VerilogAImportRefused => "{file_name} cannot be read: {reason}",
    VerilogADeviceTitle => "New Verilog-A device",
    VerilogADevicePrimary => "Create",
    VerilogADeviceDescription => "Create a cell whose veriloga view holds a starter module, with a symbol generated from the module's ports.",
    VerilogADeviceLibrary => "Library",
    VerilogADeviceCell => "Cell name",
    VerilogADevicePorts => "Ports",
    VerilogADevicePortsHint => "inp:in inn:in out:out vdd vss",
    VerilogADeviceModule => "module {module}",
    VerilogADevicePortCount => "{count} ports · symbol generated from them",
    VerilogADeviceOnePort => "1 port · symbol generated from it",
    VerilogADeviceNoPorts => "no ports · the compiled module defines them",
    VerilogADeviceNoLibrary => "No library can be written; create or unlock one first.",
    VerilogADeviceCellInvalid => "Letters, digits and underscores only.",
    VerilogADeviceCellExists => "{library} already has a cell named {cell}.",
    VerilogADevicePortDirection => "{entry}: {direction} is not in, out or inout.",
    VerilogADevicePortNoDirection => "{entry}: give a direction after the colon.",
    VerilogADevicePortSupply => "{entry}: a port is in, out or inout.",
    VerilogADevicePortName => "{entry}: not a port name or a bus like d[7:0].",
    VerilogADevicePortTwice => "{port} is listed twice.",
    VerilogAOpenInDesign => "Open in Design",
    VerilogAReplaceTitle => "Replace source from file",
    VerilogAReplacePrimary => "Replace",
    VerilogAReplaceDescription => "Replace one file of this Verilog-A view's source with the text of a file from disk.",
    VerilogAReplaceView => "View",
    VerilogAReplaceFile => "Replaces",
    VerilogAReplaceWith => "With",
    VerilogAReplaceRestorable => "The current text stays in this source's revision history; Manage source document > Revision history restores it.",
    VerilogAReplaceNotRestorable => "The current text of {file} cannot be restored after this.",
    VerilogAEntryModule => "Entry module",
    VerilogABuildSettingsTitle => "Build settings",
    VerilogABuildSettingsSave => "Save",
    VerilogABuildSettingsPrimary => "Save & compile",
    VerilogABuildSettingsDescription => "How this Verilog-A view compiles: its entry module, runtime targets, include paths, preprocessor symbols and checks.",
    VerilogABuildSettingsTargets => "Targets",
    VerilogABuildSettingsGeneratedRust => "Generated Rust",
    VerilogABuildSettingsNativeJit => "Native x86-64 JIT",
    VerilogABuildSettingsFallback => "If a target fails",
    VerilogABuildSettingsUseInterpreter => "Use the interpreter",
    VerilogABuildSettingsFailCompile => "Fail the compile",
    VerilogABuildSettingsIncludePaths => "Include paths",
    VerilogABuildSettingsIncludeHint => "include/models",
    VerilogABuildSettingsAddInclude => "Add include path",
    VerilogABuildSettingsDefines => "Defines",
    VerilogABuildSettingsDefineHint => "NAME=value",
    VerilogABuildSettingsAddDefine => "Add define",
    VerilogABuildSettingsUndefines => "Undefines",
    VerilogABuildSettingsUndefineHint => "NAME",
    VerilogABuildSettingsAddUndefine => "Add undefine",
    VerilogABuildSettingsRemove => "Remove",
    VerilogABuildSettingsChecks => "Checks",
    VerilogABuildSettingsUnitsRanges => "Units and ranges",
    VerilogABuildSettingsConvergence => "Convergence",
    VerilogABuildSettingsDiscontinuities => "Discontinuities",
    VerilogABuildSettingsHiddenState => "Hidden state",
    VerilogABuildSettingsPortability => "Portability",
    VerilogABuildSettingsIncludeInvalid => "A relative path inside the source, such as include/models.",
    VerilogABuildSettingsIncludeTwice => "{path} is listed already.",
    VerilogABuildSettingsDefineShape => "Write NAME or NAME=value.",
    VerilogABuildSettingsSymbolInvalid => "{name}: letters, digits and underscores, not starting with a digit.",
    VerilogABuildSettingsDefineTwice => "{name} is defined already.",
    VerilogABuildSettingsUndefineTwice => "{name} is undefined already.",
    VerilogABuildSettingsDefinedAndUndefined => "{name} cannot be both defined and undefined.",
    VerilogABuildSettingsReadOnly => "The project is open read-only.",
    VerilogAStatusCompiling => "Compiling…",
    VerilogAStatusPassAdvisorySingular => "{count} advisory",
    VerilogAStatusPassAdvisories => "{count} advisories",
    VerilogAStatusModified => "Edited since the last compile",
    VerilogAStatusFailed => "Compile failed",
    VerilogAModuleSection => "Module",
    VerilogAModuleRow => "Module",
    VerilogAModuleAuto => "auto",
    VerilogANoBytecode => "The compile produced no bytecode, so the module cannot run.",
    VerilogAParameters => "Parameters",
    VerilogACompileRequired => "compile required",
    VerilogAErrorSingular => "{count} error",
    VerilogAErrors => "{count} errors",
    VerilogAAdvisorySingular => "{count} advisory",
    VerilogAAdvisories => "{count} advisories",
    VerilogADiagnostics => "Diagnostics",
    VerilogADiagnosticsMore => "{count} additional diagnostics are available in Problems.",
    VerilogAParameterName => "Name",
    VerilogAParameterType => "Type",
    VerilogAParameterDefault => "Default",
    VerilogAParameterRange => "Range",
    VerilogAParameterUnits => "Units",
    VerilogAParameterTypeReal => "real",
    VerilogAParameterTypeInteger => "integer",
    VerilogAParameterTypeString => "string",
    VerilogAParameterComputed => "computed",
    VerilogAParameterAnyValue => "any",
    VerilogAParametersNotCompiled => "Compile the module to list the parameters an instance takes.",
    VerilogAStatusPortsDiffer => "Ports differ from the schematic",
    VerilogAPortsRow => "Ports",
    VerilogAPortsDifferFromSchematic => "differ from schematic",
    VerilogASymbolRow => "Symbol",
    VerilogASymbolPinsMissing => "{missing} pins missing, {extra} extra",
    VerilogASymbolPinMissing => "1 pin missing, {extra} extra",
    VerilogASymbolPinsTurned => ", {count} changed direction",
    VerilogASymbolStrip => "Symbol: {drift}",
    VerilogAUpdateSymbol => "Update symbol",
    VerilogAInterfaceMismatchStrip => "Module ports differ from the cell's schematic, which defines them",
    VerilogAInterfaceRefusedStrip => "Ports not written: {reason}",
    VerilogAPlaceReady => "{cell} is ready to place · {count} ports",
    VerilogAPlaceReadyOne => "{cell} is ready to place · 1 port",
    VerilogAPlaceIn => "Place in {schematic}",
    AutomationNoSource => "No Automation source",
    AutomationNoSourceDescription => "This project does not own an Automation workflow workspace.",
    AutomationWorkspaceRequired => "Create or import an Automation source workspace first.",
    AutomationFileMissing => "Automation file missing",
    AutomationFileMissingDescription => "The selected file is not in the Automation workspace.",
    AutomationDropOneSource => "Drop one Automation project file at a time so each source-graph revision remains independently reviewable.",
    AutomationHeadingTitle => "Automation and CI console",
    AutomationValidate => "Validate",
    AutomationDryRun => "Dry run",
    AutomationValidating => "Validating…",
    AutomationStopping => "Stopping…",
    AutomationStopWorkflow => "Stop workflow",
    AutomationRunWorkflow => "Run workflow",
    AutomationStatusValidating => "validating with managed CPython",
    AutomationExecutionValidationRequired => "validation required",
    AutomationExecutionPythonDryRun => "Python dry run",
    AutomationExecutionPythonWorkflowRunning => "Python workflow running",
    AutomationExecutionPythonDryRunPassed => "Python dry run passed",
    AutomationExecutionPythonWorkflowCompleted => "Python workflow completed",
    AutomationExecutionPreflightAccepted => "preflight accepted",
    AutomationExecutionWorkflowRunning => "workflow running",
    AutomationExecutionReleasePass => "release gates pass",
    AutomationExecutionReleaseFailed => "release gates failed",
    AutomationExecutionCancelled => "workflow cancelled",
    AutomationExecutionFailed => "workflow failed",
    AutomationManagedRuntimeVerified => "managed runtime verified",
    AutomationTaskSummarySingular => "{count} task",
    AutomationTaskSummary => "{count} tasks",
    AutomationRuntimeApi => "Runtime and API",
    AutomationPython => "Python",
    AutomationRspiceApi => "RSpice API",
    AutomationEnvironment => "Environment",
    AutomationPlatform => "Platform",
    AutomationNotValidated => "not validated",
    AutomationPermissions => "Permissions",
    AutomationProjectFiles => "Project files",
    AutomationArtifactDirectory => "Artifact directory",
    AutomationNetwork => "Network",
    AutomationProcessSpawn => "Process spawn",
    AutomationSecretLogging => "Secret logging",
    AutomationDeny => "deny",
    AutomationRedact => "redact",
    AutomationDiagnostics => "Diagnostics",
    AutomationCurrentFindings => "Current findings",
    AutomationValidation => "Validation",
    AutomationExactRevisionRetained => "validated",
    AutomationRequired => "required",
    AutomationExecutionPreview => "Execution preview",
    AutomationProjectRevision => "Project revision",
    AutomationRunTarget => "Run target",
    AutomationLocalSlots => "local · {slots} slots",
    AutomationPvtPoints => "PVT points",
    AutomationReleaseChecks => "Release checks",
    AutomationConfiguredCount => "{count} configured",
    AutomationArtifacts => "Artifacts",
    AutomationArtifactCi => "CI",
    AutomationArtifactMachine => "machine",
    AutomationArtifactReport => "report",
    AutomationSessionSafety => "Session safety",
    AutomationManagedRuntimeSafety => "Python runs and debugging use only the Python runtime that RSpice manages. RSpice never falls back to a system interpreter or invents run or debug results.",
    AutomationDebug => "Debug",
    AutomationDebugInactive => "inactive",
    AutomationDebugPaused => "paused",
    AutomationDebugRunning => "running",
    AutomationDebugStopped => "stopped",
    AutomationDebugFailed => "failed",
    AutomationDebugStart => "Start",
    AutomationDebugContinue => "Continue",
    AutomationDebugPausePending => "Pause pending",
    AutomationDebugPause => "Pause",
    AutomationDebugStepOver => "Step over",
    AutomationDebugStepInto => "Step into",
    AutomationDebugStepOut => "Step out",
    AutomationDebugRestart => "Restart",
    AutomationDebugStop => "Stop",
    AutomationDebugCurrentLine => "Current line",
    AutomationDebugExceptionPolicy => "Exception policy",
    AutomationDebugAllExceptions => "all exceptions",
    AutomationDebugUncaughtExceptions => "uncaught exceptions",
    AutomationDebugNever => "never",
    AutomationBreakpoints => "Breakpoints",
    AutomationRemove => "Remove",
    AutomationBreakpointStop => "stop",
    AutomationBreakpointCondition => "condition",
    AutomationBreakpointLogpoint => "logpoint",
    AutomationBreakpointHitCount => "hit count",
    AutomationPythonCondition => "Python condition",
    AutomationLogMessageHint => "Log message or {expression} template",
    AutomationPauseAfter => "Pause after",
    AutomationHitSummary => "hits · observed {count}",
    AutomationCallStack => "Call stack",
    AutomationLocals => "Locals",
    AutomationGlobals => "Globals",
    AutomationRedacted => "<redacted>",
    AutomationWatches => "Watches",
    AutomationExpression => "expression",
    AutomationAdd => "Add",
    AutomationStructuredOutput => "Structured output",
    NetlistLibSectionBound => "section {name}",
    NetlistLibSectionUnbound => "no section · {count} available",
    NetlistLibSectionsHeader => "Sections in {name}",
    NetlistLibSectionModel => "{count} model",
    NetlistLibSectionModels => "{count} models",
    NetlistLibSectionSubcircuit => "{count} subckt",
    NetlistLibSectionSubcircuits => "{count} subckts",
    NetlistLibSectionBoundMarker => "bound by this deck",
    NetlistLibUseSection => "Use section",
    NetlistLibSectionAlreadyBound => "This card already binds this section.",
    NetlistLibSectionReadOnly => "This source is read-only; its .lib card cannot be rewritten.",
    NetlistHoverDefinedLine => "defined line {line}",
    NetlistHoverUndefinedParameter => "{name} has no definition visible here",
    NetlistHoverNotEvaluable => "not evaluable here: {reason}",
    NetlistHoverNotRead => "not read: {reason}",
    NetlistHoverDefinedOnlyUnread => "{name} is defined only on line {line}, {reason}",
    NetlistHoverUntakenBranch => "in an .if branch not taken ({condition} is {holds})",
    NetlistHoverUnselectedSection => "in .lib section {name}, not selected",
    StimulusLibraryEmpty => "No stimulus definitions",
    StimulusLibraryEmptyDetail => "This project has no stimulus definitions yet. Definitions are authored here or saved from a placed source's properties.",
    StimulusDefinitionSection => "Definition",
    StimulusSavedRevision => "saved · r{revision}",
    // The lifecycle chip where the band is too narrow to carry the whole
    // spelling. The word only; the revision and the error count are stated by
    // the realization band and the audit strip.
    StimulusSaved => "saved",
    StimulusDraft => "draft",
    StimulusFieldName => "Name",
    StimulusFieldFamily => "Family",
    StimulusFieldQuantity => "Quantity",
    StimulusFieldPurpose => "Purpose",
    StimulusBrowserEmpty => "This project has no stimulus definitions",
    StimulusBrowserNoMatch => "No definition matches this filter",
    StimulusBrowserClearFilters => "Clear filters",
    // The browser's footer band, which renders only in these states: a
    // library every placed source is in step with has nothing to report, and
    // a permanent band saying so is a band reporting the absence of a
    // problem.
    StimulusBrowserBehindSingular => "1 placed source uses an older revision",
    StimulusBrowserBehind => "{behind} placed sources use an older revision",
    StimulusAdoptersSection => "Adopters",
    StimulusAdoptersNone => "Not used in this design",
    StimulusShowInSchematic => "Show in schematic",
    StimulusOpenPropertiesShort => "Properties…",
    StimulusAdopterElsewhere => "Open the occurrence this instance is drawn in to edit or re-adopt it",
    StimulusConsumersSection => "Analysis consumers",
    StimulusConsumerDisabled => "{analysis} · {role} · disabled",
    StimulusConsumerRow => "{analysis} · {role}",
    StimulusFieldRevision => "Revision",
    StimulusRevisionDraft => "r{revision} + draft",
    StimulusRevisionSaved => "r{revision}",
    StimulusFieldModified => "Modified",
    StimulusPurposeHint => "What this stimulus is for",
    StimulusPlaceHint => "Place a source carrying this definition's saved revision",
    StimulusNewDefinition => "New definition",
    StimulusDuplicate => "Duplicate definition",
    StimulusDelete => "Delete definition",
    StimulusDeleteWithAdopterSingular => "Delete definition · {count} placed source keeps its card",
    StimulusDeleteWithAdopters => "Delete definition · {count} placed sources keep their cards",
    StimulusRevert => "Revert draft",
    StimulusApply => "Apply",
    StimulusDraftNotApplied => "draft · not applied",
    StimulusUndoAvailable => "Undo steps this draft back",
    StimulusDraftErrorSingular => "draft · {count} error",
    StimulusDraftErrors => "draft · {count} errors",
    StimulusNoAdopters => "no adopters",
    StimulusAdopterCountSingular => "{count} adopter",
    StimulusAdopterCount => "{count} adopters",
    StimulusNoFundamental => "This family has no fundamental period",
    StimulusEngineEvaluator => "engine evaluator",
    StimulusHoverUnset => "t = — · v = —",
    StimulusWaveformOf => "Engine-evaluated waveform of {name} over {span}",
    StimulusNoCurve => "There is no curve to draw for these values",
    StimulusBiasColumn => "Bias & small-signal",
    StimulusShapeColumn => "Transient shape · {family}",
    StimulusNoBiasFields => "This family's sheet carries no DC or AC field",
    StimulusDcShapeNote => "A DC profile holds its bias for the whole transient; choose a family in the identity band to give it a waveform",
    StimulusSwitchOn => "on",
    StimulusSwitchOff => "off",
    StimulusPwlPoints => "Points · {count}",
    StimulusPwlAdd => "Add",
    StimulusPwlDelete => "Delete",
    StimulusPwlKeepOnePoint => "A PWL shape keeps at least one point",
    StimulusPwlTable => "Point table",
    StimulusPwlTime => "time",
    StimulusPwlLevel => "level",
    StimulusPwlCurrent => "current",
    StimulusPwlSlope => "slope",
    StimulusPwlTimeOf => "Point {index} time",
    StimulusPwlLevelOf => "Point {index} level",
    StimulusImportFile => "Import file…",
    StimulusNoFileRetained => "no file retained",
    StimulusRetainedFileDetail => "{name} · {bytes} B retained · {digest}",
    StimulusRetainedNote => "The definition retains the file's bytes in the project document. The card still names the file; where that file is not reachable, previews and runs read the retained copy and the run log says so.",
    StimulusCopySpec => "Copy the source specification",
    StimulusReadopt => "Re-adopt r{revision}",
}

#[derive(Debug, Clone, Copy)]
pub struct MessageCatalog {
    locale: UiTextLocale,
}

impl MessageCatalog {
    #[must_use]
    pub const fn new(locale: UiTextLocale) -> Self {
        Self { locale }
    }

    #[must_use]
    pub fn text(self, id: MessageId) -> String {
        self.format(id, &[])
    }

    #[must_use]
    pub fn format(self, id: MessageId, arguments: &[(&str, &str)]) -> String {
        let template = id.english_template();
        let template = match self.locale {
            UiTextLocale::EnglishUnitedStates => template.to_owned(),
            UiTextLocale::PseudoExpanded => pseudolocalize_template(template),
        };
        format_template(&template, arguments).unwrap_or_else(|error| {
            debug_assert!(false, "invalid localized message: {error}");
            id.english_template().to_owned()
        })
    }
}

fn format_template(template: &str, arguments: &[(&str, &str)]) -> Result<String, String> {
    let arguments = arguments.iter().copied().collect::<HashMap<_, _>>();
    let mut output = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        output.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let close = after
            .find('}')
            .ok_or_else(|| "message template has an unmatched opening brace".to_owned())?;
        let name = &after[..close];
        if name.is_empty()
            || name
                .chars()
                .any(|ch| !(ch.is_ascii_alphanumeric() || ch == '_'))
        {
            return Err(format!("message template has invalid placeholder {name:?}"));
        }
        output.push_str(
            arguments
                .get(name)
                .copied()
                .ok_or_else(|| format!("message argument {name:?} is missing"))?,
        );
        rest = &after[close + 1..];
    }
    if rest.contains('}') {
        return Err("message template has an unmatched closing brace".to_owned());
    }
    output.push_str(rest);
    Ok(output)
}

fn pseudolocalize_template(template: &str) -> String {
    let mut output = String::with_capacity(template.len() * 2);
    output.push_str("[!! ");
    let mut placeholder = false;
    for ch in template.chars() {
        match ch {
            '{' => {
                placeholder = true;
                output.push(ch);
            }
            '}' => {
                placeholder = false;
                output.push(ch);
            }
            _ if placeholder => output.push(ch),
            _ => {
                output.push(match ch {
                    'a' => 'à',
                    'A' => 'À',
                    'e' => 'ë',
                    'E' => 'Ë',
                    'i' => 'ï',
                    'I' => 'Ï',
                    'o' => 'ô',
                    'O' => 'Ô',
                    'u' => 'ü',
                    'U' => 'Ü',
                    'y' => 'ÿ',
                    'Y' => 'Ÿ',
                    other => other,
                });
                if matches!(ch.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y') {
                    output.push('~');
                }
            }
        }
    }
    output.push_str(" !!]");
    output
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn every_message_id_has_a_valid_catalog_entry_and_stable_key() {
        for locale in UiTextLocale::ALL {
            assert!(!locale.label().is_empty());
        }
        let mut keys = HashSet::new();
        for id in MessageId::ALL.iter().copied() {
            let template = id.english_template();
            assert!(!template.trim().is_empty());
            assert!(keys.insert(id.stable_key()), "duplicate key for {id:?}");
            let placeholders = template
                .split('{')
                .skip(1)
                .filter_map(|tail| tail.split_once('}').map(|(name, _)| name))
                .collect::<Vec<_>>();
            let values = placeholders
                .iter()
                .map(|name| (*name, "value"))
                .collect::<Vec<_>>();
            let _ = MessageCatalog::new(UiTextLocale::EnglishUnitedStates).format(id, &values);
        }
    }

    #[test]
    fn pseudolocalization_expands_literals_but_preserves_engineering_arguments() {
        let message = MessageCatalog::new(UiTextLocale::PseudoExpanded)
            .format(MessageId::NetlistSyntaxErrors, &[("count", "12345")]);
        assert!(message.starts_with("[!! "));
        assert!(message.ends_with(" !!]"));
        assert!(message.contains("12345"));
        assert!(message.len() > "12345 syntax errors".len() * 3 / 2);
    }

    #[test]
    fn expanded_locale_preserves_every_catalog_argument_as_opaque_project_data() {
        let catalog = MessageCatalog::new(UiTextLocale::PseudoExpanded);
        for id in MessageId::ALL.iter().copied() {
            let template = id.english_template();
            let mut seen = HashSet::new();
            let placeholders = template
                .split('{')
                .skip(1)
                .filter_map(|tail| tail.split_once('}').map(|(name, _)| name))
                .filter(|name| seen.insert(*name))
                .collect::<Vec<_>>();
            let values = placeholders
                .iter()
                .enumerate()
                .map(|(index, name)| {
                    format!(
                        "RSPICE_ARG_{index}_{name}_\u{00b5}/\u{7535}\u{8def}/{{literal}}/\u{1f9ea}"
                    )
                })
                .collect::<Vec<_>>();
            let arguments = placeholders
                .iter()
                .zip(&values)
                .map(|(name, value)| (*name, value.as_str()))
                .collect::<Vec<_>>();
            let message = catalog.format(id, &arguments);

            assert!(message.starts_with("[!! "), "missing prefix for {id:?}");
            assert!(message.ends_with(" !!]"), "missing suffix for {id:?}");
            assert!(!message.contains('\0'), "NUL reached output for {id:?}");
            for (name, value) in placeholders.iter().zip(&values) {
                let expected = template.matches(&format!("{{{name}}}")).count();
                assert_eq!(
                    message.matches(value).count(),
                    expected,
                    "argument {name:?} was altered or duplicated for {id:?}"
                );
                assert!(
                    !message.contains(&format!("{{{name}}}")),
                    "unresolved placeholder {name:?} for {id:?}"
                );
            }
        }
    }

    /// Message families whose every entry must be reached by shipped code.
    ///
    /// An unused entry is not harmless. The catalog is what a translator is
    /// given, so an entry nothing paints is either a string that will be
    /// translated and never shown, or — worse, and what this wave found — an
    /// entry whose wording has drifted from the literal the surface actually
    /// paints, quietly disagreeing with the screen.
    ///
    /// Families join this list as their surface is wired.
    const WIRED_FAMILIES: [&str; 7] = [
        "CodeNavigator",
        "CodeSearch",
        "CodeSource",
        "CodeSymbolKind",
        "CodeTools",
        "NetlistNavigator",
        "Stimulus",
    ];

    #[test]
    fn every_message_in_a_wired_family_is_painted_by_something() {
        let mut referenced = std::collections::HashSet::new();
        let mut pending = vec![std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src")];
        while let Some(directory) = pending.pop() {
            let Ok(entries) = std::fs::read_dir(&directory) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    pending.push(path);
                } else if path.extension().is_some_and(|extension| extension == "rs")
                    && path
                        .file_name()
                        .is_some_and(|name| name != "localization.rs")
                    && let Ok(source) = std::fs::read_to_string(&path)
                {
                    for (index, _) in source.match_indices("MessageId::") {
                        let name = source[index + "MessageId::".len()..]
                            .split(|character: char| {
                                !character.is_ascii_alphanumeric() && character != '_'
                            })
                            .next()
                            .unwrap_or_default();
                        if !name.is_empty() {
                            referenced.insert(name.to_owned());
                        }
                    }
                }
            }
        }
        assert!(
            referenced.len() > 100,
            "the scan found almost nothing, so it is not reading the crate"
        );

        let orphaned = MessageId::ALL
            .iter()
            .map(|id| id.stable_key())
            .filter(|key| {
                WIRED_FAMILIES
                    .iter()
                    .any(|family| key.starts_with(family) && !referenced.contains(*key))
            })
            .collect::<Vec<_>>();
        assert!(
            orphaned.is_empty(),
            "these catalog entries are in a wired family but nothing paints them: {orphaned:?}"
        );
    }

    #[test]
    fn formatter_rejects_missing_and_malformed_placeholders() {
        assert!(format_template("{count} files", &[]).is_err());
        assert!(format_template("{bad-name}", &[("bad-name", "x")]).is_err());
        assert!(format_template("{open", &[("open", "x")]).is_err());
        assert!(format_template("close}", &[]).is_err());
    }

    /// Every module that paints a code-workspace surface, found rather than
    /// listed.
    ///
    /// This guard used to name four files. Splitting those files moved most of
    /// the drawing into submodules the guard had never heard of, so it went on
    /// passing while covering a fraction of what it was written for. A walk
    /// cannot fall behind a split.
    fn code_workspace_surface_sources() -> Vec<(String, String)> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/workbench");
        let roots = [
            "surfaces/netlist.rs",
            "surfaces/veriloga.rs",
            "surfaces/automation.rs",
            "docks/navigator/netlist.rs",
            "docks/navigator/source_bundle.rs",
            // A Verilog-A cell view's Design docks, the inspector body they
            // draw, the form that imports a file as cell views, and the one
            // that creates a cell for a new module.
            "docks/navigator/veriloga.rs",
            "docks/inspector/veriloga.rs",
            "documents/code_workspace/veriloga_inspector.rs",
            "app/dialogs/library/veriloga_import.rs",
            "app/dialogs/library/new_veriloga_device.rs",
            "app/dialogs/veriloga_build_settings.rs",
            "app/dialogs/source_find.rs",
            "app/dialogs/source_document.rs",
            "app/dialogs/source_language_tools.rs",
        ];
        let mut found = Vec::new();
        for relative in roots {
            let path = root.join(relative);
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{relative} is no longer readable: {error}"));
            found.push((relative.to_owned(), source));

            // A module's submodule directory sits beside it under the same
            // stem, and everything in it draws the same surface.
            let Some(directory) = path.parent().map(|parent| {
                parent.join(
                    path.file_stem()
                        .map(std::ffi::OsStr::to_os_string)
                        .unwrap_or_default(),
                )
            }) else {
                continue;
            };
            let Ok(entries) = std::fs::read_dir(&directory) else {
                continue;
            };
            for entry in entries.flatten() {
                let child = entry.path();
                if child.extension().is_some_and(|extension| extension == "rs")
                    && let Ok(source) = std::fs::read_to_string(&child)
                {
                    found.push((child.display().to_string(), source));
                }
            }
        }
        assert!(
            found.len() > roots.len(),
            "the walk found no submodules at all, so it is scanning less than it claims"
        );
        found
    }

    #[test]
    fn code_workspace_surfaces_do_not_reintroduce_literal_widget_copy() {
        let forbidden = [
            "Dialog::new(\"",
            ".description(\"",
            ".ghost(\"",
            "egui::Button::new(\"",
            ".hint_text(\"",
        ];
        for (name, source) in code_workspace_surface_sources() {
            let production = crate::source_guard::production_source(&source);
            for pattern in forbidden {
                assert!(
                    !production.contains(pattern),
                    "{name} bypasses the message catalog with {pattern:?}"
                );
            }
        }

        let navigator = include_str!("docks/navigator.rs");
        let code_workspace = navigator
            .split_once("fn header(ui: &mut Ui, app: &mut RSpiceApp)")
            .expect("code-workspace navigator boundary")
            .0;
        for pattern in forbidden {
            assert!(
                !code_workspace.contains(pattern),
                "code-workspace navigator bypasses the message catalog with {pattern:?}"
            );
        }
        for removed_state_label in [
            "draft.action.title()",
            "draft.action.primary_label()",
            "search.scope.label()",
            "scope.label()",
            "automation.execution.label()",
        ] {
            assert!(
                !code_workspace.contains(removed_state_label),
                "code-workspace navigator bypasses the catalog through {removed_state_label}"
            );
        }
    }
}
