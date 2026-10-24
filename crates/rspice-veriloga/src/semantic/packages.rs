//! Packages, imports and nettypes, bound once the whole unit is parsed (IEEE
//! 1800-2017 sections 3.12.1, 6.6.7 and 26.2-26.4).
//!
//! The HDL sources of a run are one compilation unit, so its packages are
//! collected before any name is bound: a module may import a package that a
//! later file declares. That is this compiler's rule for its one unit, not
//! something section 26 says about file order.
//!
//! What binding leaves behind:
//! - a nettype-typed net is checked to name a nettype of `real` data, through
//!   any aliases, and stays [`DigitalNetKind::Nettype`];
//! - a package or compilation-unit parameter a module uses, plainly or as
//!   `p::name`, becomes one of the module's local parameters: under the name
//!   it is used by, with its expression rewritten into names that say which
//!   scope each part comes from (`p::other`), and those added too. A scoped
//!   name contains `::`, which no declaration can spell, so none can collide
//!   with the module's own.
//!
//! The visibility rules are section 26.3's, read at the positions the names
//! are written in a module: a local declaration hides a wildcard import from
//! where it stands, a reference that a wildcard import satisfies before the
//! local declaration binds the import (and the declaration then conflicts
//! with it), a name two wildcard imports provide is ambiguous only where it
//! is referenced, and an explicitly imported name may not also be declared
//! in the importing scope.

use crate::ast::*;
use crate::error::{SemanticError, SemanticErrorKind};
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Bind the packages, imports and nettypes of a parsed unit.
pub(crate) fn resolve(source: &mut SourceFile) -> Result<(), SemanticError> {
    let systemverilog = source.items.iter().any(|item| match item {
        Item::Package(_) | Item::UnitItem(_) => true,
        Item::Module(module) | Item::ConnectModule(module) => {
            module.language == SourceLanguage::SystemVerilog
        }
        _ => false,
    });
    if !systemverilog {
        return Ok(());
    }
    let unit = Unit::collect(source)?;
    unit.check()?;
    for item in &mut source.items {
        let (Item::Module(module) | Item::ConnectModule(module)) = item else {
            continue;
        };
        if module.language != SourceLanguage::SystemVerilog {
            continue;
        }
        let mut added = unit.bind_module(module)?;
        if added.is_empty() {
            continue;
        }
        // A module holds what it imports from where it starts, so the use
        // order check never sees a package that a later file declares as a
        // declaration below the use. Its expressions keep the package's
        // spans, which is where any error in them is written.
        let start = Span {
            end: module.span.start,
            ..module.span
        };
        for parameter in &mut added {
            parameter.span = start;
        }
        if let Some(template) = &mut module.generate_template {
            template
                .module
                .localparams
                .splice(0..0, added.iter().cloned());
        }
        module.localparams.splice(0..0, added);
    }
    Ok(())
}

/// The scope a package-level or compilation-unit declaration lives in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Scope<'u> {
    Package(&'u str),
    /// The compilation-unit scope, `$unit` (section 3.12.1).
    Unit,
}

impl Scope<'_> {
    /// How a scoped name spells it.
    fn prefix(self) -> &'static str {
        match self {
            Self::Package(_) => "",
            Self::Unit => "$unit",
        }
    }

    fn qualify(self, name: &str) -> SmolStr {
        match self {
            Self::Package(package) => format!("{package}::{name}").into(),
            Self::Unit => format!("{}::{name}", self.prefix()).into(),
        }
    }

    fn describe(self) -> String {
        match self {
            Self::Package(package) => format!("package `{package}`"),
            Self::Unit => "the compilation-unit scope".into(),
        }
    }
}

