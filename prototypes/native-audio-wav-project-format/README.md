# PROTOTYPE — Native Audio and WAV Project Format

This throwaway Tauri test harness answers one question:

> Does the proposed CPAL capture, PCM WAV Take, frame-range Trim, local Project record, and LAME export design work on macOS and Windows?

It is not production code. It puts each required test action on one screen and shows the complete test state after each action.

## Run

Start the prototype:

```sh
cargo run --manifest-path prototypes/native-audio-wav-project-format/src-tauri/Cargo.toml --bin native-audio-wav-prototype
```

The first microphone action can show an operating-system permission request.

Run the safe checks without microphone capture or playback:

```sh
cargo run --manifest-path prototypes/native-audio-wav-project-format/src-tauri/Cargo.toml --bin safe-diagnostics
```

## Test order

1. Select `Create fixture EPUB`, then select `Import fixture EPUB`.
2. Use `Previous` and `Next` to move the Teleprompter.
3. Select an input and output device.
4. Select `Run 3-second test`, listen to the result, and record the result.
5. Start a Take. Pause it, continue it, and stop it.
6. Select a Take, set a start and end frame, and apply a Trim.
7. Play the selected Take. Confirm that playback uses the Trim.
8. Start a Take and disconnect the selected input device. Record the result.
9. Select a different input device and make a new Take. Record the result.
10. Deny microphone access in the operating-system settings. Start a test. Record the result. Restore access.
11. Start a Take and leave it active for 60 minutes. Stop it. Record the result.
12. Close and restart the prototype. Open the same Project. Confirm the Take selection and Trim.
13. Select `Export selected Take`. Check the MP3 values that the prototype reports.

Run this order on macOS and Windows. Use [TEST-RESULTS.md](TEST-RESULTS.md) to record facts. A manual device-loss test and a microphone-permission test need a person at the computer.

## Limits

- The fixture importer checks reflowable spine text. It is not a complete or secure EPUB importer.
- The exporter calls a local LAME 3.100 or later executable. It does not bundle LAME.
- The MP3 check reads the LAME output. It does not make an ACX upload.
- The prototype saves all test data below the operating-system temporary directory. It prints the exact Project path.
