use crate::error::{Error, Result};
use crate::osc::{Arg, OscClient};

/// A device (instrument or effect) on a track.
#[derive(Clone)]
pub struct Device {
    pub(crate) osc: OscClient,
    pub track_idx: i32,
    pub device_idx: i32,
}

/// A named parameter with its current value.
#[derive(Debug, Clone)]
pub struct Param {
    pub index: usize,
    pub name: String,
    pub value: f32,
}

impl Device {
    pub(crate) fn new(osc: OscClient, track_idx: i32, device_idx: i32) -> Self {
        Self { osc, track_idx, device_idx }
    }

    fn prefix(&self) -> [Arg; 2] {
        [Arg::Int(self.track_idx), Arg::Int(self.device_idx)]
    }

    // -- Identity --

    pub fn name(&self) -> Result<String> {
        let resp = self.osc.query("/live/device/get/name", &self.prefix())?;
        resp.get(2)
            .and_then(|a| a.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/device/get/name".into(),
                expected: 3,
                got: resp.len(),
            })
    }

    pub fn class_name(&self) -> Result<String> {
        let resp = self.osc.query("/live/device/get/class_name", &self.prefix())?;
        resp.get(2)
            .and_then(|a| a.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/device/get/class_name".into(),
                expected: 3,
                got: resp.len(),
            })
    }

    // -- Parameters --

    pub fn parameter_names(&self) -> Result<Vec<String>> {
        let resp = self.osc.query("/live/device/get/parameters/name", &self.prefix())?;
        Ok(resp[2..].iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    pub fn parameter_values(&self) -> Result<Vec<f32>> {
        let resp = self.osc.query("/live/device/get/parameters/value", &self.prefix())?;
        Ok(resp[2..].iter().filter_map(|a| a.as_f32()).collect())
    }

    pub fn parameters(&self) -> Result<Vec<Param>> {
        let names = self.parameter_names()?;
        let values = self.parameter_values()?;
        Ok(names
            .into_iter()
            .zip(values)
            .enumerate()
            .map(|(i, (name, value))| Param { index: i, name, value })
            .collect())
    }

    pub fn get_param(&self, param_idx: i32) -> Result<f32> {
        let args = [Arg::Int(self.track_idx), Arg::Int(self.device_idx), Arg::Int(param_idx)];
        let resp = self.osc.query("/live/device/get/parameter/value", &args)?;
        resp.last()
            .and_then(|a| a.as_f32())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/device/get/parameter/value".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    pub fn set_param(&self, param_idx: i32, value: f32) -> Result<()> {
        self.osc.send(
            "/live/device/set/parameter/value",
            &[Arg::Int(self.track_idx), Arg::Int(self.device_idx), Arg::Int(param_idx), Arg::Float(value)],
        )
    }

    /// Set a parameter by name (case-insensitive).
    pub fn set_param_by_name(&self, name: &str, value: f32) -> Result<()> {
        let names = self.parameter_names()?;
        let name_lower = name.to_lowercase();
        for (i, n) in names.iter().enumerate() {
            if n.to_lowercase() == name_lower {
                return self.set_param(i as i32, value);
            }
        }
        Err(Error::ParamNotFound(name.to_string()))
    }

    /// Get (min, max) range for a parameter.
    pub fn param_range(&self, param_idx: i32) -> Result<(f32, f32)> {
        let min_resp = self.osc.query("/live/device/get/parameters/min", &self.prefix())?;
        let max_resp = self.osc.query("/live/device/get/parameters/max", &self.prefix())?;
        let idx = param_idx as usize + 2; // skip track_idx, device_idx
        let min = min_resp.get(idx).and_then(|a| a.as_f32()).unwrap_or(0.0);
        let max = max_resp.get(idx).and_then(|a| a.as_f32()).unwrap_or(1.0);
        Ok((min, max))
    }
}
