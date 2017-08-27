# Build Instructions

build instructions for the rust static library

# Setup

 - `brew install openssl`
 - `rustup install nightly`
 - `rustup target install --toolchain nightly x86_64-apple-ios`

# Build

 - `export OPENSSL_INCLUDE_DIR=/usr/local/Cellar/openssl/1.0.2j/include/`
 - `rustup run nightly cargo build --target x86_64-apple-ios`