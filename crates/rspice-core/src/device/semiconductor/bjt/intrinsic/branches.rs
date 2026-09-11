//! Intrinsic branch current and derivative builders.

use super::*;

impl Bjt {
    pub(in crate::device::semiconductor::bjt) fn intrinsic_terminal_derivatives(
        &self,
        linearized: BjtLinearization,
    ) -> (
        [Value; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
    ) {
        let mut collector = [0.0; INTERNAL_DIM];
        collector[IDX_VCI] = -linearized.dic_dvbc;
        collector[IDX_VBI] = linearized.dic_dvbe + linearized.dic_dvbc;
        collector[IDX_VEI] = -linearized.dic_dvbe;
        collector[IDX_VRTH] = linearized.dic_dvrth;

        let mut base = [0.0; INTERNAL_DIM];
        base[IDX_VCI] = -linearized.dib_dvbc;
        base[IDX_VBI] = linearized.dib_dvbe + linearized.dib_dvbc;
        base[IDX_VEI] = -linearized.dib_dvbe;
        base[IDX_VRTH] = linearized.dib_dvrth;

        let mut emitter = [0.0; INTERNAL_DIM];
        for idx in 0..INTERNAL_DIM {
            emitter[idx] = -(collector[idx] + base[idx]);
        }

        (collector, base, emitter)
    }

    pub(in crate::device::semiconductor::bjt) fn ircx_branch(
        &self,
        vc: Value,
        vcx: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rcx) {
            return branch;
        }

        let g = 1.0 / self.guarded_series_resistance(self.rcx);
        branch.current = g * (vc - vcx);
        branch.d_internal[IDX_VCX] = -g;
        branch.d_external[0] = g;
        branch
    }

    /// Xyce VBIC 1.3 `avalm` and its voltage derivative, including the
    /// finite lower bound on the effective reverse bias.
    pub(in crate::device::semiconductor::bjt) fn vbic13_avalanche_factor(
        &self,
        voltage: Value,
        potential: Value,
        grading: Value,
        av1: Value,
        av2: Value,
    ) -> (Value, Value) {
        if av1 <= 0.0 {
            return (0.0, 0.0);
        }
        let minimum = (0.02 * (av2 + 1.0)).powf(1.0 / (1.01 - grading));
        let delta = potential - voltage - minimum;
        let root = delta.hypot(0.1);
        let (bias, slope) = if delta >= 0.0 {
            (minimum + 0.5 * (root + delta), -0.5 * (1.0 + delta / root))
        } else {
            // Rationalize the subtraction in the forward-biased tail.
            (
                minimum + 0.005 / (root - delta),
                -0.005 / (root * (root - delta)),
            )
        };
        let power = grading - 1.0;
        let argument = -av2 * bias.powf(power);
        let argument_slope = -av2 * power * bias.powf(power - 1.0) * slope;
        let (exponential, exponential_slope) = self.vbic_general_exp(argument);
        (
            av1 * bias * exponential,
            av1 * (slope * exponential + bias * exponential_slope * argument_slope),
        )
    }

