use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use crate::{
    errors::ApiError,
    models::{AppState, Course, CourseType, UpdateCourse, NewCourse},
    models::{OnlineCourse, InPersonWorkshop, RecordedWebinar, Teachable, TeachableOut},
    models::{UpdateCourseTs, Unchecked, Checked},
    utils::*
};
// use std::collections::HashMap;
use utoipa::IntoParams;
use serde::Deserialize;
use sqlx::QueryBuilder;

#[utoipa::path(
    post,
    path = "/courses",
    request_body = NewCourse,
    responses(
        (status = 201, description = "Course created", body = Course),
        (status = 400, description = "Invalid data")
    )
)]
pub async fn create_course(
    State(state): State<AppState>,
    Json(new_course): Json<NewCourse>,
) -> Result<(StatusCode, Json<Course>), ApiError> {
    // Unpack common fields
    let (course_type, title, instructor, description, price, url, location, video_link) =
        match new_course {
            NewCourse::Online { title, instructor, description, price, url } => (
                CourseType::Online, title, instructor, description, price,
                Some(url), None, None,
            ),
            NewCourse::Workshop { title, instructor, description, price, location } => (
                CourseType::Workshop, title, instructor, description, price,
                None, Some(location), None,
            ),
            NewCourse::Webinar { title, instructor, description, price, video_link } => (
                CourseType::Webinar, title, instructor, description, price,
                None, None, Some(video_link),
            ),
        };

    let course = sqlx::query_as::<_, Course>(
        "INSERT INTO courses (id, title, instructor, description, price, course_type, url, location, video_link)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING *",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(title)
    .bind(instructor)
    .bind(description)
    .bind(price)
    .bind(course_type.to_string())
    .bind(url)
    .bind(location)
    .bind(video_link)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(course)))
}

#[utoipa::path(
    get,
    path = "/courses/{id}",
    responses(
        (status = 200, description = "Found course", body = Course),
        (status = 404, description = "Course not found")
    )
)]
pub async fn get_course(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Course>, ApiError> {
    let course = sqlx::query_as::<_, Course>(
        "SELECT * FROM courses WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound("Course not found".into()))?;

    Ok(Json(course))
}

#[utoipa::path(
    put,
    path = "/courses/{id}",
    request_body = UpdateCourse,
    responses(
        (status = 200, description = "Course updated", body = Course),
        (status = 404, description = "Course not found")
    )
)]
pub async fn update_course(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateCourse>,
) -> Result<Json<Course>, ApiError> {
    // Load existing row
    let existing = sqlx::query_as::<_, Course>("SELECT * FROM courses WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(ApiError::NotFound("Course not found".into()))?;

    // Build workflow: Unchecked -> validate() -> apply()
    let checked = UpdateCourseTs::<Unchecked>::new(existing, payload)
        .validate()?; // ApiError::BadRequest on invalid

    let updated = checked.apply(&state.pool).await?; // sqlx::Error -> ApiError via ? at the top level
    Ok(Json(updated))
}

#[utoipa::path(
    delete,
    path = "/courses/{id}",
    responses(
        (status = 204, description = "Course deleted"),
        (status = 404, description = "Course not found")
    )
)]
pub async fn delete_course(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    let rows = sqlx::query("DELETE FROM courses WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?
        .rows_affected();

    if rows == 0 {
        Err(ApiError::NotFound("Course not found".into()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListParams {
    instructor: Option<String>,
    min_price: Option<f64>,
}

#[utoipa::path(
    get,
    path = "/courses",
    params(ListParams),
    responses((status = 200, description = "List of courses", body = [Course]))
)]
pub async fn list_courses(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Course>>, ApiError> {
    let mut query = QueryBuilder::new("SELECT * FROM courses WHERE 1=1");

    if let Some(ref instructor) = params.instructor {
        query.push(" AND instructor = ").push_bind(instructor);
    }

    if let Some(min_price) = params.min_price {
        query.push(" AND price >= ").push_bind(min_price);
    }

    let courses = query
        .build_query_as::<Course>()
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(courses))
}


#[utoipa::path(
    get,
    path = "/teachables",
    responses((status = 200, description = "List of teachables", body = [TeachableOut]))
)]
pub async fn list_teachables(
    State(state): State<AppState>
) -> Result<axum::Json<Vec<TeachableOut>>, ApiError> {
    let rows = sqlx::query!(
        r#"SELECT title, instructor, price, course_type, url, location, video_link FROM courses"#
    )
    .fetch_all(&state.pool)
    .await?;

    let out = rows
        .into_iter()
        .filter_map(|row| match row.course_type.as_str() {
            "online" => {
                let url = row.url?;
                Some(Box::new(OnlineCourse {
                    title: row.title,
                    instructor: row.instructor,
                    url,
                    price: row.price,
                }) as Box<dyn Teachable>)
            }
            "workshop" => {
                let location = row.location?;
                Some(Box::new(InPersonWorkshop {
                    title: row.title,
                    instructor: row.instructor,
                    location,
                    price: row.price,
                }) as Box<dyn Teachable>)
            }
            "webinar" => {
                let video_link = row.video_link?;
                Some(Box::new(RecordedWebinar {
                    title: row.title,
                    instructor: row.instructor,
                    video_link,
                    price: row.price,
                }) as Box<dyn Teachable>)
            }
            _ => None,
        })
        .map(|t| TeachableOut {
            title: t.title().to_string(),
            instructor: t.instructor().to_string(),
            mode: t.teach_mode(),
            price: t.price(),
        })
        .collect();

    for t in &out {
        tracing::info!("{}", format_teachable(t));
    }
       
    Ok(axum::Json(out))
}