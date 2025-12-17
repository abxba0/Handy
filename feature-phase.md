
# Meta-Prompt: Handy Application Feature Expansion
**Target Repository:** `cjpais/Handy`
**Framework:** OpenSpec / Tauri v2
**Language Stack:** Rust (Backend), TypeScript/React (Frontend)

## System Role & Objective
You are an expert Senior Full-Stack Engineer specializing in Tauri, Rust, and React. Your objective is to implement four distinct major features into the `Handy` speech-to-text application.

You must execute this strictly in **Phases**. Do not proceed to the next phase until the current phase is implemented, compiled, and tested.

## 🛠️ Global Tech Stack Context
*   **Core:** Tauri v2 (Rust Backend + React Frontend)
*   **Audio Pipeline:** `cpal` (Input), `vad-rs` / `Silero` (Voice Activity Detection)
*   **Transcription:** `whisper-rs`, `transcription-rs` (Parakeet)
*   **State Management:** React Context / Hooks
*   **Styling:** Tailwind CSS

---

## 📅 Phase 1: Cloud Inference (OpenAI Whisper)
**Goal:** Integrate OpenAI's Whisper API as a selectable transcription engine alongside existing local models.

### 1.1 Backend Implementation (Rust)
*   **File:** `src-tauri/src/settings.rs`
    *   Add `openai_api_key` (String, secure storage recommended) and `openai_model` (String, default "whisper-1") to the `Settings` struct.
*   **File:** `src-tauri/src/cloud_transcription/openai.rs` (Create New)
    *   Implement an API client using `reqwest`.
    *   Handle audio file conversion (WAV to MP3/M4A if required by OpenAI limits).
    *   Implement error handling for: Invalid Key, Rate Limits (429), and Network Timeouts.
*   **File:** `src-tauri/src/managers/transcription.rs`
    *   Update `TranscriptionManager` to accept a new `EngineType::CloudWhisper`.
    *   Route transcription requests to the OpenAI client when this engine is selected.

### 1.2 Frontend Implementation (React)
*   **File:** `src/components/settings/Models.tsx`
    *   Add an input field for the OpenAI API Key.
    *   Add a dropdown option in the Model Selector for "Cloud: OpenAI Whisper".
    *   Add visual loading states (spinners) specifically for network latency during cloud requests.

### 1.3 Verification (Test Cases)
Create `src-tauri/tests/openai_tests.rs`:
```rust
#[tokio::test]
async fn test_openai_api_key_validation() { /* Validate sk- prefix */ }
#[tokio::test]
async fn test_openai_transcription_network_error() { /* Mock network failure */ }
#[test]
fn test_audio_format_conversion() { /* Ensure correct MIME type for API */ }
```

---

## 📅 Phase 2: Recording Storage Management
**Goal:** Allow users to define retention policies for audio and text (Audio Only, Text Only, or Both).

### 2.1 Backend Implementation (Rust)
*   **File:** `src-tauri/src/settings.rs`
    *   Create enum `RecordingSaveMode { AudioOnly, TextOnly, Both }`.
    *   Add `recording_save_mode` to `Settings` struct.
*   **File:** `src-tauri/src/managers/history.rs`
    *   Modify `save_transcription()`:
        *   **Audio Only:** Do not write the text transcription to the history database/JSON.
        *   **Text Only:** Write text to history, then immediately delete the `.wav` file from disk.
        *   **Both:** Maintain current behavior.

### 2.2 Frontend Implementation (React)
*   **File:** `src/components/settings/History.tsx`
    *   Add a Radio Group or Toggle set for: "Save Audio Only", "Save Text Only", "Save Both".
    *   Ensure the UI reflects the current state from the Rust backend.

### 2.3 Verification (Test Cases)
Create `src-tauri/tests/storage_tests.rs`:
```rust
#[tokio::test]
async fn test_save_text_only() {
    // 1. Simulate recording
    // 2. Assert text exists in history
    // 3. Assert audio file is DELETED from disk
}
#[tokio::test]
async fn test_save_audio_only() {
    // 1. Simulate recording
    // 2. Assert audio file exists
    // 3. Assert no text entry in history
}
```