/// A declaration another scope can name.
#[derive(Debug, Clone, Copy)]
enum Declared<'u> {
    Nettype(&'u NettypeDecl),
    Parameter(&'u ParameterDecl),
}

/// The declarations of one package, or of the compilation-unit scope.
#[derive(Debug, Default)]
struct Declarations {
    imports: Vec<ImportDecl>,
    nettypes: Vec<NettypeDecl>,
    parameters: Vec<ParameterDecl>,
    span: Option<Span>,
}

impl Declarations {
    fn own(&self, name: &str) -> Option<Declared<'_>> {
        self.nettypes
            .iter()
            .find(|nettype| nettype.name == name)
            .map(Declared::Nettype)
            .or_else(|| {
                self.parameters
                    .iter()
                    .find(|parameter| parameter.name == name)
                    .map(Declared::Parameter)
            })
    }
}

/// Every package of the unit and its compilation-unit declarations.
struct Unit {
    packages: BTreeMap<SmolStr, Declarations>,
    unit: Declarations,
}

fn error(message: String, span: Span) -> SemanticError {
    SemanticError::new(SemanticErrorKind::SvPackage(message), span)
}

fn nettype_error(message: String, span: Span) -> SemanticError {
    SemanticError::new(SemanticErrorKind::SvNettype(message), span)
}

impl Unit {
    fn collect(source: &SourceFile) -> Result<Self, SemanticError> {
        let mut packages: BTreeMap<SmolStr, Declarations> = BTreeMap::new();
        let mut unit = Declarations::default();
        for item in &source.items {
            match item {
                Item::Package(package) => {
                    if let Some(first) = packages.get(&package.name) {
                        return Err(error(
                            format!(
                                "package `{}` is declared twice in one compilation unit; the first \
                                 declaration is at byte {} (IEEE 1800-2017 section 26.2)",
                                package.name,
                                first.span.map_or(0, |span| span.start)
                            ),
                            package.span,
                        ));
                    }
                    packages.insert(
                        package.name.clone(),
                        Declarations {
                            imports: package.imports.clone(),
                            nettypes: package.nettypes.clone(),
                            parameters: package.parameters.clone(),
                            span: Some(package.span),
                        },
                    );
                }
                Item::UnitItem(UnitItem::Import(import)) => unit.imports.push(import.clone()),
                Item::UnitItem(UnitItem::Nettype(nettype)) => unit.nettypes.push(nettype.clone()),
                Item::UnitItem(UnitItem::Parameter(parameter)) => {
                    unit.parameters.push((**parameter).clone());
                }
                _ => {}
            }
        }
        Ok(Self { packages, unit })
    }

    fn declarations(&self, scope: Scope<'_>) -> &Declarations {
        match scope {
            Scope::Package(package) => &self.packages[package],
            Scope::Unit => &self.unit,
        }
    }

