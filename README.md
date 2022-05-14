# TechBier Rust & WebAssembly

## Rust

### Links

* <https://doc.rust-lang.org/>

### Installation

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

git clone https://github.com/cs0978/techbier-wasm.git
cd techbier-wasm/hello_world
```

### Build & Run

```bash
# build
cargo build

# run
cargo run techbier
```

### VSCode Plugins

* Rust
* CodeLLDB

## WebAssembly

### Links

* <https://wasmedge.org/book/en/index.html>
* <https://bytecodealliance.org/>
* <https://wasi.dev/>
* <https://wasmtime.dev/>

### Installation

```bash
# target
rustup target add wasm32-wasi
```

### VSCode Plugins

* WebAssembly

### Browser

#### Installation

```bash
# wasm-pack
cargo install wasm-pack
cd ../hello_browser

# alternative: wasm32 (does currently not work properly)
cd ../..
sudo dnf -y install libatomic
rustup target add wasm32-unknown-emscripten
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install 2.0.34
./emsdk activate 2.0.34
cd ../techbier-wasm/hello_browser
```

#### Build

```bash
# wasm-pack
wasm-pack build --target web

# alternative: wasm32 (does currently not work properly)
EMCC_CFLAGS="-s ERROR_ON_UNDEFINED_SYMBOLS=0 --no-entry -gsource-map -s STANDALONE_WASM" cargo build --release --target wasm32-unknown-emscripten
# additional flags because of bug in Rust: https://github.com/rust-lang/rust/issues/85821
# no js because of bug in Cargo: https://github.com/rust-lang/cargo/issues/7449
```

#### Run

```bash
# run webserver
python3 -m http.server
```

* Open browser with <http://localhost:8000>.
* Reload

### Wasmtime

#### Installation

```bash
curl https://wasmtime.dev/install.sh -sSf | bash
cd ../hello_world
```

#### Build & Run

```bash
cargo build --target wasm32-wasi
wasmtime target/wasm32-wasi/debug/hello_world.wasm techbier
```

### Wasmedge

#### Installation

```bash
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
cd ../hello_world
```

#### Build & Run

```bash
cargo build --target wasm32-wasi
wasmedge target/wasm32-wasi/debug/hello_world.wasm techbier
```

## Web Server

### Installation

```bash
cd ../..
git clone https://github.com/second-state/wasmedge_wasi_socket.git
cd techbier-wasm/http_server
```

#### Build & Run

```bash
cargo build --target wasm32-wasi
wasmedge target/wasm32-wasi/debug/http_server.wasm

# test
curl -X POST http://127.0.0.1:8080 -d "techbier"
```
