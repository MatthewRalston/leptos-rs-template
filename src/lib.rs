// use leptos::prelude::*;
// use leptos_meta::*;
// use leptos_router::{components::*, path};


//use leptos_template::*;

//use self::pages::home::Home;
// Modules
pub mod todo;
pub mod example;
pub mod errors;
pub mod error_template;


// #[cfg(feature = "hydrate")]
// #[wasm_bindgen::prelude::wasm_bindgen]
// pub fn hydrate() {
//     use crate::todo::TodoApp;
//     console_error_panic_hook::set_once();
//     leptos::mount::hydrate_body(TodoApp);
// }


// /// An app router which renders the homepage and handles 404's
// #[component]
// pub fn App() -> impl IntoView {
//     // Provides context that manages stylesheets, titles, meta tags, etc.
//     provide_meta_context();

//     view! {
//         <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

//         // sets the document title
//         <Title text="Welcome to Leptos CSR" />

//         // injects metadata in the <head> of the page
//         <Meta charset="UTF-8" />
//         <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

//         <Router>
//             <Routes fallback=|| view! { NotFound }>
// 	        // TODO : make all routes
//                 <Route path=path!("/") view=Home />
//             </Routes>
//         </Router>
//     }
// }
