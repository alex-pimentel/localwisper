use std::path::Path;

use anyhow::{Context, Result};

pub struct WavInfo {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub samples: Vec<f32>,
    pub duration_secs: f64,
}

pub fn read_wav(path: &Path) -> Result<WavInfo> {
    let data = std::fs::read(path).context("failed to read WAV file")?;
    parse_wav(&data)
}

pub fn parse_wav(data: &[u8]) -> Result<WavInfo> {
    if data.len() < 44 {
        anyhow::bail!("WAV file too short");
    }

    let channels = u16::from_le_bytes([data[22], data[23]]);
    let sample_rate = u32::from_le_bytes([data[24], data[25], data[26], data[27]]);
    let bits_per_sample = u16::from_le_bytes([data[34], data[35]]);

    let mut offset = 12;
    let mut audio_data: Option<&[u8]> = None;

    while offset + 8 <= data.len() {
        let chunk_id = &data[offset..offset + 4];
        let chunk_size = u32::from_le_bytes([
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]) as usize;

        if chunk_id == b"data" {
            if offset + 8 + chunk_size <= data.len() {
                audio_data = Some(&data[offset + 8..offset + 8 + chunk_size]);
            }
            break;
        }
        offset += 8 + chunk_size;
        if chunk_size % 2 != 0 {
            offset += 1;
        }
    }

    let raw = audio_data.context("no data chunk found in WAV file")?;
    let total_samples = raw.len() * 8 / bits_per_sample as usize / channels as usize;
    let mut samples = Vec::with_capacity(total_samples);

    match bits_per_sample {
        16 => {
            for frame in raw.chunks(2 * channels as usize) {
                if frame.len() < 2 * channels as usize {
                    break;
                }
                let mut sum = 0i64;
                for ch in 0..channels as usize {
                    let val = i16::from_le_bytes([frame[ch * 2], frame[ch * 2 + 1]]);
                    sum += val as i64;
                }
                samples.push(sum as f32 / channels as f32 / i16::MAX as f32);
            }
        }
        32 => {
            for frame in raw.chunks(4 * channels as usize) {
                if frame.len() < 4 * channels as usize {
                    break;
                }
                let mut sum = 0f32;
                for ch in 0..channels as usize {
                    let val = f32::from_le_bytes([
                        frame[ch * 4],
                        frame[ch * 4 + 1],
                        frame[ch * 4 + 2],
                        frame[ch * 4 + 3],
                    ]);
                    sum += val;
                }
                samples.push(sum / channels as f32);
            }
        }
        8 => {
            for &byte in raw.iter() {
                samples.push((byte as f32 - 128.0) / 128.0);
            }
        }
        _ => anyhow::bail!("unsupported bits per sample: {}", bits_per_sample),
    }

    let duration_secs = samples.len() as f64 / sample_rate as f64;

    Ok(WavInfo {
        sample_rate,
        channels,
        bits_per_sample,
        samples,
        duration_secs,
    })
}

pub fn generate_sine_wav(
    path: &Path,
    duration_secs: f64,
    sample_rate: u32,
    frequency_hz: f64,
) -> Result<()> {
    let num_samples = (sample_rate as f64 * duration_secs) as usize;
    let mut data = Vec::with_capacity(num_samples * 2);

    for i in 0..num_samples {
        let t = i as f64 / sample_rate as f64;
        let sample = (2.0 * std::f64::consts::PI * frequency_hz * t).sin() as f32;
        let amplitude = 0.3;
        let modulated = sample * amplitude;
        let int_val = (modulated * i16::MAX as f32) as i16;
        data.extend_from_slice(&int_val.to_le_bytes());
    }

    let data_size = data.len() as u32;
    let file_size = 36 + data_size;
    let byte_rate = sample_rate * 2;
    let block_align: u16 = 2;

    let mut wav = Vec::new();
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&file_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&block_align.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    wav.extend_from_slice(&data);

    std::fs::write(path, &wav).context("failed to write WAV file")?;
    Ok(())
}