    /// Extrinsic base-to-collector branch. This is separate from Ibc and
    /// Ibep, and remains present as a GMIN parallel with AVCX1=0.
    pub(in crate::device::semiconductor::bjt) fn igcx_branch(
        &self,
        vc: Value,
        vcx: Value,
        vbx: Value,
    ) -> BranchLinearization {
        if !self.vbic_13 {
            return BranchLinearization::default();
        }
        let polarity = self.polarity();
        let gmin = self.nonlinear_branch_gmin();
        let (factor, slope) = self.vbic13_avalanche_factor(
            polarity * (vbx - vcx),
            0.0,
            self.mcx,
            self.avcx1,
            self.avcx2,
        );
        let ircx = self.ircx_branch(vc, vcx);
        // vbic_1p3.va uses physical Ircx here, then applies VBICtype to
        // the completed Igcx branch, including for PNP instances.
        let mut branch = Self::scale_branch(ircx, -polarity * factor);
        branch.current += gmin * (vbx - vcx);
        branch.d_internal[IDX_VBX] += gmin - ircx.current * slope;
        branch.d_internal[IDX_VCX] -= gmin - ircx.current * slope;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn irbx_branch(
        &self,
        vb: Value,
        vbx: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbx) {
            return branch;
        }

        let g = 1.0 / self.guarded_series_resistance(self.rbx);
        branch.current = g * (vb - vbx);
        branch.d_internal[IDX_VBX] = -g;
        branch.d_external[1] = g;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn ire_branch(
        &self,
        ve: Value,
        vei: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.re) {
            return branch;
        }

        let g = 1.0 / self.guarded_series_resistance(self.re);
        branch.current = g * (ve - vei);
        branch.d_internal[IDX_VEI] = -g;
        branch.d_external[2] = g;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn irbi_branch(
        &self,
        linearized: BjtLinearization,
        vbx: Value,
        vbi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbi) {
            return branch;
        }

        let rb = self.guarded_series_resistance(self.rbi);
        let vrbi = vbx - vbi;
        if self.charge_model == BjtChargeModel::LegacyGummelPoon {
            // Xyce's legacy GP load evaluates the bias-dependent base
            // resistance conductance at the current operating point, then
            // holds that conductance fixed in the Newton Jacobian
            // (N_DEV_BJT.C: diBrdvCp/diBrdvEp are zero).  Do not feed dQB/dV
            // (or dIB/dV when IRB is present) back into this branch; the
            // compact-model junction derivatives already carry the complete
            // GP charge dependence.
            let conductance = self.legacy_gp_base_resistance(linearized, rb).recip();
            branch.current = conductance * vrbi;
            branch.d_internal[IDX_VBX] = conductance;
            branch.d_internal[IDX_VBI] = -conductance;
            return branch;
        }

        // VBIC has no legacy IRB path; its conductance remains qb/rbi and
        // retains the existing qB derivatives in its Jacobian.
        let qb = linearized.qb.max(1e-12);
        let scale = vrbi / rb;
        let dqb_dvbi = linearized.dqb_dvbe + linearized.dqb_dvbc;
        let dqb_dvci = -linearized.dqb_dvbc;
        let dqb_dvei = -linearized.dqb_dvbe;
        branch.current = scale * qb;
        branch.d_internal[IDX_VBX] = qb / rb;
        branch.d_internal[IDX_VBI] = -qb / rb + scale * dqb_dvbi;
        branch.d_internal[IDX_VCI] = scale * dqb_dvci;
        branch.d_internal[IDX_VEI] = scale * dqb_dvei;
        branch
    }

    /// Return the physical legacy Gummel-Poon intrinsic base resistance.
    ///
    /// Xyce's `N_DEV_BJT.C` computes the conductance from the operating-point
    /// base charge when IRB/JRB/IOB is absent.  When one of those aliases is
    /// given, it uses the analytic high-current base-spreading law from the
    /// original SPICE BJT model:
    ///
    /// `R(I_B) = R_BM + 3 (R_B - R_BM) (tan(z)-z)/(z tan(z)^2)`
    ///
    /// with `z = (-1 + sqrt(1 + 14.59025 I_B/I_RB)) /
    /// (2.4317 sqrt(I_B/I_RB))`.  Xyce freezes this operating-point
    /// conductance in the Newton Jacobian, so only the voltage-difference
    /// derivatives are returned by `irbi_branch`.
    #[inline]
    pub(in crate::device::semiconductor::bjt) fn legacy_gp_base_resistance(
        &self,
        linearized: BjtLinearization,
        rb: Value,
    ) -> Value {
        self.legacy_gp_base_resistance_law::<false>(linearized, rb)
            .current
    }

