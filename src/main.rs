use leptos_template::todo::*;
use axum::{
    body::Body,
    extract::Path,
    http::Request,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};


use tokio::net::TcpListener;
//use dotenvy::dotenv;
use std::env;


use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
//use leptos_template::*;


//mod auth;
//mod routes;


//Define a handler to test extractor with state
async fn custom_handler(
    Path(id): Path<String>,
    req: Request<Body>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            provide_context(id.clone());
        },
        TodoApp,
    );
    handler(req).await.into_response()
}

#[tokio::main]
async fn main() {

    //dotenv().ok();

    
    use leptos_template::todo::ssr::db;

    // simple_logger::init_with_level(log::Level::Error)
    //     .expect("couldn't initialize logging");

    let mut conn = db().await.expect("couldn't connect to DB");
    if let Err(e) = sqlx::migrate!().run(&mut conn).await {
        eprintln!("{e:?}");
    }

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(TodoApp);

    // build our application with a route
    let app = Router::new()
        .route("/special/:id", get(custom_handler))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options.clone());


    //let app = routes::create_router();
    let addr = leptos_options.site_addr.to_string();
    //let addr = env::var("SERVER_ADDRESS").unwrap_or_else(|_| leptos_options.site_addr.to_string());
    let listener = TcpListener::bind(&addr).await.unwrap();
    
    println!("listening on http://{}", &addr);
    
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`


    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
