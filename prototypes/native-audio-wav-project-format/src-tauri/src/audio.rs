use anyhow::{Context, Result, anyhow, bail};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, SampleRate, Stream, StreamConfig};
use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, bounded};
use hound::{SampleFormat as WavSampleFormat, WavReader, WavSpec, WavWriter};
use serde::Serialize;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDescription {
    pub name: String,
    pub is_default: bool,
    pub settings: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceList {
    pub inputs: Vec<DeviceDescription>,
    pub outputs: Vec<DeviceDescription>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingStatus {
    pub active: bool,
    pub paused: bool,
    pub elapsed_seconds: u64,
    pub source_setting: Option<String>,
    pub failure: Option<String>,
}

#[derive(Clone, Debug)]
pub struct RecordingOutcome {
    pub take_id: String,
    pub relative_path: String,
    pub source_device: String,
    pub source_channels: u16,
    pub source_sample_rate: u32,
    pub source_sample_format: String,
    pub wav_sample_rate: u32,
    pub frame_count: u64,
    pub peak_dbfs: f32,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportOutcome {
    pub path: String,
    pub encoder: String,
    pub sample_rate: u32,
    pub bit_rate_kbps: u16,
    pub channel_mode: String,
}

pub struct AudioRuntime {
    command_sender: Sender<AudioCommand>,
    shared_status: Arc<Mutex<SharedStatus>>,
    pub input_name: Option<String>,
    pub output_name: Option<String>,
}

#[derive(Default)]
struct SharedStatus {
    active: bool,
    paused: bool,
    started_at: Option<Instant>,
    source_setting: Option<String>,
    failure: Option<String>,
}

enum AudioCommand {
    Start {
        project_dir: PathBuf,
        input_name: Option<String>,
        response: Sender<std::result::Result<(), String>>,
    },
    Pause {
        response: Sender<std::result::Result<(), String>>,
    },
    Resume {
        response: Sender<std::result::Result<(), String>>,
    },
    Stop {
        project_dir: PathBuf,
        response: Sender<std::result::Result<RecordingOutcome, String>>,
    },
    Play {
        path: PathBuf,
        start_frame: u64,
        end_frame: u64,
        output_name: Option<String>,
        response: Sender<std::result::Result<(), String>>,
    },
}

struct RecordingSession {
    stream: Stream,
    sender: Sender<WriterMessage>,
    writer: JoinHandle<Result<WriterSummary>>,
    paused: Arc<AtomicBool>,
    temp_path: PathBuf,
    final_path: PathBuf,
    take_id: String,
    source_device: String,
    source_channels: u16,
    source_sample_rate: u32,
    source_sample_format: String,
}

struct PlaybackSession {
    _stream: Stream,
    ends_at: Instant,
}

enum WriterMessage {
    Frames(Vec<i32>),
    Stop,
}

struct WriterSummary {
    frame_count: u64,
    peak: f32,
}

impl AudioRuntime {
    pub fn new() -> Self {
        let (command_sender, command_receiver) = bounded(16);
        let shared_status = Arc::new(Mutex::new(SharedStatus::default()));
        let worker_status = shared_status.clone();
        thread::spawn(move || audio_worker(command_receiver, worker_status));
        Self {
            command_sender,
            shared_status,
            input_name: None,
            output_name: None,
        }
    }

    pub fn devices(&self) -> Result<DeviceList> {
        let host = cpal::default_host();
        let default_input = host
            .default_input_device()
            .and_then(|device| device.name().ok());
        let default_output = host
            .default_output_device()
            .and_then(|device| device.name().ok());
        let inputs = host
            .input_devices()?
            .map(|device| describe_input(&device, default_input.as_deref()))
            .collect::<Result<Vec<_>>>()?;
        let outputs = host
            .output_devices()?
            .map(|device| describe_output(&device, default_output.as_deref()))
            .collect::<Result<Vec<_>>>()?;
        Ok(DeviceList { inputs, outputs })
    }

    pub fn select_devices(&mut self, input_name: String, output_name: String) {
        self.input_name = Some(input_name);
        self.output_name = Some(output_name);
    }

    pub fn status(&self) -> RecordingStatus {
        let Ok(status) = self.shared_status.lock() else {
            return RecordingStatus {
                active: false,
                paused: false,
                elapsed_seconds: 0,
                source_setting: None,
                failure: Some("The audio status lock failed.".into()),
            };
        };
        RecordingStatus {
            active: status.active,
            paused: status.paused,
            elapsed_seconds: status
                .started_at
                .map(|started_at| started_at.elapsed().as_secs())
                .unwrap_or(0),
            source_setting: status.source_setting.clone(),
            failure: status.failure.clone(),
        }
    }

    pub fn start_recording(&self, project_dir: &Path) -> Result<()> {
        self.request(|response| AudioCommand::Start {
            project_dir: project_dir.to_path_buf(),
            input_name: self.input_name.clone(),
            response,
        })
    }

    pub fn pause(&self) -> Result<()> {
        self.request(|response| AudioCommand::Pause { response })
    }

    pub fn resume(&self) -> Result<()> {
        self.request(|response| AudioCommand::Resume { response })
    }

    pub fn stop_recording(&self, project_dir: &Path) -> Result<RecordingOutcome> {
        self.request(|response| AudioCommand::Stop {
            project_dir: project_dir.to_path_buf(),
            response,
        })
    }

    pub fn play(&self, path: &Path, start_frame: u64, end_frame: u64) -> Result<()> {
        self.request(|response| AudioCommand::Play {
            path: path.to_path_buf(),
            start_frame,
            end_frame,
            output_name: self.output_name.clone(),
            response,
        })
    }

    fn request<T>(
        &self,
        make_command: impl FnOnce(Sender<std::result::Result<T, String>>) -> AudioCommand,
    ) -> Result<T> {
        let (response_sender, response_receiver) = bounded(1);
        self.command_sender
            .send(make_command(response_sender))
            .context("The audio worker is not available")?;
        response_receiver
            .recv()
            .context("The audio worker did not return a result")?
            .map_err(anyhow::Error::msg)
    }

    pub fn encoder_version() -> String {
        Command::new("lame")
            .arg("--version")
            .output()
            .ok()
            .filter(|output| output.status.success())
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|text| text.lines().next().map(str::to_string))
            .unwrap_or_else(|| "LAME executable not found".into())
    }

    pub fn export_mp3(
        &self,
        source_path: &Path,
        project_dir: &Path,
        start_frame: u64,
        end_frame: u64,
    ) -> Result<ExportOutcome> {
        let export_id = Uuid::new_v4().to_string();
        let export_dir = project_dir.join("exports").join(export_id);
        fs::create_dir_all(&export_dir)?;
        let trimmed_path = export_dir.join("trimmed-source.wav");
        write_trimmed_wav(source_path, &trimmed_path, start_frame, end_frame)?;
        let output_path = export_dir.join("prototype-export.mp3");
        let output = Command::new("lame")
            .args([
                "--silent",
                "--resample",
                "44.1",
                "-b",
                "192",
                "--cbr",
                "-m",
                "m",
            ])
            .arg(&trimmed_path)
            .arg(&output_path)
            .output()
            .context("The prototype could not start LAME")?;
        if !output.status.success() {
            bail!("LAME failed: {}", String::from_utf8_lossy(&output.stderr));
        }
        let (sample_rate, bit_rate_kbps, channel_mode) = read_mp3_header(&output_path)?;
        Ok(ExportOutcome {
            path: output_path.display().to_string(),
            encoder: Self::encoder_version(),
            sample_rate,
            bit_rate_kbps,
            channel_mode,
        })
    }
}

fn audio_worker(receiver: Receiver<AudioCommand>, status: Arc<Mutex<SharedStatus>>) {
    let mut recording: Option<RecordingSession> = None;
    let mut playback: Option<PlaybackSession> = None;
    loop {
        let command = if let Some(session) = playback.as_ref() {
            let wait = session.ends_at.saturating_duration_since(Instant::now());
            match receiver.recv_timeout(wait) {
                Ok(command) => command,
                Err(RecvTimeoutError::Timeout) => {
                    playback = None;
                    continue;
                }
                Err(RecvTimeoutError::Disconnected) => break,
            }
        } else {
            match receiver.recv() {
                Ok(command) => command,
                Err(_) => break,
            }
        };
        match command {
            AudioCommand::Start {
                project_dir,
                input_name,
                response,
            } => {
                playback = None;
                let result = if recording.is_some() {
                    Err("A Take is already recording.".into())
                } else {
                    start_session(&project_dir, input_name.as_deref(), status.clone())
                        .map(|session| {
                            if let Ok(mut current) = status.lock() {
                                current.active = true;
                                current.paused = false;
                                current.started_at = Some(Instant::now());
                                current.source_setting = Some(format!(
                                    "{} channel(s), {} Hz, {}",
                                    session.source_channels,
                                    session.source_sample_rate,
                                    session.source_sample_format
                                ));
                                current.failure = None;
                            }
                            recording = Some(session);
                        })
                        .map_err(|error| error.to_string())
                };
                let _ = response.send(result);
            }
            AudioCommand::Pause { response } => {
                let result = recording
                    .as_mut()
                    .context("No Take is recording")
                    .and_then(|session| {
                        session
                            .stream
                            .pause()
                            .context("The input stream did not pause")?;
                        session.paused.store(true, Ordering::Relaxed);
                        if let Ok(mut current) = status.lock() {
                            current.paused = true;
                        }
                        Ok(())
                    })
                    .map_err(|error| error.to_string());
                let _ = response.send(result);
            }
            AudioCommand::Resume { response } => {
                let result = recording
                    .as_mut()
                    .context("No Take is recording")
                    .and_then(|session| {
                        session
                            .stream
                            .play()
                            .context("The input stream did not continue")?;
                        session.paused.store(false, Ordering::Relaxed);
                        if let Ok(mut current) = status.lock() {
                            current.paused = false;
                        }
                        Ok(())
                    })
                    .map_err(|error| error.to_string());
                let _ = response.send(result);
            }
            AudioCommand::Stop {
                project_dir,
                response,
            } => {
                let result = recording
                    .take()
                    .context("No Take is recording")
                    .and_then(|session| stop_session(session, &project_dir))
                    .map_err(|error| error.to_string());
                if let Ok(mut current) = status.lock() {
                    current.active = false;
                    current.paused = false;
                    current.started_at = None;
                    current.source_setting = None;
                }
                let _ = response.send(result);
            }
            AudioCommand::Play {
                path,
                start_frame,
                end_frame,
                output_name,
                response,
            } => {
                let result = if recording.is_some() {
                    Err("Stop the active Take before playback.".into())
                } else {
                    start_playback(&path, start_frame, end_frame, output_name.as_deref())
                        .map(|session| playback = Some(session))
                        .map_err(|error| error.to_string())
                };
                let _ = response.send(result);
            }
        }
    }
}

fn start_session(
    project_dir: &Path,
    input_name: Option<&str>,
    status: Arc<Mutex<SharedStatus>>,
) -> Result<RecordingSession> {
    let host = cpal::default_host();
    let device = find_input_device(&host, input_name)?;
    let device_name = device.name()?;
    let supported = choose_input_config(&device)?;
    let sample_format = supported.sample_format();
    let config = supported.config();
    let take_id = Uuid::new_v4().to_string();
    let take_dir = project_dir.join("takes");
    fs::create_dir_all(&take_dir)?;
    let temp_path = take_dir.join(format!("{take_id}.recording.wav"));
    let final_path = take_dir.join(format!("{take_id}.wav"));
    let (sender, receiver) = bounded::<WriterMessage>(128);
    let writer_path = temp_path.clone();
    let wav_rate = config.sample_rate.0;
    let writer = thread::spawn(move || write_wav(receiver, &writer_path, wav_rate));
    let paused = Arc::new(AtomicBool::new(false));
    let stream = build_input_stream(
        &device,
        &config,
        sample_format,
        sender.clone(),
        paused.clone(),
        status,
    )?;
    stream.play().context("The input stream did not start")?;
    Ok(RecordingSession {
        stream,
        sender,
        writer,
        paused,
        temp_path,
        final_path,
        take_id,
        source_device: device_name,
        source_channels: config.channels,
        source_sample_rate: config.sample_rate.0,
        source_sample_format: format!("{sample_format:?}"),
    })
}

fn stop_session(session: RecordingSession, project_dir: &Path) -> Result<RecordingOutcome> {
    drop(session.stream);
    session.sender.send(WriterMessage::Stop)?;
    let summary = session
        .writer
        .join()
        .map_err(|_| anyhow!("The WAV writer thread stopped unexpectedly"))??;
    if summary.frame_count == 0 {
        bail!("The input device returned no audio frames. Select a different input device.");
    }
    let reader = WavReader::open(&session.temp_path)?;
    if reader.duration() as u64 != summary.frame_count {
        bail!("The completed WAV frame count does not match its header.");
    }
    drop(reader);
    fs::rename(&session.temp_path, &session.final_path)?;
    let relative_path = session
        .final_path
        .strip_prefix(project_dir)?
        .to_string_lossy()
        .replace('\\', "/");
    Ok(RecordingOutcome {
        take_id: session.take_id,
        relative_path,
        source_device: session.source_device,
        source_channels: session.source_channels,
        source_sample_rate: session.source_sample_rate,
        source_sample_format: session.source_sample_format,
        wav_sample_rate: session.source_sample_rate,
        frame_count: summary.frame_count,
        peak_dbfs: if summary.peak > 0.0 {
            20.0 * summary.peak.log10()
        } else {
            -120.0
        },
    })
}

fn start_playback(
    path: &Path,
    start_frame: u64,
    end_frame: u64,
    output_name: Option<&str>,
) -> Result<PlaybackSession> {
    if start_frame >= end_frame {
        bail!("The selected Take Trim has no audio frames.");
    }
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    if spec.channels != 1 || spec.sample_format != WavSampleFormat::Int {
        bail!("The playback test supports only the prototype mono integer WAV format.");
    }
    let samples = reader
        .samples::<i32>()
        .skip(start_frame as usize)
        .take((end_frame - start_frame) as usize)
        .map(|sample| sample.map(|value| value as f32 / 8_388_607.0))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let host = cpal::default_host();
    let device = find_output_device(&host, output_name)?;
    let supported = device.default_output_config()?;
    let config = supported.config();
    if config.sample_rate.0 != spec.sample_rate {
        bail!(
            "The output device uses {} Hz. The Take uses {} Hz. This prototype does not resample playback.",
            config.sample_rate.0,
            spec.sample_rate
        );
    }
    let position = Arc::new(AtomicUsize::new(0));
    let stream = build_output_stream(
        &device,
        &config,
        supported.sample_format(),
        Arc::new(samples),
        position,
    )?;
    stream.play()?;
    let seconds = (end_frame - start_frame) as f64 / spec.sample_rate as f64;
    Ok(PlaybackSession {
        _stream: stream,
        ends_at: Instant::now() + Duration::from_secs_f64(seconds + 0.15),
    })
}

fn describe_input(device: &Device, default_name: Option<&str>) -> Result<DeviceDescription> {
    let name = device.name()?;
    let settings = device
        .supported_input_configs()?
        .map(|setting| {
            format!(
                "{} channel(s), {}-{} Hz, {:?}",
                setting.channels(),
                setting.min_sample_rate().0,
                setting.max_sample_rate().0,
                setting.sample_format()
            )
        })
        .collect();
    Ok(DeviceDescription {
        is_default: default_name == Some(name.as_str()),
        name,
        settings,
    })
}

fn describe_output(device: &Device, default_name: Option<&str>) -> Result<DeviceDescription> {
    let name = device.name()?;
    let settings = device
        .supported_output_configs()?
        .map(|setting| {
            format!(
                "{} channel(s), {}-{} Hz, {:?}",
                setting.channels(),
                setting.min_sample_rate().0,
                setting.max_sample_rate().0,
                setting.sample_format()
            )
        })
        .collect();
    Ok(DeviceDescription {
        is_default: default_name == Some(name.as_str()),
        name,
        settings,
    })
}

fn find_input_device(host: &cpal::Host, selected: Option<&str>) -> Result<Device> {
    if let Some(name) = selected {
        return host
            .input_devices()?
            .find(|device| device.name().ok().as_deref() == Some(name))
            .with_context(|| format!("The selected input device is not available: {name}"));
    }
    host.default_input_device()
        .context("No input device is available")
}

fn find_output_device(host: &cpal::Host, selected: Option<&str>) -> Result<Device> {
    if let Some(name) = selected {
        return host
            .output_devices()?
            .find(|device| device.name().ok().as_deref() == Some(name))
            .with_context(|| format!("The selected output device is not available: {name}"));
    }
    host.default_output_device()
        .context("No output device is available")
}

fn choose_input_config(device: &Device) -> Result<cpal::SupportedStreamConfig> {
    let mut settings = device.supported_input_configs()?.collect::<Vec<_>>();
    settings.sort_by_key(|setting| {
        (
            setting.channels() != 1,
            !(setting.min_sample_rate().0 <= 48_000 && setting.max_sample_rate().0 >= 48_000),
            sample_format_rank(setting.sample_format()),
        )
    });
    let setting = settings
        .into_iter()
        .next()
        .context("The input device has no supported setting")?;
    if setting.min_sample_rate().0 <= 48_000 && setting.max_sample_rate().0 >= 48_000 {
        Ok(setting.with_sample_rate(SampleRate(48_000)))
    } else {
        Ok(setting.with_max_sample_rate())
    }
}

fn sample_format_rank(format: SampleFormat) -> u8 {
    match format {
        SampleFormat::F32 => 0,
        SampleFormat::I32 => 1,
        SampleFormat::I16 => 2,
        SampleFormat::U16 => 3,
        _ => 4,
    }
}

fn build_input_stream(
    device: &Device,
    config: &StreamConfig,
    format: SampleFormat,
    sender: Sender<WriterMessage>,
    paused: Arc<AtomicBool>,
    status: Arc<Mutex<SharedStatus>>,
) -> Result<Stream> {
    let channels = config.channels as usize;
    macro_rules! stream {
        ($sample:ty, $convert:expr) => {{
            let tx = sender.clone();
            let is_paused = paused.clone();
            let queue_status = status.clone();
            let error_status = status.clone();
            device.build_input_stream(
                config,
                move |data: &[$sample], _| {
                    if is_paused.load(Ordering::Relaxed) {
                        return;
                    }
                    let frames = data
                        .chunks(channels)
                        .map(|frame| {
                            let sum: f32 = frame.iter().map($convert).sum();
                            let mono = (sum / frame.len() as f32).clamp(-1.0, 1.0);
                            (mono * 8_388_607.0) as i32
                        })
                        .collect::<Vec<_>>();
                    if tx.try_send(WriterMessage::Frames(frames)).is_err() {
                        is_paused.store(true, Ordering::Relaxed);
                        if let Ok(mut target) = queue_status.lock() {
                            target.paused = true;
                            target.failure = Some(
                                "The bounded WAV writer queue is full. Recording stopped safely."
                                    .into(),
                            );
                        }
                    }
                },
                move |error| {
                    if let Ok(mut target) = error_status.lock() {
                        target.failure = Some(format!("The input stream failed: {error}"));
                    }
                },
                None,
            )?
        }};
    }
    Ok(match format {
        SampleFormat::F32 => stream!(f32, |sample: &f32| *sample),
        SampleFormat::I32 => stream!(i32, |sample: &i32| *sample as f32 / i32::MAX as f32),
        SampleFormat::I16 => stream!(i16, |sample: &i16| *sample as f32 / i16::MAX as f32),
        SampleFormat::U16 => stream!(u16, |sample: &u16| (*sample as f32 - 32_768.0) / 32_768.0),
        other => bail!("The prototype does not support input format {other:?}"),
    })
}

fn write_wav(
    receiver: Receiver<WriterMessage>,
    path: &Path,
    sample_rate: u32,
) -> Result<WriterSummary> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 24,
        sample_format: WavSampleFormat::Int,
    };
    let file = File::create(path)?;
    let mut writer = WavWriter::new(BufWriter::new(file), spec)?;
    let mut frame_count = 0_u64;
    let mut peak = 0.0_f32;
    while let Ok(message) = receiver.recv() {
        match message {
            WriterMessage::Frames(frames) => {
                for sample in frames {
                    peak = peak.max((sample as f32 / 8_388_607.0).abs());
                    writer.write_sample(sample)?;
                    frame_count += 1;
                }
            }
            WriterMessage::Stop => break,
        }
    }
    writer.finalize()?;
    Ok(WriterSummary { frame_count, peak })
}

