//! An output expression read once: its authored tree and the `.FUNC` bodies
//! its calls reach, each body parsed once into the context's shared table and
//! called by reference.
//!
//! Writing a call graph out as text costs the product of its fan-outs: an
//! argument that is itself a call is copied into every place its formal is
//! read, and a deck of nested functions grows to hundreds of thousands of
//! leaves. Every consumer of an output expression (validation, dependency
//! scans, typing, evaluation) walks this graph instead, so its work follows
//! the calls and bodies the deck wrote.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use super::behavioral::{
    BehavioralPreparationError, expand_spice_poly_expression, integral_node_name, is_circuit_probe,
    is_simple_probe_reference, serialize_expr,
};
use super::{
    Expr, FunctionDef, FunctionProbeOperands, ParamContext, ParseExpressionWithAbortError,
    parse_expression_with_abort, parse_xyce_expression_with_abort,
};
use crate::abort_signal::AbortSignal;
use crate::config::ExpressionDialect;

/// How a text is read: the deck's expression dialect and its reading of
/// number suffixes, the two settings a parse depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Grammar {
    colon_identifiers: bool,
    ngspice_numbers: bool,
}

impl Grammar {
    fn of(params: &ParamContext) -> Self {
        Self {
            colon_identifiers: params.expression_dialect() == ExpressionDialect::Xyce,
            ngspice_numbers: crate::spice_number::reading_numbers_as_ngspice(),
        }
    }

    fn parse(
        self,
        text: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Expr, ParseExpressionWithAbortError> {
        if self.colon_identifiers {
            parse_xyce_expression_with_abort(text, abort)
        } else {
            parse_expression_with_abort(text, abort)
        }
    }
}

type BodyKey = (String, String, Grammar);
type Body = Result<Arc<Expr>, Arc<str>>;

/// The parsed `.FUNC` bodies of one context, shared with the clones made of
/// it once a body has been read.
///
/// A body is keyed by its function's name, its text and the grammar it is
/// read in, so a scope that redefines a function, or a deck read in another
/// dialect, reads its own body and no entry is ever stale.
#[derive(Clone, Default)]
pub(crate) struct FunctionBodies(OnceLock<Arc<Mutex<HashMap<BodyKey, Body>>>>);

impl std::fmt::Debug for FunctionBodies {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // What the definitions already say, read ahead: no part of the
        // context's own state.
        formatter.write_str("FunctionBodies")
    }
}

impl FunctionBodies {
    /// The body of `function` read in `grammar`, parsed on its first request.
    fn body(
        &self,
        function: &FunctionDef,
        grammar: Grammar,
        abort: &dyn AbortSignal,
    ) -> Result<Body, OutputExpressionError> {
        let key = (function.name.clone(), function.body.clone(), grammar);
        if let Some(body) = self.table().get(&key) {
            return Ok(body.clone());
        }
        let parsed = match expand_spice_poly_expression(&function.body, abort) {
            Err(BehavioralPreparationError::Aborted) => return Err(OutputExpressionError::Aborted),
            Err(BehavioralPreparationError::Semantic(detail)) => Err(Arc::from(detail)),
            Ok(text) => match grammar.parse(&text, abort) {
                Err(ParseExpressionWithAbortError::Aborted) => {
                    return Err(OutputExpressionError::Aborted);
                }
                Err(ParseExpressionWithAbortError::Parse(error)) => Err(Arc::from(format!(
                    "Failed to parse .FUNC {} body '{}': {}",
                    function.name, function.body, error
                ))),
                Ok(body) => Ok(Arc::new(body)),
            },
        };
        self.table().insert(key, parsed.clone());
        Ok(parsed)
    }

    fn table(&self) -> std::sync::MutexGuard<'_, HashMap<BodyKey, Body>> {
        // A panic while the table was held leaves only whole entries in it.
        self.0
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// Why an output expression could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(hidden)]
pub enum OutputExpressionError {
    Aborted,
    /// The authored text does not parse.
    Parse(String),
    /// A call the graph cannot make: an arity mismatch, recursion, a body
    /// that does not parse, or a probe operand bound to no name.
    Semantic(String),
}

/// One function the expression's calls reach, as those calls bind it.
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct FunctionInstance {
    /// The name calls reach it by: the function's own, or, where the deck
    /// binds probe operands ([`FunctionProbeOperands::Bound`]), the name and
    /// the nodes its probe operands read, as `F(N1)`.
    key: String,
    function: FunctionDef,
    body: Arc<Expr>,
    /// Each formal read as a probe operand and the node it names here.
    probe_bindings: Vec<(String, String)>,
}

impl FunctionInstance {
    #[doc(hidden)]
    pub fn key(&self) -> &str {
        &self.key
    }

