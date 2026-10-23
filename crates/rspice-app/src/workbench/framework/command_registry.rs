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
pub(crate) enum CommandOwner {
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

/// The yes-or-no facts a command declares about itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CommandFlags(u8);

impl CommandFlags {
    pub(crate) const NONE: Self = Self(0);
    /// Listed in the command palette.
    pub(crate) const PALETTE_VISIBLE: Self = Self(1);
    /// A browser keeps the command's primary chord for itself, so a browser
    /// profile needs an alternate for it.
    pub(crate) const PRIMARY_RESERVED_BY_BROWSER: Self = Self(1 << 1);

    /// These flags, with `flag` set when `set` holds.
    #[must_use]
    pub(crate) const fn with(self, flag: Self, set: bool) -> Self {
        if set { Self(self.0 | flag.0) } else { self }
    }

    #[must_use]
    pub(crate) const fn contains(self, flag: Self) -> bool {
        self.0 & flag.0 == flag.0
    }
}

/// What one command declares about itself.
#[derive(Debug, Clone, Copy)]
pub(crate) struct CommandSpec {
    pub(crate) key: CommandKey,
    pub(crate) flags: CommandFlags,
    /// The context the command's keys act in.
    pub(crate) context: ShortcutContext,
    /// The bindings the command ships with, in declaration order.
    pub(crate) bindings: &'static [ShortcutBinding],
    pub(crate) owner: CommandOwner,
}

/// Every command, in order, with the ids it answers to.
pub(crate) struct Registry {
    order: &'static [CommandKey],
    specs: Vec<CommandSpec>,
    /// The first spec declaring each key.
    by_key: HashMap<&'static str, usize>,
    /// Retired ids, each with the key it names now.
    aliases: &'static [(&'static str, CommandKey)],
}

impl Registry {
    /// A registry of `specs`, listed in `order`, that also answers `aliases`.
    /// It is taken as given; [`Self::validate`] says whether it holds
    /// together.
    pub(crate) fn new(
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
        }
    }

    /// Every command, in the order menus list them and the shortcut resolver
    /// breaks ties by.
    #[must_use]
    pub(crate) fn order(&self) -> &'static [CommandKey] {
        self.order
    }

    /// The command a stored id names: an alias first, then a key. `None` for
    /// an id no command answers to.
    #[must_use]
    pub(crate) fn resolve(&self, id: &str) -> Option<CommandKey> {
        if let Some(&(_, key)) = self.aliases.iter().find(|(alias, _)| *alias == id) {
            return Some(key);
        }
        self.by_key.get(id).map(|&index| self.specs[index].key)
    }

    /// What the command under `key` declares; `None` for a key no command
    /// declares.
    #[must_use]
    pub(crate) fn spec(&self, key: CommandKey) -> Option<&CommandSpec> {
        self.by_key
            .get(key.as_str())
            .map(|&index| &self.specs[index])
    }

    /// Whether the registry holds together: every key is declared by exactly
    /// one owner, the order lists each declared key once and nothing else,
    /// and every alias names a declared key without being one. The error
    /// lists every problem, one per line.
    pub(crate) fn validate(&self) -> Result<(), String> {
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
        if problems.is_empty() {
            Ok(())
        } else {
            Err(problems.join("\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NO_BINDINGS: &[ShortcutBinding] = &[];

    fn spec(key: &'static str, owner: CommandOwner) -> CommandSpec {
        CommandSpec {
            key: CommandKey::new(key),
            flags: CommandFlags::NONE,
            context: ShortcutContext::Global,
            bindings: NO_BINDINGS,
            owner,
        }
    }

    fn problems(registry: &Registry) -> Vec<String> {
        registry
            .validate()
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
        assert_eq!(registry.validate(), Ok(()));
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
}