    /// The package `name`, for an import or a scoped name at `span`.
    fn package<'u>(&'u self, name: &str, span: Span) -> Result<Scope<'u>, SemanticError> {
        self.packages
            .get_key_value(name)
            .map(|(name, _)| Scope::Package(name.as_str()))
            .ok_or_else(|| {
                error(
                    format!("there is no package `{name}` in the compilation unit"),
                    span,
                )
            })
    }

    /// What `package` declares as `name` for a scope importing it: its own
    /// declarations, and not the names it imports itself (section 26.3).
    fn exported<'u>(&'u self, package: Scope<'u>, name: &str) -> Option<Declared<'u>> {
        self.declarations(package).own(name)
    }

    /// `scope::name`, for a scoped name at `span`.
    fn scoped<'u>(
        &'u self,
        scope: &str,
        name: &str,
        span: Span,
    ) -> Result<(Scope<'u>, Declared<'u>), SemanticError> {
        let scope = if scope == "$unit" {
            Scope::Unit
        } else {
            self.package(scope, span)?
        };
        self.exported(scope, name)
            .map(|declared| (scope, declared))
            .ok_or_else(|| error(format!("{} declares no `{name}`", scope.describe()), span))
    }

    /// `name` as a package's or the unit scope's own declarations see it:
    /// its own declarations, then its explicit imports, then the one
    /// package of its wildcard imports that declares it.
    fn lookup<'u>(
        &'u self,
        scope: Scope<'u>,
        name: &str,
        span: Span,
    ) -> Result<Option<(Scope<'u>, Declared<'u>)>, SemanticError> {
        let declarations = self.declarations(scope);
        if let Some(declared) = declarations.own(name) {
            return Ok(Some((scope, declared)));
        }
        for import in &declarations.imports {
            if import.name.as_deref() == Some(name) {
                let package = self.package(&import.package, import.span)?;
                return Ok(self.exported(package, name).map(|found| (package, found)));
            }
        }
        let mut providers = BTreeMap::new();
        for import in declarations
            .imports
            .iter()
            .filter(|import| import.name.is_none())
        {
            let package = self.package(&import.package, import.span)?;
            if let Some(declared) = self.exported(package, name) {
                providers.insert(package, declared);
            }
        }
        if providers.len() > 1 {
            return Err(ambiguous(name, providers.keys().copied(), span));
        }
        Ok(providers.into_iter().next())
    }

    /// The imports, nettype aliases and parameters of every package and of
    /// the unit scope name what exists.
    fn check(&self) -> Result<(), SemanticError> {
        let scopes = self
            .packages
            .keys()
            .map(|package| Scope::Package(package.as_str()))
            .chain(std::iter::once(Scope::Unit));
        for scope in scopes {
            let declarations = self.declarations(scope);
            for import in &declarations.imports {
                let package = self.package(&import.package, import.span)?;
                if Scope::Package(&import.package) == scope {
                    return Err(error(
                        format!("package `{}` imports itself", import.package),
                        import.span,
                    ));
                }
                if let Some(name) = &import.name {
                    self.exported(package, name).ok_or_else(|| {
                        error(
                            format!(
                                "package `{}` declares no `{name}` to import",
                                import.package
                            ),
                            import.span,
                        )
                    })?;
                    if declarations.own(name).is_some() {
                        return Err(explicit_conflict(
                            &import.package,
                            name,
                            scope.describe(),
                            import.span,
                        ));
                    }
                }
            }
            for nettype in &declarations.nettypes {
                self.nettype_data(scope, nettype, 0)?;
            }
        }
        Ok(())
    }

    /// Follow `nettype`'s aliases, declared in `scope`, to its data.
    fn nettype_data(
        &self,
        scope: Scope<'_>,
        nettype: &NettypeDecl,
        depth: usize,
    ) -> Result<(), SemanticError> {
        let NettypeData::Alias(target) = &nettype.data else {
            return Ok(());
        };
        if depth > MAX_ALIAS_DEPTH {
            return Err(nettype_error(
                format!("nettype `{}` is an alias of itself", nettype.name),
                nettype.span,
            ));
        }
        let (scope, declared) = match &target.package {
            Some(package) => self.scoped(package, &target.name, target.span)?,
            None => self
                .lookup(scope, &target.name, target.span)?
                .ok_or_else(|| unknown_nettype(target, None))?,
        };
        match declared {
            Declared::Nettype(found) => self.nettype_data(scope, found, depth + 1),
            Declared::Parameter(_) => Err(not_a_nettype(target)),
        }
    }

    /// Bind the imports and nettypes of a SystemVerilog module, returning the
    /// local parameters its uses of package and unit parameters stand for.
    fn bind_module(&self, module: &Module) -> Result<Vec<ParameterDecl>, SemanticError> {
        let declared = local_declarations(module);
        let mut explicit: HashMap<&str, (Scope<'_>, Declared<'_>)> = HashMap::new();
        let mut wildcards = Vec::new();
        for import in &module.imports {
            let package = self.package(&import.package, import.span)?;
            let Some(name) = &import.name else {
                wildcards.push((package, import.span));
                continue;
            };
            let found = self.exported(package, name).ok_or_else(|| {
                error(
                    format!(
                        "package `{}` declares no `{name}` to import",
                        import.package
                    ),
                    import.span,
                )
            })?;
            if declared.contains_key(name.as_str()) {
                return Err(explicit_conflict(
                    &import.package,
                    name,
                    format!("module `{}`", module.name),
                    import.span,
                ));
            }
            if let Some((other, _)) = explicit.get(name.as_str())
                && *other != package
            {
                return Err(error(
                    format!(
                        "module `{}` imports `{name}` explicitly from both {} and {} (IEEE \
                         1800-2017 section 26.3)",
                        module.name,
                        other.describe(),
                        package.describe()
                    ),
                    import.span,
                ));
            }
            explicit.insert(name, (package, found));
        }

        let mut uses: BTreeMap<&str, Vec<Span>> = BTreeMap::new();
        for (name, span) in &module.identifier_uses {
            uses.entry(name.as_str()).or_default().push(*span);
        }
        let mut bound: BTreeMap<&str, (Scope<'_>, Declared<'_>)> = BTreeMap::new();
        let mut scoped = Vec::new();
        for (&name, spans) in &uses {
            if let Some((scope, member)) = name.split_once("::") {
                scoped.push((self.scoped(scope, member, spans[0])?, spans[0]));
                continue;
            }
            if let Some(&found) = explicit.get(name) {
                bound.insert(name, found);
                continue;
            }
            let local = declared.get(name).copied();
            let mut providers = BTreeMap::new();
            let mut first: Option<Span> = None;
            for &(package, import) in &wildcards {
                let Some(declared) = self.exported(package, name) else {
                    continue;
                };
                // The first reference this import can satisfy, unless the
                // local declaration already stands before it.
                let Some(&reference) = spans
                    .iter()
                    .find(|span| span.start > import.start)
                    .filter(|span| local.is_none_or(|local| span.start < local))
                else {
                    continue;
                };
                providers.insert(package, declared);
                if first.is_none_or(|first| reference.start < first.start) {
                    first = Some(reference);
                }
            }
            if let Some(reference) = first {
                if local.is_some() {
                    let package = providers.keys().next().copied().unwrap_or(Scope::Unit);
                    return Err(error(
                        format!(
                            "`{name}` is used in module `{}` before its declaration there, so \
                             the use names the `{name}` the wildcard import of {} provides, and \
                             the later declaration conflicts with it; declare `{name}` before \
                             using it (IEEE 1800-2017 section 26.3)",
                            module.name,
                            package.describe()
                        ),
                        reference,
                    ));
                }
                if providers.len() > 1 {
                    return Err(ambiguous(name, providers.keys().copied(), reference));
                }
                bound.extend(providers.into_iter().map(|found| (name, found)));
                continue;
            }
            if local.is_none()
                && let Some(found) = self.lookup(Scope::Unit, name, spans[0])?
            {
                bound.insert(name, found);
            }
        }

        let nettypes: HashMap<&str, &NettypeDecl> = module
            .nettypes
            .iter()
            .map(|nettype| (nettype.name.as_str(), nettype))
            .collect();
        let context = ModuleScope {
            unit: self,
            module,
            declared: &declared,
            nettypes: &nettypes,
            bound: &bound,
        };
        for nettype in &module.nettypes {
            if let NettypeData::Alias(target) = &nettype.data {
                context.nettype(target, 0)?;
            }
        }
        for net in &module.digital_nets {
            if net.kind == DigitalNetKind::Nettype
                && let Some(target) = &net.nettype
            {
                context.nettype(target, 0)?;
            }
        }

        let mut parameters = Parameters::new(self);
        for (&name, &(scope, found)) in &bound {
            if let Declared::Parameter(parameter) = found {
                parameters.add(name.into(), scope, parameter)?;
            }
        }
        for ((scope, found), _) in scoped {
            if let Declared::Parameter(parameter) = found {
                parameters.add(scope.qualify(&parameter.name), scope, parameter)?;
            }
        }
        Ok(parameters.finish())
    }
}

