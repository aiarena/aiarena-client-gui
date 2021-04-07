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

In order for the executable to work, copy all the dlls from `\src-tauri\target\release\build\openssl-sys-79e6122484900016\out`
to `/src-tauri/target/release/` or the installation directory after installing.

For more information around compiling, please see the [readme](./backend/README.md) for the backend.