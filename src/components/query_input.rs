use crate::components::{Available, Sources};
use leptos::*;

/// Renders search input field
#[component]
pub fn QueryInput(
    query_value: Memo<Option<String>>,
    set_query_value: SignalSetter<Option<String>>,
) -> impl IntoView {
    use leptos::ev::Event;
    use leptos::html::Input;

    let input_ref = create_node_ref::<Input>();
    let handle_input = move |ev: Event| set_query_value(Some(event_target_value(&ev)));
    let query = create_memo(move |_| query_value().unwrap_or("".to_string()));

    create_effect(move |_| {
        let node = input_ref.get().expect("Input should be loaded");
        let _ = node.focus();
    });

    view! {
        <div class="w-full sticky top-0 bg-white px-6 pt-6 pb-3">
            <label for="search" class="text-sm pl-3 pb-1">
                "Search availability of project name"
            </label>
            <input
                _ref=input_ref
                role="search"
                type="text"
                id="search"
                class="px-3 py-1 border w-full"
                placeholder="Search name"
                value=query
                on:input=handle_input
            />
            <div class="flex gap-3 my-1">
                <Show when=move || { query().len() > 0 }>
                    <Available source=Sources::PackageNpm query=query/>
                    <Available source=Sources::OrgNpm query=query/>
                    <Available source=Sources::Github query=query/>
                </Show>
            </div>
        </div>
    }
}
