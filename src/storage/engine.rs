use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::storage::error::StorageError;

pub struct Engine {
    store: Arc<RwLock<HashMap<String, String>>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        let store = self.store.read().map_err(StorageError::from)?;
        Ok(store.get(key).cloned())
    }

    pub fn set(&self, key: String, value: String) -> Result<(), StorageError> {
        let mut store = self.store.write().map_err(StorageError::from)?;
        store.insert(key, value);
        Ok(())
    }

    pub fn delete(&self, key: &str) -> Result<Option<String>, StorageError> {
        let mut store = self.store.write().map_err(StorageError::from)?;
        Ok(store.remove(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let engine = Engine::new();
        let _ = engine.set("foo".to_string(), "bar".to_string());

        match engine.get("foo") {
            Ok(opt) => assert_eq!(opt, Some("bar".to_string())),
            Err(e) => panic!("error: {}", e),
        };
    }

    #[test]
    fn test_delete() {
        let engine = Engine::new();
        let _ = engine.set("foo".to_string(), "bar".to_string());

        match engine.delete("foo") {
            Ok(opt) => assert_eq!(opt, Some("bar".to_string())),
            Err(e) => panic!("error: {}", e),
        };

        match engine.get("foo") {
            Ok(opt) => assert_eq!(opt, None),
            Err(e) => panic!("error: {}", e),
        };
    }
}
