# TechBier Rust & WebAssembly

## Rust

This tutorial is made for Linux and MacOs and WLS (Windows Linux Subsystem).

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

* <https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm>
* <https://wasmedge.org/book/en/index.html>
* <https://bytecodealliance.org/>
* <https://wasi.dev/>
* <https://wasmtime.dev/>

### VSCode Plugins

* WebAssembly

### Web

#### Installation

```bash
cargo install wasm-pack
cd ../hello_browser
```

#### Build & Run

```bash
wasm-pack build --target web

# run webserver
python3 -m http.server
```

* Open browser with <http://localhost:8000>.
* Reload

### Wasi

#### Installation

```bash
# target
rustup target add wasm32-wasi
```

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

### Web Server with Wasmedge

#### Installation

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
