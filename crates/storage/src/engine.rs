use crate::error::StorageError;
use crate::value::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Engine {
    store: Arc<RwLock<HashMap<String, Value>>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<Value>, StorageError> {
        let store = self.store.read().map_err(StorageError::from)?;
        Ok(store.get(key).cloned())
    }

    pub fn set(&self, key: String, value: Value) -> Result<(), StorageError> {
        let mut store = self.store.write().map_err(StorageError::from)?;
        store.insert(key, value);
        Ok(())
    }

    pub fn delete(&self, key: &str) -> Result<Option<Value>, StorageError> {
        let mut store = self.store.write().map_err(StorageError::from)?;
        Ok(store.remove(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_raw_for_string() {
        let engine = Engine::new();
        let _ = engine.set(
            "foo".to_string(),
            Value::from_serializable(&"bar").expect("Failed to create value object"),
        );

        match engine.get("foo") {
            Ok(opt) => assert_eq!(
                opt,
                Some(Value::from_serializable(&"bar").expect("Failed to create value object"))
            ),
            Err(e) => panic!("error: {}", e),
        };
    }

    #[test]
    fn test_set_and_get_deserialized_for_string() {
        let engine = Engine::new();
        let string_value = "bar".to_string();
        let _ = engine.set(
            "foo".to_string(),
            Value::from_serializable(&string_value).expect("Failed to create value object"),
        );

        match engine.get("foo") {
            Ok(opt) => {
                match opt {
                    Some(val) => assert_eq!(
                        string_value,
                        val.to_deserializable::<String>()
                            .expect("Failed to deserialize value")
                    ),
                    None => panic!("failed to get value"),
                };
            }
            Err(e) => panic!("error: {}", e),
        };

        println!("value[foo]: {:?}", engine.get("foo"));
    }

    #[test]
    fn test_delete() {
        let engine = Engine::new();
        let _ = engine.set(
            "foo".to_string(),
            Value::from_serializable(&"bar").expect("Failed to create value object"),
        );

        match engine.delete("foo") {
            Ok(opt) => assert_eq!(
                opt,
                Some(Value::from_serializable(&"bar").expect("Failed to create value object"))
            ),
            Err(e) => panic!("error: {}", e),
        };

        match engine.get("foo") {
            Ok(opt) => assert_eq!(opt, None),
            Err(e) => panic!("error: {}", e),
        };
    }
}
