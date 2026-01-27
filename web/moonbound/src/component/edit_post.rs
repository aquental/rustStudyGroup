use crate::model::blog_post::BlogPost;
use crate::model::blog_post::Post;
use chrono::NaiveDateTime;
use crate::repository::blog_repository::get_post;
use leptos::*;
use leptos_router::use_params;
use serde::{Deserialize, Serialize};

#[derive(Params, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
struct EditPostParams {
    pub post_id: Option<String>,
}

fn format_dt(datetime: NaiveDateTime) -> String {
    datetime.format("%Y-%m-%dT%H:%M").to_string()
}

#[component]
pub fn EditPost() -> impl IntoView {
    let params = use_params::<EditPostParams>();

    let post_resource = create_resource(
        move || params.get(),
        |params| async move {
            match params {
            Ok(EditPostParams { post_id: Some(s) }) => get_post(s).await,
            _ => Ok(Post::new_empty()),
            }
        },
    );

    let res: Option<Result<Post>> = post_resource.read();

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=move |err| view! { <p>"Error: {err}"</p> }>
                <div class="flex h-screen">
                    <div class="min-w-[50%] max-h-[90%] text-gray-200 dark:bg-gray-800 bg-gray-100 p-10 rounded-md">
                        <form>
                            <input type="hidden" name="id" value={move || post_resource.get().and_then(|res| res.as_ref().map(|post| post.id.clone())) }/>
                            <label class="block mb-4">
                            <span>Date</span>
                            <input class="mt-1 p-2 w-full" type="datetime-local" id="dt" name="dt"
                                value={move || post_resource.get().and_then(|res| res.as_ref().map(|post| format_dt(post.dt))) }
                                on:input=move |ev| {
                                    let dt: String = event_target_value(&ev);
                                    let chrono_dt = NaiveDateTime::parse_from_str(&dt, "%Y-%m-%dT%H:%M");
                                    let utc_dt = match chrono_dt {
                                        Ok(dt) => dt,
                                        Err(_) => NaiveDateTime::from_timestamp(0, 0),
                                    };
                                    set_post.update(|post| {
                                        post.dt = utc_dt;
                                    });
                                }/>
                            </label>
                            <label class="block mb-4">
                            <span>Title</span>
                            <input class="mt-1 p-2 w-full" type="text" id="title" name="title" placeholder="Post Title"
                                on:input=move |ev| {
                                    post_resource.update(|curr| {
                                        if let Some(Ok(post)) = curr {
                                            post.title = event_target_value(&ev);
                                        }
                                    });
                                }
                                prop:value={move || post_resource.get().and_then(|res| res.as_ref().map(|post| post.title).ok()) }/>
                            />
                            </label>
                            <label class="block mb-4">
                            <span>Entry</span>
                            <textarea class="mt-1 p-2 w-full" id="text" name="text" placeholder="Post Entry"
                                on:input=move |ev| {
                                    post_resource.update(|curr| {
                                        if let Some(Ok(post)) = curr {
                                            post.text = event_target_value(&ev);
                                        }
                                    });
                                }
                                prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.text).ok())
                            }
                            />
                            </label>
                            <div class="flex justify-center pb-4">
                                <input type="submit" value="Submit" class="mx-auto w-1/3 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"/>
                            </div>
                        </form>
                    </div>
                    //right side preview
                    <div>
                        {move || post_resource.and_then(|post| view! {<BlogPost post=post.clone()/>})}
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }


        

}
