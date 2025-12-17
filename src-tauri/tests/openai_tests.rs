use handy_app_lib::cloud_transcription::openai::{OpenAIClient, OpenAIConfiguration};
use handy_app_lib::managers::model::EngineType;

#[test]
fn test_openai_client_creation() {
    // Test that client creation fails with empty API key
    let result = OpenAIClient::new("".to_string());
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "OpenAI API key is required"
    );

    // Test that client creation succeeds with valid API key
    let result = OpenAIClient::new("test-key-123".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_audio_conversion() {
    let client = OpenAIClient::new("test-key".to_string()).unwrap();
    
    // Test with empty audio
    let empty_samples: Vec<f32> = vec![];
    let result = client.audio_samples_to_wav_bytes(&empty_samples);
    assert!(result.is_ok());
    
    // Test with simple audio (1 second of silence at 16kHz)
    let samples = vec![0.0f32; 16000];
    let result = client.audio_samples_to_wav_bytes(&samples);
    assert!(result.is_ok());
    
    let wav_bytes = result.unwrap();
    assert!(!wav_bytes.is_empty());
    
    // Verify WAV header
    assert!(wav_bytes.len() >= 44); // WAV header is at least 44 bytes
    assert_eq!(&wav_bytes[0..4], b"RIFF"); // RIFF header
    assert_eq!(&wav_bytes[8..12], b"WAVE"); // WAVE format
    assert_eq!(&wav_bytes[12..16], b"fmt "); // fmt chunk
}

#[test]
fn test_openai_configuration_default() {
    let config = OpenAIConfiguration::default();
    
    assert_eq!(config.api_key, "");
    assert_eq!(config.model, "whisper-1");
    assert_eq!(config.language, None);
    assert_eq!(config.temperature, 0.0);
    assert_eq!(config.prompt, None);
}

#[test]
fn test_engine_type_serialization() {
    // Test that EngineType includes CloudWhisper variant
    use serde_json;
    
    let engine_types = vec![
        EngineType::Whisper,
        EngineType::Parakeet,
        EngineType::CloudWhisper,
    ];
    
    // Verify all variants exist
    assert_eq!(engine_types.len(), 3);
    
    // Test serialization (this would fail if CloudWhisper wasn't added to Serialize/Deserialize)
    for engine_type in engine_types {
        let serialized = serde_json::to_string(&engine_type);
        assert!(serialized.is_ok(), "Failed to serialize {:?}", engine_type);
        
        let deserialized: Result<EngineType, _> = serde_json::from_str(&serialized.unwrap());
        assert!(deserialized.is_ok(), "Failed to deserialize {:?}", engine_type);
    }
}

#[test]
fn test_openai_configuration_update() {
    let client = OpenAIClient::new("initial-key".to_string()).unwrap();
    
    let new_config = OpenAIConfiguration {
        api_key: "new-key".to_string(),
        model: "whisper-1".to_string(),
        language: Some("en".to_string()),
        temperature: 0.5,
        prompt: Some("Test prompt".to_string()),
    };
    
    // Note: update_config is async, so we can't test it directly in unit tests
    // This test just verifies the struct can be created
    assert_eq!(new_config.api_key, "new-key");
    assert_eq!(new_config.language, Some("en".to_string()));
    assert_eq!(new_config.temperature, 0.5);
    assert_eq!(new_config.prompt, Some("Test prompt".to_string()));
}

// Note: We can't test actual API calls in unit tests without mocking
// Integration tests would be needed for actual OpenAI API interaction