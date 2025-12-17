use anyhow::{anyhow, Result};
use async_openai::{
    config::OpenAIConfig,
    types::{
        AudioInput, AudioResponseFormat, CreateTranscriptionRequest, CreateTranscriptionRequestArgs,
    },
    Client,
};
use hound::{WavSpec, WavWriter};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Configuration for OpenAI Whisper API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfiguration {
    pub api_key: String,
    pub model: String,
    pub language: Option<String>,
    pub temperature: f32,
    pub prompt: Option<String>,
}

impl Default for OpenAIConfiguration {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "whisper-1".to_string(),
            language: None,
            temperature: 0.0,
            prompt: None,
        }
    }
}

/// Client for OpenAI Whisper API transcription
#[derive(Clone)]
pub struct OpenAIClient {
    client: Client<OpenAIConfig>,
    config: Arc<Mutex<OpenAIConfiguration>>,
}

impl OpenAIClient {
    /// Create a new OpenAI client with the given API key
    pub fn new(api_key: String) -> Result<Self> {
        if api_key.is_empty() {
            return Err(anyhow!("OpenAI API key is required"));
        }

        let config = OpenAIConfig::new().with_api_key(api_key.clone());
        let client = Client::with_config(config);

        Ok(Self {
            client,
            config: Arc::new(Mutex::new(OpenAIConfiguration {
                api_key,
                ..Default::default()
            })),
        })
    }

    /// Update the configuration
    pub async fn update_config(&self, config: OpenAIConfiguration) {
        let mut current_config = self.config.lock().await;
        *current_config = config;
    }

    /// Convert raw f32 audio samples to WAV format bytes
    /// Audio is expected to be mono, 16kHz sample rate
    fn audio_samples_to_wav_bytes(&self, samples: &[f32]) -> Result<Vec<u8>> {
        let spec = WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut buffer = Vec::new();
        {
            let mut writer = WavWriter::new(Cursor::new(&mut buffer), spec)?;

            // Convert f32 samples to i16 for WAV format
            for sample in samples {
                // Clamp sample to [-1.0, 1.0] and convert to i16
                let clamped_sample = sample.clamp(-1.0, 1.0);
                let int_sample = (clamped_sample * i16::MAX as f32) as i16;
                writer.write_sample(int_sample)?;
            }

            writer.finalize()?;
        }

        Ok(buffer)
    }

    /// Transcribe audio using OpenAI Whisper API
    pub async fn transcribe(
        &self,
        audio_samples: Vec<f32>,
        language: Option<String>,
        translate_to_english: bool,
    ) -> Result<String> {
        let config = self.config.lock().await.clone();

        if config.api_key.is_empty() {
            return Err(anyhow!("OpenAI API key is not configured"));
        }

        // Convert audio samples to WAV format
        let wav_bytes = self.audio_samples_to_wav_bytes(&audio_samples)?;
        debug!("Converted {} samples to WAV ({} bytes)", audio_samples.len(), wav_bytes.len());

        // Create the transcription request
        let mut request_builder = CreateTranscriptionRequestArgs::default();

        request_builder
            .model(&config.model)
            .file(AudioInput::Bytes {
                data: wav_bytes.into(),
                file_name: "audio.wav".to_string(),
            })
            .response_format(AudioResponseFormat::Json);

        // Set language if provided
        if let Some(lang) = language {
            request_builder.language(&lang);
        } else if let Some(config_lang) = &config.language {
            request_builder.language(config_lang);
        }

        // Set prompt if configured
        if let Some(prompt) = &config.prompt {
            request_builder.prompt(prompt);
        }

        // Set temperature
        request_builder.temperature(config.temperature);

        let request = request_builder
            .build()
            .map_err(|e| anyhow!("Failed to build transcription request: {}", e))?;

        info!("Sending transcription request to OpenAI Whisper API");
        let start_time = std::time::Instant::now();

        // Make the API call
        let response = self
            .client
            .audio()
            .create_transcription(request)
            .await
            .map_err(|e| anyhow!("OpenAI API error: {}", e))?;

        let duration = start_time.elapsed();
        info!("OpenAI transcription completed in {}ms", duration.as_millis());

        Ok(response.text)
    }

    /// Validate the API key by making a simple request
    pub async fn validate_api_key(&self) -> Result<()> {
        let config = self.config.lock().await.clone();

        if config.api_key.is_empty() {
            return Err(anyhow!("OpenAI API key is empty"));
        }

        // Try to list models as a simple validation
        // Note: This requires appropriate permissions
        match self.client.models().list().await {
            Ok(_) => {
                debug!("OpenAI API key validation successful");
                Ok(())
            }
            Err(e) => {
                error!("OpenAI API key validation failed: {}", e);
                Err(anyhow!("Invalid API key: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_conversion() {
        // Create a simple test client with dummy API key
        let client = OpenAIClient::new("test-key".to_string()).unwrap();
        
        // Create test audio samples (1 second of silence at 16kHz)
        let samples = vec![0.0f32; 16000];
        
        // Convert to WAV
        let result = client.audio_samples_to_wav_bytes(&samples);
        assert!(result.is_ok());
        
        let wav_bytes = result.unwrap();
        assert!(!wav_bytes.is_empty());
        
        // Verify it's a valid WAV file by checking the header
        assert!(wav_bytes.len() > 44); // WAV header is 44 bytes
        assert_eq!(&wav_bytes[0..4], b"RIFF"); // RIFF header
        assert_eq!(&wav_bytes[8..12], b"WAVE"); // WAVE format
    }

    #[test]
    fn test_empty_api_key() {
        let result = OpenAIClient::new("".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "OpenAI API key is required");
    }
}