//! Parse editor text into the shared DC sweep contract.
use super::DcAxisMode;

impl DcAxisMode {
    pub fn from_draft(mode: usize, values: &str, points: &str) -> Result<Self, String> {
        match mode {
            0 => Ok(Self::Linear),
            1 => {
                let values = values
                    .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
                    .filter(|s| !s.is_empty())
                    .map(crate::simulation::spice_value::parse_spice_value_checked)
                    .collect::<Result<Vec<_>, _>>()?;
                if values.is_empty() {
                    return Err("DC list cannot be empty".into());
                }
                Ok(Self::List { values })
            }
            2 | 3 => {
                let count = points
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| "DC points per interval must be a positive integer")?;
                if count == 0 {
                    return Err("DC points per interval must be positive".into());
                }
                if mode == 2 {
                    Ok(Self::Decade {
                        points_per_decade: count,
                    })
                } else {
                    Ok(Self::Octave {
                        points_per_octave: count,
                    })
                }
            }
            _ => Err("Unknown DC sweep mode".into()),
        }
    }
}
