use handy_app::settings::{RecordingMode, RecordingSaveMode};

#[test]
fn test_recording_mode_enum_serialization() {
    // Test that RecordingMode enum serializes correctly
    let push_to_talk = RecordingMode::PushToTalk;
    let voice_activated = RecordingMode::VoiceActivated;
    
    // Test default value
    assert_eq!(RecordingMode::default(), RecordingMode::PushToTalk);
    
    // Test serialization to string (through serde)
    let push_to_talk_str = serde_json::to_string(&push_to_talk).unwrap();
    let voice_activated_str = serde_json::to_string(&voice_activated).unwrap();
    
    assert!(push_to_talk_str.contains("push_to_talk"));
    assert!(voice_activated_str.contains("voice_activated"));
}

#[test]
fn test_recording_save_mode_enum_serialization() {
    // Test that RecordingSaveMode enum serializes correctly
    let audio_only = RecordingSaveMode::AudioOnly;
    let text_only = RecordingSaveMode::TextOnly;
    let both = RecordingSaveMode::Both;
    
    // Test default value
    assert_eq!(RecordingSaveMode::default(), RecordingSaveMode::Both);
    
    // Test serialization to string (through serde)
    let audio_only_str = serde_json::to_string(&audio_only).unwrap();
    let text_only_str = serde_json::to_string(&text_only).unwrap();
    let both_str = serde_json::to_string(&both).unwrap();
    
    assert!(audio_only_str.contains("audio_only"));
    assert!(text_only_str.contains("text_only"));
    assert!(both_str.contains("both"));
}

#[test]
fn test_settings_struct_includes_new_fields() {
    // Test that AppSettings struct includes the new fields
    use handy_app::settings::AppSettings;
    use std::collections::HashMap;
    
    let settings = AppSettings {
        bindings: HashMap::new(),
        push_to_talk: true,
        audio_feedback: false,
        audio_feedback_volume: 1.0,
        sound_theme: handy_app::settings::SoundTheme::Marimba,
        start_hidden: false,
        autostart_enabled: false,
        update_checks_enabled: true,
        selected_model: "".to_string(),
        always_on_microphone: false,
        selected_microphone: None,
        clamshell_microphone: None,
        selected_output_device: None,
        translate_to_english: false,
        selected_language: "auto".to_string(),
        overlay_position: handy_app::settings::OverlayPosition::Bottom,
        debug_mode: false,
        log_level: handy_app::settings::LogLevel::Debug,
        custom_words: Vec::new(),
        model_unload_timeout: handy_app::settings::ModelUnloadTimeout::Never,
        word_correction_threshold: 0.18,
        history_limit: 5,
        recording_retention_period: handy_app::settings::RecordingRetentionPeriod::PreserveLimit,
        paste_method: handy_app::settings::PasteMethod::CtrlV,
        clipboard_handling: handy_app::settings::ClipboardHandling::DontModify,
        post_process_enabled: false,
        post_process_provider_id: "openai".to_string(),
        post_process_providers: Vec::new(),
        post_process_api_keys: HashMap::new(),
        post_process_models: HashMap::new(),
        post_process_prompts: Vec::new(),
        post_process_selected_prompt_id: None,
        mute_while_recording: false,
        append_trailing_space: false,
        app_language: "en".to_string(),
        openai_api_key: "".to_string(),
        openai_model: "whisper-1".to_string(),
        recording_save_mode: RecordingSaveMode::Both,
        recording_mode: RecordingMode::PushToTalk,
        voice_activated_silence_timeout: 2000,
    };
    
    // Test that the fields are accessible
    assert_eq!(settings.recording_save_mode, RecordingSaveMode::Both);
    assert_eq!(settings.recording_mode, RecordingMode::PushToTalk);
    assert_eq!(settings.voice_activated_silence_timeout, 2000);
}

// Note: In a real test environment, we would also test:
// 1. The audio manager's voice-activated mode state machine
// 2. The VAD integration with silence timeout
// 3. The shortcut registration for voice-activated mode toggle
// 4. The frontend integration

// However, these tests require mocking audio devices and VAD,
// which is complex in a unit test environment.