use cfg_if::cfg_if;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct Cached<T> {
    value: T,
    duration: Duration,
    cached_at: Instant,
}

#[derive(Debug, Clone)]
pub enum CacheData {
    Exists(Cached<bool>), // I used String here as a placeholder for your cached data type
    Html(String),
    Expired,
    // NotFound
}

#[derive(Debug, Clone)]
pub struct AppCache(Arc<Mutex<HashMap<String, CacheData>>>);

impl AppCache {
    pub fn get_value(&self, key: String) -> Option<CacheData> {
        match self.0.lock() {
            Ok(v) => match v.get(&key) {
                Some(y) => Some(y.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn set_value(&self, key: String, data: CacheData) -> Option<CacheData> {
        match self.0.lock() {
            Ok(mut v) => v.insert(key, data),
            _ => None,
        }
    }

    pub fn get_exists(&self, key: String) -> Option<bool> {
        let value = self.get_value(key.clone())?;
        match value {
            CacheData::Exists(cached) => {
                if cached.cached_at.elapsed() > cached.duration {
                    self.set_value(key, CacheData::Expired);
                    return None;
                }
                Some(cached.value.clone())
            }
            _ => None,
        }
    }

    pub fn set_exists(&self, key: String, exists: bool) -> Option<()> {
        self.set_value(
            key,
            CacheData::Exists(Cached {
                value: exists,
                duration: Duration::from_secs(86_400),
                cached_at: Instant::now(),
            }),
        )?;
        Some(())
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos_router::RouteListing;
        use leptos::LeptosOptions;
        use axum::extract::FromRef;

        #[derive(Debug, Clone)]
        pub struct AppRoutes(pub Vec<RouteListing>);

        #[derive(Debug, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub routes: AppRoutes,
            pub cache: AppCache,
        }

        impl FromRef<AppState> for AppCache {
            fn from_ref(app_state: &AppState) -> AppCache {
                AppCache(Arc::clone(&app_state.cache.0))
            }
        }

        impl FromRef<AppState> for LeptosOptions {
            fn from_ref(app_state: &AppState) -> LeptosOptions {
                app_state.leptos_options.clone()
            }
        }

        impl FromRef<AppState> for AppRoutes {
            fn from_ref(app_state: &AppState) -> AppRoutes {
                app_state.routes.clone()
            }
        }

        impl AppState {
            pub fn new(routes: Vec<RouteListing>, leptos_options: LeptosOptions) -> Self {
                AppState {
                    leptos_options,
                    routes: AppRoutes(routes),
                    cache: AppCache(Arc::new(Mutex::new(HashMap::new()))),
                }
            }
        }
    }
}
