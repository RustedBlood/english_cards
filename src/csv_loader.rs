use dashmap::DashMap;

use crate::models::{Word, WordsDash};

impl WordsDash {
    pub fn new() -> Self {
        let new_dash: DashMap<String, Vec<Word>> = DashMap::new();

        let mut readed_csv_db = csv::Reader::from_path("db/words.csv").unwrap();

        for recorded in readed_csv_db.deserialize() {
            let record: Word = recorded.unwrap();

            let grade = record.grade.clone();

            new_dash
                .entry(grade)
                .or_insert_with(|| Vec::new())
                .push(record);
        }
        Self {
            words_dash: new_dash,
        }
    }
}
