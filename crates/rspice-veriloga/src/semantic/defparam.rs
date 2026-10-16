//! `defparam` (IEEE 1364-2005 section 12.2.1), folded into instance
//! parameter overrides before analysis.
//!
//! A `defparam` gives a parameter of an instance anywhere in the design a
//! value, across files, and takes precedence over the instance's own `#(...)`
//! override. This pass runs once over the whole compilation unit, right after
//! library binding, and leaves an ordinary design behind: each assignment
//! becomes the named override of the instance that holds the target, which
//! every elaboration pass already reads. Precedence is then the override's,
//! highest first: `defparam`, the instance's `#(...)`, the declared default.
//!
//! A target is resolved downward from the module that writes it, or from a
//! top-level module when its first name is one. An override is a property of
//! the module text that holds the instance, so it is exact for every instance
//! of the writing module when the target is one of that module's own
//! instances. A deeper target is exact only when each module it passes
//! through is instantiated once in the design, and its value is then closed in
//! another module's scope, so it must be a constant that names nothing. Every
//! other form is refused by name.
//!
//! Two `defparam`s on one parameter: the last in source order wins (section
//! 12.2.1), with a warning naming both.

use super::SemanticWarning;
use crate::ast::{
    DefparamAssignment, DigitalExpr, Expression, Item, Module, ParameterOverride, SourceFile,
};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::{BTreeMap, HashMap};

/// A `defparam` on a parameter of a top-level module itself.
#[derive(Debug, Clone)]
pub(crate) struct RootDefparam {
    pub module: SmolStr,
    pub parameter: SmolStr,
    pub value: Expression,
    pub span: Span,
    /// The module that wrote it and its target as written.
    pub writer: SmolStr,
    pub target: String,
}

/// One resolved assignment: the instance `instance` of module `holder`
/// receives `parameter`.
struct Resolved {
    holder: SmolStr,
    instance: SmolStr,
    /// The target module's declared name for the parameter.
    parameter: SmolStr,
    writer: SmolStr,
    target: String,
    value: Expression,
    span: Span,
}

fn refuse(detail: String, span: Span) -> CompileError {
    SemanticError::new(SemanticErrorKind::UnsupportedFeature(detail), span).into()
}

fn invalid(detail: String, span: Span) -> CompileError {
    SemanticError::new(SemanticErrorKind::InvalidExpression(detail), span).into()
}

