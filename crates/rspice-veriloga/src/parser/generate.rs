//! Generate regions: the grammar of IEEE 1364-2005 section 12.4, and the
//! elaboration-time unrolling that removes it again.
//!
//! # Why the unrolling is here
//!
//! Section 12.4 makes a generate region an *elaboration-time* construct. It
//! contributes no run-time behaviour of its own: what it contributes is module
//! items, decided once, from constants. So the honest place for it is a pass
//! that turns a module with generate regions into the module the author would
//! have had to write by hand, and everything after that pass — semantic
//! analysis, digital elaboration, lowering, the event kernel — sees only the
//! flat result and needs no arm for a generate anything.
//!
//! It runs at `endmodule` rather than in a later phase because the values it
//! needs are the module's own: section 12.2 fixes a parameter at elaboration,
//! and with a parameter override on a digital instance refused, "the value at
//! elaboration" is the declared default. A region whose bounds need anything
//! else — a name from another module, a value only semantic analysis can
//! compute — is refused, by clause, rather than unrolled with a guess.
//!
//! # What identity the unrolled items get
//!
//! Section 12.4.2 gives a generate block a hierarchical name, and a loop's
//! iterations are distinguished by the genvar's value: the instance `stage`
//! inside `begin : bit_slice` at `i == 3` is `bit_slice[3].stage`. That is the
//! name this pass writes onto the instance, so the digital elaborator's own
//! path qualification produces `bit_slice[3].stage.a` for its ports, and two
//! iterations are two instances a scheduler and a resolver can tell apart —
//! the same identity discipline the hierarchy flattening already keeps.
//!
//! A process gets a fresh [`DigitalProcessId`](crate::ast::DigitalProcessId)
//! per iteration, for the same reason: two copies of one `always` block are two
//! things to resume, not one resumed twice.
//!
//! A continuous assignment gets no name at all, because it never had one. It
//! is a driver on whatever net its target names, and that net is outside the
//! block — see the refusal list below.
//!
//! # What is refused, and why the list is short
//!
//! A generate block may not *declare* anything here: no net, no variable, no
//! parameter, no function, no analog block. Section 12.4.2 gives such a
//! declaration a hierarchical name of its own (`bit_slice[3].carry`), which
//! means every reference to it inside the block has to be rewritten to that
//! name and every reference from outside has to be able to reach it. That is a
//! scoping mechanism, not a copying one, and this pass copies. Refusing it by
//! name leaves an author with a diagnostic naming the clause; unrolling it
//! without the renaming would silently merge N iterations' `carry` into one
//! net.
//!
//! Everything else a module item can be — an instance, a continuous
//! assignment, a gate primitive (which is already a continuous assignment by
//! the time it gets here), a process — copies faithfully, because all it
//! references is the enclosing scope plus the genvar.

use super::Parser;
use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::TokenKind;
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::HashMap;

/// Ceiling on the iterations one generate loop may produce.
///
/// A loop whose update never reaches its bound describes an infinite design,
/// and the only thing to do with one is say so. Chosen large enough that no
/// hand-written design reaches it and small enough that hitting it is a
/// diagnostic rather than an out-of-memory.
const MAX_GENERATE_ITERATIONS: usize = 65_536;

