//! Generic voice interaction layer for Mazerion.
//!
//! This crate defines abstract traits for voice input/output and a simple
//! session driver. Concrete implementations (desktop, Android, WearOS, etc.)
//! live in platform-specific crates or applications.

use thiserror::Error;

/// Errors that can occur while handling voice I/O or delegating to the engine.
#[derive(Debug, Error)]
pub enum VoiceError {
    #[error("speech input error: {0}")]
    Input(String),

    #[error("speech output error: {0}")]
    Output(String),

    #[error("engine error: {0}")]
    Engine(String),
}

/// Trait for something that can provide text transcriptions from the user.
pub trait VoiceInput {
    /// Block or poll until the next utterance is available.
    ///
    /// Returning `Ok(None)` means the session ended cleanly.
    fn next_utterance(&mut self) -> Result<Option<String>, VoiceError>;
}

/// Trait for something that can speak text back to the user.
pub trait VoiceOutput {
    fn speak(&mut self, text: &str) -> Result<(), VoiceError>;
}

/// Minimal abstraction over "something that can answer text queries".
///
/// In the future, you can implement this for a thin wrapper around
/// `mazerion_api::ApiEngine` that parses utterances and selects calculators.
pub trait QueryEngine {
    /// Handle a single user utterance (already converted to text) and produce
    /// a textual response to be spoken or displayed.
    fn handle_text(&mut self, utterance: &str) -> Result<String, VoiceError>;
}

/// High-level orchestration for a voice session.
///
/// This does not know anything about Mazerion calculators directly.
/// It just glues together `VoiceInput`, `VoiceOutput`, and a `QueryEngine`.
pub struct VoiceSession<I, O, E>
where
    I: VoiceInput,
    O: VoiceOutput,
    E: QueryEngine,
{
    input: I,
    output: O,
    engine: E,
}

impl<I, O, E> VoiceSession<I, O, E>
where
    I: VoiceInput,
    O: VoiceOutput,
    E: QueryEngine,
{
    pub fn new(input: I, output: O, engine: E) -> Self {
        Self {
            input,
            output,
            engine,
        }
    }

    /// Run an interaction loop until the input source ends or an error occurs.
    ///
    /// This is generic enough to be reused on Desktop, Android, and WearOS.
    pub fn run(mut self) -> Result<(), VoiceError> {
        loop {
            let Some(utterance) = self.input.next_utterance()? else {
                // Session ended cleanly.
                return Ok(());
            };

            let reply = self.engine.handle_text(&utterance)?;
            self.output.speak(&reply)?;
        }
    }
}
