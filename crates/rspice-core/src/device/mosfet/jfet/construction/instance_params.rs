use super::*;

impl Jfet {
    /// Apply instance-level JFET/MESFET geometry and multiplicity parameters.
    ///
    /// Supported keys:
    /// - `AREA`: direct area scaling
    /// - `M` / `MULT`: multiplicity
    /// - `W`, `L`, optional `NF`: width/length scaling fallback (`W/L * NF`)
    pub fn with_instance_params(mut self, params: &[(String, Value)]) -> Self {
        let mut area_override: Option<Value> = None;
        let mut width: Option<Value> = None;
        let mut length: Option<Value> = None;
        let mut nf = 1.0;
        let mut mult = 1.0;
        let mut temp_override: Option<Value> = None;
        let mut dtemp = 0.0;
        let mut ts_override: Option<Value> = None;
        let mut td_override: Option<Value> = None;

        for (name, value) in params {
            if !value.is_finite() {
                continue;
            }

            if name.eq_ignore_ascii_case("AREA") {
                if *value > 0.0 {
                    area_override = Some(*value);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("W") {
                if *value > 0.0 {
                    width = Some(*value);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("L") {
                if *value > 0.0 {
                    length = Some(*value);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("NF") {
                if *value > 0.0 {
                    nf = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("M") || name.eq_ignore_ascii_case("MULT") {
                if *value > 0.0 {
                    mult = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("TEMP") {
                if *value > 0.0 {
                    temp_override = Some(*value + 273.15);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("DTEMP") {
                dtemp = *value;
                continue;
            }

            if name.eq_ignore_ascii_case("TS") {
                if *value > 0.0 {
                    ts_override = Some(*value + 273.15);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("TD") && *value > 0.0 {
                td_override = Some(*value + 273.15);
            }
        }

        if let Some(w) = width {
            self.width = w;
        }
        if let Some(l) = length {
            self.length = l;
        }

        if let Some(area) = area_override {
            self.area *= area;
        } else if !matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && let (Some(w), Some(l)) = (width, length)
        {
            let wl_scale = w / l;
            if wl_scale.is_finite() && wl_scale > 0.0 {
                self.area *= wl_scale * nf;
            }
        }

        if matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && nf.is_finite()
            && nf > 0.0
        {
            self.width *= nf;
        }

        if mult.is_finite() && mult > 0.0 {
            self.m *= mult;
        }

        self.instance_temp = temp_override;
        self.instance_dtemp = dtemp;
        self.instance_ts = ts_override;
        self.instance_td = td_override;

        self
    }
}