impl Parser<'_> {
    // ------------------------------------------------------------------
    // Grammar
    // ------------------------------------------------------------------

    /// `genvar name [, name]* ;`
    pub(super) fn parse_genvar_decl(&mut self) -> Result<GenvarDecl, ParseError> {
        let start = self.current_span();
        self.expect(TokenKind::Genvar)?;
        let mut names = Vec::new();
        loop {
            names.push(SmolStr::from(self.expect_identifier("genvar name")?));
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::Semicolon)?;
        Ok(GenvarDecl {
            names,
            span: start.extend(self.previous_span()),
        })
    }

    /// `generate <generate item>* endgenerate`
    pub(super) fn parse_generate_region(&mut self) -> Result<Vec<GenerateConstruct>, ParseError> {
        self.expect(TokenKind::Generate)?;
        let mut constructs = Vec::new();
        while !self.check(TokenKind::Endgenerate) && !self.at_end() {
            constructs.push(self.parse_generate_construct()?);
        }
        self.expect(TokenKind::Endgenerate)?;
        Ok(constructs)
    }

    /// One generate item.
    ///
    /// `for`, `if`, and `case` open the three conditional-generate forms of
    /// sections 12.4.1 and 12.4.2. Nothing else can: no module item begins with
    /// one of those keywords, so the dispatch needs no lookahead.
    fn parse_generate_construct(&mut self) -> Result<GenerateConstruct, ParseError> {
        match self.current().kind {
            TokenKind::For => self.parse_generate_loop(),
            TokenKind::If => self.parse_generate_conditional(),
            TokenKind::Case => self.parse_generate_case(),
            _ => Ok(GenerateConstruct::Block(self.parse_generate_block()?)),
        }
    }

    fn parse_generate_loop(&mut self) -> Result<GenerateConstruct, ParseError> {
        let start = self.current_span();
        self.advance(); // `for`
        self.expect(TokenKind::LParen)?;
        let genvar: SmolStr = self.expect_identifier("generate loop variable")?.into();
        self.expect(TokenKind::Assign_)?;
        let init = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::Semicolon)?;

        // The update is an assignment to the same genvar, per section 12.4.1.
        // A header that advances some other name would silently loop forever.
        let update_start = self.current_span();
        let updated: SmolStr = self.expect_identifier("generate loop variable")?.into();
        if updated != genvar {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "generate for".to_string(),
                    found: format!(
                        "an update of `{updated}`; IEEE 1364-2005 section 12.4.1 advances the \
                         loop's own genvar `{genvar}`"
                    ),
                },
                update_start,
            ));
        }
        self.expect(TokenKind::Assign_)?;
        let update = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;

        let body = self.parse_generate_block()?;
        Ok(GenerateConstruct::Loop(Box::new(GenerateLoop {
            genvar,
            init,
            condition,
            update,
            body,
            span: start.extend(self.previous_span()),
        })))
    }

    fn parse_generate_conditional(&mut self) -> Result<GenerateConstruct, ParseError> {
        let start = self.current_span();
        self.advance(); // `if`
        self.expect(TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;
        let then_block = self.parse_generate_block()?;
        let else_block = if self.match_token(TokenKind::Else) {
            Some(self.parse_generate_block()?)
        } else {
            None
        };
        Ok(GenerateConstruct::Conditional(Box::new(
            GenerateConditional {
                condition,
                then_block,
                else_block,
                span: start.extend(self.previous_span()),
            },
        )))
    }

    fn parse_generate_case(&mut self) -> Result<GenerateConstruct, ParseError> {
        let start = self.current_span();
        self.advance(); // `case`
        self.expect(TokenKind::LParen)?;
        let selector = self.parse_expression()?;
        self.expect(TokenKind::RParen)?;

        let mut items = Vec::new();
        let mut default: Option<GenerateBlock> = None;
        while !self.check(TokenKind::Endcase) && !self.at_end() {
            if self.check(TokenKind::Default) {
                let default_span = self.current_span();
                self.advance();
                self.match_token(TokenKind::Colon);
                if default.is_some() {
                    return Err(ParseError::new(
                        ParseErrorKind::UnsupportedConstruct {
                            context: "generate case".to_string(),
                            found: "a second `default` arm; IEEE 1364-2005 section 12.4.2 \
                                    permits at most one"
                                .to_string(),
                        },
                        default_span,
                    ));
                }
                default = Some(self.parse_generate_block()?);
                continue;
            }

            let item_start = self.current_span();
            let mut labels = Vec::new();
            loop {
                labels.push(self.parse_expression()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::Colon)?;
            let block = self.parse_generate_block()?;
            items.push(GenerateCaseItem {
                labels,
                block,
                span: item_start.extend(self.previous_span()),
            });
        }
        self.expect(TokenKind::Endcase)?;
        Ok(GenerateConstruct::Case(Box::new(GenerateCase {
            selector,
            items,
            default,
            span: start.extend(self.previous_span()),
        })))
    }

    /// `begin [: name] <generate item>* end`, or one bare generate item.
    fn parse_generate_block(&mut self) -> Result<GenerateBlock, ParseError> {
        let start = self.current_span();
        let mut block = GenerateBlock {
            name: None,
            items: Box::new(Module::new("", start)),
            nested: Vec::new(),
            span: start,
        };

        if !self.match_token(TokenKind::Begin) {
            // A single unblocked item: `if (WIDTH > 1) buf u (y, a);`
            self.parse_generate_block_item(&mut block)?;
            block.span = start.extend(self.previous_span());
            return Ok(block);
        }

        if self.match_token(TokenKind::Colon) {
            block.name = Some(self.expect_identifier("generate block name")?.into());
        }
        while !self.check(TokenKind::End) && !self.at_end() {
            self.parse_generate_block_item(&mut block)?;
        }
        self.expect(TokenKind::End)?;
        block.span = start.extend(self.previous_span());
        Ok(block)
    }

    /// One item inside a generate block: a nested construct, or a module item.
    fn parse_generate_block_item(&mut self, block: &mut GenerateBlock) -> Result<(), ParseError> {
        match self.current().kind {
            TokenKind::For | TokenKind::If | TokenKind::Case => {
                block.nested.push(self.parse_generate_construct()?);
            }
            // A nested `generate ... endgenerate` is not legal inside a region
            // (section 12.4), and `genvar` belongs to the module rather than to
            // a block. Both are refused where they are written.
            TokenKind::Generate | TokenKind::Endgenerate | TokenKind::Genvar => {
                return Err(ParseError::new(
                    ParseErrorKind::UnsupportedConstruct {
                        context: "generate region".to_string(),
                        found: format!(
                            "`{}`; IEEE 1364-2005 section 12.4 nests generate constructs \
                             directly, without reopening a region, and declares a `genvar` at \
                             module level",
                            self.current()
                                .text
                                .clone()
                                .unwrap_or_else(|| "generate".to_string())
                        ),
                    },
                    self.current_span(),
                ));
            }
            _ => {
                let mut items =
                    std::mem::replace(&mut block.items, Box::new(Module::new("", block.span)));
                let outcome = self.parse_module_item(&mut items);
                block.items = items;
                outcome?;
            }
        }
        Ok(())
    }
}

// ============================================================================
// Unrolling
// ============================================================================

/// Unroll every generate region of `module` into its item lists.
///
/// `next_process_id` is the parser's own counter, so a process copied out of a
/// loop body gets an identity no source-level process was given.
pub(super) fn expand(module: &mut Module, next_process_id: &mut u32) -> Result<(), ParseError> {
    if module.generates.is_empty() {
        return Ok(());
    }
    let constructs = std::mem::take(&mut module.generates);
    let genvars: Vec<SmolStr> = module
        .genvars
        .iter()
        .flat_map(|declaration| declaration.names.iter().cloned())
        .collect();
    let constants = module_constants(module);

    let mut unroller = Unroller {
        genvars,
        constants,
        bindings: HashMap::new(),
        next_process_id,
    };
    let mut expanded = Module::new(module.name.clone(), module.span);
    for construct in &constructs {
        unroller.construct(construct, "", &mut expanded)?;
    }
    absorb(module, expanded);
    Ok(())
}

/// The module's own integer parameters and localparams, by name.
///
/// Section 12.2 fixes these at elaboration and a generate region is elaborated
/// against them. Folded in declaration order so a `localparam` may be written
/// in terms of a `parameter` above it, which is how a width is normally
/// derived.
///
/// A default this small evaluator cannot fold is simply absent, and a region
/// that needed it is refused rather than unrolled — see [`Unroller::value`].
fn module_constants(module: &Module) -> HashMap<SmolStr, i64> {
    let mut constants = HashMap::new();
    for declaration in module.parameters.iter().chain(&module.localparams) {
        let Some(default) = &declaration.default else {
            continue;
        };
        if let Some(value) = constant_value(default, &constants) {
            constants.insert(declaration.name.clone(), value);
        }
    }
    constants
}

/// Move every item of `expanded` onto `module`.
///
/// Appended rather than interleaved at the region's source position. Nothing
/// downstream depends on a module item's position relative to a region — a
/// declaration is collected before anything resolves against it, and process
/// execution order within a time slot is undefined by section 11 — and
/// appending keeps the order deterministic without a second index to maintain.
fn absorb(module: &mut Module, expanded: Module) {
    let Module {
        instances,
        continuous_assigns,
        digital_processes,
        ..
    } = expanded;
    module.instances.extend(instances);
    module.continuous_assigns.extend(continuous_assigns);
    module.digital_processes.extend(digital_processes);
}

struct Unroller<'a> {
    /// Names declared `genvar`, so a loop over anything else is refused.
    genvars: Vec<SmolStr>,
    constants: HashMap<SmolStr, i64>,
    /// The genvars currently bound, innermost loop last.
    bindings: HashMap<SmolStr, i64>,
    next_process_id: &'a mut u32,
}

