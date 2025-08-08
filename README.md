# Flutter Whisper.cpp

Flutter Whisper.cpp allows offline/on device - fast and accurate automatic speech recognition (ASR) using [OpenAI's Whisper](https://openai.com/research/whisper) ASR model. Built on top of [ggerganov's Whisper.cpp](https://github.com/ggerganov/whisper.cpp), the app uses [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) to bind Flutter to Rust via FFI, and [whisper-rs](https://github.com/tazz4843/whisper-rs) for Rust C bindings to Whisper.cpp. The app also utilizes the [Record](https://github.com/llfbandit/record) Dart library for recording .m4a in iOS which is then converted to a .wav file.

[Click here to see my talk at Fluttercon 2023 on using Rust with Flutter](https://www.droidcon.com/2023/08/07/supercharging-your-flutter-apps-with-rust/)

Tested with Flutter version 3.32.8 and Rust version 1.77.1

Runs on:
- [x] iOS (17.0.3 and XCode 15.0)
- [ ] Android 
- [x] Mac OS (Sonoma 14.0 and XCode 15.0)
- [ ] Linux 
- [ ] Windows

## Video
The example below took < 1 second to process the audio on an iPhone 12

https://user-images.githubusercontent.com/20296911/229925629-9f4e9fa0-6165-4d96-b61b-a04f8105a1f6.MOV

## Purpose and Benefits
The purpose of this app is to provide an example of performing automatic speech recognition using OpenAI's Whisper ASR model in Flutter. The benefits of this approach include:

* High-performance inference of Whisper ASR model.
* User-friendly interface for recording and transcribing speech.

To use the app, simply record speech by tapping the record button, and the app will automatically transcribe the speech using the Whisper ASR model. The results are displayed relatively quickly making it easy to see how accurate the transcription is.

## Installation

The app has only been set up with iOS to start with and has been tested on an iPhone 12 and an iPad Air with a 2022 M1 chip. To install the app, follow these steps:

*Note: due to using the ffmpeg library it's not currently working on the simulator, I will fix this but for the time being its recommended to use a real iOS device*

### Running without changing the model 
1. Clone the repository to your local machine.
2. Install the necessary dependencies and libraries, i.e `flutter pub get` in the main directory and `cargo build` in the `./rs_whisper_gpt` directory.
3. Ensure the model is added to `/rs_whisper_gpt/ggml-base.en.bin` this is where XCode is currently looking for it. [See Whisper.cpp for Installing/Downloading Models](https://github.com/ggerganov/whisper.cpp/tree/master/models#readme) - it is set up with `ggml-base.en.bin`. 
4. Run `flutter run -d {device}`

### Changing the model
1. Download a new model (i.e ggml-tiny.en.bin) [See Whisper.cpp for Installing/Downloading Models](https://github.com/ggerganov/whisper.cpp/tree/master/models#readme)
2. Add a reference to this file in XCode, make sure its in the Runner/Runner directory (important for the lookup in the Rust code, or change the path in the Rust code to reference this)
3. Update the Rust code in `./rs_whisper_gpt/src/api.rs` to reference the name of the model
4. Run the flutter_rust_bridge_codegen command as described below to generate new bindings.

### Changing the language

You can change the language of the model, you just need to set the language in the api call to the same code as the language you would like to use. You also need to make sure you use the correct model i.e ggml-tiny.bin instead of ggml-base.en.bin. See [OpenAI Whisper](https://github.com/openai/whisper#available-models-and-languages) for more details.

## Editing the Rust Bindings via Flutter Rust Bridge 

You will need to run the command 
```
flutter_rust_bridge_codegen --rust-input rs_whisper_gpt/src/api.rs --dart-output lib/bridge_generated.dart -c ios/Runner/bridge_generated.h -e macos/Runner/
```
Check out the [flutter_rust_bridge User Guide](https://cjycode.com/flutter_rust_bridge/) for more information

## Optimisations on iOS 

As per other examples in Whisper.cpp, the app has been optimized for performance by adding -O3 -DNDEBUG to Other C Flags. However, it is not recommended for production or real-world scenarios.
