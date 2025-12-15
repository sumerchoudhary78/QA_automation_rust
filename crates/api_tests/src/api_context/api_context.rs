use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

use crate::common::ApiResponse;

#[derive(Debug, thiserror::Error)]
pub enum ApiContextError {
    #[error("Key '{0}' not found in context")]
    KeyNotFound(String),

    #[error("Failed to deserialize value for key '{0}': {1}")]
    DeserializationError(String, String),

    #[error("Failed to serialize value: {0}")]
    SerializationError(String),

    #[error("Invalid JSON path '{0}': {1}")]
    InvalidPath(String, String),

    #[error("Lock poisoned")]
    LockPoisoned,
}

#[derive(Debug, Clone, Default)]
pub struct ApiContext {
    data: Arc<RwLock<HashMap<String, Value>>>,
}

impl ApiContext {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn global() -> &'static ApiContext {
        static GLOBAL: LazyLock<ApiContext> = LazyLock::new(|| ApiContext::new());
        &GLOBAL
    }

    pub fn store<T: Serialize>(&self, key: &str, value: &T) -> Result<(), ApiContextError> {
        let json_value = serde_json::to_value(value)
            .map_err(|e| ApiContextError::SerializationError(e.to_string()))?;

        self.data
            .write()
            .map_err(|_| ApiContextError::LockPoisoned)?
            .insert(key.to_string(), json_value);

        Ok(())
    }

    pub fn store_raw(
        &self,
        instance: &str,
        key: &str,
        value: Value,
    ) -> Result<(), ApiContextError> {
        let instance_key = format!("{} {}", instance, key);
        self.data
            .write()
            .map_err(|_| ApiContextError::LockPoisoned)?
            .insert(instance_key, value);

        Ok(())
    }

    pub fn store_row_map<T: Serialize + Clone>(
        &self,
        instance: &str,
        key: &str,
        value: ApiResponse<T>,
    ) -> Result<(), ApiContextError> {
        let instance_key = format!("{} {}", instance, key);
        let json_value = serde_json::to_value(&value)
            .map_err(|e| ApiContextError::SerializationError(e.to_string()))?;
        self.data
            .write()
            .map_err(|_| ApiContextError::LockPoisoned)?
            .insert(instance_key, json_value);

        Ok(())
    }

    pub fn get_row_map<T: DeserializeOwned>(
        &self,
        instance: &str,
        key: &str,
        id: i64,
    ) -> Result<T, ApiContextError> {
        let instance_key = format!("{} {}", instance, key);
        let data = self
            .data
            .read()
            .map_err(|_| ApiContextError::LockPoisoned)?;

        let response = data
            .get(&instance_key)
            .ok_or_else(|| ApiContextError::KeyNotFound(instance_key.clone()))?;

        let items = response["data"].as_array().ok_or_else(|| {
            ApiContextError::InvalidPath("data".to_string(), "not an array".to_string())
        })?;

        let item = items
            .iter()
            .find(|item| item["id"].as_i64() == Some(id))
            .ok_or_else(|| ApiContextError::KeyNotFound(format!("{} id={}", instance_key, id)))?;

        serde_json::from_value(item.clone())
            .map_err(|e| ApiContextError::DeserializationError(instance_key, e.to_string()))
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, ApiContextError> {
        let data = self
            .data
            .read()
            .map_err(|_| ApiContextError::LockPoisoned)?;

        let value = data
            .get(key)
            .ok_or_else(|| ApiContextError::KeyNotFound(key.to_string()))?;

        serde_json::from_value(value.clone())
            .map_err(|e| ApiContextError::DeserializationError(key.to_string(), e.to_string()))
    }

    pub fn get_raw(&self, instance: &str, key: &str) -> Result<Value, ApiContextError> {
        let instance_key = format!("{} {}", instance, key);
        let data = self
            .data
            .read()
            .map_err(|_| ApiContextError::LockPoisoned)?;

        data.get(&instance_key)
            .cloned()
            .ok_or_else(|| ApiContextError::KeyNotFound(instance_key.to_string()))
    }

    pub fn get_field<T: DeserializeOwned>(
        &self,
        instance: &str,
        key: &str,
        path: &str,
    ) -> Result<T, ApiContextError> {
        let value = self.get_field_raw(instance, key, path)?;

        serde_json::from_value(value).map_err(|e| {
            ApiContextError::DeserializationError(format!("{}.{}", key, path), e.to_string())
        })
    }

    pub fn get_field_raw(
        &self,
        instance: &str,
        key: &str,
        path: &str,
    ) -> Result<Value, ApiContextError> {
        let instance_key = format!("{} {}", instance, key);
        let data = self
            .data
            .read()
            .map_err(|_| ApiContextError::LockPoisoned)?;

        let root = data
            .get(&instance_key)
            .ok_or_else(|| ApiContextError::KeyNotFound(instance_key.to_string()))?;

        let mut current = root;
        for segment in path.split('.') {
            current = current.get(segment).ok_or_else(|| {
                ApiContextError::InvalidPath(
                    path.to_string(),
                    format!("segment '{}' not found", segment),
                )
            })?;
        }

        Ok(current.clone())
    }

    pub fn contains(&self, instance: &str, key: &str) -> bool {
        let instance_key = format!("{} {}", instance, key);
        self.data
            .read()
            .map(|data| data.contains_key(&instance_key))
            .unwrap_or(false)
    }

    pub fn remove(&self, instance: &str, key: &str) -> Result<Option<Value>, ApiContextError> {
        let instance_key = format!("{} {}", instance, key);
        self.data
            .write()
            .map_err(|_| ApiContextError::LockPoisoned)?
            .remove(&instance_key)
            .map_or(Ok(None), |v| Ok(Some(v)))
    }

    pub fn clear(&self) -> Result<(), ApiContextError> {
        self.data
            .write()
            .map_err(|_| ApiContextError::LockPoisoned)?
            .clear();

        Ok(())
    }

    pub fn keys(&self) -> Result<Vec<String>, ApiContextError> {
        let data = self
            .data
            .read()
            .map_err(|_| ApiContextError::LockPoisoned)?;

        Ok(data.keys().cloned().collect())
    }

    pub fn len(&self) -> usize {
        self.data.read().map(|d| d.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
