//! The command registry: every command the workbench offers, under the key a
//! shortcut profile stores it by, with the facts the palette and the shortcut
//! profile read about it.
//!
//! Each command's owner declares its spec; the registry holds them in one
//! order and answers lookups by key or by a stored id. The order is part of
//! the product: when two commands answer one chord in contexts of equal
//! precedence, the shortcut resolver runs the one earlier in it. A stored id
//! a retired command used to have is an alias, answered before any key, so a
//! profile an older build wrote keeps its bindings.

use std::collections::{HashMap, HashSet};

use crate::state::command_catalog::{CommandKey, ShortcutBinding, ShortcutContext};

/// The part of the product that declares a command and carries it out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandOwner {
    /// The application frame: workspaces, help, leaving the application.
    Shell,
    /// The workbench every feature shares: panels, windows, documents, and
    /// the edit, find and zoom commands each feature answers for its own
    /// document.
    Framework,
    Project,
    Design,
    Sheets,
    Simulate,
    Results,
    Models,
    Code,
    Hardcopy,
    Cloud,
    Settings,
}

/// Whether a command can run here: available, offered but refused for the
/// reason given, or not offered at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandAvailability {
    Available,
    Disabled(&'static str),
    Hidden,
}

impl CommandAvailability {
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}

/// The yes-or-no facts a command declares about itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandFlags(u8);

impl CommandFlags {
    pub const NONE: Self = Self(0);
    /// Listed in the command palette.
    pub const PALETTE_VISIBLE: Self = Self(1);
    /// A browser keeps the command's primary chord for itself, so a browser
    /// profile needs an alternate for it.
    pub const PRIMARY_RESERVED_BY_BROWSER: Self = Self(1 << 1);
    /// Runs only while a project is open.
    pub const REQUIRES_OPEN_PROJECT: Self = Self(1 << 2);
    /// Held back while a project operation is still in progress.
    pub const BLOCKED_BY_PROJECT_OPERATION: Self = Self(1 << 3);
    /// Edits the active schematic, so a read-only one refuses it.
    pub const EDITS_ACTIVE_DOCUMENT: Self = Self(1 << 4);
    /// Commits the inspector's pending edit first, and does not run when that
    /// edit cannot be applied.
    pub const COMMITS_INSPECTOR_EDIT: Self = Self(1 << 5);

    /// These flags, with `flag` set when `set` holds.
    #[must_use]
    pub const fn with(self, flag: Self, set: bool) -> Self {
        if set { Self(self.0 | flag.0) } else { self }
    }

    #[must_use]
    pub const fn contains(self, flag: Self) -> bool {
        self.0 & flag.0 == flag.0
    }
}

/// What one command declares about itself.
#[derive(Debug, Clone, Copy)]
pub struct CommandSpec {
    pub key: CommandKey,
    /// What menus, the palette and the shortcut editor call the command.
    pub label: &'static str,
    /// The heading the command is listed under.
    pub group: &'static str,
    pub flags: CommandFlags,
    /// The context the command's keys act in.
    pub context: ShortcutContext,
    /// The bindings the command ships with, in declaration order.
    pub bindings: &'static [ShortcutBinding],
    pub owner: CommandOwner,
}

/// Every command, in order, with the ids it answers to.
pub struct Registry {
    order: &'static [CommandKey],
    specs: Vec<CommandSpec>,
    /// The first spec declaring each key.
    by_key: HashMap<&'static str, usize>,
    /// Retired ids, each with the key it names now.
    aliases: &'static [(&'static str, CommandKey)],
    /// Commands an owner declares that the registry leaves out: a container
    /// holds each by key, and no menu, shortcut profile, palette or Repeat
    /// reaches it.
    unlisted: Vec<CommandSpec>,
}

