# AiArena-Client GUI

## Installation
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install `NodeJS` and `yarn`
- Install tauri-bundler:
  ```bash
  cargo install tauri-bundler
  ```
- Run <br>
```bash
> yarn
> yarn tauri build
```

Files will be in `/src-tauri/target/release/`:
- MSI file: `/src-tauri/target/release/bundle/msi/`
- Stand-alone executable: `/src-tauri/target/release/aiarena-client-gui.exe`

The backend runs by default on 127.0.0.1:8082<br>

Backend [readme](backend/README.md)


### TODO

#### Required for beta
- [x] Backend API
- [x] Basic functionality (Ability to run games)
- [x] Existing env detection
- [x] Replay saving and opening
- [x] Multiple OS environment testing
  - [x] Windows
  - [x] MacOS
  - [x] Linux (Except for FileDialog)
- [x] Frontend
- [ ] Dynamic logging to files for debugging purposes
- [ ] Rust tests
- [ ] Frontend tests
- [x] CI/CD
- [ ] Docker File with all bot dependencies
- [x] Headless mode

#### Required for v1
- [x] Download bots from AiArena and run games
- [ ] Python virtualenv outside of Docker Container (ensures correct libraries and Python version)
- [ ] One-Click bot dependency installer
- [ ] Archon mode
- [ ] Human vs Bot mode
- [ ] Real-time game rendering (for Docker games)
- [x] Faster builds

#### After v1
- [ ] Allow some website functionality through the app(TBD)
- [ ] Auto-Updater (opt-in)
- [ ] Play bot over internet (TBD)
- [ ] Ai-Arena authentication
- [ ] Real-time updates via websockets (Text-based)

