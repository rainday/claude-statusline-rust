use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheEntry<T: Serialize> {
    pub data: T,
    pub cached_at: u64,
}

pub fn read_cache<T: for<'de> Deserialize<'de> + Serialize>(
    path: &PathBuf,
    ttl_secs: u64,
) -> Option<T> {
    let content = fs::read_to_string(path).ok()?;
    let entry: CacheEntry<T> = serde_json::from_str(&content).ok()?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    if now - entry.cached_at > ttl_secs {
        return None;
    }
    Some(entry.data)
}

pub fn read_cache_stale<T: for<'de> Deserialize<'de> + Serialize>(path: &PathBuf) -> Option<T> {
    let content = fs::read_to_string(path).ok()?;
    let entry: CacheEntry<T> = serde_json::from_str(&content).ok()?;
    Some(entry.data)
}

pub fn write_cache<T: Serialize>(path: &PathBuf, data: &T) -> Option<()> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let entry = CacheEntry {
        data,
        cached_at: now,
    };
    let json = serde_json::to_string_pretty(&entry).ok()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok()?;
    }
    fs::write(path, json).ok()
}
