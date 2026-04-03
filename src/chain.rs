use crate::device::Param;
use crate::error::{Error, Result};
use crate::osc::{Arg, OscClient};

/// A chain inside a rack device (Drum Rack, Instrument Rack, etc.)
#[derive(Clone)]
pub struct Chain {
    pub(crate) osc: OscClient,
    pub track_idx: i32,
    pub device_idx: i32,
    pub chain_idx: i32,
}

/// A device inside a chain.
#[derive(Clone)]
pub struct ChainDevice {
    pub(crate) osc: OscClient,
    pub track_idx: i32,
    pub device_idx: i32,
    pub chain_idx: i32,
    pub chain_device_idx: i32,
}

impl Chain {
    fn prefix(&self) -> [Arg; 3] {
        [
            Arg::Int(self.track_idx),
            Arg::Int(self.device_idx),
            Arg::Int(self.chain_idx),
        ]
    }

    // -- Identity --

    pub fn get_name(&self) -> Result<String> {
        let resp = self.osc.query("/live/chain/get/name", &self.prefix())?;
        resp.get(3)
            .and_then(|a| a.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/chain/get/name".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.push(Arg::from(name));
        self.osc.send("/live/chain/set/name", &args)
    }

    // -- Mix --

    pub fn get_volume(&self) -> Result<f32> {
        let resp = self.osc.query("/live/chain/get/volume", &self.prefix())?;
        resp.get(3)
            .and_then(|a| a.as_f32())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/chain/get/volume".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    pub fn set_volume(&self, vol: f32) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.push(Arg::Float(vol));
        self.osc.send("/live/chain/set/volume", &args)
    }

    pub fn get_panning(&self) -> Result<f32> {
        let resp = self.osc.query("/live/chain/get/panning", &self.prefix())?;
        resp.get(3)
            .and_then(|a| a.as_f32())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/chain/get/panning".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    pub fn set_panning(&self, pan: f32) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.push(Arg::Float(pan));
        self.osc.send("/live/chain/set/panning", &args)
    }

    pub fn get_mute(&self) -> Result<bool> {
        let resp = self.osc.query("/live/chain/get/mute", &self.prefix())?;
        resp.get(3)
            .and_then(|a| a.as_bool())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/chain/get/mute".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    pub fn set_mute(&self, mute: bool) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.push(Arg::from(mute));
        self.osc.send("/live/chain/set/mute", &args)
    }

    pub fn get_solo(&self) -> Result<bool> {
        let resp = self.osc.query("/live/chain/get/solo", &self.prefix())?;
        resp.get(3)
            .and_then(|a| a.as_bool())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/chain/get/solo".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    pub fn set_solo(&self, solo: bool) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.push(Arg::from(solo));
        self.osc.send("/live/chain/set/solo", &args)
    }

    // -- Devices --

    pub fn num_devices(&self) -> Result<i32> {
        let resp = self.osc.query("/live/chain/get/num_devices", &self.prefix())?;
        resp.get(3)
            .and_then(|a| a.as_i32())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/chain/get/num_devices".into(),
                expected: 4,
                got: resp.len(),
            })
    }

    pub fn device_names(&self) -> Result<Vec<String>> {
        let resp = self.osc.query("/live/chain/get/devices/name", &self.prefix())?;
        Ok(resp[3..].iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    pub fn device(&self, chain_device_idx: i32) -> ChainDevice {
        ChainDevice {
            osc: self.osc.clone(),
            track_idx: self.track_idx,
            device_idx: self.device_idx,
            chain_idx: self.chain_idx,
            chain_device_idx,
        }
    }
}

impl ChainDevice {
    fn prefix(&self) -> [Arg; 4] {
        [
            Arg::Int(self.track_idx),
            Arg::Int(self.device_idx),
            Arg::Int(self.chain_idx),
            Arg::Int(self.chain_device_idx),
        ]
    }

    // -- Parameters --

    pub fn parameter_names(&self) -> Result<Vec<String>> {
        let resp = self.osc.query("/live/chain/device/get/parameters/name", &self.prefix())?;
        Ok(resp[4..].iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    pub fn parameter_values(&self) -> Result<Vec<f32>> {
        let resp = self.osc.query("/live/chain/device/get/parameters/value", &self.prefix())?;
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
        let resp = self.osc.query("/live/chain/device/get/parameter/value", &args)?;
        resp.last()
            .and_then(|a| a.as_f32())
            .ok_or_else(|| Error::BadResponse {
                address: "/live/chain/device/get/parameter/value".into(),
                expected: 6,
                got: resp.len(),
            })
    }

    pub fn set_param(&self, param_idx: i32, value: f32) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.push(Arg::Int(param_idx));
        args.push(Arg::Float(value));
        self.osc.send("/live/chain/device/set/parameter/value", &args)
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
