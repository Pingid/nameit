use cfg_if::cfg_if;

// boilerplate to run in different modes
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{
            body::Body as AxumBody,
            extract::{Path, RawQuery, State},
            http::{header::HeaderMap, Request},
            response::{IntoResponse, Response},
            routing::{get, post},
            Router,
        };
        // use leptos::*;
        use leptos::{provide_context, get_configuration, LeptosOptions};
        use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
        use nameit::app::*;
        use nameit::state::*;
        use nameit::fileserv::file_and_error_handler;

        async fn server_fn_handler(
            State(cache): State<AppCache>,
            path: Path<String>,
            headers: HeaderMap,
            raw_query: RawQuery,
            request: Request<AxumBody>,
        ) -> impl IntoResponse {
            log::debug!("{:?}", path);
            handle_server_fns_with_context(
                path,
                headers,
                raw_query,
                move || {
                    provide_context(cache.clone());
                },
                request,
            )
            .await
        }

        async fn leptos_routes_handler(State(cache): State<AppCache>, State(routes): State<AppRoutes>, State(leptos_options): State<LeptosOptions>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_route_with_context(leptos_options,
            routes.0,
            move || {
                provide_context(cache.clone());
            },
            App
        );
        handler(req).await.into_response()
    }


        #[tokio::main]
        async fn main() {
            simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");


            // Setting get_configuration(None) means we'll be using cargo-leptos's env values
            // For deployment these variables are:
            // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
            // Alternately a file can be specified such as Some("Cargo.toml")
            // The file would need to be included with the executable when moved to deployment
            let conf = get_configuration(None).await.unwrap();
            let routes = generate_route_list(App);
            let addr = conf.leptos_options.site_addr.clone();
            let state = AppState::new(routes.clone(), conf.leptos_options);

            // build our application with a route
            let app = Router::new()
                // .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
                // .leptos_routes(&leptos_options, routes, App)
                // .fallback(file_and_error_handler)
                // .with_state(leptos_options);
                .route("/api/*fn_name", post(server_fn_handler))
                .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
                .fallback(file_and_error_handler)
                .with_state(state);

            // run our app with hyper
            // `axum::Server` is a re-export of `hyper::Server`
            log::info!("listening on http://{}", &addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }
    else {
        pub fn main() {
            // This example cannot be built as a trunk standalone CSR-only app.
            // Only the server may directly connect to the database.
        }
    }
}
