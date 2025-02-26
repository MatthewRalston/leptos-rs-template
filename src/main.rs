//use leptos_template::todo::*; // Working todo example
use std::net::SocketAddr;

use leptos_template::example::*;
use axum::{
    body::Body,
    extract::Path,
    http::Request,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};


use serde::{Deserialize, Serialize};
use time::Duration;

use tokio::net::TcpListener;
use tokio::{signal, task::AbortHandle};

use tower_sessions::{session_store::ExpiredDeletion, Expiry, Session, SessionManagerLayer};
use tower_sessions_sqlx_store::{sqlx::SqlitePool, SqliteStore};



//use dotenvy::dotenv;
//use std::env;


use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
//use leptos_template::*;


//mod auth;
//mod routes;


// From tower-sessions-sqlx-store
const COUNTER_KEY: &str = "counter";

#[derive(Serialize, Deserialize, Default)]
struct Counter(usize);




// ORIGINAL
//Define a handler to test extractor with state ORIGINAL
async fn custom_handler(
    Path(id): Path<String>,
    req: Request<Body>,
) -> Response {

    // Requires session: Session in params list
    //let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
    //session.insert(COUNTER_KEY, counter.0 + 1).await.unwrap();

    
    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            provide_context(id.clone());
	    //provide_context(counter.0.clone());
        },
	ExampleApp,
        //TodoApp,
    );
    handler(req).await.into_response()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //dotenv().ok();

    // For PostgreSQL session store, use the following
    // https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/examples/postgres.rs
    // SQLite Pool session store
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    let session_store = SqliteStore::new(pool);
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));




    // Use example::ExampleApp and configs in example.rs
    use leptos_template::example::ssr::db;

    // simple_logger::init_with_level(log::Level::Error)
    //     .expect("couldn't initialize logging");

    let mut conn = db().await.expect("couldn't connect to DB");
    if let Err(e) = sqlx::migrate!().run(&mut conn).await {
        eprintln!("{e:?}");
    }

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    //let routes = generate_route_list(TodoApp); // Working for TodoApp
    let routes = generate_route_list(ExampleApp);

    // build our application with a route
    // TODO: FIXME!
    let app = Router::new()
        .route("/special/:id", get(custom_handler))
	.layer(session_layer)
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options.clone());



    // Final leptos configuration and router setup.
    // addr from leptos_options set in Cargo.toml
    //let app = routes::create_router();
    let addr = leptos_options.site_addr.to_string();
    //let addr = env::var("SERVER_ADDRESS").unwrap_or_else(|_| leptos_options.site_addr.to_string());
    let listener = TcpListener::bind(&addr).await.unwrap();
    
    println!("listening on http://{}", &addr);
    
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`


    axum::serve(listener, app.into_make_service())
	.with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await
        .unwrap();


    deletion_task.await??; // Delete stale sessions from sqlx::sqlite
    
    Ok(())
}



async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
