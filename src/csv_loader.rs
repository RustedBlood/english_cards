use crate::models::Word;
use dashmap::DashMap;
use scv;
use std::sync::Arc;

pub struct WordLoader {
    cache: Arc<DashMap<String, Vec<Word>>>,
    data_dir: String,
}

impl WordLoader {
    pub fn new(&self, grade: &str) -> Self {
        let filename = format!("{}.csv", grade);
        Self {
            cache: Arc::new(DashMap::new()),
            data_dir: filename,
        }
    }
    pub fn get_words(&self, grade: &str) {}
}
