use serde::{Deserialize, Serialize};

pub struct Word {
    pub grade: String,
    pub english_word: String,
    pub transcription: String,
    pub russian_word: String,
}
