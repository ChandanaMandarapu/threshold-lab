use serde::{Serialize, Deserialize};
use crate::types::Shard;
use std::fs;

#[derive(Serialize, Deserialize)]
struct ShardRecord {
    x: u64,
    y: i64,
}

pub fn save_shards(shards: &[Shard], path: &str) -> Result<(), String> {
    let records: Vec<ShardRecord> = shards
        .iter()
        .map(|s| ShardRecord { x: s.x, y: s.y })
        .collect();
    let data = serde_json::to_string_pretty(&records).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())
}

pub fn load_shards(path: &str) -> Result<Vec<Shard>, String> {
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let records: Vec<ShardRecord> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(records.into_iter().map(|r| Shard::new(r.x, r.y)).collect())
}