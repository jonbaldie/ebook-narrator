# Research: Local Tauri Technical Design

**Research date:** 2026-07-31  
**Question:** What local technical design can provide EPUB import, manual Teleprompter control, microphone selection and test, recording and playback, local Project storage, Take and Trim data, and ACX-compatible MP3 export on macOS and Windows?

## Result

Use Tauri 2 with a webview-only Teleprompter and a Rust core that owns EPUB import, Project files, audio devices, recording, playback, Trim, and export. Tauri renders with WKWebView on macOS and WebView2 on Windows. This makes browser media output different by platform. [Tauri process model](https://v2.tauri.app/concept/process-model/)

Use CPAL for native audio input and output. CPAL supports CoreAudio on macOS and WASAPI on Windows. It can list input and output devices, list supported stream settings, and create input and output streams. [CPAL repository](https://github.com/RustAudio/cpal) [CPAL API](https://docs.rs/cpal/latest/cpal/)

Save each Take as a local PCM WAV file. Save Project records in a versioned JSON file in the same Project directory. Keep a Trim as frame offsets in the Project record. Do not change the WAV source file for a Trim.

Use an encode-only LAME integration for MP3 export. It needs licence work before release. Do not use browser `MediaRecorder` as the saved-source format or as the ACX export path.

This review checks published primary sources. It does not run an implementation test. The release tests in this note are required before the design is fixed.

## Choices checked

| Choice | Result | Reason |
| --- | --- | --- |
| Tauri 2 plus a Rust core | Use | Tauri has a Rust binary and a system webview frontend. It provides native dialogs and a capability system for webview access. [Tauri README](https://github.com/tauri-apps/tauri/blob/dev/README.md) [Tauri dialog plugin](https://v2.tauri.app/plugin/dialog/) [Tauri capabilities](https://v2.tauri.app/security/capabilities/) |
| `MediaDevices` and `MediaRecorder` in the webview | Do not use for saved source audio or export | The standard can list devices, but device data can be limited before access. A positive `isTypeSupported` result does not ensure that recording will work. A recorder can also select the platform default encoding. [W3C Media Capture and Streams](https://www.w3.org/TR/mediacapture-streams/) [W3C MediaStream Recording](https://www.w3.org/TR/mediastream-recording/) |
| CPAL for recording and playback | Use | CPAL is a low-level Rust library for audio input and output. The selected device can report supported input or output settings. The calls can fail when a device disconnects. [CPAL API](https://docs.rs/cpal/latest/cpal/) |
| PCM WAV for each Take | Use | Hound reads and writes integer PCM WAV with 8, 16, 24, and 32 bits per sample. It is sufficient for a local source format without lossy re-encoding. [Hound repository](https://github.com/ruuda/hound) |
| Tauri store for Project data | Do not use | The store plugin is a persistent key-value file. Use it only for small Narration Application settings. Use the Project directory and its versioned record as the portable Project format. [Tauri store plugin](https://v2.tauri.app/plugin/store/) |
| LAME encode-only library for MP3 export | Use after licence review and prototype | LAME is an LGPL MP3 encoder. Its own licence page states that its decoder has GPL limits. The Narration Application needs encoding only because it reads local WAV sources. [LAME licence](https://lame.sourceforge.io/license.txt) [LAME project](https://lame.sourceforge.io/) |
| FFmpeg in the first release | Do not use | FFmpeg can be LGPL or GPL, based on build options. Its own legal guide has source, notice, linking, and build-record requirements. Do not add this release and licence risk when a WAV-to-MP3 encoder is sufficient. [FFmpeg legal guide](https://ffmpeg.org/legal.html) |

## Recommended design

```text
Teleprompter webview
  -> small, named Tauri commands
Rust core
  -> EPUB import and text extraction
  -> Project record read and write
  -> CPAL device list, test, record, and play
  -> WAV files and Trim frame ranges
  -> LAME encode-only ACX-compatible MP3 export
Project directory
  -> original EPUB, Project record, Take WAV files, and exported MP3 files
```

The webview displays text and sends manual control actions. It must not read the Project directory, capture a microphone, or write audio files. The Rust core must check every command parameter and every Project-relative path. Tauri permissions enable or deny frontend commands. Custom command scopes also need checks in the command implementation. [Tauri permissions](https://v2.tauri.app/security/permissions/) [Tauri command scopes](https://v2.tauri.app/security/scope/)

Use the Tauri dialog plugin for EPUB input and export-location selection. It provides native open and save dialogs, and returns file paths on macOS and Windows. Grant only `dialog:allow-open` and `dialog:allow-save` when those actions are needed. [Tauri dialog plugin](https://v2.tauri.app/plugin/dialog/)

Do not add an HTTP plugin, a localhost server, remote content, or remote webview capabilities in the first release. A local EPUB can contain remote resources. The importer must not fetch them. EPUB 3 permits some remote resources, including audio, video, fonts, and scripts. [EPUB 3.3](https://www.w3.org/TR/epub-33/)

## Project and audio formats

Create one directory for each Project. Use a short ASCII-safe identifier for generated file names. Do not use chapter titles as file names.

```text
<Project name>.ebook-narrator/
  project.json
  source/book.epub
  text/segments.json
  takes/<take-id>.wav
  exports/<export-id>/<ordered-file-name>.mp3
```

`project.json` must contain a `schemaVersion`, Project identifier, original EPUB file name and hash, EPUB reading-order records, Narration Segments, selected Take identifiers, Trim frame ranges, Teleprompter Display Settings, and export records. `segments.json` must contain the extracted text and the source location for each Narration Segment. Keep the imported EPUB unchanged.

Use PCM WAV for every Take. Prefer mono, 48 kHz, 24-bit integer PCM when the selected microphone supports it. Otherwise, select a supported input setting and save its channel count, sample rate, sample format, and frame count in the Take record. CPAL requires the Narration Application to select a device setting that the device supports. Hound supports WAV writer settings for channel count, sample rate, bit count, and integer or float sample format. [CPAL API](https://docs.rs/cpal/latest/cpal/) [Hound `WavSpec`](https://docs.rs/hound/latest/hound/struct.WavSpec.html)

Write a Take first to a temporary WAV file. Validate its header and frame count. Then rename it to its final path and update `project.json`. On Project open, ignore unreferenced temporary files and report them for recovery. This design prevents an incomplete recording from becoming the selected Take.

## EPUB import and the Teleprompter

An EPUB is an OCF ZIP container. It has a package document with a manifest and a spine. The spine gives the default reading order. [EPUB 3.3](https://www.w3.org/TR/epub-33/)

The importer must:

- Copy the selected EPUB to `source/book.epub`.
- Read `META-INF/container.xml`, then the package document, manifest, and spine.
- Accept reflowable XHTML spine content in the first release.
- Extract text in spine order into immutable Narration Segments. Save each source document path and source element identifier when available.
- Reject an EPUB when a spine item needs encrypted content, has no usable text, or uses an unsupported content type. Ignore font obfuscation because the Teleprompter uses Narration Application fonts. EPUB defines an `encryption.xml` file and does not permit ZIP encryption. [EPUB 3.3](https://www.w3.org/TR/epub-33/)
- Set limits for ZIP entry count, total extracted size, individual extracted size, and nesting depth before extraction. Reject a file that exceeds a limit.
- Remove scripts and do not fetch remote resources before the Teleprompter displays extracted text.

The first release does not support fixed-layout, image-only, or DRM-controlled books. EPUB has a fixed-layout type, and EPUB resources can include non-text media. This scope limit avoids a false text reading order. [EPUB 3.3](https://www.w3.org/TR/epub-33/)

The Teleprompter must show one Narration Segment at a time. It must have Previous and Next buttons and the same keyboard actions when the Teleprompter has focus. It saves its current Narration Segment and text offset in `project.json`. It never moves from audio time. Recording state and Teleprompter position are separate. This gives the Narrator manual control.

## Microphone selection, test, recording, and playback

On start, the Rust core lists input and output devices. It marks the current system default. Before a Narrator records, it reads the selected input device's supported settings and opens a short test stream. CPAL provides input-device enumeration, configuration queries, input callbacks, output callbacks, and error results for disconnected devices. [CPAL `HostTrait`](https://docs.rs/cpal/latest/cpal/traits/trait.HostTrait.html) [CPAL `DeviceTrait`](https://docs.rs/cpal/latest/cpal/traits/trait.DeviceTrait.html)

The microphone test must record three seconds to a temporary WAV, show duration and peak level, and play that WAV on the selected output device. It must state a clear error when there is no input device, no permitted microphone, no supported setting, no input samples, or no output device.

When the Narrator starts recording, create a new Take. The audio callback must send frames to a bounded writer queue. The writer owns the WAV file. It must report an overrun and stop safely if the queue is full. Do not write to disk or change the Project record in the audio callback. CPAL states that a stream callback runs on a high-priority thread on modern platforms. [CPAL API](https://docs.rs/cpal/latest/cpal/)

Playback reads the source WAV and applies the selected Take's Trim range. A Trim contains `startFrame` and `endFrame`, where `0 <= startFrame < endFrame <= frameCount`. A new Take never replaces an old Take. Delete is a separate future action.

## macOS and Windows permissions

macOS needs `NSMicrophoneUsageDescription` in the application `Info.plist`. macOS terminates an application that asks to capture microphone input without the usage description. Tauri can merge `src-tauri/Info.plist` into its generated file. [Apple microphone usage description](https://developer.apple.com/documentation/BundleResources/Information-Property-List/NSMicrophoneUsageDescription) [Tauri macOS bundle](https://v2.tauri.app/distribute/macos-application-bundle/)

If the macOS release uses App Sandbox, add the `com.apple.security.device.microphone` entitlement. The entitlement does not replace the Narrator permission. [Apple microphone entitlement](https://developer.apple.com/documentation/bundleresources/entitlements/com.apple.security.device.microphone) [Apple media-capture permission](https://developer.apple.com/documentation/bundleresources/requesting-authorization-for-media-capture-on-macos)

The normal Tauri Windows outputs are MSI and NSIS. If a later Windows release uses MSIX or another app package, declare `<DeviceCapability Name="microphone"/>` in the package manifest. Windows policy can also deny microphone access. The Narration Application must report this state and give the Narrator the system-settings route. [Tauri Windows installer](https://v2.tauri.app/distribute/windows-installer/) [Microsoft app capabilities](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/app-capability-declarations) [Microsoft microphone privacy policy](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy)

Request microphone permission only when the Narrator starts the microphone test or recording. Test first permission, refusal, later approval, disabled microphone, selected-device removal, and changed default device on both operating systems.

## Licence limits

Tauri code is under MIT or Apache-2.0, as applicable. CPAL and Hound use Apache-2.0. Review the resolved dependency list before release, including transitive dependencies. [Tauri licences](https://github.com/tauri-apps/tauri/blob/dev/README.md) [CPAL licence](https://github.com/RustAudio/cpal) [Hound licence](https://github.com/ruuda/hound)

LAME is LGPL. Its project says that a commercial program can use it under LGPL restrictions, and says to acknowledge LAME and link it as a separate library. Its decoder includes GPL code and must not be used by a non-GPL program. Use the LAME encoding API only. Give the licence, notices, source offer or source location, and relinking information that the final LAME distribution requires. Get a licence review before a release. [LAME licence](https://lame.sourceforge.io/license.txt)

## ACX-compatible MP3 export

The exporter must read the selected WAV and Trim range, make the required level and noise checks, resample to 44.1 kHz, then make a CBR MP3 at 192 kbps or higher. It must make all files in one Project use one channel mode. ACX requires MP3, 44.1 kHz, at least 192 kbps CBR, peak no higher than -3 dB, mean loudness from -23 dB RMS to -18 dB RMS, and noise floor lower than -60 dB RMS. [ACX audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)

Use a separate output file for each chapter or section. Keep a file at or below 120 minutes. Put the spoken chapter or section header in each file. ACX also requires room tone, consistent sound, and no outtakes or distracting noise. [ACX audio submission requirements](https://help.acx.com/s/article/what-are-the-acx-audio-submission-requirements)

The exporter must validate the actual MP3 sample rate, CBR bit rate, channel mode, duration, peak, RMS, and noise floor after encoding. It must retain the source WAV and Project record when an export fails. Do not claim ACX acceptance until an exported file passes a real ACX upload or Audio Lab review.

## Main risks and required release tests

| Area | Risk | Required test |
| --- | --- | --- |
| EPUB import | EPUB files can have fixed layout, encryption records, remote resources, malformed XML, or unsafe ZIP sizes. | Import valid reflowable EPUB files and reject each unsupported case with a useful error. |
| Teleprompter | WKWebView and WebView2 can differ in keyboard and text layout. | Test every manual action with mouse and keyboard on macOS and Windows release builds. |
| Microphone | Permission, disabled devices, disconnects, and unsupported settings can stop capture. | Test first use, refusal, re-approval, no device, device removal, and each supported microphone class. |
| Recording | A slow disk or writer can lose audio. | Test a 60-minute Take, simulate a full writer queue, and verify that the error stops safely. |
| Playback | An invalid Trim can produce empty or out-of-range audio. | Test Trim bounds, restart persistence, and playback after moving the Project directory. |
| Local storage | A stop or crash can leave incomplete files or records. | Stop the Narration Application during recording and during a Project-record update. Check that the last valid Take remains available. |
| MP3 export | Resampling, level checks, and encoding can make an invalid ACX file. | Export a fixture Project and check every ACX numeric rule before and after MP3 creation. |
| Licence | A transitive dependency or encoder bundle can add distribution duties. | Create a software bill of materials and complete a licence review before signing installers. |

## Decision

Plan the first release as a local Tauri 2 Narration Application with a Rust-owned Project directory, native CPAL capture and playback, PCM WAV Takes, non-destructive frame-range Trims, a manual webview Teleprompter, and a reviewed encode-only LAME MP3 exporter. Do not use webview recording as the Project source format or ACX exporter.