/// Nettype aliases deeper than this are taken for a cycle.
const MAX_ALIAS_DEPTH: usize = 64;

/// What a module's names see while its nettypes are checked.
struct ModuleScope<'a, 'u> {
    unit: &'u Unit,
    module: &'a Module,
    declared: &'a HashMap<&'a str, u32>,
    nettypes: &'a HashMap<&'a str, &'a NettypeDecl>,
    bound: &'a BTreeMap<&'a str, (Scope<'u>, Declared<'u>)>,
}

impl ModuleScope<'_, '_> {
    /// Check that `target`, written in the module, names a nettype of real
    /// data.
    fn nettype(&self, target: &TypeRef, depth: usize) -> Result<(), SemanticError> {
        if depth > MAX_ALIAS_DEPTH {
            return Err(nettype_error(
                format!("nettype `{}` is an alias of itself", target.name),
                target.span,
            ));
        }
        if let Some(package) = &target.package {
            return match self.unit.scoped(package, &target.name, target.span)? {
                (scope, Declared::Nettype(found)) => self.unit.nettype_data(scope, found, 0),
                (_, Declared::Parameter(_)) => Err(not_a_nettype(target)),
            };
        }
        if let Some(found) = self.nettypes.get(target.name.as_str()) {
            if found.span.start > target.span.start {
                return Err(nettype_error(
                    format!(
                        "nettype `{}` is used before its declaration in module `{}`",
                        target.name, self.module.name
                    ),
                    target.span,
                ));
            }
            return match &found.data {
                NettypeData::Real => Ok(()),
                NettypeData::Alias(next) => self.nettype(next, depth + 1),
            };
        }
        if self.declared.contains_key(target.name.as_str()) {
            return Err(not_a_nettype(target));
        }
        match self.bound.get(target.name.as_str()) {
            Some(&(scope, Declared::Nettype(found))) => self.unit.nettype_data(scope, found, 0),
            Some((_, Declared::Parameter(_))) => Err(not_a_nettype(target)),
            None => Err(unknown_nettype(target, Some(&self.module.name))),
        }
    }
}