/// Fold every `defparam` of `file` into instance overrides. Returns the
/// assignments to top-level modules' own parameters, which the root's
/// specialization applies.
pub(crate) fn apply(
    file: &mut SourceFile,
    warnings: &mut Vec<SemanticWarning>,
) -> CompileResult<Vec<RootDefparam>> {
    if !file
        .items
        .iter()
        .any(|item| matches!(item, Item::Module(module) if !module.defparams.is_empty()))
    {
        return Ok(Vec::new());
    }
    let modules: HashMap<SmolStr, &Module> = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Module(module) => Some((module.name.clone(), module)),
            _ => None,
        })
        .collect();
    let mut instantiations: HashMap<&str, usize> = HashMap::new();
    for module in modules.values() {
        for instance in &module.instances {
            *instantiations.entry(instance.module.as_str()).or_default() += 1;
        }
    }
    let count = |name: &str| instantiations.get(name).copied().unwrap_or(0);

    let mut assignments: BTreeMap<(SmolStr, SmolStr, SmolStr), Resolved> = BTreeMap::new();
    let mut order: Vec<(SmolStr, SmolStr, SmolStr)> = Vec::new();
    let mut roots: Vec<RootDefparam> = Vec::new();
    for item in &file.items {
        let Item::Module(writer) = item else { continue };
        for defparam in &writer.defparams {
            let resolved = resolve(writer, defparam, &modules, &count)?;
            let resolved = match resolved {
                Target::Root(root) => {
                    if let Some(earlier) = roots.iter().position(|earlier| {
                        earlier.module == root.module && earlier.parameter == root.parameter
                    }) {
                        let earlier = roots.remove(earlier);
                        warnings.push(replaced(
                            &format!("{}.{}", root.module, root.parameter),
                            writer.name.as_str(),
                            &root.target,
                            (&earlier.writer, &earlier.target),
                            defparam.span,
                        ));
                    }
                    roots.push(root);
                    continue;
                }
                Target::Instance(resolved) => resolved,
            };
            let key = (
                resolved.holder.clone(),
                resolved.instance.clone(),
                resolved.parameter.clone(),
            );
            if let Some(earlier) = assignments.remove(&key) {
                warnings.push(replaced(
                    &format!(
                        "{}.{}.{}",
                        resolved.holder, resolved.instance, resolved.parameter
                    ),
                    &resolved.writer,
                    &resolved.target,
                    (&earlier.writer, &earlier.target),
                    resolved.span,
                ));
                order.retain(|entry| *entry != key);
            }
            order.push(key.clone());
            assignments.insert(key, resolved);
        }
    }

    // Each holder's parameter order, for an instance with ordered overrides.
    let parameter_order: HashMap<SmolStr, Vec<SmolStr>> = modules
        .iter()
        .map(|(name, module)| {
            (
                name.clone(),
                module
                    .parameters
                    .iter()
                    .map(|parameter| parameter.name.clone())
                    .collect(),
            )
        })
        .collect();
    let aliases: HashMap<SmolStr, Vec<(SmolStr, SmolStr)>> = modules
        .iter()
        .map(|(name, module)| {
            (
                name.clone(),
                module
                    .aliasparams
                    .iter()
                    .map(|alias| (alias.alias.clone(), alias.target.clone()))
                    .collect(),
            )
        })
        .collect();
    drop(modules);

    for key in order {
        let resolved = assignments.remove(&key).expect("ordered assignment");
        for item in &mut file.items {
            let Item::Module(module) = item else { continue };
            if module.name != resolved.holder {
                continue;
            }
            let template = module
                .generate_template
                .as_deref_mut()
                .map(|template| &mut template.module);
            for instances in std::iter::once(&mut module.instances)
                .chain(template.map(|module| &mut module.instances))
            {
                for instance in instances
                    .iter_mut()
                    .filter(|instance| instance.name == resolved.instance)
                {
                    let declared = parameter_order
                        .get(&instance.module)
                        .map(Vec::as_slice)
                        .unwrap_or_default();
                    let aliases = aliases
                        .get(&instance.module)
                        .map(Vec::as_slice)
                        .unwrap_or_default();
                    name_ordered_overrides(&mut instance.parameters, declared, instance.span)?;
                    instance.parameters.retain(|existing| {
                        existing.name.as_ref().is_none_or(|name| {
                            canonical(name, aliases) != resolved.parameter.as_str()
                        })
                    });
                    instance.parameters.push(ParameterOverride {
                        name: Some(resolved.parameter.clone()),
                        value: resolved.value.clone(),
                        span: resolved.span,
                    });
                }
            }
        }
    }
    for item in &mut file.items {
        if let Item::Module(module) = item {
            module.defparams.clear();
            if let Some(template) = module.generate_template.as_deref_mut() {
                template.module.defparams.clear();
            }
        }
    }
    Ok(roots)
}

/// Give each top-level module's `defparam`-assigned parameters their values,
/// after the selected root's instance values were applied: a `defparam`
/// takes precedence over them (design P2 section 4.2).
pub(crate) fn apply_to_roots(file: &mut SourceFile, roots: &[RootDefparam]) -> CompileResult<()> {
    for item in &mut file.items {
        let Item::Module(module) = item else { continue };
        let mut changed = false;
        for root in roots.iter().filter(|root| root.module == module.name) {
            if module.unspecialized_parameters.is_none() {
                module.unspecialized_parameters = Some(module.parameters.clone());
            }
            let parameter = module
                .parameters
                .iter_mut()
                .find(|parameter| parameter.name == root.parameter)
                .expect("resolved root parameter");
            parameter.default = Some(root.value.clone());
            parameter.is_given = true;
            changed = true;
        }
        if changed {
            super::SemanticAnalyzer::validate_root_array_replacements(module)?;
            crate::parser::expand_specialized_generates(module)?;
        }
    }
    Ok(())
}

/// The warning for an instance value of a root parameter a `defparam` assigns.
pub(crate) fn instance_value_replaced(root: &RootDefparam, given: &str) -> SemanticWarning {
    SemanticWarning {
        code: "VA-SEM-DEFPARAM-REPLACED",
        message: format!(
            "`defparam {}` in module `{}` takes precedence over the instance value given for \
             parameter `{given}` of `{}`, which is ignored",
            root.target, root.writer, root.module
        ),
        span: root.span,
    }
}