impl Unroller<'_> {
    fn construct(
        &mut self,
        construct: &GenerateConstruct,
        prefix: &str,
        out: &mut Module,
    ) -> Result<(), ParseError> {
        match construct {
            GenerateConstruct::Loop(loop_) => self.unroll_loop(loop_, prefix, out),
            GenerateConstruct::Conditional(conditional) => {
                let taken = if self.value(&conditional.condition, "generate if condition")? != 0 {
                    Some(&conditional.then_block)
                } else {
                    conditional.else_block.as_ref()
                };
                match taken {
                    Some(block) => self.block(block, prefix, out),
                    None => Ok(()),
                }
            }
            GenerateConstruct::Case(case) => {
                let selector = self.value(&case.selector, "generate case selector")?;
                for item in &case.items {
                    for label in &item.labels {
                        if self.value(label, "generate case item")? == selector {
                            return self.block(&item.block, prefix, out);
                        }
                    }
                }
                match &case.default {
                    Some(block) => self.block(block, prefix, out),
                    None => Ok(()),
                }
            }
            GenerateConstruct::Block(block) => self.block(block, prefix, out),
        }
    }

    fn unroll_loop(
        &mut self,
        loop_: &GenerateLoop,
        prefix: &str,
        out: &mut Module,
    ) -> Result<(), ParseError> {
        if !self.genvars.contains(&loop_.genvar) {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "generate for".to_string(),
                    found: format!(
                        "a loop over `{}`, which is not declared `genvar`; IEEE 1364-2005 \
                         section 12.1.3.2 makes the index of a generate loop a genvar",
                        loop_.genvar
                    ),
                },
                loop_.span,
            ));
        }
        // Section 12.4.1: the loop's generate block is named, and the name plus
        // the index is what distinguishes one iteration's items from the next's.
        // Without it two iterations would produce two instances of one name.
        let Some(block_name) = &loop_.body.name else {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "generate for".to_string(),
                    found: "an unnamed generate block; IEEE 1364-2005 section 12.4.1 names it, \
                            and the name with the index is what tells one iteration's items \
                            from another's"
                        .to_string(),
                },
                loop_.body.span,
            ));
        };

        // A genvar already bound is an outer loop using the same name, which
        // would make the inner binding shadow it and every reference ambiguous.
        if self.bindings.contains_key(&loop_.genvar) {
            return Err(ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: "generate for".to_string(),
                    found: format!(
                        "a nested loop reusing the genvar `{}`; give the inner loop its own, \
                         so every reference names one index",
                        loop_.genvar
                    ),
                },
                loop_.span,
            ));
        }

        let mut index = self.value(&loop_.init, "generate for initial value")?;
        let mut iterations = 0usize;
        loop {
            self.bindings.insert(loop_.genvar.clone(), index);
            let keep_going = self.value(&loop_.condition, "generate for condition")? != 0;
            if !keep_going {
                self.bindings.remove(&loop_.genvar);
                return Ok(());
            }
            iterations += 1;
            if iterations > MAX_GENERATE_ITERATIONS {
                self.bindings.remove(&loop_.genvar);
                return Err(ParseError::new(
                    ParseErrorKind::UnsupportedConstruct {
                        context: "generate for".to_string(),
                        found: format!(
                            "more than {MAX_GENERATE_ITERATIONS} iterations; a loop whose \
                             update never reaches its bound describes an infinite design"
                        ),
                    },
                    loop_.span,
                ));
            }

            let iteration_prefix = format!("{prefix}{block_name}[{index}].");
            // The body's own name is already in the prefix, so the block is
            // expanded as if unnamed: naming it twice would produce
            // `bit_slice[3].bit_slice.stage`.
            self.block_contents(&loop_.body, &iteration_prefix, out)?;

            index = self.value(&loop_.update, "generate for update")?;
            self.bindings.remove(&loop_.genvar);
        }
    }

    /// Expand one block, qualifying its contents by the block's own name.
    fn block(
        &mut self,
        block: &GenerateBlock,
        prefix: &str,
        out: &mut Module,
    ) -> Result<(), ParseError> {
        match &block.name {
            Some(name) => self.block_contents(block, &format!("{prefix}{name}."), out),
            None => self.block_contents(block, prefix, out),
        }
    }

    fn block_contents(
        &mut self,
        block: &GenerateBlock,
        prefix: &str,
        out: &mut Module,
    ) -> Result<(), ParseError> {
        reject_declarations(&block.items)?;

        for instance in &block.items.instances {
            let mut copy = instance.clone();
            copy.name = SmolStr::from(format!("{prefix}{}", instance.name));
            for connection in &mut copy.connections {
                match connection {
                    Connection::Ordered { signal, .. } | Connection::Named { signal, .. } => {
                        if let Some(expression) = signal {
                            self.substitute(expression);
                        }
                    }
                }
            }
            for override_ in &mut copy.parameters {
                self.substitute(&mut override_.value);
            }
            out.instances.push(copy);
        }

        for assignment in &block.items.continuous_assigns {
            let mut copy = assignment.clone();
            self.substitute_lvalue(&mut copy.target);
            self.substitute(&mut copy.value);
            if let Some(delay) = &mut copy.delay {
                self.substitute(delay);
            }
            out.continuous_assigns.push(copy);
        }

        for process in &block.items.digital_processes {
            let mut copy = process.clone();
            copy.id = DigitalProcessId(*self.next_process_id);
            *self.next_process_id += 1;
            self.substitute_statement(&mut copy.body);
            out.digital_processes.push(copy);
        }

        for nested in &block.nested {
            self.construct(nested, prefix, out)?;
        }
        Ok(())
    }

    /// The constant value of an elaboration-time expression.
    fn value(&self, expression: &Expression, context: &str) -> Result<i64, ParseError> {
        let mut environment = self.constants.clone();
        environment.extend(self.bindings.iter().map(|(k, v)| (k.clone(), *v)));
        constant_value(expression, &environment).ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::UnsupportedConstruct {
                    context: context.to_string(),
                    found: "an expression this compiler cannot fold to a constant; IEEE \
                            1364-2005 section 12.4 elaborates a generate region from constants, \
                            so its bounds and conditions may name only literals, this module's \
                            own parameters and localparams, and an enclosing genvar"
                        .to_string(),
                },
                expression.span(),
            )
        })
    }

    /// Replace every bound genvar in `expression` with its value.
    fn substitute(&self, expression: &mut Expression) {
        match expression {
            Expression::Identifier(identifier) => {
                if let Some(value) = self.bindings.get(&identifier.name) {
                    *expression = Expression::Number(NumberLit {
                        value: *value as f64,
                        raw: SmolStr::from(value.to_string()),
                        span: identifier.span,
                    });
                }
            }
            Expression::Binary(binary) => {
                self.substitute(&mut binary.left);
                self.substitute(&mut binary.right);
            }
            Expression::Unary(unary) => self.substitute(&mut unary.operand),
            Expression::Conditional(conditional) => {
                self.substitute(&mut conditional.condition);
                self.substitute(&mut conditional.then_expr);
                self.substitute(&mut conditional.else_expr);
            }
            Expression::Call(call) => {
                for argument in &mut call.args {
                    self.substitute(argument);
                }
            }
            Expression::SystemFunction(function) => {
                for argument in &mut function.args {
                    self.substitute(argument);
                }
            }
            Expression::ArrayAccess(access) => self.substitute(&mut access.index),
            Expression::ArrayLiteral(literal) => {
                for element in &mut literal.elements {
                    self.substitute_concat_element(element);
                }
            }
            Expression::Digital(digital) => self.substitute_digital(digital),
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => {}
        }
    }

    fn substitute_digital(&self, digital: &mut DigitalExpr) {
        match digital {
            DigitalExpr::FourState(_) => {}
            DigitalExpr::PartSelect(select) => {
                self.substitute(&mut select.msb);
                self.substitute(&mut select.lsb);
            }
            DigitalExpr::Xnor(xnor) => {
                self.substitute(&mut xnor.left);
                self.substitute(&mut xnor.right);
            }
            DigitalExpr::CaseEquality(equality) => {
                self.substitute(&mut equality.left);
                self.substitute(&mut equality.right);
            }
            DigitalExpr::Reduction(reduction) => self.substitute(&mut reduction.operand),
            DigitalExpr::ArithmeticShiftRight(shift) => {
                self.substitute(&mut shift.left);
                self.substitute(&mut shift.right);
            }
        }
    }

    fn substitute_concat_element(&self, element: &mut ArrayLiteralElement) {
        match element {
            ArrayLiteralElement::Value(value) => self.substitute(value),
            ArrayLiteralElement::Replication(replication) => {
                self.substitute(&mut replication.count);
                for inner in &mut replication.elements {
                    self.substitute_concat_element(inner);
                }
            }
        }
    }

    fn substitute_lvalue(&self, target: &mut DigitalLValue) {
        match target {
            DigitalLValue::Identifier { .. } => {}
            DigitalLValue::BitSelect { index, .. } => self.substitute(index),
            DigitalLValue::PartSelect { msb, lsb, .. } => {
                self.substitute(msb);
                self.substitute(lsb);
            }
            DigitalLValue::Concat { elements, .. } => {
                for element in elements {
                    self.substitute_lvalue(element);
                }
            }
        }
    }

    fn substitute_statement(&self, statement: &mut DigitalStatement) {
        match statement {
            DigitalStatement::Null(_) => {}
            DigitalStatement::Block(block) => {
                for declaration in &mut block.variables {
                    for item in &mut declaration.items {
                        if let Some(init) = &mut item.init {
                            self.substitute(init);
                        }
                    }
                }
                for declaration in &mut block.digital_variables {
                    for item in &mut declaration.items {
                        if let Some(init) = &mut item.init {
                            self.substitute(init);
                        }
                    }
                }
                for inner in &mut block.statements {
                    self.substitute_statement(inner);
                }
            }
            DigitalStatement::BlockingAssign(assign)
            | DigitalStatement::NonblockingAssign(assign) => self.substitute_assign(assign),
            DigitalStatement::Conditional(conditional) => {
                self.substitute(&mut conditional.condition);
                self.substitute_statement(&mut conditional.then_branch);
                if let Some(branch) = &mut conditional.else_branch {
                    self.substitute_statement(branch);
                }
            }
            DigitalStatement::Case(case) => {
                self.substitute(&mut case.selector);
                for item in &mut case.items {
                    for label in &mut item.labels {
                        self.substitute(label);
                    }
                    self.substitute_statement(&mut item.statement);
                }
                if let Some(default) = &mut case.default {
                    self.substitute_statement(default);
                }
            }
            DigitalStatement::For(statement) => {
                self.substitute_assign(&mut statement.init);
                self.substitute(&mut statement.condition);
                self.substitute_assign(&mut statement.update);
                self.substitute_statement(&mut statement.body);
            }
            DigitalStatement::While(statement) => {
                self.substitute(&mut statement.condition);
                self.substitute_statement(&mut statement.body);
            }
            DigitalStatement::Repeat(statement) => {
                self.substitute(&mut statement.count);
                self.substitute_statement(&mut statement.body);
            }
            DigitalStatement::Forever(statement) => {
                self.substitute_statement(&mut statement.body);
            }
            DigitalStatement::Timing(timing) => {
                self.substitute_timing(&mut timing.control);
                if let Some(inner) = &mut timing.statement {
                    self.substitute_statement(inner);
                }
            }
        }
    }

    fn substitute_assign(&self, assign: &mut DigitalAssign) {
        self.substitute_lvalue(&mut assign.target);
        if let Some(timing) = &mut assign.timing {
            self.substitute_timing(timing);
        }
        self.substitute(&mut assign.value);
    }

    fn substitute_timing(&self, control: &mut TimingControl) {
        match control {
            TimingControl::Delay(delay) => self.substitute(&mut delay.value),
            TimingControl::Event(event) => {
                if let Sensitivity::Explicit(terms) = &mut event.sensitivity {
                    for term in terms {
                        self.substitute(&mut term.signal);
                    }
                }
            }
        }
    }
}