pub fn mix_speech_like_wav(path: &Path, duration_secs: f64) -> Result<()> {
    let sample_rate: u32 = 16000;
    let num_samples = (sample_rate as f64 * duration_secs) as usize;
    let mut data = Vec::with_capacity(num_samples * 2);

    let frequencies = [
        120.0, 180.0, 250.0, 350.0, 450.0, 200.0, 300.0, 400.0, 220.0, 280.0,
    ];

    for i in 0..num_samples {
        let t = i as f64 / sample_rate as f64;
        let mut sample: f64 = 0.0;
        let freq_idx = (i * frequencies.len() / num_samples) % frequencies.len();
        let base_freq = frequencies[freq_idx];

        for harmonic in 1..5 {
            let amp = 0.15 / harmonic as f64;
            sample += amp * (2.0 * std::f64::consts::PI * base_freq * harmonic as f64 * t).sin();
        }

        let amplitude = 0.5;
        let envelope = if t < 0.05 {
            t / 0.05
        } else if t > duration_secs - 0.05 {
            (duration_secs - t) / 0.05
        } else {
            1.0
        };

        let modulated = sample * amplitude * envelope;
        let int_val = (modulated * i16::MAX as f64) as i16;
        data.extend_from_slice(&int_val.to_le_bytes());
    }

    let data_size = data.len() as u32;
    let file_size = 36 + data_size;
    let byte_rate: u32 = sample_rate * 2;
    let block_align: u16 = 2;

    let mut wav = Vec::new();
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&file_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&block_align.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    wav.extend_from_slice(&data);

    std::fs::write(path, &wav).context("failed to write speech WAV file")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::whisper::engine::WhisperEngine;
    use crate::whisper::model;
    use std::path::Path;

    fn model_available() -> Option<String> {
        let downloaded = model::list_downloaded();
        if downloaded.is_empty() {
            return None;
        }
        if downloaded.contains(&"base".to_string()) {
            return Some("base".to_string());
        }
        downloaded.first().cloned()
    }

    #[test]
    fn test_generate_and_read_sine_wav() {
        let dir = std::env::temp_dir().join(format!(
            "wav_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("sine.wav");

        generate_sine_wav(&path, 1.0, 16000, 440.0).unwrap();
        assert!(path.exists());

        let info = read_wav(&path).unwrap();
        assert_eq!(info.sample_rate, 16000);
        assert_eq!(info.channels, 1);
        assert_eq!(info.bits_per_sample, 16);
        assert!((info.duration_secs - 1.0).abs() < 0.01);
        assert_eq!(info.samples.len(), 16000);
    }

    #[test]
    fn test_generate_and_read_speech_wav() {
        let dir = std::env::temp_dir().join(format!(
            "wav_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("speech.wav");

        mix_speech_like_wav(&path, 0.5).unwrap();
        let info = read_wav(&path).unwrap();
        assert_eq!(info.sample_rate, 16000);
        assert!((info.duration_secs - 0.5).abs() < 0.01);
        assert!(!info.samples.is_empty());
    }

    #[test]
    fn test_read_nonexistent_wav() {
        let result = read_wav(Path::new("/nonexistent/file.wav"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_wav() {
        let result = parse_wav(&[0u8; 10]);
        assert!(result.is_err());
    }

    #[test]
    fn test_speech_wav_has_varied_samples() {
        let dir = std::env::temp_dir().join(format!(
            "wav_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("varied.wav");

        mix_speech_like_wav(&path, 1.0).unwrap();
        let info = read_wav(&path).unwrap();

        let mean = info.samples.iter().copied().sum::<f32>() / info.samples.len() as f32;
        assert!(
            mean.abs() < 0.05,
            "speech samples should be centered around zero"
        );

        let variance = info.samples.iter().map(|s| (s - mean).powi(2)).sum::<f32>()
            / info.samples.len() as f32;
        assert!(
            variance > 0.001,
            "speech samples should have significant variance"
        );
    }

    #[test]
    #[ignore]
    fn e2e_transcribe_generated_speech() {
        let model_name = model_available().expect(
            "no whisper model downloaded. run `cargo test e2e_transcribe -- --ignored --nocapture` after downloading a model via the app",
        );

        let dir = std::env::temp_dir().join(format!(
            "e2e_transcribe_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("speech.wav");

        mix_speech_like_wav(&path, 3.0).unwrap();
        let wav_info = read_wav(&path).unwrap();

        let engine = WhisperEngine::new(&model_name).unwrap();
        let result = engine.transcribe(&wav_info.samples, Some("en")).unwrap();

        assert!(
            !result.full_text.is_empty(),
            "transcription should produce text"
        );
        assert!(
            !result.segments.is_empty(),
            "should have at least one segment"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    #[ignore]
    fn e2e_transcribe_from_command() {
        let model_name = model_available()
            .expect("no whisper model downloaded. run after downloading a model via the app");

        let dir = std::env::temp_dir().join(format!(
            "e2e_command_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("speech.wav");

        mix_speech_like_wav(&path, 3.0).unwrap();

        let opts = serde_json::json!({ "model": model_name, "language": "en" }).to_string();
        let result = crate::commands::audio::transcribe_audio_file(
            path.to_string_lossy().to_string(),
            Some(opts),
        );

        assert!(
            result.is_ok(),
            "transcribe_audio_file should succeed: {:?}",
            result.err()
        );
        let text = result.unwrap();
        assert!(!text.is_empty(), "transcription should produce text");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    #[ignore]
    fn benchmark_transcription_timing() {
        let model_name = model_available()
            .expect("no whisper model downloaded. run after downloading a model via the app");

        let durations = [1.0, 5.0, 10.0, 30.0, 60.0];
        let dir = std::env::temp_dir().join(format!(
            "benchmark_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();

        println!(
            "\n{:>8} | {:>12} | {:>12} | {:>12}",
            "Duration", "Audio Len", "Transcribe", "Real-time"
        );
        println!("{:-<8}-+-{:-<12}-+-{:-<12}-+-{:-<12}", "", "", "", "");

        for &dur in &durations {
            let path = dir.join(format!("speech_{}s.wav", dur as u64));
            mix_speech_like_wav(&path, dur).unwrap();
            let wav_info = read_wav(&path).unwrap();

            let engine = WhisperEngine::new(&model_name).unwrap();

            let start = std::time::Instant::now();
            let result = engine.transcribe(&wav_info.samples, Some("en")).unwrap();
            let elapsed = start.elapsed();

            let audio_len = wav_info.duration_secs;
            let realtime_ratio = elapsed.as_secs_f64() / audio_len;

            println!(
                "{:>6.0}s audio | {:>10.2}s | {:>10.3}s | {:>10.2}x",
                dur,
                audio_len,
                elapsed.as_secs_f64(),
                realtime_ratio
            );

            assert!(
                !result.full_text.is_empty(),
                "transcription should produce text"
            );
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
