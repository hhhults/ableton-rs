use crate::device::Param;
use crate::error::{Error, Result};
use crate::osc::{Arg, OscClient};

/// A drum pad in a Drum Rack, addressed by MIDI note.
#[derive(Clone)]
pub struct DrumPad {
    pub(crate) osc: OscClient,
    pub track_idx: i32,
    pub device_idx: i32,
    pub pad_note: i32,
}

/// A device on a drum pad's chain.
#[derive(Clone)]
pub struct DrumPadDevice {
    pub(crate) osc: OscClient,
    pub track_idx: i32,
    pub device_idx: i32,
    pub pad_note: i32,
    pub chain_device_idx: i32,
}

impl DrumPad {
    fn prefix(&self) -> [Arg; 3] {
        [
            Arg::Int(self.track_idx),
            Arg::Int(self.device_idx),
            Arg::Int(self.pad_note),
        ]
    }

    // -- Identity --

    pub fn get_name(&self) -> Result<String> {
        let resp = self.osc.query("/live/drum_pad/get/name", &self.prefix())?;
        resp.get(3)
            .and_then(|a| a.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/drum_pad/get/name".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.push(Arg::from(name));
        self.osc.send("/live/drum_pad/set/name", &args)
    }

    // -- Mute / Solo --

    pub fn get_mute(&self) -> Result<bool> {
        let resp = self.osc.query("/live/drum_pad/get/mute", &self.prefix())?;
        resp.get(3)
            .and_then(|a| a.as_bool())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/drum_pad/get/mute".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    pub fn set_mute(&self, mute: bool) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.push(Arg::from(mute));
        self.osc.send("/live/drum_pad/set/mute", &args)
    }

    pub fn get_solo(&self) -> Result<bool> {
        let resp = self.osc.query("/live/drum_pad/get/solo", &self.prefix())?;
        resp.get(3)
            .and_then(|a| a.as_bool())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/drum_pad/get/solo".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    pub fn set_solo(&self, solo: bool) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.push(Arg::from(solo));
        self.osc.send("/live/drum_pad/set/solo", &args)
    }

    // -- Chains --

    pub fn num_chains(&self) -> Result<i32> {
        let resp = self.osc.query("/live/drum_pad/get/num_chains", &self.prefix())?;
        resp.get(3)
            .and_then(|a| a.as_i32())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/drum_pad/get/num_chains".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    // -- Devices --

    pub fn device_names(&self) -> Result<Vec<String>> {
        let resp = self.osc.query("/live/drum_pad/chain/get/devices/name", &self.prefix())?;
        Ok(resp[3..].iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    pub fn device(&self, chain_device_idx: i32) -> DrumPadDevice {
        DrumPadDevice {
            osc: self.osc.clone(),
            track_idx: self.track_idx,
            device_idx: self.device_idx,
            pad_note: self.pad_note,
            chain_device_idx,
        }
    }
}

impl DrumPadDevice {
    fn prefix(&self) -> [Arg; 4] {
        [
            Arg::Int(self.track_idx),
            Arg::Int(self.device_idx),
            Arg::Int(self.pad_note),
            Arg::Int(self.chain_device_idx),
        ]
    }

    // -- Parameters --

    pub fn parameter_names(&self) -> Result<Vec<String>> {
        let resp = self.osc.query("/live/drum_pad/chain/device/get/parameters/name", &self.prefix())?;
        Ok(resp[4..].iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    pub fn parameter_values(&self) -> Result<Vec<f32>> {
        let resp = self.osc.query("/live/drum_pad/chain/device/get/parameters/value", &self.prefix())?;
        Ok(resp[4..].iter().filter_map(|a| a.as_f32()).collect())
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
        let mut args = self.prefix().to_vec();
        args.push(Arg::Int(param_idx));
        let resp = self.osc.query("/live/drum_pad/chain/device/get/parameter/value", &args)?;
        resp.last()
            .and_then(|a| a.as_f32())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/drum_pad/chain/device/get/parameter/value".into(),
                expected: 6,
                got: resp.len(),
            })
    }

    pub fn set_param(&self, param_idx: i32, value: f32) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.push(Arg::Int(param_idx));
        args.push(Arg::Float(value));
        self.osc.send("/live/drum_pad/chain/device/set/parameter/value", &args)
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
}