    /// The physical resistance law also supplies exact derivatives for HB.
    /// Native SPICE Newton/AC retains its documented frozen-resistance load.
    #[inline]
    pub(in crate::device::semiconductor::bjt) fn legacy_gp_base_resistance_law<
        const EXACT: bool,
    >(
        &self,
        linearized: BjtLinearization,
        rb: Value,
    ) -> BranchLinearization {
        let [whole, minimum] = self
            .legacy_junction_params
            .as_ref()
            .and_then(|j| j.base_resistance)
            .map_or([rb, 0.0], |r| r.operating);
        let mut partials = [0.0; 3];
        let factor = if self.irb > 0.0 {
            let current = self.polarity() * linearized.ib;
            if !current.is_finite() {
                return BranchLinearization {
                    current: Value::NAN,
                    ..Default::default()
                };
            }
            let raw_ratio = current / self.irb;
            let ratio = raw_ratio.max(1e-9);
            let root = ratio.sqrt();
            let z = if ratio < 1.0 {
                14.59025 * root / (2.4317 * ((1.0 + 14.59025 * ratio).sqrt() + 1.0))
            } else {
                ((14.59025 + ratio.recip()).sqrt() - root.recip()) / 2.4317
            };
            if EXACT && raw_ratio > 1e-9 {
                // z*dF/dz avoids division by a small z. Its series prevents
                // cancellation in the cotangent form near zero; below the
                // native small-z boundary differentiate that exact polynomial.
                let square = z * z;
                let logarithmic_slope = if z.abs() < 1e-3 {
                    -square * (8.0 / 15.0 + square * 16.0 / 105.0)
                } else if z.abs() < 0.125 {
                    -square
                        * (8.0 / 15.0
                            + square
                                * (16.0 / 105.0
                                    + square
                                        * (16.0 / 525.0
                                            + square
                                                * (32.0 / 6237.0
                                                    + square
                                                        * (11056.0 / 14189175.0
                                                            + square * 32.0 / 289575.0)))))
                } else {
                    let cotangent = z.tan().recip();
                    3.0 * ((2.0 * z * cotangent - 1.0) * (1.0 + cotangent * cotangent)
                        - cotangent / z)
                };
                let dlogz_dlogr = if ratio < 1.0 {
                    0.5 / (1.0 + 14.59025 * ratio).sqrt()
                } else {
                    (0.5 / root) / (14.59025 + ratio.recip()).sqrt()
                };
                let scale = (whole - minimum) * logarithmic_slope * dlogz_dlogr;
                partials = [
                    linearized.dib_dvbe,
                    linearized.dib_dvbc,
                    linearized.dib_dvrth,
                ]
                .map(|derivative| scale * (derivative / linearized.ib));
            }
            if z.abs() < 1e-3 {
                let square = z * z;
                1.0 - square * (4.0 / 15.0 + square * 4.0 / 105.0)
            } else {
                let tangent = z.tan();
                3.0 * (tangent - z) / (z * tangent * tangent)
            }
        } else {
            let qb = linearized.qb.max(1e-12);
            if EXACT && linearized.qb > 1e-12 {
                let scale = -(whole - minimum) / qb;
                partials = [
                    linearized.dqb_dvbe,
                    linearized.dqb_dvbc,
                    linearized.dqb_dvrth,
                ]
                .map(|derivative| scale * (derivative / qb));
            }
            qb.recip()
        };
        let current = if (0.0..=1.0).contains(&factor) {
            whole * factor + minimum * (1.0 - factor)
        } else {
            minimum + (whole - minimum) * factor
        };
        let mut result = BranchLinearization {
            current,
            ..Default::default()
        };
        if EXACT {
            result.d_internal[IDX_VBI] = partials[0] + partials[1];
            result.d_internal[IDX_VCI] = -partials[1];
            result.d_internal[IDX_VEI] = -partials[0];
            result.d_internal[IDX_VRTH] = partials[2];
        }
        result
    }