enum Target {
    Root(RootDefparam),
    Instance(Resolved),
}

fn resolve(
    writer: &Module,
    defparam: &DefparamAssignment,
    modules: &HashMap<SmolStr, &Module>,
    count: &impl Fn(&str) -> usize,
) -> CompileResult<Target> {
    let target = defparam.target();
    let (parameter, parameter_span) = defparam.path.last().expect("parsed target").clone();
    let scopes = &defparam.path[..defparam.path.len() - 1];
    // A simple name is the writer's own parameter: one of a top-level module
    // is a root assignment, and one of an instantiated module would reach
    // past every instance's own override, which is not resolved yet.
    let (mut current, chain) = if scopes.is_empty() {
        if count(writer.name.as_str()) != 0 {
            return Err(refuse(
                format!(
                    "`defparam {target}` assigns a parameter of module `{}` itself, which is \
                     instantiated; a `defparam` that reaches every instance of its own module \
                     is not resolved yet",
                    writer.name
                ),
                defparam.span,
            ));
        }
        (writer, scopes)
    } else if writer
        .instances
        .iter()
        .any(|instance| instance.name == scopes[0].0)
    {
        (writer, scopes)
    } else if let Some(root) = modules
        .get(&scopes[0].0)
        .filter(|module| count(module.name.as_str()) == 0)
    {
        (*root, &scopes[1..])
    } else {
        let (first, first_span) = &scopes[0];
        if modules.contains_key(first) {
            return Err(refuse(
                format!(
                    "`defparam {target}` in module `{}` starts at module `{first}`, which is \
                 instantiated; an upward reference (IEEE 1364-2005 section 12.6) is not \
                 resolved for a `defparam` yet",
                    writer.name
                ),
                *first_span,
            ));
        }
        return Err(invalid(
            format!(
                "`defparam {target}` in module `{}` names `{first}`, which is neither an \
                 instance of that module nor a top-level module",
                writer.name
            ),
            *first_span,
        ));
    };
    if chain.is_empty() {
        let canonical = declared_parameter(current, &parameter, &target, parameter_span)?;
        // An analog root's parameters take their instance values at run
        // time, after this pass, where nothing would let the `defparam` win.
        if current.analog_block.is_some() {
            return Err(refuse(
                format!(
                    "`defparam {target}` assigns a parameter of the top-level module `{}`, \
                     which has an analog block; its instance values are applied at run time, \
                     where a `defparam` cannot take precedence over them yet",
                    current.name
                ),
                defparam.span,
            ));
        }
        if current.name != writer.name && !scope_free(&defparam.value) {
            return Err(refuse(
                scope_message(&target, &writer.name),
                defparam.value.span(),
            ));
        }
        return Ok(Target::Root(RootDefparam {
            module: current.name.clone(),
            parameter: canonical,
            value: defparam.value.clone(),
            span: defparam.span,
            writer: writer.name.clone(),
            target,
        }));
    }
    let exact_scope = std::ptr::eq(current, writer) && chain.len() == 1;
    let mut holder = current;
    let mut instance_name = SmolStr::default();
    for (index, (name, span)) in chain.iter().enumerate() {
        let Some(instance) = current
            .instances
            .iter()
            .find(|instance| instance.name == *name)
        else {
            return Err(invalid(
                format!(
                    "`defparam {target}` names `{name}`, which is not an instance of module `{}`",
                    current.name
                ),
                *span,
            ));
        };
        let Some(child) = modules.get(&instance.module) else {
            return Err(invalid(
                format!(
                    "`defparam {target}` reaches instance `{name}` of module `{}`, which is not \
                     declared",
                    instance.module
                ),
                *span,
            ));
        };
        if index + 1 < chain.len() && count(child.name.as_str()) != 1 {
            return Err(refuse(
                format!(
                    "`defparam {target}` passes through module `{}`, which is instantiated {} \
                     times; a `defparam` below one instance of a module that has several is not \
                     resolved yet, so give the value as a `#(...)` override",
                    child.name,
                    count(child.name.as_str())
                ),
                *span,
            ));
        }
        holder = current;
        instance_name = name.clone();
        current = child;
    }
    if !exact_scope && !scope_free(&defparam.value) {
        return Err(refuse(
            scope_message(&target, &writer.name),
            defparam.value.span(),
        ));
    }
    let canonical = declared_parameter(current, &parameter, &target, parameter_span)?;
    Ok(Target::Instance(Resolved {
        holder: holder.name.clone(),
        instance: instance_name,
        parameter: canonical,
        writer: writer.name.clone(),
        target,
        value: defparam.value.clone(),
        span: defparam.span,
    }))
}

