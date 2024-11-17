use std::{
    collections::HashMap,
    fmt,
    sync::{atomic::AtomicI64, Arc},
};

use anyhow::Result;

pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        Self {
            data: Arc::new(
                metric_names
                    .iter()
                    .map(|&name| (name, AtomicI64::new(0)))
                    .collect(),
            ),
        }
    }

    pub fn increment(&self, key: impl AsRef<str>) -> Result<()> {
        self.data
            .get(key.as_ref())
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key.as_ref()))?
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    pub fn decrement(&self, key: impl AsRef<str>) -> Result<()> {
        self.data
            .get(key.as_ref())
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key.as_ref()))?
            .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(
                f,
                "{}: {}",
                key,
                value.load(std::sync::atomic::Ordering::Relaxed)
            )?;
        }
        Ok(())
    }
}
