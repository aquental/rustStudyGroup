use leptos::*;
use chrono::Local;
use crate::model::blog_post::Post;
use leptos_server::ServerFnError;

#[server(UpsertPost, "/api")]
pub async fn upsert_post(
id: Option<String>,
dt: String,
image_url: String,
title: String,
text: String,
) -> Result<String, ServerFnError> {
    // Define fields here
    Ok(String::from("Post upserted successfully"))
}


#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    Ok(Post {
        id: "1".to_string(),
        dt: Local::now().naive_local(),
        image_url: String::from("https://bit.ly/3t0bA61"),
        title: String::from("Ocean"),
        text: String::from("This is a sample blog post."),
    })  
}

#[server(GetPreviews, "/api")]
pub async fn get_previews(
oldest: Option<String>,
newest: Option<String>,
preview_length: u8,
page_size: u8) -> Result<Vec<Post>, ServerFnError> {
    Ok(vec![
        Post {
            id: "1".to_string(),
            dt: Local::now().naive_local(),
            image_url: String::from("https://bit.ly/3t0bA61"),
            title: String::from("Ocean"),
            text: String::from("This is a sample blog post."),
        },
        Post {
            id: "2".to_string(),
            dt: Local::now().naive_local(),
            image_url: String::from("https://bit.ly/3t0bA61"),
            title: String::from("Forest"),
            text: String::from("This is another sample blog post about forests."),
        },
        Post {
            id: "3".to_string(),
            dt: Local::now().naive_local(),
            image_url: String::from("https://bit.ly/3t0bA61"),
            title: String::from("Desert"),
            text: String::from("This is another sample blog post about deserts."),
        }
    ])
}