---

## 📅 Phase 3: Voice-Activated Mode (Talk-Free)
**Goal:** Implement a VAD (Voice Activity Detection) loop that records automatically when the user speaks, remappable via shortcuts.

### 3.1 Backend Implementation (Rust)
*   **File:** `src-tauri/src/managers/audio.rs`
    *   Utilize the existing `SileroVad` implementation.
    *   Implement a state machine: `RecordingMode::PushToTalk` vs `RecordingMode::VoiceActivated`.
    *   **Logic:**
        *   On `VoiceActivated` toggle ON: Open microphone stream immediately.
        *   Buffer audio.
        *   When VAD > Threshold: Set state `Recording`.
        *   When VAD < Threshold for `voice_activated_silence_timeout` (default 2000ms): Stop, Transcribe, and go back to Listening.
*   **File:** `src-tauri/src/shortcut.rs`
    *   Register a new global shortcut `voice_activated_transcribe` distinct from the standard PTT shortcut.

### 3.2 Frontend Implementation (React)
*   **File:** `src/components/settings/Shortcuts.tsx`
    *   Add UI to remap the "Voice Activated Mode" toggle shortcut.
*   **File:** `src/components/Overlay.tsx` (or equivalent recording view)
    *   Add visual distinction:
        *   🔴 "Recording" (Active speech)
        *   🔵 "Listening..." (Waiting for speech)

### 3.3 Verification (Test Cases)
Create `src-tauri/tests/vad_tests.rs`:
```rust
#[test]
fn test_silence_timeout_trigger() {
    // Feed speech samples -> Verify State::Recording
    // Feed 2.1s of silence -> Verify State::Transcribing
}
#[test]
fn test_mode_switching() {
    // Ensure PTT shortcut is ignored or handled safely when in VAD mode
}
```

---

## 📅 Phase 4: Personal Dictionary (Regex Replacement)
**Goal:** Post-processing text replacement for domain-specific corrections (e.g., "copilot" -> "Copilot").

### 4.1 Backend Implementation (Rust)
*   **File:** `src-tauri/src/dictionary.rs` (Create New)
    *   Define struct: `DictionaryEntry { pattern, replacement, is_regex, case_sensitive, enabled }`.
    *   Implement `apply_dictionary(text: String, entries: Vec<DictionaryEntry>) -> String`.
    *   **Performance:** Compile Regex patterns once if possible, or handle compilation errors gracefully without crashing the app.
*   **File:** `src-tauri/src/managers/transcription.rs`
    *   Inject `apply_dictionary` into the pipeline *after* the Whisper/Parakeet engine returns text, but *before* the text is pasted/saved.

### 4.2 Frontend Implementation (React)
*   **File:** `src/components/settings/PersonalDictionary.tsx` (Create New)
    *   CRUD Table: Columns for Pattern, Replacement, Regex Toggle, Case Sensitive Toggle.
    *   "Test" input box to try out rules in real-time.

### 4.3 Verification (Test Cases)
Create `src-tauri/tests/dictionary_tests.rs`:
```rust
#[test]
fn test_regex_replacement() {
    // Pattern: r"react\s*js", Replacement: "React.js"
    // Input: "I like react js" -> Output: "I like React.js"
}
#[test]
fn test_order_of_operations() {
    // Ensure dictionary applies before clipboard paste
}
```

---

## 📝 General Implementation Guidelines for Agent
1.  **Specta:** All new Tauri commands must use `#[specta::specta]` to ensure TypeScript types are generated correctly.
2.  **Error Handling:** Never unwrap() in production code. Use `anyhow` or `thiserror` and return proper Results to the frontend.
3.  **Atomic Commits:** When using git, commit changes after completing each Phase.
4.  **No Regressions:** Run existing test suite before starting Phase 1 to establish a baseline.