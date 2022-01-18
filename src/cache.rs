use std::time::{Duration, SystemTime};
use std::vec;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct TimeOutCache<T: Send + Sync + 'static> {
    inner: Arc<Mutex<HashMap<String, (T, SystemTime)>>>,
    join: JoinHandle<()>,
}

impl<T> TimeOutCache<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(duration: u64) -> Self {
        let duration = Duration::from_secs(duration);
        let map: Arc<Mutex<HashMap<String, (T, SystemTime)>>> = Arc::default();
        let inner = map.clone();
        let join = tokio::spawn(async move {
            loop {
                tokio::time::sleep(duration).await;
                let mut keys = vec![];
                {
                    let mut locked_map = map.lock().await;
                    for (k, v) in locked_map.iter_mut() {
                        if let Ok(_) = v.1.elapsed() {
                            keys.push(k.clone());
                        }
                    }
                    for k in keys {
                        locked_map.remove(&k);
                    }
                }
            }
        });
        Self { inner, join }
    }

    pub async fn insert(&self, key: String, value: T, delay: u64) {
        let time = SystemTime::now() + Duration::from_secs(delay);
        self.inner.lock().await.insert(key, (value, time));
    }

    pub async fn update(&self, key: &str, delay: u64) -> bool {
        if let Some((_, time)) = self.inner.lock().await.get_mut(key) {
            *time = SystemTime::now() + Duration::from_secs(delay);
            true
        } else {
            false
        }
    }
}

impl<T> Default for TimeOutCache<T>
where
    T: Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new(60)
    }
}

impl<T> std::ops::Drop for TimeOutCache<T>
where
    T: Send + Sync + 'static,
{
    fn drop(&mut self) {
        self.join.abort();
    }
}
