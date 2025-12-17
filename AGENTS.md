<!-- OPENSPEC:START -->

# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/project.md` when the request:

- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/project.md` to learn:

- Project purpose and goals
- Tech stack and architecture patterns
- Code style and testing requirements
- Domain context and constraints

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

**Prerequisites:** [Rust](https://rustup.rs/) (latest stable), [Bun](https://bun.sh/)

**Core Development:**

```bash
# Install dependencies
bun install

# Run in development mode
bun run tauri dev
# If cmake error on macOS:
CMAKE_POLICY_VERSION_MINIMUM=3.5 bun run tauri dev

# Build for production
bun run tauri build

# Frontend only development
bun run dev        # Start Vite dev server
bun run build      # Build frontend (TypeScript + Vite)
bun run preview    # Preview built frontend
```

**Linting & Formatting:**

```bash
bun run lint              # ESLint for frontend (enforces i18n)
bun run lint:fix          # ESLint with auto-fix
bun run format            # Prettier + cargo fmt
bun run format:check      # Check formatting without changes
cd src-tauri && cargo fmt # Rust formatting
cd src-tauri && cargo clippy # Rust linting
```

**Testing:**

```bash
# Rust unit tests
cd src-tauri && cargo test
cd src-tauri && cargo test --test_name # Run specific test
```

**Model Setup (Required for Development):**

```bash
mkdir -p src-tauri/resources/models
curl -o src-tauri/resources/models/silero_vad_v4.onnx https://blob.handy.computer/silero_vad_v4.onnx
```

## Architecture Overview

Handy is a cross-platform desktop speech-to-text application built with Tauri (Rust backend + React/TypeScript frontend).

### Core Components

**Backend (Rust - src-tauri/src/):**

- `lib.rs` - Main application entry point with Tauri setup, tray menu, and managers
- `managers/` - Core business logic managers:
  - `audio.rs` - Audio recording and device management
  - `model.rs` - Whisper model downloading and management
  - `transcription.rs` - Speech-to-text processing pipeline
- `audio_toolkit/` - Low-level audio processing:
  - `audio/` - Device enumeration, recording, resampling
  - `vad/` - Voice Activity Detection using Silero VAD
- `commands/` - Tauri command handlers for frontend communication
- `shortcut.rs` - Global keyboard shortcut handling
- `settings.rs` - Application settings management

**Frontend (React/TypeScript - src/):**

- `App.tsx` - Main application component with onboarding flow
- `components/settings/` - Settings UI components
- `components/model-selector/` - Model management interface
- `hooks/` - React hooks for settings and model management
- `lib/types.ts` - Shared TypeScript type definitions

### Key Architecture Patterns

**Manager Pattern:** Core functionality is organized into managers (Audio, Model, Transcription) that are initialized at startup and managed by Tauri's state system.

**Command-Event Architecture:** Frontend communicates with backend via Tauri commands, backend sends updates via events.

**Pipeline Processing:** Audio → VAD → Whisper → Text output with configurable components at each stage.

### Technology Stack

**Core Libraries:**

- `whisper-rs` - Local Whisper inference with GPU acceleration
- `cpal` - Cross-platform audio I/O
- `vad-rs` - Voice Activity Detection
- `rdev` - Global keyboard shortcuts
- `rubato` - Audio resampling
- `rodio` - Audio playback for feedback sounds

**Platform-Specific Features:**

- macOS: Metal acceleration for Whisper, accessibility permissions
- Windows: Vulkan acceleration, code signing
- Linux: OpenBLAS + Vulkan acceleration

### Application Flow

1. **Initialization:** App starts minimized to tray, loads settings, initializes managers
2. **Model Setup:** First-run downloads preferred Whisper model (Small/Medium/Turbo/Large)
3. **Recording:** Global shortcut triggers audio recording with VAD filtering
4. **Processing:** Audio sent to Whisper model for transcription
5. **Output:** Text pasted to active application via system clipboard

### Settings System

Settings are stored using Tauri's store plugin with reactive updates:

- Keyboard shortcuts (configurable, supports push-to-talk)
- Audio devices (microphone/output selection)
- Model preferences (Small/Medium/Turbo/Large Whisper variants)
- Audio feedback and translation options

### Single Instance Architecture

The app enforces single instance behavior - launching when already running brings the settings window to front rather than creating a new process.

## Code Style Guidelines

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
- Use conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
