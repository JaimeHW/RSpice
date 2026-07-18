//! Advanced native MOS-family builders used by circuit construction.

use super::*;

impl Engine {
    /// Build and register a BSIMSOI dynamic-depletion (level 56) instance.
    ///
    /// Node topology (b3soiddset.c:975-1037):
    /// - 4-terminal `m d g s e`: floating body. An internal body node
    ///   `<name>.__body.internal` is allocated; `bodyMod = 0`, `float = 1`.
    /// - 5-terminal `m d g s e p`: body tie. With `rbody == rbsh == 0`, the
    ///   external `p` node *is* the body node (`bodyMod = 2`). Otherwise, an
    ///   internal body node sits behind the body-contact resistor (`bodyMod = 1`).
    ///
    /// Positive `RSH * NRD/NRS` allocates drain/source prime nodes connected by
    /// ordinary linear resistors. Positive `RTH0` with `SHMOD=1` allocates the
    /// BSIMSOI self-heating temperature-rise node.
    pub(super) fn build_b3soi_dd(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        model_key: &str,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        deferred_params: &[(String, String)],
        temperature_kelvin: f64,
    ) -> Result<(), SimulationError> {
        use crate::device::mosfet::b3soi::dd::temp::{B3SoiDdGeometry, B3SoiDdSized};
        use crate::device::{B3SoiDd, B3SoiDdModel, BodyMode};

        let is_pmos = matches!(mos_type, crate::netlist::MosType::Pmos);
        // `config.temperature` is already in Kelvin (`TEMP_REFERENCE`).
        let temp_k = temperature_kelvin;

        // ngspice TNOM defaults to 27C; the model card may override it (Celsius).
        let tnom_c = params_map.get("TNOM").copied().unwrap_or(27.0);
        let tnom_k = crate::analysis::temperature::celsius_to_kelvin(tnom_c);

        let model = std::sync::Arc::new(
            B3SoiDdModel::try_from_params(params_map, is_pmos, tnom_k).map_err(|err| {
                SimulationError::Circuit(format!(
                    "MOSFET '{}': native B3SOIDD model '{model_key}': {err}",
                    element.name
                ))
            })?,
        );

        let node_drain_external = circuit.get_or_create_node(&element.nodes[0]);
        let node_gate_external = circuit.get_or_create_node(&element.nodes[1]);
        let node_source_external = circuit.get_or_create_node(&element.nodes[2]);
        let node_e = circuit.get_or_create_node(&element.nodes[3]);

        let ideal_body_tie = model.rbody == 0.0 && model.rbsh == 0.0;
        let (node_body, node_p, body_mode) = if element.nodes.len() > 4 {
            let p = circuit.get_or_create_node(&element.nodes[4]);
            if ideal_body_tie {
                // Ideal body tie: P is the body node.
                (p, p, BodyMode::TiedIdeal)
            } else {
                // Nonideal body tie: an internal body node sits behind the
                // body-contact resistor; P remains the external contact.
                let body = circuit.get_or_create_node(&format!("{}.__body.internal", element.name));
                (body, p, BodyMode::TiedResistive)
            }
        } else {
            // Floating body: allocate an internal body node.
            let body = circuit.get_or_create_node(&format!("{}.__body.internal", element.name));
            (body, 0, BodyMode::Floating)
        };

        // Instance geometry (W/L plus optional area/perimeter/squares).
        let l = instance_param(instance_params, &["L"]).unwrap_or(0.0);
        let w = instance_param(instance_params, &["W"]).unwrap_or(0.0);
        let geom = B3SoiDdGeometry {
            l,
            w,
            drain_area: instance_param(instance_params, &["AD"]).unwrap_or(0.0),
            source_area: instance_param(instance_params, &["AS"]).unwrap_or(0.0),
            drain_squares: instance_param(instance_params, &["NRD"]).unwrap_or(0.0),
            source_squares: instance_param(instance_params, &["NRS"]).unwrap_or(0.0),
            drain_perimeter: instance_param(instance_params, &["PD"]).unwrap_or(0.0),
            source_perimeter: instance_param(instance_params, &["PS"]).unwrap_or(0.0),
            body_squares: instance_param(instance_params, &["NRB"]).unwrap_or(0.0),
            rth0: instance_param(instance_params, &["RTH0"]).unwrap_or(model.rth0),
            cth0: instance_param(instance_params, &["CTH0"]).unwrap_or(model.cth0),
            nseg: instance_param(instance_params, &["NSEG"]).unwrap_or(1.0),
            frbody: instance_param(instance_params, &["FRBODY"]).unwrap_or(1.0),
        };

        let node_gate = match model.rgate_mod {
            1 => {
                let gate_prime = circuit.get_or_create_node(&format!("{}.__gint", element.name));
                let sized = B3SoiDdSized::new(&model, &geom, temp_k).map_err(|err| {
                    SimulationError::Circuit(format!(
                        "MOSFET '{}': native B3SOIDD gate resistance: {err}",
                        element.name
                    ))
                })?;
                if sized.grgeltd.is_finite() && sized.grgeltd > 0.0 {
                    circuit.resistors.add(
                        format!("{}.__rg", element.name),
                        node_gate_external,
                        gate_prime,
                        1.0 / sized.grgeltd,
                    );
                }
                gate_prime
            }
            2 => circuit.get_or_create_node(&format!("{}.__gint", element.name)),
            _ => node_gate_external,
        };

        let drain_resistance = model.sheet_resistance * geom.drain_squares;
        let node_drain = if drain_resistance.is_finite() && drain_resistance > 0.0 {
            let dint = circuit.get_or_create_node(&format!("{}.__dint", element.name));
            circuit.resistors.add(
                format!("{}.__rd", element.name),
                node_drain_external,
                dint,
                drain_resistance,
            );
            dint
        } else {
            node_drain_external
        };
        let source_resistance = model.sheet_resistance * geom.source_squares;
        let node_source = if source_resistance.is_finite() && source_resistance > 0.0 {
            let sint = circuit.get_or_create_node(&format!("{}.__sint", element.name));
            circuit.resistors.add(
                format!("{}.__rs", element.name),
                node_source_external,
                sint,
                source_resistance,
            );
            sint
        } else {
            node_source_external
        };
        let node_temp = if model.sh_mod == 1 && geom.rth0 != 0.0 {
            circuit.get_or_create_node(&format!("{}.__temp.internal", element.name))
        } else {
            0
        };
        let instance_ic = Self::native_b3soi_instance_ic(
            circuit,
            &element.name,
            "B3SOIDD",
            instance_params,
            deferred_params,
            node_drain,
            node_gate_external,
            node_source,
            node_body,
            node_e,
            node_p,
        )?;

        let mut device = B3SoiDd::new(
            element.name.clone(),
            node_drain,
            node_gate_external,
            node_gate,
            node_source,
            node_e,
            node_body,
            node_p,
            node_temp,
            body_mode,
            model,
            geom,
            temp_k,
        )
        .map_err(SimulationError::Circuit)?;
        device.set_instance_ic(instance_ic);

        // DEBUG=-1 runs the device without dynamic charges (ngspice debugMod).
        if let Some(debug) =
            native_b3soi_debug_mod(&element.name, "B3SOIDD", instance_params, deferred_params)?
        {
            device.set_debug_mod(debug);
        }
        circuit.b3soi.add(device);
        Ok(())
    }

