//! Voltage-rating conventions from ngspice b3/b4/vdmos/bjt/vbic/dio soachk.
//! Only authored limits are imported; simulator infinity defaults are omitted.
use super::*;
use rspice_core::circuit::{
    BjtModelSafetyFamily, DeviceModelSafety, ModelSafetyValue, MosModelSafetyFamily,
};
use std::collections::{BTreeMap, BTreeSet};

pub(super) type ModelLimits = HashMap<String, BTreeMap<SoAParameter, SoALimit>>;

fn voltage_key(key: &str) -> bool {
    matches!(
        key,
        "VGS_MAX"
            | "VGSR_MAX"
            | "VGD_MAX"
            | "VGDR_MAX"
            | "VGB_MAX"
            | "VGBR_MAX"
            | "VDS_MAX"
            | "VBS_MAX"
            | "VBSR_MAX"
            | "VBD_MAX"
            | "VBDR_MAX"
            | "VBE_MAX"
            | "VBC_MAX"
            | "VCE_MAX"
            | "VSUB_MAX"
            | "BVBE"
            | "BVBC"
            | "BVCE"
            | "BVSUB"
            | "VSUBFWD"
            | "FV_MAX"
            | "BV_MAX"
    )
}

struct Import<'a> {
    card: &'a DeviceModelSafety,
    basis: SoaVoltageBasis,
    p_channel: bool,
    used: BTreeSet<&'static str>,
    limits: BTreeMap<SoAParameter, SoALimit>,
}

