use super::*;

impl XyceTestRunner {
    pub(super) fn bug38_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug38FamilyContract, String>> {
        let role = XyceBug38Role::for_record(&deck.relative_path)?;
        Some((|| {
            const LABEL: &str = "BUG_38_SON SUBCKT formal-parentheses family";
            if deck.section != XyceDeckSection::Netlists
                || Self::normalize_manifest_key(&self.relative_key(&deck.path))
                    != Self::normalize_manifest_key(&deck.relative_path)
            {
                return Err(format!(
                    "recognized {LABEL} record '{}' is not backed by its exact Netlists path",
                    deck.relative_path
                ));
            }
            let owner_path = self.root.join(XYCE_BUG38_OWNER_PATH);
            let control_path = self.root.join(XYCE_BUG38_CONTROL_PATH);
            let expected_path = match role {
                XyceBug38Role::WrapperOwner => &owner_path,
                XyceBug38Role::ParenthesizedControl => &control_path,
            };
            if !Self::same_path(&deck.path, expected_path) {
                return Err(format!(
                    "recognized {LABEL} role {role:?} is not backed by its canonical path"
                ));
            }
            let contract = XyceBug38FamilyContract {
                relational: XyceBaselineFamilyContract {
                    kind: XyceBaselineFamilyKind::Bug38SubcktFormalParentheses,
                    comparison: XyceBaselineFamilyComparison::ExactPrnCaseInsensitive,
                    family: "Certification_Tests/BUG_38_SON".to_string(),
                    baseline_path: control_path.clone(),
                    member_paths: vec![owner_path.clone(), control_path.clone()],
                    target_path: Some(expected_path.clone()),
                },
                owner_path,
                control_path,
                role,
            };
            self.validate_bug38_family_semantics(&contract)?;
            Ok(contract)
        })())
    }