    /// Build and register a BSIMSOI fully-depleted (level 55) instance.
    ///
    /// Node topology (b3soifdset.c): FD never solves the body as a circuit node.
    /// - 4-terminal `m d g s e`: floating body, `bNode = 0` — no internal node is
    ///   created (the body voltage is pinned to `Vbs0eff` in the load).
    /// - 5-terminal `m d g s e b`: body contact present. The external `b` node is
    ///   read for the initial guess but the load still pins `Vbs = Vbs0eff`.
    ///
    /// Positive `RSH * NRD/NRS` allocates drain/source prime nodes connected by
    /// ordinary linear resistors. Self-heating allocates an internal
    /// temperature-rise node when `SHMOD=1` and `RTH0` is nonzero.
    pub(super) fn build_b3soi_fd(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        model_key: &str,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        deferred_params: &[(String, String)],
        temperature_kelvin: f64,
    ) -> Result<(), SimulationError> {
        use crate::device::B3SoiFd;
        use crate::device::B3SoiFdModel;
        use crate::device::mosfet::b3soi::fd::BodyMode;
        use crate::device::mosfet::b3soi::fd::temp::B3SoiFdGeometry;

        let is_pmos = matches!(mos_type, crate::netlist::MosType::Pmos);
        let temp_k = temperature_kelvin;
        let tnom_c = params_map.get("TNOM").copied().unwrap_or(27.0);
        let tnom_k = crate::analysis::temperature::celsius_to_kelvin(tnom_c);

        let model = std::sync::Arc::new(
            B3SoiFdModel::try_from_params(params_map, is_pmos, tnom_k).map_err(|err| {
                SimulationError::Circuit(format!(
                    "MOSFET '{}': native B3SOIFD model '{model_key}': {err}",
                    element.name
                ))
            })?,
        );

        let node_drain_external = circuit.get_or_create_node(&element.nodes[0]);
        let node_gate = circuit.get_or_create_node(&element.nodes[1]);
        let node_source_external = circuit.get_or_create_node(&element.nodes[2]);
        let node_e = circuit.get_or_create_node(&element.nodes[3]);

        let (node_body, body_mode) = if element.nodes.len() > 4 {
            // Body contact: read its node for the initial guess only.
            let b = circuit.get_or_create_node(&element.nodes[4]);
            (b, BodyMode::TiedIdeal)
        } else {
            // Floating body: FD allocates no body node.
            (0, BodyMode::Floating)
        };

        let l = instance_param(instance_params, &["L"]).unwrap_or(0.0);
        let w = instance_param(instance_params, &["W"]).unwrap_or(0.0);
        let geom = B3SoiFdGeometry {
            l,
            w,
            drain_area: instance_param(instance_params, &["AD"]).unwrap_or(0.0),
            source_area: instance_param(instance_params, &["AS"]).unwrap_or(0.0),
            drain_squares: instance_param(instance_params, &["NRD"]).unwrap_or(0.0),
            source_squares: instance_param(instance_params, &["NRS"]).unwrap_or(0.0),
            drain_perimeter: instance_param(instance_params, &["PD"]).unwrap_or(0.0),
            source_perimeter: instance_param(instance_params, &["PS"]).unwrap_or(0.0),
            body_squares: instance_param(instance_params, &["NRB"]).unwrap_or(0.0),
            rth0: instance_param(instance_params, &["RTH0"])
                .or_else(|| params_map.get("RTH0").copied())
                .unwrap_or(0.0),
            cth0: instance_param(instance_params, &["CTH0"])
                .or_else(|| params_map.get("CTH0").copied())
                .unwrap_or(0.0),
        };

        let drain_resistance = model.sheet_resistance * geom.drain_squares;
        let node_drain = if drain_resistance.is_finite() && drain_resistance > 0.0 {
            let dint = circuit.get_or_create_node(&format!("{}.__dint", element.name));
            circuit.resistors.add(
                format!("{}.__rd", element.name),
                node_drain_external,
                dint,
                drain_resistance,
            );
            dint
        } else {
            node_drain_external
        };
        let source_resistance = model.sheet_resistance * geom.source_squares;
        let node_source = if source_resistance.is_finite() && source_resistance > 0.0 {
            let sint = circuit.get_or_create_node(&format!("{}.__sint", element.name));
            circuit.resistors.add(
                format!("{}.__rs", element.name),
                node_source_external,
                sint,
                source_resistance,
            );
            sint
        } else {
            node_source_external
        };

        let node_temp = if model.sh_mod == 1 && geom.rth0 != 0.0 {
            circuit.get_or_create_node(&format!("{}.__temp.internal", element.name))
        } else {
            0
        };
        let instance_ic = Self::native_b3soi_instance_ic(
            circuit,
            &element.name,
            "B3SOIFD",
            instance_params,
            deferred_params,
            node_drain,
            node_gate,
            node_source,
            node_body,
            node_e,
            0,
        )?;

        let mut device = B3SoiFd::new(
            element.name.clone(),
            node_drain,
            node_gate,
            node_source,
            node_e,
            node_temp,
            node_body,
            body_mode,
            model,
            geom,
            temp_k,
        )
        .map_err(SimulationError::Circuit)?;
        device.set_instance_ic(instance_ic);

        // DEBUG=-1 runs the device without dynamic charges (ngspice debugMod).
        if let Some(debug) =
            native_b3soi_debug_mod(&element.name, "B3SOIFD", instance_params, deferred_params)?
        {
            device.set_debug_mod(debug);
        }
        circuit.b3soi_fd.add(device);
        Ok(())
    }

