use crate::components::{AppError, ErrorTemplate, QueryInput};
use leptos::*;
use leptos_meta::*;
use leptos_query::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_query_client();

    view! {
        <Stylesheet id="leptos" href="/pkg/nameit.css"/>
        <Title text="Name it"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=Landing/>
                </Routes>
            </main>
        </Router>
    }
}

/// Landing page
#[component]
fn Landing() -> impl IntoView {
    view! {
        <div class="w-full h-screen flex items-center justify-center">
           <QueryInput />
        </div>
    }
}
