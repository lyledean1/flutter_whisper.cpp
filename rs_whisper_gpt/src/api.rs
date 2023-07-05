use flutter_rust_bridge::handler::Error;
use hound::{SampleFormat, WavReader};
use objc::rc::autoreleasepool;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use std::path::{Path, PathBuf};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

// Get current dir of the app
fn get_resources_dir() -> PathBuf {
    let bundle: *mut Object = unsafe { msg_send![class!(NSBundle), mainBundle] };
    let resources_dir: *mut Object = unsafe { msg_send![bundle, resourcePath] };

    let cstr: *const std::os::raw::c_char = unsafe { msg_send![resources_dir, UTF8String] };
    let path = unsafe {
        std::ffi::CStr::from_ptr(cstr)
            .to_string_lossy()
            .into_owned()
    };
    PathBuf::from(path)
}

fn parse_wav_file(path: &Path) -> Vec<i16> {
    let reader = WavReader::open(path).expect("failed to read file");

    if reader.spec().channels != 1 {
        panic!("expected mono audio file");
    }
    if reader.spec().sample_format != SampleFormat::Int {
        panic!("expected integer sample format");
    }
    if reader.spec().sample_rate != 16000 {
        panic!("expected 16KHz sample rate");
    }
    if reader.spec().bits_per_sample != 16 {
        panic!("expected 16 bits per sample");
    }

    reader
        .into_samples::<i16>()
        .map(|x| x.expect("sample"))
        .collect::<Vec<_>>()
}

fn run_whisper_audio_to_text(ctx: WhisperContext, samples: Vec<f32>) -> Vec<String> {
    let mut strings: Vec<String> = vec![];
    let mut state: whisper_rs::WhisperState<'_> =
        ctx.create_state().expect("failed to create state");

    let params = FullParams::new(SamplingStrategy::default());

    state
        .full(params, &samples)
        .expect("failed to convert samples");
    let num_segments = state
        .full_n_segments()
        .expect("failed to get number of segments");
    for i in 0..num_segments {
        let segment = state
            .full_get_segment_text(i)
            .expect("failed to get segment");
        strings.push(segment);
    }
    strings
}

pub fn run_whisper_model(path: String) -> Vec<String> {
    let result = autoreleasepool(|| {
        //Get Audio Path inside iOS
        let audio_path = Path::new(&path);
        if !audio_path.exists() && !audio_path.is_file() {
            panic!("expected a file at {:?}", audio_path);
        }
        // Load Base Model Weights
        let base_model = get_resources_dir().join("ggml-base.en.bin");
        let whisper_path = Path::new(&base_model);
        if !whisper_path.exists() && !whisper_path.is_file() {
            panic!("expected a whisper directory")
        }        
        // Parse Wave File 
        let original_samples = parse_wav_file(audio_path);
        let samples = whisper_rs::convert_integer_to_float_audio(&original_samples);
        let ctx =
            WhisperContext::new(&whisper_path.to_string_lossy()).expect("failed to open model");

        // Run Whisper Model on Samples and Return Vec<String> of Text 
        run_whisper_audio_to_text(ctx, samples)
    });
    result
}