    #[doc(hidden)]
    pub fn function(&self) -> &FunctionDef {
        &self.function
    }

    /// The function's formals, upper-cased as the definition keeps them.
    #[doc(hidden)]
    pub fn formals(&self) -> &[String] {
        &self.function.args
    }

    /// The body as this instance reads it: probe operands bound and calls
    /// naming their instances.
    #[doc(hidden)]
    pub fn body(&self) -> &Expr {
        &self.body
    }

    /// The node a formal read as a probe operand names in this instance.
    #[doc(hidden)]
    pub fn probe_binding(&self, formal: &str) -> Option<&str> {
        self.probe_bindings
            .iter()
            .find(|(bound, _)| bound.eq_ignore_ascii_case(formal))
            .map(|(_, node)| node.as_str())
    }
}

/// An authored output expression and the function instances its calls
/// reach, in the order a left-to-right reading first calls them.
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct OutputExpression {
    root: Expr,
    instances: Vec<FunctionInstance>,
    index: HashMap<String, usize>,
}

impl OutputExpression {
    /// Read `text` in the deck's grammar and resolve its calls.
    #[doc(hidden)]
    pub fn parse(
        text: &str,
        params: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<Self, OutputExpressionError> {
        let grammar = Grammar::of(params);
        let text = match expand_spice_poly_expression(text, abort) {
            Ok(text) => text,
            Err(BehavioralPreparationError::Aborted) => return Err(OutputExpressionError::Aborted),
            Err(BehavioralPreparationError::Semantic(detail)) => {
                return Err(OutputExpressionError::Semantic(detail));
            }
        };
        let root = match grammar.parse(&text, abort) {
            Ok(root) => Arc::new(root),
            Err(ParseExpressionWithAbortError::Aborted) => {
                return Err(OutputExpressionError::Aborted);
            }
            Err(ParseExpressionWithAbortError::Parse(error)) => {
                return Err(OutputExpressionError::Parse(error.to_string()));
            }
        };
        let mut resolver = CallResolver {
            params,
            grammar,
            abort,
            bound: params.function_probe_operands() == FunctionProbeOperands::Bound,
            probe_formals: HashMap::new(),
            instances: Vec::new(),
            index: HashMap::new(),
        };
        let root = if params.function_count() == 0 {
            root
        } else {
            resolver.resolve_tree(root, Caller::Root, &mut Vec::new())?
        };
        Ok(Self {
            root: Arc::unwrap_or_clone(root),
            instances: resolver.instances,
            index: resolver.index,
        })
    }

    #[doc(hidden)]
    pub fn root(&self) -> &Expr {
        &self.root
    }

    /// The instances, each once, in the order the expression first calls them.
    #[doc(hidden)]
    pub fn instances(&self) -> &[FunctionInstance] {
        &self.instances
    }

    /// The instance a call in one of the graph's trees names.
    #[doc(hidden)]
    pub fn instance(&self, name: &str) -> Option<&FunctionInstance> {
        self.index.get(name).map(|&index| &self.instances[index])
    }

    /// The root and then every instance body, each once.
    #[doc(hidden)]
    pub fn trees(&self) -> impl Iterator<Item = &Expr> {
        std::iter::once(&self.root).chain(self.instances.iter().map(FunctionInstance::body))
    }

    /// Whether any tree of the graph holds a node `predicate` accepts.
    #[doc(hidden)]
    pub fn any_node(&self, mut predicate: impl FnMut(&Expr) -> bool) -> bool {
        self.trees().any(|tree| any_node(tree, &mut predicate))
    }

    /// The node the value is read from with each call entered and each formal
    /// read as its argument: the root the expression has with every call
    /// written out.
    #[doc(hidden)]
    pub fn head(&self) -> &Expr {
        let mut calls: Vec<(&FunctionInstance, &[Expr])> = Vec::new();
        let mut node = &self.root;
        loop {
            if let Expr::FnCall { name, args } = node
                && let Some(instance) = self.instance(name)
            {
                calls.push((instance, args));
                node = instance.body();
                continue;
            }
            if let Expr::Param(name) = node
                && let Some(&(instance, args)) = calls.last()
                && let Some(position) = instance.formals().iter().position(|formal| formal == name)
            {
                calls.pop();
                node = &args[position];
                continue;
            }
            return node;
        }
    }
}

/// Whether `tree` holds a node `predicate` accepts.
fn any_node(tree: &Expr, predicate: &mut impl FnMut(&Expr) -> bool) -> bool {
    let mut pending = vec![tree];
    while let Some(node) = pending.pop() {
        if predicate(node) {
            return true;
        }
        match node {
            Expr::UnaryOp { operand, .. } => pending.push(operand),
            Expr::BinOp { left, right, .. } => {
                pending.push(right);
                pending.push(left);
            }
            Expr::FnCall { args, .. } => pending.extend(args.iter().rev()),
            Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) | Expr::Param(_) => {}
        }
    }
    false
}

/// A tree being resolved: the root, or the body of the instance at an index.
#[derive(Clone, Copy)]
enum Caller {
    Root,
    Instance(usize),
}

/// A formal a function reads as a probe operand, and where that read is
/// written: in this function's body, or in a callee's it passes the formal to.
#[derive(Debug)]
struct ProbeFormal {
    formal: String,
    function: String,
    probe: String,
    written: String,
}

struct CallResolver<'a> {
    params: &'a ParamContext,
    grammar: Grammar,
    abort: &'a dyn AbortSignal,
    bound: bool,
    probe_formals: HashMap<String, Arc<[ProbeFormal]>>,
    instances: Vec<FunctionInstance>,
    index: HashMap<String, usize>,
}