/// Refuse the module items a generate block may not contain.
///
/// Every one of these declares a name that section 12.4.2 would give a
/// hierarchical identity — `bit_slice[3].carry` — which is a scoping rule
/// rather than a copying one. Copying the declaration without the renaming
/// would merge every iteration's copy into one, so it stops here by name.
fn reject_declarations(items: &Module) -> Result<(), ParseError> {
    let refusal = |what: &str, clause: &str, span: Span| {
        Err(ParseError::new(
            ParseErrorKind::UnsupportedConstruct {
                context: "generate block".to_string(),
                found: format!(
                    "{what}; IEEE 1364-2005 section {clause} gives a name declared inside a \
                     generate block a hierarchical name of its own, which this compiler does \
                     not synthesize — declare it outside the region instead"
                ),
            },
            span,
        ))
    };

    if let Some(declaration) = items.digital_nets.first() {
        return refusal("a `wire` declaration", "12.4.2", declaration.span);
    }
    if let Some(declaration) = items.digital_variables.first() {
        return refusal("a `reg` declaration", "12.4.2", declaration.span);
    }
    if let Some(declaration) = items.variables.first() {
        return refusal("a variable declaration", "12.4.2", declaration.span);
    }
    if let Some(declaration) = items.nets.first() {
        return refusal("a net declaration", "12.4.2", declaration.span);
    }
    if let Some(declaration) = items.parameters.first().or(items.localparams.first()) {
        return refusal("a parameter declaration", "12.4.2", declaration.span);
    }
    if let Some(function) = items.functions.first() {
        return refusal("a function definition", "12.4.2", function.span);
    }
    if let Some(block) = items
        .analog_block
        .as_ref()
        .or(items.analog_initial.as_ref())
        .or(items.analog_final.as_ref())
    {
        return Err(ParseError::new(
            ParseErrorKind::UnsupportedConstruct {
                context: "generate block".to_string(),
                found: "an analog block; a generate region is elaborated into discrete-domain \
                        module items, and mixed-signal elaboration is not implemented"
                    .to_string(),
            },
            block.span,
        ));
    }
    Ok(())
}

