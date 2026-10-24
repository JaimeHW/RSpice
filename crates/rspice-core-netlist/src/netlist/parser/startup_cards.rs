//! Ordered startup cards with source-order capture and deferred lexical binding.
use super::card_binding::CardBinding;
use super::scopes::LexicalScopes;
use super::*;

#[derive(Debug, Default)]
pub(super) struct StartupCardPlan {
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    card: Card,
    line: usize,
    origin: NetlistSourceLocation,
    after_provisional_error: bool,
}

#[derive(Debug)]
enum Card {
    Ready(Box<ParsedCard>),
    Pending(Box<PendingCard>),
}

#[derive(Debug)]
struct PendingCard {
    head: Head,
    binding: CardBinding,
    scope: StartupDirectiveScope,
    defer_values: bool,
}

#[derive(Debug, Clone, Copy)]
enum Head {
    Voltage(StartupDirectiveKind),
    Device,
}

#[derive(Debug)]
enum ParsedCard {
    Voltage(StartupDirectiveRecord),
    Device(DeviceInitialConditionDirective),
}

#[derive(Clone, Copy)]
pub(super) struct StartupCardContext<'a> {
    pub(super) line: usize,
    pub(super) params: &'a ParamContext,
    pub(super) origin: &'a NetlistSourceLocation,
    pub(super) scope: &'a StartupDirectiveScope,
    pub(super) defer_values: bool,
    pub(super) after_provisional_error: bool,
}

pub(super) struct StartupCardSink<'a> {
    pub(super) initial_conditions: &'a mut Vec<InitialCondition>,
    pub(super) node_sets: &'a mut Vec<NodeSet>,
    pub(super) records: &'a mut Vec<StartupDirectiveRecord>,
    pub(super) device: &'a mut Option<DeviceInitialConditionDirective>,
}

impl StartupCardPlan {
    pub(super) fn parse(
        &mut self,
        scopes: &mut LexicalScopes,
        command: &str,
        stream: &mut TokenStream,
        context: StartupCardContext<'_>,
        sink: StartupCardSink<'_>,
    ) -> Result<bool, ParseError> {
        let head = match command {
            ".IC" => Head::Voltage(StartupDirectiveKind::Ic),
            ".NODESET" => Head::Voltage(StartupDirectiveKind::NodeSet),
            ".INITCOND" => Head::Device,
            _ => return Ok(false),
        };
        let queued = !self.entries.is_empty();
        let mut probe = stream.clone();
        let parsed = (|| {
            validate_device_owner(
                head,
                context.origin,
                sink.device.as_ref().map(|value| &value.origin),
            )?;
            // Scoped voltage expressions retain the per-instance binding
            // contract: an X-line can supply a value absent from the definition.
            let bind_numeric = !context.defer_values || matches!(head, Head::Device);
            // Failed cards must not consume any live draws.
            if card_values_may_sample(stream) {
                let isolated = context.params.isolated_random_clone();
                let mut isolated_stream = stream.clone();
                if bind_numeric {
                    isolated_stream.begin_numeric_binding();
                }
                ParsedCard::stage(
                    head,
                    &mut isolated_stream,
                    StartupCardContext {
                        params: &isolated,
                        ..context
                    },
                )?;
            }
            if bind_numeric {
                probe.begin_numeric_binding();
            }
            let card = ParsedCard::stage(head, &mut probe, context)?;
            Ok(card)
        })();
        let card = match parsed {
            Ok(card) if !queued => {
                *stream = probe;
                card.publish(sink);
                return Ok(true);
            }
            Ok(card) => Card::Ready(Box::new(card)),
            Err(error @ ParseError::ResourceLimit(_)) => return Err(error),
            Err(_) => Card::Pending(Box::new(PendingCard {
                head,
                binding: CardBinding::capture(scopes.retain(), stream.clone(), context.params),
                scope: context.scope.clone(),
                defer_values: context.defer_values,
            })),
        };
        self.entries.push(Entry {
            card,
            line: context.line,
            origin: context.origin.clone(),
            after_provisional_error: context.after_provisional_error,
        });
        stream.skip_to_eol();
        Ok(true)
    }

    pub(super) fn complete(
        self,
        state: &mut ParseState,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        let mut ready = Vec::with_capacity(self.entries.len());
        let mut device_origin = state
            .device_initial_conditions
            .as_ref()
            .map(|value| value.origin.clone());
        for entry in self.entries {
            ensure_parse_not_aborted(abort)?;
            let head = match &entry.card {
                Card::Ready(card) => match **card {
                    ParsedCard::Device(_) => Head::Device,
                    ParsedCard::Voltage(ref record) => Head::Voltage(record.kind),
                },
                Card::Pending(pending) => pending.head,
            };
            validate_device_owner(head, &entry.origin, device_origin.as_ref())?;
            let card = match entry.card {
                Card::Ready(card) => *card,
                Card::Pending(pending) => pending
                    .binding
                    .bind(
                        &mut state.scopes,
                        &state.params,
                        entry.line,
                        &entry.origin,
                        abort,
                        |stream, params| {
                            ParsedCard::stage(
                                pending.head,
                                stream,
                                StartupCardContext {
                                    line: entry.line,
                                    params,
                                    origin: &entry.origin,
                                    scope: &pending.scope,
                                    defer_values: pending.defer_values,
                                    after_provisional_error: entry.after_provisional_error,
                                },
                            )
                        },
                    )
                    .map_err(|error| binding_error(pending.head, &entry.origin, error))?,
            };
            if let ParsedCard::Device(device) = &card {
                device_origin = Some(device.origin.clone());
            }
            ready.push(card);
        }
        ensure_parse_not_aborted(abort)?;
        for card in ready {
            match card {
                ParsedCard::Voltage(record) => {
                    match &record.scope {
                        StartupDirectiveScope::TopLevel => {
                            super::super::startup::append_applied_entries(
                                &record,
                                &mut state.initial_conditions,
                                &mut state.node_sets,
                            )
                        }
                        StartupDirectiveScope::Subcircuit {
                            qualified_definition,
                            ..
                        } => {
                            append_scoped(&mut state.subcircuits, qualified_definition, &record);
                        }
                    }
                    state.startup_directives.push(record);
                }
                ParsedCard::Device(device) => state.device_initial_conditions = Some(device),
            }
        }
        Ok(())
    }