/// The declared name of `parameter` in `module`, through an `aliasparam`.
fn declared_parameter(
    module: &Module,
    parameter: &str,
    target: &str,
    span: Span,
) -> CompileResult<SmolStr> {
    let aliases: Vec<_> = module
        .aliasparams
        .iter()
        .map(|alias| (alias.alias.clone(), alias.target.clone()))
        .collect();
    let name = canonical(parameter, &aliases);
    if let Some(declared) = module
        .parameters
        .iter()
        .find(|declared| declared.name == name)
    {
        return Ok(declared.name.clone());
    }
    if module
        .localparams
        .iter()
        .any(|declared| declared.name == name)
    {
        return Err(invalid(
            format!(
                "`defparam {target}` assigns `{parameter}`, a `localparam` of module `{}`; IEEE \
                 1364-2005 section 12.2 lets no override reach a local parameter",
                module.name
            ),
            span,
        ));
    }
    Err(invalid(
        format!(
            "`defparam {target}`: module `{}` declares no parameter `{parameter}`",
            module.name
        ),
        span,
    ))
}

fn canonical<'a>(name: &'a str, aliases: &'a [(SmolStr, SmolStr)]) -> &'a str {
    aliases
        .iter()
        .find(|(alias, _)| alias == name)
        .map_or(name, |(_, target)| target.as_str())
}

/// Give an instance's ordered overrides their parameters' names, so a
/// `defparam` can replace one by name.
fn name_ordered_overrides(
    overrides: &mut [ParameterOverride],
    declared: &[SmolStr],
    span: Span,
) -> CompileResult<()> {
    for (index, entry) in overrides.iter_mut().enumerate() {
        if entry.name.is_some() {
            continue;
        }
        let Some(name) = declared.get(index) else {
            return Err(invalid(
                format!(
                    "the instance has {} ordered parameter overrides but its module declares {} \
                     parameters",
                    index + 1,
                    declared.len()
                ),
                span,
            ));
        };
        entry.name = Some(name.clone());
    }
    Ok(())
}

/// Whether a value means the same in every scope: it names nothing.
fn scope_free(value: &Expression) -> bool {
    let mut free = true;
    crate::ast::visit_expression(value, &mut |expression| {
        if matches!(
            expression,
            Expression::Identifier(_)
                | Expression::Call(_)
                | Expression::ArrayAccess(_)
                | Expression::BranchAccess(_)
                | Expression::AnalogOperator(_)
                | Expression::NoiseSource(_)
                | Expression::Digital(
                    DigitalExpr::PartSelect(_)
                        | DigitalExpr::IndexedPartSelect(_)
                        | DigitalExpr::ArraySelect(_)
                )
        ) {
            free = false;
        }
    });
    free
}

fn scope_message(target: &str, writer: &str) -> String {
    format!(
        "the value of `defparam {target}` in module `{writer}` names a declaration, and the \
         target is not an instance of `{writer}` itself, so the value would be read in another \
         module's scope; such a value is not resolved yet, so write a constant or give the value \
         as a `#(...)` override"
    )
}

fn replaced(
    parameter: &str,
    writer: &str,
    target: &str,
    (earlier_writer, earlier_target): (&str, &str),
    span: Span,
) -> SemanticWarning {
    SemanticWarning {
        code: "VA-SEM-DEFPARAM-REPLACED",
        message: format!(
            "`defparam {target}` in module `{writer}` replaces the earlier `defparam \
             {earlier_target}` in module `{earlier_writer}` on parameter `{parameter}`; IEEE \
             1364-2005 section 12.2.1 keeps the last one in source order"
        ),
        span,
    }
}
