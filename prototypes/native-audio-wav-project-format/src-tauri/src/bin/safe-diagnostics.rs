fn main() {
    match native_audio_wav_prototype_lib::run_safe_diagnostics() {
        Ok(report) => println!("{report}"),
        Err(error) => {
            eprintln!("Safe diagnostics failed: {error}");
            std::process::exit(1);
        }
    }
}
