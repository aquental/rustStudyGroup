use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use crate::{
    errors::ApiError,
    models::{AppState, Course, CourseType, UpdateCourse, NewCourse},
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
    // First, get the existing course to determine its type
    let existing_course = sqlx::query_as::<_, Course>(
        "SELECT * FROM courses WHERE id = $1",
    )
    .bind(&id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound("Course not found".into()))?;

    // Validate fields according to the course type
    match existing_course.course_type {
        CourseType::Online => {
            if payload.location.is_some() || payload.video_link.is_some() {
                return Err(ApiError::BadRequest(
                    "Cannot set location or video_link for online course".into()
                ));
            }
        }
        CourseType::Workshop => {
            if payload.url.is_some() || payload.video_link.is_some() {
                return Err(ApiError::BadRequest(
                    "Cannot set url or video_link for workshop".into()
                ));
            }
        }
        CourseType::Webinar => {
            if payload.url.is_some() || payload.location.is_some() {
                return Err(ApiError::BadRequest(
                    "Cannot set url or location for webinar".into()
                ));
            }
        }
    }

    // Perform the update
    let course = sqlx::query_as::<_, Course>(
        "UPDATE courses
         SET title = COALESCE($2, title),
             instructor = COALESCE($3, instructor),
             description = COALESCE($4, description),
             price = COALESCE($5, price),
             url = COALESCE($6, url),
             location = COALESCE($7, location),
             video_link = COALESCE($8, video_link)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(payload.title)
    .bind(payload.instructor)
    .bind(payload.description)
    .bind(payload.price)
    .bind(payload.url)
    .bind(payload.location)
    .bind(payload.video_link)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(course))
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