    /// The owner and the parenthesized control must parse to one transient circuit.
    fn validate_bug38_family_semantics(
        &self,
        contract: &XyceBug38FamilyContract,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG_38_SON SUBCKT formal-parentheses family";
        let control_plan = self.static_tran_family_plan_for_path(
            &contract.control_path,
            XyceStaticTranPlanPurpose::RelationalFamily,
        )?;
        let owner_plan = self.static_tran_family_plan_for_path(
            &contract.owner_path,
            XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
        )?;
        Self::validate_bug38_transient_plan(&control_plan, XyceBug38Role::ParenthesizedControl)?;
        Self::validate_bug38_transient_plan(&owner_plan, XyceBug38Role::WrapperOwner)?;
        let control_netlist =
            Self::parse_xyce_netlist(&control_plan.source, &control_plan.deck_path)
                .map_err(|error| format!("failed to parse {LABEL} control: {error}"))?;
        let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_plan.deck_path)
            .map_err(|error| format!("failed to parse {LABEL} owner: {error}"))?;
        let control_snapshot = Self::bug38_family_snapshot(&control_netlist, &control_plan)?;
        let owner_snapshot = Self::bug38_family_snapshot(&owner_netlist, &owner_plan)?;
        Self::compare_bug38_family_snapshots(&control_snapshot, &owner_snapshot)
    }

    pub(super) fn bug38_source_qualification(
        source: &str,
    ) -> Result<(XyceBug38SubcktRepresentation, Vec<String>), String> {
        const LABEL: &str = "BUG_38_SON SUBCKT formal-parentheses family";
        let records = source
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();
        if records.len() != 9 {
            return Err(format!(
                "{LABEL} requires exactly nine nonempty physical source records"
            ));
        }
        let representation = match (records[0], records[1]) {
            (
                "Bug 38 SON:  HSpice compatibility test.  (no parentheses)",
                ".subckt RESISTOR 1 2",
            ) => XyceBug38SubcktRepresentation::BareFormals,
            (
                "Bug 38 SON:  HSpice compatibility test.  (with subckt node parentheses)",
                ".subckt RESISTOR (1 2)",
            ) => XyceBug38SubcktRepresentation::ParenthesizedFormals,
            _ => {
                return Err(format!(
                    "{LABEL} admits only the exact bare or balanced-parenthesized formal-port spelling"
                ));
            }
        };
        let expected_tail = [
            "R1 1 2 10",
            ".ENDS",
            "X1 1 0 RESISTOR",
            "V1 1 0 sin (0 10 10MEG 0 0)",
            ".print tran v(1) I(v1)",
            ".tran 1ns 1us",
            ".end",
        ];
        if records[2..] != expected_tail {
            return Err(format!(
                "{LABEL} source differs outside the qualified formal-port representation"
            ));
        }
        Ok((
            representation,
            vec![
                ".subckt resistor 1 2".to_string(),
                "r1 1 2 10".to_string(),
                ".ends".to_string(),
                "x1 1 0 resistor".to_string(),
                "v1 1 0 sin (0 10 10meg 0 0)".to_string(),
                ".print tran v(1) i(v1)".to_string(),
                ".tran 1ns 1us".to_string(),
                ".end".to_string(),
            ],
        ))
    }

    pub(super) fn validate_bug38_transient_plan(
        plan: &XyceStaticTranPlan,
        role: XyceBug38Role,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG_38_SON SUBCKT formal-parentheses family";
        let (representation, _) = Self::bug38_source_qualification(&plan.source)?;
        let expected_representation = match role {
            XyceBug38Role::WrapperOwner => XyceBug38SubcktRepresentation::BareFormals,
            XyceBug38Role::ParenthesizedControl => {
                XyceBug38SubcktRepresentation::ParenthesizedFormals
            }
        };
        let expected_contract = match role {
            XyceBug38Role::WrapperOwner => XyceStaticTranContract::WrapperStatic,
            XyceBug38Role::ParenthesizedControl => XyceStaticTranContract::PlainStatic,
        };
        let print = plan.require_print(LABEL)?;
        let probes = print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if representation != expected_representation
            || plan.contract != expected_contract
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || !matches!(
                plan.comparison_mode,
                XyceStaticTranComparisonMode::Pointwise
            )
            || plan.tran.step.to_bits() != 1.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 1.0e-6f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || probes != ["v(1)", "i(v1)"]
        {
            return Err(format!(
                "{LABEL} requires its exact role, default PRN plan, '.TRAN 1ns 1us', and ordered 'V(1) I(V1)' print request"
            ));
        }
        Ok(())
    }

    pub(super) fn bug38_family_snapshot(
        netlist: &Netlist,
        plan: &XyceStaticTranPlan,
    ) -> Result<XyceBug38FamilySnapshot, String> {
        const LABEL: &str = "BUG_38_SON SUBCKT formal-parentheses family";
        let source = netlist
            .source_text
            .as_deref()
            .ok_or_else(|| format!("{LABEL} requires original source text"))?;
        if source != plan.source {
            return Err(format!("{LABEL} parsed source and plan source differ"));
        }
        let (representation, semantic_source) = Self::bug38_source_qualification(source)?;
        let role = match representation {
            XyceBug38SubcktRepresentation::BareFormals => XyceBug38Role::WrapperOwner,
            XyceBug38SubcktRepresentation::ParenthesizedFormals => {
                XyceBug38Role::ParenthesizedControl
            }
        };
        Self::validate_bug38_transient_plan(plan, role)?;
        let expected_title = match representation {
            XyceBug38SubcktRepresentation::BareFormals => {
                "Bug 38 SON:  HSpice compatibility test.  (no parentheses)"
            }
            XyceBug38SubcktRepresentation::ParenthesizedFormals => {
                "Bug 38 SON:  HSpice compatibility test.  (with subckt node parentheses)"
            }
        };
        if netlist.title.trim() != expected_title
            || !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 2
            || netlist.subcircuits.len() != 1
            || !netlist.models.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "{LABEL} requires the exact diagnostic-free one-subcircuit transient topology"
            ));
        }
        let [
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                uic,
            },
        ] = netlist.analyses.as_slice()
        else {
            return Err(format!("{LABEL} requires exactly one transient analysis"));
        };
        if step.to_bits() != 1.0e-9f64.to_bits()
            || stop.to_bits() != 1.0e-6f64.to_bits()
            || start.is_some()
            || max_step.is_some()
            || *uic
        {
            return Err(format!("{LABEL} parsed transient analysis changed"));
        }

        let fingerprint = |element: &rspice_core::netlist::Element| {
            if element.provenance != ElementProvenance::Authored {
                return Err(format!(
                    "{LABEL} element '{}' is not authored source state",
                    element.name
                ));
            }
            let nodes = element
                .nodes
                .iter()
                .map(|node| node.trim().to_ascii_lowercase())
                .collect::<Vec<_>>();
            match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } if (nodes == ["1", "2"] || nodes == ["1", "0"])
                    && value.to_bits() == 10.0f64.to_bits()
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty() =>
                {
                    Ok(XyceRelationalElementFingerprint {
                        kind: "R".to_string(),
                        nodes,
                        numeric_bits: vec![value.to_bits()],
                        text: Vec::new(),
                    })
                }
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } if nodes == ["1", "0"]
                    && subckt_name.eq_ignore_ascii_case("resistor")
                    && params.is_empty() =>
                {
                    Ok(XyceRelationalElementFingerprint {
                        kind: "X".to_string(),
                        nodes,
                        numeric_bits: Vec::new(),
                        text: vec!["resistor".to_string()],
                    })
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Sin {
                    offset,
                    amplitude,
                    frequency,
                    delay,
                    damping,
                    phase,
                }) if nodes == ["1", "0"]
                    && offset.to_bits() == 0.0f64.to_bits()
                    && amplitude.to_bits() == 10.0f64.to_bits()
                    && frequency.to_bits() == 10.0e6f64.to_bits()
                    && delay.to_bits() == 0.0f64.to_bits()
                    && damping.to_bits() == 0.0f64.to_bits()
                    && phase.to_bits() == 0.0f64.to_bits() =>
                {
                    Ok(XyceRelationalElementFingerprint {
                        kind: "V:SIN".to_string(),
                        nodes,
                        numeric_bits: [*offset, *amplitude, *frequency, *delay, *damping, *phase]
                            .into_iter()
                            .map(Value::to_bits)
                            .collect(),
                        text: Vec::new(),
                    })
                }
                other => Err(format!(
                    "{LABEL} contains an unqualified element '{}': {other:?}",
                    element.name
                )),
            }
        };

        let mut top_level_elements = BTreeMap::new();
        for element in &netlist.elements {
            let name = Self::normalize_device_instance_name(&element.name);
            if !matches!(name.as_str(), "x1" | "v1")
                || top_level_elements
                    .insert(name, fingerprint(element)?)
                    .is_some()
            {
                return Err(format!(
                    "{LABEL} contains an unknown or duplicate top-level element"
                ));
            }
        }

        let subcircuit = &netlist.subcircuits[0];
        if !subcircuit.name.eq_ignore_ascii_case("resistor")
            || subcircuit
                .ports
                .iter()
                .map(|port| port.to_ascii_lowercase())
                .collect::<Vec<_>>()
                != ["1", "2"]
            || subcircuit.elements.len() != 1
            || !subcircuit.initial_conditions.is_empty()
            || !subcircuit.node_sets.is_empty()
            || !subcircuit.params.is_empty()
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.string_params.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.body_string_params.is_empty()
            || !subcircuit.body_functions.is_empty()
            || !subcircuit.local_options.is_empty()
            || subcircuit.library_ref.is_some()
            || !subcircuit.nested_subcircuits.is_empty()
        {
            return Err(format!(
                "{LABEL} subcircuit definition, formal ports, or local state changed"
            ));
        }
        let body_element = &subcircuit.elements[0];
        if !body_element.name.eq_ignore_ascii_case("r1") {
            return Err(format!("{LABEL} requires the single body resistor R1"));
        }
        let subcircuit_elements = BTreeMap::from([("r1".to_string(), fingerprint(body_element)?)]);

        let flattened = flatten_netlist(netlist)
            .map_err(|error| format!("failed to flatten {LABEL}: {error}"))?;
        if flattened.len() != 2 {
            return Err(format!(
                "{LABEL} must flatten to one source and one 10-ohm resistor"
            ));
        }
        let mut flattened_elements = BTreeMap::new();
        for element in &flattened {
            let name = Self::normalize_device_instance_name(&element.name);
            let is_expected = name == "v1" || name == "x1.r1";
            if !is_expected
                || flattened_elements
                    .insert(name, fingerprint(element)?)
                    .is_some()
            {
                return Err(format!(
                    "{LABEL} flattened topology contains an unknown or duplicate element"
                ));
            }
        }

        let ordered_probes = plan
            .require_print(LABEL)?
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        Ok(XyceBug38FamilySnapshot {
            representation,
            semantic_source,
            top_level_elements,
            subcircuit_name: subcircuit.name.to_ascii_lowercase(),
            subcircuit_ports: subcircuit
                .ports
                .iter()
                .map(|port| port.to_ascii_lowercase())
                .collect(),
            subcircuit_elements,
            flattened_elements,
            tran_step_bits: step.to_bits(),
            tran_stop_bits: stop.to_bits(),
            ordered_probes,
        })
    }

    pub(super) fn compare_bug38_family_snapshots(
        baseline: &XyceBug38FamilySnapshot,
        target: &XyceBug38FamilySnapshot,
    ) -> Result<(), String> {
        if baseline.representation != XyceBug38SubcktRepresentation::ParenthesizedFormals
            || target.representation != XyceBug38SubcktRepresentation::BareFormals
        {
            return Err(
                "BUG_38_SON must compare the parenthesized-formal control to the bare-formal wrapper owner"
                    .to_string(),
            );
        }
        let mut baseline = baseline.clone();
        let mut target = target.clone();
        baseline.representation = XyceBug38SubcktRepresentation::BareFormals;
        target.representation = XyceBug38SubcktRepresentation::BareFormals;
        if baseline != target {
            return Err(
                "BUG_38_SON source semantics, topology, formal bindings, TRAN plan, or PRINT order differ outside the admitted lexical parentheses"
                    .to_string(),
            );
        }
        Ok(())
    }
}
