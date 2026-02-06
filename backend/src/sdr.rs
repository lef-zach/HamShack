use crossbeam_channel::{bounded, Receiver, Sender};
use num_complex::Complex32;
use rand::Rng;
use rayon::prelude::*;
use rustfft::algorithm::Radix4;
use rustfft::Fft;
use serde::Serialize;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SDRConfig {
    pub device: String,
    pub sample_rate: u32,
    pub frequency: u64,
    pub gain: f32,
    pub fft_size: usize,
}

#[derive(Debug, Clone)]
pub struct SpectrumData {
    pub frequency: u64,
    pub spectrum: Vec<f32>,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct SDRManager {
    config: SDRConfig,
    is_running: bool,
    spectrum_tx: Option<Sender<SpectrumData>>,
    spectrum_rx: Option<Receiver<SpectrumData>>,
}

impl SDRManager {
    pub fn new() -> Self {
        let (spectrum_tx, spectrum_rx) = bounded(100);

        Self {
            config: SDRConfig {
                device: "rtlsdr".to_string(),
                sample_rate: 2400000,
                frequency: 14200000, // 14.2 MHz
                gain: 30.0,
                fft_size: 1024,
            },
            is_running: false,
            spectrum_tx: Some(spectrum_tx),
            spectrum_rx: Some(spectrum_rx),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.is_running {
            return Err("SDR already running".to_string());
        }

        println!(
            "Starting SDR: {} @ {} Hz",
            self.config.device, self.config.frequency
        );

        let config = self.config.clone();
        let spectrum_tx = self.spectrum_tx.take().unwrap();

        thread::spawn(move || {
            let mut sdr_thread = SDRThread::new(config, spectrum_tx);
            sdr_thread.run();
        });

        self.is_running = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if !self.is_running {
            return Err("SDR not running".to_string());
        }

        println!("Stopping SDR");
        self.is_running = false;
        self.spectrum_tx.take(); // This will break the SDR thread

        Ok(())
    }

    pub fn set_frequency(&mut self, freq: u64) -> Result<(), String> {
        if !self.is_running {
            return Err("SDR not running".to_string());
        }

        self.config.frequency = freq;
        println!("Tuning to {} Hz", freq);

        Ok(())
    }

    pub fn get_status(&self) -> SDRStatus {
        SDRStatus {
            is_running: self.is_running,
            frequency: self.config.frequency,
            sample_rate: self.config.sample_rate,
            gain: self.config.gain,
        }
    }

    pub fn get_spectrum_data(&self) -> Option<SpectrumData> {
        if !self.is_running {
            return None;
        }

        // Get latest spectrum data from receiver
        match self.spectrum_rx.as_ref().unwrap().try_recv() {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }
}

struct SDRThread {
    config: SDRConfig,
    spectrum_tx: Sender<SpectrumData>,
    fft: Arc<dyn Fft<f32>>,
}

impl SDRThread {
    fn new(config: SDRConfig, spectrum_tx: Sender<SpectrumData>) -> Self {
        let fft = Arc::new(Radix4::new(config.fft_size, rustfft::FftDirection::Forward));

        Self {
            config,
            spectrum_tx,
            fft,
        }
    }

    fn run(&mut self) {
        println!("SDR thread started");

        // Simulate SDR data acquisition
        let mut buffer = vec![Complex32::new(0.0, 0.0); self.config.fft_size];

        loop {
            // Generate simulated IQ data
            self.generate_simulated_iq(&mut buffer);

            // Perform FFT
            let mut spectrum = buffer.clone();
            self.fft.process(&mut spectrum);

            // Convert to power spectrum
            let power_spectrum: Vec<f32> = spectrum
                .par_iter()
                .map(|c| c.norm_sqr().log10() * 10.0) // dB scale
                .collect();

            // Send spectrum data
            let spectrum_data = SpectrumData {
                frequency: self.config.frequency,
                spectrum: power_spectrum,
                timestamp: Instant::now(),
            };

            if self.spectrum_tx.send(spectrum_data).is_err() {
                break; // Receiver dropped
            }

            thread::sleep(Duration::from_millis(100)); // 10 FPS
        }
    }

    fn generate_simulated_iq(&self, buffer: &mut [Complex32]) {
        let sample_rate = self.config.sample_rate as f32;
        let center_freq = self.config.frequency as f32;
        let mut rng = rand::thread_rng();

        for (i, sample) in buffer.iter_mut().enumerate() {
            let t = i as f32 / sample_rate;

            // Simulate multiple signals
            let signal1 = (2.0 * std::f32::consts::PI * center_freq * t).sin();
            let signal2 = (2.0 * std::f32::consts::PI * (center_freq + 10000.0) * t).sin() * 0.5;
            let noise = (rng.gen::<f32>() - 0.5) * 0.1;

            let i = signal1 + signal2 + noise;
            let q = (2.0 * std::f32::consts::PI * center_freq * t + std::f32::consts::PI / 2.0)
                .sin()
                + (2.0 * std::f32::consts::PI * (center_freq + 10000.0) * t
                    + std::f32::consts::PI / 2.0)
                    .sin()
                    * 0.5
                + noise;

            *sample = Complex32::new(i, q);
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SDRStatus {
    pub is_running: bool,
    pub frequency: u64,
    pub sample_rate: u32,
    pub gain: f32,
}
