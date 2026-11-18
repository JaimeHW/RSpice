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
    #[cfg(any(test, feature = "test-support"))]
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
            #[cfg(any(test, feature = "test-support"))]
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

/// The one spelling of the action that opens a run's executed task deck.
///
/// Four surfaces printed it as a literal of their own: the command's palette
/// entry, the netlist run strip's message below, the Results manifest's button
/// and the jobs manager's. Four literals is four chances for an operator to
/// meet the same command under two names, so the catalog decides it here,
/// once, and the command's own label reads it from here.
pub const OPEN_TASK_DECK: &str = "Open task deck";

define_messages! {
    EditorGeneratedSpiceNetlistViewer => "Generated SPICE netlist viewer",
    EditorProjectSpiceNetlistEditor => "Project SPICE netlist editor",
    EditorProjectSpiceNetlistViewer => "Project SPICE netlist viewer",
    EditorGeneratedSpiceComparisonViewer => "Generated SPICE comparison viewer",
    EditorRunSnapshotSpiceNetlistViewer => "Run snapshot SPICE netlist viewer",
    EditorVerilogASourceEditor => "Verilog-A/AMS source editor",
    EditorVerilogASourceViewer => "Verilog-A/AMS source viewer",
    EditorTomlManifestEditor => "TOML manifest editor",
    EditorTomlManifestViewer => "TOML manifest viewer",
    EditorAccessiblePositionEditable => "Line {line} of {total_lines}, column {column}. Editable.",
    EditorAccessiblePositionReadOnly => "Line {line} of {total_lines}, column {column}. Read-only.",
    EditorAccessibleOneCharacterSelected => "One character selected.",
    EditorAccessibleBlankLine => "blank",
    AccessibleRoleButton => "button",
    AccessibleRoleLink => "link",
    AccessibleRoleCheckBox => "check box",
    AccessibleRoleSwitch => "switch",
    AccessibleRoleRadioButton => "radio button",
    AccessibleRoleComboBox => "combo box",
    AccessibleRoleEdit => "edit",
    AccessibleRoleSlider => "slider",
    AccessibleRoleSpinButton => "spin button",
    AccessibleRoleListItem => "list item",
    AccessibleRoleTab => "tab",
    AccessibleRoleMenuItem => "menu item",
    AccessibleRoleTreeItem => "tree item",
    AccessibleRoleDialog => "dialog",
    AccessibleStateChecked => "checked",
    AccessibleStateNotChecked => "not checked",
    AccessibleStatePartlyChecked => "partly checked",
    AccessibleStateSelected => "selected",
    AccessibleStateUnavailable => "unavailable",
    EditorAccessibleCharactersSelected => "{count} characters selected.",
    EditorAccessibleSelectionBeyondExcerpt => "The selection extends beyond the active-line excerpt.",
    EditorAccessibleMultipleCursors => "{count} cursors active.",
    EditorAccessibleCrossProbe => "Cross-probed source line.",
    EditorAccessibleModifiedLine => "Modified line.",
    EditorAccessibleNetMarked => "Connects the net shown from the schematic.",
    EditorAccessibleSourceExcerpt => "Source excerpt columns {first_column} through {last_column} of {total_columns}.",
    EditorAccessibleSeverityHint => "hint",
    EditorAccessibleSeverityInformation => "information",
    EditorAccessibleSeverityWarning => "warning",
    EditorAccessibleSeverityError => "error",
    EditorAccessibleTabMovesFocus => "Tab moves focus; {shortcut} makes it indent again.",
    EditorAccessibleTabIndents => "Tab indents; {shortcut} makes it move focus.",
    EditorTabMovesFocusBadge => "Tab moves focus",
    NetlistFindActiveDocument => "Find in active netlist document",
    NetlistSearchUnavailable => "Open or generate a netlist document before searching.",
    NetlistReturnRoot => "Return to root",
    NetlistTopDeck => "Top deck",
    NetlistSaveSourceDeck => "Save",
    NetlistValidateSource => "Validate source",
    NetlistReturnPrimary => "Return to primary",
    NetlistReturnGeneratedPrimary => "Return to generated primary",
    NetlistGenerateBeforeAction => "Generate a primary netlist before using this action.",
    NetlistNothingToSave => "No changes to save.",
    NetlistGenerationBlocked => "generation blocked",
    NetlistGenerationStaleBlocked => "stale · generation blocked",
    NetlistSyntaxErrorSingular => "{count} syntax error",
    NetlistSyntaxErrors => "{count} syntax errors",
    NetlistErrors => "Errors",
    NetlistAdvisories => "Advisories",
    NetlistMoreInProblems => "{count} more in Problems",
    NetlistIncludeProvenance => "Include",
    NetlistDisplayName => "Name",
    NetlistLogicalIdentity => "Include",
    NetlistRequestedAs => "Written as",
    NetlistOrigin => "From",
    NetlistRetainedAuthenticatedSource => "Stored in the project",
    NetlistRelationship => "Relationship",
    NetlistDirectInclude => "Direct include",
    NetlistTransitiveInclude => "Transitive include",
    NetlistParent => "Parent",
    NetlistGeneratedFrom => "Generated from",
    NetlistState => "State",
    NetlistEncoding => "Encoding",
    NetlistNotGeneratedYet => "Not generated yet",
    NetlistCouldNotGenerate => "Could not generate; showing the last deck",
    NetlistOlderThanSchematic => "Older than the schematic",
    NetlistLineColumn => "line {line} · column {column}",
    NetlistDocumentScope => "document scope",
    NetlistUnresolved => "Not found",
    NetlistRootValidationRequired => "An include file cannot be validated on its own. Return to the root deck to validate it with all of its includes.",
    NetlistDependencyCannotRun => "Dependency documents cannot run on their own; return to the root deck, validate it with its includes, and run it.",
    NetlistRunAlreadyActive => "A simulation execution is already active",
    NetlistRunNeedsText => "Enter a netlist before running",
    NetlistRunComparison => "Generated comparison documents cannot be executed",
    NetlistRunUsedDeck => "The deck a completed run used belongs to that run; return to the working deck to run it again.",
    NetlistRunStale => "Regenerate the stale netlist before running",
    NetlistRunUnvalidated => "Validate the current source and project revision before running",
    NetlistRerunCancelled => "Queued netlist re-run cancelled: {reason}",
    NetlistRunDeckUnavailable => "The deck this run used cannot be opened: {reason}.",
    NetlistRunSourceUnavailable => "The source Run {id} executed cannot be opened: {reason}.",
    NetlistRunNotKept => "Run {id} is no longer kept in this session, so Results was not opened on it.",
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
    NetlistRunStripOpenTaskDeck => OPEN_TASK_DECK,
    NetlistRunStripOpenTaskDeckTooltip => "Opens the deck Run {id} gave its first task, as a read-only document saved with the run.",
    NetlistRunStripNextTask => "Task {index}/{count}",
    NetlistRunStripNextTaskTooltip => "Showing task {index} of {count}. Opens the next task's executed deck.",
    NetlistRunStripDeckUnmatched => "Differs from the run's deck",
    NetlistRunStripDeckUnmatchedTooltip => "Nothing retained for Run {id} reproduces the source digest sealed on its receipt. Whatever this deck is, the run was not authorized over it.",
    NetlistRunSnapshotCompare => "Compare",
    NetlistRunSnapshotCompareTooltip => "Compare this run's deck with the working revision",
    NetlistRunSnapshotOpenRun => "Open Run {id}",
    NetlistRunSnapshotDocument => "Run {id} deck",
    NetlistGeneratedInclude => "{name} · generated",
    NetlistComparisonDocument => "Netlist comparison",
    CommonCancel => "Cancel",
    CommonClose => "Close",
    CommonFind => "Find",
    CommonReplace => "Replace",
    CommonScope => "Scope",
    NetlistGeneratedUnavailable => "Generated netlist unavailable",
    NetlistGeneratedUnavailableDescription => "Add a circuit before generating the primary netlist.",
    NetlistEmptyWorkspaceTitle => "Open a SPICE deck",
    NetlistEmptyWorkspaceDescription => "Open a SPICE file as one of the project's decks. The deck stays a file: Save writes it, and a run reads it as written.",
    NetlistOpenDeck => "Open deck…",
    NetlistDeckChangedOnDisk => "{name} changed on disk since you opened it.",
    NetlistDeckReload => "Reload",
    NetlistDeckKeepMine => "Keep mine",
    NetlistDeckCompare => "Compare",
    NetlistDeckMissing => "{name} is not at {path}.",
    NetlistDeckOnTheDesktop => "{name} is a file on the desktop ({path}). Open the project on the desktop to edit or run it.",
    NetlistDeckLocate => "Locate…",
    NetlistDeckRemoveFromProject => "Remove from project",
    NetlistDeckRemoveConfirm => "Remove {name} from the project? The file stays on disk.",
    NetlistDeckRemove => "Remove",
    NetlistIncludeNotSaved => "Not saved: the check and other tabs read the saved file; Run saves it.",
    NetlistComparisonGeneratedTitle => "Compare generated revisions",
    NetlistCompareRevisions => "Compare revisions",
    NetlistComparisonGeneratedDescription => "Select an earlier generated revision and compare its source with the current generated netlist.",
    NetlistComparisonGeneratedPrompt => "Select an earlier generated revision to compare with the current generated netlist.",
    NetlistPriorRevision => "Prior revision",
    NetlistGeneratedRevisionChoice => "Input revision {revision} · {digest}…",
    NetlistExportDeck => "Export\u{2026}",
    NetlistExportTitle => "Export generated netlist",
    NetlistExportDescription => "Export the current generated revision as a deck, or as a bundle with its dependencies and a checksum manifest, without changing project source.",
    NetlistExportDependencyBundle => "Dependencies as a .zip bundle",
    NetlistExportSourceMap => "Source map",
    NetlistControlRegionDisposition => ".control not executed",
    NetlistControlRegionCounts => "{promoted} promoted · {dropped} dropped",
    NetlistControlRegionDetail => "RSpice runs SPICE cards and has no .control interpreter. {promoted} command(s) in this deck were promoted into declarative directives; {dropped} were read and ignored.",
    NetlistDialect => "Dialect",
    NetlistDialectRspice => "RSpice",
    NetlistDialectNgspice => "ngspice",
    NetlistDialectLtspice => "LTspice",
    NetlistDialectQspice => "QSPICE",
    NetlistDialectSpectre => "Spectre",
    NetlistDialectXyceDevices => "Xyce devices",
    NetlistDialectDetail => "How the engine reads this deck. Another choice writes its .options line into the deck.",
    NetlistDialectXyceDetail => "Xyce expression syntax is read only by rspice run --spice-dialect xyce",
    NetlistDialectSpectreDetail => "The file's .scs name or a simulator lang= line makes this a Spectre deck.",
    NetlistDialectHint => "Written by {program}? {marker} on line {line}.",
    NetlistDialectHintRead => "Read as {program}",
    NetlistDialectHintDismiss => "Dismiss",
    NetlistFindEnterTextHint => "Enter text to search.",
    NetlistFindTitle => "Find and replace in source",
    NetlistFindNext => "Find next",
    NetlistFindDescription => "Search the deck and the generated netlist; Replace changes only the open deck and its includes.",
    NetlistFindExpressionHint => "Symbol, text, or expression",
    NetlistFindMatchCase => "Match case",
    NetlistFindWholeSymbol => "Whole symbol",
    NetlistFindRegularExpression => "Regular expression",
    NetlistFindIncludeComments => "Include comments",
    NetlistFindIncludeGeneratedReferences => "Include generated references",
    NetlistFindSelectionRequired => "Select source text before using the Selection scope.",
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
    NetlistFindReplacedSingular => "Replaced {count} match in the deck.",
    NetlistFindReplaced => "Replaced {count} matches in the deck.",
    NetlistNavigatorStructure => "Structure",
    NetlistNavigatorIncludes => "Includes",
    NetlistNavigatorDecks => "Decks",
    NetlistNavigatorDeckStored => "in project",
    NetlistNavigatorDeckMissing => "not found",
    NetlistNavigatorProjectSymbols => "Symbols",
    NetlistNavigatorSymbolsInFiles => "{count} in {files} files",
    NetlistNavigatorProjectReferences => "References",
    NetlistNavigatorReferenceSingular => "{count} reference",
    NetlistNavigatorReferences => "{count} references",
    NetlistNavigatorHierarchyInstanceSingular => "{count} instance",
    NetlistNavigatorHierarchyInstances => "{count} instances",
    NetlistNavigatorHierarchyBounded => "bounded view",
    NetlistNavigatorHierarchyRecursive => "recursive edge",
    NetlistLanguageTools => "SPICE language tools",
    NetlistSignatureHelp => "Signature help",
    NetlistGoToDeclaration => "Go to declaration",
    NetlistGoToDefinition => "Go to definition",
    NetlistWorkspaceSymbols => "Workspace symbols…",
    NetlistCodeActions => "Code actions",
    NetlistApplyQuickFix => "Apply preferred quick fix",
    NetlistFormatDocument => "Format document",
    NetlistNavigatorSourceMapping => "Source mapping",
    NetlistNavigatorProvenance => "provenance",
    NetlistNavigatorNoMatches => "No symbol, directive, or source line matches this filter.",
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
    NetlistIncludeChainUntraced => "Not read yet: the check walks the search chain for this line when it reads the deck up to it.",
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
    NetlistNavigatorModels => "Models",
    NetlistNavigatorAnalyses => "Analyses",
    NetlistNavigatorMeasurements => "Measurements",
    NetlistNavigatorHierarchy => "Hierarchy",
    NetlistNavigatorSubcircuits => "Subcircuits",
    NetlistNavigatorGlobals => "Global nodes",
    NetlistNavigatorFunctions => "Functions",
    NetlistNavigatorOptions => "Solver options",
    NetlistNavigatorOutputs => "Save and probe",
    NetlistNavigatorControls => "Other cards",
    NetlistNavigatorLine => "line {line}",
    NetlistNavigatorNoProvenance => "This line has no schematic source.",
    NetlistNavigatorMappedLine => "Line {line} · {cell}",
    NetlistNavigatorInstance => "Instance {instance}",
    NetlistShowInstanceLocated => "Located instance {instance} at generated line {line}.",
    NetlistShowInstanceStale => "Located instance {instance} at generated line {line} in a stale deck; regenerating it may move the card.",
    NetlistShowNetLocated => "Located net {net} at generated line {line}; every line that connects it is marked.",
    NetlistShowNetStale => "Located net {net} at generated line {line} in a stale deck; regenerating it may move the cards.",
    CodeNavigatorFiles => "Files",
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
    CodeSourceLifecycleTitle => "Manage source document",
    CodeSourceLifecycleDescription => "Review ownership, persistence, includes, and revision history for {document}, and run the file operations this project source graph allows.",
    CodeSourceOperationsHeld => "File operations are held",
    CodeSourceReadOnlyDocument => "Read-only document",
    CodeSourceReadOnlyDocumentDetail => "This document is not editable in the active project, so operations that rewrite it are unavailable.",
    CodeSourceCurrentDocument => "Current document",
    CodeSourceOwnedReadOnly => "project-owned · read-only",
    CodeSourcePath => "Path",
    CodeSourceLineEndings => "Line endings",
    CodeSourceLineEndingsNone => "none · single line",
    CodeSourceLineEndingsLf => "LF",
    CodeSourceLineEndingsCrLf => "CRLF",
    CodeSourceLineEndingsMixed => "mixed LF and CRLF",
    CodeSourceValidation => "Validation",
    CodeSourceValidationStale => "Out of date",
    CodeSourceRecordedRole => "Role",
    CodeSourceRoleOrdinary => "Ordinary source",
    CodeSourceRoleBuildProfile => "Verilog-A build profile",
    CodeSourceFileOperations => "File operations",
    CodeSourceNewFile => "New file…",
    CodeSourceAddFiles => "Add files…",
    CodeSourceRename => "Rename…",
    CodeSourceMove => "Move…",
    CodeSourceDuplicate => "Duplicate…",
    CodeSourceDelete => "Delete…",
    CodeSourceRevisionHistory => "Revision history…",
    CodeSourceReplaceFromFile => "Replace from file…",
    CodeSourceRootDeleteBlocked => "The main file can't be deleted. Rename it or replace its source instead.",
    CodeInspectorNetlistTitle => "Netlist problems",
    CodeSourceCreatePrimary => "Create",
    CodeSourceRenamePrimary => "Rename",
    CodeSourceMovePrimary => "Move",
    CodeSourceDuplicatePrimary => "Duplicate",
    CodeSourceDeletePrimary => "Delete",
    CodeSourceRecordedIncludes => "Includes",
    CodeSourceNoIncludes => "This document includes no file, and no file includes it.",
    CodeSourceImports => "Includes",
    CodeSourceImportedBy => "Included by",
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
    CodeSourceFileNewTitle => "New {language} file",
    CodeSourceFileRenameTitle => "Rename file",
    CodeSourceFileMoveTitle => "Move file",
    CodeSourceFileDuplicateTitle => "Duplicate file",
    CodeSourceFileDeleteTitle => "Delete file",
    CodeSourceFileName => "Name",
    CodeSourceFileNewName => "New name",
    CodeSourceFileNewPath => "New path",
    CodeSourceFileFile => "File",
    CodeSourceFileIncludedFrom => "Included from",
    CodeSourceFileAddsInclude => "Adds an `include at the top of {file}",
    CodeSourceFileUpdatesInclude => "Updates the `include in {file}",
    CodeSourceFileUpdatesIncludes => "Updates the `include in {count} files",
    CodeSourceFileRemovesInclude => "Removes its `include from {file}",
    CodeSourceFileRemovesIncludes => "Removes its `include from {count} files",
    CodeSourceFileHistoryKeepsCopy => "Revision history keeps a copy.",
    CodeSourceFileRoleBound => "{file} is the {role}; it can't be deleted.",
    CodeSourceFileIncludesOthers => "{file} includes {dependency}. Delete {dependency} first.",
    CodeSourceFileEnterName => "Enter a file name.",
    CodeSourceFileStale => "The files changed while this was open. Cancel and try again.",
    CodeSourceFileNameCharacters => "A name can't contain \" ' : * ? < > |",
    CodeSourceFileNameReserved => "Windows reserves this name; choose another.",
    CodeSourceFileNameTooLong => "The path is longer than {limit} bytes.",
    CodeSourceFileMainExtension => "The main file must end in {accepted}.",
    CodeSourceFileNameBackslash => "Use / to separate folders.",
    CodeSourceFileNameAbsolute => "A path can't start with / or a drive.",
    CodeSourceFileNameEmptyPart => "Each folder and file needs a name; '.' and '..' aren't allowed.",
    CodeSourceFileNameEdgeSpace => "A name can't begin or end with a space.",
    CodeSourceFileNameTrailingPeriod => "A name can't end with a period.",
    CodeSourceFileExists => "A file named {name} already exists.",
    CodeSearchTitle => "Find in source",
    CodeSearchDescription => "Search the project-owned source closure. Finding changes nothing; replacement is one atomic bundle revision that skips every ineligible source.",
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
    CodeSearchMatchSingular => "{count} match",
    CodeSearchMatches => "{count} matches",
    CodeSearchMatchesReadOnly => "{count} matches; {read_only} in read-only sources are not replaced",
    CodeSearchNoMatches => "No match in the selected scope.",
    CodeSearchResultAccessible => "{path}, line {line}, column {column}. {preview}",
    CodeSearchResultAccessibleReadOnly => "{path}, line {line}, column {column}, in a read-only source. {preview}",
    CodeSearchShowingFirst => "Showing the first {limit} of {count} matches. Narrow the query to see the rest.",
    CodeSearchShowingFirstStreamStopped => "Showing the first {limit} of {count} matches, and the search stopped at its streaming limit. Narrow the query to see the rest.",
    CodeGeneratedNetlist => "Generated netlist",
    VerilogANoSource => "No Verilog-A/AMS source",
    VerilogAImportTitle => "Import Verilog/Verilog-A/AMS",
    VerilogAImportPrimary => "Create cells",
    VerilogAImportDescription => "The file and every file it includes become one source. Each chosen top module becomes a cell whose view selects it: Verilog for a .v file, Verilog-A for .va, Verilog-AMS for .vams.",
    VerilogAImportFile => "File",
    VerilogAImportLibrary => "Library",
    VerilogAImportCreates => "Creates",
    VerilogAImportNoLibrary => "No library can be written; create or unlock one first.",
    VerilogAImportRefused => "{file_name} cannot be read: {reason}",
    VerilogAImportIncludes => "Includes",
    VerilogAImportIncludeSource => "Source",
    VerilogAImportIncludeFound => "Found",
    VerilogAImportIncludeBuiltIn => "Built in",
    VerilogAImportMissing => "Missing",
    VerilogAImportMissingFile => "{file}, included by {including}",
    VerilogAImportLocate => "Locate…",
    VerilogAImportMissingOne => "Locate {file}, which {including} includes, before importing.",
    VerilogAImportMissingMany => "{count} included files are missing; locate each before importing.",
    VerilogAImportModules => "Modules",
    VerilogAImportNoModuleChosen => "Choose at least one module to import.",
    VerilogADeviceTitle => "New Verilog-A device",
    VerilogADevicePrimary => "Create",
    VerilogADeviceDescription => "Create a cell whose Verilog-A view holds a starter module, with a symbol generated from the module's ports.",
    VerilogADeviceLibrary => "Library",
    VerilogADeviceCell => "Cell name",
    VerilogADeviceCellMissing => "Enter a name for the cell.",
    VerilogADeviceInputs => "Inputs",
    VerilogADeviceOutputs => "Outputs",
    VerilogADeviceInouts => "Inouts",
    VerilogADeviceInputsHint => "inp inn",
    VerilogADeviceOutputsHint => "out",
    VerilogADeviceInoutsHint => "vdd vss",
    VerilogADeviceModule => "module {module}",
    VerilogADevicePortCount => "{count} ports · symbol generated from them",
    VerilogADeviceOnePort => "1 port · symbol generated from it",
    VerilogADeviceNoPorts => "The module starts with no ports.",
    VerilogADeviceNoLibrary => "No library can be written; create or unlock one first.",
    VerilogADeviceCellInvalid => "Letters, digits and underscores only.",
    VerilogADeviceCellExists => "{library} already has a cell named {cell}.",
    VerilogADevicePortDirected => "{entry}: the row already sets the direction.",
    VerilogADevicePortName => "{entry}: not a port name or a bus like d[7:0].",
    VerilogADevicePortIsInput => "{port} is already an input.",
    VerilogADevicePortIsOutput => "{port} is already an output.",
    VerilogADevicePortIsInout => "{port} is already an inout.",
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
    VerilogABuildSettingsDescription => "How this Verilog-A view compiles: its entry module, include paths, preprocessor symbols and checks.",
    VerilogABuildSettingsAdvanced => "Advanced",
    VerilogABuildSettingsTargets => "Also build",
    VerilogABuildSettingsGeneratedRust => "Packaged model",
    VerilogABuildSettingsNativeJit => "Native code",
    VerilogABuildSettingsFallback => "If a build fails",
    VerilogABuildSettingsUseInterpreter => "Run interpreted",
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
    VerilogAStatusCompiling => "Compiling… {elapsed}",
    VerilogAStatusCompiled => "Compiled",
    VerilogAStatusTimed => "{status} · {duration}",
    VerilogAStatusResultDiscarded => "Result discarded: the source changed",
    VerilogACompileDiscardedAnnouncement => "The source changed during the compile, so its result was discarded.",
    VerilogACompileCancelledAnnouncement => "Compile cancelled",
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
    EditorAccessibleLineSeverity => "{severity} diagnostic on this line.",
    EditorAccessibleLineDiagnostic => "{severity} on this line: {message}",
    VerilogACompletionRow => "{kind} completion: {label}, {detail}",
    VerilogACompletionHint => "Tab to accept",
    VerilogACompletionAccessible => "Completion list of {count}: {label}, {kind}. {description}",
    VerilogACallAccessible => "In a call of {signature}.",
    VerilogACallArgumentAccessible => "Argument {position} of {signature}: {argument}.",
    VerilogAAssistNothingAtCaret => "Nothing to explain at the caret.",
    SourceDiagnosticSeverityError => "Error",
    SourceDiagnosticSeverityWarning => "Warning",
    SourceDiagnosticSeverityInformation => "Information",
    SourceDiagnosticSeverityHint => "Hint",
    SourceDiagnosticStep => "{severity} {index} of {count}, line {line}: {message}",
    SourceDiagnosticAtColumn => "{severity} at line {line}, column {column}: {message}",
    SourceDiagnosticAtLine => "{severity} at line {line}: {message}",
    SourceDiagnosticUnplaced => "{severity}: {message}",
    SourceDiagnosticNone => "No problems in this document.",
    SourceDiagnosticNoneAtCaret => "No problem at the caret.",
    SourceDiagnosticAtCaret => "{severity} {code} at line {line}, column {column}: {message}",
    SourceDiagnosticAtCaretOutdated => "{severity} {code} at line {line}, column {column}, found before your latest edits: {message}",
    SourceDiagnosticHoverPlace => "{severity} {code}, line {line}, column {column}",
    SourceDiagnosticOutdated => "Found before your latest edits.",
    VerilogACompiledAnnouncement => "Compiled {module}: {ports}, {parameters}",
    VerilogACompiledWithAdvisoriesAnnouncement => "Compiled {module}: {ports}, {parameters}, {advisories}",
    VerilogACompileFailedAnnouncement => "Compile failed: {errors}",
    VerilogAPortCountSingular => "{count} port",
    VerilogAPortCount => "{count} ports",
    VerilogAParameterCountSingular => "{count} parameter",
    VerilogAParameterCount => "{count} parameters",
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
    VerilogASymbolStrip => "Symbol out of date: {details}",
    VerilogASymbolStripMissing => "{names} missing",
    VerilogASymbolStripTurned => "{names} changed direction",
    VerilogASymbolStripExtra => "{names} no longer in the module",
    VerilogAUpdateSymbol => "Update symbol",
    VerilogAInterfaceMismatchStrip => "Module ports differ from the cell's schematic, which defines them",
    VerilogAInterfaceRefusedStrip => "Ports not written: {reason}",
    VerilogAPlaceReady => "{cell} is ready to place · {count} ports",
    VerilogAPlaceReadyOne => "{cell} is ready to place · 1 port",
    VerilogAPlaceIn => "Place in {schematic}",
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
    NetlistSymbolParameter => "Parameter {name}",
    NetlistSymbolSubcircuit => "Subcircuit {name}",
    NetlistSymbolModel => "Model {name}",
    NetlistSymbolFunction => "Function {name}",
    NetlistSymbolPorts => "Ports {ports}",
    NetlistSymbolPlace => "{file} line {line}",
    NetlistNothingInScopeDefines => "Nothing in scope defines {name}.",
    NetlistNoReferences => "no references",
    NetlistDefinedIn => "{kind} defined in {file}:{line}",
    NetlistPlaceCaretOnName => "Place the caret on a model, subcircuit, parameter, or function name.",
    NetlistNoDefinition => "No definition for '{name}' exists in the project's sources.",
    NetlistOpenedDefinition => "Opened definition of {name}",
    NetlistFoundDefinitions => "Found {count} definitions of {name}; the navigator lists where each one is.",
    NetlistShowingSymbols => "Showing {count} project symbol(s).",
    NetlistSignatureParameters => "Parameters {parameters}",
    NetlistSignatureNoIndex => "The active source has no project index.",
    NetlistSignaturePlaceCaret => "Place the caret on a directive, an instance, or a subcircuit, model, parameter or function name.",
    NetlistSignatureShowing => "Showing signature help for {name}",
    NetlistSignatureHelpHint => "Signature help · Esc to close",
    EditorGoToLineTitle => "Go to line",
    EditorGoToLineGo => "Go",
    EditorGoToLineField => "Line",
    EditorGoToLineRange => "1 to {count}",
    EditorGoToLineRefusal => "Enter a line from 1 to {count}.",
    CountOneReference => "1 reference",
    CountReferences => "{count} references",
    CountOneFile => "1 file",
    CountFiles => "{count} files",
    CountOneOccurrence => "1 occurrence",
    CountOccurrences => "{count} occurrences",
    CountOnePlace => "1 place",
    CountPlaces => "{count} places",
    SourceExtent => "{occurrences} in {files}",
    SourceRenameTitle => "Rename symbol",
    SourceRenamePrimary => "Rename",
    SourceRenameSymbol => "Symbol",
    SourceRenameNewName => "New name",
    SourceRenameEnterName => "Enter a new name.",
    SourceRenameExtent => "Renames {extent}",
    SourceRenamed => "Renamed {old} to {new}: {extent}.",
    SourceRenameReadOnly => "{file} is read-only, so nothing in it is renamed.",
    SourceRenameSourceChanged => "The source changed after Rename opened. Close it and rename again.",
    SourceRenameDeckChanged => "The deck changed after Rename opened. Close it and rename again.",
    SourceRenameColumnFile => "File",
    SourceRenameColumnLine => "Line",
    SourceRenameColumnBefore => "Before",
    SourceRenameColumnAfter => "After",
    SourceNoBundle => "This project owns no {language} source yet.",
    SourcePlaceCaretToRename => "Place the caret on a name to rename it.",
    SourceIndexUnavailable => "The source index could not be built.",
    SourcePlaceCaretToFindReferences => "Place the caret on a name to find its references.",
    SourcePlaceCaretToGoToDefinition => "Place the caret on a name to go to its definition.",
    SourceNoDefinition => "No definition of {name} exists in this source bundle.",
    SourceNoReferences => "No references to {name} in this source bundle.",
    SourceReferencesFound => "{references} to {name} in {files}.",
    NetlistKindNode => "node",
    NetlistKindName => "name",
    NetlistKindParameter => "parameter",
    NetlistKindFunction => "function",
    NetlistKindSubcircuit => "subcircuit",
    NetlistKindModel => "model",
    NetlistKindInstance => "instance",
    NetlistKindNodeLabel => "Node",
    NetlistKindNameLabel => "Name",
    NetlistKindParameterLabel => "Parameter",
    NetlistKindFunctionLabel => "Function",
    NetlistKindSubcircuitLabel => "Subcircuit",
    NetlistKindModelLabel => "Model",
    NetlistKindInstanceLabel => "Instance",
    NetlistFollowPlaceCaret => "Place the caret on a parameter, model, subcircuit, function or node.",
    NetlistFollowNoNames => "This document has no names to follow.",
    NetlistFollowStillReading => "The deck is still being read; try again in a moment.",
    NetlistFollowUndefined => "Nothing in scope defines {name}, so it has no places to follow.",
    NetlistReferencesFound => "{kind} {name} is written in {places} in {files}.",
    NetlistRenameGeneratedDeck => "This netlist is written from the schematic; rename the name there.",
    NetlistRenameGround => "Ground keeps its name.",
    NetlistRenameUnreadableNode => "A deck cannot read {name} as one name: a name holds no spaces or {characters}.",
    NetlistRenameUnreadableName => "A deck cannot read {name} as one name: a name holds no spaces or {characters}, and does not start with a digit.",
    NetlistRenameCollision => "{name} is already a {kind} where {old} is read: {place} defines it.",
    NetlistRenameIncludeNotOpen => "{file} is not open beside the deck, so {old} cannot be renamed in it.",
    NetlistRenameUnbound => "{place} names {old} {inside}, which Rename cannot follow. Change it there by hand first.",
    NetlistInsideControl => "in a .control block",
    NetlistInsideString => "inside a string",
    NetlistInsideExpression => "inside an expression",
    NetlistRenameStep => "rename {old} to {new}",
    NetlistRenamed => "Renamed {old} to {new} in {places} in {files}.",
    NetlistDirectiveTran => "Transient analysis.",
    NetlistDirectiveAc => "Small-signal frequency sweep.",
    NetlistDirectiveDc => "DC transfer sweep.",
    NetlistDirectiveOp => "DC operating point.",
    NetlistDirectiveNoise => "Output-referred noise analysis.",
    NetlistDirectiveMeas => "Post-run measurement (drives the specs matrix).",
    NetlistDirectiveParam => "Deck parameter.",
    NetlistDirectiveModel => "Device model card.",
    NetlistDirectiveSubckt => "Subcircuit definition, closed by .ends.",
    NetlistDirectiveEnds => "End of the enclosing .subckt.",
    NetlistDirectiveInclude => "Splice another deck file.",
    NetlistDirectiveLib => "Include a library section.",
    NetlistDirectiveOptions => "Simulator options (reltol, abstol, temp…).",
    NetlistDirectiveIc => "Transient initial conditions.",
    NetlistDirectiveInitcond => "Xyce-compatible device initial-condition overrides.",
    NetlistDirectiveNodeset => "DC convergence hints.",
    NetlistDirectiveSave => "Restrict saved outputs.",
    NetlistDirectiveProbe => "Mark signals for output.",
    NetlistDirectivePrint => "Tabular output selection.",
    NetlistDirectiveGlobal => "Declare global nodes.",
    NetlistDirectiveFunc => "User function for expressions.",
    NetlistDirectiveEndl => "End of a library section.",
    NetlistDirectiveTf => "DC transfer function and input/output resistance.",
    NetlistDirectiveSens => "Sensitivity of an output.",
    NetlistDirectivePz => "Pole-zero analysis.",
    NetlistDirectiveStep => "Repeat the analyses over a parameter.",
    NetlistDirectivePlot => "Plot output selection.",
    NetlistDirectiveIf => "Read the cards up to .else only if true.",
    NetlistDirectiveElseif => "Alternative branch of an .if.",
    NetlistDirectiveElse => "The branch read when no condition held.",
    NetlistDirectiveEndif => "End of an .if block.",
    NetlistDirectiveControl => "Interactive command block, closed by .endc.",
    NetlistDirectiveEndc => "End of a .control block.",
    NetlistDirectiveTemp => "Simulation temperature.",
    NetlistDirectiveFour => "Fourier analysis of a transient.",
    NetlistDirectiveEnd => "End of the deck.",
    NetlistTerminalPositive => "n+",
    NetlistTerminalNegative => "n-",
    NetlistTerminalAnode => "anode",
    NetlistTerminalCathode => "cathode",
    NetlistTerminalCollector => "collector",
    NetlistTerminalBase => "base",
    NetlistTerminalEmitter => "emitter",
    NetlistTerminalSubstrate => "substrate",
    NetlistTerminalDrain => "drain",
    NetlistTerminalGate => "gate",
    NetlistTerminalSource => "source",
    NetlistTerminalBulk => "bulk",
    NetlistTerminalControlPositive => "control+",
    NetlistTerminalControlNegative => "control-",
    NetlistTerminalPortOnePositive => "port 1+",
    NetlistTerminalPortOneNegative => "port 1-",
    NetlistTerminalPortTwoPositive => "port 2+",
    NetlistTerminalPortTwoNegative => "port 2-",
    NetlistCompletionPin => "pin",
    NetlistCompletionNode => "node",
    NetlistCompletionDirective => "cmd",
    NetlistCompletionParameter => "param",
    NetlistCompletionPinNote => "{order}. Pin {position} of {count} is {pin}.",
    NetlistCompletionPinDetail => "{subcircuit} pin {position} of {count}",
    NetlistCompletionAsPin => "as {pin}",
    NetlistCompletionParameterNote => "Deck parameter `{name}` (.param).",
    NetlistCompletionAccessible => "{kind} completion: {label}, {detail}",
    NetlistCompletionAccept => "{note}\u{a0}\u{a0}·\u{a0}\u{a0}⇥\u{a0}to\u{a0}accept",
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

    /// A counted word in its two forms, `one` for a count of one, else
    /// `many` with the count as `{count}`: "1 file", "3 files".
    #[must_use]
    pub fn counted(self, count: usize, one: MessageId, many: MessageId) -> String {
        if count == 1 {
            self.text(one)
        } else {
            self.format(many, &[("count", &count.to_string())])
        }
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

    #[test]
    fn formatter_rejects_missing_and_malformed_placeholders() {
        assert!(format_template("{count} files", &[]).is_err());
        assert!(format_template("{bad-name}", &[("bad-name", "x")]).is_err());
        assert!(format_template("{open", &[("open", "x")]).is_err());
        assert!(format_template("close}", &[]).is_err());
    }
}
