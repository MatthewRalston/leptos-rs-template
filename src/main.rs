use crate::example::*


// Axum 'use' statements
use axum::{
    body::Body,
    extract::Path,
    routing::{get, post},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
// Leptos and leptos-axum
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
// Project-specific
use leptos_axum_template::App;

use serde::{Deserialize, Serialize};


// Entry point to Tokio application
// Uses hyper::Server via axum in a tokio asynchronous context.
// run our app with hyper
// `axum::Server` is a re-export of `hyper::Server`
#[tokio::main]
async fn main() {

    // Imports the database  server-side rendering (SSR) configuration for the 'todo' app'
    use crate::example::ssr::db;

    // FIXME: Rrequires logging configuration as a Toml file probably
    simple_logger::init_with_level(log::Level::Error)
        .expect("couldn't initialize logging");


    // Uses the db method of 
    let mut conn = db().await.expect("couldn't connect to DB");
    if let Err(e) = sqlx::migrate!().run(&mut conn).await {
        eprintln!("{e:?}");
    }

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(ExampleApp);

    // build our application with a route
    let app = Router::new()
	.route("/", get(root)) // 'GET /' goes to 'root'
	.route("/model", get(create_model)); // 'POST /model' goes to 'create_model'

        // .route("/special/:id", get(custom_handler))
        // .leptos_routes(&leptos_options, routes, {
        //     let leptos_options = leptos_options.clone();
        //     move || shell(leptos_options.clone())
        // })
        // .fallback(leptos_axum::file_and_error_handler(shell))
        // .with_state(leptos_options);


    // 
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}



// // Old main function from examples/tailwind_axum/src/main.rs
// fn main() {
//     // set up logging
//     _ = console_log::init_with_level(log::Level::Debug);
//     console_error_panic_hook::set_once();

//     mount_to_body(|| {
//         view! {
//             <App />
//         }
//     })
// }

// Custom-handler function from leptos-rs/leptos:examples/todo_sqlite_axum/src/main.rs
//Define a handler to test extractor with state
async fn custom_handler(
    Path(id): Path<String>,
    req: Request<Body>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            provide_context(id.clone());
        },
        ExampleApp,
    );
    handler(req).await.into_response()
}

