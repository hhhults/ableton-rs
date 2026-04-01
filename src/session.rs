use std::time::Duration;

use crate::error::{Error, Result};
use crate::osc::{Arg, OscClient};
use crate::track::Track;
use crate::transport::Transport;

/// A return track in the session.
#[derive(Clone)]
pub struct ReturnTrack {
    osc: OscClient,
    pub track_idx: i32,
}

impl ReturnTrack {
    fn prefix(&self) -> [Arg; 1] {
        [Arg::Int(self.track_idx)]
    }

    fn query(&self, addr: &str, extra: &[Arg]) -> Result<Vec<Arg>> {
        let mut args = self.prefix().to_vec();
        args.extend_from_slice(extra);
        self.osc.query(addr, &args)
    }

    fn send(&self, addr: &str, extra: &[Arg]) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.extend_from_slice(extra);
        self.osc.send(addr, &args)
    }

    pub fn get_name(&self) -> Result<String> {
        let resp = self.query("/live/return_track/get/name", &[])?;
        Ok(resp.get(1).and_then(|a| a.as_str()).unwrap_or("").to_string())
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        self.send("/live/return_track/set/name", &[Arg::from(name)])
    }

    pub fn get_volume(&self) -> Result<f32> {
        let resp = self.query("/live/return_track/get/volume", &[])?;
        Ok(resp.get(1).and_then(|a| a.as_f32()).unwrap_or(0.85))
    }

    pub fn set_volume(&self, volume: f32) -> Result<()> {
        self.send("/live/return_track/set/volume", &[Arg::Float(volume)])
    }

    pub fn get_panning(&self) -> Result<f32> {
        let resp = self.query("/live/return_track/get/panning", &[])?;
        Ok(resp.get(1).and_then(|a| a.as_f32()).unwrap_or(0.0))
    }

    pub fn set_panning(&self, pan: f32) -> Result<()> {
        self.send("/live/return_track/set/panning", &[Arg::Float(pan)])
    }

    pub fn device_names(&self) -> Result<Vec<String>> {
        let resp = self.query("/live/return_track/get/devices/name", &[])?;
        Ok(resp.iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }
}

/// A scene in the session.
#[derive(Clone)]
pub struct Scene {
    osc: OscClient,
    pub scene_idx: i32,
}

impl Scene {
    pub fn fire(&self) -> Result<()> {
        self.osc.send("/live/scene/fire", &[Arg::Int(self.scene_idx)])
    }

    pub fn get_name(&self) -> Result<String> {
        let resp = self.osc.query("/live/scene/get/name", &[Arg::Int(self.scene_idx)])?;
        Ok(resp.get(1).and_then(|a| a.as_str()).unwrap_or("").to_string())
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        self.osc.send("/live/scene/set/name", &[Arg::Int(self.scene_idx), Arg::from(name)])
    }
}

/// Main interface to an Ableton Live session via AbletonOSC.
pub struct Session {
    pub(crate) osc: OscClient,
}

impl Session {
    /// Connect to AbletonOSC with custom host/port via direct UDP.
    pub fn new(host: &str, send_port: u16) -> Result<Self> {
        Ok(Self { osc: OscClient::udp(host, send_port)? })
    }

    /// Connect with default settings (localhost:11000, direct UDP).
    pub fn connect() -> Result<Self> {
        Ok(Self { osc: OscClient::connect()? })
    }

    /// Create a session backed by a custom transport (e.g., daemon proxy).
    pub fn with_transport(transport: impl Transport + 'static) -> Self {
        Self { osc: OscClient::from_transport(transport) }
    }

    /// Get the underlying OscClient for raw access.
    pub fn osc(&self) -> &OscClient {
        &self.osc
    }

    // -----------------------------------------------------------------------
    // Transport
    // -----------------------------------------------------------------------

    pub fn play(&self) -> Result<()> {
        self.osc.send("/live/song/start_playing", &[])
    }

    pub fn stop(&self) -> Result<()> {
        self.osc.send("/live/song/stop_playing", &[])
    }

    pub fn continue_playing(&self) -> Result<()> {
        self.osc.send("/live/song/continue_playing", &[])
    }

    pub fn is_playing(&self) -> Result<bool> {
        let resp = self.osc.query("/live/song/get/is_playing", &[])?;
        Ok(resp.first().and_then(|a| a.as_bool()).unwrap_or(false))
    }

    pub fn get_tempo(&self) -> Result<f32> {
        let resp = self.osc.query("/live/song/get/tempo", &[])?;
        Ok(resp.first().and_then(|a| a.as_f32()).unwrap_or(120.0))
    }

    pub fn set_tempo(&self, bpm: f32) -> Result<()> {
        self.osc.send("/live/song/set/tempo", &[Arg::Float(bpm)])
    }

