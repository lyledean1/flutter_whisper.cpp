# Flutter Whisper.cpp

Flutter App that has high-performance inference of [OpenAI's Whisper](https://github.com/openai/whisper) automatic speech recognition (ASR) model from [ggerganov Whisper.cpp](https://github.com/ggerganov/whisper.cpp)

Note: I've only tested this on iOS with a Iphone 12 and an iPad Air (2022 M1 chip), its not been set up for Android or any other Flutter platforms

Uses:
- Whisper.cpp for OpenAI automatic speech recognition [ggerganov Whisper.cpp](https://github.com/ggerganov/whisper.cpp)
- Flutter Rust Bridge for bindings from Flutter to Rust via FFI [fzyzcjy flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge)
- whisper-rs for Rust C bindings to Whisper.cpp [tazz4843 whisper-rs](https://github.com/tazz4843/whisper-rs)
- Record Dart library for recording .m4a in iOS, some of the Dart code was copied over from the really useful example for the main screen! [llfbandit record](https://github.com/llfbandit/record)

## Screenshot 


<img src="https://user-images.githubusercontent.com/20296911/229306858-56e52825-b16d-4b08-b810-75360bb65a2d.jpeg" width=300 height=432>

## Updating whisper.cpp files 

Update [whisper-rs in the Cargo.toml file](https://github.com/lyledean1/flutter_whisper.cpp/blob/ee49c35935ad549e0eebfef226b98a06c65ca5b6/rs_whisper_gpt/Cargo.toml#LL20C14-L20C19) and run the flutter_rust_bridge commands as detailed below. 

## Installing base models

[See Whisper.cpp for Installing/Downloading Models](https://github.com/ggerganov/whisper.cpp/tree/master/models#readme)

You will need to set the relevant model filename in the `/rs_whisper_gpt/api.rs` folder under the line of code
```
let base_model = get_resources_dir().join("ggml-base.en.bin");
```

And ensure the model is added to Xcode under the root of the Runner/Runner. Finally if the name is changed then run the flutter_rust_bridge_codegen as described below.

## Editing the Rust Bindings via Flutter Rust Bridge 

You will need to run the command 
```
flutter_rust_bridge_codegen --rust-input rs_whisper_gpt/src/api.rs --dart-output lib/bridge_generated.dart -c ios/Runner/bridge_generated.h
```

Check out the [flutter_rust_bridge User Guide](https://cjycode.com/flutter_rust_bridge/) for more information

## Optimisations on iOS 

As per other examples in Whisper.cpp 
- I've added `-DGGML_USE_ACCELERATE` compiler flag in Build Phasese
- I've added `-O3 -DNDEBUG` to `Other C Flags`. But is not recommended for production/real world scenarios 