impl Registry {
    /// A registry of `specs`, listed in `order`, that also answers `aliases`.
    /// It is taken as given; [`Self::validate`] says whether it holds
    /// together.
    pub fn new(
        order: &'static [CommandKey],
        specs: Vec<CommandSpec>,
        aliases: &'static [(&'static str, CommandKey)],
    ) -> Self {
        let mut by_key = HashMap::with_capacity(specs.len());
        for (index, spec) in specs.iter().enumerate() {
            by_key.entry(spec.key.as_str()).or_insert(index);
        }
        Self {
            order,
            specs,
            by_key,
            aliases,
            unlisted: Vec::new(),
        }
    }

    /// This registry, knowing the commands `unlisted` declares that it leaves
    /// out, so [`Self::validate`] checks them too.
    #[must_use]
    pub fn with_unlisted(mut self, unlisted: Vec<CommandSpec>) -> Self {
        self.unlisted = unlisted;
        self
    }

    /// Every command, in the order menus list them and the shortcut resolver
    /// breaks ties by.
    #[must_use]
    pub fn order(&self) -> &'static [CommandKey] {
        self.order
    }

    /// The command a stored id names: an alias first, then a key. `None` for
    /// an id no command answers to.
    #[must_use]
    pub fn resolve(&self, id: &str) -> Option<CommandKey> {
        if let Some(&(_, key)) = self.aliases.iter().find(|(alias, _)| *alias == id) {
            return Some(key);
        }
        self.by_key.get(id).map(|&index| self.specs[index].key)
    }

    /// What the command under `key` declares; `None` for a key no command
    /// declares.
    #[must_use]
    pub fn spec(&self, key: CommandKey) -> Option<&CommandSpec> {
        self.by_key
            .get(key.as_str())
            .map(|&index| &self.specs[index])
    }

    /// What every ordered command declares, in order.
    pub fn ordered(&self) -> impl Iterator<Item = &CommandSpec> {
        self.order.iter().filter_map(|key| self.spec(*key))
    }

    /// What the command under `key` is called; the key itself for a key no
    /// command declares.
    #[must_use]
    pub fn label(&self, key: CommandKey) -> &'static str {
        self.spec(key).map_or(key.as_str(), |spec| spec.label)
    }

    /// Whether the registry holds together: every key is declared by exactly
    /// one owner, the order lists each declared key once and nothing else,
    /// every alias names a declared key without being one, every key in
    /// `named` names a declared command, and every command an owner leaves
    /// out is declared once, by no listed command's key, and not ordered.
    /// `named` pairs each key a menu or framework code names a command by
    /// with what names it. The error lists every problem, one per line.
    pub fn validate(
        &self,
        named: impl IntoIterator<Item = (&'static str, CommandKey)>,
    ) -> Result<(), String> {
        let mut problems = Vec::new();
        let mut owners: HashMap<&'static str, CommandOwner> = HashMap::new();
        for spec in &self.specs {
            if let Some(first) = owners.insert(spec.key.as_str(), spec.owner) {
                problems.push(format!(
                    "`{}` is declared by {first:?} and by {:?}",
                    spec.key.as_str(),
                    spec.owner
                ));
            }
        }
        let mut ordered = HashSet::new();
        for key in self.order {
            if !ordered.insert(key.as_str()) {
                problems.push(format!("`{}` is ordered twice", key.as_str()));
            } else if !owners.contains_key(key.as_str()) {
                problems.push(format!(
                    "`{}` is ordered but no owner declares it",
                    key.as_str()
                ));
            }
        }
        for spec in &self.specs {
            if !ordered.contains(spec.key.as_str()) {
                problems.push(format!(
                    "`{}` is declared by {:?} but not ordered",
                    spec.key.as_str(),
                    spec.owner
                ));
            }
        }
        for (alias, target) in self.aliases {
            if owners.contains_key(alias) {
                problems.push(format!("alias `{alias}` is also a key"));
            }
            if !owners.contains_key(target.as_str()) {
                problems.push(format!(
                    "alias `{alias}` names `{}`, which no owner declares",
                    target.as_str()
                ));
            }
        }
        for (namer, key) in named {
            if !owners.contains_key(key.as_str()) {
                problems.push(format!(
                    "{namer} names `{}`, which no owner declares",
                    key.as_str()
                ));
            }
        }
        let mut left_out: HashMap<&'static str, CommandOwner> = HashMap::new();
        for spec in &self.unlisted {
            let key = spec.key.as_str();
            if let Some(first) = owners.get(key).or_else(|| left_out.get(key)) {
                problems.push(format!(
                    "`{key}` is declared by {first:?} and left out by {:?}",
                    spec.owner
                ));
            }
            if ordered.contains(key) {
                problems.push(format!("`{key}` is left out but ordered"));
            }
            left_out.insert(key, spec.owner);
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(problems.join("\n"))
        }
    }
}

