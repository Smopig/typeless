use anyhow::Context;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct AudioCapturer {
    stop_tx: crossbeam_channel::Sender<()>,
    buffer: Arc<Mutex<Vec<f32>>>,
    native_sample_rate: u32,
}

impl AudioCapturer {
    pub fn start() -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .context("No input device available")?;
        let config = device.default_input_config()?;
        let native_sample_rate = config.sample_rate().0;
        let channels = config.channels() as usize;

        let buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = buffer.clone();

        let (stop_tx, stop_rx) = crossbeam_channel::bounded(1);

        std::thread::spawn(move || {
            let stream = device
                .build_input_stream(
                    &config.into(),
                    move |data: &[f32], _| {
                        let mut buf = buffer_clone.lock();
                        // Mix down to mono
                        for frame in data.chunks(channels) {
                            let mono = frame.iter().sum::<f32>() / channels as f32;
                            buf.push(mono);
                        }
                    },
                    |err| log::error!("Audio stream error: {}", err),
                    None,
                )
                .expect("Failed to build input stream");

            stream.play().expect("Failed to start audio stream");
            // Block until stop signal
            let _ = stop_rx.recv();
            // stream dropped here, stopping capture
        });

        Ok(Self {
            stop_tx,
            buffer,
            native_sample_rate,
        })
    }

    pub fn stop(self) -> anyhow::Result<(Vec<f32>, u32)> {
        let _ = self.stop_tx.send(());
        // Small sleep to let the stream thread drain its last callback
        std::thread::sleep(std::time::Duration::from_millis(50));

        let samples = self.buffer.lock().clone();
        Ok((samples, self.native_sample_rate))
    }
}

/// Downsample from `from_rate` to `to_rate` using linear interpolation.
pub fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate {
        return samples.to_vec();
    }
    let ratio = from_rate as f64 / to_rate as f64;
    let out_len = (samples.len() as f64 / ratio).ceil() as usize;
    let mut out = Vec::with_capacity(out_len);
    for i in 0..out_len {
        let src_pos = i as f64 * ratio;
        let idx = src_pos as usize;
        let frac = (src_pos - idx as f64) as f32;
        let a = samples.get(idx).copied().unwrap_or(0.0);
        let b = samples.get(idx + 1).copied().unwrap_or(0.0);
        out.push(a + (b - a) * frac);
    }
    out
}