    pub(in crate::device::semiconductor::bjt) fn ibep_branch(
        &self,
        vbx: Value,
        vbp: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        // VBIC's parasitic base-emitter branch does not exist in the legacy
        // Gummel-Poon topology. Its unconditional CKTgmin parallel would
        // otherwise connect the collapsed legacy base to the collector-side
        // parasitic node and create a second reverse-junction leakage path.
        if self.charge_model == BjtChargeModel::LegacyGummelPoon {
            return branch;
        }
        // ngspice vbicload.c stamps the `CKTgmin` parallel on Vbep
        // unconditionally, even when the parasitic diode currents are zero.
        let gmin = self.nonlinear_branch_gmin();
        branch.current = gmin * (vbx - vbp);
        branch.d_internal[IDX_VBX] = gmin;
        branch.d_internal[IDX_VBP] = -gmin;
        if self.ibeip <= 0.0 && self.ibenp <= 0.0 {
            return branch;
        }

        let p = self.polarity();
        let vbep_eff = p * (vbx - vbp);
        let (ibeip, gbeip) = self.vbic_diode_iv(
            self.ibeip,
            vbep_eff,
            self.nci,
            self.vbic_junction_limits.ibeip,
        );
        let (ibenp, gbenp) = self.vbic_diode_iv(
            self.ibenp,
            vbep_eff,
            self.ncn,
            self.vbic_junction_limits.ibenp,
        );
        let gbep = gbeip + gbenp;

        branch.current += p * (ibeip + ibenp);
        branch.d_internal[IDX_VBX] += gbep;
        branch.d_internal[IDX_VBP] -= gbep;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn parasitic_transport_state(
        &self,
        vbx: Value,
        vbi: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> ParasiticTransportState {
        let mut state = ParasiticTransportState {
            qbp: 1.0,
            d_qbp: [0.0; INTERNAL_DIM],
            ifp: 0.0,
            d_ifp: [0.0; INTERNAL_DIM],
            irp: 0.0,
            d_irp: [0.0; INTERNAL_DIM],
        };

        if self.isp <= 0.0 {
            return state;
        }

        let p = self.polarity();
        let nfp_vt = (self.nfp.max(1e-12) * self.vt.max(1e-12)).max(1e-18);
        let vbep_eff = p * (vbx - vbp);
        let vbci_eff = p * (vbi - vci);
        let diode = |v| {
            if self.vbic_13 {
                self.vbic_diode_iv(self.isp, v, self.nfp, self.vbic_junction_limits.ip)
            } else {
                let (exponential, derivative) = Self::limited_exp(v / nfp_vt);
                (
                    self.isp * (exponential - 1.0),
                    self.isp * derivative / nfp_vt,
                )
            }
        };
        let (ibep, gbep) = diode(vbep_eff);
        let (ibci, gbci) = diode(vbci_eff);
        let d_ifp_d_vbep_eff = self.wsp * gbep;
        let d_ifp_d_vbci_eff = (1.0 - self.wsp) * gbci;
        state.ifp = self.wsp * ibep + (1.0 - self.wsp) * ibci;
        state.d_ifp[IDX_VBX] = d_ifp_d_vbep_eff * p;
        state.d_ifp[IDX_VBP] = -d_ifp_d_vbep_eff * p;
        state.d_ifp[IDX_VBI] = d_ifp_d_vbci_eff * p;
        state.d_ifp[IDX_VCI] = -d_ifp_d_vbci_eff * p;

        let iikp = if self.ikp.is_finite() && self.ikp > 0.0 {
            // Ifp already includes AREA*M. IKP remains a nominal parameter,
            // so normalize by the same scale before forming the base charge.
            1.0 / self.ikp / self.instance_scale()
        } else {
            0.0
        };
        let (sqrt_term, d_sqrt_term) =
            self.vbic_high_injection_power(1.0 + 4.0 * state.ifp * iikp, 0.5);
        state.qbp = 0.5 * (1.0 + sqrt_term);
        if iikp > 0.0 {
            let d_qbp_d_ifp = 2.0 * iikp * d_sqrt_term;
            for idx in 0..INTERNAL_DIM {
                state.d_qbp[idx] = d_qbp_d_ifp * state.d_ifp[idx];
            }
        }

        // Three-terminal VBIC still uses Ifp and qbp in Qbep and Rbp,
        // but has no reverse parasitic transport or substrate terminal.
        if self.vbic_three_terminal {
            return state;
        }
        let vbcp_eff = p * (vsi - vbp);
        let (irp, d_irp_d_vbcp_eff) = diode(vbcp_eff);
        state.irp = irp;
        state.d_irp[IDX_VSI] = d_irp_d_vbcp_eff * p;
        state.d_irp[IDX_VBP] = -d_irp_d_vbcp_eff * p;

        state
    }

    pub(in crate::device::semiconductor::bjt) fn irbp_branch(
        &self,
        vbx: Value,
        vbi: Value,
        vcx: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rbp) {
            return branch;
        }

        let parasitic = self.parasitic_transport_state(vbx, vbi, vci, vbp, vsi);
        let rbp = self.guarded_series_resistance(self.rbp);
        let vrbp = vbp - vcx;
        let scale = vrbp / rbp;

        branch.current = scale * parasitic.qbp;
        branch.d_internal[IDX_VCX] = -parasitic.qbp / rbp;
        branch.d_internal[IDX_VBP] = parasitic.qbp / rbp + scale * parasitic.d_qbp[IDX_VBP];
        branch.d_internal[IDX_VBX] = scale * parasitic.d_qbp[IDX_VBX];
        branch.d_internal[IDX_VBI] = scale * parasitic.d_qbp[IDX_VBI];
        branch.d_internal[IDX_VCI] = scale * parasitic.d_qbp[IDX_VCI];
        branch.d_internal[IDX_VSI] = scale * parasitic.d_qbp[IDX_VSI];
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn ibcp_branch(
        &self,
        voltages: BjtNodeVoltages,
    ) -> BranchLinearization {
        let BjtNodeVoltages { vbp, vsi, .. } = voltages;
        let mut branch = BranchLinearization::default();
        if self.charge_model == BjtChargeModel::LegacyGummelPoon {
            // Ngspice's substrate parallel connects to the intrinsic
            // collector (vertical) or base (lateral). Xyce GP omits it.
            // Ngspice 46 omits this conductance in bjtacld.c. Retain the
            // DC current's derivative here so AC and transient linearize
            // the same substrate branch and preserve current conservation.
            if !self.xyce_compatibility {
                let (connection, index) = match self.substrate_topology {
                    BjtSubstrateTopology::Vertical => (voltages.vci, IDX_VCI),
                    BjtSubstrateTopology::Lateral => (voltages.vbi, IDX_VBI),
                };
                let gmin = self.nonlinear_branch_gmin();
                branch.current = gmin * (vsi - connection);
                branch.d_internal[IDX_VSI] = gmin;
                branch.d_internal[index] = -gmin;
                if let Some(junctions) = &self.legacy_junction_params {
                    let polarity = self.polarity() * self.substrate_topology.ngspice_sign();
                    let (current, conductance) = self.legacy_junction_iv(
                        LegacyCurrent::Substrate,
                        junctions.substrate_current,
                        polarity * (vsi - connection),
                        junctions.substrate_emission.unwrap_or(1.0),
                    );
                    branch.current += polarity * current;
                    branch.d_internal[IDX_VSI] += conductance;
                    branch.d_internal[index] -= conductance;
                }
            }
            return branch;
        }
        // Three-terminal VBIC has no substrate branch, including GMIN.
        if self.vbic_three_terminal {
            return branch;
        }
        // ngspice vbicload.c stamps the `CKTgmin` parallel on Vbcp
        // unconditionally, even when the parasitic diode currents are zero.
        let gmin = self.nonlinear_branch_gmin();
        branch.current = gmin * (vsi - vbp);
        branch.d_internal[IDX_VSI] = gmin;
        branch.d_internal[IDX_VBP] = -gmin;
        if self.ibcip <= 0.0 && self.ibcnp <= 0.0 {
            return branch;
        }

        let p = self.polarity();
        let vbcp_eff = p * (vsi - vbp);
        let (ibcip, gbcip) = self.vbic_diode_iv(
            self.ibcip,
            vbcp_eff,
            self.ncip,
            self.vbic_junction_limits.ibcip,
        );
        let (ibcnp, gbcnp) = self.vbic_diode_iv(
            self.ibcnp,
            vbcp_eff,
            self.ncnp,
            self.vbic_junction_limits.ibcnp,
        );
        let gbcp = gbcip + gbcnp;

        branch.current += p * (ibcip + ibcnp);
        branch.d_internal[IDX_VSI] += gbcp;
        branch.d_internal[IDX_VBP] -= gbcp;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn iccp_branch(
        &self,
        vbx: Value,
        vbi: Value,
        vci: Value,
        vbp: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if self.isp <= 0.0 || self.vbic_three_terminal {
            return branch;
        }

        let parasitic = self.parasitic_transport_state(vbx, vbi, vci, vbp, vsi);
        let p = self.polarity();
        let inv_qbp = 1.0 / parasitic.qbp.max(1e-12);
        let delta = parasitic.ifp - parasitic.irp;

        branch.current = p * delta * inv_qbp;
        for idx in 0..INTERNAL_DIM {
            branch.d_internal[idx] = p
                * ((parasitic.d_ifp[idx] - parasitic.d_irp[idx]) * inv_qbp
                    - delta * parasitic.d_qbp[idx] * inv_qbp * inv_qbp);
        }
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn irs_branch(
        &self,
        vs: Value,
        vsi: Value,
    ) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        if !self.has_substrate_resistance() {
            return branch;
        }

        let g = 1.0 / self.guarded_series_resistance(self.rs);
        branch.current = g * (vs - vsi);
        branch.d_internal[IDX_VSI] = -g;
        branch.d_external[EXT_S] = g;
        branch
    }

    pub(in crate::device::semiconductor::bjt) fn irci_branch(
        &self,
        vcx: Value,
        vci: Value,
        vbi: Value,
    ) -> BranchLinearization {
        self.irci_branch_with_self_conductance(vcx, vci, vbi).0
    }

    /// Kull epi branch together with its `dIrci/dVrci` partial in ngspice's
    /// parameterization. The three controlling voltages satisfy
    /// `vrci = vbci − vbcx` identically, so this partial is not recoverable
    /// from the node-space derivatives; vbicload.c stores it as the
    /// `Irci_Vrci` state (gmin parallel included) and vbicnoise.c reads that
    /// state as the thermal-noise conductance of the epi resistance.
    pub(in crate::device::semiconductor::bjt) fn irci_branch_with_self_conductance(
        &self,
        vcx: Value,
        vci: Value,
        vbi: Value,
    ) -> (BranchLinearization, Value) {
        let mut branch = BranchLinearization::default();
        if !Self::series_active(self.rci) {
            return (branch, 0.0);
        }

        let p = self.polarity();
        let vt = self.vt.max(1e-12);
        let rci = self.guarded_series_resistance(self.rci);
        let gamm = self.gamm.max(0.0);
        let ivo = if self.vo.is_finite() && self.vo > 0.0 {
            1.0 / self.vo
        } else {
            0.0
        };
        let ihrcf = if self.hrcf.is_finite() && self.hrcf > 0.0 {
            1.0 / self.hrcf
        } else {
            0.0
        };

        let vrci_eff = p * (vcx - vci);
        let vbci_eff = p * (vbi - vci);
        let vbcx_eff = p * (vbi - vcx);

        let (exp_bci, dexp_bci_darg) = self.vbic_general_exp(vbci_eff / vt);
        let (exp_bcx, dexp_bcx_darg) = self.vbic_general_exp(vbcx_eff / vt);
        let d_exp_bci_dvbci_eff = dexp_bci_darg / vt;
        let d_exp_bcx_dvbcx_eff = dexp_bcx_darg / vt;

        let kbci = (1.0 + gamm * exp_bci).sqrt().max(1e-12);
        let kbcx = (1.0 + gamm * exp_bcx).sqrt().max(1e-12);
        let d_kbci_dvbci_eff = if gamm > 0.0 {
            gamm * d_exp_bci_dvbci_eff / (2.0 * kbci)
        } else {
            0.0
        };
        let d_kbcx_dvbcx_eff = if gamm > 0.0 {
            gamm * d_exp_bcx_dvbcx_eff / (2.0 * kbcx)
        } else {
            0.0
        };

        let ratio = ((kbci + 1.0) / (kbcx + 1.0)).max(1e-18);
        let log_ratio = ratio.ln();
        let d_ratio_dkbci = 1.0 / (kbcx + 1.0);
        let d_ratio_dkbcx = -(kbci + 1.0) / (kbcx + 1.0).powi(2);
        let d_log_ratio_dkbci = d_ratio_dkbci / ratio;
        let d_log_ratio_dkbcx = d_ratio_dkbcx / ratio;

        let iohm = (vrci_eff + vt * (kbci - kbcx - log_ratio)) / rci;
        let d_iohm_dvrci_eff = 1.0 / rci;
        let d_iohm_dvbci_eff = vt * d_kbci_dvbci_eff * (1.0 - d_log_ratio_dkbci) / rci;
        let d_iohm_dvbcx_eff = vt * d_kbcx_dvbcx_eff * (-1.0 - d_log_ratio_dkbcx) / rci;

        let sqrt_vrci = (vrci_eff * vrci_eff + 0.01).sqrt();
        let denom = 1.0 + 0.5 * ivo * ihrcf * sqrt_vrci;
        let d_denom_dvrci_eff = if ivo > 0.0 && ihrcf > 0.0 {
            0.5 * ivo * ihrcf * vrci_eff / sqrt_vrci
        } else {
            0.0
        };

        let derf_scale = ivo * rci;
        let derf = if derf_scale > 0.0 {
            derf_scale * iohm / denom
        } else {
            0.0
        };
        let d_derf_dvrci_eff = if derf_scale > 0.0 {
            derf_scale * (d_iohm_dvrci_eff / denom - iohm * d_denom_dvrci_eff / denom.powi(2))
        } else {
            0.0
        };
        let d_derf_dvbci_eff = if derf_scale > 0.0 {
            derf_scale * d_iohm_dvbci_eff / denom
        } else {
            0.0
        };
        let d_derf_dvbcx_eff = if derf_scale > 0.0 {
            derf_scale * d_iohm_dvbcx_eff / denom
        } else {
            0.0
        };

        let irci_scale = (1.0 + derf * derf).sqrt();
        let inv_irci_scale = 1.0 / irci_scale;
        let common = -iohm * derf / (irci_scale * irci_scale * irci_scale);
        // ngspice vbicload.c gives Irci a `CKTgmin` parallel per controlling
        // voltage (Vrci, Vbci, Vbcx). These are what keep the CX/CI rows
        // nonsingular when the Kull epi exponential crosses its saturation
        // knife edge (the very rows ngspice reports as singular on decks it
        // cannot solve), and gmin stepping ramps them with `CKTgmin`.
        let gmin = if self.vbic_13 {
            0.0
        } else {
            self.nonlinear_branch_gmin()
        };
        let d_irci_eff_dvrci_eff =
            d_iohm_dvrci_eff * inv_irci_scale + common * d_derf_dvrci_eff + gmin;
        let d_irci_eff_dvbci_eff =
            d_iohm_dvbci_eff * inv_irci_scale + common * d_derf_dvbci_eff + gmin;
        let d_irci_eff_dvbcx_eff =
            d_iohm_dvbcx_eff * inv_irci_scale + common * d_derf_dvbcx_eff + gmin;
        let irci_eff = iohm * inv_irci_scale + gmin * (vrci_eff + vbci_eff + vbcx_eff);

        branch.current = p * irci_eff;
        branch.d_internal[IDX_VCX] = d_irci_eff_dvrci_eff - d_irci_eff_dvbcx_eff;
        branch.d_internal[IDX_VCI] = -(d_irci_eff_dvrci_eff + d_irci_eff_dvbci_eff);
        branch.d_internal[IDX_VBI] = d_irci_eff_dvbci_eff + d_irci_eff_dvbcx_eff;
        (branch, d_irci_eff_dvrci_eff)
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn thermal_derivative_step(
        &self,
        vrth: Value,
    ) -> Value {
        // Use a small relative perturbation to keep Vrth-derivative finite
        // differences accurate for strongly temperature-sensitive currents.
        let raw_temperature = self.requested_temperature() + vrth;
        (raw_temperature.abs().max(1.0) * 1e-6).clamp(1e-7, 1e-3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn vbic13_reverse_parasitic_transport_keeps_a_constant_charge_floor() {
        for level in [11.0, 12.0] {
            for model in [
                Bjt::new_npn("q".into(), 1, 2, 0),
                Bjt::new_pnp("q".into(), 1, 2, 0),
            ] {
                let model = model
                    .with_params(&HashMap::from([
                        ("LEVEL".into(), level),
                        ("ISP".into(), 1e-8),
                        ("IKP".into(), 1e-10),
                        ("WSP".into(), 0.6),
                    ]))
                    .with_instance_params(&[("M".into(), 3.0)]);
                let p = model.polarity();
                for bias in [-0.100_001, -0.1, -0.099_999] {
                    let state =
                        model.parasitic_transport_state(p * bias, p * bias, 0.0, 0.0, p * bias);
                    assert!(state.ifp < 0.0);
                    assert_eq!(state.qbp, 0.500_05);
                    assert_eq!(state.d_qbp, [0.0; INTERNAL_DIM]);
                    assert!(state.d_ifp[IDX_VBX].abs() > 0.0);
                }
            }
        }
    }

    #[test]
    fn vbic_parasitic_base_charge_is_independent_of_parallel_instance_count() {
        for level in [4.0, 11.0, 12.0] {
            for polarity in [1.0, -1.0] {
                let params = HashMap::from([
                    ("LEVEL".into(), level),
                    ("ISP".into(), 1e-15),
                    ("IKP".into(), 1e-5),
                    ("WSP".into(), 0.6),
                ]);
                let make = |area, m| {
                    let model = if polarity > 0.0 {
                        Bjt::new_npn("q".into(), 1, 2, 0)
                    } else {
                        Bjt::new_pnp("q".into(), 1, 2, 0)
                    };
                    model
                        .with_params(&params)
                        .with_instance_params(&[("AREA".into(), area), ("M".into(), m)])
                };
                let evaluate = |model: &Bjt| {
                    model.parasitic_transport_state(
                        polarity * 0.7,
                        polarity * 0.68,
                        polarity * 0.1,
                        polarity * 0.12,
                        0.0,
                    )
                };
                let unit = evaluate(&make(1.0, 1.0));
                assert!(unit.qbp > 1.1, "the fixture must exercise high injection");
                for (area, m) in [(1.0, 3.0), (2.0, 1.0), (2.0, 3.0)] {
                    let scaled = evaluate(&make(area, m));
                    assert!((scaled.qbp - unit.qbp).abs() < 1e-12);
                    assert!((scaled.ifp - area * m * unit.ifp).abs() < 1e-12 * scaled.ifp.abs());
                    for (actual, expected) in scaled.d_qbp.into_iter().zip(unit.d_qbp) {
                        assert!((actual - expected).abs() < 1e-12 * expected.abs().max(1.0));
                    }
                }
            }
        }
    }

    #[test]
    fn vbic13_extrinsic_avalanche_jacobian_includes_its_collector_current_control() {
        for level in [11.0, 12.0] {
            for polarity in [1.0, -1.0] {
                for resistance in [0.0, 4.0] {
                    let params = HashMap::from([
                        ("LEVEL".into(), level),
                        ("RCX".into(), resistance),
                        ("AVCX1".into(), 0.2),
                        ("AVCX2".into(), 0.3),
                        ("MCX".into(), 0.33),
                        ("GMIN".into(), 1e-5),
                    ]);
                    let bjt = if polarity > 0.0 {
                        Bjt::new_npn("q".into(), 1, 2, 0)
                    } else {
                        Bjt::new_pnp("q".into(), 1, 2, 0)
                    }
                    .with_params(&params)
                    .with_instance_params(&[("M".into(), 3.0)]);
                    assert!(
                        bjt.rcx > 0.0,
                        "active avalanche must retain its controlling resistor"
                    );
                    for bias in [-4.0, -0.7, 0.0, 0.5] {
                        let vcx = polarity * 1.0;
                        let vc = vcx + polarity * 0.01;
                        let vbx = vcx + polarity * bias;
                        let branch = bjt.igcx_branch(vc, vcx, vbx);
                        let h = 1e-6;
                        for (index, actual) in [
                            (0, branch.d_external[EXT_C]),
                            (1, branch.d_internal[IDX_VCX]),
                            (2, branch.d_internal[IDX_VBX]),
                        ] {
                            let mut plus = [vc, vcx, vbx];
                            let mut minus = plus;
                            plus[index] += h;
                            minus[index] -= h;
                            let fd = (bjt.igcx_branch(plus[0], plus[1], plus[2]).current
                                - bjt.igcx_branch(minus[0], minus[1], minus[2]).current)
                                / (2.0 * h);
                            assert!(
                                (actual - fd).abs() < 2e-7 * actual.abs().max(1e-4),
                                "level={level}, p={polarity}, R={resistance}, bias={bias}, column={index}: {actual} vs {fd}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn vbic13_extrinsic_leakage_remains_when_avalanche_is_off() {
        for level in [4.0, 11.0, 12.0] {
            let bjt = Bjt::new_npn("q".into(), 1, 2, 0)
                .with_params(&HashMap::from([
                    ("LEVEL".into(), level),
                    ("GMIN".into(), 1e-4),
                ]))
                .with_instance_params(&[("M".into(), 3.0)]);
            let branch = bjt.igcx_branch(1.2, 1.2, 0.5);
            let conductance = if level >= 11.0 { 3e-4 } else { 0.0 };
            assert!((branch.current - conductance * -0.7).abs() < 1e-18);
            assert!((branch.d_internal[IDX_VBX] - conductance).abs() < 1e-18);
            assert!((branch.d_internal[IDX_VCX] + conductance).abs() < 1e-18);
        }
    }

    #[test]
    fn legacy_gp_base_resistance_freezes_qb_jacobian_like_xyce() {
        let legacy = Bjt::new_npn("legacy".to_string(), 1, 2, 3).with_params(&HashMap::from([
            ("IS".to_string(), 1.0e-16),
            ("BF".to_string(), 100.0),
            ("BR".to_string(), 8.0),
            ("RB".to_string(), 100.0),
            ("RBM".to_string(), 4.0),
            ("IKF".to_string(), 0.1),
            ("IKR".to_string(), 0.02),
        ]));
        let legacy_eval = legacy.evaluate_state_fixed_temperature(BjtNodeVoltages {
            vc: 0.0,
            vb: 0.7,
            ve: 0.0,
            vs: 0.0,
            vcx: 0.0,
            vci: 0.0,
            vbx: 0.5,
            vbi: 0.4,
            vei: 0.0,
            vbp: 0.0,
            vsi: 0.0,
        });
        assert!(legacy_eval.linearized.qb.is_finite());
        assert_eq!(legacy_eval.irbi.d_internal[IDX_VCI], 0.0);
        assert_eq!(legacy_eval.irbi.d_internal[IDX_VEI], 0.0);

        let vbic = Bjt::new_npn("vbic".to_string(), 1, 2, 3).with_params(&HashMap::from([
            ("LEVEL".to_string(), 11.0),
            ("IS".to_string(), 1.0e-16),
            ("RBI".to_string(), 96.0),
            ("VAF".to_string(), 10.0),
            ("VAR".to_string(), 4.0),
            ("IKF".to_string(), 0.1),
        ]));
        let vbic_eval = vbic.evaluate_state_fixed_temperature(BjtNodeVoltages {
            vc: 0.0,
            vb: 0.8,
            ve: 0.0,
            vs: 0.0,
            vcx: 0.0,
            vci: 0.0,
            vbx: 0.7,
            vbi: 0.6,
            vei: 0.0,
            vbp: 0.0,
            vsi: 0.0,
        });
        assert!(vbic_eval.irbi.d_internal[IDX_VCI].abs() > 0.0);
        assert!(vbic_eval.irbi.d_internal[IDX_VEI].abs() > 0.0);
    }

    #[test]
    fn legacy_gp_irb_resistance_law_reduces_forward_resistance() {
        let mut no_irb = Bjt::new_npn("no_irb".to_string(), 1, 2, 3);
        no_irb.rbi = 96.0;
        let linearized = BjtLinearization {
            ib: 1.0e-2,
            qb: 1.0,
            ..BjtLinearization::default()
        };
        let baseline = no_irb.legacy_gp_base_resistance(linearized, no_irb.rbi);

        let mut with_irb = no_irb;
        with_irb.irb = 1.0e-3;
        let reduced = with_irb.legacy_gp_base_resistance(linearized, with_irb.rbi);

        assert!(baseline.is_finite() && reduced.is_finite());
        assert!(
            reduced < baseline,
            "IRB must reduce the effective resistance"
        );
    }

    #[test]
    fn legacy_gp_irb_uses_model_oriented_pnp_base_current() {
        let mut pnp = Bjt::new_pnp("pnp".to_string(), 1, 2, 3);
        pnp.rbi = 96.0;
        pnp.irb = 1.0e-3;
        let linearized = BjtLinearization {
            // PNP terminal base current is negative; Xyce's IRB argument is
            // model-oriented and therefore uses the polarity-adjusted value.
            ib: -1.0e-2,
            qb: 1.0,
            ..BjtLinearization::default()
        };

        let resistance = pnp.legacy_gp_base_resistance(linearized, pnp.rbi);
        assert!(resistance.is_finite() && resistance > 0.0);
    }
}