/// The local parameters a module's uses of package and unit parameters
/// stand for, each rewritten into scoped names.
struct Parameters<'u> {
    unit: &'u Unit,
    added: BTreeMap<SmolStr, ParameterDecl>,
    /// The added names, each after the ones its expressions use, which is
    /// the order a parameter default may refer back in.
    order: Vec<SmolStr>,
}

impl<'u> Parameters<'u> {
    fn new(unit: &'u Unit) -> Self {
        Self {
            unit,
            added: BTreeMap::new(),
            order: Vec::new(),
        }
    }

    /// `parameter`, declared in `scope`, as a local parameter called `name`,
    /// with the parameters its expressions use added under their scoped
    /// names.
    fn add(
        &mut self,
        name: SmolStr,
        scope: Scope<'u>,
        parameter: &ParameterDecl,
    ) -> Result<(), SemanticError> {
        if self.added.contains_key(&name) {
            return Ok(());
        }
        let mut copy = parameter.clone();
        copy.name = name.clone();
        copy.is_given = true;
        // Reserve the name first, so a parameter that uses itself ends.
        self.added.insert(name.clone(), copy.clone());
        let mut needed = Vec::new();
        let mut failure = None;
        for_each_expression(&mut copy, &mut |expression| {
            rewrite(expression, &mut |identifier| {
                if failure.is_some() {
                    return;
                }
                match self.qualified(scope, identifier) {
                    Ok(Some((qualified, scope, parameter))) => {
                        identifier.name = qualified.clone();
                        needed.push((qualified, scope, parameter));
                    }
                    Ok(None) => {}
                    Err(error) => failure = Some(error),
                }
            });
        });
        if let Some(error) = failure {
            return Err(error);
        }
        self.added.insert(name.clone(), copy);
        for (qualified, scope, parameter) in needed {
            self.add(qualified, scope, parameter)?;
        }
        self.order.push(name);
        Ok(())
    }

