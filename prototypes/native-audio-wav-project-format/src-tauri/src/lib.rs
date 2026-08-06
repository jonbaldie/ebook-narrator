mod audio;
mod epub;
mod model;

use audio::{AudioRuntime, DeviceList, ExportOutcome, RecordingStatus};
use chrono::Utc;
use model::{Project, Take, Trim, safe_project_relative_path};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

struct PrototypeState {
    project_dir: PathBuf,
    project: Mutex<Project>,
    audio: Mutex<AudioRuntime>,
    last_action: Mutex<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Snapshot {
    prototype_warning: &'static str,
    project_dir: String,
    project: Project,
    devices: DeviceList,
    recording: RecordingStatus,
    selected_input: Option<String>,
    selected_output: Option<String>,
    encoder: String,
    last_action: String,
}

#[tauri::command]
fn snapshot(state: State<'_, PrototypeState>) -> Result<Snapshot, String> {
    let audio = state.audio.lock().map_err(lock_error)?;
    Ok(Snapshot {
        prototype_warning: "PROTOTYPE — Test data can be deleted.",
        project_dir: state.project_dir.display().to_string(),
        project: state.project.lock().map_err(lock_error)?.clone(),
        devices: audio.devices().map_err(report_error)?,
        recording: audio.status(),
        selected_input: audio.input_name.clone(),
        selected_output: audio.output_name.clone(),
        encoder: AudioRuntime::encoder_version(),
        last_action: state.last_action.lock().map_err(lock_error)?.clone(),
    })
}

#[tauri::command]
fn create_fixture_epub(state: State<'_, PrototypeState>) -> Result<String, String> {
    let path = epub::create_fixture(&state.project_dir).map_err(report_error)?;
    set_action(
        &state,
        format!("Created fixture EPUB at {}", path.display()),
    )?;
    Ok(path.display().to_string())
}

#[tauri::command]
fn import_epub(path: String, state: State<'_, PrototypeState>) -> Result<usize, String> {
    let count = {
        let mut project = state.project.lock().map_err(lock_error)?;
        let count = epub::import(Path::new(&path), &state.project_dir, &mut project)
            .map_err(report_error)?;
        save_project(&state.project_dir, &project).map_err(report_error)?;
        count
    };
    set_action(
        &state,
        format!("Imported {count} Narration Segments from {path}"),
    )?;
    Ok(count)
}

#[tauri::command]
fn move_segment(offset: i32, state: State<'_, PrototypeState>) -> Result<(), String> {
    let index = {
        let mut project = state.project.lock().map_err(lock_error)?;
        project.move_segment(offset);
        let index = project.current_segment_index;
        save_project(&state.project_dir, &project).map_err(report_error)?;
        index
    };
    set_action(
        &state,
        format!("Moved the Teleprompter to Narration Segment {}", index + 1),
    )
}

#[tauri::command]
fn select_devices(
    input_name: String,
    output_name: String,
    state: State<'_, PrototypeState>,
) -> Result<(), String> {
    state
        .audio
        .lock()
        .map_err(lock_error)?
        .select_devices(input_name.clone(), output_name.clone());
    set_action(
        &state,
        format!("Selected input {input_name} and output {output_name}"),
    )
}

#[tauri::command]
fn start_recording(state: State<'_, PrototypeState>) -> Result<(), String> {
    if state
        .project
        .lock()
        .map_err(lock_error)?
        .current_segment()
        .is_none()
    {
        return Err("Import an EPUB before you record a Take.".into());
    }
    state
        .audio
        .lock()
        .map_err(lock_error)?
        .start_recording(&state.project_dir)
        .map_err(report_error)?;
    set_action(&state, "Started a new Take".into())
}

#[tauri::command]
fn pause_recording(state: State<'_, PrototypeState>) -> Result<(), String> {
    state
        .audio
        .lock()
        .map_err(lock_error)?
        .pause()
        .map_err(report_error)?;
    set_action(&state, "Paused the active Take".into())
}

#[tauri::command]
fn resume_recording(state: State<'_, PrototypeState>) -> Result<(), String> {
    state
        .audio
        .lock()
        .map_err(lock_error)?
        .resume()
        .map_err(report_error)?;
    set_action(&state, "Continued the active Take".into())
}

#[tauri::command]
fn stop_recording(state: State<'_, PrototypeState>) -> Result<String, String> {
    let outcome = state
        .audio
        .lock()
        .map_err(lock_error)?
        .stop_recording(&state.project_dir)
        .map_err(report_error)?;
    let take_id = outcome.take_id.clone();
    {
        let mut project = state.project.lock().map_err(lock_error)?;
        let segment_id = project
            .current_segment()
            .ok_or_else(|| "The current Narration Segment is missing.".to_string())?
            .id
            .clone();
        project.takes.push(Take {
            id: outcome.take_id.clone(),
            segment_id,
            relative_path: outcome.relative_path,
            created_at: Utc::now().to_rfc3339(),
            source_device: outcome.source_device,
            source_channels: outcome.source_channels,
            source_sample_rate: outcome.source_sample_rate,
            source_sample_format: outcome.source_sample_format,
            wav_channels: 1,
            wav_sample_rate: outcome.wav_sample_rate,
            wav_bits_per_sample: 24,
            frame_count: outcome.frame_count,
            peak_dbfs: outcome.peak_dbfs,
            trim: Trim {
                start_frame: 0,
                end_frame: outcome.frame_count,
            },
        });
        project.selected_take_id = Some(outcome.take_id);
        save_project(&state.project_dir, &project).map_err(report_error)?;
    }
    set_action(&state, format!("Stopped and validated Take {take_id}"))?;
    Ok(take_id)
}

#[tauri::command]
fn select_take(take_id: String, state: State<'_, PrototypeState>) -> Result<(), String> {
    {
        let mut project = state.project.lock().map_err(lock_error)?;
        if !project.takes.iter().any(|take| take.id == take_id) {
            return Err("The selected Take is not in this Project.".into());
        }
        project.selected_take_id = Some(take_id.clone());
        save_project(&state.project_dir, &project).map_err(report_error)?;
    }
    set_action(&state, format!("Selected Take {take_id}"))
}

#[tauri::command]
fn apply_trim(
    start_frame: u64,
    end_frame: u64,
    state: State<'_, PrototypeState>,
) -> Result<(), String> {
    {
        let mut project = state.project.lock().map_err(lock_error)?;
        let take = project
            .selected_take_mut()
            .ok_or_else(|| "Select a Take before you apply a Trim.".to_string())?;
        take.trim = Trim::checked(start_frame, end_frame, take.frame_count)?;
        save_project(&state.project_dir, &project).map_err(report_error)?;
    }
    set_action(
        &state,
        format!("Applied a non-destructive Trim from frame {start_frame} to frame {end_frame}"),
    )
}

#[tauri::command]
fn play_selected_take(state: State<'_, PrototypeState>) -> Result<(), String> {
    let (path, trim) = {
        let project = state.project.lock().map_err(lock_error)?;
        let take = project
            .selected_take()
            .ok_or_else(|| "Select a Take before playback.".to_string())?;
        let relative = safe_project_relative_path(&take.relative_path)?;
        (state.project_dir.join(relative), take.trim.clone())
    };
    state
        .audio
        .lock()
        .map_err(lock_error)?
        .play(&path, trim.start_frame, trim.end_frame)
        .map_err(report_error)?;
    set_action(&state, "Played the selected Take Trim".into())
}

#[tauri::command]
fn export_selected_take(state: State<'_, PrototypeState>) -> Result<ExportOutcome, String> {
    let (path, trim) = {
        let project = state.project.lock().map_err(lock_error)?;
        let take = project
            .selected_take()
            .ok_or_else(|| "Select a Take before export.".to_string())?;
        let relative = safe_project_relative_path(&take.relative_path)?;
        (state.project_dir.join(relative), take.trim.clone())
    };
    let result = state
        .audio
        .lock()
        .map_err(lock_error)?
        .export_mp3(&path, &state.project_dir, trim.start_frame, trim.end_frame)
        .map_err(report_error)?;
    set_action(&state, format!("Exported an MP3 to {}", result.path))?;
    Ok(result)
}

#[tauri::command]
fn record_observation(
    test: String,
    result: String,
    note: String,
    state: State<'_, PrototypeState>,
) -> Result<(), String> {
    let record = serde_json::json!({
        "recordedAt": Utc::now().to_rfc3339(),
        "test": test,
        "result": result,
        "note": note,
        "operatingSystem": std::env::consts::OS,
        "architecture": std::env::consts::ARCH,
    });
    let path = state.project_dir.join("test-observations.jsonl");
    let mut text = fs::read_to_string(&path).unwrap_or_default();
    text.push_str(&serde_json::to_string(&record).map_err(report_error)?);
    text.push('\n');
    fs::write(&path, text).map_err(report_error)?;
    set_action(&state, format!("Recorded the {test} result as {result}"))
}

fn initial_state() -> Result<PrototypeState, String> {
    let project_dir = std::env::temp_dir()
        .join("ebook-narrator-native-audio-prototype")
        .join("project.ebook-narrator");
    fs::create_dir_all(project_dir.join("takes")).map_err(report_error)?;
    fs::create_dir_all(project_dir.join("exports")).map_err(report_error)?;
    let project_path = project_dir.join("project.json");
    let project = if project_path.exists() {
        serde_json::from_slice(&fs::read(&project_path).map_err(report_error)?)
            .map_err(report_error)?
    } else {
        let project = Project::new(Uuid::new_v4().to_string());
        save_project(&project_dir, &project).map_err(report_error)?;
        project
    };
    Ok(PrototypeState {
        project_dir,
        project: Mutex::new(project),
        audio: Mutex::new(AudioRuntime::new()),
        last_action: Mutex::new("Opened the test Project".into()),
    })
}

fn save_project(project_dir: &Path, project: &Project) -> anyhow::Result<()> {
    let final_path = project_dir.join("project.json");
    let temp_path = project_dir.join("project.json.saving");
    let bytes = serde_json::to_vec_pretty(project)?;
    fs::write(&temp_path, bytes)?;
    fs::rename(temp_path, final_path)?;
    Ok(())
}

fn set_action(state: &State<'_, PrototypeState>, action: String) -> Result<(), String> {
    *state.last_action.lock().map_err(lock_error)? = action;
    Ok(())
}

fn lock_error<T>(error: std::sync::PoisonError<T>) -> String {
    format!("A prototype state lock failed: {error}")
}

fn report_error(error: impl std::fmt::Display) -> String {
    error.to_string()
}

pub fn run_safe_diagnostics() -> Result<String, String> {
    let state = initial_state()?;
    let fixture_path = epub::create_fixture(&state.project_dir).map_err(report_error)?;
    let (segment_count, teleprompter_moved) = {
        let mut project = state.project.lock().map_err(lock_error)?;
        let segment_count =
            epub::import(&fixture_path, &state.project_dir, &mut project).map_err(report_error)?;
        project.move_segment(1);
        let teleprompter_moved = project.current_segment_index == 1
            && project
                .current_segment()
                .is_some_and(|segment| segment.text.contains("Second test chapter"));
        save_project(&state.project_dir, &project).map_err(report_error)?;
        (segment_count, teleprompter_moved)
    };

    let persisted: Project = serde_json::from_slice(
        &fs::read(state.project_dir.join("project.json")).map_err(report_error)?,
    )
    .map_err(report_error)?;
    let synthetic_wav = state.project_dir.join("takes/safe-diagnostic.wav");
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48_000,
        bits_per_sample: 24,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&synthetic_wav, spec).map_err(report_error)?;
    for frame in 0..48_000 {
        let value =
            ((frame as f32 * 440.0 * std::f32::consts::TAU / 48_000.0).sin() * 1_000_000.0) as i32;
        writer.write_sample(value).map_err(report_error)?;
    }
    writer.finalize().map_err(report_error)?;

    let audio = state.audio.lock().map_err(lock_error)?;
    let devices = audio.devices().map_err(report_error)?;
    let export = audio
        .export_mp3(&synthetic_wav, &state.project_dir, 0, 48_000)
        .map_err(report_error)?;
    serde_json::to_string_pretty(&serde_json::json!({
        "projectDir": state.project_dir,
        "fixtureEpub": fixture_path,
        "segmentCount": segment_count,
        "teleprompterMoved": teleprompter_moved,
        "projectRestartRead": persisted.current_segment_index == 1 && persisted.segments.len() == segment_count,
        "inputDevices": devices.inputs,
        "outputDevices": devices.outputs,
        "encoder": AudioRuntime::encoder_version(),
        "syntheticWav": synthetic_wav,
        "mp3Export": export,
    }))
    .map_err(report_error)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = initial_state().expect("The prototype state must open");
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            snapshot,
            create_fixture_epub,
            import_epub,
            move_segment,
            select_devices,
            start_recording,
            pause_recording,
            resume_recording,
            stop_recording,
            select_take,
            apply_trim,
            play_selected_take,
            export_selected_take,
            record_observation,
        ])
        .run(tauri::generate_context!())
        .expect("The Tauri prototype failed");
}
