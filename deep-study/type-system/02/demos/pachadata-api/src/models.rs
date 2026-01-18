use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use utoipa::{ToSchema};
use std::string::ToString;
use strum_macros::Display;
use std::fmt::{Debug, Display, Formatter, Result};
use std::cmp::Ordering;


// ---------------------------------------------------
//                     Traits
// ---------------------------------------------------
// #[allow(dead_code)]
pub trait Teachable: Send + Sync {
    fn title(&self) -> &str;
    fn instructor(&self) -> &str;
    fn price(&self) -> f64;
    fn teach_mode(&self) -> String;
}

pub trait Discountable {
    fn discount_price(&self, percent: f64) -> f64;
}

pub trait PremiumCourse {}

#[allow(dead_code)]
pub trait Certifiable {
    fn issue_certificate(&self, learner_id: u32) -> String;
}

#[allow(dead_code)]
pub trait Downloadable {
    fn materials(&self) -> Vec<String>;
}

#[allow(dead_code)]
pub trait Interactive {
    fn schedule_session(&self, date: &str);
}

// Trait composition using supertraits
// pub trait EngagingCourse: Teachable + Interactive + Certifiable {}

#![feature(trait_alias)]
pub trait EngagingCourse = Teachable + Interactive + Downloadable;


#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

#[derive(Display, Debug, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum CourseType {
    #[serde(rename = "online")]
    #[strum(serialize = "online")]
    #[sqlx(rename = "online")]
    Online,
    #[serde(rename = "workshop")]
    #[strum(serialize = "workshop")]
    #[sqlx(rename = "workshop")]
    Workshop,
    #[serde(rename = "webinar")]
    #[strum(serialize = "webinar")]
    #[sqlx(rename = "webinar")]
    Webinar,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(tag = "course_type", rename_all = "lowercase")]
pub enum NewCourse {
    Online {
        title: String,
        instructor: String,
        description: Option<String>,
        price: f64,
        url: String,
    },
    Workshop {
        title: String,
        instructor: String,
        description: Option<String>,
        price: f64,
        location: String,
    },
    Webinar {
        title: String,
        instructor: String,
        description: Option<String>,
        price: f64,
        video_link: String,
    },
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub instructor: String,
    pub description: Option<String>,
    pub price: f64,
    pub course_type: CourseType, // Added field for course type
    pub url: Option<String>,
    pub location: Option<String>,
    pub video_link: Option<String>,
}

// Specific implementations for different course types

// ---------------------------------------------------
//                OnlineCourse
// ---------------------------------------------------
#[derive(Debug, Clone)]
pub struct OnlineCourse {
    pub title: String,
    pub instructor: String,
    pub url: String,
    pub price: f64,
}

impl Display for OnlineCourse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} by {}", self.title, self.instructor)
    }
}

impl PartialEq for OnlineCourse {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl PartialOrd for OnlineCourse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl Teachable for OnlineCourse {
    fn title(&self) -> &str {
        &self.title
    }
    fn instructor(&self) -> &str {
        &self.instructor
    }
    fn price(&self) -> f64 {
        self.price
    }
    fn teach_mode(&self) -> String {
        format!("Online via {}", self.url)
    }
}

impl Discountable for OnlineCourse {
    fn discount_price(&self, percent: f64) -> f64 {
        self.price * (100.0 - percent) / 100.0
    }
}

impl Certifiable for OnlineCourse { 
    fn issue_certificate(&self, learner_id: u32) -> String {
        format!(
            "Certificate of Completion\nCourse: {}\nInstructor: {}\nLearner ID: {}\nMode: Online\nURL: {}",
            self.title, self.instructor, learner_id, self.url
        )
    }
 }

impl Interactive for OnlineCourse { 
    fn schedule_session(&self, date: &str) {
        println!(
            "Scheduled an interactive online session for '{}' with instructor '{}' on {} via {}.",
            self.title, self.instructor, date, self.url
        );
    }
}

// no extra work:
impl EngagingCourse for OnlineCourse {}

// ---------------------------------------------------
//                InPersonWorkshop
// ---------------------------------------------------
#[derive(Debug, Clone)]
pub struct InPersonWorkshop {
    pub title: String,
    pub instructor: String,
    pub location: String,
    pub price: f64,
}

impl Teachable for InPersonWorkshop {
    fn title(&self) -> &str {
        &self.title
    }
    fn instructor(&self) -> &str {
        &self.instructor
    }
    fn price(&self) -> f64 {
        self.price
    }
    fn teach_mode(&self) -> String {
        format!("In-person at {}", self.location)
    }
}

impl PremiumCourse for InPersonWorkshop {}

// ---------------------------------------------------
//                RecordedWebinar
// ---------------------------------------------------
#[derive(Debug, Clone)]
pub struct RecordedWebinar {
    pub title: String,
    pub instructor: String,
    pub video_link: String,
    pub price: f64,
}

impl Teachable for RecordedWebinar {
    fn title(&self) -> &str {
        &self.title
    }
    fn instructor(&self) -> &str {
        &self.instructor
    }
    fn price(&self) -> f64 {
        self.price
    }
    fn teach_mode(&self) -> String {
        format!("On-demand: {}", self.video_link)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCourse {
    pub title: Option<String>,
    pub instructor: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub url: Option<String>,
    pub location: Option<String>,
    pub video_link: Option<String>,
}

#[allow(dead_code)]
impl Course {
    pub fn from_teachable<T: Teachable>(
        id: String,
        teachable: &T,
        description: Option<String>,
        course_type: CourseType,
        url: Option<String>,
        location: Option<String>,
        video_link: Option<String>,
    ) -> Self {
        Self {
            id,
            title: teachable.title().to_string(),
            instructor: teachable.instructor().to_string(),
            description,
            price: teachable.price(),
            course_type,
            url,
            location,
            video_link,
        }
    }
}

// ---------------------------------------------------
//                TeachableOut
// ---------------------------------------------------
#[derive(Serialize, utoipa::ToSchema)]
pub struct TeachableOut {
    pub title: String,
    pub instructor: String,
    pub mode: String,
    pub price: f64,
}

impl Teachable for TeachableOut {
    fn title(&self) -> &str {
        &self.title
    }
    fn instructor(&self) -> &str {
        &self.instructor
    }
    fn price(&self) -> f64 {
        self.price
    }
    fn teach_mode(&self) -> String {
        format!("teach mode is {}", self.mode)
    }
}
