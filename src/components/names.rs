use leptos::*;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::state::{AppCache, CacheData};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Flag {
    title: String,
    link: String,
    src: String,
}

pub fn scrape_flag(flag: scraper::ElementRef<'_>) -> Option<Flag> {
    let title = flag
        .select(&scraper::Selector::parse("a").unwrap())
        .next()
        .and_then(|a| a.value().attr("title"))?;
    let src = flag
        .select(&scraper::Selector::parse("img").unwrap())
        .next()
        .and_then(|a| a.value().attr("src"))?;
    let link = flag
        .select(&scraper::Selector::parse("a").unwrap())
        .next()
        .and_then(|a| a.value().attr("href"))?;

    Some(Flag {
        title: title.to_string().clone(),
        src: src.to_string().clone(),
        link: link.to_string().clone(),
    })
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entry {
    link: String,
    title: String,
    flags: Vec<Flag>,
}

pub fn scrape_entry(item: scraper::ElementRef<'_>) -> Option<Entry> {
    let title: String = item
        .select(&scraper::Selector::parse("a").unwrap())
        .next()
        .and_then(|a| Some(a.text().collect::<String>().trim().to_owned()))?;
    let link = item
        .select(&scraper::Selector::parse("a").unwrap())
        .next()
        .and_then(|a| a.value().attr("href"))
        .and_then(|f| Some(format!("{}{}", "https://en.wikipedia.org", f)))?;

    let mut flags = vec![];

    for flag in item.select(&scraper::Selector::parse(".flagicon").unwrap()) {
        match scrape_flag(flag) {
            Some(flag) => flags.push(flag),
            _ => {}
        }
    }

    let mut unique_set: HashSet<_> = HashSet::new();

    // Filter the vector for unique elements based on the 'field' property
    let flags: Vec<_> = flags
        .into_iter()
        .filter(move |item| unique_set.insert(item.src.clone()))
        .collect();

    if flags.len() == 0 {
        None
    } else {
        Some(Entry { title, link, flags })
    }
}

pub fn scrape_entries(text: &String) -> Option<Vec<Entry>> {
    let document = Html::parse_document(&text);
    let select_items = scraper::Selector::parse(".mw-parser-output ul > li").unwrap();
    let items = document.select(&select_items);
    let mut entries = vec![];
    for item in items {
        match scrape_entry(item) {
            Some(entry) => entries.push(entry),
            _ => {}
        }
    }
    return Some(entries);
}

// #[server(WW2Names, "/api")]
pub async fn ww2_names() -> Result<Vec<Entry>, ServerFnError> {
    let url = "https://en.wikipedia.org/wiki/List_of_World_War_II_military_operations";
    let cache = use_context::<AppCache>().expect("Missing context provider");
    if let Some(CacheData::Html(str)) = cache.get_value(url.to_string().clone()) {
        return Ok(scrape_entries(&str).unwrap_or(vec![]));
    }
    let result = reqwest::get(url).await?;
    let text = result.text().await?;
    cache.set_value(
        url.to_string().clone(),
        CacheData::Html(text.to_string().clone()),
    );
    Ok(scrape_entries(&text).unwrap_or(vec![]))
}

#[component]
pub fn Names(set_query_value: SignalSetter<Option<String>>) -> impl IntoView {
    let value = create_resource(|| (), |_| ww2_names());
    let value = move || match value.get() {
        Some(Ok(v)) => v,
        _ => vec![],
    };

    view! {
        <Suspense fallback=|| "Loading">
            <div class="w-full grid grid-cols-4 border-t">
                <For
                    each=move || value()
                    key=move |x| x.link.clone()
                    children=move |x| {
                        let title = x.title.clone();
                        view! {
                            <div class="flex items-start gap-1 border-b px-6 py-2 uppercase bg-white">
                                <button
                                    target="__blank"
                                    class="leading-none"
                                    on:click=move |_| set_query_value(Some(title.clone()))
                                >
                                    {x.title.clone()}
                                </button>
                                <For
                                    each=move || x.flags.clone().into_iter()
                                    key=move |x| x.link.clone()
                                    children=move |x| {
                                        view! {
                                            <div
                                                class="aspect-[2/1] w-[1.5rem] mt-[2px] bg-cover flex-shrink-0"
                                                style:background-image=format!("url('{}')", x.src)
                                            >// <img class="w-6" src=x.src />
                                            </div>
                                        }
                                            .into_view()
                                    }
                                />

                            </div>
                        }
                    }
                />

            </div>
        </Suspense>
    }
}
