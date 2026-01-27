use serde::{Deserialize, Serialize};

#[cfg(feature = "hydrate")]
use chrono::Local;
#[cfg(feature = "hydrate")]
use chrono::NaiveDateTime;
#[cfg(feature = "ssr")]
use sqlx::types::chrono::Local;
#[cfg(feature = "ssr")]
use sqlx::types::chrono::NaiveDateTime;
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(Serialize, Deserialize, Debug, Clone, FromRow))]
#[cfg_attr(feature = "hydrate", derive(Serialize, Deserialize, Debug, Clone))]
pub struct Post {
    pub id: String,
    pub dt: NaiveDateTime,
    pub image_url: String,
    pub title: String,
    pub text: String,
}

impl Post {
    pub fn new(id: String, image_url: String, title: String, text: String) -> Self {
        Self {
            id,
            dt: Local::now().naive_local(),
            image_url,
            title,
            text,
        }
    }
    pub fn new_empty() -> Self {
        Self {
            id: "".to_string(),
            dt: Local::now().naive_local(),
            image_url: "".to_string(),
            title: "".to_string(),
            text: "".to_string(),
        }
    }
}
