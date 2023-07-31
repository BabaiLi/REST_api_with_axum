use crate::{
    models::task,
    errors::CustomError,
};

use axum::{
    response::IntoResponse,
    extract::Path,
    http::StatusCode,
    Extension,
    Json
};
use sqlx::PgPool;
use serde_json::{json, Value};

pub async fn all_tasks(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM task ".to_string();

    let task = sqlx::query_as::<_, task::Task>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK,Json(task))
}

pub async fn task(
    Path(id):Path<i32>,
    Extension(pool): Extension<PgPool>
)
-> Result <Json<task::Task>, CustomError> {
    
    let sql = "SELECT * FROM task where id=$1".to_string();

    let task: task::Task = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::TaskNotFound
        })?;
    
    Ok(Json(task))  
}

pub async fn new_task(
    Extension(pool): Extension<PgPool>, 
    Json(task): Json<task::NewTask>
)
-> Result <(StatusCode, Json<task::NewTask>), CustomError> {
    
    if task.task.is_empty() {
        return Err(CustomError::BadRequest)
    }
    let sql = "INSERT INTO task (id, task, stat) values ($1, $2, $3)";

    let _ = sqlx::query(&sql)
        .bind(&task.id)
        .bind(&task.task)
        .bind(&task.stat)
        .execute(&pool)
        .await
        .map_err(|_| {
            CustomError::InternalServerError
        })?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn update_task(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    Json(task): Json<task::UpdateTask>
)
-> Result<(StatusCode, Json<task::UpdateTask>), CustomError> {
    
    let sql = "SELECT * FROM task where id=$1".to_string();

    let _find: task::Task = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool).await
        .map_err(|_| {
            CustomError::TaskNotFound
        })?;

    let _ = sqlx::query("UPDATE task SET task=$1, stat=$2 WHERE id=$3")
        .bind(&task.task)
        .bind(&task.stat)
        .bind(id)
        .execute(&pool)
        .await;
    
    Ok((StatusCode::OK, Json(task)))
}

pub async fn patch_task(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    Json(task): Json<task::UpdatePartialTask>
)
-> Result<(StatusCode, Json<task::UpdatePartialTask>), CustomError> {
    
    let sql = "SELECT * FROM task where id=$1".to_string();

    let _find: task::Task = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool).await
        .map_err(|_| {
            CustomError::TaskNotFound
        })?;
    
    if let Some(task_task) = &task.task {
        let _ = sqlx::query("UPDATE task SET task=$1 WHERE id=$2")
            .bind(task_task)
            .bind(id)
            .execute(&pool)
            .await;
    }

    if let Some(task_stat) = &task.stat {
        let _ = sqlx::query("UPDATE task SET stat=$1 WHERE id=$2")
            .bind(task_stat)
            .bind(id)
            .execute(&pool)
            .await;
    }
    
    Ok((StatusCode::OK, Json(task)))
}

pub async fn delete_task(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>
)
-> Result <(StatusCode, Json<Value>), CustomError> {

    let _find: task::Task = sqlx::query_as("SELECT * FROM task where id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::TaskNotFound
        })?;

    sqlx::query("DELETE FROM task WHERE id=$1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| {
            CustomError::TaskNotFound
        })?;
    
    Ok((StatusCode::OK, Json(json!({"msg": "Task Deleted"}))))
}