impl CallResolver<'_> {
    /// Resolve every call in `tree`, creating the instances it reaches, and
    /// return the tree with each call naming its instance.
    fn resolve_tree(
        &mut self,
        tree: Arc<Expr>,
        caller: Caller,
        active: &mut Vec<String>,
    ) -> Result<Arc<Expr>, OutputExpressionError> {
        if self.abort.is_aborted() {
            return Err(OutputExpressionError::Aborted);
        }
        let mut renames = Vec::new();
        for (name, args) in calls_in(&tree, self.params) {
            renames.push(self.resolve_call(name, args, caller, active)?);
        }
        if renames.iter().all(Option::is_none) {
            return Ok(tree);
        }
        let mut renames = renames.into_iter();
        Ok(Arc::new(rename_calls(
            Arc::unwrap_or_clone(tree),
            self.params,
            &mut renames,
        )))
    }

    /// Resolve one call: check it against its definition, bind its probe
    /// operands, and resolve its instance's body the first time it is
    /// reached. Returns the instance's key where it is not the call's name.
    fn resolve_call(
        &mut self,
        name: &str,
        args: &[Expr],
        caller: Caller,
        active: &mut Vec<String>,
    ) -> Result<Option<String>, OutputExpressionError> {
        let Some(function) = self.params.get_function(name).cloned() else {
            unreachable!("calls_in keeps calls to defined functions only")
        };
        if args.len() != function.args.len() {
            return Err(OutputExpressionError::Semantic(format!(
                "Function '{}' expects {} args but got {}",
                name,
                function.args.len(),
                args.len()
            )));
        }
        if active.contains(&function.name) {
            return Err(OutputExpressionError::Semantic(format!(
                "Detected recursive .FUNC expansion for '{}'",
                function.name
            )));
        }
        let mut probe_bindings = Vec::new();
        if self.bound {
            let formals = self.probe_formals(&function, active)?;
            for formal in formals.iter() {
                let Some(position) = function.args.iter().position(|arg| *arg == formal.formal)
                else {
                    unreachable!("a probe formal is one of the function's formals")
                };
                let node = self.bind_probe_operand(formal, &args[position], caller)?;
                probe_bindings.push((formal.formal.clone(), node));
            }
        }
        let key = if probe_bindings.is_empty() {
            function.name.clone()
        } else {
            let nodes = probe_bindings
                .iter()
                .map(|(_, node)| node.to_ascii_uppercase())
                .collect::<Vec<_>>();
            format!("{}({})", function.name, nodes.join(","))
        };
        if !self.index.contains_key(&key) {
            let body = self.body(&function)?;
            let body = if probe_bindings.is_empty() {
                body
            } else {
                Arc::new(bind_probe_operands(
                    Arc::unwrap_or_clone(body),
                    &probe_bindings,
                ))
            };
            let instance = self.instances.len();
            self.index.insert(key.clone(), instance);
            self.instances.push(FunctionInstance {
                key: key.clone(),
                function: function.clone(),
                body: Arc::clone(&body),
                probe_bindings,
            });
            active.push(function.name.clone());
            let resolved = self.resolve_tree(body, Caller::Instance(instance), active);
            active.pop();
            self.instances[instance].body = resolved?;
        }
        Ok((key != name).then_some(key))
    }

    fn body(&self, function: &FunctionDef) -> Result<Arc<Expr>, OutputExpressionError> {
        self.params
            .function_bodies()
            .body(function, self.grammar, self.abort)?
            .map_err(|detail| OutputExpressionError::Semantic(detail.to_string()))
    }

    /// The formals of `function` read as probe operands, in definition order:
    /// in its own body, or passed bare to a callee's probe formal.
    fn probe_formals(
        &mut self,
        function: &FunctionDef,
        active: &mut Vec<String>,
    ) -> Result<Arc<[ProbeFormal]>, OutputExpressionError> {
        if let Some(formals) = self.probe_formals.get(&function.name) {
            return Ok(Arc::clone(formals));
        }
        let body = self.body(function)?;
        active.push(function.name.clone());
        let formals = self.probe_formals_in(function, &body, active);
        active.pop();
        let mut formals = formals?;
        formals.sort_by_key(|read| function.args.iter().position(|arg| *arg == read.formal));
        let formals: Arc<[ProbeFormal]> = formals.into();
        self.probe_formals
            .insert(function.name.clone(), Arc::clone(&formals));
        Ok(formals)
    }

    fn probe_formals_in(
        &mut self,
        function: &FunctionDef,
        body: &Expr,
        active: &mut Vec<String>,
    ) -> Result<Vec<ProbeFormal>, OutputExpressionError> {
        let mut formals: Vec<ProbeFormal> = Vec::new();
        let mut pending = vec![body];
        while let Some(node) = pending.pop() {
            match node {
                Expr::FnCall { name, args } if is_circuit_probe(name) => {
                    for arg in args {
                        if let Expr::Param(operand) = arg
                            && function.args.contains(operand)
                            && !formals.iter().any(|known| known.formal == *operand)
                        {
                            formals.push(ProbeFormal {
                                formal: operand.clone(),
                                function: function.name.clone(),
                                probe: name.to_ascii_uppercase(),
                                written: operand.clone(),
                            });
                        }
                    }
                }
                Expr::FnCall { name, args } => {
                    if let Some(callee) = self.params.get_function(name).cloned()
                        && callee.args.len() == args.len()
                        && !active.contains(&callee.name)
                    {
                        for read in self.probe_formals(&callee, active)?.iter() {
                            let position = callee.args.iter().position(|arg| *arg == read.formal);
                            if let Some(Expr::Param(operand)) = position.map(|at| &args[at])
                                && function.args.contains(operand)
                                && !formals.iter().any(|known| known.formal == *operand)
                            {
                                formals.push(ProbeFormal {
                                    formal: operand.clone(),
                                    function: read.function.clone(),
                                    probe: read.probe.clone(),
                                    written: read.written.clone(),
                                });
                            }
                        }
                    }
                    pending.extend(args.iter().rev());
                }
                Expr::UnaryOp { operand, .. } => pending.push(operand),
                Expr::BinOp { left, right, .. } => {
                    pending.push(right);
                    pending.push(left);
                }
                Expr::Number(_)
                | Expr::ComplexNumber(_)
                | Expr::StringLiteral(_)
                | Expr::Param(_) => {}
            }
        }
        Ok(formals)
    }

    /// The node or device a probe formal reads at this call: the name the
    /// argument spells, its parameter's integral value, or the node the
    /// caller's own probe formal it passes on is bound to. An argument that
    /// names nothing, such as `1+2`, is refused, as writing the call out
    /// refuses it.
    fn bind_probe_operand(
        &self,
        read: &ProbeFormal,
        actual: &Expr,
        caller: Caller,
    ) -> Result<String, OutputExpressionError> {
        let named = match actual {
            Expr::Param(name) => match caller {
                Caller::Instance(index) if self.instances[index].formals().contains(name) => {
                    self.instances[index].probe_binding(name).map(str::to_owned)
                }
                _ => match self.params.get(name) {
                    Some(value) => integral_node_name(value),
                    None => is_simple_probe_reference(name).then(|| name.clone()),
                },
            },
            Expr::Number(value) => integral_node_name(*value),
            _ => None,
        };
        named.ok_or_else(|| {
            OutputExpressionError::Semantic(format!(
                ".FUNC {function} reads its argument '{formal}' as a node or device name in \
                 {probe}({formal}), but the call passes '{actual}', which names neither",
                function = read.function,
                formal = read.written,
                probe = read.probe,
                actual = serialize_expr(actual),
            ))
        })
    }
}

