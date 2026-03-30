use std::time::Duration;

use crate::clip::Clip;
use crate::device::Device;
use crate::error::{Error, Result};
use crate::osc::{Arg, OscClient};

/// A track in the Ableton session.
#[derive(Clone)]
pub struct Track {
    pub(crate) osc: OscClient,
    pub track_idx: i32,
}

impl Track {
    pub(crate) fn new(osc: OscClient, track_idx: i32) -> Self {
        Self { osc, track_idx }
    }

    fn prefix(&self) -> [Arg; 1] {
        [Arg::Int(self.track_idx)]
    }

    fn send(&self, addr: &str, extra: &[Arg]) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.extend_from_slice(extra);
        self.osc.send(addr, &args)
    }

    fn query(&self, addr: &str, extra: &[Arg]) -> Result<Vec<Arg>> {
        let mut args = self.prefix().to_vec();
        args.extend_from_slice(extra);
        self.osc.query(addr, &args)
    }

    // -- Properties --

    fn get_prop(&self, prop: &str) -> Result<Arg> {
        let resp = self.query(&format!("/live/track/get/{prop}"), &[])?;
        resp.into_iter().nth(1).ok_or_else(|| Error::BadResponse {
            address: format!("/live/track/get/{prop}"),
            expected: 2,
            got: 0,
        })
    }

    fn set_prop(&self, prop: &str, value: Arg) -> Result<()> {
        self.send(&format!("/live/track/set/{prop}"), &[value])
    }

    pub fn get_name(&self) -> Result<String> {
        self.get_prop("name").map(|a| a.as_str().unwrap_or("").to_string())
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        self.set_prop("name", Arg::from(name))
    }

    pub fn get_volume(&self) -> Result<f32> {
        self.get_prop("volume").map(|a| a.as_f32().unwrap_or(0.85))
    }

    pub fn set_volume(&self, volume: f32) -> Result<()> {
        self.set_prop("volume", Arg::Float(volume))
    }

    pub fn get_panning(&self) -> Result<f32> {
        self.get_prop("panning").map(|a| a.as_f32().unwrap_or(0.0))
    }

    pub fn set_panning(&self, pan: f32) -> Result<()> {
        self.set_prop("panning", Arg::Float(pan))
    }

    pub fn get_mute(&self) -> Result<bool> {
        self.get_prop("mute").map(|a| a.as_bool().unwrap_or(false))
    }

    pub fn set_mute(&self, mute: bool) -> Result<()> {
        self.set_prop("mute", Arg::from(mute))
    }

    pub fn get_solo(&self) -> Result<bool> {
        self.get_prop("solo").map(|a| a.as_bool().unwrap_or(false))
    }

    pub fn set_solo(&self, solo: bool) -> Result<()> {
        self.set_prop("solo", Arg::from(solo))
    }

    pub fn get_arm(&self) -> Result<bool> {
        self.get_prop("arm").map(|a| a.as_bool().unwrap_or(false))
    }

    pub fn set_arm(&self, arm: bool) -> Result<()> {
        self.set_prop("arm", Arg::from(arm))
    }

    pub fn get_color(&self) -> Result<i32> {
        self.get_prop("color").map(|a| a.as_i32().unwrap_or(0))
    }

    pub fn set_color(&self, color: i32) -> Result<()> {
        self.set_prop("color", Arg::Int(color))
    }

    // -- Sends --

    pub fn get_send(&self, send_idx: i32) -> Result<f32> {
        let resp = self.query("/live/track/get/send", &[Arg::Int(send_idx)])?;
        resp.get(2).and_then(|a| a.as_f32()).ok_or_else(|| Error::BadResponse {
            address: "/live/track/get/send".into(),
            expected: 3,
            got: resp.len(),
        })
    }

    pub fn set_send(&self, send_idx: i32, value: f32) -> Result<()> {
        self.send("/live/track/set/send", &[Arg::Int(send_idx), Arg::Float(value)])
    }

    // -- Clips --

    /// Create an empty MIDI clip in the given slot.
    pub fn create_clip(&self, slot_idx: i32, length: f32) -> Result<Clip> {
        self.osc.send(
            "/live/clip_slot/create_clip",
            &[Arg::Int(self.track_idx), Arg::Int(slot_idx), Arg::Float(length)],
        )?;
        std::thread::sleep(Duration::from_millis(100));
        Ok(Clip::new(self.osc.clone(), self.track_idx, slot_idx))
    }

    /// Get a reference to an existing clip.
    pub fn clip(&self, slot_idx: i32) -> Clip {
        Clip::new(self.osc.clone(), self.track_idx, slot_idx)
    }

    pub fn delete_clip(&self, slot_idx: i32) -> Result<()> {
        self.osc.send(
            "/live/clip_slot/delete_clip",
            &[Arg::Int(self.track_idx), Arg::Int(slot_idx)],
        )
    }

    pub fn has_clip(&self, slot_idx: i32) -> Result<bool> {
        let resp = self.osc.query(
            "/live/clip_slot/get/has_clip",
            &[Arg::Int(self.track_idx), Arg::Int(slot_idx)],
        )?;
        Ok(resp.get(2).and_then(|a| a.as_bool()).unwrap_or(false))
    }

    pub fn clip_names(&self) -> Result<Vec<String>> {
        let resp = self.query("/live/track/get/clips/name", &[])?;
        Ok(resp[1..].iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    // -- Devices --

    pub fn num_devices(&self) -> Result<i32> {
        let resp = self.query("/live/track/get/num_devices", &[])?;
        Ok(resp.get(1).and_then(|a| a.as_i32()).unwrap_or(0))
    }

    pub fn devices(&self) -> Result<Vec<Device>> {
        let count = self.num_devices()?;
        Ok((0..count).map(|i| Device::new(self.osc.clone(), self.track_idx, i)).collect())
    }

    pub fn device(&self, device_idx: i32) -> Device {
        Device::new(self.osc.clone(), self.track_idx, device_idx)
    }

    pub fn device_names(&self) -> Result<Vec<String>> {
        let resp = self.query("/live/track/get/devices/name", &[])?;
        Ok(resp[1..].iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    pub fn delete_device(&self, device_idx: i32) -> Result<()> {
        self.osc.send(
            "/live/track/delete_device",
            &[Arg::Int(self.track_idx), Arg::Int(device_idx)],
        )
    }

    pub fn stop_all_clips(&self) -> Result<()> {
        self.send("/live/track/stop_all_clips", &[])
    }

    /// Copy a session clip to the arrangement view at a given beat position.
    pub fn duplicate_clip_to_arrangement(&self, slot_idx: i32, time: f32) -> Result<()> {
        self.send(
            "/live/track/duplicate_clip_to_arrangement",
            &[Arg::Int(slot_idx), Arg::Float(time)],
        )
    }

    // -- Sample loading --

    /// Load a sample by name via the browser.
    pub fn load_sample(&self, name: &str) -> Result<String> {
        let resp = self.osc.query_timeout(
            "/live/browser/load_sample",
            &[Arg::Int(self.track_idx), Arg::from(name)],
            Duration::from_secs(5),
        )?;
        if let Some(Arg::String(ref s)) = resp.first() {
            if s == "error" {
                let msg = resp.get(1).and_then(|a| a.as_str()).unwrap_or("unknown");
                return Err(Error::Ableton(msg.to_string()));
            }
        }
        resp.get(1)
            .and_then(|a| a.as_str())
            .map(String::from)
            .ok_or_else(|| Error::Ableton("no response from load_sample".into()))
    }

    // -- Output routing --

    pub fn output_routing_types(&self) -> Result<Vec<String>> {
        let resp = self.query("/live/track/get/available_output_routing_types", &[])?;
        Ok(resp[1..].iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    pub fn set_output_routing(&self, type_name: &str) -> Result<()> {
        self.send("/live/track/set/output_routing_type", &[Arg::from(type_name)])
    }

    /// Read the current output meter level (left channel, 0.0–1.0).
    pub fn get_output_meter(&self) -> Result<f32> {
        self.get_prop("output_meter_left").map(|a| a.as_f32().unwrap_or(0.0))
    }
}