    pub fn get_time(&self) -> Result<f32> {
        let resp = self.osc.query("/live/song/get/current_song_time", &[])?;
        Ok(resp.first().and_then(|a| a.as_f32()).unwrap_or(0.0))
    }

    pub fn set_time(&self, beats: f32) -> Result<()> {
        self.osc.send("/live/song/set/current_song_time", &[Arg::Float(beats)])
    }

    pub fn get_metronome(&self) -> Result<bool> {
        let resp = self.osc.query("/live/song/get/metronome", &[])?;
        Ok(resp.first().and_then(|a| a.as_bool()).unwrap_or(false))
    }

    pub fn set_metronome(&self, on: bool) -> Result<()> {
        self.osc.send("/live/song/set/metronome", &[Arg::from(on)])
    }

    pub fn get_loop(&self) -> Result<bool> {
        let resp = self.osc.query("/live/song/get/loop", &[])?;
        Ok(resp.first().and_then(|a| a.as_bool()).unwrap_or(false))
    }

    pub fn set_loop(&self, on: bool) -> Result<()> {
        self.osc.send("/live/song/set/loop", &[Arg::from(on)])
    }

    pub fn get_loop_start(&self) -> Result<f32> {
        let resp = self.osc.query("/live/song/get/loop_start", &[])?;
        Ok(resp.first().and_then(|a| a.as_f32()).unwrap_or(0.0))
    }

    pub fn set_loop_start(&self, beats: f32) -> Result<()> {
        self.osc.send("/live/song/set/loop_start", &[Arg::Float(beats)])
    }

    pub fn get_loop_length(&self) -> Result<f32> {
        let resp = self.osc.query("/live/song/get/loop_length", &[])?;
        Ok(resp.first().and_then(|a| a.as_f32()).unwrap_or(4.0))
    }

    pub fn set_loop_length(&self, beats: f32) -> Result<()> {
        self.osc.send("/live/song/set/loop_length", &[Arg::Float(beats)])
    }

    pub fn tap_tempo(&self) -> Result<()> {
        self.osc.send("/live/song/tap_tempo", &[])
    }

    // -----------------------------------------------------------------------
    // Time signature
    // -----------------------------------------------------------------------

    pub fn get_time_signature(&self) -> Result<(i32, i32)> {
        let num = self.osc.query("/live/song/get/signature_numerator", &[])?;
        let den = self.osc.query("/live/song/get/signature_denominator", &[])?;
        Ok((
            num.first().and_then(|a| a.as_i32()).unwrap_or(4),
            den.first().and_then(|a| a.as_i32()).unwrap_or(4),
        ))
    }

    pub fn set_time_signature(&self, numerator: i32, denominator: i32) -> Result<()> {
        self.osc.send("/live/song/set/signature_numerator", &[Arg::Int(numerator)])?;
        self.osc.send("/live/song/set/signature_denominator", &[Arg::Int(denominator)])
    }

    // -----------------------------------------------------------------------
    // Tracks
    // -----------------------------------------------------------------------

    pub fn num_tracks(&self) -> Result<i32> {
        let resp = self.osc.query("/live/song/get/num_tracks", &[])?;
        Ok(resp.first().and_then(|a| a.as_i32()).unwrap_or(0))
    }

    pub fn track(&self, idx: i32) -> Track {
        Track::new(self.osc.clone(), idx)
    }

    pub fn tracks(&self) -> Result<Vec<Track>> {
        let n = self.num_tracks()?;
        Ok((0..n).map(|i| Track::new(self.osc.clone(), i)).collect())
    }

