use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::components::Badge;
use crate::signals::debounce_signal;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug, Serialize, PartialEq, Hash, Eq, Copy)]
pub enum Sources {
    PackageNpm,
    OrgNpm,
    Github,
}

pub async fn page_exists(url: String) -> Result<bool, ServerFnError> {
    let result = reqwest::get(url).await;
    match result {
        Ok(result) => {
            if http::StatusCode::NOT_FOUND == result.status() {
                return Ok(true);
            }
            return Ok(false);
        }
        Err(_) => Ok(false),
    }
}

#[server(CheckAvailable, "/api")]
pub async fn check_available(source: Sources, title: String) -> Result<bool, ServerFnError> {
    use crate::state::*;

    async fn cached_page_exists(url: String) -> Result<bool, ServerFnError> {
        let cache = use_context::<AppCache>().expect("Missing context provider");
        if let Some(value) = cache.get_exists(url.clone()) {
            return Ok(value);
        }
        let result = page_exists(url.clone()).await;
        cache.set_exists(url);
        result
    }
    match source {
        Sources::PackageNpm => {
            cached_page_exists(format!("https://www.npmjs.com/package/{}", title)).await
        }
        Sources::OrgNpm => cached_page_exists(format!("https://www.npmjs.com/org/{}", title)).await,
        Sources::Github => cached_page_exists(format!("https://github.com/{}", title)).await,
    }
}

/// Renders the npm package availability
#[component]
pub fn Available(source: Sources, query: Memo<String>) -> impl IntoView {
    let debounced = debounce_signal(std::time::Duration::from_millis(300), query);
    let cache: Rc<RefCell<HashMap<String, bool>>> = Rc::new(RefCell::new(HashMap::new()));

    let once = create_resource(debounced, {
        move |query| {
            let cache = cache.clone();
            async move {
                let key = || format!("{:?}-{}", source, query);
                {
                    let cache = cache.borrow();
                    if let Some((_, v)) = cache.get_key_value(&key()) {
                        return Ok(v.clone());
                    }
                }
                let result = check_available(source, query.clone()).await;
                let mut cache = cache.borrow_mut();
                if let Ok(v) = result {
                    cache.insert(key(), v);
                    return Ok(v);
                }
                result
            }
        }
    });

    let available = move || match once.get() {
        Some(Ok(v)) => v,
        _ => false,
    };

    let match_source = move || match source {
        Sources::PackageNpm => (
            "icon-[devicon--npm-wordmark]".to_string(),
            "package".to_string(),
        ),
        Sources::OrgNpm => (
            "icon-[devicon--npm-wordmark]".to_string(),
            "org".to_string(),
        ),
        Sources::Github => ("icon-[devicon--github]".to_string(), "".to_string()),
    };

    view! {
        <Suspense fallback=move || view! { <Badge
            icon={match_source().0}
            label={match_source().1}
            loading=once.loading()
            available=move || false
        /> }>
            <Badge
                icon={match_source().0}
                label={match_source().1}
                loading=once.loading()
                available=available
            />
        </Suspense>
    }
}
