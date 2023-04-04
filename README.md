# Flutter Whisper.cpp

Flutter Whisper.cpp is a high-performance Flutter app that allows for fast and accurate automatic speech recognition (ASR) using OpenAI's Whisper ASR model. Built on top of [ggerganov's Whisper.cpp](https://github.com/ggerganov/whisper.cpp), the app uses [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) to bind Flutter to Rust via FFI, and [whisper-rs](https://github.com/tazz4843/whisper-rs) for Rust C bindings to Whisper.cpp. The app also utilizes the [Record](https://github.com/llfbandit/record) Dart library for recording .m4a in iOS which is then converted to a .wav file.

## Purpose and Benefits
The purpose of this app is to provide an example to perform automatic speech recognition using OpenAI's Whisper ASR model. The benefits of this approach include:

* High-performance inference of Whisper ASR model.
* User-friendly interface for recording and transcribing speech.

To use the app, simply record speech by tapping the record button, and the app will automatically transcribe the speech using the Whisper ASR model. The results are displayed in real-time, making it easy to see how accurate the transcription is.

<img src="https://user-images.githubusercontent.com/20296911/229306858-56e52825-b16d-4b08-b810-75360bb65a2d.jpeg" width=150 height=216>

## Installation

The app is currently available only on iOS and has been tested on an iPhone 12 and an iPad Air with a 2022 M1 chip. To install the app, follow these steps:

1. Clone the repository to your local machine.
2. Install the necessary dependencies and libraries as detailed in the README.
3. Add the relevant model filename to the /rs_whisper_gpt/api.rs folder under the line of code let base_model = get_resources_dir().join("ggml-base.en.bin");.
4. Ensure the model is added to Xcode under the root of the Runner/Runner. [See Whisper.cpp for Installing/Downloading Models](https://github.com/ggerganov/whisper.cpp/tree/master/models#readme) - it is set up with `ggml-base.en.bin`
5. Run the flutter_rust_bridge_codegen command as described below.

Note: I've only tested this on iOS with a Iphone 12 and an iPad Air (2022 M1 chip), its not been set up for Android or any other Flutter platforms

## Editing the Rust Bindings via Flutter Rust Bridge 

You will need to run the command 
```
flutter_rust_bridge_codegen --rust-input rs_whisper_gpt/src/api.rs --dart-output lib/bridge_generated.dart -c ios/Runner/bridge_generated.h
```
Check out the [flutter_rust_bridge User Guide](https://cjycode.com/flutter_rust_bridge/) for more information

## Optimisations on iOS 

As per other examples in Whisper.cpp, the app has been optimized for performance by adding -O3 -DNDEBUG to Other C Flags. However, it is not recommended for production or real-world scenarios.
