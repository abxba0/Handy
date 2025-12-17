# Project Context

## Purpose

Handy is a free, open source, cross-platform desktop speech-to-text application that works completely offline. It provides privacy-focused speech transcription where pressing a shortcut captures speech, processes it locally using Whisper/Parakeet models, and pastes the transcribed text into any application. The goal is to create the most forkable speech-to-text app that prioritizes simplicity, privacy, and extensibility.

## Tech Stack

- **Frontend**: React + TypeScript with Tailwind CSS, Vite
- **Backend**: Rust with Tauri 2.x framework
- **UI Framework**: Tauri for cross-platform desktop apps
- **State Management**: Zustand (frontend), Tauri state system (backend)
- **Internationalization**: i18next with ESLint enforcement
- **Audio Processing**: cpal, rubato, vad-rs, rodio
- **Speech Recognition**: whisper-rs, transcribe-rs (Parakeet models)
- **System Integration**: rdev (global shortcuts), enigo (input simulation)

## Project Conventions

### Code Style

**TypeScript/React:**

- Strict TypeScript: avoid `any` types, use explicit types
- Functional components with hooks, not class components
- Path aliases: `@/` → `./src/`, `~/*` → `./src/*`
- Tailwind CSS for styling (no inline styles)
- ESLint enforces i18n: all user-facing strings must use `t()` from `useTranslation()`
- Imports: React first, then external libs, then internal modules

**Rust:**

- Use `cargo fmt` (Rust 2021 edition) and `cargo clippy`
- Handle errors explicitly: prefer `Result` over `unwrap()` in production
- Use descriptive names, add doc comments (`///`) for public APIs
- Modules: group related functionality, use `mod.rs` for module definitions
- Error handling: use `anyhow::Result` for application errors

**General:**

- No hardcoded strings in JSX (use i18n translations)
- Keep functions small and single-purpose
- Add comments for non-obvious logic
- Follow existing patterns in the codebase

### Architecture Patterns

**Manager Pattern:** Core functionality organized into managers (Audio, Model, Transcription, History) initialized at startup and managed via Tauri state.

**Command-Event Architecture:** Frontend → Backend via Tauri commands; Backend → Frontend via events.

**Pipeline Processing:** Audio → VAD → Whisper/Parakeet → Text output → Clipboard/Paste.

**Single Instance:** App enforces single instance behavior - launching when already running brings settings window to front.

### Testing Strategy

**Rust Unit Tests:** Use `#[test]` attribute with `cargo test`. Tests are co-located with source code in `*.rs` files.

**Manual Testing:** Primary testing method due to audio/system integration complexity. Test with:

- Different audio devices and configurations
- Various transcription scenarios
- Platform-specific features (macOS accessibility, Windows shortcuts, Linux Wayland)
- Debug mode enabled (`Cmd/Ctrl+Shift+D`)

**Integration Testing:** Through Tauri commands and event system verification.

### Git Workflow

- **Branching**: Feature branches from `main` (e.g., `feature/`, `fix/`, `docs/`)
- **Commits**: Conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
- **PR Process**: Community feedback encouraged via GitHub Discussions before significant features
- **Release**: Semantic versioning with automated builds via GitHub Actions

## Domain Context

**Speech-to-Text Pipeline:**

1. Audio capture via system microphone
2. Voice Activity Detection (VAD) using Silero model
3. Audio resampling to 16kHz mono
4. Speech recognition via Whisper (GPU-accelerated) or Parakeet (CPU-optimized)
5. Text post-processing and correction
6. Output via clipboard or simulated keystrokes

**Platform-Specific Considerations:**

- **macOS**: Metal acceleration, accessibility permissions required, Apple Intelligence SDK integration
- **Windows**: Vulkan acceleration, code signing, different audio APIs
- **Linux**: OpenBLAS + Vulkan, Wayland limitations, overlay focus issues

**Model Management:**

- Models downloaded on-demand to `src-tauri/resources/models/`
- Supports Whisper (Small/Medium/Turbo/Large) and Parakeet V3
- Automatic language detection and translation options

## Important Constraints

**Privacy:** All processing must be local - no cloud services for audio or transcription.

**Offline-First:** Core functionality must work without internet connectivity.

**Cross-Platform:** Must support Windows, macOS, and Linux with consistent UX.

**Performance:** Real-time or near-real-time transcription with minimal latency.

**Accessibility:** Must work with screen readers and accessibility tools.

**Resource Usage:** Memory and CPU usage should be reasonable for desktop use.

## External Dependencies

**Required Models:**

- `silero_vad_v4.onnx`: Voice Activity Detection model (downloaded during setup)
- Whisper models: Various sizes downloaded on-demand
- Parakeet models: CPU-optimized models for broader compatibility

**System Dependencies:**

- **macOS**: Xcode command line tools, accessibility permissions
- **Windows**: Visual Studio Build Tools, Vulkan SDK
- **Linux**: Development libraries (ALSA/PulseAudio), Vulkan/OpenBLAS

**Build Tools:**

- Rust toolchain (latest stable)
- Bun package manager
- Platform-specific build tools (see BUILD.md)
