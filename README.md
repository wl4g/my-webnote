# A personal web notebook based on Excalidraw/Tldraw written in React and Rust.

## Quickstart

```bash
# TODO
```

## Building

```bash
git clone https://github.com/wl4g/my-webnote.git

cd my-webnote/server
RUSTFLAGS="--cfg tokio_unstable" && cargo build --release

cd ../ui
yarn electron-vite build
```

