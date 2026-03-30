/*
   The technical specification was prepared using Gemini

 * PROJECT: RustyCache
 * DESCRIPTION: A simple in-memory key-value store with TTL (Time-To-Live) support.
 *
 * GOALS:
 * 1. Define an enum `CacheValue` to store different data types:
 * - Text(String)
 * - Numeric(i64)
 * - Boolean(bool)
 *
 * 2. Define a struct `CacheEntry` to represent a single record:
 * - value: CacheValue
 * - created_at: std::time::Instant (to track when it was added)
 * - ttl: std::time::Duration (how long the record is valid)
 *
 * 3. Create a `Cache` struct that manages the storage:
 * - store: std::collections::HashMap<String, CacheEntry>
 *
 * 4. Implement the following methods for `Cache`:
 * - new() -> Self: Initialize an empty cache.
 * - set(&mut self, key: &str, value: CacheValue, ttl_secs: u64): 
 * Insert a value with a specific TTL in seconds.
 *
 * - get(&mut self, key: &str) -> Option<&CacheValue>: 
 * Retrieve a value only if it exists AND has not expired. 
 * CRITICAL: If expired, remove it from the map and return None.
 *
 * - remove(&mut self, key: &str) -> Result<(), String>: 
 * Remove a key. Return Err if the key wasn't found.
 *
 * - stats(&self): Print the total number of entries and 
 * breakdown by type (how many Text, Numeric, etc.).
 *
 * 5. MAIN FUNCTION:
 * - Demonstrate all functionality.
 * - Test TTL by using `std::thread::sleep` to wait for expiration.
 * 
 * 
 */

use std::collections::HashMap;
use std::time::{Instant, Duration};

enum CacheValue {
    Text(String),
    Nimeric(i64),
    Boolean(bool)
}

struct CacheEntry {
    value: CacheValue,
    created_at: Instant,
    ttl: Duration
}

struct Cache {
    store: std::collections::HashMap<String, CacheEntry>
}

impl Cache {
    fn new() -> Self {
        Self {
            store: std::collections::HashMap::new()
        }
    }

    fn set(&mut self, key: &str, value: CacheValue, ttl_secs: u64) {
        let entry = CacheEntry {
            value: value,
            created_at: Instant::now(),
            ttl: Duration::from_secs(ttl_secs),

        };

        self.store.insert(key.to_string(), entry);
    }

    fn get(&mut self, key: &str) -> Option<&CacheValue> {
        let expired = match self.store.get(key) {
            Some(cv) => cv.created_at.elapsed() > cv.ttl,
            None => return None,
        };

        if expired {
            self.store.remove(key);
            return None
        }

        match self.store.get(key) {
            Some(cv) => Some(&cv.value),
            None => None,
        }
    }

    fn remove(&mut self, key: &str) -> Result<(), String> {
        match self.store.remove(key) {
            Some(_v) => Ok(()),
            None => Err(String::from("Key not found"))
        }
    }
}




fn main() {

}