fn build_output_stream(
    device: &Device,
    config: &StreamConfig,
    format: SampleFormat,
    samples: Arc<Vec<f32>>,
    position: Arc<AtomicUsize>,
) -> Result<Stream> {
    let channels = config.channels as usize;
    macro_rules! stream {
        ($sample:ty, $convert:expr) => {{
            let source = samples.clone();
            let cursor = position.clone();
            device.build_output_stream(
                config,
                move |output: &mut [$sample], _| {
                    for frame in output.chunks_mut(channels) {
                        let index = cursor.fetch_add(1, Ordering::Relaxed);
                        let value = source.get(index).copied().unwrap_or(0.0);
                        for sample in frame {
                            *sample = $convert(value);
                        }
                    }
                },
                |_error| {},
                None,
            )?
        }};
    }
    Ok(match format {
        SampleFormat::F32 => stream!(f32, |value: f32| value),
        SampleFormat::I16 => stream!(i16, |value: f32| (value * i16::MAX as f32) as i16),
        SampleFormat::U16 => stream!(u16, |value: f32| ((value + 1.0) * 32_767.5) as u16),
        other => bail!("The prototype does not support output format {other:?}"),
    })
}

fn write_trimmed_wav(source: &Path, target: &Path, start_frame: u64, end_frame: u64) -> Result<()> {
    let mut reader = WavReader::open(source)?;
    let spec = reader.spec();
    let mut writer = WavWriter::create(target, spec)?;
    for sample in reader
        .samples::<i32>()
        .skip(start_frame as usize)
        .take((end_frame - start_frame) as usize)
    {
        writer.write_sample(sample?)?;
    }
    writer.finalize()?;
    Ok(())
}

fn read_mp3_header(path: &Path) -> Result<(u32, u16, String)> {
    let mut bytes = fs::read(path)?;
    if bytes.starts_with(b"ID3") && bytes.len() >= 10 {
        let size = ((bytes[6] as usize & 0x7f) << 21)
            | ((bytes[7] as usize & 0x7f) << 14)
            | ((bytes[8] as usize & 0x7f) << 7)
            | (bytes[9] as usize & 0x7f);
        bytes.drain(..(10 + size).min(bytes.len()));
    }
    let header = bytes
        .windows(4)
        .find(|part| part[0] == 0xff && part[1] & 0xe0 == 0xe0)
        .context("The export has no MPEG audio frame")?;
    let bit_rate_index = (header[2] >> 4) & 0x0f;
    let sample_rate_index = (header[2] >> 2) & 0x03;
    let bit_rate_kbps = [
        0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0,
    ][bit_rate_index as usize];
    let sample_rate = [44_100, 48_000, 32_000, 0][sample_rate_index as usize];
    let channel_mode = if header[3] >> 6 == 3 {
        "mono"
    } else {
        "stereo"
    }
    .to_string();
    Ok((sample_rate, bit_rate_kbps, channel_mode))
}