/// The calls to defined functions in `tree`, in reading order.
fn calls_in<'t>(tree: &'t Expr, params: &ParamContext) -> Vec<(&'t str, &'t [Expr])> {
    let mut calls = Vec::new();
    let mut pending = vec![tree];
    while let Some(node) = pending.pop() {
        match node {
            Expr::FnCall { name, args } => {
                if params.has_function(name) {
                    calls.push((name.as_str(), args.as_slice()));
                }
                pending.extend(args.iter().rev());
            }
            Expr::UnaryOp { operand, .. } => pending.push(operand),
            Expr::BinOp { left, right, .. } => {
                pending.push(right);
                pending.push(left);
            }
            Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) | Expr::Param(_) => {}
        }
    }
    calls
}

/// `tree` with each call to a defined function renamed to the next of
/// `renames` that is set, visiting calls in [`calls_in`]'s order.
fn rename_calls(
    tree: Expr,
    params: &ParamContext,
    renames: &mut impl Iterator<Item = Option<String>>,
) -> Expr {
    match tree {
        Expr::FnCall { name, args } => {
            let rename = if params.has_function(&name) {
                renames.next().flatten()
            } else {
                None
            };
            Expr::FnCall {
                name: rename.unwrap_or(name),
                args: args
                    .into_iter()
                    .map(|arg| rename_calls(arg, params, renames))
                    .collect(),
            }
        }
        Expr::UnaryOp { op, operand } => Expr::UnaryOp {
            op,
            operand: Box::new(rename_calls(*operand, params, renames)),
        },
        Expr::BinOp { op, left, right } => {
            let left = Box::new(rename_calls(*left, params, renames));
            Expr::BinOp {
                op,
                left,
                right: Box::new(rename_calls(*right, params, renames)),
            }
        }
        leaf => leaf,
    }
}

/// `body` with each probe operand that is a bound formal replaced by the
/// node it names.
fn bind_probe_operands(body: Expr, bindings: &[(String, String)]) -> Expr {
    match body {
        Expr::FnCall { name, args } if is_circuit_probe(&name) => Expr::FnCall {
            name,
            args: args
                .into_iter()
                .map(|arg| match arg {
                    Expr::Param(operand) => Expr::Param(
                        bindings
                            .iter()
                            .find(|(formal, _)| *formal == operand)
                            .map_or(operand, |(_, node)| node.clone()),
                    ),
                    other => other,
                })
                .collect(),
        },
        Expr::FnCall { name, args } => Expr::FnCall {
            name,
            args: args
                .into_iter()
                .map(|arg| bind_probe_operands(arg, bindings))
                .collect(),
        },
        Expr::UnaryOp { op, operand } => Expr::UnaryOp {
            op,
            operand: Box::new(bind_probe_operands(*operand, bindings)),
        },
        Expr::BinOp { op, left, right } => Expr::BinOp {
            op,
            left: Box::new(bind_probe_operands(*left, bindings)),
            right: Box::new(bind_probe_operands(*right, bindings)),
        },
        leaf => leaf,
    }
}