    /// What the identifier in a parameter expression of `scope` names, as
    /// a scoped name, when it names a parameter.
    fn qualified(
        &self,
        scope: Scope<'u>,
        identifier: &Identifier,
    ) -> Result<Option<(SmolStr, Scope<'u>, &'u ParameterDecl)>, SemanticError> {
        let found = match identifier.name.split_once("::") {
            Some((package, name)) => Some(self.unit.scoped(package, name, identifier.span)?),
            None => self.unit.lookup(scope, &identifier.name, identifier.span)?,
        };
        Ok(match found {
            Some((scope, Declared::Parameter(parameter))) => {
                Some((scope.qualify(&parameter.name), scope, parameter))
            }
            _ => None,
        })
    }

    fn finish(mut self) -> Vec<ParameterDecl> {
        self.order
            .iter()
            .filter_map(|name| self.added.remove(name))
            .collect()
    }
}

/// Every expression a parameter declaration holds.
fn for_each_expression(parameter: &mut ParameterDecl, visit: &mut impl FnMut(&mut Expression)) {
    if let Some(default) = &mut parameter.default {
        visit(default);
    }
    if let Some(range) = &mut parameter.packed_range {
        visit(&mut range.msb);
        visit(&mut range.lsb);
    }
    for dimension in &mut parameter.dimensions {
        visit(&mut dimension.start);
        visit(&mut dimension.end);
    }
}

/// Apply `rename` to every identifier of a constant expression.
fn rewrite(expression: &mut Expression, rename: &mut impl FnMut(&mut Identifier)) {
    match expression {
        Expression::Identifier(identifier) => rename(identifier),
        Expression::Binary(binary) => {
            rewrite(&mut binary.left, rename);
            rewrite(&mut binary.right, rename);
        }
        Expression::Unary(unary) => rewrite(&mut unary.operand, rename),
        Expression::Conditional(conditional) => {
            rewrite(&mut conditional.condition, rename);
            rewrite(&mut conditional.then_expr, rename);
            rewrite(&mut conditional.else_expr, rename);
        }
        Expression::Call(call) => {
            for argument in &mut call.args {
                rewrite(argument, rename);
            }
        }
        Expression::SystemFunction(function) => {
            for argument in &mut function.args {
                rewrite(argument, rename);
            }
        }
        Expression::ArrayLiteral(literal) => {
            for element in &mut literal.elements {
                rewrite_element(element, rename);
            }
        }
        Expression::ArrayAccess(access) => {
            for child in access.children_mut() {
                rewrite(child, rename);
            }
        }
        _ => {}
    }
}

fn rewrite_element(element: &mut ArrayLiteralElement, rename: &mut impl FnMut(&mut Identifier)) {
    match element {
        ArrayLiteralElement::Value(value) => rewrite(value, rename),
        ArrayLiteralElement::Replication(replication) => {
            rewrite(&mut replication.count, rename);
            for element in &mut replication.elements {
                rewrite_element(element, rename);
            }
        }
    }
}

