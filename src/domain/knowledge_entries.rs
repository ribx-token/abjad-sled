use std::collections::HashMap;
use sled::Db;
use crate::learning::models::knowledge::Knowledge;
use crate::utils::read_from_sled::read_from_sled;

pub fn knowledge_entries(db: &Db, level: Option<u8>) -> HashMap<String, Knowledge> {
    db.iter()
        .filter_map(Result::ok)
        .filter_map(|(key, _)| {
            read_from_sled::<Knowledge, _>(&db, &key)
        })
        .filter(|knowledge| {
            // Filter out entries without audio
            knowledge.audio.is_some() &&
            // Filter by level if provided
            level.map_or(true, |lvl| knowledge.level.map_or(false, |k_lvl| k_lvl == lvl as u8))
        })
        .map(|knowledge| (knowledge.id.clone(), knowledge))
        .collect()
}
