# AiArena-Client GUI

## Installation
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install `NodeJS` and `yarn`
- Run <br>
```bash
> yarn
> yarn tauri build
```

Files will be in `/src-tauri/target/release/`:
- MSI file: `/src-tauri/target/release/bundle/msi/`
- Stand-alone executable: `/src-tauri/target/release/aiarena-client-gui.exe`

### Dynamic linking
Follow the instructions for openssl installation in this [readme](./backend/README.md).
If you are using Dynamic Linking copy all the dlls from `\src-tauri\target\release\build\openssl-sys-*\out`
to `/src-tauri/target/release/` or the installation directory after installing.

For more information around compiling, please see the [readme](./backend/README.md) for the backend.

The backend runs by default on 127.0.0.1:8082