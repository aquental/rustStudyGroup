use crate::component::blog_post::BlogPost;
use crate::model::blog_post::Post;
use crate::repository::blog_repository::get_post;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Params, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
struct ViewPostParams {
    pub post_id: Option<String>,
}

#[component]
pub fn ViewPost() -> impl IntoView {
    let params = Memo<Result<_,_>> = use_params::<ViewPostParams>();

    let post_resource = create_resource(
        move || params.get(),
        |params| async move {
            match params {
            Ok(ViewPostParams { post_id: Some(s) }) => get_post(s).await,
            _ => Ok(Post::new_empty()),
            }
        },
    );

    let res: Option<Result<Post>> = post_resource.read();

    let post_view = move || {
        post_resource.and_then(|post| {
            let post_saved = post.clone();
            view! {
                <div class="w-full flex justify-center">
                    <div class="max-w-[800]">
                        <div class="flex justify-center pt-10">
                            <a href={format!("/edit/{}", &post.id)}>Edit</a>
                        </div>
                        <BlogPost post={post_saved} />
                    </div>
                </div>
            }
        })
    };

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=move |err| view! { <p>"Error: {err}"</p> }>
                {post_view}
            </ErrorBoundary>
        </Suspense>
    }
}