/// The commands framework code raises or asks about by name rather than
/// reading them from a table an owner hands it. Each is the key of a
/// registered command, which the registry's check asserts.
pub mod ids {
    use crate::state::command_catalog::CommandKey;

    macro_rules! ids {
        ($($name:ident = $id:literal;)*) => {
            $(pub const $name: CommandKey = CommandKey::new($id);)*

            /// Every key above.
            const ALL: &[CommandKey] = &[$($name),*];
        };
    }

    /// Every key above, as [`super::Registry::validate`] is handed the keys
    /// something names.
    pub fn named() -> impl Iterator<Item = (&'static str, CommandKey)> {
        ALL.iter().map(|&key| ("framework code", key))
    }

    ids! {
        ACCOUNT_ORGANIZATION = "account-organization";
        CALCULATOR = "calculator";
        CANCEL = "cancel-active-command";
        CLEAR_CONSOLE = "console-clear";
        COMMAND_PALETTE = "command-palette";
        COPY = "copy-selection";
        DESIGN_WORKSPACE = "design";
        EDIT_SPECIFICATIONS = "specifications";
        EXPORT_RESULTS = "export-results";
        EXPRESSION_DIAGNOSTICS = "expression-diagnostics";
        FAMILY_SLICING = "family-slicing";
        HELP_CENTER = "help-center";
        LEGAL_PRIVACY = "legal-privacy-center";
        MEASUREMENT_LIBRARY = "measurement-library";
        MODELS_WORKSPACE = "models";
        NETLIST_WORKSPACE = "netlist";
        OPEN_CONSOLE = "open-console";
        OPEN_NOTIFICATIONS = "open-notifications";
        OPEN_RUN_IN_RESULTS = "open-run-in-results";
        PREFERENCES = "preferences";
        PRINT_HARDCOPY = "print-hardcopy";
        PROJECT_LAUNCHER = "project-launcher";
        PROJECT_RECOVERY = "project-recovery";
        PROJECT_WORKSPACE = "project";
        RESET_ACTIVE_VIEW = "reset-active-view";
        RESET_LAYOUT = "reset-workspace-layout";
        RESULTS_WORKSPACE = "results";
        RESULT_OPERATING_POINT = "result-operating-point";
        RESULT_SPECIFICATIONS = "result-specifications";
        RESULT_WAVEFORMS = "waveforms";
        SAVE = "save-project";
        SHEET_PROPERTIES = "page-setup";
        SIMULATION_ANALYSES = "simulation-analyses";
        SIMULATE_WORKSPACE = "simulate";
        SPECIALIST_TOOL_BROWSER = "specialist-tool-browser";
        STIMULUS_WORKSPACE = "stimulus";
        SUPPORT_BUNDLE = "support-bundle";
        TOGGLE_CONSOLE = "console";
        TOGGLE_CONSOLE_MAXIMIZED = "console-maximize";
        TOGGLE_FOCUS_MODE = "toggle-focus-mode";
        TOGGLE_SPLIT_VIEW = "toggle-split-view";
        VERIFY_WORKSPACE = "verify";
        VISIBILITY_OPTIONS = "visibility-options";
        WORKSPACE_LAYOUTS = "workspace-layouts";
        ZOOM_FIT = "canvas-fit-sheet";
        ZOOM_IN = "zoom-in";
        ZOOM_OUT = "zoom-out";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NO_BINDINGS: &[ShortcutBinding] = &[];

    fn spec(key: &'static str, owner: CommandOwner) -> CommandSpec {
        CommandSpec {
            key: CommandKey::new(key),
            label: key,
            group: "",
            flags: CommandFlags::NONE,
            context: ShortcutContext::Global,
            bindings: NO_BINDINGS,
            owner,
        }
    }

    fn problems(registry: &Registry) -> Vec<String> {
        registry
            .validate([])
            .err()
            .map(|problems| problems.lines().map(str::to_owned).collect())
            .unwrap_or_default()
    }

    #[test]
    fn a_registry_that_holds_together_validates() {
        static ORDER: [CommandKey; 2] = [CommandKey::new("save"), CommandKey::new("open")];
        static ALIASES: [(&str, CommandKey); 1] = [("save-file", CommandKey::new("save"))];
        let registry = Registry::new(
            &ORDER,
            vec![
                spec("open", CommandOwner::Project),
                spec("save", CommandOwner::Project),
            ],
            &ALIASES,
        );
        assert_eq!(registry.validate([]), Ok(()));
        assert_eq!(registry.order(), ORDER);
        assert_eq!(registry.resolve("open"), Some(CommandKey::new("open")));
        assert_eq!(registry.resolve("save-file"), Some(CommandKey::new("save")));
        assert_eq!(registry.resolve("close"), None);
        assert_eq!(
            registry.spec(CommandKey::new("save")).map(|spec| spec.key),
            Some(CommandKey::new("save"))
        );
        assert!(registry.spec(CommandKey::new("save-file")).is_none());
    }

    #[test]
    fn a_key_two_owners_declare_is_reported_with_both() {
        static ORDER: [CommandKey; 1] = [CommandKey::new("undo")];
        let registry = Registry::new(
            &ORDER,
            vec![
                spec("undo", CommandOwner::Framework),
                spec("undo", CommandOwner::Design),
            ],
            &[],
        );
        assert_eq!(
            problems(&registry),
            ["`undo` is declared by Framework and by Design"]
        );
        // The first declaration answers until the registry is fixed.
        assert_eq!(
            registry
                .spec(CommandKey::new("undo"))
                .map(|spec| spec.owner),
            Some(CommandOwner::Framework)
        );
    }

    #[test]
    fn the_order_must_list_each_declared_key_once_and_nothing_else() {
        static ORDER: [CommandKey; 3] = [
            CommandKey::new("open"),
            CommandKey::new("open"),
            CommandKey::new("retired"),
        ];
        let registry = Registry::new(
            &ORDER,
            vec![
                spec("open", CommandOwner::Project),
                spec("save", CommandOwner::Project),
            ],
            &[],
        );
        assert_eq!(
            problems(&registry),
            [
                "`open` is ordered twice",
                "`retired` is ordered but no owner declares it",
                "`save` is declared by Project but not ordered",
            ]
        );
    }

    #[test]
    fn an_alias_must_name_a_declared_key_without_being_one() {
        static ORDER: [CommandKey; 2] = [CommandKey::new("open"), CommandKey::new("save")];
        static ALIASES: [(&str, CommandKey); 2] = [
            ("open", CommandKey::new("save")),
            ("export-old", CommandKey::new("export")),
        ];
        let registry = Registry::new(
            &ORDER,
            vec![
                spec("open", CommandOwner::Project),
                spec("save", CommandOwner::Project),
            ],
            &ALIASES,
        );
        assert_eq!(
            problems(&registry),
            [
                "alias `open` is also a key",
                "alias `export-old` names `export`, which no owner declares",
            ]
        );
        // An alias is answered before the key it shadows.
        assert_eq!(registry.resolve("open"), Some(CommandKey::new("save")));
    }

    #[test]
    fn a_key_a_menu_or_the_framework_names_must_be_declared() {
        static ORDER: [CommandKey; 1] = [CommandKey::new("open")];
        let registry = Registry::new(&ORDER, vec![spec("open", CommandOwner::Project)], &[]);
        assert_eq!(
            registry.validate([
                ("the File menu", CommandKey::new("open")),
                ("the File menu", CommandKey::new("close")),
                ("framework code", CommandKey::new("retired")),
            ]),
            Err("the File menu names `close`, which no owner declares\n\
                 framework code names `retired`, which no owner declares"
                .to_owned())
        );
    }

    #[test]
    fn a_command_left_out_is_declared_once_and_never_ordered() {
        static ORDER: [CommandKey; 2] = [CommandKey::new("open"), CommandKey::new("add-page")];
        let registry = Registry::new(&ORDER, vec![spec("open", CommandOwner::Project)], &[])
            .with_unlisted(vec![
                spec("save-page", CommandOwner::Hardcopy),
                spec("open", CommandOwner::Hardcopy),
                spec("save-page", CommandOwner::Design),
                spec("add-page", CommandOwner::Hardcopy),
            ]);
        assert_eq!(
            problems(&registry),
            [
                "`add-page` is ordered but no owner declares it",
                "`open` is declared by Project and left out by Hardcopy",
                "`open` is left out but ordered",
                "`save-page` is declared by Hardcopy and left out by Design",
                "`add-page` is left out but ordered",
            ]
        );
        // A command left out is never answered by key or id.
        assert!(registry.spec(CommandKey::new("save-page")).is_none());
        assert_eq!(registry.resolve("save-page"), None);
    }
}
