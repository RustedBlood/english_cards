use dashmap::DashMap;
use serde::{Deserialize, Serialize};

//Структура для парсинга и хранения информации о слове
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Word {
    pub grade: String,
    pub en_word: String,
    pub transcription: String,
    pub ru_word: String,
}

//Структура для хранения слов вида
// уровень английского - ключ
// значение - вектор слов со структурой Word
pub struct WordsDash {
    pub words_dash: DashMap<String, Vec<Word>>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonWords {
    pub grade: String,
    pub words: Vec<Word>,
}