impl Import<'_> {
    fn number(&self, key: &str) -> Result<Option<f64>, String> {
        match self.card.parameters.get(key) {
            None => Ok(None),
            Some(ModelSafetyValue::Numeric(value)) if value.is_finite() => Ok(Some(*value)),
            Some(value) => Err(format!(
                "model '{}' {key} is not a resolved finite scalar: {value:?}",
                self.card.model_name
            )),
        }
    }

    fn rating(&mut self, key: &'static str) -> Result<Option<(f64, &'static str)>, String> {
        self.used.insert(key);
        let value = self.number(key)?;
        if value.is_some_and(|value| value < 0.0) {
            return Err(format!(
                "model '{}' {key} must be nonnegative",
                self.card.model_name
            ));
        }
        Ok(value.map(|value| (value, key)))
    }

    fn aliased_rating(
        &mut self,
        key: &'static str,
        alias: &'static str,
    ) -> Result<Option<(f64, &'static str)>, String> {
        let canonical = self.rating(key)?;
        let alternate = self.rating(alias)?;
        if let (Some((value, _)), Some((alias_value, _))) = (canonical, alternate)
            && value != alias_value
        {
            return Err(format!(
                "model '{}' has conflicting voltage-rating aliases {key}={value} and {alias}={alias_value}; specify one value",
                self.card.model_name
            ));
        }
        Ok(canonical.or(alternate))
    }

    fn add(&mut self, parameter: SoAParameter, rating: Option<(f64, &'static str)>) {
        let Some((max_value, key)) = rating else {
            return;
        };
        // The generic checker permits zero only on directional limits.
        if max_value == 0.0 && parameter.polarity().is_none() {
            let (positive, negative) = parameter.directional_pair().expect("voltage pair");
            self.add(positive, rating);
            self.add(negative, rating);
            return;
        }
        let limit = SoALimit {
            duration_mode: Default::default(),
            minimum_duration_s: None,
            power_derating: None,
            parameter,
            max_value,
            voltage_basis: self.basis,
            unit: "V".into(),
            description: format!(
                "Model '{}' {}={} V: maximum {} at {}",
                self.card.model_name,
                key,
                max_value,
                parameter.stress_code(),
                match self.basis {
                    SoaVoltageBasis::ExternalTerminals => "authored terminals",
                    SoaVoltageBasis::IntrinsicNodes => "intrinsic electrical model nodes",
                }
            ),
        };
        // Some BSIM fallbacks repeat a check; retain the stricter equivalent.
        if self
            .limits
            .get(&parameter)
            .is_none_or(|old| max_value < old.max_value)
        {
            self.limits.insert(parameter, limit);
        }
    }

    fn pair(
        &mut self,
        parameter: SoAParameter,
        forward: Option<(f64, &'static str)>,
        reverse: Option<(f64, &'static str)>,
    ) {
        if reverse.is_none() {
            self.add(parameter, forward);
        } else {
            let (positive, negative) = parameter.directional_pair().expect("voltage pair");
            let (f, r) = if self.p_channel {
                (negative, positive)
            } else {
                (positive, negative)
            };
            self.add(f, forward);
            self.add(r, reverse);
        }
    }
}

pub(super) fn resolve(
    circuit: &rspice_core::CircuitData,
    elements: &[Element],
    config: &SoaRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ModelLimits> {
    let mut all = HashMap::new();
    if !config.import_model_voltage_ratings {
        return Ok(all);
    }
    for (index, element) in elements.iter().enumerate() {
        poll_periodically(abort, index)?;
        if !config.observation.includes(element) {
            continue;
        }
        let Some(card) = circuit.device_model_safety(&element.name) else {
            continue;
        };
        if !card.parameters.keys().any(|key| voltage_key(key)) {
            continue;
        }
        let limits = import_card(card).map_err(|error| {
            ServiceRunError::Failure(format!(
                "SOA voltage-rating import for '{}': {error}",
                element.name
            ))
        })?;
        if !limits.is_empty() {
            all.insert(element.name.clone(), limits);
        }
    }
    if all.is_empty() {
        return Err(ServiceRunError::Failure("SOA model voltage-rating import found no authored voltage ratings in the selected devices".into()));
    }
    Ok(all)
}

fn import_card(card: &DeviceModelSafety) -> Result<BTreeMap<SoAParameter, SoALimit>, String> {
    use SoAParameter::*;
    if card.generated {
        return Err(format!(
            "model '{}' uses a generated implementation whose rating conventions are not supported",
            card.model_name
        ));
    }
    let mut importer = Import {
        card,
        basis: SoaVoltageBasis::IntrinsicNodes,
        p_channel: card.mos.is_some_and(|mos| mos.p_channel),
        used: BTreeSet::new(),
        limits: BTreeMap::new(),
    };
    let level = importer.number("LEVEL")?.unwrap_or(1.0);
    let kind = card.model_type.to_ascii_uppercase();
    let bsim = card.mos.is_some_and(|mos| {
        matches!(
            mos.family,
            MosModelSafetyFamily::Bsim3 | MosModelSafetyFamily::Bsim4
        )
    });
    let vdmos = card
        .mos
        .is_some_and(|mos| mos.family == MosModelSafetyFamily::Vdmos);
    if bsim || vdmos {
        if vdmos {
            importer.basis = SoaVoltageBasis::ExternalTerminals;
        }
        let vgs = importer.rating("VGS_MAX")?;
        let vgsr = importer.rating("VGSR_MAX")?;
        let vgd = importer.rating("VGD_MAX")?;
        let vgdr = importer.rating("VGDR_MAX")?;
        let vds = importer.rating("VDS_MAX")?;
        importer.pair(Vgs, vgs, vgsr);
        importer.pair(Vgd, vgd, vgdr);
        importer.add(Vds, vds);
        if bsim {
            let vgb = importer.rating("VGB_MAX")?;
            let vgbr = importer.rating("VGBR_MAX")?;
            let vbs = importer.rating("VBS_MAX")?;
            let vbsr = importer.rating("VBSR_MAX")?;
            let vbd = importer.rating("VBD_MAX")?;
            let vbdr = importer.rating("VBDR_MAX")?;
            // Both BSIM checkers additionally check |Vgb| in the symmetric
            // Vgs branch, even when a separate reverse Vgb limit is given.
            if vgsr.is_none() {
                importer.add(Vgb, vgb.or(vgs));
            }
            importer.pair(Vgb, vgb, vgbr);
            importer.pair(Vbs, vbs.or(vbd), vbsr);
            importer.pair(Vbd, vbd, vbdr);
        }
    } else if card.bjt_family == Some(BjtModelSafetyFamily::Vbic) {
        importer.basis = SoaVoltageBasis::ExternalTerminals;
        for (parameter, key, alias) in [
            (Vbe, "VBE_MAX", "BVBE"),
            (Vbc, "VBC_MAX", "BVBC"),
            (Vce, "VCE_MAX", "BVCE"),
            (Vcsub, "VSUB_MAX", "BVSUB"),
        ] {
            let rating = importer.aliased_rating(key, alias)?;
            importer.add(parameter, rating);
        }
        // VBIC checks type * (Vsubstrate - Vcollector) against VSUBFWD.
        // Vcsub is collector minus substrate, so forward NPN is negative.
        let forward_substrate = importer.rating("VSUBFWD")?;
        importer.add(
            if kind == "PNP" {
                VcsubPositive
            } else {
                VcsubNegative
            },
            forward_substrate,
        );
        // VBEFWD/VBCFWD classify the operating region, not a stress limit.
    } else if card.bjt_family == Some(BjtModelSafetyFamily::GummelPoon) {
        for (parameter, key) in [(Vbe, "VBE_MAX"), (Vbc, "VBC_MAX"), (Vce, "VCE_MAX")] {
            let rating = importer.rating(key)?;
            importer.add(parameter, rating);
        }
    } else if matches!(kind.as_str(), "D" | "DIODE") {
        importer.basis = SoaVoltageBasis::ExternalTerminals;
        for (parameter, key) in [(VakPositive, "FV_MAX"), (VakNegative, "BV_MAX")] {
            let rating = importer.rating(key)?;
            importer.add(parameter, rating);
        }
    } else {
        return Err(format!(
            "model '{}' type {} LEVEL={} has no supported native voltage-rating convention",
            card.model_name, card.model_type, level
        ));
    }
    if let Some(key) = card
        .parameters
        .keys()
        .find(|key| voltage_key(key) && !importer.used.contains(key.as_str()))
    {
        return Err(format!(
            "model '{}' rating {key} is not supported by its selected model family",
            card.model_name
        ));
    }
    Ok(importer.limits)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn soa_model_voltage_import_preserves_polarity_fallbacks_and_rejects_invalid_ratings() {
        use SoAParameter::*;
        let mut card = DeviceModelSafety {
            model_name: "PM.1".into(),
            model_type: "PMOS".into(),
            generated: false,
            bjt_family: None,
            mos: Some(rspice_core::circuit::MosModelSafety {
                family: MosModelSafetyFamily::Bsim4,
                p_channel: true,
            }),
            parameters: [
                ("LEVEL", 54.0),
                ("VGS_MAX", 2.0),
                ("VGSR_MAX", 0.5),
                ("VBD_MAX", 3.0),
                ("VBSR_MAX", 0.25),
            ]
            .into_iter()
            .map(|(k, v)| (k.into(), ModelSafetyValue::Numeric(v)))
            .collect(),
        };
        let limits = import_card(&card).unwrap();
        assert_eq!(limits[&VgsNegative].max_value, 2.0);
        assert_eq!(limits[&VgsPositive].max_value, 0.5);
        assert_eq!(limits[&VbsNegative].max_value, 3.0);
        assert_eq!(limits[&VbsPositive].max_value, 0.25);
        assert!(!limits.contains_key(&Vgb)); // Vgs reverse suppresses Vgb fallback.
        card.parameters.remove("VGSR_MAX");
        let limits = import_card(&card).unwrap();
        assert_eq!(limits[&Vgb].max_value, 2.0);
        assert_eq!(limits[&Vgs].voltage_basis, SoaVoltageBasis::IntrinsicNodes);
        card.parameters
            .insert("VDS_MAX".into(), ModelSafetyValue::Numeric(0.0));
        let limits = import_card(&card).unwrap();
        assert_eq!(limits[&VdsPositive].max_value, 0.0);
        assert_eq!(limits[&VdsNegative].max_value, 0.0);
        for value in [
            ModelSafetyValue::Numeric(-1.0),
            ModelSafetyValue::Numeric(f64::NAN),
            ModelSafetyValue::Unresolved("missing".into()),
        ] {
            card.parameters.insert("VDS_MAX".into(), value);
            assert!(import_card(&card).unwrap_err().contains("VDS_MAX"));
        }
        card.parameters.remove("VDS_MAX");
        card.generated = true;
        assert!(import_card(&card).unwrap_err().contains("generated"));
        card.generated = false;
        card.parameters
            .insert("LEVEL".into(), ModelSafetyValue::Numeric(9.0));
        // Selected family/polarity remains authoritative over raw card metadata.
        card.model_type = "NMOS".into();
        assert_eq!(import_card(&card).unwrap()[&VbsNegative].max_value, 3.0);
        card.mos = None;
        assert!(
            import_card(&card)
                .unwrap_err()
                .contains("no supported native")
        );
    }
}

#[cfg(test)]
#[test]
fn soa_model_voltage_override_preserves_opposite_direction_and_reports_missing_ratings() {
    use SoAParameter::*;
    let netlist = rspice_core::Netlist::parse("Ratings\nM1 d g 0 0 PM W=10u L=1u\n.model PM PMOS LEVEL=54 VGS_MAX=2 VGB_MAX=2 VGBR_MAX=0.25\n.end\n").unwrap();
    let engine = rspice_core::engine::Engine::new(Default::default());
    let circuit = engine.build_circuit(&netlist).unwrap();
    let mut config = SoaRunConfig {
        import_model_voltage_ratings: true,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        rules: vec![SoaRuleConfig {
            duration_mode: Default::default(),
            minimum_duration_s: None,
            power_derating: None,
            parameter: VgbPositive,
            voltage_basis: SoaVoltageBasis::IntrinsicNodes,
            max_value: 3.0,
            devices: vec![],
            models: vec![],
        }],
        ..Default::default()
    };
    let imported = resolve(&circuit, &netlist.elements, &config, &NoAbort).unwrap();
    let layouts = HashMap::from([(
        "M1".into(),
        super::terminals::TerminalLayout {
            body: Some(3),
            intrinsic_voltages: u128::MAX,
            ..Default::default()
        },
    )]);
    let resolved =
        rules::resolve(&netlist.elements, &config, &layouts, &imported, &NoAbort).unwrap();
    let limit = |parameter| {
        resolved[0]
            .1
            .limits
            .iter()
            .find(|r| r.parameter == parameter)
            .unwrap()
            .max_value
    };
    // PMOS reverse is positive. Override negative instead to leave that
    // stricter positive rating intact when splitting the symmetric fallback.
    assert_eq!(limit(VgbPositive), 3.0);
    assert_eq!(limit(VgbNegative), 2.0);
    config.rules[0].parameter = VgbNegative;
    let resolved =
        rules::resolve(&netlist.elements, &config, &layouts, &imported, &NoAbort).unwrap();
    assert_eq!(
        resolved[0]
            .1
            .limits
            .iter()
            .find(|r| r.parameter == VgbPositive)
            .unwrap()
            .max_value,
        0.25
    );
    config.rules[0].parameter = Vgb;
    let resolved =
        rules::resolve(&netlist.elements, &config, &layouts, &imported, &NoAbort).unwrap();
    assert!(
        resolved[0]
            .1
            .limits
            .iter()
            .all(|r| !matches!(r.parameter, VgbPositive | VgbNegative))
    );
    config.rules.clear();
    config.check_vgs_max = true;
    config.max_vgs = 1.8;
    let one_sided = rspice_core::Netlist::parse(
        "One side\nM1 d g 0 0 PM W=10u L=1u\n.model PM PMOS LEVEL=54 VGSR_MAX=0.5\n.end\n",
    )
    .unwrap();
    let circuit = engine.build_circuit(&one_sided).unwrap();
    let imported = resolve(&circuit, &one_sided.elements, &config, &NoAbort).unwrap();
    let resolved =
        rules::resolve(&one_sided.elements, &config, &layouts, &imported, &NoAbort).unwrap();
    let retained_default = resolved[0]
        .1
        .limits
        .iter()
        .find(|r| r.parameter == VgsNegative)
        .unwrap();
    assert_eq!(retained_default.max_value, 1.8);
    assert_eq!(
        retained_default.voltage_basis,
        SoaVoltageBasis::ExternalTerminals
    );
    let model_reverse = resolved[0]
        .1
        .limits
        .iter()
        .find(|r| r.parameter == VgsPositive)
        .unwrap();
    assert_eq!(model_reverse.max_value, 0.5);
    assert_eq!(model_reverse.voltage_basis, SoaVoltageBasis::IntrinsicNodes);
    let mos9 = rspice_core::Netlist::parse("Berkeley MOS9\nM1 d g 0 0 NM W=10u L=1u\n.model NM NMOS LEVEL=9 VTO=0.4 KP=1m VGS_MAX=1\n.end\n").unwrap();
    let circuit = engine.build_circuit(&mos9).unwrap();
    assert!(
        resolve(&circuit, &mos9.elements, &config, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("no supported native voltage-rating convention")
    );
    let plain =
        rspice_core::Netlist::parse("Unrated\nD1 a 0 DM\n.model DM D IS=1e-14\n.end\n").unwrap();
    let circuit = engine.build_circuit(&plain).unwrap();
    assert!(
        resolve(&circuit, &plain.elements, &config, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("no authored voltage ratings")
    );
}

#[cfg(test)]
#[test]
fn soa_vbic_model_ratings_validate_aliases_polarity_and_electrical_substrate() {
    use SoAParameter::*;
    let config = SoaRunConfig {
        import_model_voltage_ratings: true,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        ..Default::default()
    };
    let engine = rspice_core::engine::Engine::new(Default::default());
    let netlist = rspice_core::Netlist::parse("Inferred VBIC\nQ1 c b 0 s PM\n.model PM PNP RCI=1 BVBE=2 VBE_MAX=2 BVBC=3 BVCE=4 BVSUB=5 VSUBFWD=0.4 VBEFWD=0.2 VBCFWD=0.3\n.end\n").unwrap();
    let circuit = engine.build_circuit(&netlist).unwrap();
    let mut card = circuit.device_model_safety("Q1").unwrap().clone();
    let limits = import_card(&card).unwrap();
    assert_eq!(limits.len(), 5);
    for (parameter, value) in [
        (Vbe, 2.0),
        (Vbc, 3.0),
        (Vce, 4.0),
        (Vcsub, 5.0),
        (VcsubPositive, 0.4),
    ] {
        assert_eq!(limits[&parameter].max_value, value);
        assert_eq!(
            limits[&parameter].voltage_basis,
            SoaVoltageBasis::ExternalTerminals
        );
    }
    assert!(limits[&Vbc].description.contains("BVBC"));
    card.model_type = "NPN".into();
    let limits = import_card(&card).unwrap();
    assert_eq!(limits[&VcsubNegative].max_value, 0.4);
    assert!(!limits.contains_key(&VcsubPositive));
    card.parameters
        .insert("BVBE".into(), ModelSafetyValue::Numeric(1.0));
    assert!(
        import_card(&card)
            .unwrap_err()
            .contains("conflicting voltage-rating aliases")
    );
    card.parameters.insert(
        "BVBE".into(),
        ModelSafetyValue::Unresolved("unbound".into()),
    );
    assert!(import_card(&card).unwrap_err().contains("BVBE"));
    for nodes in ["c b 0 th", "c b 0"] {
        let deck = format!(
            "No substrate\nQ1 {nodes} VM\n.model VM NPN LEVEL=11 IS=1e-16 VSUB_MAX=1\n.end\n"
        );
        let netlist = rspice_core::Netlist::parse(&deck).unwrap();
        let (layouts, ratings) =
            super::terminals::resolve(&netlist, &netlist.elements, &config, &engine, &NoAbort)
                .unwrap();
        let error = rules::resolve(&netlist.elements, &config, &layouts, &ratings, &NoAbort)
            .unwrap_err()
            .to_string();
        assert!(error.contains("electrical substrate pin"), "{error}");
    }
}
