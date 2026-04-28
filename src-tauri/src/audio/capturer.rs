use anyhow::Context;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, SizedSample};
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
            .context("No input device available. Check microphone permissions.")?;

        let config = device.default_input_config()?;
        let native_sample_rate = config.sample_rate().0;
        let channels = config.channels() as usize;
        let sample_format = config.sample_format();
        let stream_config: cpal::StreamConfig = config.into();

        let buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = buffer.clone();
        let (stop_tx, stop_rx) = crossbeam_channel::bounded(1);

        std::thread::spawn(move || {
            let err_fn = |err| log::error!("Audio stream error: {}", err);

            let stream = match sample_format {
                SampleFormat::F32 => device.build_input_stream(
                    &stream_config,
                    build_mono_callback::<f32>(buffer_clone, channels),
                    err_fn,
                    None,
                ),
                SampleFormat::I16 => device.build_input_stream(
                    &stream_config,
                    build_mono_callback::<i16>(buffer_clone, channels),
                    err_fn,
                    None,
                ),
                SampleFormat::U16 => device.build_input_stream(
                    &stream_config,
                    build_mono_callback::<u16>(buffer_clone, channels),
                    err_fn,
                    None,
                ),
                _ => {
                    log::warn!("Unsupported sample format {:?}, falling back to f32", sample_format);
                    device.build_input_stream(
                        &stream_config,
                        build_mono_callback::<f32>(buffer_clone, channels),
                        err_fn,
                        None,
                    )
                }
            }
            .expect("Failed to build input stream");

            stream.play().expect("Failed to start audio stream");
            let _ = stop_rx.recv();
        });

        Ok(Self {
            stop_tx,
            buffer,
            native_sample_rate,
        })
    }

    pub fn stop(self) -> anyhow::Result<(Vec<f32>, u32)> {
        let _ = self.stop_tx.send(());
        std::thread::sleep(std::time::Duration::from_millis(50));
        let samples = self.buffer.lock().clone();
        Ok((samples, self.native_sample_rate))
    }
}

fn build_mono_callback<T>(
    buffer: Arc<Mutex<Vec<f32>>>,
    channels: usize,
) -> impl FnMut(&[T], &cpal::InputCallbackInfo) + Send + 'static
where
    T: SizedSample + Send + 'static,
    f32: FromSample<T>,
{
    move |data: &[T], _| {
        let mut buf = buffer.lock();
        for frame in data.chunks(channels) {
            let mono: f32 =
                frame.iter().map(|s| f32::from_sample(*s)).sum::<f32>() / channels as f32;
            buf.push(mono);
        }
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
