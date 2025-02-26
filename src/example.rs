use crate::error_template::ErrorTemplate;
use leptos::either::Either;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

use chrono::prelude::*;
//use chrono::{DateTime, Utc};


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
	<!DOCTYPE html>
	    <html lang="en">
	    <head>
	    <meta charset="utf-8"/>
	    <meta name="viewport" content="width=device-width, initial-scale=1"/>
	    <AutoReload options=options.clone() />
	    <HydrationScripts options/>
	    <link rel="stylesheet" id="leptos" href="/pkg/todo_app_sqlite_axum.css" />
	    <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
	    </head>
	    <body>
	    <ExampleApp/>
	    </body>
	    </html>

    }
}



#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Model {
    id: u16,
    key: String,
    created_date: DateTime<Utc>,
    updated_date: DateTime<Utc>,
    value: String,
}

// For a PostgreSQL model storage use the following
// https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/examples/postgres.rs
#[cfg(feature = "ssr")]
pub mod ssr {
    // use http://{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use leptos::server_fn::ServerFnError;
    use sqlx::{Connection, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
	Ok(SqliteConnection::connect("sqlite:Models.db").await?)
    }

}

#[server]
pub async fn get_models() -> Result<Vec<Model>, ServerFnError> {
    use self::ssr::*;
    use http::request::Parts;

    let req_parts = use_context::<Parts>();

    if let Some(req_parts) = req_parts {
	println!("Uri = {:?}", req_parts.uri);
    }

    use futures::TryStreamExt;

    let mut conn = db().await?;

    let mut models = Vec::new();

    // Make sure to replace 'models' with the plural of your model name
    let mut rows =
	sqlx::query_as::<_, Model>("SELECT * FROM models").fetch(&mut conn);

    while let Some(row) = rows.try_next().await? {
	models.push(row);
    }

    
    // Lines below show how to set status code and headers on the response
    // let resp = expect_context::<ResponseOptions>();
    // resp.set_status(StatusCode::IM_A_TEAPOT);
    // resp.insert_header(SET_COOKIE, HeaderValue::from_str("fizz=buzz").unwrap());

    Ok(models)

}

#[server]
pub async fn add_model(key: String, value: String) -> Result<(), ServerFnError> {

    use self::ssr::*;
    use chrono::prelude::*;

    let utc_now: DateTime<Utc> = Utc::now();
    //let utc_now_string: String = utc_now.naive_utc().to_string();
    let mut conn = db().await?;


    match sqlx::query("INSERT INTO models (key, value, created_date, updated_date) VALUES ($1, $2, $3, $3)")
	.bind(key)
	.bind(value)
	.bind(utc_now)
	.bind(utc_now)
	.execute(&mut conn)
	.await
    {
	Ok(_row) => Ok(()),
	Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}


#[server]
pub async fn delete_model(id: u16) -> Result<(), ServerFnError> {
    use self::ssr::*;
    let mut conn = db().await?;


    Ok(sqlx::query("DELETE FROM models WHERE id = $1")
       .bind(id)
       .execute(&mut conn)
       .await
       .map(|_| ())?)

}


#[component]
pub fn ExampleApp() -> impl IntoView {
    view! {
	<header>
	    <h1>"My Models"</h1>
	    </header>
	    <main>
	    <Models/>
	    </main>
    }
}


#[component]
pub fn Models() -> impl IntoView {
    let add_model = ServerMultiAction::<AddModel>::new();
    let submissions = add_model.submissions();
    let delete_model = ServerAction::<DeleteModel>::new();


    let models = Resource::new(
	move || {
	    (
		delete_model.version().get(),
		add_model.version().get(),
		delete_model.version().get(),
	    )
	},
	move |_| get_models(),
    );



    let existing_models = move || {
	Suspend::new(async move {
	    models.await
		.map(|models| {
		    if models.is_empty() {
			Either::Left(view! { <p>"No models were found."</p> })
		    } else {
			Either::Right(
			    models.iter()
				.map(move |model| {
				    let id = model.id;
				    view! {
					<li>
					    <p><strong>"Key"</strong> : {model.key.clone()} </p>
					    <p><strong>"Value"</strong> : {model.value.clone()} </p>
					<ActionForm action=delete_model>
					    <input type="hidden" name="id" value=id/>
					    <input type="submit" value="Delete"/>
					    </ActionForm>
					    </li>
				    }
				})
				.collect::<Vec<_>>(),
			    )
		    }
		})
	})
    };


    view! {
	<MultiActionForm action=add_model>
	    <label>"Add a model: key" <input type="text" name="key"/></label>
	    <label>"Add a model: value" <input type="text" name="value"/></label>
	    <input type="submit" value="Add"/>
	    </MultiActionForm>
	    <div>
	    <Transition fallback=move || view! { <p>"Loading..."</p> }>
	    <ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors/> } >
	    <ul>
	{existing_models}
	{move || {
	    submissions.get()
		.into_iter()
		.filter(|submission| submission.pending().get())
		.map(|submission| {
		    view! {
			<li class="pending">
			{move || submission.input().get().map(|data| view! { <p>data.key | data.value</p> } )}
			</li>
		    }
		})
		.collect::<Vec<_>>()
	}}

	    </ul>
	    </ErrorBoundary>
	    </Transition>
	    </div>
    }
}
