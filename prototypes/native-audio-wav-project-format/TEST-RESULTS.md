# Prototype Test Results

This file records direct observations. Do not record an expected result as a pass.

## Decision to stop

On 2026-07-31 the Narrator stopped further prototype testing. The macOS core path already answered the design question. Remaining edge cases and Windows checks move to release tests during build. They are not required to accept the design for the production specification.

## macOS

- Status: Stopped after core-path proof
- Operating system: macOS 26.5.2 (25F84), Apple silicon
- Input device: Not selected
- Output device: Not selected
- Source-audio setting: Not selected
- WAV setting: Mono, 48 kHz when the input device supports it, 24-bit integer PCM
- Encoder: LAME 3.100 is present

Safe diagnostic on 2026-07-31:

- The fixture EPUB imported two spine items.
- The Project record saved and opened with Narration Segment 2 selected.
- CPAL listed three input devices and two output devices.
- The default iMac Microphone offers mono, 48 kHz, F32 input.
- LAME exported the synthetic PCM WAV fixture as 44.1 kHz, 192 kbps, mono MP3.
- The Narrator confirmed that the Tauri window opened and the visible device-list flow worked.
- The first visible restart failed because two silent Takes had a `null` peak level.
- The corrected loader opened that Project record and saved each silent peak level as -120 dBFS.
- A later session showed a beach ball. A short Take did not make sound during manual playback.
- The saved Project record showed that the visible device control selected `iPhone Microphone` instead of the default `iMac Microphone`. The recorded Take had a -120 dBFS peak level.
- After the corrections, the Narrator selected the default input, recorded a short Take, and heard it during playback. The Narration Application stayed responsive.
- The Narrator confirmed that Pause and Continue kept speech from before and after the pause, and excluded speech made during the pause.
- The Narrator selected and played two different Takes. Each selection played the correct recording.
- The first Trim check removed one half-second. The Narrator could not hear a clear difference, so the result was inconclusive.
- A clear Trim check kept “KEEP” and omitted “REMOVE.” The Project Trim starts at frame 144000, while the source WAV still contains all 380416 frames.
- After restart, the selected Take kept its Trim and still played “KEEP” without “REMOVE.”
- The visible export reported 44.1 kHz, 192 kbps, and mono. The exported MP3 had these values and a trimmed duration of 4.96 seconds.

| Test | Result | Failure behavior or observation |
| --- | --- | --- |
| Local EPUB import | Partial pass | The core imported two ordered spine items. The visible flow is not reviewed. |
| Manual Teleprompter movement | Partial pass | The Project position changed and persisted. The visible controls are not reviewed. |
| Input and output device list | Pass | CPAL listed three input devices and two output devices. The Narrator confirmed the visible flow. |
| Three-second microphone test | Pass | The Narrator confirmed that short microphone capture and playback worked with the default input. |
| 60-minute Take | Not run | |
| Pause and continue | Pass | Playback contained speech from before and after the pause. It did not contain speech made during the pause. |
| Playback | Pass | The first manual check failed. After the corrections, the Narrator heard the short Take and the Narration Application stayed responsive. |
| Take selection | Pass | Two different selected Takes played the correct recordings. |
| Non-destructive Trim | Pass | Playback kept “KEEP” and omitted “REMOVE.” The saved Trim changed the frame range, and the source WAV frame count did not change. |
| Input-device loss | Not run | Requires a person to disconnect or disable the active device. |
| Input-device change | Not run | |
| Microphone permission refusal | Not run | Requires a person to change the operating-system permission. |
| Narration Application restart | Pass | The first restart found an invalid silent peak level. After correction, the Project opened, and the selected Take and Trim persisted. |
| ACX-compatible MP3 export | Partial pass | The visible flow made a 44.1 kHz, 192 kbps, mono MP3 from the saved Trim. ACX loudness and noise checks are not part of this result. |

## Windows

- Status: Not run. Deferred to release tests by the decision to stop.
- Operating system: Not recorded
- Input device: Not selected
- Output device: Not selected
- Source-audio setting: Not selected
- WAV setting: Mono, 48 kHz when the input device supports it, 24-bit integer PCM
- Encoder: Not recorded

| Test | Result | Failure behavior or observation |
| --- | --- | --- |
| Local EPUB import | Not run | |
| Manual Teleprompter movement | Not run | |
| Input and output device list | Not run | |
| Three-second microphone test | Not run | |
| 60-minute Take | Not run | |
| Pause and continue | Not run | |
| Playback | Not run | |
| Take selection | Not run | |
| Non-destructive Trim | Not run | |
| Input-device loss | Not run | Requires a person to disconnect or disable the active device. |
| Input-device change | Not run | |
| Microphone permission refusal | Not run | Requires a person to change the operating-system permission. |
| Narration Application restart | Not run | |
| ACX-compatible MP3 export | Not run | No ACX upload is part of this prototype. |