/// Each name a module declares, at the earliest byte any declaration of it
/// starts.
fn local_declarations(module: &Module) -> HashMap<&str, u32> {
    let names = module
        .ports
        .iter()
        .map(|port| (&port.name, port.span))
        .chain(module.port_declarations.iter().flat_map(|declaration| {
            declaration
                .names
                .iter()
                .map(move |name| (name, declaration.span))
        }))
        .chain(
            module
                .nets
                .iter()
                .flat_map(|net| net.names.iter().map(move |name| (name, net.span))),
        )
        .chain(
            module
                .digital_nets
                .iter()
                .flat_map(|net| net.items.iter().map(|item| (&item.name, item.span))),
        )
        .chain(
            module
                .digital_variables
                .iter()
                .flat_map(|variable| variable.items.iter().map(|item| (&item.name, item.span))),
        )
        .chain(
            module
                .variables
                .iter()
                .flat_map(|variable| variable.items.iter().map(|item| (&item.name, item.span))),
        )
        .chain(
            module
                .parameters
                .iter()
                .chain(&module.localparams)
                .map(|parameter| (&parameter.name, parameter.span)),
        )
        .chain(
            module
                .genvars
                .iter()
                .flat_map(|genvar| genvar.names.iter().map(move |name| (name, genvar.span))),
        )
        .chain(
            module
                .aliasparams
                .iter()
                .map(|alias| (&alias.alias, alias.span)),
        )
        .chain(
            module
                .functions
                .iter()
                .map(|function| (&function.name, function.span)),
        )
        .chain(
            module
                .digital_routines
                .iter()
                .map(|routine| (&routine.name, routine.span)),
        )
        .chain(
            module
                .instances
                .iter()
                .map(|instance| (&instance.name, instance.span)),
        )
        .chain(
            module
                .nettypes
                .iter()
                .map(|nettype| (&nettype.name, nettype.span)),
        );
    let mut declared: HashMap<&str, u32> = HashMap::new();
    for (name, span) in names {
        let start = declared.entry(name.as_str()).or_insert(span.start);
        *start = (*start).min(span.start);
    }
    declared
}

fn ambiguous<'u>(
    name: &str,
    packages: impl Iterator<Item = Scope<'u>>,
    span: Span,
) -> SemanticError {
    let packages: Vec<String> = packages.map(Scope::describe).collect();
    error(
        format!(
            "`{name}` is ambiguous: the wildcard imports of {} each provide it; import it \
             explicitly from one of them (IEEE 1800-2017 section 26.3)",
            packages.join(" and ")
        ),
        span,
    )
}

fn explicit_conflict(package: &str, name: &str, scope: String, span: Span) -> SemanticError {
    error(
        format!(
            "`import {package}::{name};` and a declaration of `{name}` in {scope}: an \
             explicitly imported name may not also be declared in the importing scope (IEEE \
             1800-2017 section 26.3)"
        ),
        span,
    )
}

fn not_a_nettype(target: &TypeRef) -> SemanticError {
    nettype_error(
        format!(
            "`{}` is not a nettype, so it cannot type a net (IEEE 1800-2017 section 6.6.7)",
            target.written()
        ),
        target.span,
    )
}

/// The refusal of a type name nothing declares: a Verilog-AMS discipline is
/// analog content, `wreal` is Verilog-AMS's own real net, and anything else
/// is a name to declare or import.
fn unknown_nettype(target: &TypeRef, module: Option<&str>) -> SemanticError {
    let name = target.name.as_str();
    if target.package.is_none() && (name == "ground" || is_discipline(name)) {
        return SemanticError::new(
            SemanticErrorKind::SvAnalog {
                module: module.unwrap_or_default().to_owned(),
            },
            target.span,
        );
    }
    if crate::ast::WrealResolution::from_keyword(name).is_some() {
        return nettype_error(
            format!(
                "`{name}` is a Verilog-AMS net type, which SystemVerilog does not have; declare \
                 a real nettype, `nettype real rnet;`, and nets of it, `rnet x;` (IEEE 1800-2017 \
                 section 6.6.7)"
            ),
            target.span,
        );
    }
    nettype_error(
        format!(
            "there is no nettype `{}` here: declare it, or import the package that does (IEEE \
             1800-2017 sections 6.6.7 and 26.3)",
            target.written()
        ),
        target.span,
    )
}

/// Whether `name` is one of the disciplines the standard discipline header
/// declares.
fn is_discipline(name: &str) -> bool {
    static DISCIPLINES: std::sync::OnceLock<BTreeSet<String>> = std::sync::OnceLock::new();
    DISCIPLINES
        .get_or_init(|| {
            crate::stdlib::builtin_include_document("disciplines.vams")
                .map(|(_, text)| {
                    text.lines()
                        .filter_map(|line| line.trim().strip_prefix("discipline "))
                        .map(|rest| rest.trim_end_matches(';').trim().to_owned())
                        .collect()
                })
                .unwrap_or_default()
        })
        .contains(name)
}
