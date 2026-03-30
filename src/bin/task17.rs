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

#[derive(Debug)]
enum CacheValue {
    Text(String),
    Numeric(i64),
    Boolean(bool)
}

struct CacheEntry {
    value: CacheValue,
    created_at: Instant,
    ttl: Duration
}

struct Cache {
    store: HashMap<String, CacheEntry>
}

impl Cache {
    fn new() -> Self {
        Self {
            store: HashMap::new()
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
            None => Err(format!("Key '{}' not found", key))
        }
    }

    fn stats(&self) {
        let mut text_count = 0;
        let mut numeric_count = 0;
        let mut boolean_count = 0;

        for entry in self.store.values() {


            match &entry.value {
                CacheValue::Text(_) => text_count += 1,
                CacheValue::Numeric(_) => numeric_count += 1,
                CacheValue::Boolean(_) => boolean_count += 1,
            }
        }

        println!("--- CACHE STATS ---");
        println!("Total entries: {}", self.store.len());
        println!("Strings: {}, Numbers: {}, Booleans: {}", text_count, numeric_count, boolean_count);
    }
}


fn main() {
    use std::{thread::sleep, time::Duration};

    let mut cache = Cache::new();

    cache.set("name", CacheValue::Text("Arsenii".to_string()), 10);
    cache.set("age", CacheValue::Numeric(17), 5);
    cache.set("active", CacheValue::Boolean(true), 3);

    println!("--- Initial stats ---");
    cache.stats();

    println!("\n--- Get values ---");
    if let Some(value) = cache.get("name") {
        match value {
            CacheValue::Text(s) => println!("name: {s}"),
            CacheValue::Numeric(n) => println!("name (num): {n}"),
            CacheValue::Boolean(b) => println!("name (bool): {b}"),
        }
    }

    println!("\n--- TTL test ---");
    cache.set("temp", CacheValue::Text("temporary".to_string()), 2);

    println!("temp before sleep: {:?}", cache.get("temp"));

    sleep(Duration::from_secs(3));

    println!("temp after sleep: {:?}", cache.get("temp"));


    println!("\n--- Remove ---");
    match cache.remove("age") {
        Ok(()) => println!("age removed"),
        Err(e) => println!("error: {e}"),
    }

    match cache.remove("unknown") {
        Ok(()) => println!("unknown removed"),
        Err(e) => println!("error: {e}"),
    }


    println!("\n--- Final stats ---");
    cache.stats();
}
