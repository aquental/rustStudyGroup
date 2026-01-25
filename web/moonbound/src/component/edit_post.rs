use crate::model::blog_post::Post;
use leptos::*;
use leptos_router::use_params;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct EditPostParams {
    pub post_id: Option<String>,
}
#[component]
pub fn EditPost() -> impl IntoView {
    let params = use_params::<EditPostParams>();
    let display_params = move || match params.get() {
        Some(p) => format!("Editing post with ID: {:?}", p.post_id),
        None => "Creating a new post".to_string(),
    };
    view! {
    {display_params}
    }
}