/// Fold one elaboration-time expression against a constant environment.
///
/// Deliberately small. What a generate bound is allowed to be is a *constant
/// expression* of section 12.4, and everything it may legally contain is
/// either a literal, a name the environment knows, or an operator over those.
/// Anything outside that returns `None`, which becomes a refusal naming the
/// clause rather than a value nothing derived.
fn constant_value(expression: &Expression, environment: &HashMap<SmolStr, i64>) -> Option<i64> {
    match expression {
        Expression::Number(number) if number.value.fract() == 0.0 && number.value.is_finite() => {
            Some(number.value as i64)
        }
        Expression::Identifier(identifier) => environment.get(&identifier.name).copied(),
        Expression::Unary(unary) => {
            let operand = constant_value(&unary.operand, environment)?;
            Some(match unary.op {
                UnaryOp::Neg => operand.checked_neg()?,
                UnaryOp::Pos => operand,
                UnaryOp::Not => i64::from(operand == 0),
                UnaryOp::BitNot => !operand,
            })
        }
        Expression::Binary(binary) => {
            let left = constant_value(&binary.left, environment)?;
            let right = constant_value(&binary.right, environment)?;
            let boolean = |flag: bool| Some(i64::from(flag));
            match binary.op {
                BinaryOp::Add => left.checked_add(right),
                BinaryOp::Sub => left.checked_sub(right),
                BinaryOp::Mul => left.checked_mul(right),
                BinaryOp::Div => left.checked_div(right),
                BinaryOp::Mod => left.checked_rem(right),
                BinaryOp::Pow => u32::try_from(right).ok().and_then(|e| left.checked_pow(e)),
                BinaryOp::Eq => boolean(left == right),
                BinaryOp::Ne => boolean(left != right),
                BinaryOp::Lt => boolean(left < right),
                BinaryOp::Le => boolean(left <= right),
                BinaryOp::Gt => boolean(left > right),
                BinaryOp::Ge => boolean(left >= right),
                BinaryOp::And => boolean(left != 0 && right != 0),
                BinaryOp::Or => boolean(left != 0 || right != 0),
                BinaryOp::BitAnd => Some(left & right),
                BinaryOp::BitOr => Some(left | right),
                BinaryOp::BitXor => Some(left ^ right),
                BinaryOp::Shl => u32::try_from(right).ok().map(|shift| left << shift),
                BinaryOp::Shr => u32::try_from(right).ok().map(|shift| left >> shift),
            }
        }
        Expression::Conditional(conditional) => {
            let condition = constant_value(&conditional.condition, environment)?;
            if condition != 0 {
                constant_value(&conditional.then_expr, environment)
            } else {
                constant_value(&conditional.else_expr, environment)
            }
        }
        Expression::Digital(DigitalExpr::Xnor(xnor)) => {
            let left = constant_value(&xnor.left, environment)?;
            let right = constant_value(&xnor.right, environment)?;
            Some(!(left ^ right))
        }
        _ => None,
    }
}