    /// Build and register a BSIMSOI partially-depleted (level 57) instance.
    ///
    /// Node topology (b3soipdset.c) matches DD: a 4-terminal `m d g s e` device
    /// has a floating body modeled with an internal node
    /// `<name>.__body.internal` (`bodyMod = 0`); a 5-terminal `m d g s e b`
    /// device is a body tie. With `rbody == rbsh == 0` it is an ideal tie
    /// (`bodyMod = 2`, the external `b` *is* the body node); otherwise it is a
    /// nonideal tie (`bodyMod = 1`) whose body resistor is folded into the body
    /// stamping. The supported PD decks use `rbody = 1`, so 5-terminal `t4` is a
    /// nonideal tie.
    pub(super) fn build_b3soi_pd(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        model_key: &str,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        deferred_params: &[(String, String)],
        temperature_kelvin: f64,
    ) -> Result<(), SimulationError> {
        use crate::device::B3SoiPd;
        use crate::device::B3SoiPdModel;
        use crate::device::mosfet::b3soi::pd::BodyMode;
        use crate::device::mosfet::b3soi::pd::temp::B3SoiPdGeometry;

        let is_pmos = matches!(mos_type, crate::netlist::MosType::Pmos);
        let temp_k = temperature_kelvin;
        let tnom_c = params_map.get("TNOM").copied().unwrap_or(27.0);
        let tnom_k = crate::analysis::temperature::celsius_to_kelvin(tnom_c);

        let model = std::sync::Arc::new(
            B3SoiPdModel::try_from_params(params_map, is_pmos, tnom_k).map_err(|err| {
                SimulationError::Circuit(format!(
                    "MOSFET '{}': native B3SOIPD model '{model_key}': {err}",
                    element.name
                ))
            })?,
        );

        let node_drain_external = circuit.get_or_create_node(&element.nodes[0]);
        let node_gate = circuit.get_or_create_node(&element.nodes[1]);
        let node_source_external = circuit.get_or_create_node(&element.nodes[2]);
        let node_e = circuit.get_or_create_node(&element.nodes[3]);

        let rbody = params_map.get("RBODY").copied().unwrap_or(0.0);
        let rbsh = params_map.get("RBSH").copied().unwrap_or(0.0);
        let ideal_tie = rbody == 0.0 && rbsh == 0.0;

        let (node_body, node_p, body_mode) = if element.nodes.len() > 4 {
            let b = circuit.get_or_create_node(&element.nodes[4]);
            if ideal_tie {
                // Ideal body tie: the external contact is the body node.
                (b, b, BodyMode::TiedIdeal)
            } else {
                // Nonideal body tie: an internal body node sits behind the body
                // resistor; the external contact is the `p` node.
                let body = circuit.get_or_create_node(&format!("{}.__body.internal", element.name));
                (body, b, BodyMode::TiedResistive)
            }
        } else {
            // Floating body: allocate an internal body node.
            let body = circuit.get_or_create_node(&format!("{}.__body.internal", element.name));
            (body, 0, BodyMode::Floating)
        };

        let l = instance_param(instance_params, &["L"]).unwrap_or(0.0);
        let w = instance_param(instance_params, &["W"]).unwrap_or(0.0);
        let geom = B3SoiPdGeometry {
            l,
            w,
            drain_area: instance_param(instance_params, &["AD"]).unwrap_or(0.0),
            source_area: instance_param(instance_params, &["AS"]).unwrap_or(0.0),
            drain_squares: instance_param(instance_params, &["NRD"]).unwrap_or(1.0),
            source_squares: instance_param(instance_params, &["NRS"]).unwrap_or(1.0),
            drain_perimeter: instance_param(instance_params, &["PD"]).unwrap_or(0.0),
            source_perimeter: instance_param(instance_params, &["PS"]).unwrap_or(0.0),
            body_squares: instance_param(instance_params, &["NRB"]).unwrap_or(1.0),
            rth0: instance_param(instance_params, &["RTH0"])
                .or_else(|| params_map.get("RTH0").copied())
                .unwrap_or(0.0),
            cth0: instance_param(instance_params, &["CTH0"])
                .or_else(|| params_map.get("CTH0").copied())
                .unwrap_or(0.0),
            nseg: instance_param(instance_params, &["NSEG"]).unwrap_or(1.0),
            frbody: instance_param(instance_params, &["FRBODY"]).unwrap_or(1.0),
        };

        // B3SOIPD creates drain/source prime nodes when RSH and NRD/NRS are
        // positive (b3soipdset.c:1118-1159). The intrinsic device below is
        // evaluated at those primes; the fixed sheet resistance is an ordinary
        // linear resistor between the external terminal and its prime.
        let drain_resistance = model.sheet_resistance * geom.drain_squares;
        let node_drain = if drain_resistance.is_finite() && drain_resistance > 0.0 {
            let dint = circuit.get_or_create_node(&format!("{}.__dint", element.name));
            circuit.resistors.add(
                format!("{}.__rd", element.name),
                node_drain_external,
                dint,
                drain_resistance,
            );
            dint
        } else {
            node_drain_external
        };
        let source_resistance = model.sheet_resistance * geom.source_squares;
        let node_source = if source_resistance.is_finite() && source_resistance > 0.0 {
            let sint = circuit.get_or_create_node(&format!("{}.__sint", element.name));
            circuit.resistors.add(
                format!("{}.__rs", element.name),
                node_source_external,
                sint,
                source_resistance,
            );
            sint
        } else {
            node_source_external
        };
        let node_temp = if model.sh_mod == 1 && geom.rth0 != 0.0 {
            circuit.get_or_create_node(&format!("{}.__temp.internal", element.name))
        } else {
            0
        };
        let instance_ic = Self::native_b3soi_instance_ic(
            circuit,
            &element.name,
            "B3SOIPD",
            instance_params,
            deferred_params,
            node_drain,
            node_gate,
            node_source,
            node_body,
            node_e,
            node_p,
        )?;

        let mut device = B3SoiPd::new(
            element.name.clone(),
            node_drain,
            node_gate,
            node_source,
            node_e,
            node_body,
            node_p,
            node_temp,
            body_mode,
            model,
            geom,
            temp_k,
        )
        .map_err(SimulationError::Circuit)?;
        device.set_instance_ic(instance_ic);

        // DEBUG=-1 runs the device without dynamic charges (ngspice debugMod).
        if let Some(debug) =
            native_b3soi_debug_mod(&element.name, "B3SOIPD", instance_params, deferred_params)?
        {
            device.set_debug_mod(debug);
        }
        circuit.b3soi_pd.add(device);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn native_b3soi_instance_ic(
        circuit: &mut CircuitData,
        element_name: &str,
        family: &str,
        instance_params: &[(String, f64)],
        deferred_params: &[(String, String)],
        node_drain: crate::circuit::NodeId,
        node_gate: crate::circuit::NodeId,
        node_source: crate::circuit::NodeId,
        node_body: crate::circuit::NodeId,
        node_e: crate::circuit::NodeId,
        node_p: crate::circuit::NodeId,
    ) -> Result<crate::device::mosfet::b3soi::common::B3SoiInstanceIc, SimulationError> {
        for (name, expr) in deferred_params {
            if b3soi_ic_param_name(name) {
                return Err(SimulationError::Circuit(format!(
                    "MOSFET '{element_name}': native {family} instance initial condition {name}={expr} \
                     must resolve to a finite numeric value before circuit construction"
                )));
            }
        }

        let mut instance_ic = crate::device::mosfet::b3soi::common::B3SoiInstanceIc::new();
        if let Some(value) = native_b3soi_ic_value(element_name, family, "IC_VDS", instance_params)?
        {
            let branch = allocate_b3soi_ic_branch(
                circuit,
                element_name,
                family,
                "IC_VDS",
                value,
                node_drain,
                node_source,
            )?;
            if let Some(branch) = branch {
                instance_ic.set_vds(node_drain, node_source, value, branch);
            }
        }
        if let Some(value) = native_b3soi_ic_value(element_name, family, "IC_VGS", instance_params)?
        {
            let branch = allocate_b3soi_ic_branch(
                circuit,
                element_name,
                family,
                "IC_VGS",
                value,
                node_gate,
                node_source,
            )?;
            if let Some(branch) = branch {
                instance_ic.set_vgs(node_gate, node_source, value, branch);
            }
        }
        if let Some(value) = native_b3soi_ic_value(element_name, family, "IC_VBS", instance_params)?
        {
            let branch = allocate_b3soi_ic_branch(
                circuit,
                element_name,
                family,
                "IC_VBS",
                value,
                node_body,
                node_source,
            )?;
            if let Some(branch) = branch {
                instance_ic.set_vbs(node_body, node_source, value, branch);
            }
        }
        if let Some(value) = native_b3soi_ic_value(element_name, family, "IC_VES", instance_params)?
        {
            let branch = allocate_b3soi_ic_branch(
                circuit,
                element_name,
                family,
                "IC_VES",
                value,
                node_e,
                node_source,
            )?;
            if let Some(branch) = branch {
                instance_ic.set_ves(node_e, node_source, value, branch);
            }
        }
        if let Some(value) = native_b3soi_ic_value(element_name, family, "IC_VPS", instance_params)?
        {
            let branch = allocate_b3soi_ic_branch(
                circuit,
                element_name,
                family,
                "IC_VPS",
                value,
                node_p,
                node_source,
            )?;
            if let Some(branch) = branch {
                instance_ic.set_vps(node_p, node_source, value, branch);
            }
        }

        Ok(instance_ic)
    }

    /// Build and register a native BSIM3v3.3 (MOS level 8/9/49) instance.
    ///
    /// Topology is the standard 4-terminal bulk MOSFET `m d g s b`. Series
    /// drain/source resistance follows b3temp.c: a conductance of
    /// `1 / (RSH * NRD)` (resp. NRS) exists only when both factors are
    /// positive; it is lowered to an ordinary linear resistor of
    /// `RSH * NRD / M` ohms at an internal prime node, and the device's
    /// drain/source point at the primes (ngspice stamps `m *
    /// drainConductance` between dNode and dNodePrime, b3ld.c:3050).
    #[allow(clippy::too_many_arguments)]
    pub(super) fn build_bsim3v3(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        model_key: &str,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        deferred_params: &[(String, String)],
        temperature_kelvin: f64,
        tnom_default_k: f64,
        equation_set: crate::device::Bsim3v3EquationSet,
        shared: &mut HashMap<Bsim3v3SharedModelKey, Bsim3v3SharedModel>,
    ) -> Result<(), SimulationError> {
        use crate::device::Bsim3v3Device;
        use crate::device::mosfet::bsim3v3::{
            Bsim3v3, Bsim3v3Geometry, Bsim3v3Model, Bsim3v3ModelTemp, SizeDepCache,
        };

        let is_pmos = matches!(mos_type, crate::netlist::MosType::Pmos);
        // BSIM3v3.3 has no instance TEMP/DTEMP (b3set.c); every instance
        // evaluates at the circuit temperature, like ngspice's CKTtemp.
        let temp_k = temperature_kelvin;

        let shared_key = Bsim3v3SharedModelKey {
            model_name: model_key.to_string(),
            equation_set,
        };
        let entry = match shared.entry(shared_key) {
            std::collections::hash_map::Entry::Occupied(occupied) => occupied.into_mut(),
            std::collections::hash_map::Entry::Vacant(vacant) => {
                let model = std::sync::Arc::new(
                    Bsim3v3Model::try_from_params_with_equation_set(
                        params_map,
                        is_pmos,
                        tnom_default_k,
                        equation_set,
                    )
                    .map_err(|message| {
                        SimulationError::Circuit(format!(
                            "MOSFET '{}': BSIM3 model '{}': {message}",
                            element.name, model_key
                        ))
                    })?,
                );
                let model_temp = std::sync::Arc::new(Bsim3v3ModelTemp::new(&model, temp_k));
                vacant.insert(Bsim3v3SharedModel {
                    model,
                    model_temp,
                    size_cache: SizeDepCache::new(),
                })
            }
        };

        let multiplier = native_mos_instance_multiplier(&element.name, "BSIM3", instance_params)?;
        let (nqs_mod, _) = native_mos_instance_integer_selector_reset_to_default(
            &element.name,
            "BSIM3",
            instance_params,
            deferred_params,
            "NQSMOD",
            entry.model.nqs_mod,
            &[0, 1],
        )?;
        let (acnqs_mod, _) = native_mos_instance_integer_selector_reset_to_default(
            &element.name,
            "BSIM3",
            instance_params,
            deferred_params,
            "ACNQSMOD",
            entry.model.acnqs_mod,
            &[0, 1],
        )?;
        let effective_model =
            if nqs_mod != entry.model.nqs_mod || acnqs_mod != entry.model.acnqs_mod {
                let mut model = (*entry.model).clone();
                model.nqs_mod = nqs_mod;
                model.acnqs_mod = acnqs_mod;
                std::sync::Arc::new(model)
            } else {
                std::sync::Arc::clone(&entry.model)
            };
        let defaults = Bsim3v3Geometry::default();
        let geom = Bsim3v3Geometry {
            l: instance_param(instance_params, &["L"]).unwrap_or(defaults.l),
            w: instance_param(instance_params, &["W"]).unwrap_or(defaults.w),
            m: multiplier,
            drain_area: instance_param(instance_params, &["AD"]).unwrap_or(0.0),
            source_area: instance_param(instance_params, &["AS"]).unwrap_or(0.0),
            drain_squares: instance_param(instance_params, &["NRD"])
                .unwrap_or(defaults.drain_squares),
            source_squares: instance_param(instance_params, &["NRS"])
                .unwrap_or(defaults.source_squares),
            drain_perimeter: instance_param(instance_params, &["PD"]).unwrap_or(0.0),
            source_perimeter: instance_param(instance_params, &["PS"]).unwrap_or(0.0),
            delvto: instance_param(instance_params, &["DELVTO", "DELVT0"]).unwrap_or(0.0),
            mulu0: instance_param(instance_params, &["MULU0"]).unwrap_or(1.0),
            ..defaults
        };

        let core = Bsim3v3::new_shared(
            element.name.clone(),
            effective_model,
            std::sync::Arc::clone(&entry.model_temp),
            &mut entry.size_cache,
            geom,
        )
        .map_err(SimulationError::Circuit)?;

        let drain_external = circuit.get_or_create_node(&element.nodes[0]);
        let gate = circuit.get_or_create_node(&element.nodes[1]);
        let source_external = circuit.get_or_create_node(&element.nodes[2]);
        let bulk_external = circuit.get_or_create_node(&element.nodes[3]);

        // Internal prime nodes only when the series conductance exists
        // (drain_conductance = 1/(RSH*NRD) > 0, b3temp.c:811-851).
        let drain = if core.inst.drain_conductance > 0.0 {
            let dint = circuit.get_or_create_node(&format!("{}.__dint", element.name));
            circuit.resistors.add(
                format!("{}.__rd", element.name),
                drain_external,
                dint,
                1.0 / (core.inst.drain_conductance * multiplier),
            );
            dint
        } else {
            drain_external
        };
        let source = if core.inst.source_conductance > 0.0 {
            let sint = circuit.get_or_create_node(&format!("{}.__sint", element.name));
            circuit.resistors.add(
                format!("{}.__rs", element.name),
                source_external,
                sint,
                1.0 / (core.inst.source_conductance * multiplier),
            );
            sint
        } else {
            source_external
        };
        let charge_deficit = if core.model.nqs_mod != 0 {
            circuit.get_or_create_node(&format!("{}.__charge", element.name))
        } else {
            0
        };

        circuit.bsim3v3.add(Bsim3v3Device::new(
            element.name.clone(),
            drain,
            gate,
            source,
            bulk_external,
            charge_deficit,
            multiplier,
            core,
        ));
        Ok(())
    }

    /// Build and register the native VDMOS power MOSFET.
    ///
    /// The same internal device backs Xyce MOS LEVEL=18 and ngspice's
    /// `.model ... VDMOS` compatibility syntax. The current native core is a
    /// source-referenced power MOSFET; Xyce's explicit bulk terminal is parsed
    /// and retained as circuit syntax, but the VDMOS conduction model itself
    /// uses source/body-referenced equations.
    pub(super) fn build_vdmos(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        circuit_temperature_kelvin: f64,
        default_tnom_kelvin: f64,
    ) -> Result<(), SimulationError> {
        if element.nodes.len() < 4 {
            return Err(SimulationError::Circuit(format!(
                "VDMOS '{}' requires drain, gate, source, and model/body syntax",
                element.name
            )));
        }

        let drain = circuit.get_or_create_node(&element.nodes[0]);
        let gate = circuit.get_or_create_node(&element.nodes[1]);
        let source = circuit.get_or_create_node(&element.nodes[2]);
        let _bulk = circuit.get_or_create_node(&element.nodes[3]);

        let mut vdmos = match mos_type {
            crate::netlist::MosType::Nmos => {
                crate::device::Vdmos::new_nvdmos(element.name.clone(), drain, gate, source)
            }
            crate::netlist::MosType::Pmos => {
                crate::device::Vdmos::new_pvdmos(element.name.clone(), drain, gate, source)
            }
        }
        .with_params(params_map)
        .with_instance_params(params_map, instance_params);
        let tnom_kelvin = params_map
            .get("TNOM")
            .copied()
            .map(crate::analysis::temperature::celsius_to_kelvin)
            .unwrap_or(default_tnom_kelvin);
        let temp_kelvin = if let Some(temp_celsius) = instance_param(instance_params, &["TEMP"]) {
            crate::analysis::temperature::celsius_to_kelvin(temp_celsius)
        } else if let Some(dtemp) = instance_param(instance_params, &["DTEMP"]) {
            circuit_temperature_kelvin + dtemp
        } else {
            circuit_temperature_kelvin
        };
        vdmos.set_temperature(temp_kelvin, tnom_kelvin);
        vdmos.set_bulk_node(_bulk);

        let has_xyce_drift = vdmos.xyce_level18
            && (vdmos.xyce_drift_param_a > 0.0 || vdmos.xyce_drift_param_b > 0.0);
        let has_drain_resistance = vdmos.rd.is_finite() && vdmos.rd > 1e-12;
        let drain_drift = if has_xyce_drift {
            if has_drain_resistance {
                circuit.get_or_create_node(&format!("{}.__ddrift", element.name))
            } else {
                circuit.get_or_create_node(&format!("{}.__dint", element.name))
            }
        } else {
            drain
        };
        let drain_int = if has_drain_resistance {
            circuit.get_or_create_node(&format!("{}.__dint", element.name))
        } else if has_xyce_drift {
            drain_drift
        } else {
            drain
        };
        let source_int = if vdmos.rs.is_finite() && vdmos.rs > 1e-12 {
            circuit.get_or_create_node(&format!("{}.__sint", element.name))
        } else {
            source
        };
        let d1_prime = if vdmos.d1_rs.is_finite() && vdmos.d1_rs > 1e-12 {
            circuit.get_or_create_node(&format!("{}.__d1prime", element.name))
        } else {
            source
        };

        if drain_int != drain || source_int != source {
            vdmos.set_internal_nodes(drain_int, source_int);
        }
        if drain_drift != drain {
            vdmos.set_drain_drift_node(drain_drift);
        }
        if d1_prime != source {
            vdmos.set_d1_prime_node(d1_prime);
        }

        circuit.vdmoses.add(vdmos);
        Ok(())
    }

    /// Build and register a native BSIM4 v4.8 (MOS level 14/54) instance.
    ///
    /// Topology is the standard 4-terminal bulk MOSFET `m d g s b` (the
    /// canonical mode set collapses every optional internal node of
    /// b4set.c). Fixed series drain/source resistance follows b4temp.c:
    /// explicit `NRD`/`NRS` wins, otherwise `RGEOMOD=1..8` may derive the
    /// resistance from implicit S/D geometry when the square count is not
    /// given. The conductance is lowered to an ordinary linear resistor at
    /// an internal prime node, and the device's drain/source point at the
    /// primes (ngspice stamps `m * drainConductance` between dNode and
    /// dNodePrime). `RDSMOD=1` forces prime nodes and stamps its
    /// bias-dependent external branches in the BSIM4 device. `RGATEMOD=1`
    /// similarly lowers to a linear external-gate resistor and routes the
    /// intrinsic device through a gate-prime node; `RGATEMOD=2` allocates the
    /// same prime node but lets the BSIM4 device stamp the bias-dependent
    /// gate-resistance branch natively. `RGATEMOD=3` additionally allocates a
    /// middle-gate node for the native gate network. `RBODYMOD=1/2` allocates
    /// the body-prime, drain-body, and source-body nodes used by the native DC
    /// substrate network. Unsupported NQS combinations surface the module's
    /// typed construction error.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn build_bsim4v8(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        model_key: &str,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        temperature_kelvin: f64,
        tnom_default_k: f64,
        shared: &mut HashMap<String, Bsim4v8SharedModel>,
    ) -> Result<(), SimulationError> {
        use crate::device::Bsim4v8Device;
        use crate::device::mosfet::bsim4v8::{
            Bsim4v8, Bsim4v8Geometry, Bsim4v8Model, Bsim4v8ModelTemp, SizeDepCache,
        };

        let is_pmos = matches!(mos_type, crate::netlist::MosType::Pmos);
        // ngspice-46's BSIM4 parses an instance DTEMP but never uses it
        // (see the module docs); every instance evaluates at the circuit
        // temperature, like ngspice's CKTtemp.
        let temp_k = temperature_kelvin;

        let entry = match shared.entry(model_key.to_string()) {
            std::collections::hash_map::Entry::Occupied(occupied) => occupied.into_mut(),
            std::collections::hash_map::Entry::Vacant(vacant) => {
                let model = std::sync::Arc::new(
                    Bsim4v8Model::try_from_params(params_map, is_pmos, tnom_default_k).map_err(
                        |message| {
                            SimulationError::Circuit(format!(
                                "MOSFET '{}': BSIM4 model '{}': {message}",
                                element.name, model_key
                            ))
                        },
                    )?,
                );
                // The charge model implements CAPMOD=0/1/2 (including the
                // BSIM4 default CAPMOD=2) with integer CVCHARGEMOD=0/1/2/3.
                // Reject unknown CVCHARGEMOD selectors up front when intrinsic charge is active;
                // XPART<0 (intrinsic charge suppression) remains honored.
                if !model.cvcharge_mod_supported_for_charges() && model.xpart >= 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "MOSFET '{}': BSIM4 model '{}' requests CVCHARGEMOD={} which is \
                         not implemented (only integer CVCHARGEMOD=0, 1, 2, or 3)",
                        element.name, model_key, model.cvcharge_mod_value
                    )));
                }
                let model_temp = std::sync::Arc::new(Bsim4v8ModelTemp::new(&model, temp_k));
                vacant.insert(Bsim4v8SharedModel {
                    model,
                    model_temp,
                    size_cache: SizeDepCache::new(),
                })
            }
        };

        let multiplier = native_mos_instance_multiplier(&element.name, "BSIM4", instance_params)?;
        let defaults = Bsim4v8Geometry::default();
        let given = |names: &[&str]| instance_param(instance_params, names);
        let (geo_mod, geo_mod_given) = native_bsim4v8_instance_selector(
            &element.name,
            "BSIM4",
            instance_params,
            "GEOMOD",
            defaults.geo_mod,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        )?;
        let (rgeo_mod, rgeo_mod_given) = native_bsim4v8_instance_selector(
            &element.name,
            "BSIM4",
            instance_params,
            "RGEOMOD",
            defaults.rgeo_mod,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8],
        )?;
        let (min_sd, _) = native_bsim4v8_instance_selector(
            &element.name,
            "BSIM4",
            instance_params,
            "MIN",
            defaults.min_sd,
            &[0, 1],
        )?;
        let geom = Bsim4v8Geometry {
            l: given(&["L"]).unwrap_or(defaults.l),
            w: given(&["W"]).unwrap_or(defaults.w),
            nf: given(&["NF"])
                .filter(|nf| nf.is_finite() && *nf >= 1.0)
                .unwrap_or(defaults.nf),
            m: multiplier,
            geo_mod,
            geo_mod_given,
            rgeo_mod,
            rgeo_mod_given,
            min_sd,
            drain_area: given(&["AD"]).unwrap_or(0.0),
            drain_area_given: given(&["AD"]).is_some(),
            source_area: given(&["AS"]).unwrap_or(0.0),
            source_area_given: given(&["AS"]).is_some(),
            drain_perimeter: given(&["PD"]).unwrap_or(0.0),
            drain_perimeter_given: given(&["PD"]).is_some(),
            source_perimeter: given(&["PS"]).unwrap_or(0.0),
            source_perimeter_given: given(&["PS"]).is_some(),
            drain_squares: given(&["NRD"]).unwrap_or(defaults.drain_squares),
            drain_squares_given: given(&["NRD"]).is_some(),
            source_squares: given(&["NRS"]).unwrap_or(defaults.source_squares),
            source_squares_given: given(&["NRS"]).is_some(),
            delvto: given(&["DELVTO", "DELVT0"]).unwrap_or(0.0),
            mulu0: given(&["MULU0"]).unwrap_or(1.0),
            dtemp: given(&["DTEMP"]).unwrap_or(0.0),
            sa: given(&["SA"]).unwrap_or(0.0),
            sb: given(&["SB"]).unwrap_or(0.0),
            sd: given(&["SD"]).unwrap_or(0.0),
            sc: given(&["SC"]).unwrap_or(0.0),
            sc_given: given(&["SC"]).is_some(),
            sca: given(&["SCA"]).unwrap_or(0.0),
            sca_given: given(&["SCA"]).is_some(),
            scb: given(&["SCB"]).unwrap_or(0.0),
            scb_given: given(&["SCB"]).is_some(),
            scc: given(&["SCC"]).unwrap_or(0.0),
            scc_given: given(&["SCC"]).is_some(),
            off: given(&["OFF"]).is_some_and(|v| v != 0.0),
            ic_vds: given(&["ICVDS", "IC"]).unwrap_or(0.0),
            ic_vgs: given(&["ICVGS"]).unwrap_or(0.0),
            ic_vbs: given(&["ICVBS"]).unwrap_or(0.0),
        };

        let core = Bsim4v8::new_shared(
            element.name.clone(),
            std::sync::Arc::clone(&entry.model),
            std::sync::Arc::clone(&entry.model_temp),
            &mut entry.size_cache,
            geom,
        )
        .map_err(SimulationError::Circuit)?;

        let drain_external = circuit.get_or_create_node(&element.nodes[0]);
        let gate_external = circuit.get_or_create_node(&element.nodes[1]);
        let source_external = circuit.get_or_create_node(&element.nodes[2]);
        let bulk_external = circuit.get_or_create_node(&element.nodes[3]);

        // RDSMOD=1 forces prime nodes and stamps the nonlinear external
        // branches inside the BSIM4 device. RDSMOD=0 keeps the older lowering
        // of fixed RSH*NRD/NRS conductance to ordinary linear resistors.
        let rds_mod = core.model.rds_mod == 1;
        let drain = if rds_mod {
            circuit.get_or_create_node(&format!("{}.__dint", element.name))
        } else if core.inst.drain_conductance > 0.0 {
            let dint = circuit.get_or_create_node(&format!("{}.__dint", element.name));
            circuit.resistors.add(
                format!("{}.__rd", element.name),
                drain_external,
                dint,
                1.0 / (core.inst.drain_conductance * multiplier),
            );
            dint
        } else {
            drain_external
        };
        let source = if rds_mod {
            circuit.get_or_create_node(&format!("{}.__sint", element.name))
        } else if core.inst.source_conductance > 0.0 {
            let sint = circuit.get_or_create_node(&format!("{}.__sint", element.name));
            circuit.resistors.add(
                format!("{}.__rs", element.name),
                source_external,
                sint,
                1.0 / (core.inst.source_conductance * multiplier),
            );
            sint
        } else {
            source_external
        };
        let (gate_mid, gate) = match core.model.rgate_mod {
            1 => {
                let gint = circuit.get_or_create_node(&format!("{}.__gint", element.name));
                circuit.resistors.add(
                    format!("{}.__rg", element.name),
                    gate_external,
                    gint,
                    1.0 / (core.inst.gate_conductance * multiplier),
                );
                (gate_external, gint)
            }
            2 => {
                let gint = circuit.get_or_create_node(&format!("{}.__gint", element.name));
                (gate_external, gint)
            }
            3 => {
                let gmid = circuit.get_or_create_node(&format!("{}.__gmid", element.name));
                let gint = circuit.get_or_create_node(&format!("{}.__gint", element.name));
                circuit.resistors.add(
                    format!("{}.__rg", element.name),
                    gate_external,
                    gmid,
                    1.0 / (core.inst.gate_conductance * multiplier),
                );
                (gmid, gint)
            }
            _ => (gate_external, gate_external),
        };
        let rbody_mod = core.model.rbody_mod != 0;
        let bulk = if rbody_mod {
            circuit.get_or_create_node(&format!("{}.__body", element.name))
        } else {
            bulk_external
        };
        let drain_body = if rbody_mod {
            circuit.get_or_create_node(&format!("{}.__dbody", element.name))
        } else {
            bulk
        };
        let source_body = if rbody_mod {
            circuit.get_or_create_node(&format!("{}.__sbody", element.name))
        } else {
            bulk
        };
        let charge_deficit = if core.model.trnqs_mod != 0 {
            circuit.get_or_create_node(&format!("{}.__charge", element.name))
        } else {
            0
        };

        circuit.bsim4v8.add(Bsim4v8Device::new(
            element.name.clone(),
            drain_external,
            drain,
            gate_external,
            gate_mid,
            gate,
            source_external,
            source,
            bulk_external,
            bulk,
            drain_body,
            source_body,
            charge_deficit,
            multiplier,
            core,
        ));
        Ok(())
    }

    pub(super) fn build_ekv26(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        model_key: &str,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        circuit_temperature_kelvin: f64,
    ) -> Result<(), SimulationError> {
        if element.nodes.len() != 4 {
            return Err(SimulationError::Circuit(format!(
                "MOSFET '{}': EKV26 LEVEL=260 native DC slice requires exactly four terminals (drain, gate, source, bulk)",
                element.name
            )));
        }

        let device_mos_type = match mos_type {
            crate::netlist::MosType::Nmos => crate::device::MosType::Nmos,
            crate::netlist::MosType::Pmos => crate::device::MosType::Pmos,
        };
        let drain = circuit.get_or_create_node(&element.nodes[0]);
        let gate = circuit.get_or_create_node(&element.nodes[1]);
        let source = circuit.get_or_create_node(&element.nodes[2]);
        let bulk = circuit.get_or_create_node(&element.nodes[3]);
        let device = crate::device::EkvMosfet::from_params(
            element.name.clone(),
            drain,
            gate,
            source,
            bulk,
            device_mos_type,
            params_map,
            instance_params,
            circuit_temperature_kelvin,
        )
        .map_err(|error| {
            SimulationError::Circuit(format!(
                "MOSFET '{}': model '{}' {}",
                element.name, model_key, error
            ))
        })?;
        circuit.ekv26s.add(device);
        Ok(())
    }

    pub(super) fn build_ekv3(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        model_key: &str,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        temperature_kelvin: f64,
    ) -> Result<(), SimulationError> {
        if element.nodes.len() != 4 {
            return Err(SimulationError::Circuit(format!(
                "MOSFET '{}': EKV3 LEVEL=301 native NMOS150 slice requires exactly four terminals (drain, gate, source, bulk)",
                element.name
            )));
        }

        let device_mos_type = match mos_type {
            crate::netlist::MosType::Nmos => crate::device::MosType::Nmos,
            crate::netlist::MosType::Pmos => crate::device::MosType::Pmos,
        };
        let drain = circuit.get_or_create_node(&element.nodes[0]);
        let gate = circuit.get_or_create_node(&element.nodes[1]);
        let source = circuit.get_or_create_node(&element.nodes[2]);
        let bulk = circuit.get_or_create_node(&element.nodes[3]);
        let device = crate::device::Ekv3Device::from_params(
            element.name.clone(),
            drain,
            gate,
            source,
            bulk,
            device_mos_type,
            params_map,
            instance_params,
            temperature_kelvin,
        )
        .map_err(|error| {
            SimulationError::Circuit(format!(
                "MOSFET '{}': model '{}' {}",
                element.name, model_key, error
            ))
        })?;
        circuit.ekv3s.add(device);
        Ok(())
    }
}