    pub fn track_names(&self) -> Result<Vec<String>> {
        let resp = self.osc.query("/live/song/get/track_names", &[])?;
        Ok(resp.iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    pub fn create_midi_track(&self, index: i32) -> Result<Track> {
        self.osc.send("/live/song/create_midi_track", &[Arg::Int(index)])?;
        std::thread::sleep(Duration::from_millis(150));
        if index == -1 {
            let n = self.num_tracks()?;
            Ok(Track::new(self.osc.clone(), n - 1))
        } else {
            Ok(Track::new(self.osc.clone(), index))
        }
    }

    pub fn create_audio_track(&self, index: i32) -> Result<Track> {
        self.osc.send("/live/song/create_audio_track", &[Arg::Int(index)])?;
        std::thread::sleep(Duration::from_millis(150));
        if index == -1 {
            let n = self.num_tracks()?;
            Ok(Track::new(self.osc.clone(), n - 1))
        } else {
            Ok(Track::new(self.osc.clone(), index))
        }
    }

    pub fn delete_track(&self, index: i32) -> Result<()> {
        self.osc.send("/live/song/delete_track", &[Arg::Int(index)])
    }

    pub fn duplicate_track(&self, index: i32) -> Result<Track> {
        self.osc.send("/live/song/duplicate_track", &[Arg::Int(index)])?;
        std::thread::sleep(Duration::from_millis(150));
        Ok(Track::new(self.osc.clone(), index + 1))
    }

    // -----------------------------------------------------------------------
    // Return Tracks
    // -----------------------------------------------------------------------

    pub fn return_track_names(&self) -> Result<Vec<String>> {
        let resp = self.osc.query("/live/return_track/get/names", &[])?;
        Ok(resp.iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    pub fn return_track(&self, idx: i32) -> ReturnTrack {
        ReturnTrack { osc: self.osc.clone(), track_idx: idx }
    }

    // -----------------------------------------------------------------------
    // Scenes
    // -----------------------------------------------------------------------

    pub fn num_scenes(&self) -> Result<i32> {
        let resp = self.osc.query("/live/song/get/num_scenes", &[])?;
        Ok(resp.first().and_then(|a| a.as_i32()).unwrap_or(0))
    }

    pub fn scene(&self, idx: i32) -> Scene {
        Scene { osc: self.osc.clone(), scene_idx: idx }
    }

    pub fn create_scene(&self, index: i32) -> Result<()> {
        self.osc.send("/live/song/create_scene", &[Arg::Int(index)])
    }

    pub fn fire_scene(&self, index: i32) -> Result<()> {
        self.osc.send("/live/scene/fire", &[Arg::Int(index)])
    }

    // -----------------------------------------------------------------------
    // View
    // -----------------------------------------------------------------------

    pub fn select_track(&self, idx: i32) -> Result<()> {
        self.osc.send("/live/view/set/selected_track", &[Arg::Int(idx)])
    }

    pub fn select_clip(&self, track_idx: i32, scene_idx: i32) -> Result<()> {
        self.osc.send("/live/view/set/selected_clip", &[Arg::Int(track_idx), Arg::Int(scene_idx)])
    }

    // -----------------------------------------------------------------------
    // Undo / Redo
    // -----------------------------------------------------------------------

    pub fn undo(&self) -> Result<()> {
        self.osc.send("/live/song/undo", &[])
    }

    pub fn redo(&self) -> Result<()> {
        self.osc.send("/live/song/redo", &[])
    }

    // -----------------------------------------------------------------------
    // Utility
    // -----------------------------------------------------------------------

    pub fn stop_all_clips(&self) -> Result<()> {
        self.osc.send("/live/song/stop_all_clips", &[])
    }

    pub fn cpu_load(&self) -> Result<f32> {
        let resp = self.osc.query("/live/application/get/average_process_usage", &[])?;
        Ok(resp.first().and_then(|a| a.as_f32()).unwrap_or(0.0))
    }

    pub fn version(&self) -> Result<(i32, i32)> {
        let resp = self.osc.query("/live/application/get/version", &[])?;
        Ok((
            resp.first().and_then(|a| a.as_i32()).unwrap_or(0),
            resp.get(1).and_then(|a| a.as_i32()).unwrap_or(0),
        ))
    }

    // -----------------------------------------------------------------------
    // Browser
    // -----------------------------------------------------------------------

    /// Search the Ableton browser. Category: "samples", "instruments", "effects", "all".
    pub fn search_browser(&self, query: &str, category: &str) -> Result<Vec<String>> {
        let resp = self.osc.query_timeout(
            "/live/browser/search",
            &[Arg::from(query), Arg::from(category)],
            Duration::from_secs(5),
        )?;
        Ok(resp.iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    /// List items in a browser category.
    pub fn list_browser_category(&self, category: &str) -> Result<Vec<String>> {
        let resp = self.osc.query_timeout(
            "/live/browser/list_children",
            &[Arg::from(category)],
            Duration::from_secs(5),
        )?;
        Ok(resp.iter().filter_map(|a| a.as_str().map(String::from)).collect())
    }

    /// Load an instrument by name onto a track.
    pub fn load_instrument(&self, track_idx: i32, name: &str) -> Result<String> {
        let resp = self.osc.query_timeout(
            "/live/browser/load_instrument",
            &[Arg::Int(track_idx), Arg::from(name)],
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
            .ok_or_else(|| Error::Ableton("no response from load_instrument".into()))
    }

    /// Load an audio/midi effect by name onto a track.
    pub fn load_effect(&self, track_idx: i32, name: &str) -> Result<String> {
        let resp = self.osc.query_timeout(
            "/live/browser/load_effect",
            &[Arg::Int(track_idx), Arg::from(name)],
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
            .ok_or_else(|| Error::Ableton("no response from load_effect".into()))
    }
}
