use hound::{SampleFormat, WavReader};
use objc::rc::autoreleasepool;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use std::path::{Path, PathBuf};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

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

fn run_whisper_audio_to_text(ctx: WhisperContext, samples: Vec<f32>, lang: Option<String>) -> Vec<String> {
    let mut strings: Vec<String> = vec![];
    let mut state: whisper_rs::WhisperState<'_> =
        ctx.create_state().expect("failed to create state");

    let mut params = FullParams::new(SamplingStrategy::default());

    // here we set the number of threads to use to 1
    params.set_n_threads(1);
    // we also enable translation
    params.set_translate(true);
    // and set the language to translate
    // default to english, try to unwrap if provided
    let lang_code = &lang.unwrap_or("en".to_string());
    params.set_language(Some(&lang_code));
    // we also explicitly disable anything that prints to stdout
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    let st = std::time::Instant::now();
    // Run whisper model inference
    state
        .full(params, &samples)
        .expect("failed to convert samples");
    let et = std::time::Instant::now();

    let num_segments = state
        .full_n_segments()
        .expect("failed to get number of segments");
    for i in 0..num_segments {
        let segment = state
            .full_get_segment_text(i)
            .expect("failed to get segment");
        strings.push(segment);
    }
    println!("took {}ms", (et - st).as_millis());
    strings
}

pub fn run_whisper_model(path: String, lang: Option<String>) -> Vec<String> {
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
        run_whisper_audio_to_text(ctx, samples, lang)
    });
    result
}
