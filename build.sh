rustup target add armv7-linux-androideabi
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
rustup target add i686-linux-android
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios

cd rust
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86_64 -t x86 -o ../android/app/src/main/jniLibs build --release