fn b3soi_ic_param_name(name: &str) -> bool {
    matches!(
        name.to_ascii_uppercase().as_str(),
        "IC_VDS" | "IC_VGS" | "IC_VBS" | "IC_VES" | "IC_VPS"
    )
}

fn native_b3soi_ic_value(
    element_name: &str,
    family: &str,
    name: &str,
    instance_params: &[(String, f64)],
) -> Result<Option<f64>, SimulationError> {
    let Some(value) = instance_param(instance_params, &[name]) else {
        return Ok(None);
    };
    if value.is_finite() {
        Ok(Some(value))
    } else {
        Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native {family} instance initial condition {name}={value} \
             must be finite"
        )))
    }
}

fn allocate_b3soi_ic_branch(
    circuit: &mut CircuitData,
    element_name: &str,
    family: &str,
    name: &str,
    value: f64,
    node_pos: crate::circuit::NodeId,
    node_neg: crate::circuit::NodeId,
) -> Result<Option<crate::circuit::NodeId>, SimulationError> {
    if node_pos == node_neg {
        if value.abs() <= 1.0e-15 {
            return Ok(None);
        }
        return Err(SimulationError::Circuit(format!(
            "MOSFET '{element_name}': native {family} instance initial condition {name}={value} \
             cannot constrain identical nodes to a nonzero voltage"
        )));
    }

    Ok(Some(circuit.allocate_branch()))
}

