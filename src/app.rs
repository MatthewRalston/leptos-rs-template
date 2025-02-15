use leptos::*;

#[component]
pub fn LoginButton() -> impl IntoView {
    view! {
        <a href="/auth/login">"Login with OAuth2"</a>
    }
}