    pub(super) fn prefer_error(
        &self,
        state: &ParseState,
        error: ParseWithAbortError,
        abort: &dyn AbortSignal,
    ) -> ParseWithAbortError {
        if matches!(
            error,
            ParseWithAbortError::Aborted | ParseWithAbortError::Parse(ParseError::ResourceLimit(_))
        ) {
            return error;
        }
        let mut device_origin = state
            .device_initial_conditions
            .as_ref()
            .map(|value| &value.origin);
        for entry in self
            .entries
            .iter()
            .take_while(|entry| !entry.after_provisional_error)
        {
            if let Err(error) = ensure_parse_not_aborted(abort) {
                return error;
            }
            let head = match &entry.card {
                Card::Ready(card) => match **card {
                    ParsedCard::Device(_) => Head::Device,
                    ParsedCard::Voltage(ref record) => Head::Voltage(record.kind),
                },
                Card::Pending(pending) => pending.head,
            };
            if let Err(error) = validate_device_owner(head, &entry.origin, device_origin) {
                return error.into();
            }
            if let Card::Pending(pending) = &entry.card {
                let environment = match state.scopes.environment(
                    pending.binding.scope,
                    &state.params,
                    &state.subckt_stack,
                    abort,
                ) {
                    Ok(environment) => environment,
                    Err(error) => return error,
                };
                let probe = pending.binding.probe(
                    &environment,
                    entry.line,
                    &entry.origin,
                    abort,
                    |stream, params| {
                        ParsedCard::stage(
                            head,
                            stream,
                            StartupCardContext {
                                line: entry.line,
                                params,
                                origin: &entry.origin,
                                scope: &pending.scope,
                                defer_values: pending.defer_values,
                                after_provisional_error: entry.after_provisional_error,
                            },
                        )
                    },
                );
                if let Err(error) = probe {
                    return binding_error(head, &entry.origin, error);
                }
            }
            if matches!(head, Head::Device) {
                device_origin = Some(&entry.origin);
            }
        }
        error
    }
}

fn binding_error(
    head: Head,
    origin: &NetlistSourceLocation,
    error: ParseWithAbortError,
) -> ParseWithAbortError {
    match (head, error) {
        (Head::Device, ParseWithAbortError::Parse(ParseError::InvalidValue(detail))) => {
            ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedDirective {
                    origin: origin.clone(),
                    detail,
                },
            ))
            .into()
        }
        (_, error) => error,
    }
}

fn append_scoped(definitions: &mut [SubcircuitDef], name: &str, record: &StartupDirectiveRecord) {
    for definition in definitions {
        if definition.name.eq_ignore_ascii_case(name) {
            super::super::startup::append_applied_entries(
                record,
                &mut definition.initial_conditions,
                &mut definition.node_sets,
            );
        }
        append_scoped(&mut definition.nested_subcircuits, name, record);
    }
}

impl ParsedCard {
    fn stage(
        head: Head,
        stream: &mut TokenStream,
        context: StartupCardContext<'_>,
    ) -> Result<Self, ParseError> {
        match head {
            Head::Voltage(kind) => {
                let entries = parse_voltage_hint_command(
                    stream,
                    context.line,
                    context.params,
                    kind,
                    context.defer_values,
                )?;
                let disposition = if entries.is_empty() {
                    StartupDirectiveDisposition::Ignored(StartupDiagnosticCode::EmptyDirective)
                } else {
                    StartupDirectiveDisposition::Applied
                };
                Ok(Self::Voltage(StartupDirectiveRecord {
                    kind,
                    origin: context.origin.clone(),
                    scope: context.scope.clone(),
                    entries,
                    disposition,
                }))
            }
            Head::Device => {
                let mut device = None;
                parse_device_initial_condition_command(
                    stream,
                    context.line,
                    context.params,
                    context.origin,
                    &mut device,
                )?;
                Ok(Self::Device(
                    device.expect("successful device startup card"),
                ))
            }
        }
    }

    fn publish(self, sink: StartupCardSink<'_>) {
        match self {
            Self::Voltage(record) => {
                super::super::startup::append_applied_entries(
                    &record,
                    sink.initial_conditions,
                    sink.node_sets,
                );
                sink.records.push(record);
            }
            Self::Device(device) => *sink.device = Some(device),
        }
    }
}

fn validate_device_owner(
    head: Head,
    origin: &NetlistSourceLocation,
    first: Option<&NetlistSourceLocation>,
) -> Result<(), ParseError> {
    if let (Head::Device, Some(first)) = (head, first) {
        return Err(ParseError::DeviceInitialCondition(Box::new(
            DeviceInitialConditionError::DuplicateDirective {
                first: first.clone(),
                duplicate: origin.clone(),
            },
        )));
    }
    Ok(())
}