/// Per-`.model` shared BSIM3v3.3 state: the parsed card, its temperature
/// block, and the (W, L)-keyed size-dependent parameter knots.
pub(super) struct Bsim3v3SharedModel {
    model: std::sync::Arc<crate::device::Bsim3v3Model>,
    model_temp: std::sync::Arc<crate::device::mosfet::bsim3v3::Bsim3v3ModelTemp>,
    size_cache: crate::device::mosfet::bsim3v3::SizeDepCache,
}

/// Semantic identity of a shared BSIM3 model card. The equation family is
/// part of the key so a compatibility front can never reuse temperature or
/// size-dependent state prepared for another canonical model revision.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct Bsim3v3SharedModelKey {
    model_name: String,
    equation_set: crate::device::Bsim3v3EquationSet,
}

/// Per-`.model` shared BSIM4 v4.8 state: the parsed card, its temperature
/// block, and the (W, L, NF)-keyed size-dependent parameter knots.
pub(super) struct Bsim4v8SharedModel {
    model: std::sync::Arc<crate::device::Bsim4v8Model>,
    model_temp: std::sync::Arc<crate::device::mosfet::bsim4v8::Bsim4v8ModelTemp>,
    size_cache: crate::device::mosfet::bsim4v8::SizeDepCache,
}
